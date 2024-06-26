use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

use curie::PrefixMapping;
use horned_bin::path_type;
use horned_owl::error::HornedError;
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration, ResourceType};
use horned_owl::model::*;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::ontology::{get_ancestors, get_descendants, PyIndexedOntology, IndexCreationStrategy};

#[macro_use]
mod doc;
mod model;
mod ontology;



#[macro_export]
macro_rules! to_py_err {
    ($message:literal) => {
        | error | PyValueError::new_err(format!("{}: {:?}", $message, error))
    };
}


fn parse_serialization(serialization: Option<&str>) -> Option<ResourceType> {
    match serialization.map(|s| s.to_lowercase()).as_deref() {
        Some("owx") => Some(ResourceType::OWX),
        Some("ofn") => Some(ResourceType::OFN),
        Some("rdf") => Some(ResourceType::RDF),
        Some("owl") => Some(ResourceType::RDF),
        _ => None
    }
}

fn guess_serialization(path: &String, serialization: Option<&str>) -> PyResult<ResourceType> {
    parse_serialization(serialization).map(Ok).unwrap_or(
        match serialization.map(|s| s.to_lowercase()).as_deref() {
            Some(f) => Err(PyValueError::new_err(format!("Unsupported serialization '{}'", f))),
            None => Ok(path_type(path.as_ref()).unwrap_or(ResourceType::OWX))
        })
}

fn open_ontology_owx<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
    index_strategy: IndexCreationStrategy
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    let (o, m) = horned_owl::io::owx::reader::read_with_build(content, &b)?;
    Ok((PyIndexedOntology::from_set_ontology(o, index_strategy), m))
}

fn open_ontology_ofn<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
    index_strategy: IndexCreationStrategy
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    let (o, m) = horned_owl::io::ofn::reader::read_with_build(content, &b)?;
    Ok((PyIndexedOntology::from_set_ontology(o, index_strategy), m))
}

fn open_ontology_rdf<R: BufRead>(
    content: &mut R,
    b: &Build<ArcStr>,
    index_strategy: IndexCreationStrategy
) -> Result<(PyIndexedOntology, PrefixMapping), HornedError> {
    horned_owl::io::rdf::reader::read_with_build::<ArcStr, ArcAnnotatedComponent, R>(
        content,
        &b, 
        ParserConfiguration {
            rdf: RDFParserConfiguration { lax: true },
            ..Default::default()
        },
    ).map(|(o, _)| (PyIndexedOntology::from_rdf_ontology(o, index_strategy), PrefixMapping::default()))
}

/// open_ontology_from_file(path: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology
///
/// Opens an ontology from a file
///
/// If the serialization is not specified it is guessed from the file extension. Defaults to OWL/XML.
#[pyfunction(signature = (path, serialization = None, index_strategy = IndexCreationStrategy::OnQuery))]
fn open_ontology_from_file(path: String, serialization: Option<&str>, index_strategy: IndexCreationStrategy) -> PyResult<PyIndexedOntology> {
    let serialization = guess_serialization(&path, serialization)?;

    let file = File::open(path)?;
    let mut f = BufReader::new(file);

    let b = Build::new_arc();

    let (mut pio, mapping) = match serialization {
        ResourceType::OFN => open_ontology_ofn(&mut f, &b, index_strategy),
        ResourceType::OWX => open_ontology_owx(&mut f, &b, index_strategy),
        ResourceType::RDF => open_ontology_rdf(&mut f, &b, index_strategy)
    }.map_err(to_py_err!("Failed to open ontology"))?;

    pio.mapping = mapping;
    Ok(pio)
}

/// open_ontology_from_string(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology
///
/// Opens an ontology from plain text.
///
/// If no serialization is specified, all parsers are tried until one succeeds
#[pyfunction(signature = (ontology, serialization = None, index_strategy = IndexCreationStrategy::OnQuery))]
fn open_ontology_from_string(ontology: String, serialization: Option<&str>, index_strategy: IndexCreationStrategy) -> PyResult<PyIndexedOntology> {
    let serialization = parse_serialization(serialization);
    let mut f = BufReader::new(ontology.as_bytes());

    let b = Build::new_arc();

    let (imo, mapping) = match serialization {
        Some(ResourceType::OFN) => open_ontology_ofn(&mut f, &b, index_strategy),
        Some(ResourceType::OWX) => open_ontology_owx(&mut f, &b, index_strategy),
        Some(ResourceType::RDF) => open_ontology_rdf(&mut f, &b, index_strategy),
        None => open_ontology_ofn(&mut f, &b, index_strategy)
            .or_else(|_| open_ontology_rdf(&mut f, &b, index_strategy))
            .or_else(|_| open_ontology_owx(&mut f, &b, index_strategy))
    }.map_err(to_py_err!("Failed to open ontology"))?;

    let mut lo = PyIndexedOntology::from(imo);
    lo.mapping = mapping; //Needed when saving
    Ok(lo)
}

/// open_ontology(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology
///
/// Opens an ontology from a path or plain text.
///
/// If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interpreted as an ontology
/// in plain text.
/// If no serialization is specified the serialization is guessed by the file extension or all parsers are tried
/// until one succeeds.
#[pyfunction(signature = (ontology, serialization = None, index_strategy = IndexCreationStrategy::OnQuery))]
fn open_ontology(ontology: String, serialization: Option<&str>, index_strategy: IndexCreationStrategy) -> PyResult<PyIndexedOntology> {
    if Path::exists(ontology.as_ref()) {
        open_ontology_from_file(ontology, serialization, index_strategy)
    } else {
        open_ontology_from_string(ontology, serialization, index_strategy)
    }
}

#[pymodule]
fn pyhornedowl(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyIndexedOntology>()?;
    m.add_class::<IndexCreationStrategy>()?;

    m.add_function(wrap_pyfunction!(open_ontology, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology_from_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_descendants, m)?)?;
    m.add_function(wrap_pyfunction!(get_ancestors, m)?)?;

    let model_sub_module = model::py_module(py)?;
    m.add_submodule(&model_sub_module)?;

    Ok(())
}
