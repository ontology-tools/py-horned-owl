use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
//use failure::Error;
use std::sync::Arc;

use curie::PrefixMapping;
use horned_bin::path_type;
use horned_owl::error::HornedError;
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration, ResourceType};
use horned_owl::model::*;
use horned_owl::ontology::iri_mapped::IRIMappedOntology;
use horned_owl::ontology::set::SetOntology;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use crate::ontology::{get_ancestors, get_descendants, PyIndexedOntology};

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
) -> Result<(SetOntology<ArcStr>, PrefixMapping), HornedError> {
    horned_owl::io::owx::reader::read_with_build(content, &b)
}

fn open_ontology_ofn<R: BufRead>(
    content: &mut R,
    b: &Build<Arc<str>>,
) -> Result<(SetOntology<ArcStr>, PrefixMapping), HornedError> {
    horned_owl::io::ofn::reader::read_with_build(content, &b)
}

fn open_ontology_rdf<R: BufRead>(
    content: &mut R,
    b: &Build<ArcStr>,
) -> Result<(SetOntology<ArcStr>, PrefixMapping), HornedError> {
    horned_owl::io::rdf::reader::read_with_build::<ArcStr, ArcAnnotatedComponent, R>(
        content,
        &b,
        ParserConfiguration {
            rdf: RDFParserConfiguration { lax: true },
            ..Default::default()
        },
    ).map(|(o, _)| (SetOntology::from(o), Default::default()))
}

/// open_ontology_from_file(path: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> PyIndexedOntology
///
/// Opens an ontology from a file
///
/// If the serialization is not specified it is guessed from the file extension. Defaults to OWL/XML.
#[pyfunction(signature = (path, serialization = None))]
fn open_ontology_from_file(path: String, serialization: Option<&str>) -> PyResult<PyIndexedOntology> {
    let serialization = guess_serialization(&path, serialization)?;

    let file = File::open(path)?;
    let mut f = BufReader::new(file);

    let b = Build::new_arc();

    let (onto, mapping) = match serialization {
        ResourceType::OFN => open_ontology_ofn(&mut f, &b),
        ResourceType::OWX => open_ontology_owx(&mut f, &b),
        ResourceType::RDF => open_ontology_rdf(&mut f, &b)
    }.map_err(to_py_err!("Failed to open ontology"))?;

    let iro = IRIMappedOntology::from(onto);
    let mut lo = PyIndexedOntology::from(iro);
    lo.mapping = mapping; //Needed when saving
    Ok(lo)
}

/// open_ontology_from_string(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> PyIndexedOntology
///
/// Opens an ontology from plain text.
///
/// If no serialization is specified, all parsers are tried until one succeeds
#[pyfunction(signature = (ontology, serialization = None))]
fn open_ontology_from_string(ontology: String, serialization: Option<&str>) -> PyResult<PyIndexedOntology> {
    let serialization = parse_serialization(serialization);
    let mut f = BufReader::new(ontology.as_bytes());

    let b = Build::new_arc();

    let (onto, mapping) = match serialization {
        Some(ResourceType::OFN) => open_ontology_ofn(&mut f, &b),
        Some(ResourceType::OWX) => open_ontology_owx(&mut f, &b),
        Some(ResourceType::RDF) => open_ontology_rdf(&mut f, &b),
        None => open_ontology_ofn(&mut f, &b)
            .or_else(|_| open_ontology_rdf(&mut f, &b))
            .or_else(|_| open_ontology_owx(&mut f, &b))
    }.map_err(to_py_err!("Failed to open ontology"))?;

    let iro = IRIMappedOntology::from(onto);
    let mut lo = PyIndexedOntology::from(iro);
    lo.mapping = mapping; //Needed when saving
    Ok(lo)
}

/// open_ontology(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> PyIndexedOntology
///
/// Opens an ontology from a path or plain text.
///
/// If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interpreted as an ontology
/// in plain text.
/// If no serialization is specified the serialization is guessed by the file extension or all parsers are tried
/// until one succeeds.
#[pyfunction(signature = (ontology, serialization = None))]
fn open_ontology(ontology: String, serialization: Option<&str>) -> PyResult<PyIndexedOntology> {
    if Path::exists(ontology.as_ref()) {
        open_ontology_from_file(ontology, serialization)
    } else {
        open_ontology_from_string(ontology, serialization)
    }
}

#[pymodule]
fn pyhornedowl(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyIndexedOntology>()?;

    m.add_function(wrap_pyfunction!(open_ontology, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(open_ontology_from_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_descendants, m)?)?;
    m.add_function(wrap_pyfunction!(get_ancestors, m)?)?;

    let model_sub_module = model::py_module(py)?;
    m.add_submodule(&model_sub_module)?;

    Ok(())
}
