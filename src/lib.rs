use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use curie::PrefixMapping;
use horned_owl::error::HornedError;
use horned_owl::io::{
    InputFormat, ParserConfiguration as HornedParserConfiguration, RDFParserConfiguration,
    ResourceType,
};
use horned_owl::model::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use wrappers::{LiteralStr, Serializations};

use pyhornedowlreasoner::PyReasoner;

type ParserConfiguration = HornedParserConfiguration<ArcStr>;

#[macro_use]
mod doc;
pub mod model;
pub mod model_generated;
pub mod ontology;
pub mod prefix_mapping;
pub mod profile;
pub mod reasoning;
pub mod snippet;
pub mod structural_reasoner;
mod wrappers;

pub use reasoning::create_reasoner;

pub use ontology::{IndexCreationStrategy, PyIndexedOntology};

#[macro_export]
macro_rules! to_py_err {
    ($message:literal) => {
        |error| PyValueError::new_err(format!("{}: {:?}", $message, error))
    };
}

fn parse_serialization(serialization: &str) -> PyResult<InputFormat> {
    match InputFormat::from_str(serialization) {
        Ok(InputFormat::Guess) | Err(_) => Err(PyValueError::new_err(format!(
            "Unknown serialization {}",
            serialization
        ))),
        Ok(input_format) => Ok(input_format),
    }
}

fn to_input_format(
    (resource_type, format): (ResourceType, Option<oxrdfio::RdfFormat>),
) -> InputFormat {
    match resource_type {
        ResourceType::OFN => InputFormat::OFN,
        ResourceType::OWX => InputFormat::OWX,
        ResourceType::OMN => InputFormat::OMN,
        ResourceType::OBO => InputFormat::OBO,
        ResourceType::RDF => InputFormat::Rdf(format),
    }
}

fn sniff_input_format(path: &Path) -> Option<InputFormat> {
    use std::io::Read;

    let mut buf = [0u8; 512];
    let n = File::open(path).ok()?.read(&mut buf).ok()?;

    horned_owl::io::detect_format(&buf[..n]).map(to_input_format)
}

fn parser_config(
    path: &Path,
    serialisation: Option<&str>,
    build: Option<Build<ArcStr>>,
) -> PyResult<(InputFormat, ParserConfiguration)> {
    let input_format = serialisation
        .map(parse_serialization)
        .transpose()?
        .or_else(|| {
            path.extension()
                .and_then(OsStr::to_str)
                .and_then(|extension| parse_serialization(extension).ok())
        })
        .or_else(|| sniff_input_format(path));

    let input_format = input_format.ok_or_else(|| {
        PyValueError::new_err(format!(
            "Cannot determine serialization for file {}",
            path.display()
        ))
    })?;

    Ok((
        input_format,
        default_parser_config(Some(input_format), build),
    ))
}

fn default_parser_config(
    input_format: Option<InputFormat>,
    build: Option<Build<ArcStr>>,
) -> ParserConfiguration {
    let mut conf = ParserConfiguration::new(build.unwrap_or_else(|| Build::new_arc()));

    conf.lax = true;
    conf.input_format = input_format;

    conf
}

fn open_ontology_owx<R: BufRead>(
    content: &mut R,
    config: ParserConfiguration,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::owx::reader::read(content, config)
}

fn open_ontology_ofn<R: BufRead>(
    content: &mut R,
    config: ParserConfiguration,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::ofn::reader::read(content, config)
}

fn open_ontology_omn<R: BufRead>(
    content: &mut R,
    config: ParserConfiguration,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::omn::reader::read(content, config)
}

fn open_ontology_obo<R: BufRead>(
    content: &mut R,
    config: ParserConfiguration,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::obo::reader::read(content, config)
}

fn open_ontology_rdf<R: BufRead>(
    content: &mut R,
    config: ParserConfiguration,
    index_strategy: IndexCreationStrategy,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    let rdf_config = RDFParserConfiguration {
        format: config.input_format.and_then(|f| match f {
            InputFormat::Rdf(format) => format,
            _ => None,
        }),
        common: config,
    };

    horned_owl::io::rdf::reader::read::<ArcStr, ArcAnnotatedComponent, Build<ArcStr>, R>(
        content, rdf_config,
    )
    .map(|(o, _)| {
        (
            PyIndexedOntology::from_rdf_ontology(o, index_strategy),
            PrefixMapping::default(),
        )
    })
}

/// Opens an ontology from a file
///
/// If the serialization is not specified it is guessed from the file extension. Defaults to OWL/XML.
#[pyfunction(
    signature = (path, serialization = None, index_strategy = IndexCreationStrategy::OnQuery)
)]
fn open_ontology_from_file(
    py: Python<'_>,
    path: String,
    serialization: Option<LiteralStr<Serializations>>,
    index_strategy: IndexCreationStrategy,
) -> PyResult<PyIndexedOntology> {
    let file = File::open(&path)?;

    let b = Build::new_arc();
    let (input_format, config) =
        parser_config(Path::new(&path), serialization.as_deref(), Some(b))?;

    let mut f = BufReader::new(file);

    let (mut pio, mapping) = match input_format {
        InputFormat::OFN => open_ontology_ofn(&mut f, config),
        InputFormat::OMN => open_ontology_omn(&mut f, config),
        InputFormat::OBO => open_ontology_obo(&mut f, config),
        InputFormat::Rdf(_) => open_ontology_rdf(&mut f, config, index_strategy),
        InputFormat::OWX => open_ontology_owx(&mut f, config),
        InputFormat::Guess => {
            return Err(PyValueError::new_err(format!(
                "Cannot determine serialization for file {}",
                path
            )))
        }
    }
    .map_err(to_py_err!("Failed to open ontology"))?;

    if let IndexCreationStrategy::OnLoad = index_strategy {
        pio.build_indexes()
    }

    pio.mapping = Py::new(py, prefix_mapping::PrefixMapping::from(mapping))?;
    Ok(pio)
}

/// Opens an ontology from plain text.
///
/// If no serialization is specified, it is detected from the content. Raises ValueError if it cannot be detected.
#[pyfunction(
    signature = (ontology, serialization = None, index_strategy = IndexCreationStrategy::OnQuery)
)]
fn open_ontology_from_string(
    py: Python<'_>,
    ontology: String,
    serialization: Option<LiteralStr<Serializations>>,
    index_strategy: IndexCreationStrategy,
) -> PyResult<PyIndexedOntology> {
    let input_format = serialization
        .as_deref()
        .map(parse_serialization)
        .transpose()?
        .or_else(|| horned_owl::io::detect_format(ontology.as_bytes()).map(to_input_format));

    let b = Build::new_arc();

    let config = default_parser_config(input_format, Some(b));

    let mut f = BufReader::new(ontology.as_bytes());

    let (mut pio, mapping) = match input_format {
        Some(InputFormat::OFN) => open_ontology_ofn(&mut f, config),
        Some(InputFormat::OWX) => open_ontology_owx(&mut f, config),
        Some(InputFormat::Rdf(_)) => open_ontology_rdf(&mut f, config, index_strategy),
        Some(InputFormat::OMN) => open_ontology_omn(&mut f, config),
        Some(InputFormat::OBO) => open_ontology_obo(&mut f, config),
        None | Some(InputFormat::Guess) => {
            return Err(PyValueError::new_err(
                "Cannot determine serialization for ontology string",
            ));
        }
    }
    .map_err(to_py_err!("Failed to open ontology"))?;

    if let IndexCreationStrategy::OnLoad = index_strategy {
        pio.build_indexes()
    }

    pio.mapping = Py::new(py, prefix_mapping::PrefixMapping::from(mapping))?;
    Ok(pio)
}

/// Opens an ontology from a path or plain text.
///
/// If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interpreted as an ontology
/// in plain text.
/// If no serialization is specified, it is guessed from the file extension (for a path) or detected from the content.
/// Raises a `ValueError` if the serialization cannot be determined.
#[pyfunction(
    signature = (ontology, serialization = None, index_strategy = IndexCreationStrategy::OnQuery)
)]
fn open_ontology(
    py: Python<'_>,
    ontology: String,
    serialization: Option<LiteralStr<Serializations>>,
    index_strategy: IndexCreationStrategy,
) -> PyResult<PyIndexedOntology> {
    if Path::exists(ontology.as_ref()) {
        open_ontology_from_file(py, ontology, serialization, index_strategy)
    } else {
        open_ontology_from_string(py, ontology, serialization, index_strategy)
    }
}

/// Creates a structural reasoner for the given ontology. The structural reasoner only uses the asserted named subclass and sub-property hierarchies to answer queries.
#[pyfunction]
fn create_structural_reasoner(ontology: PyIndexedOntology) -> reasoning::PyReasoner {
    reasoning::PyReasoner(Arc::new(Mutex::new(
        crate::reasoning::DynamicLoadedReasoner(
            Box::new(structural_reasoner::StructuralReasoner::create_reasoner(
                ontology.into(),
            )),
            Box::new(None),
        ),
    )))
}

#[pymodule]
mod pyhornedowl {
    use pyo3::prelude::*;

    #[pymodule_export]
    use super::prefix_mapping::PrefixMapping;
    #[pymodule_export]
    use super::{open_ontology, open_ontology_from_file, open_ontology_from_string};
    #[pymodule_export]
    use super::{IndexCreationStrategy, PyIndexedOntology};

    #[pymodule_export]
    use crate::model::py_model;
    #[pymodule_export]
    use crate::profile::py_profile;

    #[pymodule]
    mod reasoning {
        #[pymodule_export]
        use crate::reasoning::PyReasoner;
        #[pymodule_export]
        use crate::{create_reasoner, create_structural_reasoner};
    }

    #[pymodule_export]
    #[allow(non_upper_case_globals)]
    const __version__: &str = env!("CARGO_PKG_VERSION");
}
