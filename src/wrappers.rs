use pyo3::conversion::FromPyObjectOwned;
use pyo3::types::PyAnyMethods;
use pyo3::{Borrowed, Bound, FromPyObject, IntoPyObject};
use std::collections::BTreeSet;
use std::convert::Infallible;
use std::hash::Hash;
use std::sync::Arc;

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
        *value
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
        let (s, t) = value;
        (U::from_c(s), V::from_c(t))
    }
}

impl<'a, U, V, S, T> FromCompatible<&'a (S, T)> for (U, V)
where
    U: FromCompatible<&'a S>,
    V: FromCompatible<&'a T>,
{
    fn from_c(value: &'a (S, T)) -> Self {
        let (s, t) = value;
        (U::from_c(s), V::from_c(t))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VecWrap<T>(pub Vec<T>);

impl<T> Default for VecWrap<T> {
    fn default() -> Self {
        VecWrap(Vec::new())
    }
}

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

impl<'py, T: FromPyObjectOwned<'py>> FromPyObject<'_, 'py> for VecWrap<T> {
    type Error = pyo3::PyErr;

    fn extract(ob: Borrowed<'_, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        let mut v = VecWrap(Vec::new());
        for item in ob.try_iter()? {
            v.0.push(item?.extract::<T>().map_err(Into::into)?);
        }
        Ok(v)
    }
}

impl<'py, T: IntoPyObject<'py>> IntoPyObject<'py> for VecWrap<T> {
    type Target = pyo3::PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_pyobject(py)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoxWrap<T>(pub Box<T>);

impl<T> From<Box<T>> for BoxWrap<T> {
    fn from(value: Box<T>) -> Self {
        BoxWrap(value)
    }
}

impl<'a, 'py, T: FromPyObject<'a, 'py>> FromPyObject<'a, 'py> for BoxWrap<T> {
    type Error = T::Error;

    fn extract(ob: Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        ob.extract::<T>().map(Box::new).map(BoxWrap)
    }
}

impl<'py, T: IntoPyObject<'py>> IntoPyObject<'py> for BoxWrap<T> {
    type Target = T::Target;
    type Output = T::Output;
    type Error = T::Error;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self.0).into_pyobject(py)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'py> IntoPyObject<'py> for StringWrapper {
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_pyobject(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for StringWrapper {
    type Error = pyo3::PyErr;

    fn extract(ob: Borrowed<'_, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        ob.extract().map(StringWrapper)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BTreeSetWrap<T>(pub(crate) BTreeSet<T>);

impl<T: Ord> Default for BTreeSetWrap<T> {
    fn default() -> Self {
        BTreeSetWrap(BTreeSet::new())
    }
}

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

impl<'py, T: IntoPyObject<'py> + Ord> IntoPyObject<'py> for BTreeSetWrap<T> {
    type Target = pyo3::types::PySet;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_pyobject(py)
    }
}

impl<'py, T: FromPyObjectOwned<'py> + Ord> FromPyObject<'_, 'py> for BTreeSetWrap<T> {
    type Error = pyo3::PyErr;

    fn extract(ob: Borrowed<'_, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        let mut v = BTreeSetWrap(BTreeSet::new());
        for item in ob.try_iter()? {
            v.0.insert(item?.extract::<T>().map_err(Into::into)?);
        }
        Ok(v)
    }
}
