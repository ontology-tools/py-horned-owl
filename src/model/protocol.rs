//! The Python protocol shared by all model classes, as ordinary generic
//! functions. The `#[model]` macro (`lib/macros`) generates the
//! [`ModelFields`] impl and methods that forward here.

use horned_owl::model::Build;
use pyo3::inspect::PyStaticExpr;
use pyo3::prelude::*;
use pyo3::pyclass::boolean_struct::True;
use pyo3::types::{PyDict, PyString, PyTuple, PyType};
use pyo3::{type_hint_union, PyClass, PyTypeInfo};

use super::{EntityKind, IRI};

/// Read the IRI hook without requiring a kind, preserving IRI-only wrappers.
pub fn named_entity_iri(value: &Bound<'_, PyAny>) -> PyResult<IRI> {
    let method = match value.getattr("__owl_entity_iri__") {
        Ok(method) => method,
        Err(err) if err.is_instance_of::<pyo3::exceptions::PyAttributeError>(value.py()) => {
            return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                "expected an IRI, a str or a named entity implementing __owl_entity_iri__(), got {}",
                value.get_type().name()?
            )));
        }
        Err(err) => return Err(err),
    };
    let iri = method.call0()?;
    Ok(iri
        .cast::<IRI>()
        .map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("__owl_entity_iri__() must return an IRI")
        })?
        .get()
        .clone())
}

/// Resolve the existing NamedEntity protocol without an argument wrapper type.
pub fn named_entity(value: &Bound<'_, PyAny>) -> PyResult<(EntityKind, IRI)> {
    let method = value.getattr("entity_kind").map_err(|err| {
        if err.is_instance_of::<pyo3::exceptions::PyAttributeError>(value.py()) {
            pyo3::exceptions::PyTypeError::new_err(
                "expected a NamedEntity with entity_kind() and __owl_entity_iri__()",
            )
        } else {
            err
        }
    })?;
    let kind = method.call0()?;
    let kind = *kind
        .cast::<EntityKind>()
        .map_err(|_| {
            pyo3::exceptions::PyTypeError::new_err("entity_kind() must return an EntityKind")
        })?
        .get();
    Ok((kind, named_entity_iri(value)?))
}

/// A model class's fields, in constructor order.
pub trait ModelFields: PyClass<Frozen = True> + Sync + Clone {
    const FIELDS: &'static [&'static str];

    fn field_values<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyAny>>>;
}

pub fn match_args<T: ModelFields>(py: Python<'_>) -> PyResult<Bound<'_, PyTuple>> {
    PyTuple::new(py, T::FIELDS)
}

/// `Name(field=repr(value), ...)`, which `eval`s back to an equal object.
pub fn repr<T: ModelFields>(slf: &Bound<'_, T>) -> PyResult<String> {
    let values = slf.get().field_values(slf.py())?;
    let fields = T::FIELDS
        .iter()
        .zip(values)
        .map(|(name, value)| Ok(format!("{}={}", name, value.repr()?)))
        .collect::<PyResult<Vec<_>>>()?;
    Ok(format!(
        "{}({})",
        slf.as_any().get_type().name()?,
        fields.join(", ")
    ))
}

/// Pickling: the class and its constructor arguments.
pub fn reduce<'py, T: ModelFields>(
    slf: &Bound<'py, T>,
) -> PyResult<(Bound<'py, PyType>, Bound<'py, PyTuple>)> {
    let values = slf.get().field_values(slf.py())?;
    Ok((slf.as_any().get_type(), PyTuple::new(slf.py(), values)?))
}

/// `copy.replace`: a new object from the current fields and `changes`.
pub fn replace<'py, T: ModelFields>(
    slf: &Bound<'py, T>,
    changes: Option<&Bound<'py, PyDict>>,
) -> PyResult<Bound<'py, T>> {
    let kwargs = PyDict::new(slf.py());
    for (name, value) in T::FIELDS.iter().zip(slf.get().field_values(slf.py())?) {
        kwargs.set_item(name, value)?;
    }
    if let Some(changes) = changes {
        kwargs.update(changes.as_mapping())?;
    }
    Ok(slf
        .as_any()
        .get_type()
        .call((), Some(&kwargs))?
        .cast_into::<T>()?)
}

/// Convert an IRI or a string to a model entity.
/// Returns `None` if the argument is neither an IRI nor a string.
pub fn entity_from_iri<T: From<IRI>>(ob: &Bound<'_, PyAny>) -> PyResult<Option<T>> {
    if let Ok(iri) = ob.cast::<IRI>() {
        return Ok(Some(iri.get().clone().into()));
    }

    if let Ok(s) = ob.cast::<PyString>() {
        return Ok(Some(IRI::new(s.to_str()?, &Build::new_arc()).into()));
    }

    Ok(None)
}

/// The stub type of what [`entity_from_iri`] accepts, for `accepts = [...]`.
pub const IRI_OR_STR: PyStaticExpr = type_hint_union!(IRI::TYPE_HINT, String::INPUT_TYPE);
