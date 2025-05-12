use crate::model::Component;
use crate::{IndexCreationStrategy, PyIndexedOntology};
use libloading;
use pyhornedowlreasoner::Reasoner;
use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, pyfunction, pymethods, PyErr, PyResult};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use libloading::Library;
use std::env;

/// A pyo3 compatible wrapper for all reasoner implementations
#[pyclass(unsendable)]
pub struct PyReasoner(
    /// The reasoner instance created by the `create_reasoner` function exposed by the reasoner library
    Box<dyn Reasoner>,
    /// The loaded library of the reasoner. Must be only be dropped when the reasoner is dropped as the library is unloaded when dropped.
    Box<Library>,
);

#[pymethods]
impl PyReasoner {
    /// classify(self, o: PyIndexedOntology) -> PyIndexedOntology
    ///
    /// Classifies the given ontology using the reasoner.
    pub fn classify(&self, o: &PyIndexedOntology) -> PyResult<PyIndexedOntology> {
        self.0
            .classify(&o.into())
            .map(|o| PyIndexedOntology::from_set_ontology(o, IndexCreationStrategy::OnLoad))
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}", e)))
    }

    /// infer(self, o: PyIndexedOntology) -> set[Component]
    ///
    /// Infers axioms from the given ontology using the reasoner.
    pub fn infer(&self, o: &PyIndexedOntology) -> PyResult<HashSet<Component>> {
        self.0
            .infer(&o.into())
            .map(|c| c.into_iter().map(|c| c.into()).collect())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }

    /// consistency(self, o: PyIndexedOntology) -> bool
    ///
    /// Checks if the given ontology is consistent using the reasoner.
    pub fn consistency(&self, o: &PyIndexedOntology) -> PyResult<bool> {
        self.0
            .consistency(&o.into())
            .map_err(|e| PyErr::new::<PyValueError, _>(format!("{:?}s", e)))
    }
}

fn to_py_err(e: libloading::Error) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to load reasoner: {}", e))
}

/// load_reasoner(name: str) -> PyReasoner
///
/// Loads a reasoner from a shared library.
///
/// :param str name: name of the reasoner to load or path to the shared library to load
#[pyfunction(signature = (name))]
pub fn load_reasoner(name: String) -> PyResult<PyReasoner> {
    let mut path = PathBuf::from(&name);

    if !path.exists() && !name.contains(std::path::MAIN_SEPARATOR) {
        let filename = match env::consts::OS {
            "linux" => Ok(format!("lib{}.so", name)),
            "windows" => Ok(format!("{}.dll", name)),
            "macos" => Ok(format!("lib{}.dylib", name)),
            _ => Err(PyErr::new::<PyValueError, _>("Unsupported OS for automatically detecting reasoner path")),
        }?;

        path = env::current_dir()?.join(&filename);
    }

    if !path.exists() {
        return Err(PyErr::new::<PyValueError, _>(format!(
            "Reasoner library not found: {}",
            path.display()
        )));
    }

    let lib = unsafe {
        libloading::Library::new(std::path::absolute(&path)?)
        .map_err(to_py_err)?
    };

    let reasoner: Box<dyn Reasoner> = unsafe {
        let func: libloading::Symbol<fn() -> Box<dyn Reasoner>> =
            lib.get(b"create_reasoner").map_err(to_py_err)?;
        func()
    };

    let py_reasoner = PyReasoner(reasoner, Box::new(lib));
    Ok(py_reasoner)
}
