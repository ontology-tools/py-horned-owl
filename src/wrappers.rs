use pyo3::conversion::FromPyObjectOwned;
use pyo3::inspect::PyStaticExpr;
use pyo3::types::PyAnyMethods;
use pyo3::{type_hint_identifier, type_hint_subscript};
use pyo3::{Borrowed, Bound, FromPyObject, IntoPyObject};
use std::collections::BTreeSet;
use std::convert::Infallible;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::{Arc, OnceLock};

/// Conversion from a value's counterpart in the other model: between a
/// Python class and its horned-owl type, field by field and element by
/// element. A trait of its own because the orphan rules forbid `From` impls
/// such as `From<&Option<H>> for Option<P>` or `From<&u32> for u32`.
pub trait FromCounterpart<T> {
    fn from_counterpart(value: &T) -> Self;
}

/// The counterpart of a value in the other model; see [`FromCounterpart`].
pub trait ToCounterpart<T> {
    fn to_counterpart(&self) -> T;
}

impl<T, U: FromCounterpart<T>> ToCounterpart<U> for T {
    fn to_counterpart(&self) -> U {
        U::from_counterpart(self)
    }
}

impl FromCounterpart<Arc<str>> for StringWrapper {
    fn from_counterpart(value: &Arc<str>) -> Self {
        StringWrapper::from(value)
    }
}

impl FromCounterpart<StringWrapper> for Arc<str> {
    fn from_counterpart(value: &StringWrapper) -> Self {
        Arc::<str>::from(value)
    }
}

impl FromCounterpart<String> for String {
    fn from_counterpart(value: &String) -> Self {
        value.clone()
    }
}

impl FromCounterpart<u32> for u32 {
    fn from_counterpart(value: &u32) -> Self {
        *value
    }
}

impl<T, U: FromCounterpart<T>> FromCounterpart<Option<T>> for Option<U> {
    fn from_counterpart(value: &Option<T>) -> Self {
        value.as_ref().map(U::from_counterpart)
    }
}

impl<U, V, S, T> FromCounterpart<(S, T)> for (U, V)
where
    U: FromCounterpart<S>,
    V: FromCounterpart<T>,
{
    fn from_counterpart(value: &(S, T)) -> Self {
        let (s, t) = value;
        (U::from_counterpart(s), V::from_counterpart(t))
    }
}

// The containers convert element-wise, generically: `FromCounterpart` is
// local, so unlike `From` (`From<BoxWrap<P>> for Box<H>` would overlap std's
// `From<T> for Box<T>`) it allows these impls.

impl<H, P: FromCounterpart<H>> FromCounterpart<Box<H>> for BoxWrap<P> {
    fn from_counterpart(value: &Box<H>) -> Self {
        BoxWrap::new(P::from_counterpart(value))
    }
}

impl<P, H: FromCounterpart<P>> FromCounterpart<BoxWrap<P>> for Box<H> {
    fn from_counterpart(value: &BoxWrap<P>) -> Self {
        Box::new(H::from_counterpart(value))
    }
}

impl<H, P: FromCounterpart<H>> FromCounterpart<Vec<H>> for VecWrap<P> {
    fn from_counterpart(value: &Vec<H>) -> Self {
        Vec::from_iter(value.iter().map(P::from_counterpart)).into()
    }
}

impl<P, H: FromCounterpart<P>> FromCounterpart<VecWrap<P>> for Vec<H> {
    fn from_counterpart(value: &VecWrap<P>) -> Self {
        value.iter().map(H::from_counterpart).collect()
    }
}

impl<H, P: FromCounterpart<H> + Ord> FromCounterpart<BTreeSet<H>> for BTreeSetWrap<P> {
    fn from_counterpart(value: &BTreeSet<H>) -> Self {
        BTreeSet::from_iter(value.iter().map(P::from_counterpart)).into()
    }
}

impl<P, H: FromCounterpart<P> + Ord> FromCounterpart<BTreeSetWrap<P>> for BTreeSet<H> {
    fn from_counterpart(value: &BTreeSetWrap<P>) -> Self {
        value.iter().map(H::from_counterpart).collect()
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

#[derive(Clone)]
struct Shared<T> {
    value: T,
    hash: OnceLock<u64>,
}

impl<T> Shared<T> {
    fn new(value: T) -> Arc<Self> {
        Arc::new(Shared {
            value,
            hash: OnceLock::new(),
        })
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Shared<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: Hash> Hash for Shared<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = self.hash.get_or_init(|| {
            let mut hasher = DefaultHasher::new();
            self.value.hash(&mut hasher);
            hasher.finish()
        });
        state.write_u64(*hash);
    }
}

impl<T: PartialEq> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq> Eq for Shared<T> {}

impl<T: PartialOrd> PartialOrd for Shared<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Shared<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VecWrap<T>(Arc<Shared<Vec<T>>>);

impl<T: Clone> VecWrap<T> {
    /// Moves the elements out if unshared, else clones them (one level deep).
    pub fn into_inner(self) -> Vec<T> {
        Arc::unwrap_or_clone(self.0).value
    }
}

impl<T> Deref for VecWrap<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0.value
    }
}

impl<T> Default for VecWrap<T> {
    fn default() -> Self {
        VecWrap(Shared::new(Vec::new()))
    }
}

impl<T> From<Vec<T>> for VecWrap<T> {
    fn from(value: Vec<T>) -> Self {
        VecWrap(Shared::new(value))
    }
}

impl<T: Clone> From<VecWrap<T>> for Vec<T> {
    fn from(value: VecWrap<T>) -> Self {
        value.into_inner()
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
        let mut v = Vec::new();
        for item in ob.try_iter()? {
            v.push(item?.extract::<T>().map_err(Into::into)?);
        }
        Ok(v.into())
    }
}

impl<'py, T: IntoPyObject<'py> + Clone> IntoPyObject<'py> for VecWrap<T> {
    const OUTPUT_TYPE: PyStaticExpr = <Vec<T>>::OUTPUT_TYPE;
    type Target = pyo3::PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        self.into_inner().into_pyobject(py)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoxWrap<T>(Arc<Shared<T>>);

impl<T> BoxWrap<T> {
    pub fn new(value: T) -> Self {
        BoxWrap(Shared::new(value))
    }
}

impl<T: Clone> BoxWrap<T> {
    /// Moves the value out if unshared, else clones it (one level deep).
    pub fn into_inner(self) -> T {
        Arc::unwrap_or_clone(self.0).value
    }
}

impl<T> Deref for BoxWrap<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0.value
    }
}

impl<T> From<T> for BoxWrap<T> {
    fn from(value: T) -> Self {
        BoxWrap::new(value)
    }
}

impl<'a, 'py, T: FromPyObject<'a, 'py>> FromPyObject<'a, 'py> for BoxWrap<T> {
    const INPUT_TYPE: PyStaticExpr = T::INPUT_TYPE;
    type Error = T::Error;

    fn extract(ob: Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        ob.extract::<T>().map(BoxWrap::new)
    }
}

impl<'py, T: IntoPyObject<'py> + Clone> IntoPyObject<'py> for BoxWrap<T> {
    const OUTPUT_TYPE: PyStaticExpr = T::OUTPUT_TYPE;
    type Target = T::Target;
    type Output = T::Output;
    type Error = T::Error;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        self.into_inner().into_pyobject(py)
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
pub struct BTreeSetWrap<T>(Arc<Shared<BTreeSet<T>>>);

impl<T: Clone> BTreeSetWrap<T> {
    /// Moves the elements out if unshared, else clones them (one level deep).
    pub fn into_inner(self) -> BTreeSet<T> {
        Arc::unwrap_or_clone(self.0).value
    }
}

impl<T> Deref for BTreeSetWrap<T> {
    type Target = BTreeSet<T>;

    fn deref(&self) -> &BTreeSet<T> {
        &self.0.value
    }
}

impl<T: Ord> Default for BTreeSetWrap<T> {
    fn default() -> Self {
        BTreeSetWrap(Shared::new(BTreeSet::new()))
    }
}

impl<T> From<BTreeSet<T>> for BTreeSetWrap<T> {
    fn from(value: BTreeSet<T>) -> Self {
        BTreeSetWrap(Shared::new(value))
    }
}

impl<T: Clone> From<BTreeSetWrap<T>> for BTreeSet<T> {
    fn from(value: BTreeSetWrap<T>) -> Self {
        value.into_inner()
    }
}

impl<'py, T: IntoPyObject<'py> + Ord + Clone> IntoPyObject<'py> for BTreeSetWrap<T> {
    const OUTPUT_TYPE: PyStaticExpr = <BTreeSet<T>>::OUTPUT_TYPE;
    type Target = pyo3::types::PySet;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        self.into_inner().into_pyobject(py)
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
        let mut v = BTreeSet::new();
        for item in ob.try_iter()? {
            v.insert(item?.extract::<T>().map_err(Into::into)?);
        }
        Ok(v.into())
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

#[cfg(test)]
mod sharing_tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    struct NoDeepClone;

    impl Clone for NoDeepClone {
        fn clone(&self) -> Self {
            panic!("cloning an immutable container must not clone its children")
        }
    }

    #[test]
    fn cloning_recursive_containers_shares_children() {
        let values: VecWrap<_> = vec![NoDeepClone].into();
        assert!(Arc::ptr_eq(&values.0, &values.clone().0));
        let boxed = BoxWrap::new(NoDeepClone);
        assert!(Arc::ptr_eq(&boxed.0, &boxed.clone().0));
        let set: BTreeSetWrap<_> = BTreeSet::from([NoDeepClone]).into();
        assert!(Arc::ptr_eq(&set.0, &set.clone().0));
    }

    #[derive(Debug, Clone)]
    struct CountsHashes(Arc<std::sync::atomic::AtomicUsize>);

    impl Hash for CountsHashes {
        fn hash<H: Hasher>(&self, _: &mut H) {
            self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    fn hash_of<T: Hash>(value: &T) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn hashes_are_computed_once_and_shared_by_clones() {
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let boxed = BoxWrap::new(CountsHashes(counter.clone()));
        let values: VecWrap<_> = vec![boxed.clone(), boxed.clone()].into();
        let first = hash_of(&values);
        assert_eq!(first, hash_of(&values.clone()));
        assert_eq!(hash_of(&boxed), hash_of(&boxed.clone()));
        assert_eq!(counter.load(std::sync::atomic::Ordering::Relaxed), 1);
    }

    #[test]
    fn equal_values_hash_equally() {
        let a: VecWrap<_> = vec![BoxWrap::new(1), BoxWrap::new(2)].into();
        let b: VecWrap<_> = vec![BoxWrap::new(1), BoxWrap::new(2)].into();
        assert_eq!(a, b);
        assert_eq!(hash_of(&a), hash_of(&b));
    }
}
