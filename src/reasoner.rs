use std::sync::Arc;
use horned_owl::model::{ArcStr, Build, Class, MutableOntology};
use pyo3::{exceptions, IntoPy, Py, pyclass, PyErr, pymethods, PyRef, PyResult, Python};
use whelk::whelk::reasoner::Whelk as WhelkR;
use horned_owl::reasoner::Reasoner;
use crate::ontology::PyIndexedOntology;

#[pyclass(unsendable)]
pub struct Whelk(whelk::whelk::reasoner::Whelk<ArcStr>);

#[pymethods]
impl Whelk {
    #[new]
    pub fn new(onto: &PyIndexedOntology) -> Whelk {
        // let mut set_ontology = horned_owl::ontology::set::SetOntology::new();
        //
        // for ax in onto.ontology.iter(){
        //     set_ontology.insert(ax.clone());
        // }

        let whelk = WhelkR::<ArcStr>::for_ontology(&onto.ontology);

        return Whelk(whelk);
    }

    pub fn classify(&mut self, py: Python) -> PyResult<()> {
        let w =  &mut self.0;
        let r = w.classify();

        match r {
            Ok(_) => Ok(()),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }

    pub fn get_subclasses(&mut self, iri: String) -> PyResult<Vec<String>> {
        let b = Build::new_arc();
        let i = b.iri(iri);
        let result = self.0.get_subclasses(&i);

        match result {
            Ok(r) => Ok(r.iter().map(|x| x.0.to_string()).collect()),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }

    pub fn get_superclasses(&mut self, iri: String) -> PyResult<Vec<String>> {
        let b = Build::new_arc();
        let i = b.iri(iri);
        let result = self.0.get_superclasses(&i);

        match result {
            Ok(r) => Ok(r.iter().map(|x| x.0.to_string()).collect()),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }
}


