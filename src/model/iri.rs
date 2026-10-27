use std::borrow::Borrow;
use std::fmt::Display;

use horned_owl::model::{ArcStr, Build};
use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyType};
use pyo3::{intern, type_hint_union, PyTypeInfo};

use crate::wrappers::*;

use super::{AnnotationProperty, Class, DataProperty, Datatype, NamedIndividual, ObjectProperty};

/// An IRI.
///
/// Anywhere the model expects an IRI, it also takes a `str` (used as is, so
/// it should be an absolute IRI; CURIEs are only expanded by the ontology's
/// methods, which know its prefixes) or a `NamedEntity`, whose
/// `__owl_entity_iri__()` method must return an `IRI`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(
    module = "pyhornedowl.model",
    frozen,
    eq,
    ord,
    hash,
    str,
    skip_from_py_object
)]
pub struct IRI(pub(crate) horned_owl::model::IRI<ArcStr>);

impl IRI {
    pub fn new<A: Borrow<str>>(iri: A, build: &Build<ArcStr>) -> Self {
        IRI(build.iri(iri))
    }
}

impl Display for IRI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[pymethods]
impl IRI {
    #[new]
    fn py_new(value: &str) -> Self {
        IRI::new(value, &Build::new_arc())
    }

    #[classmethod]
    pub fn parse(_: &Bound<'_, PyType>, value: &str) -> Self {
        IRI::new(value, &Build::new_arc())
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!("IRI({})", PyString::new(py, &self.0).repr()?))
    }

    fn __reduce__<'py>(slf: &Bound<'py, Self>) -> (Bound<'py, PyType>, (String,)) {
        (slf.get_type(), (slf.get().0.to_string(),))
    }

    /// Renders this element on its own, in OWL functional (`"ofn"`, the
    /// default) or OWL 2 Manchester (`"omn"`) syntax. The per-element
    /// counterpart to `Ontology.save_to_string`.
    #[pyo3(signature = (serialization = None, prefix_mapping = None))]
    pub fn serialize(
        &self,
        serialization: Option<LiteralStr<SnippetSyntaxes>>,
        prefix_mapping: Option<&crate::prefix_mapping::PrefixMapping>,
    ) -> PyResult<String> {
        let value = self.0.clone();

        Ok(
            match crate::snippet::parse_syntax(serialization.as_deref().unwrap_or("ofn"))? {
                crate::snippet::SnippetSyntax::Manchester => crate::as_omn!(value, prefix_mapping),
                crate::snippet::SnippetSyntax::Functional => crate::as_ofn!(value, prefix_mapping),
            },
        )
    }
}

impl<'py> FromPyObject<'_, 'py> for IRI {
    const INPUT_TYPE: PyStaticExpr = type_hint_union!(
        IRI::TYPE_HINT,
        String::INPUT_TYPE,
        // Not the `NamedEntity` protocol: it is defined in Python, in the
        // module the stubs describe, which they cannot import from.
        Class::TYPE_HINT,
        ObjectProperty::TYPE_HINT,
        DataProperty::TYPE_HINT,
        AnnotationProperty::TYPE_HINT,
        NamedIndividual::TYPE_HINT,
        Datatype::TYPE_HINT
    );
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(iri) = ob.cast::<IRI>() {
            return Ok(iri.get().clone());
        }
        if let Ok(s) = ob.cast::<PyString>() {
            return Ok(IRI::new(s.to_str()?, &Build::new_arc()));
        }
        // Explicit opt-in: an ordinary `iri` field also occurs on variables,
        // imports and ontology identifiers, none of which are named entities.
        match ob.getattr(intern!(ob.py(), "__owl_entity_iri__")) {
            Ok(method) => {
                let value = method.call0()?;
                let iri = value.cast::<IRI>().map_err(|_| {
                    pyo3::exceptions::PyTypeError::new_err(
                        "__owl_entity_iri__() must return an IRI",
                    )
                })?;
                return Ok(iri.get().clone());
            }
            Err(err) if err.is_instance_of::<pyo3::exceptions::PyAttributeError>(ob.py()) => {}
            Err(err) => return Err(err),
        }
        Err(pyo3::exceptions::PyTypeError::new_err(format!(
            "expected an IRI, a str or a named entity, got {}",
            ob.get_type().name()?
        )))
    }
}

impl From<horned_owl::model::IRI<ArcStr>> for IRI {
    fn from(value: horned_owl::model::IRI<ArcStr>) -> Self {
        IRI(value)
    }
}

impl From<&horned_owl::model::IRI<ArcStr>> for IRI {
    fn from(value: &horned_owl::model::IRI<ArcStr>) -> Self {
        IRI(value.clone())
    }
}

impl From<IRI> for horned_owl::model::IRI<ArcStr> {
    fn from(value: IRI) -> Self {
        value.0
    }
}

impl From<&IRI> for horned_owl::model::IRI<ArcStr> {
    fn from(value: &IRI) -> Self {
        value.0.clone()
    }
}

impl From<IRI> for String {
    fn from(value: IRI) -> Self {
        value.0.to_string()
    }
}

impl FromCounterpart<horned_owl::model::IRI<ArcStr>> for IRI {
    fn from_counterpart(value: &horned_owl::model::IRI<ArcStr>) -> Self {
        value.into()
    }
}

impl FromCounterpart<IRI> for horned_owl::model::IRI<ArcStr> {
    fn from_counterpart(value: &IRI) -> Self {
        value.into()
    }
}
