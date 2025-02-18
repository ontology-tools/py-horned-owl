use pyo3::{FromPyObject, IntoPy};
use std::collections::BTreeSet;
use std::sync::Arc;
use std::hash::Hash;

pub trait FromCompatible<T> {
    fn from_c(value: T) -> Self;
}

pub trait IntoCompatible<T> {
    fn into_c(self) -> T;
}

impl<T, U> IntoCompatible<U> for T
where
    U: FromCompatible<T>,
{
    fn into_c(self) -> U {
        U::from_c(self)
    }
}

impl FromCompatible<&Arc<str>> for StringWrapper {
    fn from_c(value: &Arc<str>) -> Self {
        StringWrapper::from(value)
    }
}

impl FromCompatible<&StringWrapper> for Arc<str> {
    fn from_c(value: &StringWrapper) -> Self {
        Arc::<str>::from(value)
    }
}

impl FromCompatible<&String> for String {
    fn from_c(value: &String) -> Self {
        String::from(value)
    }
}

impl FromCompatible<&u32> for u32 {
    fn from_c(value: &u32) -> Self {
        value.clone()
    }
}

impl<'a, T: 'a, U> FromCompatible<&'a Option<T>> for Option<U>
    where
    U: FromCompatible<&'a T>,
{
    fn from_c(value: &'a Option<T>) -> Self {
        match value {
            None => None,
            Some(x) => Some(U::from_c(x)),
        }
    }
}

impl<U, V, S, T> FromCompatible<(S, T)> for (U, V)
    where
        U: FromCompatible<S>,
    V: FromCompatible<T>,
{
    fn from_c(value: (S, T)) -> Self {
        match value {
            (s, t) => (U::from_c(s), V::from_c(t)),
        }
    }
}

impl<'a, U, V, S, T> FromCompatible<&'a (S, T)> for (U, V)
    where
        U: FromCompatible<&'a S>,
    V: FromCompatible<&'a T>,
{
    fn from_c(value: &'a (S, T)) -> Self {
        match value {
            (s, t) => (U::from_c(s), V::from_c(t)),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VecWrap<T>(pub Vec<T>);

impl<T> From<Vec<T>> for VecWrap<T> {
    fn from(value: Vec<T>) -> Self {
        VecWrap(value)
    }
}

impl<T> From<VecWrap<T>> for Vec<T> {
    fn from(value: VecWrap<T>) -> Self {
        value.0
    }
}

impl<'source, T: FromPyObject<'source>> FromPyObject<'source> for VecWrap<T> {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        ob.extract().map(VecWrap)
    }
}

impl<T: IntoPy<pyo3::PyObject>> IntoPy<pyo3::PyObject> for VecWrap<T> {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.0.into_py(py)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoxWrap<T>(pub Box<T>);

impl<T> From<Box<T>> for BoxWrap<T> {
    fn from(value: Box<T>) -> Self {
        BoxWrap(value)
    }
}

impl<'source, T: FromPyObject<'source>> FromPyObject<'source> for BoxWrap<T> {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        ob.extract::<T>().map(Box::new).map(BoxWrap)
    }
}

impl<T: IntoPy<pyo3::PyObject>> IntoPy<pyo3::PyObject> for BoxWrap<T> {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        (*self.0).into_py(py)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringWrapper(pub String);

impl From<String> for StringWrapper {
    fn from(value: String) -> Self {
        StringWrapper(value)
    }
}

impl From<StringWrapper> for String {
    fn from(value: StringWrapper) -> Self {
        value.0
    }
}

impl From<&Arc<str>> for StringWrapper {
    fn from(value: &Arc<str>) -> Self {
        StringWrapper(value.to_string())
    }
}

impl From<&StringWrapper> for Arc<str> {
    fn from(value: &StringWrapper) -> Self {
        Arc::<str>::from(value.0.clone())
    }
}

impl IntoPy<pyo3::PyObject> for StringWrapper {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.0.into_py(py)
    }
}

impl<'source> FromPyObject<'source> for StringWrapper {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        ob.extract().map(StringWrapper)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BTreeSetWrap<T>(pub (crate) BTreeSet<T>);

impl<T> From<BTreeSet<T>> for BTreeSetWrap<T> {
    fn from(value: BTreeSet<T>) -> Self {
        BTreeSetWrap(value)
    }
}

impl<T> From<BTreeSetWrap<T>> for BTreeSet<T> {
    fn from(value: BTreeSetWrap<T>) -> Self {
        value.0
    }
}
