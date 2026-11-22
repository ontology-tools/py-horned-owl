use curie::Curie;
use pyo3::prelude::*;

use crate::{model, prefix_mapping::PrefixMapping, PyIndexedOntology};

#[pymethods]
impl PyIndexedOntology {
    /// Gets the ID of term by it IRI.
    ///
    /// If the term does not have an ID, `None` is returned.
    #[pyo3(signature = (iri))]
    pub fn get_id_for_iri(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<Option<String>> {
        let iri: String = into_iri!(self, py, iri).into();
        let mapping = self.mapping.borrow_mut(py);

        let res = mapping.0.shrink_iri(iri.as_str());

        if let Ok(curie) = res {
            Ok(Some(curie.to_string()))
        } else {
            //Return null
            Ok(None)
        }
    }

    /// Gets the IRI of a term by its ID.
    ///
    /// If the term does not have an IRI, `None` is returned.
    pub fn get_iri_for_id(&mut self, py: Python<'_>, id: String) -> Option<String> {
        let idparts: Vec<&str> = id.split(":").collect();

        if idparts.len() == 2 {
            let curie = Curie::new(Some(idparts[0]), idparts[1]);

            let mapping = self.mapping.borrow_mut(py);
            let res = mapping.0.expand_curie(&curie);

            res.ok().map(|iri| iri.to_string())
        } else {
            //Not a CURIE, at least not of the form PREFIX:NUMBER
            None
        }
    }

    /// The prefix mapping
    #[getter]
    pub fn get_prefix_mapping<'py>(&self, py: Python<'py>) -> PyResult<&Bound<'py, PrefixMapping>> {
        Ok(self.mapping.bind(py))
    }

    /// Adds the prefix `iriprefix`.
    pub fn add_prefix_mapping<'py>(
        &mut self,
        py: Python<'py>,
        iriprefix: String,
        mappedid: String,
    ) -> PyResult<()> {
        self.mapping.borrow_mut(py).add_prefix(iriprefix, mappedid)
    }
}
