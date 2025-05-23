use crate::model::{ClassExpression, Component};
use crate::wrappers::BTreeSetWrap;
use crate::{model, IndexCreationStrategy, PyIndexedOntology};
use horned_owl::model::{AnnotatedComponent, ArcStr};
use horned_owl::ontology::set::SetOntology;
use libloading;
use libloading::Library;
use pyhornedowlreasoner::Reasoner2;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::PyAnyMethods;
use pyo3::{pyclass, pyfunction, pymethods, Bound, PyAny, PyErr, PyResult};
use std::collections::{BTreeSet, HashSet};
use std::env;
use std::path::PathBuf;

/// A pyo3 compatible wrapper for all reasoner implementations
#[pyclass(unsendable)]
pub struct PyReasoner(
    /// The reasoner instance created by the `create_reasoner` function exposed by the reasoner library
    Box<dyn Reasoner2>,
    /// The loaded library of the reasoner. Must be only be dropped when the reasoner is dropped as the library is unloaded when dropped.
    #[allow(dead_code)]
    Box<Library>,
);

#[pymethods]
impl PyReasoner {
    fn get_name(&self) -> String {
        self.0.get_name()
    }
    fn get_version(&self) -> String {
        self.0.get_version()
    }

    fn is_consistent(&self) -> PyResult<bool> {
        self.0
            .is_consistent()
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    fn is_entailed(&self, cmp: Component) -> PyResult<bool> {
        self.0
            .is_entailed(&horned_owl::model::Component::<ArcStr>::from(cmp.clone()))
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    fn is_satifisable(&self, cmp: ClassExpression) -> PyResult<bool> {
        self.0
            .is_satifisable(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    fn get_unsatisfiable_classes(&self) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .get_unsatisfiable_classes()
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    fn get_subclasses(&self, cmp: ClassExpression) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .get_subclasses(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    fn get_superclasses(&self, cmp: ClassExpression) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .get_superclasses(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    fn get_equivalent_classes(&self, cmp: ClassExpression) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .get_equivalent_classes(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    fn get_disjoint_classes(&self, cmp: ClassExpression) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .get_disjoint_classes(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    fn get_current_ontology(&self) -> PyResult<PyIndexedOntology> {
        Ok(PyIndexedOntology::from_set_ontology(
            self.0.get_ontology(),
            IndexCreationStrategy::OnQuery,
        ))
    }

    /// add_component(self, component: model.Component, annotations: Optional[Union[List[model.Annotation]|Set[model.Annotation]]]=None) -> None
    ///
    /// Adds an axiom to the ontology with optional annotations.
    #[pyo3(signature = (component, annotations = None))]
    pub fn add_component(
        &mut self,
        component: model::Component,
        annotations: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<model::AnnotatedComponent> {
        let ann: BTreeSetWrap<model::Annotation> = match annotations {
            Some(a) => a
                .extract::<BTreeSet<model::Annotation>>()
                .or_else(|_| {
                    Ok::<BTreeSet<model::Annotation>, PyErr>(
                        a.extract::<Vec<model::Annotation>>()?.into_iter().collect(),
                    )
                })?
                .into(),
            None => BTreeSet::new(),
        }
        .into();

        let annotated_component: AnnotatedComponent<ArcStr> =
            model::AnnotatedComponent { component, ann }.into();
        self.0.index_insert(annotated_component.clone());

        Ok(annotated_component.into())
    }

    /// remove_component(self, component: model.Component) -> bool
    ///
    /// Removes a component from the ontology.
    pub fn remove_component(&mut self, component: model::AnnotatedComponent) -> PyResult<bool> {
        Ok(self.0.index_remove(&component.into()))
    }
}

fn to_py_err(e: libloading::Error) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to load reasoner: {}", e))
}

/// create_reasoner(name: str, ontology: PyIndexedOntology) -> PyReasoner
///
/// Loads a reasoner from a shared library.
///
/// :param str name: name of the reasoner to load or path to the shared library to load
#[pyfunction(signature = (name, ontology))]
pub fn create_reasoner(name: String, ontology: PyIndexedOntology) -> PyResult<PyReasoner> {
    let mut path = PathBuf::from(&name);

    if !path.exists() && !name.contains(std::path::MAIN_SEPARATOR) {
        let filename = match env::consts::OS {
            "linux" => Ok(format!("lib{}.so", name)),
            "windows" => Ok(format!("{}.dll", name)),
            "macos" => Ok(format!("lib{}.dylib", name)),
            _ => Err(PyErr::new::<PyValueError, _>(
                "Unsupported OS for automatically detecting reasoner path",
            )),
        }?;

        path = env::current_dir()?.join(&filename);
    }

    if !path.exists() {
        return Err(PyErr::new::<PyValueError, _>(format!(
            "Reasoner library not found: {}",
            path.display()
        )));
    }

    let lib = unsafe { libloading::Library::new(std::path::absolute(&path)?).map_err(to_py_err)? };

    let reasoner: Box<dyn Reasoner2> = unsafe {
        let func: libloading::Symbol<fn(ontology: SetOntology<ArcStr>) -> Box<dyn Reasoner2>> =
            lib.get(b"create_reasoner").map_err(to_py_err)?;
        func(ontology.into())
    };

    let py_reasoner = PyReasoner(reasoner, Box::new(lib));
    Ok(py_reasoner)
}
