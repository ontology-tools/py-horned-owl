//! The Python protocol shared by all model classes, as ordinary generic
//! functions. The `#[model]` macro (`lib/macros`) generates the
//! [`ModelFields`] impl and methods that forward here.

use horned_owl::model::Build;
use pyo3::prelude::*;
use pyo3::pyclass::boolean_struct::True;
use pyo3::types::{PyDict, PyString, PyTuple, PyType};
use pyo3::PyClass;

use super::IRI;

/// A model class's fields, in constructor order.
pub trait ModelFields: PyClass<Frozen = True> + Sync + Clone {
    const FIELDS: &'static [&'static str];

    fn field_values<'py>(&self, py: Python<'py>) -> PyResult<Vec<Bound<'py, PyAny>>>;
}

/// A model class that is one variant of the horned-owl enum `H`.
pub trait EnumVariant<H> {
    fn to_horned(&self) -> H;

    /// Panics if `value` is another variant.
    fn from_horned(value: &H) -> Self;
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
) -> PyResult<T> {
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
        .cast_into::<T>()?
        .get()
        .clone())
}

/// A `str` where an entity is expected is its IRI, used as is: CURIEs are
/// only expanded by the ontology's methods, which know its prefixes.
pub fn entity_from_str<T: From<IRI>>(ob: &Bound<'_, PyAny>) -> PyResult<Option<T>> {
    Ok(match ob.cast::<PyString>() {
        Ok(s) => Some(IRI::new(s.to_str()?, &Build::new_arc()).into()),
        Err(_) => None,
    })
}
