use crate::model::{ClassExpression, Component};
use crate::PyIndexedOntology;
use horned_owl::model::{ArcAnnotatedComponent, ArcStr};
use horned_owl::ontology::set::SetOntology;
use libloading;
use libloading::Library;
use pyhornedowlreasoner::Reasoner;
use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, pyfunction, pymethods, PyErr, PyResult};
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct DynamicLoadedReasoner(
    /// The reasoner instance created by the `create_reasoner` function exposed by the reasoner library
    pub(crate) Box<dyn Reasoner<ArcStr, ArcAnnotatedComponent>>,
    /// The loaded library of the reasoner. Must be only be dropped when the reasoner is dropped as the library is unloaded when dropped.
    #[allow(dead_code)]
    Box<Library>,
);

#[derive(Clone)]
#[pyclass]
pub struct PyReasoner(pub(crate) Arc<Mutex<DynamicLoadedReasoner>>);

unsafe impl Send for PyReasoner {}
unsafe impl Sync for PyReasoner {}

#[pymethods]
impl PyReasoner {
    /// get_name(self) -> str
    ///
    /// Returns the name of the reasoner.
    fn get_name(&self) -> String {
        self.0.lock().unwrap().0.get_name()
    }

    /// get_version(self) -> str
    ///
    /// Returns the version of the reasoner.
    fn get_version(&self) -> String {
        self.0.lock().unwrap().0.get_version()
    }

    /// flush(self) -> None
    ///
    /// Flushes pending changes to the reasoner. This invalidates any cached results and updates the reasoner with the current state of the ontology.
    fn flush(&self) -> PyResult<()> {
        self.0
            .lock()
            .unwrap()
            .0
            .flush()
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// inferred_axioms(self) -> Set[Component]
    ///
    /// Returns a set of inferred axioms from the reasoner.
    fn inferred_axioms(&self) -> HashSet<Component> {
        self.0
            .lock()
            .unwrap()
            .0
            .inferred_axioms()
            .into_iter()
            .map(|c| c.into())
            .collect()
    }

    /// is_consistent(self) -> bool
    ///
    /// Checks if the ontology is consistent.
    fn is_consistent(&self) -> PyResult<bool> {
        self.0
            .lock()
            .unwrap()
            .0
            .is_consistent()
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// is_entailed(self, cmp: Component) -> bool
    ///
    /// Checks if the ontology entails the given component.
    fn is_entailed(&self, cmp: Component) -> PyResult<bool> {
        self.0
            .lock()
            .unwrap()
            .0
            .is_entailed(&horned_owl::model::Component::<ArcStr>::from(cmp.clone()))
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// is_satifisable(self, cmp: ClassExpression) -> bool
    ///
    /// Checks if the given class expression is satisfiable.
    fn is_satifisable(&self, cmp: ClassExpression) -> PyResult<bool> {
        self.0
            .lock()
            .unwrap()
            .0
            .is_satifisable(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// get_unsatisfiable_classes(self) -> Set[ClassExpression]
    ///
    /// Returns the set of unsatisfiable classes.
    fn get_unsatisfiable_classes(&self) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .lock()
            .unwrap()
            .0
            .get_unsatisfiable_classes()
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// get_subclasses(self, cmp: ClassExpression) -> Set[ClassExpression]
    ///
    /// Returns the set of asserted and inferred subclasses for the given class expression.
    fn get_subclasses(&self, cmp: ClassExpression) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .lock()
            .unwrap()
            .0
            .get_subclasses(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// get_superclasses(self, cmp: ClassExpression) -> Set[ClassExpression]
    ///
    /// Returns the set of asserted and inferred superclasses for the given class expression.
    fn get_superclasses(&self, cmp: ClassExpression) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .lock()
            .unwrap()
            .0
            .get_superclasses(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// get_equivalent_classes(self, cmp: ClassExpression) -> Set[ClassExpression]
    ///
    /// Returns the set of classes asserted or inferred to be equivalent to given the class expression.
    fn get_equivalent_classes(&self, cmp: ClassExpression) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .lock()
            .unwrap()
            .0
            .get_equivalent_classes(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// get_disjoint_classes(self, cmp: ClassExpression) -> Set[ClassExpression]
    ///
    /// Returns the set of classes asserted or inferred to be disjoint with the given class expression.
    fn get_disjoint_classes(&self, cmp: ClassExpression) -> PyResult<HashSet<ClassExpression>> {
        self.0
            .lock()
            .unwrap()
            .0
            .get_disjoint_classes(&horned_owl::model::ClassExpression::<ArcStr>::from(
                cmp.clone(),
            ))
            .map(|s| s.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
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
pub fn create_reasoner(name: String, ontology: &mut PyIndexedOntology) -> PyResult<PyReasoner> {
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

    let lib = unsafe { Library::new(std::path::absolute(&path)?).map_err(to_py_err)? };

    let reasoner: Box<dyn Reasoner<ArcStr, ArcAnnotatedComponent>> = unsafe {
        let func: libloading::Symbol<
            fn(ontology: SetOntology<ArcStr>) -> Box<dyn Reasoner<ArcStr, ArcAnnotatedComponent>>,
        > = lib.get(b"create_reasoner").map_err(to_py_err)?;
        func(ontology.clone().into())
    };

    let py_reasoner = DynamicLoadedReasoner(reasoner, Box::new(lib));
    Ok(ontology.add_reasoner(py_reasoner))
}
