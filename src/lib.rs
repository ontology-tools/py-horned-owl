use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::BufReader;

#[macro_use]
mod doc;
mod model;
mod reasoner;
mod ontology;

use crate::ontology::{PyIndexedOntology, get_descendants, get_ancestors};
use crate::reasoner::{Whelk};

use horned_owl::model::*;
use horned_owl::ontology::iri_mapped::IRIMappedOntology;
use horned_owl::io::rdf::reader::IncompleteParse;
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration};
use horned_owl::error::HornedError;
use horned_owl::io::rdf::reader::RDFOntology;
use horned_owl::ontology::set::SetOntology;

use curie::PrefixMapping;


use std::path::Path;
//use failure::Error;
use std::sync::Arc;


fn open_ontology_owx(
    ontology: &str,
    b: &Build<Arc<str>>,
) -> Result<(SetOntology<ArcStr>, PrefixMapping), HornedError> {
    let r = if Path::new(&ontology).exists() {
        let file = File::open(ontology).ok().unwrap();
        let mut f = BufReader::new(file);
        horned_owl::io::owx::reader::read_with_build(&mut f, &b)
    } else {
        //just try to parse the string
        let str_val = ontology.as_bytes();
        let mut f = BufReader::new(str_val);
        horned_owl::io::owx::reader::read_with_build(&mut f, &b)
    };
    r
}

fn open_ontology_rdf(
    ontology: &str,
    b: &Build<Arc<str>>,
) -> Result<
    (
        RDFOntology<ArcStr, Arc<AnnotatedComponent<ArcStr>>>,
        IncompleteParse<Arc<str>>,
    ),
    HornedError,
> {
    let r = if Path::new(&ontology).exists() {
        let file = File::open(ontology).ok().unwrap();
        let mut f = BufReader::new(file);
        horned_owl::io::rdf::reader::read_with_build(&mut f, &b, ParserConfiguration::default())
    } else {
        //just try to parse the string
        let str_val = ontology.as_bytes();
        let mut f = BufReader::new(str_val);
        horned_owl::io::rdf::reader::read_with_build(
            &mut f,
            &b,
            ParserConfiguration {
                rdf: RDFParserConfiguration { lax: true },
                ..Default::default()
            },
        )
    };
    r
}


/// open_ontology(ontology: str) -> PyIndexedOntology
///
/// Opens an ontology from a path or plain text.
///
/// If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interepreted as an ontology in either owx or owl format.
/// Note: Only .owl and .owx files are currently supported.
#[pyfunction]
fn open_ontology(ontology: &PyString) -> PyResult<PyIndexedOntology> {
    let ontology: String = ontology.extract().unwrap();

    let b = Build::new_arc();

    let result = if ontology.ends_with("owx") {
        let r = open_ontology_owx(&ontology, &b);
        //println!("Got result {:?}",r);
        match r {
            Ok((o, m)) => {
                //println!("Got ontology from owx {:?}",o);
                //println!("About to build indexes");
                let iro = IRIMappedOntology::from(o);
                let mut lo = PyIndexedOntology::from(iro);
                lo.mapping = m; //Needed when saving
                Ok(lo)
            }
            Err(e) => {
                Err(PyValueError::new_err(format!("Unable to open ontology: {}", e.to_string())))
            }
        }
    } else if ontology.ends_with("owl") {
        let r2 = open_ontology_rdf(&ontology, &b);
        if r2.is_ok() {
            let (o, _) = r2.ok().unwrap();
            //println!("Got ontology from rdf {:?}",o);
            let so = SetOntology::from(o);
            let iro = IRIMappedOntology::from(so);
            let lo = PyIndexedOntology::from(iro);
            Ok(lo)
        } else {
            Err(PyValueError::new_err("Unable to open ontology"))
        }
    } else {
        // No recognised suffix, maybe it is a string value, just try to parse
        let r = open_ontology_owx(&ontology, &b);
        if r.is_ok() {
            let (o, m) = r.ok().unwrap();
            //println!("Got ontology from owx {:?}",o);
            //println!("About to build indexes");
            let iro = IRIMappedOntology::from(o);
            let mut lo = PyIndexedOntology::from(iro);
            lo.mapping = m; //Needed when saving
            Ok(lo)
        } else {
            let r2 = open_ontology_rdf(&ontology, &b);
            if r2.is_ok() {
                let (o, _) = r2.ok().unwrap();
                //println!("Got ontology from rdf {:?}",o);
                let so = SetOntology::from(o);
                let iro = IRIMappedOntology::from(so);
                let lo = PyIndexedOntology::from(iro);
                Ok(lo)
            } else {
                Err(PyValueError::new_err("Unable to open ontology"))
            }
        }
    };
    result
}

#[pymodule]
fn pyhornedowl(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyIndexedOntology>()?;

    m.add_function(wrap_pyfunction!(open_ontology, m)?)?;
    m.add_function(wrap_pyfunction!(get_descendants, m)?)?;
    m.add_function(wrap_pyfunction!(get_ancestors, m)?)?;

    let model_sub_module = model::py_module(py)?;
    m.add_submodule(model_sub_module)?;

    let reasoner_sub_module = PyModule::new(py, "reasoner")?;
    reasoner_sub_module.add_class::<Whelk>()?;
    m.add_submodule(reasoner_sub_module)?;

    Ok(())
}
