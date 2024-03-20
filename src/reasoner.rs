use horned_owl::model::{ArcStr, Build};
use pyo3::{exceptions, pyclass, pymethods, PyResult, Python};
use whelk::whelk::reasoner::Whelk as WhelkR;
use horned_owl::reasoner::Reasoner;
use crate::ontology::PyIndexedOntology;
use crate::model::Component;



/// __new__(self, onto: pyhornedowl.PyIndexedOntology)
///
/// Creates a reasoner for a loaded ontology
#[pyclass(unsendable)]
pub struct Whelk(whelk::whelk::reasoner::Whelk<ArcStr>);

#[pymethods]
impl Whelk {
    #[new]
    pub fn new(onto: &PyIndexedOntology) -> Whelk {
        let whelk = WhelkR::<ArcStr>::for_ontology(&onto.ontology);

        return Whelk(whelk);
    }

    /// classify(self) -> None
    ///
    /// Calculates the class and role hierarchy. This method is expected to be called before calling
    /// any other method.
    pub fn classify(&mut self, _py: Python) -> PyResult<()> {
        let w =  &mut self.0;
        let r = w.classify();

        match r {
            Ok(_) => Ok(()),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }

    /// get_subclasses(self, iri: str) -> List[str]
    ///
    /// Return all asserted and inferred subclasses (direct and indirect) of a class.
    pub fn get_subclasses(&mut self, iri: String) -> PyResult<Vec<String>> {
        let b = Build::new_arc();
        let i = b.iri(iri);
        let result = self.0.get_subclasses(&i);

        match result {
            Ok(r) => Ok(r.iter().map(|x| x.0.to_string()).collect()),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }

    /// get_superclasses(self, iri: str) -> List[str]
    ///
    /// Return all asserted and inferred superclasses (direct and indirect) of a class.
    pub fn get_superclasses(&mut self, iri: String) -> PyResult<Vec<String>> {
        let b = Build::new_arc();
        let i = b.iri(iri);
        let result = self.0.get_superclasses(&i);

        match result {
            Ok(r) => Ok(r.iter().map(|x| x.0.to_string()).collect()),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }

    /// entails(self, axiom: pyhornedowl.model.Component) -> bool
    ///
    /// Check if the ontology entails a given axiom.
    fn entails(&mut self, axiom: Component) -> PyResult<bool> {
        let ax = horned_owl::model::Component::from(axiom);
        let result = self.0.entails(&ax);

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }

    /// consistent(self) -> bool
    ///
    /// Check whether the ontology is consistent
    fn consistent(&mut self) -> PyResult<bool> {
        let result = self.0.consistent();

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }

    /// inferred_axioms(self) -> List[pyhornedowl.model.Component]
    ///
    /// Get all axioms the reasoner could infer from the given ontology
    fn inferred_axioms(&mut self) -> PyResult<Vec<Component>> {
        let result = self.0.inferred_axioms();

        match result {
            Ok(r) => Ok(r.iter().map(|x| x.into()).collect()),
            Err(e) => Err(exceptions::PyValueError::new_err(e))
        }
    }
}


