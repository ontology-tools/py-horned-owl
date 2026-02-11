use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, Mutex};

use curie::PrefixMapping;
use horned_bin::path_type;
use horned_owl::error::HornedError;
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration, ResourceType};
use horned_owl::model::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use pyhornedowlreasoner::PyReasoner;

#[macro_use]
mod doc;
pub mod model;
pub mod model_generated;
pub mod ontology;
pub mod prefix_mapping;
pub mod reasoning;
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

fn parse_serialization(serialization: &str) -> PyResult<ResourceType> {
    match serialization {
        "owx" => Ok(ResourceType::OWX),
        "ofn" => Ok(ResourceType::OFN),
        "rdf" => Ok(ResourceType::RDF),
        "owl" => Ok(ResourceType::RDF),
        s => Err(PyValueError::new_err(format!(
            "Unknown serialization {}",
            s
        ))),
    }
}

pub fn guess_serialization(path: &String, serialization: Option<&str>) -> PyResult<ResourceType> {
    match serialization {
        Some(s) => parse_serialization(s),
        None => Ok(path_type(path.as_ref()).unwrap_or(ResourceType::OWX)),
    }
}

fn open_ontology_owx<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::owx::reader::read_with_build(content, b)
}

fn open_ontology_ofn<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::ofn::reader::read_with_build(content, b)
}

fn open_ontology_rdf<R: BufRead>(
    content: &mut R,
    b: &Build<ArcStr>,
    index_strategy: IndexCreationStrategy,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::rdf::reader::read_with_build::<ArcStr, ArcAnnotatedComponent, R>(
        content,
        b,
        ParserConfiguration {
            rdf: RDFParserConfiguration {
                lax: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .map(|(o, _)| {
        (
            PyIndexedOntology::from_rdf_ontology(o, index_strategy),
            PrefixMapping::default(),
        )
    })
}

/// open_ontology_from_file(path: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology
///
/// Opens an ontology from a file
///
/// If the serialization is not specified it is guessed from the file extension. Defaults to OWL/XML.
#[pyfunction(
    signature = (path, serialization = None, index_strategy = IndexCreationStrategy::OnQuery)
)]
fn open_ontology_from_file(
    py: Python<'_>,
    path: String,
    serialization: Option<&str>,
    index_strategy: IndexCreationStrategy,
) -> PyResult<PyIndexedOntology> {
    let serialization = guess_serialization(&path, serialization)?;

    let file = File::open(path)?;
    let mut f = BufReader::new(file);

    let b = Build::new_arc();

    let (mut pio, mapping) = match serialization {
        ResourceType::OFN => open_ontology_ofn(&mut f, &b),
        ResourceType::OWX => open_ontology_owx(&mut f, &b),
        ResourceType::RDF => open_ontology_rdf(&mut f, &b, index_strategy),
    }
    .map_err(to_py_err!("Failed to open ontology"))?;

    if let IndexCreationStrategy::OnLoad = index_strategy {
        pio.build_indexes()
    }

    pio.mapping = Py::new(py, prefix_mapping::PrefixMapping::from(mapping))?;
    Ok(pio)
}

/// open_ontology_from_string(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology
///
/// Opens an ontology from plain text.
///
/// If no serialization is specified, all parsers are tried until one succeeds
#[pyfunction(
    signature = (ontology, serialization = None, index_strategy = IndexCreationStrategy::OnQuery)
)]
fn open_ontology_from_string(
    py: Python<'_>,
    ontology: String,
    serialization: Option<&str>,
    index_strategy: IndexCreationStrategy,
) -> PyResult<PyIndexedOntology> {
    let serialization = match serialization {
        None => Ok(None),
        Some(s) => parse_serialization(s).map(Some),
    }?;
    let mut f = BufReader::new(ontology.as_bytes());

    let b = Build::new_arc();

    let (mut pio, mapping) = match serialization {
        Some(ResourceType::OFN) => open_ontology_ofn(&mut f, &b),
        Some(ResourceType::OWX) => open_ontology_owx(&mut f, &b),
        Some(ResourceType::RDF) => open_ontology_rdf(&mut f, &b, index_strategy),
        None => open_ontology_owx(&mut BufReader::new(ontology.as_bytes()), &b)
            .or_else(|_| open_ontology_ofn(&mut BufReader::new(ontology.as_bytes()), &b))
            .or_else(|_| {
                open_ontology_rdf(&mut BufReader::new(ontology.as_bytes()), &b, index_strategy)
            }),
    }
    .map_err(to_py_err!("Failed to open ontology"))?;

    if let IndexCreationStrategy::OnLoad = index_strategy {
        pio.build_indexes()
    }

    pio.mapping = Py::new(py, prefix_mapping::PrefixMapping::from(mapping))?;
    Ok(pio)
}

/// open_ontology(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology
///
/// Opens an ontology from a path or plain text.
///
/// If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interpreted as an ontology
/// in plain text.
/// If no serialization is specified the serialization is guessed by the file extension or all parsers are tried
/// until one succeeds.
#[pyfunction(
    signature = (ontology, serialization = None, index_strategy = IndexCreationStrategy::OnQuery)
)]
fn open_ontology(
    py: Python<'_>,
    ontology: String,
    serialization: Option<&str>,
    index_strategy: IndexCreationStrategy,
) -> PyResult<PyIndexedOntology> {
    if Path::exists(ontology.as_ref()) {
        open_ontology_from_file(py, ontology, serialization, index_strategy)
    } else {
        open_ontology_from_string(py, ontology, serialization, index_strategy)
    }
}

/// create_structural_reasoner(ontology: PyIndexedOntology) -> PyReasoner
///
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
fn pyhornedowl(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyIndexedOntology>()?;
    m.add_class::<IndexCreationStrategy>()?;
    m.add_class::<prefix_mapping::PrefixMapping>()?;

    m.add_function(wrap_pyfunction!(open_ontology, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology_from_string, m)?)?;
    
    let model_sub_module = model::py_module(py)?;
    m.add_submodule(&model_sub_module)?;


    let reasoning_sub_module = PyModule::new(py, "reasoning")?;
    reasoning_sub_module.add_function(wrap_pyfunction!(create_reasoner, &reasoning_sub_module)?)?;
    reasoning_sub_module.add_function(wrap_pyfunction!(create_structural_reasoner, &reasoning_sub_module)?)?;
    reasoning_sub_module.add_class::<reasoning::PyReasoner>()?;
    m.add_submodule(&reasoning_sub_module)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
