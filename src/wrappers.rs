use pyo3::conversion::FromPyObjectOwned;
use pyo3::inspect::PyStaticExpr;
use pyo3::types::PyAnyMethods;
use pyo3::{type_hint_identifier, type_hint_subscript};
use pyo3::{Borrowed, Bound, FromPyObject, IntoPyObject};
use std::collections::BTreeSet;
use std::convert::Infallible;
use std::hash::Hash;
use std::marker::PhantomData;
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

// The containers convert element-wise, generically: `FromCompatible` is
// local, so unlike `From` (`From<BoxWrap<P>> for Box<H>` would overlap std's
// `From<T> for Box<T>`) it allows these impls.

impl<'a, H, P: FromCompatible<&'a H>> FromCompatible<&'a Box<H>> for BoxWrap<P> {
    fn from_c(value: &'a Box<H>) -> Self {
        BoxWrap(Box::new(P::from_c(value)))
    }
}

impl<'a, P, H: FromCompatible<&'a P>> FromCompatible<&'a BoxWrap<P>> for Box<H> {
    fn from_c(value: &'a BoxWrap<P>) -> Self {
        Box::new(H::from_c(&value.0))
    }
}

impl<'a, H, P: FromCompatible<&'a H>> FromCompatible<&'a Vec<H>> for VecWrap<P> {
    fn from_c(value: &'a Vec<H>) -> Self {
        VecWrap(value.iter().map(P::from_c).collect())
    }
}

impl<'a, P, H: FromCompatible<&'a P>> FromCompatible<&'a VecWrap<P>> for Vec<H> {
    fn from_c(value: &'a VecWrap<P>) -> Self {
        value.0.iter().map(H::from_c).collect()
    }
}

impl<'a, H, P: FromCompatible<&'a H> + Ord> FromCompatible<&'a BTreeSet<H>> for BTreeSetWrap<P> {
    fn from_c(value: &'a BTreeSet<H>) -> Self {
        BTreeSetWrap(value.iter().map(P::from_c).collect())
    }
}

impl<'a, P, H: FromCompatible<&'a P> + Ord> FromCompatible<&'a BTreeSetWrap<P>> for BTreeSet<H> {
    fn from_c(value: &'a BTreeSetWrap<P>) -> Self {
        value.0.iter().map(H::from_c).collect()
    }
}

/// A `str` is iterable, but never meant as a collection of its characters: a
/// collection of classes, say, would take every character as a class name.
fn reject_str(ob: &pyo3::Bound<'_, pyo3::PyAny>) -> pyo3::PyResult<()> {
    if ob.is_instance_of::<pyo3::types::PyString>() {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "expected a collection, got a str",
        ));
    }
    Ok(())
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
    const INPUT_TYPE: PyStaticExpr = type_hint_subscript!(
        type_hint_identifier!("collections.abc", "Iterable"),
        T::INPUT_TYPE
    );
    type Error = pyo3::PyErr;

    fn extract(ob: Borrowed<'_, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        reject_str(&ob)?;
        let mut v = VecWrap(Vec::new());
        for item in ob.try_iter()? {
            v.0.push(item?.extract::<T>().map_err(Into::into)?);
        }
        Ok(v)
    }
}

impl<'py, T: IntoPyObject<'py>> IntoPyObject<'py> for VecWrap<T> {
    const OUTPUT_TYPE: PyStaticExpr = <Vec<T>>::OUTPUT_TYPE;
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
    const INPUT_TYPE: PyStaticExpr = T::INPUT_TYPE;
    type Error = T::Error;

    fn extract(ob: Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        ob.extract::<T>().map(Box::new).map(BoxWrap)
    }
}

impl<'py, T: IntoPyObject<'py>> IntoPyObject<'py> for BoxWrap<T> {
    const OUTPUT_TYPE: PyStaticExpr = T::OUTPUT_TYPE;
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

impl std::fmt::Display for StringWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'py> IntoPyObject<'py> for StringWrapper {
    const OUTPUT_TYPE: PyStaticExpr = String::OUTPUT_TYPE;
    type Target = pyo3::types::PyString;
    type Output = Bound<'py, Self::Target>;
    type Error = Infallible;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_pyobject(py)
    }
}

impl<'py> FromPyObject<'_, 'py> for StringWrapper {
    const INPUT_TYPE: PyStaticExpr = String::INPUT_TYPE;
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
    const OUTPUT_TYPE: PyStaticExpr = <BTreeSet<T>>::OUTPUT_TYPE;
    type Target = pyo3::types::PySet;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_pyobject(py)
    }
}

impl<'py, T: FromPyObjectOwned<'py> + Ord> FromPyObject<'_, 'py> for BTreeSetWrap<T> {
    const INPUT_TYPE: PyStaticExpr = type_hint_subscript!(
        type_hint_identifier!("collections.abc", "Iterable"),
        T::INPUT_TYPE
    );
    type Error = pyo3::PyErr;

    fn extract(ob: Borrowed<'_, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        reject_str(&ob)?;
        let mut v = BTreeSetWrap(BTreeSet::new());
        for item in ob.try_iter()? {
            v.0.insert(item?.extract::<T>().map_err(Into::into)?);
        }
        Ok(v)
    }
}

/// Builds a `typing.Literal["a", "b", ...]` type hint.
macro_rules! literal_hint {
    ($($s:literal),+) => {
        PyStaticExpr::Subscript {
            value: &type_hint_identifier!("typing", "Literal"),
            slice: &PyStaticExpr::Tuple {
                elts: &[$(PyStaticExpr::Constant { value: pyo3::inspect::PyStaticConstant::Str($s) }),+],
            },
        }
    };
}

/// The allowed values of a [`LiteralStr`] argument.
pub trait StrChoices {
    const HINT: PyStaticExpr;
}

/// A `str` argument that shows up as `typing.Literal[...]` in the type stubs.
/// Values are not validated here; the function receiving it does that.
pub struct LiteralStr<C>(String, PhantomData<C>);

impl<C> std::ops::Deref for LiteralStr<C> {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl<'py, C: StrChoices> FromPyObject<'_, 'py> for LiteralStr<C> {
    const INPUT_TYPE: PyStaticExpr = C::HINT;
    type Error = pyo3::PyErr;

    fn extract(ob: Borrowed<'_, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        Ok(LiteralStr(ob.extract()?, PhantomData))
    }
}

/// Serializations horned-owl can read and write whole ontologies in.
pub struct Serializations;
impl StrChoices for Serializations {
    const HINT: PyStaticExpr = literal_hint!("owl", "rdf", "ofn", "owx", "omn", "obo");
}

/// Syntaxes a single model element can be rendered in.
pub struct SnippetSyntaxes;
impl StrChoices for SnippetSyntaxes {
    const HINT: PyStaticExpr = literal_hint!("ofn", "omn");
}

/// For model elements horned-owl can only render in functional syntax.
pub struct FunctionalSyntax;
impl StrChoices for FunctionalSyntax {
    const HINT: PyStaticExpr = literal_hint!("ofn");
}
