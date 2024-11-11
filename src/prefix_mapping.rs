use pyo3::{
    exceptions::{PyKeyError, PyValueError},
    prelude::*,
};

use crate::to_py_err;

#[pyclass(mapping)]
pub struct PrefixMapping(pub(crate) curie::PrefixMapping);

impl Default for PrefixMapping {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl From<curie::PrefixMapping> for PrefixMapping {
    fn from(value: curie::PrefixMapping) -> Self {
        PrefixMapping(value)
    }
}

impl From<PrefixMapping> for curie::PrefixMapping {
    fn from(value: PrefixMapping) -> Self {
        value.0
    }
}

#[pyclass]
struct Iter {
    inner: std::vec::IntoIter<(String, String)>
}

#[pymethods]
impl Iter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(String, String)> {
        slf.inner.next()
    }
}

#[pymethods]
impl PrefixMapping {

    /// __iter__(self) -> typing.Iterable[typing.Tuple[str, str]]
    ///
    /// Get an iterator over all prefixes
    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<Iter>> {
        let iter = Iter {
            inner: slf.0.mappings().map(|(x,y)| ((x.clone(), y.clone()))).collect::<Vec<(String, String)>>().into_iter(),
        };
        Py::new(slf.py(), iter)
    }

    pub fn __getitem__(&self, key: &str) -> PyResult<String> {
        self.0
            .expand_curie(&curie::Curie::new(Some(key), ""))
            .map_err(|_| PyKeyError::new_err(format!("No such prefix '{}'", key)))
    }

    pub fn __contains__(&self, key: &str) -> PyResult<bool> {
        Ok(self.__getitem__(key).is_ok())
    }

    pub fn __setitem__(&mut self, key: &str, value: &str) -> PyResult<()> {
        self.0
            .add_prefix(key, value)
            .map_err(|e| PyKeyError::new_err(format!("Invalid prefix '{}': {:?}", key, e)))
    }

    pub fn __delitem__(&mut self, key: &str) -> PyResult<()> {
        Ok(self.0.remove_prefix(key))
    }

    pub fn __len__(&self) -> usize {
        self.0.mappings().len()
    }

    /// add_default_prefix_names(self) -> None
    ///
    /// Adds the prefix for rdf, rdfs, xsd, and owl
    pub fn add_default_prefix_names(&mut self) -> PyResult<()> {
        self.0
            .add_prefix("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")
            .map_err(to_py_err!("Error while adding predefined prefix 'rdf'"))?;
        self.0
            .add_prefix("rdfs", "http://www.w3.org/2000/01/rdf-schema#")
            .map_err(to_py_err!("Error while adding predefined prefix 'rdfs'"))?;
        self.0
            .add_prefix("xsd", "http://www.w3.org/2001/XMLSchema#")
            .map_err(to_py_err!("Error while adding predefined prefix 'xsd'"))?;
        self.0
            .add_prefix("owl", "http://www.w3.org/2002/07/owl#")
            .map_err(to_py_err!("Error while adding predefined prefix 'owl'"))?;

        Ok(())
    }

    /// add_prefix(self, iriprefix: str, mappedid: str) -> None
    ///
    /// Adds the prefix `iriprefix`.
    pub fn add_prefix(&mut self, iriprefix: String, mappedid: String) -> PyResult<()> {
        let result = self.0.add_prefix(&iriprefix, &mappedid);
        result.map_err(to_py_err!("Error - prefix is invalid."))?;

        if iriprefix.is_empty() {
            self.0.set_default(mappedid.as_str());
        }

        Ok(())
    }

    /// remove_prefix(self, iriprefix: str) -> None
    ///
    /// Remove a prefix from the mapping.
    pub fn remove_prefix(&mut self, prefix: &str) {
        self.0.remove_prefix(prefix);

        if prefix == "" {
            let mut new_mapping = curie::PrefixMapping::default();
            for (p, v) in self.0.mappings() {
                new_mapping.add_prefix(p, v).expect("Cannot happen since self.0 contains only valid prefix mappings");
            }

            self.0 = new_mapping;
        }
    }

    /// expand_curie(self, curie: str) -> str
    ///
    /// Expands a curie. Throws a ValueError if the prefix is invalid or unknown
    pub fn expand_curie(&self, curie: &str) -> PyResult<String> {
        self.0
            .expand_curie_string(curie)
            .map_err(to_py_err!("Invalid or unknown prefix"))
    }

    /// shrink_iri(self, iri: str) -> str
    ///
    /// Shrinks an absolute IRI to a CURIE. Throws a ValueError on failure
    pub fn shrink_iri(&self, iri: &str) -> PyResult<String> {
        self.0
            .shrink_iri(iri)
            .map(|c| c.to_string())
            .map_err(|e| PyValueError::new_err(e))
    }
}
