use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use curie::PrefixMapping;
use horned_owl::error::HornedError;
use horned_owl::io::ofn::writer::AsFunctional;
use horned_owl::io::omn::writer::AsManchester;
use horned_owl::io::{InputFormat, ParserConfiguration, RDFParserConfiguration, ResourceType};
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

fn parse_serialization(serialization: &str) -> PyResult<InputFormat> {
    // `InputFormat::from_str` accepts "omn" but not the spelled-out alias, which
    // reads better next to "obo" in user-facing code.
    if serialization == "manchester" {
        return Ok(InputFormat::OMN);
    }
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

    Ok((input_format, default_parser_config(Some(input_format))))
}

fn default_parser_config(input_format: Option<InputFormat>) -> ParserConfiguration {
    ParserConfiguration {
        lax: true,
        rdf: RDFParserConfiguration {
            format: match input_format {
                Some(InputFormat::Rdf(format)) => format,
                _ => None,
            },
        },
        input_format,
        ..Default::default()
    }
}

fn open_ontology_owx<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
    config: ParserConfiguration,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::owx::reader::read_with_build(content, b, config)
}

fn open_ontology_ofn<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::ofn::reader::read_with_build(content, b)
}

fn open_ontology_omn<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::omn::reader::read_with_build(content, b)
}

fn open_ontology_obo<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::obo::reader::read_with_build(content, b)
}

fn open_ontology_rdf<R: BufRead>(
    content: &mut R,
    b: &Build<ArcStr>,
    index_strategy: IndexCreationStrategy,
    config: ParserConfiguration,
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::rdf::reader::read_with_build::<ArcStr, ArcAnnotatedComponent, R>(
        content, b, config,
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
    let file = File::open(&path)?;

    let (input_format, config) = parser_config(Path::new(&path), serialization)?;

    let mut f = BufReader::new(file);

    let b = Build::new_arc();

    let (mut pio, mapping) = match input_format {
        InputFormat::OFN => open_ontology_ofn(&mut f, &b),
        InputFormat::OMN => open_ontology_omn(&mut f, &b),
        InputFormat::OBO => open_ontology_obo(&mut f, &b),
        InputFormat::Rdf(_) => open_ontology_rdf(&mut f, &b, index_strategy, config),
        InputFormat::OWX => open_ontology_owx(&mut f, &b, config),
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
    let input_format = serialization
        .map(parse_serialization)
        .transpose()?
        .or_else(|| {
            horned_owl::io::detect_format(ontology.as_bytes()).map(to_input_format)
        });

    let config = default_parser_config(input_format);

    let mut f = BufReader::new(ontology.as_bytes());

    let b = Build::new_arc();

    let (mut pio, mapping) = match input_format {
        Some(InputFormat::OFN) => open_ontology_ofn(&mut f, &b),
        Some(InputFormat::OWX) => open_ontology_owx(&mut f, &b, config),
        Some(InputFormat::Rdf(_)) => open_ontology_rdf(&mut f, &b, index_strategy, config),
        Some(InputFormat::OMN) => open_ontology_omn(&mut f, &b),
        Some(InputFormat::OBO) => open_ontology_obo(&mut f, &b),
        None | Some(InputFormat::Guess) => {
            open_ontology_owx(&mut BufReader::new(ontology.as_bytes()), &b, config.clone())
                .or_else(|_| open_ontology_ofn(&mut BufReader::new(ontology.as_bytes()), &b))
                .or_else(|_| {
                    open_ontology_rdf(
                        &mut BufReader::new(ontology.as_bytes()),
                        &b,
                        index_strategy,
                        config,
                    )
                })
        }
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

/// The serializations that have a per-element writer in horned-owl.
enum SnippetSyntax {
    Manchester,
    Functional,
}

fn parse_snippet_syntax(serialization: &str) -> PyResult<SnippetSyntax> {
    match serialization {
        "omn" | "manchester" => Ok(SnippetSyntax::Manchester),
        "ofn" | "functional" => Ok(SnippetSyntax::Functional),
        other => Err(PyValueError::new_err(format!(
            "Cannot write a snippet in {:?}. horned-owl has per-element writers only for \
             \"omn\" (\"manchester\") and \"ofn\" (\"functional\"); the OWL/XML and RDF \
             writers work on whole ontologies only.",
            other
        ))),
    }
}

/// write_snippet(element: typing.Union[model.AnnotatedComponent, model.Component, model.ClassExpression], serialization: typing.Literal['omn', 'manchester', 'ofn', 'functional']='omn', prefix_mapping: typing.Optional[PrefixMapping]=None) -> str
///
/// Renders a single axiom, component, or class expression as a string.
///
/// This is the per-element counterpart to `save_to_string`, which serializes a whole
/// ontology. Manchester output in particular cannot be recovered from
/// `save_to_string("omn")`, because that groups axioms into entity frames.
///
/// Only `"omn"` (alias `"manchester"`) and `"ofn"` (alias `"functional"`) are supported:
/// those are the serializations for which horned-owl provides a per-element writer. The
/// OWL/XML and RDF writers operate on whole ontologies only.
///
/// If a `prefix_mapping` is given, IRIs are abbreviated with it where possible.
///
/// Note that `"ofn"` renders an `AnnotatedComponent` including its axiom annotations,
/// while `"omn"` renders only the component: horned-owl's Manchester writer has no
/// per-element rendering for annotated axioms.
#[pyfunction]
#[pyo3(signature = (element, serialization = "omn", prefix_mapping = None))]
fn write_snippet(
    element: &Bound<'_, PyAny>,
    serialization: &str,
    prefix_mapping: Option<&prefix_mapping::PrefixMapping>,
) -> PyResult<String> {
    let syntax = parse_snippet_syntax(serialization)?;

    macro_rules! render {
        ($value:expr) => {
            match (&syntax, prefix_mapping) {
                (SnippetSyntax::Manchester, Some(pm)) => {
                    $value.as_manchester_with_prefixes(&pm.0).to_string()
                }
                (SnippetSyntax::Manchester, None) => $value.as_manchester().to_string(),
                (SnippetSyntax::Functional, Some(pm)) => {
                    $value.as_functional_with_prefixes(&pm.0).to_string()
                }
                (SnippetSyntax::Functional, None) => $value.as_functional().to_string(),
            }
        };
    }

    if let Ok(ac) = element.extract::<model::AnnotatedComponent>() {
        let ac: AnnotatedComponent<Arc<str>> = ac.into();
        // AsManchester has no impl for AnnotatedComponent, so Manchester renders the
        // bare component; functional syntax renders the annotations too.
        return Ok(match (&syntax, prefix_mapping) {
            (SnippetSyntax::Manchester, Some(pm)) => {
                ac.component.as_manchester_with_prefixes(&pm.0).to_string()
            }
            (SnippetSyntax::Manchester, None) => ac.component.as_manchester().to_string(),
            (SnippetSyntax::Functional, Some(pm)) => {
                ac.as_functional_with_prefixes(&pm.0).to_string()
            }
            (SnippetSyntax::Functional, None) => ac.as_functional().to_string(),
        });
    }
    if let Ok(c) = element.extract::<model::Component>() {
        let c: Component<Arc<str>> = c.into();
        return Ok(render!(c));
    }
    if let Ok(ce) = element.extract::<model::ClassExpression>() {
        let ce: ClassExpression<Arc<str>> = ce.into();
        return Ok(render!(ce));
    }
    Err(PyValueError::new_err(
        "write_snippet expects a model.AnnotatedComponent, model.Component, or model.ClassExpression",
    ))
}

#[pymodule]
fn pyhornedowl(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyIndexedOntology>()?;
    m.add_class::<IndexCreationStrategy>()?;
    m.add_class::<prefix_mapping::PrefixMapping>()?;

    m.add_function(wrap_pyfunction!(write_snippet, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology_from_string, m)?)?;

    let model_sub_module = model::py_module(py)?;
    m.add_submodule(&model_sub_module)?;

    let reasoning_sub_module = PyModule::new(py, "reasoning")?;
    reasoning_sub_module.add_function(wrap_pyfunction!(create_reasoner, &reasoning_sub_module)?)?;
    reasoning_sub_module.add_function(wrap_pyfunction!(
        create_structural_reasoner,
        &reasoning_sub_module
    )?)?;
    reasoning_sub_module.add_class::<reasoning::PyReasoner>()?;
    m.add_submodule(&reasoning_sub_module)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
