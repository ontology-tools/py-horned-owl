#![allow(clippy::all)]

use std::{borrow::Borrow, collections::{BTreeSet,hash_map::DefaultHasher}, sync::Arc};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use horned_owl::io::ofn::writer::AsFunctional;

use horned_owl::model::ArcStr;
use pyo3::{exceptions::PyKeyError, prelude::*, PyAny, types::PyType};

use crate::wrappers::*;

/****************** IRI **********************/

impl FromCompatible<&horned_owl::model::IRI<Arc<str>>> for IRI {
    fn from_c(value: &horned_owl::model::IRI<Arc<str>>) -> Self {
        IRI::from(value)
    }
}

impl FromCompatible<&IRI> for horned_owl::model::IRI<Arc<str>> {
    fn from_c(value: &IRI) -> Self {
        horned_owl::model::IRI::<Arc<str>>::from(value)
    }
}

impl FromCompatible<horned_owl::model::IRI<Arc<str>>> for IRI {
    fn from_c(value: horned_owl::model::IRI<Arc<str>>) -> Self {
        IRI::from(value.borrow())
    }
}

impl FromCompatible<IRI> for horned_owl::model::IRI<Arc<str>> {
    fn from_c(value: IRI) -> Self {
        horned_owl::model::IRI::<Arc<str>>::from(value.borrow())
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(module = "pyhornedowl.model")]
pub struct IRI(horned_owl::model::IRI<ArcStr>);

impl From<IRI> for horned_owl::model::IRI<ArcStr> {
    fn from(value: IRI) -> Self {
        value.0
    }
}

impl From<horned_owl::model::IRI<ArcStr>> for IRI {
    fn from(value: horned_owl::model::IRI<ArcStr>) -> Self {
        IRI(value)
    }
}

impl From<&IRI> for horned_owl::model::IRI<ArcStr> {
    fn from(value: &IRI) -> Self {
        value.0.clone()
    }
}

impl From<&horned_owl::model::IRI<ArcStr>> for IRI {
    fn from(value: &horned_owl::model::IRI<ArcStr>) -> Self {
        IRI(value.clone())
    }
}

impl From<IRI> for String {
    fn from(value: IRI) -> Self {
        value.0.to_string()
    }
}

#[pymethods]
impl IRI {
    pub fn __repr__(&self) -> String {
        format!("IRI.parse(\"{}\")", self.0)
    }
    pub fn __str__(&self) -> String {
        self.0.to_string()
    }

    #[classmethod]
    pub fn parse(_: &Bound<'_, PyType>, value: String) -> Self {
        let builder = horned_owl::model::Build::new_arc();
        IRI(builder.iri(value))
    }
}

impl IRI {
    pub fn new<A: Borrow<str>>(iri: A, build: &horned_owl::model::Build<ArcStr>) -> Self {
        IRI(build.iri(iri))
    }
}

/********************* FACET ***********************/

impl FromCompatible<&horned_owl::vocab::Facet> for Facet {
    fn from_c(value: &horned_owl::vocab::Facet) -> Self {
        Facet::from(value)
    }
}

impl FromCompatible<Facet> for horned_owl::vocab::Facet {
    fn from_c(value: Facet) -> Self {
        From::from(value.borrow())
    }
}

impl FromCompatible<&Facet> for horned_owl::vocab::Facet {
    fn from_c(value: &Facet) -> Self {
        horned_owl::vocab::Facet::from(value)
    }
}

impl FromCompatible<horned_owl::vocab::Facet> for Facet {
    fn from_c(value: horned_owl::vocab::Facet) -> Self {
        Facet::from(value.borrow())
    }
}


#[doc = doc!(Facet)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(module = "pyhornedowl.model")]
pub enum Facet {
    Length = 1,
    MinLength = 2,
    MaxLength = 3,
    Pattern = 4,
    MinInclusive = 5,
    MinExclusive = 6,
    MaxInclusive = 7,
    MaxExclusive = 8,
    TotalDigits = 9,
    FractionDigits = 10,
    LangRange = 11,
}

#[pymethods]
impl Facet {
    #[cfg(pyi)]
    #[classmethod]
    fn __pyi__(_: &Bound<'_, PyType>) -> String {
        "class Facet:
    Length: Facet
    MinLength: Facet
    MaxLength: Facet
    Pattern: Facet
    MinInclusive: Facet
    MinExclusive: Facet
    MaxInclusive: Facet
    MaxExclusive: Facet
    TotalDigits: Facet
    FractionDigits: Facet
    LangRange: Facet
"
            .to_owned()
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&Facet> for horned_owl::vocab::Facet {
    fn from(value: &Facet) -> Self {
        match value {
            Facet::Length => horned_owl::vocab::Facet::Length,
            Facet::MinLength => horned_owl::vocab::Facet::MinLength,
            Facet::MaxLength => horned_owl::vocab::Facet::MaxLength,
            Facet::Pattern => horned_owl::vocab::Facet::Pattern,
            Facet::MinInclusive => horned_owl::vocab::Facet::MinInclusive,
            Facet::MinExclusive => horned_owl::vocab::Facet::MinExclusive,
            Facet::MaxInclusive => horned_owl::vocab::Facet::MaxInclusive,
            Facet::MaxExclusive => horned_owl::vocab::Facet::MaxExclusive,
            Facet::TotalDigits => horned_owl::vocab::Facet::TotalDigits,
            Facet::FractionDigits => horned_owl::vocab::Facet::FractionDigits,
            Facet::LangRange => horned_owl::vocab::Facet::LangRange,
        }
    }
}

impl From<&horned_owl::vocab::Facet> for Facet {
    fn from(value: &horned_owl::vocab::Facet) -> Self {
        match value {
            horned_owl::vocab::Facet::Length => Facet::Length,
            horned_owl::vocab::Facet::MinLength => Facet::MinLength,
            horned_owl::vocab::Facet::MaxLength => Facet::MaxLength,
            horned_owl::vocab::Facet::Pattern => Facet::Pattern,
            horned_owl::vocab::Facet::MinInclusive => Facet::MinInclusive,
            horned_owl::vocab::Facet::MinExclusive => Facet::MinExclusive,
            horned_owl::vocab::Facet::MaxInclusive => Facet::MaxInclusive,
            horned_owl::vocab::Facet::MaxExclusive => Facet::MaxExclusive,
            horned_owl::vocab::Facet::TotalDigits => Facet::TotalDigits,
            horned_owl::vocab::Facet::FractionDigits => Facet::FractionDigits,
            horned_owl::vocab::Facet::LangRange => Facet::LangRange,
        }
    }
}

impl From<Facet> for horned_owl::vocab::Facet {
    fn from(value: Facet) -> Self {
        value.borrow().into()
    }
}

impl From<horned_owl::vocab::Facet> for Facet {
    fn from(value: horned_owl::vocab::Facet) -> Self {
        (&value).into()
    }
}


/************ Annotations *******************/

impl FromCompatible<&BTreeSet<horned_owl::model::Annotation<Arc<str>>>>
for BTreeSetWrap<Annotation>
{
    fn from_c(value: &BTreeSet<horned_owl::model::Annotation<Arc<str>>>) -> Self {
        BTreeSetWrap::<Annotation>::from(value)
    }
}

impl FromCompatible<&BTreeSetWrap<Annotation>>
for BTreeSet<horned_owl::model::Annotation<Arc<str>>>
{
    fn from_c(value: &BTreeSetWrap<Annotation>) -> Self {
        BTreeSet::<horned_owl::model::Annotation<Arc<str>>>::from(value)
    }
}

impl FromCompatible<BTreeSet<horned_owl::model::Annotation<Arc<str>>>>
for BTreeSetWrap<Annotation>
{
    fn from_c(value: BTreeSet<horned_owl::model::Annotation<Arc<str>>>) -> Self {
        FromCompatible::from_c(value.borrow())
    }
}

impl FromCompatible<BTreeSetWrap<Annotation>>
for BTreeSet<horned_owl::model::Annotation<Arc<str>>>
{
    fn from_c(value: BTreeSetWrap<Annotation>) -> Self {
        FromCompatible::from_c(value.borrow())
    }
}

#[doc = concat!(
    "Class(first: IRI,)",
    "\n\n",
    doc!(Class)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Class (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl Class {
    #[new]
    fn new(first: IRI,) -> Self {
        Class (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&horned_owl::model::Class<ArcStr>> for Class {
    fn from(value: &horned_owl::model::Class<ArcStr>) -> Self {

        Class (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&Class> for horned_owl::model::Class<ArcStr> {
    fn from(value: &Class) -> Self {
        horned_owl::model::Class::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for Class ****************/
impl FromCompatible<horned_owl::model::Class<ArcStr>> for Class {
    fn from_c(value: horned_owl::model::Class<ArcStr>) -> Self {
        Class::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Class<ArcStr>> for Class {
    fn from_c(value: &horned_owl::model::Class<ArcStr>) -> Self {
        Class::from(value)
    }
}

impl FromCompatible<Class> for horned_owl::model::Class<ArcStr> {
    fn from_c(value: Class) -> Self {
        horned_owl::model::Class::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Class> for horned_owl::model::Class<ArcStr> {
    fn from_c(value: &Class) -> Self {
        horned_owl::model::Class::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Class<ArcStr>> for Class {
    fn from(value: horned_owl::model::Class<ArcStr>) -> Self {
        Class::from(value.borrow())
    }
}

impl From<Class> for horned_owl::model::Class<ArcStr> {
    fn from(value: Class) -> Self {
        horned_owl::model::Class::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Class>> for Box<horned_owl::model::Class<ArcStr>> {
    fn from(value: &BoxWrap<Class>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Class<ArcStr>>> for BoxWrap<Class> {
    fn from(value: &Box<horned_owl::model::Class<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Class>::into(*value.clone())))
    }
}

impl From<BoxWrap<Class>> for Box<horned_owl::model::Class<ArcStr>> {
    fn from(value: BoxWrap<Class>) -> Self {
        Into::<Box<horned_owl::model::Class<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Class<ArcStr>>> for BoxWrap<Class> {
    fn from(value: Box<horned_owl::model::Class<ArcStr>>) -> Self {
        Into::<BoxWrap<Class>>::into(value.borrow())
    }
}

impl From<VecWrap<Class>> for Vec<horned_owl::model::Class<ArcStr>> {
    fn from(value: VecWrap<Class>) -> Self {
        Into::<Vec<horned_owl::model::Class<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Class<ArcStr>>> for VecWrap<Class> {
    fn from(value: Vec<horned_owl::model::Class<ArcStr>>) -> Self {
        Into::<VecWrap<Class>>::into(value.borrow())
    }
}

impl From<&VecWrap<Class>> for Vec<horned_owl::model::Class<ArcStr>> {
    fn from(value: &VecWrap<Class>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Class::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Class<ArcStr>>> for VecWrap<Class> {
    fn from(value: &Vec<horned_owl::model::Class<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Class::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Class>> for Box<horned_owl::model::Class<ArcStr>> {
    fn from_c(value: &BoxWrap<Class>) -> Self {
        Box::<horned_owl::model::Class<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Class<ArcStr>>> for BoxWrap<Class> {
    fn from_c(value: &Box<horned_owl::model::Class<ArcStr>>) -> Self {
        BoxWrap::<Class>::from(value)
    }
}
impl FromCompatible<BoxWrap<Class>> for Box<horned_owl::model::Class<ArcStr>> {
    fn from_c(value: BoxWrap<Class>) -> Self {
        Box::<horned_owl::model::Class<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Class<ArcStr>>> for BoxWrap<Class> {
    fn from_c(value: Box<horned_owl::model::Class<ArcStr>>) -> Self {
        BoxWrap::<Class>::from(value)
    }
}
impl FromCompatible<VecWrap<Class>> for Vec<horned_owl::model::Class<ArcStr>> {
    fn from_c(value: VecWrap<Class>) -> Self {
        Vec::<horned_owl::model::Class<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Class<ArcStr>>> for VecWrap<Class> {
    fn from_c(value: Vec<horned_owl::model::Class<ArcStr>>) -> Self {
        VecWrap::<Class>::from(value)
    }
}
impl FromCompatible<&VecWrap<Class>> for Vec<horned_owl::model::Class<ArcStr>> {
    fn from_c(value: &VecWrap<Class>) -> Self {
        Vec::<horned_owl::model::Class<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Class<ArcStr>>> for VecWrap<Class> {
    fn from_c(value: &Vec<horned_owl::model::Class<ArcStr>>) -> Self {
        VecWrap::<Class>::from(value)
    }
}
    /******** EXTENSION "named" for Class ********/
    impl Display for Class {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.0)
        }
    }

    #[pymethods]
    impl Class {
        fn __str__(&self) -> String {
            self.to_string()
        }
    }
    /******** EXTENSION "class-expression" for Class ********/
    #[pymethods]
    impl Class {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }
#[doc = concat!(
    "AnonymousIndividual(first: str,)",
    "\n\n",
    doc!(AnonymousIndividual)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnonymousIndividual (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub StringWrapper,
);

#[pymethods]
impl AnonymousIndividual {
    #[new]
    fn new(first: StringWrapper,) -> Self {
        AnonymousIndividual (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&horned_owl::model::AnonymousIndividual<ArcStr>> for AnonymousIndividual {
    fn from(value: &horned_owl::model::AnonymousIndividual<ArcStr>) -> Self {

        AnonymousIndividual (
    IntoCompatible::<StringWrapper>::into_c(&value.0),
        )
    }
}

impl From<&AnonymousIndividual> for horned_owl::model::AnonymousIndividual<ArcStr> {
    fn from(value: &AnonymousIndividual) -> Self {
        horned_owl::model::AnonymousIndividual::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for AnonymousIndividual ****************/
impl FromCompatible<horned_owl::model::AnonymousIndividual<ArcStr>> for AnonymousIndividual {
    fn from_c(value: horned_owl::model::AnonymousIndividual<ArcStr>) -> Self {
        AnonymousIndividual::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AnonymousIndividual<ArcStr>> for AnonymousIndividual {
    fn from_c(value: &horned_owl::model::AnonymousIndividual<ArcStr>) -> Self {
        AnonymousIndividual::from(value)
    }
}

impl FromCompatible<AnonymousIndividual> for horned_owl::model::AnonymousIndividual<ArcStr> {
    fn from_c(value: AnonymousIndividual) -> Self {
        horned_owl::model::AnonymousIndividual::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AnonymousIndividual> for horned_owl::model::AnonymousIndividual<ArcStr> {
    fn from_c(value: &AnonymousIndividual) -> Self {
        horned_owl::model::AnonymousIndividual::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AnonymousIndividual<ArcStr>> for AnonymousIndividual {
    fn from(value: horned_owl::model::AnonymousIndividual<ArcStr>) -> Self {
        AnonymousIndividual::from(value.borrow())
    }
}

impl From<AnonymousIndividual> for horned_owl::model::AnonymousIndividual<ArcStr> {
    fn from(value: AnonymousIndividual) -> Self {
        horned_owl::model::AnonymousIndividual::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AnonymousIndividual>> for Box<horned_owl::model::AnonymousIndividual<ArcStr>> {
    fn from(value: &BoxWrap<AnonymousIndividual>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AnonymousIndividual<ArcStr>>> for BoxWrap<AnonymousIndividual> {
    fn from(value: &Box<horned_owl::model::AnonymousIndividual<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AnonymousIndividual>::into(*value.clone())))
    }
}

impl From<BoxWrap<AnonymousIndividual>> for Box<horned_owl::model::AnonymousIndividual<ArcStr>> {
    fn from(value: BoxWrap<AnonymousIndividual>) -> Self {
        Into::<Box<horned_owl::model::AnonymousIndividual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AnonymousIndividual<ArcStr>>> for BoxWrap<AnonymousIndividual> {
    fn from(value: Box<horned_owl::model::AnonymousIndividual<ArcStr>>) -> Self {
        Into::<BoxWrap<AnonymousIndividual>>::into(value.borrow())
    }
}

impl From<VecWrap<AnonymousIndividual>> for Vec<horned_owl::model::AnonymousIndividual<ArcStr>> {
    fn from(value: VecWrap<AnonymousIndividual>) -> Self {
        Into::<Vec<horned_owl::model::AnonymousIndividual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AnonymousIndividual<ArcStr>>> for VecWrap<AnonymousIndividual> {
    fn from(value: Vec<horned_owl::model::AnonymousIndividual<ArcStr>>) -> Self {
        Into::<VecWrap<AnonymousIndividual>>::into(value.borrow())
    }
}

impl From<&VecWrap<AnonymousIndividual>> for Vec<horned_owl::model::AnonymousIndividual<ArcStr>> {
    fn from(value: &VecWrap<AnonymousIndividual>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AnonymousIndividual::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AnonymousIndividual<ArcStr>>> for VecWrap<AnonymousIndividual> {
    fn from(value: &Vec<horned_owl::model::AnonymousIndividual<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AnonymousIndividual::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AnonymousIndividual>> for Box<horned_owl::model::AnonymousIndividual<ArcStr>> {
    fn from_c(value: &BoxWrap<AnonymousIndividual>) -> Self {
        Box::<horned_owl::model::AnonymousIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AnonymousIndividual<ArcStr>>> for BoxWrap<AnonymousIndividual> {
    fn from_c(value: &Box<horned_owl::model::AnonymousIndividual<ArcStr>>) -> Self {
        BoxWrap::<AnonymousIndividual>::from(value)
    }
}
impl FromCompatible<BoxWrap<AnonymousIndividual>> for Box<horned_owl::model::AnonymousIndividual<ArcStr>> {
    fn from_c(value: BoxWrap<AnonymousIndividual>) -> Self {
        Box::<horned_owl::model::AnonymousIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AnonymousIndividual<ArcStr>>> for BoxWrap<AnonymousIndividual> {
    fn from_c(value: Box<horned_owl::model::AnonymousIndividual<ArcStr>>) -> Self {
        BoxWrap::<AnonymousIndividual>::from(value)
    }
}
impl FromCompatible<VecWrap<AnonymousIndividual>> for Vec<horned_owl::model::AnonymousIndividual<ArcStr>> {
    fn from_c(value: VecWrap<AnonymousIndividual>) -> Self {
        Vec::<horned_owl::model::AnonymousIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AnonymousIndividual<ArcStr>>> for VecWrap<AnonymousIndividual> {
    fn from_c(value: Vec<horned_owl::model::AnonymousIndividual<ArcStr>>) -> Self {
        VecWrap::<AnonymousIndividual>::from(value)
    }
}
impl FromCompatible<&VecWrap<AnonymousIndividual>> for Vec<horned_owl::model::AnonymousIndividual<ArcStr>> {
    fn from_c(value: &VecWrap<AnonymousIndividual>) -> Self {
        Vec::<horned_owl::model::AnonymousIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AnonymousIndividual<ArcStr>>> for VecWrap<AnonymousIndividual> {
    fn from_c(value: &Vec<horned_owl::model::AnonymousIndividual<ArcStr>>) -> Self {
        VecWrap::<AnonymousIndividual>::from(value)
    }
}
    /******** EXTENSION "named" for AnonymousIndividual ********/
    impl Display for AnonymousIndividual {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.0)
        }
    }

    #[pymethods]
    impl AnonymousIndividual {
        fn __str__(&self) -> String {
            self.to_string()
        }
    }
#[doc = concat!(
    "NamedIndividual(first: IRI,)",
    "\n\n",
    doc!(NamedIndividual)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamedIndividual (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl NamedIndividual {
    #[new]
    fn new(first: IRI,) -> Self {
        NamedIndividual (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&horned_owl::model::NamedIndividual<ArcStr>> for NamedIndividual {
    fn from(value: &horned_owl::model::NamedIndividual<ArcStr>) -> Self {

        NamedIndividual (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&NamedIndividual> for horned_owl::model::NamedIndividual<ArcStr> {
    fn from(value: &NamedIndividual) -> Self {
        horned_owl::model::NamedIndividual::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for NamedIndividual ****************/
impl FromCompatible<horned_owl::model::NamedIndividual<ArcStr>> for NamedIndividual {
    fn from_c(value: horned_owl::model::NamedIndividual<ArcStr>) -> Self {
        NamedIndividual::from(value)
    }
}

impl FromCompatible<&horned_owl::model::NamedIndividual<ArcStr>> for NamedIndividual {
    fn from_c(value: &horned_owl::model::NamedIndividual<ArcStr>) -> Self {
        NamedIndividual::from(value)
    }
}

impl FromCompatible<NamedIndividual> for horned_owl::model::NamedIndividual<ArcStr> {
    fn from_c(value: NamedIndividual) -> Self {
        horned_owl::model::NamedIndividual::<ArcStr>::from(value)
    }
}

impl FromCompatible<&NamedIndividual> for horned_owl::model::NamedIndividual<ArcStr> {
    fn from_c(value: &NamedIndividual) -> Self {
        horned_owl::model::NamedIndividual::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::NamedIndividual<ArcStr>> for NamedIndividual {
    fn from(value: horned_owl::model::NamedIndividual<ArcStr>) -> Self {
        NamedIndividual::from(value.borrow())
    }
}

impl From<NamedIndividual> for horned_owl::model::NamedIndividual<ArcStr> {
    fn from(value: NamedIndividual) -> Self {
        horned_owl::model::NamedIndividual::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<NamedIndividual>> for Box<horned_owl::model::NamedIndividual<ArcStr>> {
    fn from(value: &BoxWrap<NamedIndividual>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::NamedIndividual<ArcStr>>> for BoxWrap<NamedIndividual> {
    fn from(value: &Box<horned_owl::model::NamedIndividual<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<NamedIndividual>::into(*value.clone())))
    }
}

impl From<BoxWrap<NamedIndividual>> for Box<horned_owl::model::NamedIndividual<ArcStr>> {
    fn from(value: BoxWrap<NamedIndividual>) -> Self {
        Into::<Box<horned_owl::model::NamedIndividual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::NamedIndividual<ArcStr>>> for BoxWrap<NamedIndividual> {
    fn from(value: Box<horned_owl::model::NamedIndividual<ArcStr>>) -> Self {
        Into::<BoxWrap<NamedIndividual>>::into(value.borrow())
    }
}

impl From<VecWrap<NamedIndividual>> for Vec<horned_owl::model::NamedIndividual<ArcStr>> {
    fn from(value: VecWrap<NamedIndividual>) -> Self {
        Into::<Vec<horned_owl::model::NamedIndividual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::NamedIndividual<ArcStr>>> for VecWrap<NamedIndividual> {
    fn from(value: Vec<horned_owl::model::NamedIndividual<ArcStr>>) -> Self {
        Into::<VecWrap<NamedIndividual>>::into(value.borrow())
    }
}

impl From<&VecWrap<NamedIndividual>> for Vec<horned_owl::model::NamedIndividual<ArcStr>> {
    fn from(value: &VecWrap<NamedIndividual>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::NamedIndividual::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::NamedIndividual<ArcStr>>> for VecWrap<NamedIndividual> {
    fn from(value: &Vec<horned_owl::model::NamedIndividual<ArcStr>>) -> Self {
        VecWrap(value.iter().map(NamedIndividual::from).collect())
    }
}

impl FromCompatible<&BoxWrap<NamedIndividual>> for Box<horned_owl::model::NamedIndividual<ArcStr>> {
    fn from_c(value: &BoxWrap<NamedIndividual>) -> Self {
        Box::<horned_owl::model::NamedIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::NamedIndividual<ArcStr>>> for BoxWrap<NamedIndividual> {
    fn from_c(value: &Box<horned_owl::model::NamedIndividual<ArcStr>>) -> Self {
        BoxWrap::<NamedIndividual>::from(value)
    }
}
impl FromCompatible<BoxWrap<NamedIndividual>> for Box<horned_owl::model::NamedIndividual<ArcStr>> {
    fn from_c(value: BoxWrap<NamedIndividual>) -> Self {
        Box::<horned_owl::model::NamedIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::NamedIndividual<ArcStr>>> for BoxWrap<NamedIndividual> {
    fn from_c(value: Box<horned_owl::model::NamedIndividual<ArcStr>>) -> Self {
        BoxWrap::<NamedIndividual>::from(value)
    }
}
impl FromCompatible<VecWrap<NamedIndividual>> for Vec<horned_owl::model::NamedIndividual<ArcStr>> {
    fn from_c(value: VecWrap<NamedIndividual>) -> Self {
        Vec::<horned_owl::model::NamedIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::NamedIndividual<ArcStr>>> for VecWrap<NamedIndividual> {
    fn from_c(value: Vec<horned_owl::model::NamedIndividual<ArcStr>>) -> Self {
        VecWrap::<NamedIndividual>::from(value)
    }
}
impl FromCompatible<&VecWrap<NamedIndividual>> for Vec<horned_owl::model::NamedIndividual<ArcStr>> {
    fn from_c(value: &VecWrap<NamedIndividual>) -> Self {
        Vec::<horned_owl::model::NamedIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::NamedIndividual<ArcStr>>> for VecWrap<NamedIndividual> {
    fn from_c(value: &Vec<horned_owl::model::NamedIndividual<ArcStr>>) -> Self {
        VecWrap::<NamedIndividual>::from(value)
    }
}
    /******** EXTENSION "named" for NamedIndividual ********/
    impl Display for NamedIndividual {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.0)
        }
    }

    #[pymethods]
    impl NamedIndividual {
        fn __str__(&self) -> String {
            self.to_string()
        }
    }
#[doc = concat!(
    "ObjectProperty(first: IRI,)",
    "\n\n",
    doc!(ObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl ObjectProperty {
    #[new]
    fn new(first: IRI,) -> Self {
        ObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&horned_owl::model::ObjectProperty<ArcStr>> for ObjectProperty {
    fn from(value: &horned_owl::model::ObjectProperty<ArcStr>) -> Self {

        ObjectProperty (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&ObjectProperty> for horned_owl::model::ObjectProperty<ArcStr> {
    fn from(value: &ObjectProperty) -> Self {
        horned_owl::model::ObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for ObjectProperty ****************/
impl FromCompatible<horned_owl::model::ObjectProperty<ArcStr>> for ObjectProperty {
    fn from_c(value: horned_owl::model::ObjectProperty<ArcStr>) -> Self {
        ObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::ObjectProperty<ArcStr>> for ObjectProperty {
    fn from_c(value: &horned_owl::model::ObjectProperty<ArcStr>) -> Self {
        ObjectProperty::from(value)
    }
}

impl FromCompatible<ObjectProperty> for horned_owl::model::ObjectProperty<ArcStr> {
    fn from_c(value: ObjectProperty) -> Self {
        horned_owl::model::ObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&ObjectProperty> for horned_owl::model::ObjectProperty<ArcStr> {
    fn from_c(value: &ObjectProperty) -> Self {
        horned_owl::model::ObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::ObjectProperty<ArcStr>> for ObjectProperty {
    fn from(value: horned_owl::model::ObjectProperty<ArcStr>) -> Self {
        ObjectProperty::from(value.borrow())
    }
}

impl From<ObjectProperty> for horned_owl::model::ObjectProperty<ArcStr> {
    fn from(value: ObjectProperty) -> Self {
        horned_owl::model::ObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<ObjectProperty>> for Box<horned_owl::model::ObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<ObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::ObjectProperty<ArcStr>>> for BoxWrap<ObjectProperty> {
    fn from(value: &Box<horned_owl::model::ObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<ObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<ObjectProperty>> for Box<horned_owl::model::ObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<ObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::ObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::ObjectProperty<ArcStr>>> for BoxWrap<ObjectProperty> {
    fn from(value: Box<horned_owl::model::ObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<ObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<ObjectProperty>> for Vec<horned_owl::model::ObjectProperty<ArcStr>> {
    fn from(value: VecWrap<ObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::ObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::ObjectProperty<ArcStr>>> for VecWrap<ObjectProperty> {
    fn from(value: Vec<horned_owl::model::ObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<ObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<ObjectProperty>> for Vec<horned_owl::model::ObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<ObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::ObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::ObjectProperty<ArcStr>>> for VecWrap<ObjectProperty> {
    fn from(value: &Vec<horned_owl::model::ObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(ObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<ObjectProperty>> for Box<horned_owl::model::ObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<ObjectProperty>) -> Self {
        Box::<horned_owl::model::ObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::ObjectProperty<ArcStr>>> for BoxWrap<ObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::ObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<ObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<ObjectProperty>> for Box<horned_owl::model::ObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<ObjectProperty>) -> Self {
        Box::<horned_owl::model::ObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::ObjectProperty<ArcStr>>> for BoxWrap<ObjectProperty> {
    fn from_c(value: Box<horned_owl::model::ObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<ObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<ObjectProperty>> for Vec<horned_owl::model::ObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<ObjectProperty>) -> Self {
        Vec::<horned_owl::model::ObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::ObjectProperty<ArcStr>>> for VecWrap<ObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::ObjectProperty<ArcStr>>) -> Self {
        VecWrap::<ObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<ObjectProperty>> for Vec<horned_owl::model::ObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<ObjectProperty>) -> Self {
        Vec::<horned_owl::model::ObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::ObjectProperty<ArcStr>>> for VecWrap<ObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::ObjectProperty<ArcStr>>) -> Self {
        VecWrap::<ObjectProperty>::from(value)
    }
}
    /******** EXTENSION "named" for ObjectProperty ********/
    impl Display for ObjectProperty {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.0)
        }
    }

    #[pymethods]
    impl ObjectProperty {
        fn __str__(&self) -> String {
            self.to_string()
        }
    }
    /******** EXTENSION "object-property-expression" for ObjectProperty ********/
       #[pymethods]
        impl ObjectProperty {
            fn some(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectSomeValuesFrom> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectSomeValuesFrom {
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn only(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectAllValuesFrom> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectAllValuesFrom {
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn has_value(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectHasValue> {
                let i: Individual = obj.extract()?;
                Ok(ObjectHasValue{
                    ope: self.clone().into(),
                    i
                })
            }

            fn has_self(&self) -> PyResult<ObjectHasSelf> {
                Ok(ObjectHasSelf(self.clone().into()))
            }

            fn min(&self, n: u32, obj: &Bound<'_, PyAny>) -> PyResult<ObjectMinCardinality> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectMinCardinality {
                    n,
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn max(&self, n: u32, obj: &Bound<'_, PyAny>) -> PyResult<ObjectMaxCardinality> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectMaxCardinality {
                    n,
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn exact(&self, n: u32, obj: &Bound<'_, PyAny>) -> PyResult<ObjectExactCardinality> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectExactCardinality {
                    n,
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn __invert__(&self) -> ObjectPropertyExpression {
                let ope: ObjectPropertyExpression = self.clone().into();
                let inner: ObjectPropertyExpression_Inner = match ope.0 {
                    ObjectPropertyExpression_Inner::InverseObjectProperty(
                        InverseObjectProperty(i),
                    ) => ObjectPropertyExpression_Inner::ObjectProperty(i),
                    ObjectPropertyExpression_Inner::ObjectProperty(i) => {
                        ObjectPropertyExpression_Inner::InverseObjectProperty(
                            InverseObjectProperty(i),
                        )
                }
                };

                ObjectPropertyExpression(inner)
            }
        }
#[doc = concat!(
    "Datatype(first: IRI,)",
    "\n\n",
    doc!(Datatype)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Datatype (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl Datatype {
    #[new]
    fn new(first: IRI,) -> Self {
        Datatype (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&horned_owl::model::Datatype<ArcStr>> for Datatype {
    fn from(value: &horned_owl::model::Datatype<ArcStr>) -> Self {

        Datatype (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&Datatype> for horned_owl::model::Datatype<ArcStr> {
    fn from(value: &Datatype) -> Self {
        horned_owl::model::Datatype::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for Datatype ****************/
impl FromCompatible<horned_owl::model::Datatype<ArcStr>> for Datatype {
    fn from_c(value: horned_owl::model::Datatype<ArcStr>) -> Self {
        Datatype::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Datatype<ArcStr>> for Datatype {
    fn from_c(value: &horned_owl::model::Datatype<ArcStr>) -> Self {
        Datatype::from(value)
    }
}

impl FromCompatible<Datatype> for horned_owl::model::Datatype<ArcStr> {
    fn from_c(value: Datatype) -> Self {
        horned_owl::model::Datatype::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Datatype> for horned_owl::model::Datatype<ArcStr> {
    fn from_c(value: &Datatype) -> Self {
        horned_owl::model::Datatype::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Datatype<ArcStr>> for Datatype {
    fn from(value: horned_owl::model::Datatype<ArcStr>) -> Self {
        Datatype::from(value.borrow())
    }
}

impl From<Datatype> for horned_owl::model::Datatype<ArcStr> {
    fn from(value: Datatype) -> Self {
        horned_owl::model::Datatype::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Datatype>> for Box<horned_owl::model::Datatype<ArcStr>> {
    fn from(value: &BoxWrap<Datatype>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Datatype<ArcStr>>> for BoxWrap<Datatype> {
    fn from(value: &Box<horned_owl::model::Datatype<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Datatype>::into(*value.clone())))
    }
}

impl From<BoxWrap<Datatype>> for Box<horned_owl::model::Datatype<ArcStr>> {
    fn from(value: BoxWrap<Datatype>) -> Self {
        Into::<Box<horned_owl::model::Datatype<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Datatype<ArcStr>>> for BoxWrap<Datatype> {
    fn from(value: Box<horned_owl::model::Datatype<ArcStr>>) -> Self {
        Into::<BoxWrap<Datatype>>::into(value.borrow())
    }
}

impl From<VecWrap<Datatype>> for Vec<horned_owl::model::Datatype<ArcStr>> {
    fn from(value: VecWrap<Datatype>) -> Self {
        Into::<Vec<horned_owl::model::Datatype<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Datatype<ArcStr>>> for VecWrap<Datatype> {
    fn from(value: Vec<horned_owl::model::Datatype<ArcStr>>) -> Self {
        Into::<VecWrap<Datatype>>::into(value.borrow())
    }
}

impl From<&VecWrap<Datatype>> for Vec<horned_owl::model::Datatype<ArcStr>> {
    fn from(value: &VecWrap<Datatype>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Datatype::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Datatype<ArcStr>>> for VecWrap<Datatype> {
    fn from(value: &Vec<horned_owl::model::Datatype<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Datatype::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Datatype>> for Box<horned_owl::model::Datatype<ArcStr>> {
    fn from_c(value: &BoxWrap<Datatype>) -> Self {
        Box::<horned_owl::model::Datatype<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Datatype<ArcStr>>> for BoxWrap<Datatype> {
    fn from_c(value: &Box<horned_owl::model::Datatype<ArcStr>>) -> Self {
        BoxWrap::<Datatype>::from(value)
    }
}
impl FromCompatible<BoxWrap<Datatype>> for Box<horned_owl::model::Datatype<ArcStr>> {
    fn from_c(value: BoxWrap<Datatype>) -> Self {
        Box::<horned_owl::model::Datatype<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Datatype<ArcStr>>> for BoxWrap<Datatype> {
    fn from_c(value: Box<horned_owl::model::Datatype<ArcStr>>) -> Self {
        BoxWrap::<Datatype>::from(value)
    }
}
impl FromCompatible<VecWrap<Datatype>> for Vec<horned_owl::model::Datatype<ArcStr>> {
    fn from_c(value: VecWrap<Datatype>) -> Self {
        Vec::<horned_owl::model::Datatype<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Datatype<ArcStr>>> for VecWrap<Datatype> {
    fn from_c(value: Vec<horned_owl::model::Datatype<ArcStr>>) -> Self {
        VecWrap::<Datatype>::from(value)
    }
}
impl FromCompatible<&VecWrap<Datatype>> for Vec<horned_owl::model::Datatype<ArcStr>> {
    fn from_c(value: &VecWrap<Datatype>) -> Self {
        Vec::<horned_owl::model::Datatype<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Datatype<ArcStr>>> for VecWrap<Datatype> {
    fn from_c(value: &Vec<horned_owl::model::Datatype<ArcStr>>) -> Self {
        VecWrap::<Datatype>::from(value)
    }
}
    /******** EXTENSION "named" for Datatype ********/
    impl Display for Datatype {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.0)
        }
    }

    #[pymethods]
    impl Datatype {
        fn __str__(&self) -> String {
            self.to_string()
        }
    }
#[doc = concat!(
    "DataProperty(first: IRI,)",
    "\n\n",
    doc!(DataProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl DataProperty {
    #[new]
    fn new(first: IRI,) -> Self {
        DataProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
}

impl From<&horned_owl::model::DataProperty<ArcStr>> for DataProperty {
    fn from(value: &horned_owl::model::DataProperty<ArcStr>) -> Self {

        DataProperty (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&DataProperty> for horned_owl::model::DataProperty<ArcStr> {
    fn from(value: &DataProperty) -> Self {
        horned_owl::model::DataProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DataProperty ****************/
impl FromCompatible<horned_owl::model::DataProperty<ArcStr>> for DataProperty {
    fn from_c(value: horned_owl::model::DataProperty<ArcStr>) -> Self {
        DataProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DataProperty<ArcStr>> for DataProperty {
    fn from_c(value: &horned_owl::model::DataProperty<ArcStr>) -> Self {
        DataProperty::from(value)
    }
}

impl FromCompatible<DataProperty> for horned_owl::model::DataProperty<ArcStr> {
    fn from_c(value: DataProperty) -> Self {
        horned_owl::model::DataProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DataProperty> for horned_owl::model::DataProperty<ArcStr> {
    fn from_c(value: &DataProperty) -> Self {
        horned_owl::model::DataProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DataProperty<ArcStr>> for DataProperty {
    fn from(value: horned_owl::model::DataProperty<ArcStr>) -> Self {
        DataProperty::from(value.borrow())
    }
}

impl From<DataProperty> for horned_owl::model::DataProperty<ArcStr> {
    fn from(value: DataProperty) -> Self {
        horned_owl::model::DataProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DataProperty>> for Box<horned_owl::model::DataProperty<ArcStr>> {
    fn from(value: &BoxWrap<DataProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DataProperty<ArcStr>>> for BoxWrap<DataProperty> {
    fn from(value: &Box<horned_owl::model::DataProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DataProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<DataProperty>> for Box<horned_owl::model::DataProperty<ArcStr>> {
    fn from(value: BoxWrap<DataProperty>) -> Self {
        Into::<Box<horned_owl::model::DataProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DataProperty<ArcStr>>> for BoxWrap<DataProperty> {
    fn from(value: Box<horned_owl::model::DataProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<DataProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<DataProperty>> for Vec<horned_owl::model::DataProperty<ArcStr>> {
    fn from(value: VecWrap<DataProperty>) -> Self {
        Into::<Vec<horned_owl::model::DataProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DataProperty<ArcStr>>> for VecWrap<DataProperty> {
    fn from(value: Vec<horned_owl::model::DataProperty<ArcStr>>) -> Self {
        Into::<VecWrap<DataProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<DataProperty>> for Vec<horned_owl::model::DataProperty<ArcStr>> {
    fn from(value: &VecWrap<DataProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DataProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DataProperty<ArcStr>>> for VecWrap<DataProperty> {
    fn from(value: &Vec<horned_owl::model::DataProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DataProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DataProperty>> for Box<horned_owl::model::DataProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<DataProperty>) -> Self {
        Box::<horned_owl::model::DataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DataProperty<ArcStr>>> for BoxWrap<DataProperty> {
    fn from_c(value: &Box<horned_owl::model::DataProperty<ArcStr>>) -> Self {
        BoxWrap::<DataProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<DataProperty>> for Box<horned_owl::model::DataProperty<ArcStr>> {
    fn from_c(value: BoxWrap<DataProperty>) -> Self {
        Box::<horned_owl::model::DataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DataProperty<ArcStr>>> for BoxWrap<DataProperty> {
    fn from_c(value: Box<horned_owl::model::DataProperty<ArcStr>>) -> Self {
        BoxWrap::<DataProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<DataProperty>> for Vec<horned_owl::model::DataProperty<ArcStr>> {
    fn from_c(value: VecWrap<DataProperty>) -> Self {
        Vec::<horned_owl::model::DataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DataProperty<ArcStr>>> for VecWrap<DataProperty> {
    fn from_c(value: Vec<horned_owl::model::DataProperty<ArcStr>>) -> Self {
        VecWrap::<DataProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<DataProperty>> for Vec<horned_owl::model::DataProperty<ArcStr>> {
    fn from_c(value: &VecWrap<DataProperty>) -> Self {
        Vec::<horned_owl::model::DataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DataProperty<ArcStr>>> for VecWrap<DataProperty> {
    fn from_c(value: &Vec<horned_owl::model::DataProperty<ArcStr>>) -> Self {
        VecWrap::<DataProperty>::from(value)
    }
}
    /******** EXTENSION "named" for DataProperty ********/
    impl Display for DataProperty {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0.0)
        }
    }

    #[pymethods]
    impl DataProperty {
        fn __str__(&self) -> String {
            self.to_string()
        }
    }
#[doc = concat!("FacetRestriction(f: Facet,l: Literal,)",
    "\n\n",
    doc!(FacetRestriction)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FacetRestriction {
        #[doc="f: Facet"]
        #[pyo3(get,set)]
        pub f: Facet,
    
        #[doc="l: Literal"]
        #[pyo3(get,set)]
        pub l: Literal,
    }

#[pymethods]
impl FacetRestriction {
    #[new]
    fn new(f: Facet,l: Literal,) -> Self {
        FacetRestriction {
                f,
                l,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "f" => self.f.clone().into_pyobject(py).map(Bound::into_any),
            "l" => self.l.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "f" => {
                self.f = value.extract()?;
                Ok(())
            },
            "l" => {
                self.l = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::FacetRestriction<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::FacetRestriction<ArcStr>> for FacetRestriction {
    fn from(value: &horned_owl::model::FacetRestriction<ArcStr>) -> Self {
        FacetRestriction {
            f: IntoCompatible::<Facet>::into_c(value.f.borrow()),
            l: IntoCompatible::<Literal>::into_c(value.l.borrow()),
        }
    }
}


impl From<&FacetRestriction> for horned_owl::model::FacetRestriction<ArcStr> {
    fn from(value: &FacetRestriction) -> Self {
        horned_owl::model::FacetRestriction::<ArcStr> {
            f: value.f.borrow().into_c(),
            l: value.l.borrow().into_c(),
        }
    }
}



/**************** Base implementations for FacetRestriction ****************/
impl FromCompatible<horned_owl::model::FacetRestriction<ArcStr>> for FacetRestriction {
    fn from_c(value: horned_owl::model::FacetRestriction<ArcStr>) -> Self {
        FacetRestriction::from(value)
    }
}

impl FromCompatible<&horned_owl::model::FacetRestriction<ArcStr>> for FacetRestriction {
    fn from_c(value: &horned_owl::model::FacetRestriction<ArcStr>) -> Self {
        FacetRestriction::from(value)
    }
}

impl FromCompatible<FacetRestriction> for horned_owl::model::FacetRestriction<ArcStr> {
    fn from_c(value: FacetRestriction) -> Self {
        horned_owl::model::FacetRestriction::<ArcStr>::from(value)
    }
}

impl FromCompatible<&FacetRestriction> for horned_owl::model::FacetRestriction<ArcStr> {
    fn from_c(value: &FacetRestriction) -> Self {
        horned_owl::model::FacetRestriction::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::FacetRestriction<ArcStr>> for FacetRestriction {
    fn from(value: horned_owl::model::FacetRestriction<ArcStr>) -> Self {
        FacetRestriction::from(value.borrow())
    }
}

impl From<FacetRestriction> for horned_owl::model::FacetRestriction<ArcStr> {
    fn from(value: FacetRestriction) -> Self {
        horned_owl::model::FacetRestriction::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<FacetRestriction>> for Box<horned_owl::model::FacetRestriction<ArcStr>> {
    fn from(value: &BoxWrap<FacetRestriction>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::FacetRestriction<ArcStr>>> for BoxWrap<FacetRestriction> {
    fn from(value: &Box<horned_owl::model::FacetRestriction<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<FacetRestriction>::into(*value.clone())))
    }
}

impl From<BoxWrap<FacetRestriction>> for Box<horned_owl::model::FacetRestriction<ArcStr>> {
    fn from(value: BoxWrap<FacetRestriction>) -> Self {
        Into::<Box<horned_owl::model::FacetRestriction<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::FacetRestriction<ArcStr>>> for BoxWrap<FacetRestriction> {
    fn from(value: Box<horned_owl::model::FacetRestriction<ArcStr>>) -> Self {
        Into::<BoxWrap<FacetRestriction>>::into(value.borrow())
    }
}

impl From<VecWrap<FacetRestriction>> for Vec<horned_owl::model::FacetRestriction<ArcStr>> {
    fn from(value: VecWrap<FacetRestriction>) -> Self {
        Into::<Vec<horned_owl::model::FacetRestriction<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::FacetRestriction<ArcStr>>> for VecWrap<FacetRestriction> {
    fn from(value: Vec<horned_owl::model::FacetRestriction<ArcStr>>) -> Self {
        Into::<VecWrap<FacetRestriction>>::into(value.borrow())
    }
}

impl From<&VecWrap<FacetRestriction>> for Vec<horned_owl::model::FacetRestriction<ArcStr>> {
    fn from(value: &VecWrap<FacetRestriction>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::FacetRestriction::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::FacetRestriction<ArcStr>>> for VecWrap<FacetRestriction> {
    fn from(value: &Vec<horned_owl::model::FacetRestriction<ArcStr>>) -> Self {
        VecWrap(value.iter().map(FacetRestriction::from).collect())
    }
}

impl FromCompatible<&BoxWrap<FacetRestriction>> for Box<horned_owl::model::FacetRestriction<ArcStr>> {
    fn from_c(value: &BoxWrap<FacetRestriction>) -> Self {
        Box::<horned_owl::model::FacetRestriction<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::FacetRestriction<ArcStr>>> for BoxWrap<FacetRestriction> {
    fn from_c(value: &Box<horned_owl::model::FacetRestriction<ArcStr>>) -> Self {
        BoxWrap::<FacetRestriction>::from(value)
    }
}
impl FromCompatible<BoxWrap<FacetRestriction>> for Box<horned_owl::model::FacetRestriction<ArcStr>> {
    fn from_c(value: BoxWrap<FacetRestriction>) -> Self {
        Box::<horned_owl::model::FacetRestriction<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::FacetRestriction<ArcStr>>> for BoxWrap<FacetRestriction> {
    fn from_c(value: Box<horned_owl::model::FacetRestriction<ArcStr>>) -> Self {
        BoxWrap::<FacetRestriction>::from(value)
    }
}
impl FromCompatible<VecWrap<FacetRestriction>> for Vec<horned_owl::model::FacetRestriction<ArcStr>> {
    fn from_c(value: VecWrap<FacetRestriction>) -> Self {
        Vec::<horned_owl::model::FacetRestriction<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::FacetRestriction<ArcStr>>> for VecWrap<FacetRestriction> {
    fn from_c(value: Vec<horned_owl::model::FacetRestriction<ArcStr>>) -> Self {
        VecWrap::<FacetRestriction>::from(value)
    }
}
impl FromCompatible<&VecWrap<FacetRestriction>> for Vec<horned_owl::model::FacetRestriction<ArcStr>> {
    fn from_c(value: &VecWrap<FacetRestriction>) -> Self {
        Vec::<horned_owl::model::FacetRestriction<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::FacetRestriction<ArcStr>>> for VecWrap<FacetRestriction> {
    fn from_c(value: &Vec<horned_owl::model::FacetRestriction<ArcStr>>) -> Self {
        VecWrap::<FacetRestriction>::from(value)
    }
}
#[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Individual {
    
        #[pyo3(transparent)]
        Anonymous (AnonymousIndividual),
    
        #[pyo3(transparent)]
        Named (NamedIndividual),
    
}

impl<'py> IntoPyObject<'py> for Individual {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
        
            Individual::Anonymous(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Individual::Named(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
        }
    }
}

impl From<&Individual> for horned_owl::model::Individual<ArcStr> {
    fn from(value: &Individual) -> Self {
        match value {
        
            Individual::Anonymous(inner) => horned_owl::model::Individual::Anonymous(inner.into()),
        
            Individual::Named(inner) => horned_owl::model::Individual::Named(inner.into()),
        
        }
    }
}

impl From<&horned_owl::model::Individual<ArcStr>> for Individual {

    fn from(value: &horned_owl::model::Individual<ArcStr>) -> Self {
        match value {
        
            horned_owl::model::Individual::Anonymous(inner) => Individual::Anonymous(inner.into()),
        
            horned_owl::model::Individual::Named(inner) => Individual::Named(inner.into()),
        
        }
    }
}


impl Individual {
    pub fn py_def() -> String {
        "typing.Union[m.AnonymousIndividual,m.NamedIndividual,]".into()
    }
}



/**************** Base implementations for Individual ****************/
impl FromCompatible<horned_owl::model::Individual<ArcStr>> for Individual {
    fn from_c(value: horned_owl::model::Individual<ArcStr>) -> Self {
        Individual::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Individual<ArcStr>> for Individual {
    fn from_c(value: &horned_owl::model::Individual<ArcStr>) -> Self {
        Individual::from(value)
    }
}

impl FromCompatible<Individual> for horned_owl::model::Individual<ArcStr> {
    fn from_c(value: Individual) -> Self {
        horned_owl::model::Individual::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Individual> for horned_owl::model::Individual<ArcStr> {
    fn from_c(value: &Individual) -> Self {
        horned_owl::model::Individual::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Individual<ArcStr>> for Individual {
    fn from(value: horned_owl::model::Individual<ArcStr>) -> Self {
        Individual::from(value.borrow())
    }
}

impl From<Individual> for horned_owl::model::Individual<ArcStr> {
    fn from(value: Individual) -> Self {
        horned_owl::model::Individual::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Individual>> for Box<horned_owl::model::Individual<ArcStr>> {
    fn from(value: &BoxWrap<Individual>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Individual<ArcStr>>> for BoxWrap<Individual> {
    fn from(value: &Box<horned_owl::model::Individual<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Individual>::into(*value.clone())))
    }
}

impl From<BoxWrap<Individual>> for Box<horned_owl::model::Individual<ArcStr>> {
    fn from(value: BoxWrap<Individual>) -> Self {
        Into::<Box<horned_owl::model::Individual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Individual<ArcStr>>> for BoxWrap<Individual> {
    fn from(value: Box<horned_owl::model::Individual<ArcStr>>) -> Self {
        Into::<BoxWrap<Individual>>::into(value.borrow())
    }
}

impl From<VecWrap<Individual>> for Vec<horned_owl::model::Individual<ArcStr>> {
    fn from(value: VecWrap<Individual>) -> Self {
        Into::<Vec<horned_owl::model::Individual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Individual<ArcStr>>> for VecWrap<Individual> {
    fn from(value: Vec<horned_owl::model::Individual<ArcStr>>) -> Self {
        Into::<VecWrap<Individual>>::into(value.borrow())
    }
}

impl From<&VecWrap<Individual>> for Vec<horned_owl::model::Individual<ArcStr>> {
    fn from(value: &VecWrap<Individual>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Individual::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Individual<ArcStr>>> for VecWrap<Individual> {
    fn from(value: &Vec<horned_owl::model::Individual<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Individual::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Individual>> for Box<horned_owl::model::Individual<ArcStr>> {
    fn from_c(value: &BoxWrap<Individual>) -> Self {
        Box::<horned_owl::model::Individual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Individual<ArcStr>>> for BoxWrap<Individual> {
    fn from_c(value: &Box<horned_owl::model::Individual<ArcStr>>) -> Self {
        BoxWrap::<Individual>::from(value)
    }
}
impl FromCompatible<BoxWrap<Individual>> for Box<horned_owl::model::Individual<ArcStr>> {
    fn from_c(value: BoxWrap<Individual>) -> Self {
        Box::<horned_owl::model::Individual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Individual<ArcStr>>> for BoxWrap<Individual> {
    fn from_c(value: Box<horned_owl::model::Individual<ArcStr>>) -> Self {
        BoxWrap::<Individual>::from(value)
    }
}
impl FromCompatible<VecWrap<Individual>> for Vec<horned_owl::model::Individual<ArcStr>> {
    fn from_c(value: VecWrap<Individual>) -> Self {
        Vec::<horned_owl::model::Individual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Individual<ArcStr>>> for VecWrap<Individual> {
    fn from_c(value: Vec<horned_owl::model::Individual<ArcStr>>) -> Self {
        VecWrap::<Individual>::from(value)
    }
}
impl FromCompatible<&VecWrap<Individual>> for Vec<horned_owl::model::Individual<ArcStr>> {
    fn from_c(value: &VecWrap<Individual>) -> Self {
        Vec::<horned_owl::model::Individual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Individual<ArcStr>>> for VecWrap<Individual> {
    fn from_c(value: &Vec<horned_owl::model::Individual<ArcStr>>) -> Self {
        VecWrap::<Individual>::from(value)
    }
}
/**************** ENUM implementation for ObjectPropertyExpression ****************/
#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ObjectPropertyExpression_Inner {
    ObjectProperty(ObjectProperty),
	InverseObjectProperty(InverseObjectProperty),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectPropertyExpression(ObjectPropertyExpression_Inner);


/**************** ENUM VARIANTS for ObjectPropertyExpression ****************/

    /**************** ENUM VARIANT InverseObjectProperty for ObjectPropertyExpression ****************/
    #[doc = concat!("InverseObjectProperty(first: ObjectProperty",
        "\n\n",doc!(InverseObjectProperty))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InverseObjectProperty(
        #[pyo3(get,set,name="first")]
        pub ObjectProperty,
    );

    impl From<InverseObjectProperty> for ObjectPropertyExpression {
        fn from(value: InverseObjectProperty) -> Self {
            ObjectPropertyExpression(ObjectPropertyExpression_Inner::InverseObjectProperty(value))
        }
    }

    #[pymethods]
    impl InverseObjectProperty {
        #[new]
        fn new(first: ObjectProperty,) -> Self {
            InverseObjectProperty(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ObjectPropertyExpression<ArcStr>>::into(Into::<ObjectPropertyExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "object-property-expression" for InverseObjectProperty ********/
       #[pymethods]
        impl InverseObjectProperty {
            fn some(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectSomeValuesFrom> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectSomeValuesFrom {
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn only(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectAllValuesFrom> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectAllValuesFrom {
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn has_value(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectHasValue> {
                let i: Individual = obj.extract()?;
                Ok(ObjectHasValue{
                    ope: self.clone().into(),
                    i
                })
            }

            fn has_self(&self) -> PyResult<ObjectHasSelf> {
                Ok(ObjectHasSelf(self.clone().into()))
            }

            fn min(&self, n: u32, obj: &Bound<'_, PyAny>) -> PyResult<ObjectMinCardinality> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectMinCardinality {
                    n,
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn max(&self, n: u32, obj: &Bound<'_, PyAny>) -> PyResult<ObjectMaxCardinality> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectMaxCardinality {
                    n,
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn exact(&self, n: u32, obj: &Bound<'_, PyAny>) -> PyResult<ObjectExactCardinality> {
                let ce: ClassExpression = obj.extract()?;
                Ok(ObjectExactCardinality {
                    n,
                    ope: self.clone().into(),
                    bce: Box::new(ce).into(),
                })
            }

            fn __invert__(&self) -> ObjectPropertyExpression {
                let ope: ObjectPropertyExpression = self.clone().into();
                let inner: ObjectPropertyExpression_Inner = match ope.0 {
                    ObjectPropertyExpression_Inner::InverseObjectProperty(
                        InverseObjectProperty(i),
                    ) => ObjectPropertyExpression_Inner::ObjectProperty(i),
                    ObjectPropertyExpression_Inner::ObjectProperty(i) => {
                        ObjectPropertyExpression_Inner::InverseObjectProperty(
                            InverseObjectProperty(i),
                        )
                }
                };

                ObjectPropertyExpression(inner)
            }
        }

    // Transparent variant implementation
    impl From<ObjectProperty> for ObjectPropertyExpression {
        fn from(value: ObjectProperty) -> Self {
            ObjectPropertyExpression(ObjectPropertyExpression_Inner::ObjectProperty(value))
        }
    }



impl From<&horned_owl::model::ObjectPropertyExpression<ArcStr>> for ObjectPropertyExpression {
    fn from(value: &horned_owl::model::ObjectPropertyExpression<ArcStr>) -> Self {
        match value {
        		horned_owl::model::ObjectPropertyExpression::ObjectProperty::<ArcStr>(f0) =>
                    ObjectPropertyExpression(ObjectPropertyExpression_Inner::ObjectProperty(f0.into())),
                horned_owl::model::ObjectPropertyExpression::InverseObjectProperty::<ArcStr>(first) =>
                    ObjectPropertyExpression(ObjectPropertyExpression_Inner::InverseObjectProperty(InverseObjectProperty((first).into(),))),
        }
    }
}

impl From<&ObjectPropertyExpression> for horned_owl::model::ObjectPropertyExpression<ArcStr> {
    fn from(value: &ObjectPropertyExpression) -> Self {
        match value.0.borrow() {
                ObjectPropertyExpression_Inner::ObjectProperty(f0) => horned_owl::model::ObjectPropertyExpression::<ArcStr>::ObjectProperty(f0.into()),
                ObjectPropertyExpression_Inner::InverseObjectProperty(InverseObjectProperty(first)) =>
                horned_owl::model::ObjectPropertyExpression::<ArcStr>::InverseObjectProperty(
                    first.into_c(),
                ),
        }
    }
}

impl<'py> IntoPyObject<'py> for ObjectPropertyExpression {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            
                ObjectPropertyExpression_Inner::ObjectProperty(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ObjectPropertyExpression_Inner::InverseObjectProperty(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
        }
    }

}

impl <'py> FromPyObject<'py> for ObjectPropertyExpression {
    fn extract_bound(ob: &Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            {
            	let r = ObjectProperty::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ObjectPropertyExpression_Inner::ObjectProperty(local);
                    return Ok(ObjectPropertyExpression(inner));
                }
            }
            {
                let r = InverseObjectProperty::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ObjectPropertyExpression_Inner::InverseObjectProperty(local);
                    return Ok(ObjectPropertyExpression(inner));
                }
            }

        Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object cannot be converted to ObjectPropertyExpression"))
    }
}

impl ObjectPropertyExpression {
    pub fn py_def() -> String {
        "typing.Union[m.ObjectProperty,m.InverseObjectProperty,]".into()
    }
}



/**************** Base implementations for ObjectPropertyExpression ****************/
impl FromCompatible<horned_owl::model::ObjectPropertyExpression<ArcStr>> for ObjectPropertyExpression {
    fn from_c(value: horned_owl::model::ObjectPropertyExpression<ArcStr>) -> Self {
        ObjectPropertyExpression::from(value)
    }
}

impl FromCompatible<&horned_owl::model::ObjectPropertyExpression<ArcStr>> for ObjectPropertyExpression {
    fn from_c(value: &horned_owl::model::ObjectPropertyExpression<ArcStr>) -> Self {
        ObjectPropertyExpression::from(value)
    }
}

impl FromCompatible<ObjectPropertyExpression> for horned_owl::model::ObjectPropertyExpression<ArcStr> {
    fn from_c(value: ObjectPropertyExpression) -> Self {
        horned_owl::model::ObjectPropertyExpression::<ArcStr>::from(value)
    }
}

impl FromCompatible<&ObjectPropertyExpression> for horned_owl::model::ObjectPropertyExpression<ArcStr> {
    fn from_c(value: &ObjectPropertyExpression) -> Self {
        horned_owl::model::ObjectPropertyExpression::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::ObjectPropertyExpression<ArcStr>> for ObjectPropertyExpression {
    fn from(value: horned_owl::model::ObjectPropertyExpression<ArcStr>) -> Self {
        ObjectPropertyExpression::from(value.borrow())
    }
}

impl From<ObjectPropertyExpression> for horned_owl::model::ObjectPropertyExpression<ArcStr> {
    fn from(value: ObjectPropertyExpression) -> Self {
        horned_owl::model::ObjectPropertyExpression::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<ObjectPropertyExpression>> for Box<horned_owl::model::ObjectPropertyExpression<ArcStr>> {
    fn from(value: &BoxWrap<ObjectPropertyExpression>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>> for BoxWrap<ObjectPropertyExpression> {
    fn from(value: &Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<ObjectPropertyExpression>::into(*value.clone())))
    }
}

impl From<BoxWrap<ObjectPropertyExpression>> for Box<horned_owl::model::ObjectPropertyExpression<ArcStr>> {
    fn from(value: BoxWrap<ObjectPropertyExpression>) -> Self {
        Into::<Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>> for BoxWrap<ObjectPropertyExpression> {
    fn from(value: Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>) -> Self {
        Into::<BoxWrap<ObjectPropertyExpression>>::into(value.borrow())
    }
}

impl From<VecWrap<ObjectPropertyExpression>> for Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>> {
    fn from(value: VecWrap<ObjectPropertyExpression>) -> Self {
        Into::<Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>> for VecWrap<ObjectPropertyExpression> {
    fn from(value: Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>) -> Self {
        Into::<VecWrap<ObjectPropertyExpression>>::into(value.borrow())
    }
}

impl From<&VecWrap<ObjectPropertyExpression>> for Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>> {
    fn from(value: &VecWrap<ObjectPropertyExpression>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::ObjectPropertyExpression::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>> for VecWrap<ObjectPropertyExpression> {
    fn from(value: &Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>) -> Self {
        VecWrap(value.iter().map(ObjectPropertyExpression::from).collect())
    }
}

impl FromCompatible<&BoxWrap<ObjectPropertyExpression>> for Box<horned_owl::model::ObjectPropertyExpression<ArcStr>> {
    fn from_c(value: &BoxWrap<ObjectPropertyExpression>) -> Self {
        Box::<horned_owl::model::ObjectPropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>> for BoxWrap<ObjectPropertyExpression> {
    fn from_c(value: &Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>) -> Self {
        BoxWrap::<ObjectPropertyExpression>::from(value)
    }
}
impl FromCompatible<BoxWrap<ObjectPropertyExpression>> for Box<horned_owl::model::ObjectPropertyExpression<ArcStr>> {
    fn from_c(value: BoxWrap<ObjectPropertyExpression>) -> Self {
        Box::<horned_owl::model::ObjectPropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>> for BoxWrap<ObjectPropertyExpression> {
    fn from_c(value: Box<horned_owl::model::ObjectPropertyExpression<ArcStr>>) -> Self {
        BoxWrap::<ObjectPropertyExpression>::from(value)
    }
}
impl FromCompatible<VecWrap<ObjectPropertyExpression>> for Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>> {
    fn from_c(value: VecWrap<ObjectPropertyExpression>) -> Self {
        Vec::<horned_owl::model::ObjectPropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>> for VecWrap<ObjectPropertyExpression> {
    fn from_c(value: Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>) -> Self {
        VecWrap::<ObjectPropertyExpression>::from(value)
    }
}
impl FromCompatible<&VecWrap<ObjectPropertyExpression>> for Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>> {
    fn from_c(value: &VecWrap<ObjectPropertyExpression>) -> Self {
        Vec::<horned_owl::model::ObjectPropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>> for VecWrap<ObjectPropertyExpression> {
    fn from_c(value: &Vec<horned_owl::model::ObjectPropertyExpression<ArcStr>>) -> Self {
        VecWrap::<ObjectPropertyExpression>::from(value)
    }
}
/**************** ENUM implementation for Literal ****************/
#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Literal_Inner {
	Simple(SimpleLiteral),
	Language(LanguageLiteral),
	Datatype(DatatypeLiteral),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Literal(Literal_Inner);


/**************** ENUM VARIANTS for Literal ****************/

    /**************** ENUM VARIANT SimpleLiteral for Literal ****************/
    #[doc = concat!("SimpleLiteral(literal: str",
        "\n\n",doc!(SimpleLiteral))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SimpleLiteral{
        #[doc="literal: str"]
        #[pyo3(get,set)]
        pub literal: String,}

    impl From<SimpleLiteral> for Literal {
        fn from(value: SimpleLiteral) -> Self {
            Literal(Literal_Inner::Simple(value))
        }
    }

    #[pymethods]
    impl SimpleLiteral {
        #[new]
        fn new(literal: String,) -> Self {
            SimpleLiteral{
                literal,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"literal" => self.literal.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"literal" => {
                    self.literal = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Literal<ArcStr>>::into(Into::<Literal>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT LanguageLiteral for Literal ****************/
    #[doc = concat!("LanguageLiteral(literal: strlang: str",
        "\n\n",doc!(LanguageLiteral))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LanguageLiteral{
        #[doc="literal: str"]
        #[pyo3(get,set)]
        pub literal: String,
        #[doc="lang: str"]
        #[pyo3(get,set)]
        pub lang: String,}

    impl From<LanguageLiteral> for Literal {
        fn from(value: LanguageLiteral) -> Self {
            Literal(Literal_Inner::Language(value))
        }
    }

    #[pymethods]
    impl LanguageLiteral {
        #[new]
        fn new(literal: String,lang: String,) -> Self {
            LanguageLiteral{
                literal,
                lang,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"literal" => self.literal.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"lang" => self.lang.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"literal" => {
                    self.literal = value.extract()?;
                Ok(())
                },
            	"lang" => {
                    self.lang = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Literal<ArcStr>>::into(Into::<Literal>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT DatatypeLiteral for Literal ****************/
    #[doc = concat!("DatatypeLiteral(literal: strdatatype_iri: IRI",
        "\n\n",doc!(DatatypeLiteral))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DatatypeLiteral{
        #[doc="literal: str"]
        #[pyo3(get,set)]
        pub literal: String,
        #[doc="datatype_iri: IRI"]
        #[pyo3(get,set)]
        pub datatype_iri: IRI,}

    impl From<DatatypeLiteral> for Literal {
        fn from(value: DatatypeLiteral) -> Self {
            Literal(Literal_Inner::Datatype(value))
        }
    }

    #[pymethods]
    impl DatatypeLiteral {
        #[new]
        fn new(literal: String,datatype_iri: IRI,) -> Self {
            DatatypeLiteral{
                literal,
                datatype_iri,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"literal" => self.literal.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"datatype_iri" => self.datatype_iri.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"literal" => {
                    self.literal = value.extract()?;
                Ok(())
                },
            	"datatype_iri" => {
                    self.datatype_iri = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Literal<ArcStr>>::into(Into::<Literal>::into(self.clone())).as_functional().to_string()
        }
    }

    



impl From<&horned_owl::model::Literal<ArcStr>> for Literal {
    fn from(value: &horned_owl::model::Literal<ArcStr>) -> Self {
        match value {horned_owl::model::Literal::Simple::<ArcStr>{
                    literal
                } => Literal(Literal_Inner::Simple(SimpleLiteral{literal: IntoCompatible::<String>::into_c(literal),})),horned_owl::model::Literal::Language::<ArcStr>{
                    literal,
lang
                } => Literal(Literal_Inner::Language(LanguageLiteral{literal: IntoCompatible::<String>::into_c(literal),lang: IntoCompatible::<String>::into_c(lang),})),horned_owl::model::Literal::Datatype::<ArcStr>{
                    literal,
datatype_iri
                } => Literal(Literal_Inner::Datatype(DatatypeLiteral{literal: IntoCompatible::<String>::into_c(literal),datatype_iri: IntoCompatible::<IRI>::into_c(datatype_iri),})),
        }
    }
}

impl From<&Literal> for horned_owl::model::Literal<ArcStr> {
    fn from(value: &Literal) -> Self {
        match value.0.borrow() {
                Literal_Inner::Simple(SimpleLiteral{
                    literal
                }) => horned_owl::model::Literal::<ArcStr>::Simple{
                	literal: literal.into_c(),
                },
                Literal_Inner::Language(LanguageLiteral{
                    literal, lang
                }) => horned_owl::model::Literal::<ArcStr>::Language{
                	literal: literal.into_c(),
                	lang: lang.into_c(),
                },
                Literal_Inner::Datatype(DatatypeLiteral{
                    literal, datatype_iri
                }) => horned_owl::model::Literal::<ArcStr>::Datatype{
                	literal: literal.into_c(),
                	datatype_iri: datatype_iri.into_c(),
                },
        }
    }
}

impl<'py> IntoPyObject<'py> for Literal {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            
                Literal_Inner::Simple(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                Literal_Inner::Language(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                Literal_Inner::Datatype(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
        }
    }

}

impl <'py> FromPyObject<'py> for Literal {
    fn extract_bound(ob: &Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            {
                let r = SimpleLiteral::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Literal_Inner::Simple(local);
                    return Ok(Literal(inner));
                }
            }
            {
                let r = LanguageLiteral::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Literal_Inner::Language(local);
                    return Ok(Literal(inner));
                }
            }
            {
                let r = DatatypeLiteral::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Literal_Inner::Datatype(local);
                    return Ok(Literal(inner));
                }
            }

        Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object cannot be converted to Literal"))
    }
}

impl Literal {
    pub fn py_def() -> String {
        "typing.Union[m.SimpleLiteral,m.LanguageLiteral,m.DatatypeLiteral,]".into()
    }
}



/**************** Base implementations for Literal ****************/
impl FromCompatible<horned_owl::model::Literal<ArcStr>> for Literal {
    fn from_c(value: horned_owl::model::Literal<ArcStr>) -> Self {
        Literal::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Literal<ArcStr>> for Literal {
    fn from_c(value: &horned_owl::model::Literal<ArcStr>) -> Self {
        Literal::from(value)
    }
}

impl FromCompatible<Literal> for horned_owl::model::Literal<ArcStr> {
    fn from_c(value: Literal) -> Self {
        horned_owl::model::Literal::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Literal> for horned_owl::model::Literal<ArcStr> {
    fn from_c(value: &Literal) -> Self {
        horned_owl::model::Literal::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Literal<ArcStr>> for Literal {
    fn from(value: horned_owl::model::Literal<ArcStr>) -> Self {
        Literal::from(value.borrow())
    }
}

impl From<Literal> for horned_owl::model::Literal<ArcStr> {
    fn from(value: Literal) -> Self {
        horned_owl::model::Literal::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Literal>> for Box<horned_owl::model::Literal<ArcStr>> {
    fn from(value: &BoxWrap<Literal>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Literal<ArcStr>>> for BoxWrap<Literal> {
    fn from(value: &Box<horned_owl::model::Literal<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Literal>::into(*value.clone())))
    }
}

impl From<BoxWrap<Literal>> for Box<horned_owl::model::Literal<ArcStr>> {
    fn from(value: BoxWrap<Literal>) -> Self {
        Into::<Box<horned_owl::model::Literal<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Literal<ArcStr>>> for BoxWrap<Literal> {
    fn from(value: Box<horned_owl::model::Literal<ArcStr>>) -> Self {
        Into::<BoxWrap<Literal>>::into(value.borrow())
    }
}

impl From<VecWrap<Literal>> for Vec<horned_owl::model::Literal<ArcStr>> {
    fn from(value: VecWrap<Literal>) -> Self {
        Into::<Vec<horned_owl::model::Literal<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Literal<ArcStr>>> for VecWrap<Literal> {
    fn from(value: Vec<horned_owl::model::Literal<ArcStr>>) -> Self {
        Into::<VecWrap<Literal>>::into(value.borrow())
    }
}

impl From<&VecWrap<Literal>> for Vec<horned_owl::model::Literal<ArcStr>> {
    fn from(value: &VecWrap<Literal>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Literal::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Literal<ArcStr>>> for VecWrap<Literal> {
    fn from(value: &Vec<horned_owl::model::Literal<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Literal::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Literal>> for Box<horned_owl::model::Literal<ArcStr>> {
    fn from_c(value: &BoxWrap<Literal>) -> Self {
        Box::<horned_owl::model::Literal<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Literal<ArcStr>>> for BoxWrap<Literal> {
    fn from_c(value: &Box<horned_owl::model::Literal<ArcStr>>) -> Self {
        BoxWrap::<Literal>::from(value)
    }
}
impl FromCompatible<BoxWrap<Literal>> for Box<horned_owl::model::Literal<ArcStr>> {
    fn from_c(value: BoxWrap<Literal>) -> Self {
        Box::<horned_owl::model::Literal<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Literal<ArcStr>>> for BoxWrap<Literal> {
    fn from_c(value: Box<horned_owl::model::Literal<ArcStr>>) -> Self {
        BoxWrap::<Literal>::from(value)
    }
}
impl FromCompatible<VecWrap<Literal>> for Vec<horned_owl::model::Literal<ArcStr>> {
    fn from_c(value: VecWrap<Literal>) -> Self {
        Vec::<horned_owl::model::Literal<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Literal<ArcStr>>> for VecWrap<Literal> {
    fn from_c(value: Vec<horned_owl::model::Literal<ArcStr>>) -> Self {
        VecWrap::<Literal>::from(value)
    }
}
impl FromCompatible<&VecWrap<Literal>> for Vec<horned_owl::model::Literal<ArcStr>> {
    fn from_c(value: &VecWrap<Literal>) -> Self {
        Vec::<horned_owl::model::Literal<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Literal<ArcStr>>> for VecWrap<Literal> {
    fn from_c(value: &Vec<horned_owl::model::Literal<ArcStr>>) -> Self {
        VecWrap::<Literal>::from(value)
    }
}
/**************** ENUM implementation for DataRange ****************/
#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DataRange_Inner {
    Datatype(Datatype),
	DataIntersectionOf(DataIntersectionOf),
	DataUnionOf(DataUnionOf),
	DataComplementOf(DataComplementOf),
	DataOneOf(DataOneOf),
	DatatypeRestriction(DatatypeRestriction),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataRange(DataRange_Inner);


/**************** ENUM VARIANTS for DataRange ****************/

    /**************** ENUM VARIANT DataIntersectionOf for DataRange ****************/
    #[doc = concat!("DataIntersectionOf(first: typing.List[DataRange]",
        "\n\n",doc!(DataIntersectionOf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataIntersectionOf(
        #[pyo3(get,set,name="first")]
        pub VecWrap<DataRange>,
    );

    impl From<DataIntersectionOf> for DataRange {
        fn from(value: DataIntersectionOf) -> Self {
            DataRange(DataRange_Inner::DataIntersectionOf(value))
        }
    }

    #[pymethods]
    impl DataIntersectionOf {
        #[new]
        fn new(first: VecWrap<DataRange>,) -> Self {
            DataIntersectionOf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::DataRange<ArcStr>>::into(Into::<DataRange>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT DataUnionOf for DataRange ****************/
    #[doc = concat!("DataUnionOf(first: typing.List[DataRange]",
        "\n\n",doc!(DataUnionOf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataUnionOf(
        #[pyo3(get,set,name="first")]
        pub VecWrap<DataRange>,
    );

    impl From<DataUnionOf> for DataRange {
        fn from(value: DataUnionOf) -> Self {
            DataRange(DataRange_Inner::DataUnionOf(value))
        }
    }

    #[pymethods]
    impl DataUnionOf {
        #[new]
        fn new(first: VecWrap<DataRange>,) -> Self {
            DataUnionOf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::DataRange<ArcStr>>::into(Into::<DataRange>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT DataComplementOf for DataRange ****************/
    #[doc = concat!("DataComplementOf(first: DataRange",
        "\n\n",doc!(DataComplementOf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataComplementOf(
        #[pyo3(get,set,name="first")]
        pub BoxWrap<DataRange>,
    );

    impl From<DataComplementOf> for DataRange {
        fn from(value: DataComplementOf) -> Self {
            DataRange(DataRange_Inner::DataComplementOf(value))
        }
    }

    #[pymethods]
    impl DataComplementOf {
        #[new]
        fn new(first: BoxWrap<DataRange>,) -> Self {
            DataComplementOf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::DataRange<ArcStr>>::into(Into::<DataRange>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT DataOneOf for DataRange ****************/
    #[doc = concat!("DataOneOf(first: typing.List[Literal]",
        "\n\n",doc!(DataOneOf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataOneOf(
        #[pyo3(get,set,name="first")]
        pub VecWrap<Literal>,
    );

    impl From<DataOneOf> for DataRange {
        fn from(value: DataOneOf) -> Self {
            DataRange(DataRange_Inner::DataOneOf(value))
        }
    }

    #[pymethods]
    impl DataOneOf {
        #[new]
        fn new(first: VecWrap<Literal>,) -> Self {
            DataOneOf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::DataRange<ArcStr>>::into(Into::<DataRange>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT DatatypeRestriction for DataRange ****************/
    #[doc = concat!("DatatypeRestriction(first: Datatypesecond: typing.List[FacetRestriction]",
        "\n\n",doc!(DatatypeRestriction))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DatatypeRestriction(
        #[pyo3(get,set,name="first")]
        pub Datatype,
        #[pyo3(get,set,name="second")]
        pub VecWrap<FacetRestriction>,
    );

    impl From<DatatypeRestriction> for DataRange {
        fn from(value: DatatypeRestriction) -> Self {
            DataRange(DataRange_Inner::DatatypeRestriction(value))
        }
    }

    #[pymethods]
    impl DatatypeRestriction {
        #[new]
        fn new(first: Datatype,second: VecWrap<FacetRestriction>,) -> Self {
            DatatypeRestriction(
                first,
                second,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"second" => self.1.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
            	"second" => {
                    self.1 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::DataRange<ArcStr>>::into(Into::<DataRange>::into(self.clone())).as_functional().to_string()
        }
    }

    

    // Transparent variant implementation
    impl From<Datatype> for DataRange {
        fn from(value: Datatype) -> Self {
            DataRange(DataRange_Inner::Datatype(value))
        }
    }



impl From<&horned_owl::model::DataRange<ArcStr>> for DataRange {
    fn from(value: &horned_owl::model::DataRange<ArcStr>) -> Self {
        match value {
        		horned_owl::model::DataRange::Datatype::<ArcStr>(f0) =>
                    DataRange(DataRange_Inner::Datatype(f0.into())),
                horned_owl::model::DataRange::DataIntersectionOf::<ArcStr>(first) =>
                    DataRange(DataRange_Inner::DataIntersectionOf(DataIntersectionOf((first).into(),))),
                horned_owl::model::DataRange::DataUnionOf::<ArcStr>(first) =>
                    DataRange(DataRange_Inner::DataUnionOf(DataUnionOf((first).into(),))),
                horned_owl::model::DataRange::DataComplementOf::<ArcStr>(first) =>
                    DataRange(DataRange_Inner::DataComplementOf(DataComplementOf((first).into(),))),
                horned_owl::model::DataRange::DataOneOf::<ArcStr>(first) =>
                    DataRange(DataRange_Inner::DataOneOf(DataOneOf((first).into(),))),
                horned_owl::model::DataRange::DatatypeRestriction::<ArcStr>(first, second) =>
                    DataRange(DataRange_Inner::DatatypeRestriction(DatatypeRestriction((first).into(),(second).into(),))),
        }
    }
}

impl From<&DataRange> for horned_owl::model::DataRange<ArcStr> {
    fn from(value: &DataRange) -> Self {
        match value.0.borrow() {
                DataRange_Inner::Datatype(f0) => horned_owl::model::DataRange::<ArcStr>::Datatype(f0.into()),
                DataRange_Inner::DataIntersectionOf(DataIntersectionOf(first)) =>
                horned_owl::model::DataRange::<ArcStr>::DataIntersectionOf(
                    first.into_c(),
                ),
                DataRange_Inner::DataUnionOf(DataUnionOf(first)) =>
                horned_owl::model::DataRange::<ArcStr>::DataUnionOf(
                    first.into_c(),
                ),
                DataRange_Inner::DataComplementOf(DataComplementOf(first)) =>
                horned_owl::model::DataRange::<ArcStr>::DataComplementOf(
                    first.into_c(),
                ),
                DataRange_Inner::DataOneOf(DataOneOf(first)) =>
                horned_owl::model::DataRange::<ArcStr>::DataOneOf(
                    first.into_c(),
                ),
                DataRange_Inner::DatatypeRestriction(DatatypeRestriction(first, second)) =>
                horned_owl::model::DataRange::<ArcStr>::DatatypeRestriction(
                    first.into_c(),
                    second.into_c(),
                ),
        }
    }
}

impl<'py> IntoPyObject<'py> for DataRange {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            
                DataRange_Inner::Datatype(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                DataRange_Inner::DataIntersectionOf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                DataRange_Inner::DataUnionOf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                DataRange_Inner::DataComplementOf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                DataRange_Inner::DataOneOf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                DataRange_Inner::DatatypeRestriction(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
        }
    }

}

impl <'py> FromPyObject<'py> for DataRange {
    fn extract_bound(ob: &Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            {
            	let r = Datatype::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = DataRange_Inner::Datatype(local);
                    return Ok(DataRange(inner));
                }
            }
            {
                let r = DataIntersectionOf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = DataRange_Inner::DataIntersectionOf(local);
                    return Ok(DataRange(inner));
                }
            }
            {
                let r = DataUnionOf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = DataRange_Inner::DataUnionOf(local);
                    return Ok(DataRange(inner));
                }
            }
            {
                let r = DataComplementOf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = DataRange_Inner::DataComplementOf(local);
                    return Ok(DataRange(inner));
                }
            }
            {
                let r = DataOneOf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = DataRange_Inner::DataOneOf(local);
                    return Ok(DataRange(inner));
                }
            }
            {
                let r = DatatypeRestriction::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = DataRange_Inner::DatatypeRestriction(local);
                    return Ok(DataRange(inner));
                }
            }

        Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object cannot be converted to DataRange"))
    }
}

impl DataRange {
    pub fn py_def() -> String {
        "typing.Union[m.Datatype,m.DataIntersectionOf,m.DataUnionOf,m.DataComplementOf,m.DataOneOf,m.DatatypeRestriction,]".into()
    }
}



/**************** Base implementations for DataRange ****************/
impl FromCompatible<horned_owl::model::DataRange<ArcStr>> for DataRange {
    fn from_c(value: horned_owl::model::DataRange<ArcStr>) -> Self {
        DataRange::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DataRange<ArcStr>> for DataRange {
    fn from_c(value: &horned_owl::model::DataRange<ArcStr>) -> Self {
        DataRange::from(value)
    }
}

impl FromCompatible<DataRange> for horned_owl::model::DataRange<ArcStr> {
    fn from_c(value: DataRange) -> Self {
        horned_owl::model::DataRange::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DataRange> for horned_owl::model::DataRange<ArcStr> {
    fn from_c(value: &DataRange) -> Self {
        horned_owl::model::DataRange::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DataRange<ArcStr>> for DataRange {
    fn from(value: horned_owl::model::DataRange<ArcStr>) -> Self {
        DataRange::from(value.borrow())
    }
}

impl From<DataRange> for horned_owl::model::DataRange<ArcStr> {
    fn from(value: DataRange) -> Self {
        horned_owl::model::DataRange::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DataRange>> for Box<horned_owl::model::DataRange<ArcStr>> {
    fn from(value: &BoxWrap<DataRange>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DataRange<ArcStr>>> for BoxWrap<DataRange> {
    fn from(value: &Box<horned_owl::model::DataRange<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DataRange>::into(*value.clone())))
    }
}

impl From<BoxWrap<DataRange>> for Box<horned_owl::model::DataRange<ArcStr>> {
    fn from(value: BoxWrap<DataRange>) -> Self {
        Into::<Box<horned_owl::model::DataRange<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DataRange<ArcStr>>> for BoxWrap<DataRange> {
    fn from(value: Box<horned_owl::model::DataRange<ArcStr>>) -> Self {
        Into::<BoxWrap<DataRange>>::into(value.borrow())
    }
}

impl From<VecWrap<DataRange>> for Vec<horned_owl::model::DataRange<ArcStr>> {
    fn from(value: VecWrap<DataRange>) -> Self {
        Into::<Vec<horned_owl::model::DataRange<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DataRange<ArcStr>>> for VecWrap<DataRange> {
    fn from(value: Vec<horned_owl::model::DataRange<ArcStr>>) -> Self {
        Into::<VecWrap<DataRange>>::into(value.borrow())
    }
}

impl From<&VecWrap<DataRange>> for Vec<horned_owl::model::DataRange<ArcStr>> {
    fn from(value: &VecWrap<DataRange>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DataRange::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DataRange<ArcStr>>> for VecWrap<DataRange> {
    fn from(value: &Vec<horned_owl::model::DataRange<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DataRange::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DataRange>> for Box<horned_owl::model::DataRange<ArcStr>> {
    fn from_c(value: &BoxWrap<DataRange>) -> Self {
        Box::<horned_owl::model::DataRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DataRange<ArcStr>>> for BoxWrap<DataRange> {
    fn from_c(value: &Box<horned_owl::model::DataRange<ArcStr>>) -> Self {
        BoxWrap::<DataRange>::from(value)
    }
}
impl FromCompatible<BoxWrap<DataRange>> for Box<horned_owl::model::DataRange<ArcStr>> {
    fn from_c(value: BoxWrap<DataRange>) -> Self {
        Box::<horned_owl::model::DataRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DataRange<ArcStr>>> for BoxWrap<DataRange> {
    fn from_c(value: Box<horned_owl::model::DataRange<ArcStr>>) -> Self {
        BoxWrap::<DataRange>::from(value)
    }
}
impl FromCompatible<VecWrap<DataRange>> for Vec<horned_owl::model::DataRange<ArcStr>> {
    fn from_c(value: VecWrap<DataRange>) -> Self {
        Vec::<horned_owl::model::DataRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DataRange<ArcStr>>> for VecWrap<DataRange> {
    fn from_c(value: Vec<horned_owl::model::DataRange<ArcStr>>) -> Self {
        VecWrap::<DataRange>::from(value)
    }
}
impl FromCompatible<&VecWrap<DataRange>> for Vec<horned_owl::model::DataRange<ArcStr>> {
    fn from_c(value: &VecWrap<DataRange>) -> Self {
        Vec::<horned_owl::model::DataRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DataRange<ArcStr>>> for VecWrap<DataRange> {
    fn from_c(value: &Vec<horned_owl::model::DataRange<ArcStr>>) -> Self {
        VecWrap::<DataRange>::from(value)
    }
}
/**************** ENUM implementation for ClassExpression ****************/
#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ClassExpression_Inner {
    Class(Class),
	ObjectIntersectionOf(ObjectIntersectionOf),
	ObjectUnionOf(ObjectUnionOf),
	ObjectComplementOf(ObjectComplementOf),
	ObjectOneOf(ObjectOneOf),
	ObjectSomeValuesFrom(ObjectSomeValuesFrom),
	ObjectAllValuesFrom(ObjectAllValuesFrom),
	ObjectHasValue(ObjectHasValue),
	ObjectHasSelf(ObjectHasSelf),
	ObjectMinCardinality(ObjectMinCardinality),
	ObjectMaxCardinality(ObjectMaxCardinality),
	ObjectExactCardinality(ObjectExactCardinality),
	DataSomeValuesFrom(DataSomeValuesFrom),
	DataAllValuesFrom(DataAllValuesFrom),
	DataHasValue(DataHasValue),
	DataMinCardinality(DataMinCardinality),
	DataMaxCardinality(DataMaxCardinality),
	DataExactCardinality(DataExactCardinality),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassExpression(ClassExpression_Inner);


/**************** ENUM VARIANTS for ClassExpression ****************/

    /**************** ENUM VARIANT ObjectIntersectionOf for ClassExpression ****************/
    #[doc = concat!("ObjectIntersectionOf(first: typing.List[ClassExpression]",
        "\n\n",doc!(ObjectIntersectionOf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectIntersectionOf(
        #[pyo3(get,set,name="first")]
        pub VecWrap<ClassExpression>,
    );

    impl From<ObjectIntersectionOf> for ClassExpression {
        fn from(value: ObjectIntersectionOf) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectIntersectionOf(value))
        }
    }

    #[pymethods]
    impl ObjectIntersectionOf {
        #[new]
        fn new(first: VecWrap<ClassExpression>,) -> Self {
            ObjectIntersectionOf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectIntersectionOf ********/
    #[pymethods]
    impl ObjectIntersectionOf {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectUnionOf for ClassExpression ****************/
    #[doc = concat!("ObjectUnionOf(first: typing.List[ClassExpression]",
        "\n\n",doc!(ObjectUnionOf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectUnionOf(
        #[pyo3(get,set,name="first")]
        pub VecWrap<ClassExpression>,
    );

    impl From<ObjectUnionOf> for ClassExpression {
        fn from(value: ObjectUnionOf) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectUnionOf(value))
        }
    }

    #[pymethods]
    impl ObjectUnionOf {
        #[new]
        fn new(first: VecWrap<ClassExpression>,) -> Self {
            ObjectUnionOf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectUnionOf ********/
    #[pymethods]
    impl ObjectUnionOf {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectComplementOf for ClassExpression ****************/
    #[doc = concat!("ObjectComplementOf(first: ClassExpression",
        "\n\n",doc!(ObjectComplementOf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectComplementOf(
        #[pyo3(get,set,name="first")]
        pub BoxWrap<ClassExpression>,
    );

    impl From<ObjectComplementOf> for ClassExpression {
        fn from(value: ObjectComplementOf) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectComplementOf(value))
        }
    }

    #[pymethods]
    impl ObjectComplementOf {
        #[new]
        fn new(first: BoxWrap<ClassExpression>,) -> Self {
            ObjectComplementOf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectComplementOf ********/
    #[pymethods]
    impl ObjectComplementOf {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectOneOf for ClassExpression ****************/
    #[doc = concat!("ObjectOneOf(first: typing.List[Individual]",
        "\n\n",doc!(ObjectOneOf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectOneOf(
        #[pyo3(get,set,name="first")]
        pub VecWrap<Individual>,
    );

    impl From<ObjectOneOf> for ClassExpression {
        fn from(value: ObjectOneOf) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectOneOf(value))
        }
    }

    #[pymethods]
    impl ObjectOneOf {
        #[new]
        fn new(first: VecWrap<Individual>,) -> Self {
            ObjectOneOf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectOneOf ********/
    #[pymethods]
    impl ObjectOneOf {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectSomeValuesFrom for ClassExpression ****************/
    #[doc = concat!("ObjectSomeValuesFrom(ope: ObjectPropertyExpressionbce: ClassExpression",
        "\n\n",doc!(ObjectSomeValuesFrom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectSomeValuesFrom{
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
        #[doc="bce: ClassExpression"]
        #[pyo3(get,set)]
        pub bce: BoxWrap<ClassExpression>,}

    impl From<ObjectSomeValuesFrom> for ClassExpression {
        fn from(value: ObjectSomeValuesFrom) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectSomeValuesFrom(value))
        }
    }

    #[pymethods]
    impl ObjectSomeValuesFrom {
        #[new]
        fn new(ope: ObjectPropertyExpression,bce: BoxWrap<ClassExpression>,) -> Self {
            ObjectSomeValuesFrom{
                ope,
                bce,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"bce" => self.bce.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"ope" => {
                    self.ope = value.extract()?;
                Ok(())
                },
            	"bce" => {
                    self.bce = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectSomeValuesFrom ********/
    #[pymethods]
    impl ObjectSomeValuesFrom {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectAllValuesFrom for ClassExpression ****************/
    #[doc = concat!("ObjectAllValuesFrom(ope: ObjectPropertyExpressionbce: ClassExpression",
        "\n\n",doc!(ObjectAllValuesFrom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectAllValuesFrom{
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
        #[doc="bce: ClassExpression"]
        #[pyo3(get,set)]
        pub bce: BoxWrap<ClassExpression>,}

    impl From<ObjectAllValuesFrom> for ClassExpression {
        fn from(value: ObjectAllValuesFrom) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectAllValuesFrom(value))
        }
    }

    #[pymethods]
    impl ObjectAllValuesFrom {
        #[new]
        fn new(ope: ObjectPropertyExpression,bce: BoxWrap<ClassExpression>,) -> Self {
            ObjectAllValuesFrom{
                ope,
                bce,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"bce" => self.bce.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"ope" => {
                    self.ope = value.extract()?;
                Ok(())
                },
            	"bce" => {
                    self.bce = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectAllValuesFrom ********/
    #[pymethods]
    impl ObjectAllValuesFrom {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectHasValue for ClassExpression ****************/
    #[doc = concat!("ObjectHasValue(ope: ObjectPropertyExpressioni: Individual",
        "\n\n",doc!(ObjectHasValue))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectHasValue{
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
        #[doc="i: Individual"]
        #[pyo3(get,set)]
        pub i: Individual,}

    impl From<ObjectHasValue> for ClassExpression {
        fn from(value: ObjectHasValue) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectHasValue(value))
        }
    }

    #[pymethods]
    impl ObjectHasValue {
        #[new]
        fn new(ope: ObjectPropertyExpression,i: Individual,) -> Self {
            ObjectHasValue{
                ope,
                i,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"i" => self.i.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"ope" => {
                    self.ope = value.extract()?;
                Ok(())
                },
            	"i" => {
                    self.i = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectHasValue ********/
    #[pymethods]
    impl ObjectHasValue {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectHasSelf for ClassExpression ****************/
    #[doc = concat!("ObjectHasSelf(first: ObjectPropertyExpression",
        "\n\n",doc!(ObjectHasSelf))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectHasSelf(
        #[pyo3(get,set,name="first")]
        pub ObjectPropertyExpression,
    );

    impl From<ObjectHasSelf> for ClassExpression {
        fn from(value: ObjectHasSelf) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectHasSelf(value))
        }
    }

    #[pymethods]
    impl ObjectHasSelf {
        #[new]
        fn new(first: ObjectPropertyExpression,) -> Self {
            ObjectHasSelf(
                first,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectHasSelf ********/
    #[pymethods]
    impl ObjectHasSelf {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectMinCardinality for ClassExpression ****************/
    #[doc = concat!("ObjectMinCardinality(n: intope: ObjectPropertyExpressionbce: ClassExpression",
        "\n\n",doc!(ObjectMinCardinality))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectMinCardinality{
        #[doc="n: int"]
        #[pyo3(get,set)]
        pub n: u32,
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
        #[doc="bce: ClassExpression"]
        #[pyo3(get,set)]
        pub bce: BoxWrap<ClassExpression>,}

    impl From<ObjectMinCardinality> for ClassExpression {
        fn from(value: ObjectMinCardinality) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectMinCardinality(value))
        }
    }

    #[pymethods]
    impl ObjectMinCardinality {
        #[new]
        fn new(n: u32,ope: ObjectPropertyExpression,bce: BoxWrap<ClassExpression>,) -> Self {
            ObjectMinCardinality{
                n,
                ope,
                bce,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"n" => self.n.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"bce" => self.bce.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"n" => {
                    self.n = value.extract()?;
                Ok(())
                },
            	"ope" => {
                    self.ope = value.extract()?;
                Ok(())
                },
            	"bce" => {
                    self.bce = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectMinCardinality ********/
    #[pymethods]
    impl ObjectMinCardinality {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectMaxCardinality for ClassExpression ****************/
    #[doc = concat!("ObjectMaxCardinality(n: intope: ObjectPropertyExpressionbce: ClassExpression",
        "\n\n",doc!(ObjectMaxCardinality))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectMaxCardinality{
        #[doc="n: int"]
        #[pyo3(get,set)]
        pub n: u32,
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
        #[doc="bce: ClassExpression"]
        #[pyo3(get,set)]
        pub bce: BoxWrap<ClassExpression>,}

    impl From<ObjectMaxCardinality> for ClassExpression {
        fn from(value: ObjectMaxCardinality) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectMaxCardinality(value))
        }
    }

    #[pymethods]
    impl ObjectMaxCardinality {
        #[new]
        fn new(n: u32,ope: ObjectPropertyExpression,bce: BoxWrap<ClassExpression>,) -> Self {
            ObjectMaxCardinality{
                n,
                ope,
                bce,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"n" => self.n.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"bce" => self.bce.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"n" => {
                    self.n = value.extract()?;
                Ok(())
                },
            	"ope" => {
                    self.ope = value.extract()?;
                Ok(())
                },
            	"bce" => {
                    self.bce = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectMaxCardinality ********/
    #[pymethods]
    impl ObjectMaxCardinality {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT ObjectExactCardinality for ClassExpression ****************/
    #[doc = concat!("ObjectExactCardinality(n: intope: ObjectPropertyExpressionbce: ClassExpression",
        "\n\n",doc!(ObjectExactCardinality))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectExactCardinality{
        #[doc="n: int"]
        #[pyo3(get,set)]
        pub n: u32,
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
        #[doc="bce: ClassExpression"]
        #[pyo3(get,set)]
        pub bce: BoxWrap<ClassExpression>,}

    impl From<ObjectExactCardinality> for ClassExpression {
        fn from(value: ObjectExactCardinality) -> Self {
            ClassExpression(ClassExpression_Inner::ObjectExactCardinality(value))
        }
    }

    #[pymethods]
    impl ObjectExactCardinality {
        #[new]
        fn new(n: u32,ope: ObjectPropertyExpression,bce: BoxWrap<ClassExpression>,) -> Self {
            ObjectExactCardinality{
                n,
                ope,
                bce,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"n" => self.n.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"bce" => self.bce.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"n" => {
                    self.n = value.extract()?;
                Ok(())
                },
            	"ope" => {
                    self.ope = value.extract()?;
                Ok(())
                },
            	"bce" => {
                    self.bce = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for ObjectExactCardinality ********/
    #[pymethods]
    impl ObjectExactCardinality {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT DataSomeValuesFrom for ClassExpression ****************/
    #[doc = concat!("DataSomeValuesFrom(dp: DataPropertydr: DataRange",
        "\n\n",doc!(DataSomeValuesFrom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataSomeValuesFrom{
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
        #[doc="dr: DataRange"]
        #[pyo3(get,set)]
        pub dr: DataRange,}

    impl From<DataSomeValuesFrom> for ClassExpression {
        fn from(value: DataSomeValuesFrom) -> Self {
            ClassExpression(ClassExpression_Inner::DataSomeValuesFrom(value))
        }
    }

    #[pymethods]
    impl DataSomeValuesFrom {
        #[new]
        fn new(dp: DataProperty,dr: DataRange,) -> Self {
            DataSomeValuesFrom{
                dp,
                dr,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"dr" => self.dr.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"dp" => {
                    self.dp = value.extract()?;
                Ok(())
                },
            	"dr" => {
                    self.dr = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for DataSomeValuesFrom ********/
    #[pymethods]
    impl DataSomeValuesFrom {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT DataAllValuesFrom for ClassExpression ****************/
    #[doc = concat!("DataAllValuesFrom(dp: DataPropertydr: DataRange",
        "\n\n",doc!(DataAllValuesFrom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataAllValuesFrom{
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
        #[doc="dr: DataRange"]
        #[pyo3(get,set)]
        pub dr: DataRange,}

    impl From<DataAllValuesFrom> for ClassExpression {
        fn from(value: DataAllValuesFrom) -> Self {
            ClassExpression(ClassExpression_Inner::DataAllValuesFrom(value))
        }
    }

    #[pymethods]
    impl DataAllValuesFrom {
        #[new]
        fn new(dp: DataProperty,dr: DataRange,) -> Self {
            DataAllValuesFrom{
                dp,
                dr,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"dr" => self.dr.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"dp" => {
                    self.dp = value.extract()?;
                Ok(())
                },
            	"dr" => {
                    self.dr = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for DataAllValuesFrom ********/
    #[pymethods]
    impl DataAllValuesFrom {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT DataHasValue for ClassExpression ****************/
    #[doc = concat!("DataHasValue(dp: DataPropertyl: Literal",
        "\n\n",doc!(DataHasValue))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataHasValue{
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
        #[doc="l: Literal"]
        #[pyo3(get,set)]
        pub l: Literal,}

    impl From<DataHasValue> for ClassExpression {
        fn from(value: DataHasValue) -> Self {
            ClassExpression(ClassExpression_Inner::DataHasValue(value))
        }
    }

    #[pymethods]
    impl DataHasValue {
        #[new]
        fn new(dp: DataProperty,l: Literal,) -> Self {
            DataHasValue{
                dp,
                l,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"l" => self.l.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"dp" => {
                    self.dp = value.extract()?;
                Ok(())
                },
            	"l" => {
                    self.l = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for DataHasValue ********/
    #[pymethods]
    impl DataHasValue {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT DataMinCardinality for ClassExpression ****************/
    #[doc = concat!("DataMinCardinality(n: intdp: DataPropertydr: DataRange",
        "\n\n",doc!(DataMinCardinality))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataMinCardinality{
        #[doc="n: int"]
        #[pyo3(get,set)]
        pub n: u32,
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
        #[doc="dr: DataRange"]
        #[pyo3(get,set)]
        pub dr: DataRange,}

    impl From<DataMinCardinality> for ClassExpression {
        fn from(value: DataMinCardinality) -> Self {
            ClassExpression(ClassExpression_Inner::DataMinCardinality(value))
        }
    }

    #[pymethods]
    impl DataMinCardinality {
        #[new]
        fn new(n: u32,dp: DataProperty,dr: DataRange,) -> Self {
            DataMinCardinality{
                n,
                dp,
                dr,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"n" => self.n.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"dr" => self.dr.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"n" => {
                    self.n = value.extract()?;
                Ok(())
                },
            	"dp" => {
                    self.dp = value.extract()?;
                Ok(())
                },
            	"dr" => {
                    self.dr = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for DataMinCardinality ********/
    #[pymethods]
    impl DataMinCardinality {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT DataMaxCardinality for ClassExpression ****************/
    #[doc = concat!("DataMaxCardinality(n: intdp: DataPropertydr: DataRange",
        "\n\n",doc!(DataMaxCardinality))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataMaxCardinality{
        #[doc="n: int"]
        #[pyo3(get,set)]
        pub n: u32,
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
        #[doc="dr: DataRange"]
        #[pyo3(get,set)]
        pub dr: DataRange,}

    impl From<DataMaxCardinality> for ClassExpression {
        fn from(value: DataMaxCardinality) -> Self {
            ClassExpression(ClassExpression_Inner::DataMaxCardinality(value))
        }
    }

    #[pymethods]
    impl DataMaxCardinality {
        #[new]
        fn new(n: u32,dp: DataProperty,dr: DataRange,) -> Self {
            DataMaxCardinality{
                n,
                dp,
                dr,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"n" => self.n.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"dr" => self.dr.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"n" => {
                    self.n = value.extract()?;
                Ok(())
                },
            	"dp" => {
                    self.dp = value.extract()?;
                Ok(())
                },
            	"dr" => {
                    self.dr = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for DataMaxCardinality ********/
    #[pymethods]
    impl DataMaxCardinality {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }


    /**************** ENUM VARIANT DataExactCardinality for ClassExpression ****************/
    #[doc = concat!("DataExactCardinality(n: intdp: DataPropertydr: DataRange",
        "\n\n",doc!(DataExactCardinality))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataExactCardinality{
        #[doc="n: int"]
        #[pyo3(get,set)]
        pub n: u32,
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
        #[doc="dr: DataRange"]
        #[pyo3(get,set)]
        pub dr: DataRange,}

    impl From<DataExactCardinality> for ClassExpression {
        fn from(value: DataExactCardinality) -> Self {
            ClassExpression(ClassExpression_Inner::DataExactCardinality(value))
        }
    }

    #[pymethods]
    impl DataExactCardinality {
        #[new]
        fn new(n: u32,dp: DataProperty,dr: DataRange,) -> Self {
            DataExactCardinality{
                n,
                dp,
                dr,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"n" => self.n.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"dr" => self.dr.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"n" => {
                    self.n = value.extract()?;
                Ok(())
                },
            	"dp" => {
                    self.dp = value.extract()?;
                Ok(())
                },
            	"dr" => {
                    self.dr = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::ClassExpression<ArcStr>>::into(Into::<ClassExpression>::into(self.clone())).as_functional().to_string()
        }
    }

    
    /******** EXTENSION "class-expression" for DataExactCardinality ********/
    #[pymethods]
    impl DataExactCardinality {
        fn __and__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectIntersectionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectIntersectionOf(vec![self.clone().into(), ce].into()))
        }

        fn __or__(&self, obj: &Bound<'_, PyAny>) -> PyResult<ObjectUnionOf> {
            let ce: ClassExpression = obj.extract()?;
            Ok(ObjectUnionOf(vec![self.clone().into(), ce].into()))
        }

        fn __invert__(&self) -> ObjectComplementOf {
            ObjectComplementOf(Box::<ClassExpression>::new(self.clone().into()).into())
        }
    }

    // Transparent variant implementation
    impl From<Class> for ClassExpression {
        fn from(value: Class) -> Self {
            ClassExpression(ClassExpression_Inner::Class(value))
        }
    }



impl From<&horned_owl::model::ClassExpression<ArcStr>> for ClassExpression {
    fn from(value: &horned_owl::model::ClassExpression<ArcStr>) -> Self {
        match value {
        		horned_owl::model::ClassExpression::Class::<ArcStr>(f0) =>
                    ClassExpression(ClassExpression_Inner::Class(f0.into())),
                horned_owl::model::ClassExpression::ObjectIntersectionOf::<ArcStr>(first) =>
                    ClassExpression(ClassExpression_Inner::ObjectIntersectionOf(ObjectIntersectionOf((first).into(),))),
                horned_owl::model::ClassExpression::ObjectUnionOf::<ArcStr>(first) =>
                    ClassExpression(ClassExpression_Inner::ObjectUnionOf(ObjectUnionOf((first).into(),))),
                horned_owl::model::ClassExpression::ObjectComplementOf::<ArcStr>(first) =>
                    ClassExpression(ClassExpression_Inner::ObjectComplementOf(ObjectComplementOf((first).into(),))),
                horned_owl::model::ClassExpression::ObjectOneOf::<ArcStr>(first) =>
                    ClassExpression(ClassExpression_Inner::ObjectOneOf(ObjectOneOf((first).into(),))),horned_owl::model::ClassExpression::ObjectSomeValuesFrom::<ArcStr>{
                    ope,
bce
                } => ClassExpression(ClassExpression_Inner::ObjectSomeValuesFrom(ObjectSomeValuesFrom{ope: IntoCompatible::<ObjectPropertyExpression>::into_c(ope),bce: IntoCompatible::<BoxWrap<ClassExpression>>::into_c(bce),})),horned_owl::model::ClassExpression::ObjectAllValuesFrom::<ArcStr>{
                    ope,
bce
                } => ClassExpression(ClassExpression_Inner::ObjectAllValuesFrom(ObjectAllValuesFrom{ope: IntoCompatible::<ObjectPropertyExpression>::into_c(ope),bce: IntoCompatible::<BoxWrap<ClassExpression>>::into_c(bce),})),horned_owl::model::ClassExpression::ObjectHasValue::<ArcStr>{
                    ope,
i
                } => ClassExpression(ClassExpression_Inner::ObjectHasValue(ObjectHasValue{ope: IntoCompatible::<ObjectPropertyExpression>::into_c(ope),i: IntoCompatible::<Individual>::into_c(i),})),
                horned_owl::model::ClassExpression::ObjectHasSelf::<ArcStr>(first) =>
                    ClassExpression(ClassExpression_Inner::ObjectHasSelf(ObjectHasSelf((first).into(),))),horned_owl::model::ClassExpression::ObjectMinCardinality::<ArcStr>{
                    n,
ope,
bce
                } => ClassExpression(ClassExpression_Inner::ObjectMinCardinality(ObjectMinCardinality{n: IntoCompatible::<u32>::into_c(n),ope: IntoCompatible::<ObjectPropertyExpression>::into_c(ope),bce: IntoCompatible::<BoxWrap<ClassExpression>>::into_c(bce),})),horned_owl::model::ClassExpression::ObjectMaxCardinality::<ArcStr>{
                    n,
ope,
bce
                } => ClassExpression(ClassExpression_Inner::ObjectMaxCardinality(ObjectMaxCardinality{n: IntoCompatible::<u32>::into_c(n),ope: IntoCompatible::<ObjectPropertyExpression>::into_c(ope),bce: IntoCompatible::<BoxWrap<ClassExpression>>::into_c(bce),})),horned_owl::model::ClassExpression::ObjectExactCardinality::<ArcStr>{
                    n,
ope,
bce
                } => ClassExpression(ClassExpression_Inner::ObjectExactCardinality(ObjectExactCardinality{n: IntoCompatible::<u32>::into_c(n),ope: IntoCompatible::<ObjectPropertyExpression>::into_c(ope),bce: IntoCompatible::<BoxWrap<ClassExpression>>::into_c(bce),})),horned_owl::model::ClassExpression::DataSomeValuesFrom::<ArcStr>{
                    dp,
dr
                } => ClassExpression(ClassExpression_Inner::DataSomeValuesFrom(DataSomeValuesFrom{dp: IntoCompatible::<DataProperty>::into_c(dp),dr: IntoCompatible::<DataRange>::into_c(dr),})),horned_owl::model::ClassExpression::DataAllValuesFrom::<ArcStr>{
                    dp,
dr
                } => ClassExpression(ClassExpression_Inner::DataAllValuesFrom(DataAllValuesFrom{dp: IntoCompatible::<DataProperty>::into_c(dp),dr: IntoCompatible::<DataRange>::into_c(dr),})),horned_owl::model::ClassExpression::DataHasValue::<ArcStr>{
                    dp,
l
                } => ClassExpression(ClassExpression_Inner::DataHasValue(DataHasValue{dp: IntoCompatible::<DataProperty>::into_c(dp),l: IntoCompatible::<Literal>::into_c(l),})),horned_owl::model::ClassExpression::DataMinCardinality::<ArcStr>{
                    n,
dp,
dr
                } => ClassExpression(ClassExpression_Inner::DataMinCardinality(DataMinCardinality{n: IntoCompatible::<u32>::into_c(n),dp: IntoCompatible::<DataProperty>::into_c(dp),dr: IntoCompatible::<DataRange>::into_c(dr),})),horned_owl::model::ClassExpression::DataMaxCardinality::<ArcStr>{
                    n,
dp,
dr
                } => ClassExpression(ClassExpression_Inner::DataMaxCardinality(DataMaxCardinality{n: IntoCompatible::<u32>::into_c(n),dp: IntoCompatible::<DataProperty>::into_c(dp),dr: IntoCompatible::<DataRange>::into_c(dr),})),horned_owl::model::ClassExpression::DataExactCardinality::<ArcStr>{
                    n,
dp,
dr
                } => ClassExpression(ClassExpression_Inner::DataExactCardinality(DataExactCardinality{n: IntoCompatible::<u32>::into_c(n),dp: IntoCompatible::<DataProperty>::into_c(dp),dr: IntoCompatible::<DataRange>::into_c(dr),})),
        }
    }
}

impl From<&ClassExpression> for horned_owl::model::ClassExpression<ArcStr> {
    fn from(value: &ClassExpression) -> Self {
        match value.0.borrow() {
                ClassExpression_Inner::Class(f0) => horned_owl::model::ClassExpression::<ArcStr>::Class(f0.into()),
                ClassExpression_Inner::ObjectIntersectionOf(ObjectIntersectionOf(first)) =>
                horned_owl::model::ClassExpression::<ArcStr>::ObjectIntersectionOf(
                    first.into_c(),
                ),
                ClassExpression_Inner::ObjectUnionOf(ObjectUnionOf(first)) =>
                horned_owl::model::ClassExpression::<ArcStr>::ObjectUnionOf(
                    first.into_c(),
                ),
                ClassExpression_Inner::ObjectComplementOf(ObjectComplementOf(first)) =>
                horned_owl::model::ClassExpression::<ArcStr>::ObjectComplementOf(
                    first.into_c(),
                ),
                ClassExpression_Inner::ObjectOneOf(ObjectOneOf(first)) =>
                horned_owl::model::ClassExpression::<ArcStr>::ObjectOneOf(
                    first.into_c(),
                ),
                ClassExpression_Inner::ObjectSomeValuesFrom(ObjectSomeValuesFrom{
                    ope, bce
                }) => horned_owl::model::ClassExpression::<ArcStr>::ObjectSomeValuesFrom{
                	ope: ope.into_c(),
                	bce: bce.into_c(),
                },
                ClassExpression_Inner::ObjectAllValuesFrom(ObjectAllValuesFrom{
                    ope, bce
                }) => horned_owl::model::ClassExpression::<ArcStr>::ObjectAllValuesFrom{
                	ope: ope.into_c(),
                	bce: bce.into_c(),
                },
                ClassExpression_Inner::ObjectHasValue(ObjectHasValue{
                    ope, i
                }) => horned_owl::model::ClassExpression::<ArcStr>::ObjectHasValue{
                	ope: ope.into_c(),
                	i: i.into_c(),
                },
                ClassExpression_Inner::ObjectHasSelf(ObjectHasSelf(first)) =>
                horned_owl::model::ClassExpression::<ArcStr>::ObjectHasSelf(
                    first.into_c(),
                ),
                ClassExpression_Inner::ObjectMinCardinality(ObjectMinCardinality{
                    n, ope, bce
                }) => horned_owl::model::ClassExpression::<ArcStr>::ObjectMinCardinality{
                	n: n.into_c(),
                	ope: ope.into_c(),
                	bce: bce.into_c(),
                },
                ClassExpression_Inner::ObjectMaxCardinality(ObjectMaxCardinality{
                    n, ope, bce
                }) => horned_owl::model::ClassExpression::<ArcStr>::ObjectMaxCardinality{
                	n: n.into_c(),
                	ope: ope.into_c(),
                	bce: bce.into_c(),
                },
                ClassExpression_Inner::ObjectExactCardinality(ObjectExactCardinality{
                    n, ope, bce
                }) => horned_owl::model::ClassExpression::<ArcStr>::ObjectExactCardinality{
                	n: n.into_c(),
                	ope: ope.into_c(),
                	bce: bce.into_c(),
                },
                ClassExpression_Inner::DataSomeValuesFrom(DataSomeValuesFrom{
                    dp, dr
                }) => horned_owl::model::ClassExpression::<ArcStr>::DataSomeValuesFrom{
                	dp: dp.into_c(),
                	dr: dr.into_c(),
                },
                ClassExpression_Inner::DataAllValuesFrom(DataAllValuesFrom{
                    dp, dr
                }) => horned_owl::model::ClassExpression::<ArcStr>::DataAllValuesFrom{
                	dp: dp.into_c(),
                	dr: dr.into_c(),
                },
                ClassExpression_Inner::DataHasValue(DataHasValue{
                    dp, l
                }) => horned_owl::model::ClassExpression::<ArcStr>::DataHasValue{
                	dp: dp.into_c(),
                	l: l.into_c(),
                },
                ClassExpression_Inner::DataMinCardinality(DataMinCardinality{
                    n, dp, dr
                }) => horned_owl::model::ClassExpression::<ArcStr>::DataMinCardinality{
                	n: n.into_c(),
                	dp: dp.into_c(),
                	dr: dr.into_c(),
                },
                ClassExpression_Inner::DataMaxCardinality(DataMaxCardinality{
                    n, dp, dr
                }) => horned_owl::model::ClassExpression::<ArcStr>::DataMaxCardinality{
                	n: n.into_c(),
                	dp: dp.into_c(),
                	dr: dr.into_c(),
                },
                ClassExpression_Inner::DataExactCardinality(DataExactCardinality{
                    n, dp, dr
                }) => horned_owl::model::ClassExpression::<ArcStr>::DataExactCardinality{
                	n: n.into_c(),
                	dp: dp.into_c(),
                	dr: dr.into_c(),
                },
        }
    }
}

impl<'py> IntoPyObject<'py> for ClassExpression {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            
                ClassExpression_Inner::Class(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectIntersectionOf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectUnionOf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectComplementOf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectOneOf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectSomeValuesFrom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectAllValuesFrom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectHasValue(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectHasSelf(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectMinCardinality(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectMaxCardinality(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::ObjectExactCardinality(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::DataSomeValuesFrom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::DataAllValuesFrom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::DataHasValue(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::DataMinCardinality(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::DataMaxCardinality(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                ClassExpression_Inner::DataExactCardinality(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
        }
    }

}

impl <'py> FromPyObject<'py> for ClassExpression {
    fn extract_bound(ob: &Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            {
            	let r = Class::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::Class(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectIntersectionOf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectIntersectionOf(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectUnionOf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectUnionOf(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectComplementOf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectComplementOf(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectOneOf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectOneOf(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectSomeValuesFrom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectSomeValuesFrom(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectAllValuesFrom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectAllValuesFrom(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectHasValue::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectHasValue(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectHasSelf::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectHasSelf(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectMinCardinality::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectMinCardinality(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectMaxCardinality::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectMaxCardinality(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = ObjectExactCardinality::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::ObjectExactCardinality(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = DataSomeValuesFrom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::DataSomeValuesFrom(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = DataAllValuesFrom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::DataAllValuesFrom(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = DataHasValue::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::DataHasValue(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = DataMinCardinality::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::DataMinCardinality(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = DataMaxCardinality::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::DataMaxCardinality(local);
                    return Ok(ClassExpression(inner));
                }
            }
            {
                let r = DataExactCardinality::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = ClassExpression_Inner::DataExactCardinality(local);
                    return Ok(ClassExpression(inner));
                }
            }

        Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object cannot be converted to ClassExpression"))
    }
}

impl ClassExpression {
    pub fn py_def() -> String {
        "typing.Union[m.Class,m.ObjectIntersectionOf,m.ObjectUnionOf,m.ObjectComplementOf,m.ObjectOneOf,m.ObjectSomeValuesFrom,m.ObjectAllValuesFrom,m.ObjectHasValue,m.ObjectHasSelf,m.ObjectMinCardinality,m.ObjectMaxCardinality,m.ObjectExactCardinality,m.DataSomeValuesFrom,m.DataAllValuesFrom,m.DataHasValue,m.DataMinCardinality,m.DataMaxCardinality,m.DataExactCardinality,]".into()
    }
}



/**************** Base implementations for ClassExpression ****************/
impl FromCompatible<horned_owl::model::ClassExpression<ArcStr>> for ClassExpression {
    fn from_c(value: horned_owl::model::ClassExpression<ArcStr>) -> Self {
        ClassExpression::from(value)
    }
}

impl FromCompatible<&horned_owl::model::ClassExpression<ArcStr>> for ClassExpression {
    fn from_c(value: &horned_owl::model::ClassExpression<ArcStr>) -> Self {
        ClassExpression::from(value)
    }
}

impl FromCompatible<ClassExpression> for horned_owl::model::ClassExpression<ArcStr> {
    fn from_c(value: ClassExpression) -> Self {
        horned_owl::model::ClassExpression::<ArcStr>::from(value)
    }
}

impl FromCompatible<&ClassExpression> for horned_owl::model::ClassExpression<ArcStr> {
    fn from_c(value: &ClassExpression) -> Self {
        horned_owl::model::ClassExpression::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::ClassExpression<ArcStr>> for ClassExpression {
    fn from(value: horned_owl::model::ClassExpression<ArcStr>) -> Self {
        ClassExpression::from(value.borrow())
    }
}

impl From<ClassExpression> for horned_owl::model::ClassExpression<ArcStr> {
    fn from(value: ClassExpression) -> Self {
        horned_owl::model::ClassExpression::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<ClassExpression>> for Box<horned_owl::model::ClassExpression<ArcStr>> {
    fn from(value: &BoxWrap<ClassExpression>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::ClassExpression<ArcStr>>> for BoxWrap<ClassExpression> {
    fn from(value: &Box<horned_owl::model::ClassExpression<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<ClassExpression>::into(*value.clone())))
    }
}

impl From<BoxWrap<ClassExpression>> for Box<horned_owl::model::ClassExpression<ArcStr>> {
    fn from(value: BoxWrap<ClassExpression>) -> Self {
        Into::<Box<horned_owl::model::ClassExpression<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::ClassExpression<ArcStr>>> for BoxWrap<ClassExpression> {
    fn from(value: Box<horned_owl::model::ClassExpression<ArcStr>>) -> Self {
        Into::<BoxWrap<ClassExpression>>::into(value.borrow())
    }
}

impl From<VecWrap<ClassExpression>> for Vec<horned_owl::model::ClassExpression<ArcStr>> {
    fn from(value: VecWrap<ClassExpression>) -> Self {
        Into::<Vec<horned_owl::model::ClassExpression<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::ClassExpression<ArcStr>>> for VecWrap<ClassExpression> {
    fn from(value: Vec<horned_owl::model::ClassExpression<ArcStr>>) -> Self {
        Into::<VecWrap<ClassExpression>>::into(value.borrow())
    }
}

impl From<&VecWrap<ClassExpression>> for Vec<horned_owl::model::ClassExpression<ArcStr>> {
    fn from(value: &VecWrap<ClassExpression>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::ClassExpression::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::ClassExpression<ArcStr>>> for VecWrap<ClassExpression> {
    fn from(value: &Vec<horned_owl::model::ClassExpression<ArcStr>>) -> Self {
        VecWrap(value.iter().map(ClassExpression::from).collect())
    }
}

impl FromCompatible<&BoxWrap<ClassExpression>> for Box<horned_owl::model::ClassExpression<ArcStr>> {
    fn from_c(value: &BoxWrap<ClassExpression>) -> Self {
        Box::<horned_owl::model::ClassExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::ClassExpression<ArcStr>>> for BoxWrap<ClassExpression> {
    fn from_c(value: &Box<horned_owl::model::ClassExpression<ArcStr>>) -> Self {
        BoxWrap::<ClassExpression>::from(value)
    }
}
impl FromCompatible<BoxWrap<ClassExpression>> for Box<horned_owl::model::ClassExpression<ArcStr>> {
    fn from_c(value: BoxWrap<ClassExpression>) -> Self {
        Box::<horned_owl::model::ClassExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::ClassExpression<ArcStr>>> for BoxWrap<ClassExpression> {
    fn from_c(value: Box<horned_owl::model::ClassExpression<ArcStr>>) -> Self {
        BoxWrap::<ClassExpression>::from(value)
    }
}
impl FromCompatible<VecWrap<ClassExpression>> for Vec<horned_owl::model::ClassExpression<ArcStr>> {
    fn from_c(value: VecWrap<ClassExpression>) -> Self {
        Vec::<horned_owl::model::ClassExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::ClassExpression<ArcStr>>> for VecWrap<ClassExpression> {
    fn from_c(value: Vec<horned_owl::model::ClassExpression<ArcStr>>) -> Self {
        VecWrap::<ClassExpression>::from(value)
    }
}
impl FromCompatible<&VecWrap<ClassExpression>> for Vec<horned_owl::model::ClassExpression<ArcStr>> {
    fn from_c(value: &VecWrap<ClassExpression>) -> Self {
        Vec::<horned_owl::model::ClassExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::ClassExpression<ArcStr>>> for VecWrap<ClassExpression> {
    fn from_c(value: &Vec<horned_owl::model::ClassExpression<ArcStr>>) -> Self {
        VecWrap::<ClassExpression>::from(value)
    }
}
#[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PropertyExpression {
    
        #[pyo3(transparent)]
        ObjectPropertyExpression (ObjectPropertyExpression),
    
        #[pyo3(transparent)]
        DataProperty (DataProperty),
    
        #[pyo3(transparent)]
        AnnotationProperty (AnnotationProperty),
    
}

impl<'py> IntoPyObject<'py> for PropertyExpression {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
        
            PropertyExpression::ObjectPropertyExpression(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            PropertyExpression::DataProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            PropertyExpression::AnnotationProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
        }
    }
}

impl From<&PropertyExpression> for horned_owl::model::PropertyExpression<ArcStr> {
    fn from(value: &PropertyExpression) -> Self {
        match value {
        
            PropertyExpression::ObjectPropertyExpression(inner) => horned_owl::model::PropertyExpression::ObjectPropertyExpression(inner.into()),
        
            PropertyExpression::DataProperty(inner) => horned_owl::model::PropertyExpression::DataProperty(inner.into()),
        
            PropertyExpression::AnnotationProperty(inner) => horned_owl::model::PropertyExpression::AnnotationProperty(inner.into()),
        
        }
    }
}

impl From<&horned_owl::model::PropertyExpression<ArcStr>> for PropertyExpression {

    fn from(value: &horned_owl::model::PropertyExpression<ArcStr>) -> Self {
        match value {
        
            horned_owl::model::PropertyExpression::ObjectPropertyExpression(inner) => PropertyExpression::ObjectPropertyExpression(inner.into()),
        
            horned_owl::model::PropertyExpression::DataProperty(inner) => PropertyExpression::DataProperty(inner.into()),
        
            horned_owl::model::PropertyExpression::AnnotationProperty(inner) => PropertyExpression::AnnotationProperty(inner.into()),
        
        }
    }
}


impl PropertyExpression {
    pub fn py_def() -> String {
        "typing.Union[m.ObjectPropertyExpression,m.DataProperty,m.AnnotationProperty,]".into()
    }
}



/**************** Base implementations for PropertyExpression ****************/
impl FromCompatible<horned_owl::model::PropertyExpression<ArcStr>> for PropertyExpression {
    fn from_c(value: horned_owl::model::PropertyExpression<ArcStr>) -> Self {
        PropertyExpression::from(value)
    }
}

impl FromCompatible<&horned_owl::model::PropertyExpression<ArcStr>> for PropertyExpression {
    fn from_c(value: &horned_owl::model::PropertyExpression<ArcStr>) -> Self {
        PropertyExpression::from(value)
    }
}

impl FromCompatible<PropertyExpression> for horned_owl::model::PropertyExpression<ArcStr> {
    fn from_c(value: PropertyExpression) -> Self {
        horned_owl::model::PropertyExpression::<ArcStr>::from(value)
    }
}

impl FromCompatible<&PropertyExpression> for horned_owl::model::PropertyExpression<ArcStr> {
    fn from_c(value: &PropertyExpression) -> Self {
        horned_owl::model::PropertyExpression::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::PropertyExpression<ArcStr>> for PropertyExpression {
    fn from(value: horned_owl::model::PropertyExpression<ArcStr>) -> Self {
        PropertyExpression::from(value.borrow())
    }
}

impl From<PropertyExpression> for horned_owl::model::PropertyExpression<ArcStr> {
    fn from(value: PropertyExpression) -> Self {
        horned_owl::model::PropertyExpression::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<PropertyExpression>> for Box<horned_owl::model::PropertyExpression<ArcStr>> {
    fn from(value: &BoxWrap<PropertyExpression>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::PropertyExpression<ArcStr>>> for BoxWrap<PropertyExpression> {
    fn from(value: &Box<horned_owl::model::PropertyExpression<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<PropertyExpression>::into(*value.clone())))
    }
}

impl From<BoxWrap<PropertyExpression>> for Box<horned_owl::model::PropertyExpression<ArcStr>> {
    fn from(value: BoxWrap<PropertyExpression>) -> Self {
        Into::<Box<horned_owl::model::PropertyExpression<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::PropertyExpression<ArcStr>>> for BoxWrap<PropertyExpression> {
    fn from(value: Box<horned_owl::model::PropertyExpression<ArcStr>>) -> Self {
        Into::<BoxWrap<PropertyExpression>>::into(value.borrow())
    }
}

impl From<VecWrap<PropertyExpression>> for Vec<horned_owl::model::PropertyExpression<ArcStr>> {
    fn from(value: VecWrap<PropertyExpression>) -> Self {
        Into::<Vec<horned_owl::model::PropertyExpression<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::PropertyExpression<ArcStr>>> for VecWrap<PropertyExpression> {
    fn from(value: Vec<horned_owl::model::PropertyExpression<ArcStr>>) -> Self {
        Into::<VecWrap<PropertyExpression>>::into(value.borrow())
    }
}

impl From<&VecWrap<PropertyExpression>> for Vec<horned_owl::model::PropertyExpression<ArcStr>> {
    fn from(value: &VecWrap<PropertyExpression>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::PropertyExpression::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::PropertyExpression<ArcStr>>> for VecWrap<PropertyExpression> {
    fn from(value: &Vec<horned_owl::model::PropertyExpression<ArcStr>>) -> Self {
        VecWrap(value.iter().map(PropertyExpression::from).collect())
    }
}

impl FromCompatible<&BoxWrap<PropertyExpression>> for Box<horned_owl::model::PropertyExpression<ArcStr>> {
    fn from_c(value: &BoxWrap<PropertyExpression>) -> Self {
        Box::<horned_owl::model::PropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::PropertyExpression<ArcStr>>> for BoxWrap<PropertyExpression> {
    fn from_c(value: &Box<horned_owl::model::PropertyExpression<ArcStr>>) -> Self {
        BoxWrap::<PropertyExpression>::from(value)
    }
}
impl FromCompatible<BoxWrap<PropertyExpression>> for Box<horned_owl::model::PropertyExpression<ArcStr>> {
    fn from_c(value: BoxWrap<PropertyExpression>) -> Self {
        Box::<horned_owl::model::PropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::PropertyExpression<ArcStr>>> for BoxWrap<PropertyExpression> {
    fn from_c(value: Box<horned_owl::model::PropertyExpression<ArcStr>>) -> Self {
        BoxWrap::<PropertyExpression>::from(value)
    }
}
impl FromCompatible<VecWrap<PropertyExpression>> for Vec<horned_owl::model::PropertyExpression<ArcStr>> {
    fn from_c(value: VecWrap<PropertyExpression>) -> Self {
        Vec::<horned_owl::model::PropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::PropertyExpression<ArcStr>>> for VecWrap<PropertyExpression> {
    fn from_c(value: Vec<horned_owl::model::PropertyExpression<ArcStr>>) -> Self {
        VecWrap::<PropertyExpression>::from(value)
    }
}
impl FromCompatible<&VecWrap<PropertyExpression>> for Vec<horned_owl::model::PropertyExpression<ArcStr>> {
    fn from_c(value: &VecWrap<PropertyExpression>) -> Self {
        Vec::<horned_owl::model::PropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::PropertyExpression<ArcStr>>> for VecWrap<PropertyExpression> {
    fn from_c(value: &Vec<horned_owl::model::PropertyExpression<ArcStr>>) -> Self {
        VecWrap::<PropertyExpression>::from(value)
    }
}
#[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnnotationSubject {
    
        #[pyo3(transparent)]
        IRI (IRI),
    
        #[pyo3(transparent)]
        AnonymousIndividual (AnonymousIndividual),
    
}

impl<'py> IntoPyObject<'py> for AnnotationSubject {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
        
            AnnotationSubject::IRI(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            AnnotationSubject::AnonymousIndividual(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
        }
    }
}

impl From<&AnnotationSubject> for horned_owl::model::AnnotationSubject<ArcStr> {
    fn from(value: &AnnotationSubject) -> Self {
        match value {
        
            AnnotationSubject::IRI(inner) => horned_owl::model::AnnotationSubject::IRI(inner.into()),
        
            AnnotationSubject::AnonymousIndividual(inner) => horned_owl::model::AnnotationSubject::AnonymousIndividual(inner.into()),
        
        }
    }
}

impl From<&horned_owl::model::AnnotationSubject<ArcStr>> for AnnotationSubject {

    fn from(value: &horned_owl::model::AnnotationSubject<ArcStr>) -> Self {
        match value {
        
            horned_owl::model::AnnotationSubject::IRI(inner) => AnnotationSubject::IRI(inner.into()),
        
            horned_owl::model::AnnotationSubject::AnonymousIndividual(inner) => AnnotationSubject::AnonymousIndividual(inner.into()),
        
        }
    }
}


impl AnnotationSubject {
    pub fn py_def() -> String {
        "typing.Union[m.IRI,m.AnonymousIndividual,]".into()
    }
}



/**************** Base implementations for AnnotationSubject ****************/
impl FromCompatible<horned_owl::model::AnnotationSubject<ArcStr>> for AnnotationSubject {
    fn from_c(value: horned_owl::model::AnnotationSubject<ArcStr>) -> Self {
        AnnotationSubject::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AnnotationSubject<ArcStr>> for AnnotationSubject {
    fn from_c(value: &horned_owl::model::AnnotationSubject<ArcStr>) -> Self {
        AnnotationSubject::from(value)
    }
}

impl FromCompatible<AnnotationSubject> for horned_owl::model::AnnotationSubject<ArcStr> {
    fn from_c(value: AnnotationSubject) -> Self {
        horned_owl::model::AnnotationSubject::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AnnotationSubject> for horned_owl::model::AnnotationSubject<ArcStr> {
    fn from_c(value: &AnnotationSubject) -> Self {
        horned_owl::model::AnnotationSubject::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AnnotationSubject<ArcStr>> for AnnotationSubject {
    fn from(value: horned_owl::model::AnnotationSubject<ArcStr>) -> Self {
        AnnotationSubject::from(value.borrow())
    }
}

impl From<AnnotationSubject> for horned_owl::model::AnnotationSubject<ArcStr> {
    fn from(value: AnnotationSubject) -> Self {
        horned_owl::model::AnnotationSubject::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AnnotationSubject>> for Box<horned_owl::model::AnnotationSubject<ArcStr>> {
    fn from(value: &BoxWrap<AnnotationSubject>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AnnotationSubject<ArcStr>>> for BoxWrap<AnnotationSubject> {
    fn from(value: &Box<horned_owl::model::AnnotationSubject<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AnnotationSubject>::into(*value.clone())))
    }
}

impl From<BoxWrap<AnnotationSubject>> for Box<horned_owl::model::AnnotationSubject<ArcStr>> {
    fn from(value: BoxWrap<AnnotationSubject>) -> Self {
        Into::<Box<horned_owl::model::AnnotationSubject<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AnnotationSubject<ArcStr>>> for BoxWrap<AnnotationSubject> {
    fn from(value: Box<horned_owl::model::AnnotationSubject<ArcStr>>) -> Self {
        Into::<BoxWrap<AnnotationSubject>>::into(value.borrow())
    }
}

impl From<VecWrap<AnnotationSubject>> for Vec<horned_owl::model::AnnotationSubject<ArcStr>> {
    fn from(value: VecWrap<AnnotationSubject>) -> Self {
        Into::<Vec<horned_owl::model::AnnotationSubject<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AnnotationSubject<ArcStr>>> for VecWrap<AnnotationSubject> {
    fn from(value: Vec<horned_owl::model::AnnotationSubject<ArcStr>>) -> Self {
        Into::<VecWrap<AnnotationSubject>>::into(value.borrow())
    }
}

impl From<&VecWrap<AnnotationSubject>> for Vec<horned_owl::model::AnnotationSubject<ArcStr>> {
    fn from(value: &VecWrap<AnnotationSubject>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AnnotationSubject::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AnnotationSubject<ArcStr>>> for VecWrap<AnnotationSubject> {
    fn from(value: &Vec<horned_owl::model::AnnotationSubject<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AnnotationSubject::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AnnotationSubject>> for Box<horned_owl::model::AnnotationSubject<ArcStr>> {
    fn from_c(value: &BoxWrap<AnnotationSubject>) -> Self {
        Box::<horned_owl::model::AnnotationSubject<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AnnotationSubject<ArcStr>>> for BoxWrap<AnnotationSubject> {
    fn from_c(value: &Box<horned_owl::model::AnnotationSubject<ArcStr>>) -> Self {
        BoxWrap::<AnnotationSubject>::from(value)
    }
}
impl FromCompatible<BoxWrap<AnnotationSubject>> for Box<horned_owl::model::AnnotationSubject<ArcStr>> {
    fn from_c(value: BoxWrap<AnnotationSubject>) -> Self {
        Box::<horned_owl::model::AnnotationSubject<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AnnotationSubject<ArcStr>>> for BoxWrap<AnnotationSubject> {
    fn from_c(value: Box<horned_owl::model::AnnotationSubject<ArcStr>>) -> Self {
        BoxWrap::<AnnotationSubject>::from(value)
    }
}
impl FromCompatible<VecWrap<AnnotationSubject>> for Vec<horned_owl::model::AnnotationSubject<ArcStr>> {
    fn from_c(value: VecWrap<AnnotationSubject>) -> Self {
        Vec::<horned_owl::model::AnnotationSubject<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AnnotationSubject<ArcStr>>> for VecWrap<AnnotationSubject> {
    fn from_c(value: Vec<horned_owl::model::AnnotationSubject<ArcStr>>) -> Self {
        VecWrap::<AnnotationSubject>::from(value)
    }
}
impl FromCompatible<&VecWrap<AnnotationSubject>> for Vec<horned_owl::model::AnnotationSubject<ArcStr>> {
    fn from_c(value: &VecWrap<AnnotationSubject>) -> Self {
        Vec::<horned_owl::model::AnnotationSubject<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AnnotationSubject<ArcStr>>> for VecWrap<AnnotationSubject> {
    fn from_c(value: &Vec<horned_owl::model::AnnotationSubject<ArcStr>>) -> Self {
        VecWrap::<AnnotationSubject>::from(value)
    }
}
#[doc = concat!(
    "AnnotationProperty(first: IRI,)",
    "\n\n",
    doc!(AnnotationProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnotationProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl AnnotationProperty {
    #[new]
    fn new(first: IRI,) -> Self {
        AnnotationProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::AnnotationProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::AnnotationProperty<ArcStr>> for AnnotationProperty {
    fn from(value: &horned_owl::model::AnnotationProperty<ArcStr>) -> Self {

        AnnotationProperty (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&AnnotationProperty> for horned_owl::model::AnnotationProperty<ArcStr> {
    fn from(value: &AnnotationProperty) -> Self {
        horned_owl::model::AnnotationProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for AnnotationProperty ****************/
impl FromCompatible<horned_owl::model::AnnotationProperty<ArcStr>> for AnnotationProperty {
    fn from_c(value: horned_owl::model::AnnotationProperty<ArcStr>) -> Self {
        AnnotationProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AnnotationProperty<ArcStr>> for AnnotationProperty {
    fn from_c(value: &horned_owl::model::AnnotationProperty<ArcStr>) -> Self {
        AnnotationProperty::from(value)
    }
}

impl FromCompatible<AnnotationProperty> for horned_owl::model::AnnotationProperty<ArcStr> {
    fn from_c(value: AnnotationProperty) -> Self {
        horned_owl::model::AnnotationProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AnnotationProperty> for horned_owl::model::AnnotationProperty<ArcStr> {
    fn from_c(value: &AnnotationProperty) -> Self {
        horned_owl::model::AnnotationProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AnnotationProperty<ArcStr>> for AnnotationProperty {
    fn from(value: horned_owl::model::AnnotationProperty<ArcStr>) -> Self {
        AnnotationProperty::from(value.borrow())
    }
}

impl From<AnnotationProperty> for horned_owl::model::AnnotationProperty<ArcStr> {
    fn from(value: AnnotationProperty) -> Self {
        horned_owl::model::AnnotationProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AnnotationProperty>> for Box<horned_owl::model::AnnotationProperty<ArcStr>> {
    fn from(value: &BoxWrap<AnnotationProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AnnotationProperty<ArcStr>>> for BoxWrap<AnnotationProperty> {
    fn from(value: &Box<horned_owl::model::AnnotationProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AnnotationProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<AnnotationProperty>> for Box<horned_owl::model::AnnotationProperty<ArcStr>> {
    fn from(value: BoxWrap<AnnotationProperty>) -> Self {
        Into::<Box<horned_owl::model::AnnotationProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AnnotationProperty<ArcStr>>> for BoxWrap<AnnotationProperty> {
    fn from(value: Box<horned_owl::model::AnnotationProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<AnnotationProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<AnnotationProperty>> for Vec<horned_owl::model::AnnotationProperty<ArcStr>> {
    fn from(value: VecWrap<AnnotationProperty>) -> Self {
        Into::<Vec<horned_owl::model::AnnotationProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AnnotationProperty<ArcStr>>> for VecWrap<AnnotationProperty> {
    fn from(value: Vec<horned_owl::model::AnnotationProperty<ArcStr>>) -> Self {
        Into::<VecWrap<AnnotationProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<AnnotationProperty>> for Vec<horned_owl::model::AnnotationProperty<ArcStr>> {
    fn from(value: &VecWrap<AnnotationProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AnnotationProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AnnotationProperty<ArcStr>>> for VecWrap<AnnotationProperty> {
    fn from(value: &Vec<horned_owl::model::AnnotationProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AnnotationProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AnnotationProperty>> for Box<horned_owl::model::AnnotationProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<AnnotationProperty>) -> Self {
        Box::<horned_owl::model::AnnotationProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AnnotationProperty<ArcStr>>> for BoxWrap<AnnotationProperty> {
    fn from_c(value: &Box<horned_owl::model::AnnotationProperty<ArcStr>>) -> Self {
        BoxWrap::<AnnotationProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<AnnotationProperty>> for Box<horned_owl::model::AnnotationProperty<ArcStr>> {
    fn from_c(value: BoxWrap<AnnotationProperty>) -> Self {
        Box::<horned_owl::model::AnnotationProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AnnotationProperty<ArcStr>>> for BoxWrap<AnnotationProperty> {
    fn from_c(value: Box<horned_owl::model::AnnotationProperty<ArcStr>>) -> Self {
        BoxWrap::<AnnotationProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<AnnotationProperty>> for Vec<horned_owl::model::AnnotationProperty<ArcStr>> {
    fn from_c(value: VecWrap<AnnotationProperty>) -> Self {
        Vec::<horned_owl::model::AnnotationProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AnnotationProperty<ArcStr>>> for VecWrap<AnnotationProperty> {
    fn from_c(value: Vec<horned_owl::model::AnnotationProperty<ArcStr>>) -> Self {
        VecWrap::<AnnotationProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<AnnotationProperty>> for Vec<horned_owl::model::AnnotationProperty<ArcStr>> {
    fn from_c(value: &VecWrap<AnnotationProperty>) -> Self {
        Vec::<horned_owl::model::AnnotationProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AnnotationProperty<ArcStr>>> for VecWrap<AnnotationProperty> {
    fn from_c(value: &Vec<horned_owl::model::AnnotationProperty<ArcStr>>) -> Self {
        VecWrap::<AnnotationProperty>::from(value)
    }
}
#[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnnotationValue {
    
        #[pyo3(transparent)]
        Literal (Literal),
    
        #[pyo3(transparent)]
        IRI (IRI),
    
        #[pyo3(transparent)]
        AnonymousIndividual (AnonymousIndividual),
    
}

impl<'py> IntoPyObject<'py> for AnnotationValue {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
        
            AnnotationValue::Literal(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            AnnotationValue::IRI(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            AnnotationValue::AnonymousIndividual(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
        }
    }
}

impl From<&AnnotationValue> for horned_owl::model::AnnotationValue<ArcStr> {
    fn from(value: &AnnotationValue) -> Self {
        match value {
        
            AnnotationValue::Literal(inner) => horned_owl::model::AnnotationValue::Literal(inner.into()),
        
            AnnotationValue::IRI(inner) => horned_owl::model::AnnotationValue::IRI(inner.into()),
        
            AnnotationValue::AnonymousIndividual(inner) => horned_owl::model::AnnotationValue::AnonymousIndividual(inner.into()),
        
        }
    }
}

impl From<&horned_owl::model::AnnotationValue<ArcStr>> for AnnotationValue {

    fn from(value: &horned_owl::model::AnnotationValue<ArcStr>) -> Self {
        match value {
        
            horned_owl::model::AnnotationValue::Literal(inner) => AnnotationValue::Literal(inner.into()),
        
            horned_owl::model::AnnotationValue::IRI(inner) => AnnotationValue::IRI(inner.into()),
        
            horned_owl::model::AnnotationValue::AnonymousIndividual(inner) => AnnotationValue::AnonymousIndividual(inner.into()),
        
        }
    }
}


impl AnnotationValue {
    pub fn py_def() -> String {
        "typing.Union[m.Literal,m.IRI,m.AnonymousIndividual,]".into()
    }
}



/**************** Base implementations for AnnotationValue ****************/
impl FromCompatible<horned_owl::model::AnnotationValue<ArcStr>> for AnnotationValue {
    fn from_c(value: horned_owl::model::AnnotationValue<ArcStr>) -> Self {
        AnnotationValue::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AnnotationValue<ArcStr>> for AnnotationValue {
    fn from_c(value: &horned_owl::model::AnnotationValue<ArcStr>) -> Self {
        AnnotationValue::from(value)
    }
}

impl FromCompatible<AnnotationValue> for horned_owl::model::AnnotationValue<ArcStr> {
    fn from_c(value: AnnotationValue) -> Self {
        horned_owl::model::AnnotationValue::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AnnotationValue> for horned_owl::model::AnnotationValue<ArcStr> {
    fn from_c(value: &AnnotationValue) -> Self {
        horned_owl::model::AnnotationValue::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AnnotationValue<ArcStr>> for AnnotationValue {
    fn from(value: horned_owl::model::AnnotationValue<ArcStr>) -> Self {
        AnnotationValue::from(value.borrow())
    }
}

impl From<AnnotationValue> for horned_owl::model::AnnotationValue<ArcStr> {
    fn from(value: AnnotationValue) -> Self {
        horned_owl::model::AnnotationValue::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AnnotationValue>> for Box<horned_owl::model::AnnotationValue<ArcStr>> {
    fn from(value: &BoxWrap<AnnotationValue>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AnnotationValue<ArcStr>>> for BoxWrap<AnnotationValue> {
    fn from(value: &Box<horned_owl::model::AnnotationValue<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AnnotationValue>::into(*value.clone())))
    }
}

impl From<BoxWrap<AnnotationValue>> for Box<horned_owl::model::AnnotationValue<ArcStr>> {
    fn from(value: BoxWrap<AnnotationValue>) -> Self {
        Into::<Box<horned_owl::model::AnnotationValue<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AnnotationValue<ArcStr>>> for BoxWrap<AnnotationValue> {
    fn from(value: Box<horned_owl::model::AnnotationValue<ArcStr>>) -> Self {
        Into::<BoxWrap<AnnotationValue>>::into(value.borrow())
    }
}

impl From<VecWrap<AnnotationValue>> for Vec<horned_owl::model::AnnotationValue<ArcStr>> {
    fn from(value: VecWrap<AnnotationValue>) -> Self {
        Into::<Vec<horned_owl::model::AnnotationValue<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AnnotationValue<ArcStr>>> for VecWrap<AnnotationValue> {
    fn from(value: Vec<horned_owl::model::AnnotationValue<ArcStr>>) -> Self {
        Into::<VecWrap<AnnotationValue>>::into(value.borrow())
    }
}

impl From<&VecWrap<AnnotationValue>> for Vec<horned_owl::model::AnnotationValue<ArcStr>> {
    fn from(value: &VecWrap<AnnotationValue>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AnnotationValue::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AnnotationValue<ArcStr>>> for VecWrap<AnnotationValue> {
    fn from(value: &Vec<horned_owl::model::AnnotationValue<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AnnotationValue::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AnnotationValue>> for Box<horned_owl::model::AnnotationValue<ArcStr>> {
    fn from_c(value: &BoxWrap<AnnotationValue>) -> Self {
        Box::<horned_owl::model::AnnotationValue<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AnnotationValue<ArcStr>>> for BoxWrap<AnnotationValue> {
    fn from_c(value: &Box<horned_owl::model::AnnotationValue<ArcStr>>) -> Self {
        BoxWrap::<AnnotationValue>::from(value)
    }
}
impl FromCompatible<BoxWrap<AnnotationValue>> for Box<horned_owl::model::AnnotationValue<ArcStr>> {
    fn from_c(value: BoxWrap<AnnotationValue>) -> Self {
        Box::<horned_owl::model::AnnotationValue<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AnnotationValue<ArcStr>>> for BoxWrap<AnnotationValue> {
    fn from_c(value: Box<horned_owl::model::AnnotationValue<ArcStr>>) -> Self {
        BoxWrap::<AnnotationValue>::from(value)
    }
}
impl FromCompatible<VecWrap<AnnotationValue>> for Vec<horned_owl::model::AnnotationValue<ArcStr>> {
    fn from_c(value: VecWrap<AnnotationValue>) -> Self {
        Vec::<horned_owl::model::AnnotationValue<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AnnotationValue<ArcStr>>> for VecWrap<AnnotationValue> {
    fn from_c(value: Vec<horned_owl::model::AnnotationValue<ArcStr>>) -> Self {
        VecWrap::<AnnotationValue>::from(value)
    }
}
impl FromCompatible<&VecWrap<AnnotationValue>> for Vec<horned_owl::model::AnnotationValue<ArcStr>> {
    fn from_c(value: &VecWrap<AnnotationValue>) -> Self {
        Vec::<horned_owl::model::AnnotationValue<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AnnotationValue<ArcStr>>> for VecWrap<AnnotationValue> {
    fn from_c(value: &Vec<horned_owl::model::AnnotationValue<ArcStr>>) -> Self {
        VecWrap::<AnnotationValue>::from(value)
    }
}
#[doc = concat!("Annotation(ap: AnnotationProperty,av: AnnotationValue,)",
    "\n\n",
    doc!(Annotation)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Annotation {
        #[doc="ap: AnnotationProperty"]
        #[pyo3(get,set)]
        pub ap: AnnotationProperty,
    
        #[doc="av: AnnotationValue"]
        #[pyo3(get,set)]
        pub av: AnnotationValue,
    }

#[pymethods]
impl Annotation {
    #[new]
    fn new(ap: AnnotationProperty,av: AnnotationValue,) -> Self {
        Annotation {
                ap,
                av,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ap" => self.ap.clone().into_pyobject(py).map(Bound::into_any),
            "av" => self.av.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ap" => {
                self.ap = value.extract()?;
                Ok(())
            },
            "av" => {
                self.av = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::Annotation<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::Annotation<ArcStr>> for Annotation {
    fn from(value: &horned_owl::model::Annotation<ArcStr>) -> Self {
        Annotation {
            ap: IntoCompatible::<AnnotationProperty>::into_c(value.ap.borrow()),
            av: IntoCompatible::<AnnotationValue>::into_c(value.av.borrow()),
        }
    }
}


impl From<&Annotation> for horned_owl::model::Annotation<ArcStr> {
    fn from(value: &Annotation) -> Self {
        horned_owl::model::Annotation::<ArcStr> {
            ap: value.ap.borrow().into_c(),
            av: value.av.borrow().into_c(),
        }
    }
}



/**************** Base implementations for Annotation ****************/
impl FromCompatible<horned_owl::model::Annotation<ArcStr>> for Annotation {
    fn from_c(value: horned_owl::model::Annotation<ArcStr>) -> Self {
        Annotation::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Annotation<ArcStr>> for Annotation {
    fn from_c(value: &horned_owl::model::Annotation<ArcStr>) -> Self {
        Annotation::from(value)
    }
}

impl FromCompatible<Annotation> for horned_owl::model::Annotation<ArcStr> {
    fn from_c(value: Annotation) -> Self {
        horned_owl::model::Annotation::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Annotation> for horned_owl::model::Annotation<ArcStr> {
    fn from_c(value: &Annotation) -> Self {
        horned_owl::model::Annotation::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Annotation<ArcStr>> for Annotation {
    fn from(value: horned_owl::model::Annotation<ArcStr>) -> Self {
        Annotation::from(value.borrow())
    }
}

impl From<Annotation> for horned_owl::model::Annotation<ArcStr> {
    fn from(value: Annotation) -> Self {
        horned_owl::model::Annotation::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Annotation>> for Box<horned_owl::model::Annotation<ArcStr>> {
    fn from(value: &BoxWrap<Annotation>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Annotation<ArcStr>>> for BoxWrap<Annotation> {
    fn from(value: &Box<horned_owl::model::Annotation<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Annotation>::into(*value.clone())))
    }
}

impl From<BoxWrap<Annotation>> for Box<horned_owl::model::Annotation<ArcStr>> {
    fn from(value: BoxWrap<Annotation>) -> Self {
        Into::<Box<horned_owl::model::Annotation<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Annotation<ArcStr>>> for BoxWrap<Annotation> {
    fn from(value: Box<horned_owl::model::Annotation<ArcStr>>) -> Self {
        Into::<BoxWrap<Annotation>>::into(value.borrow())
    }
}

impl From<VecWrap<Annotation>> for Vec<horned_owl::model::Annotation<ArcStr>> {
    fn from(value: VecWrap<Annotation>) -> Self {
        Into::<Vec<horned_owl::model::Annotation<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Annotation<ArcStr>>> for VecWrap<Annotation> {
    fn from(value: Vec<horned_owl::model::Annotation<ArcStr>>) -> Self {
        Into::<VecWrap<Annotation>>::into(value.borrow())
    }
}

impl From<&VecWrap<Annotation>> for Vec<horned_owl::model::Annotation<ArcStr>> {
    fn from(value: &VecWrap<Annotation>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Annotation::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Annotation<ArcStr>>> for VecWrap<Annotation> {
    fn from(value: &Vec<horned_owl::model::Annotation<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Annotation::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Annotation>> for Box<horned_owl::model::Annotation<ArcStr>> {
    fn from_c(value: &BoxWrap<Annotation>) -> Self {
        Box::<horned_owl::model::Annotation<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Annotation<ArcStr>>> for BoxWrap<Annotation> {
    fn from_c(value: &Box<horned_owl::model::Annotation<ArcStr>>) -> Self {
        BoxWrap::<Annotation>::from(value)
    }
}
impl FromCompatible<BoxWrap<Annotation>> for Box<horned_owl::model::Annotation<ArcStr>> {
    fn from_c(value: BoxWrap<Annotation>) -> Self {
        Box::<horned_owl::model::Annotation<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Annotation<ArcStr>>> for BoxWrap<Annotation> {
    fn from_c(value: Box<horned_owl::model::Annotation<ArcStr>>) -> Self {
        BoxWrap::<Annotation>::from(value)
    }
}
impl FromCompatible<VecWrap<Annotation>> for Vec<horned_owl::model::Annotation<ArcStr>> {
    fn from_c(value: VecWrap<Annotation>) -> Self {
        Vec::<horned_owl::model::Annotation<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Annotation<ArcStr>>> for VecWrap<Annotation> {
    fn from_c(value: Vec<horned_owl::model::Annotation<ArcStr>>) -> Self {
        VecWrap::<Annotation>::from(value)
    }
}
impl FromCompatible<&VecWrap<Annotation>> for Vec<horned_owl::model::Annotation<ArcStr>> {
    fn from_c(value: &VecWrap<Annotation>) -> Self {
        Vec::<horned_owl::model::Annotation<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Annotation<ArcStr>>> for VecWrap<Annotation> {
    fn from_c(value: &Vec<horned_owl::model::Annotation<ArcStr>>) -> Self {
        VecWrap::<Annotation>::from(value)
    }
}
#[doc = concat!(
    "OntologyAnnotation(first: Annotation,)",
    "\n\n",
    doc!(OntologyAnnotation)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OntologyAnnotation (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub Annotation,
);

#[pymethods]
impl OntologyAnnotation {
    #[new]
    fn new(first: Annotation,) -> Self {
        OntologyAnnotation (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::OntologyAnnotation<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::OntologyAnnotation<ArcStr>> for OntologyAnnotation {
    fn from(value: &horned_owl::model::OntologyAnnotation<ArcStr>) -> Self {

        OntologyAnnotation (
    IntoCompatible::<Annotation>::into_c(&value.0),
        )
    }
}

impl From<&OntologyAnnotation> for horned_owl::model::OntologyAnnotation<ArcStr> {
    fn from(value: &OntologyAnnotation) -> Self {
        horned_owl::model::OntologyAnnotation::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for OntologyAnnotation ****************/
impl FromCompatible<horned_owl::model::OntologyAnnotation<ArcStr>> for OntologyAnnotation {
    fn from_c(value: horned_owl::model::OntologyAnnotation<ArcStr>) -> Self {
        OntologyAnnotation::from(value)
    }
}

impl FromCompatible<&horned_owl::model::OntologyAnnotation<ArcStr>> for OntologyAnnotation {
    fn from_c(value: &horned_owl::model::OntologyAnnotation<ArcStr>) -> Self {
        OntologyAnnotation::from(value)
    }
}

impl FromCompatible<OntologyAnnotation> for horned_owl::model::OntologyAnnotation<ArcStr> {
    fn from_c(value: OntologyAnnotation) -> Self {
        horned_owl::model::OntologyAnnotation::<ArcStr>::from(value)
    }
}

impl FromCompatible<&OntologyAnnotation> for horned_owl::model::OntologyAnnotation<ArcStr> {
    fn from_c(value: &OntologyAnnotation) -> Self {
        horned_owl::model::OntologyAnnotation::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::OntologyAnnotation<ArcStr>> for OntologyAnnotation {
    fn from(value: horned_owl::model::OntologyAnnotation<ArcStr>) -> Self {
        OntologyAnnotation::from(value.borrow())
    }
}

impl From<OntologyAnnotation> for horned_owl::model::OntologyAnnotation<ArcStr> {
    fn from(value: OntologyAnnotation) -> Self {
        horned_owl::model::OntologyAnnotation::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<OntologyAnnotation>> for Box<horned_owl::model::OntologyAnnotation<ArcStr>> {
    fn from(value: &BoxWrap<OntologyAnnotation>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::OntologyAnnotation<ArcStr>>> for BoxWrap<OntologyAnnotation> {
    fn from(value: &Box<horned_owl::model::OntologyAnnotation<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<OntologyAnnotation>::into(*value.clone())))
    }
}

impl From<BoxWrap<OntologyAnnotation>> for Box<horned_owl::model::OntologyAnnotation<ArcStr>> {
    fn from(value: BoxWrap<OntologyAnnotation>) -> Self {
        Into::<Box<horned_owl::model::OntologyAnnotation<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::OntologyAnnotation<ArcStr>>> for BoxWrap<OntologyAnnotation> {
    fn from(value: Box<horned_owl::model::OntologyAnnotation<ArcStr>>) -> Self {
        Into::<BoxWrap<OntologyAnnotation>>::into(value.borrow())
    }
}

impl From<VecWrap<OntologyAnnotation>> for Vec<horned_owl::model::OntologyAnnotation<ArcStr>> {
    fn from(value: VecWrap<OntologyAnnotation>) -> Self {
        Into::<Vec<horned_owl::model::OntologyAnnotation<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::OntologyAnnotation<ArcStr>>> for VecWrap<OntologyAnnotation> {
    fn from(value: Vec<horned_owl::model::OntologyAnnotation<ArcStr>>) -> Self {
        Into::<VecWrap<OntologyAnnotation>>::into(value.borrow())
    }
}

impl From<&VecWrap<OntologyAnnotation>> for Vec<horned_owl::model::OntologyAnnotation<ArcStr>> {
    fn from(value: &VecWrap<OntologyAnnotation>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::OntologyAnnotation::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::OntologyAnnotation<ArcStr>>> for VecWrap<OntologyAnnotation> {
    fn from(value: &Vec<horned_owl::model::OntologyAnnotation<ArcStr>>) -> Self {
        VecWrap(value.iter().map(OntologyAnnotation::from).collect())
    }
}

impl FromCompatible<&BoxWrap<OntologyAnnotation>> for Box<horned_owl::model::OntologyAnnotation<ArcStr>> {
    fn from_c(value: &BoxWrap<OntologyAnnotation>) -> Self {
        Box::<horned_owl::model::OntologyAnnotation<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::OntologyAnnotation<ArcStr>>> for BoxWrap<OntologyAnnotation> {
    fn from_c(value: &Box<horned_owl::model::OntologyAnnotation<ArcStr>>) -> Self {
        BoxWrap::<OntologyAnnotation>::from(value)
    }
}
impl FromCompatible<BoxWrap<OntologyAnnotation>> for Box<horned_owl::model::OntologyAnnotation<ArcStr>> {
    fn from_c(value: BoxWrap<OntologyAnnotation>) -> Self {
        Box::<horned_owl::model::OntologyAnnotation<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::OntologyAnnotation<ArcStr>>> for BoxWrap<OntologyAnnotation> {
    fn from_c(value: Box<horned_owl::model::OntologyAnnotation<ArcStr>>) -> Self {
        BoxWrap::<OntologyAnnotation>::from(value)
    }
}
impl FromCompatible<VecWrap<OntologyAnnotation>> for Vec<horned_owl::model::OntologyAnnotation<ArcStr>> {
    fn from_c(value: VecWrap<OntologyAnnotation>) -> Self {
        Vec::<horned_owl::model::OntologyAnnotation<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::OntologyAnnotation<ArcStr>>> for VecWrap<OntologyAnnotation> {
    fn from_c(value: Vec<horned_owl::model::OntologyAnnotation<ArcStr>>) -> Self {
        VecWrap::<OntologyAnnotation>::from(value)
    }
}
impl FromCompatible<&VecWrap<OntologyAnnotation>> for Vec<horned_owl::model::OntologyAnnotation<ArcStr>> {
    fn from_c(value: &VecWrap<OntologyAnnotation>) -> Self {
        Vec::<horned_owl::model::OntologyAnnotation<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::OntologyAnnotation<ArcStr>>> for VecWrap<OntologyAnnotation> {
    fn from_c(value: &Vec<horned_owl::model::OntologyAnnotation<ArcStr>>) -> Self {
        VecWrap::<OntologyAnnotation>::from(value)
    }
}
#[doc = concat!(
    "Import(first: IRI,)",
    "\n\n",
    doc!(Import)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Import (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl Import {
    #[new]
    fn new(first: IRI,) -> Self {
        Import (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::Import<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::Import<ArcStr>> for Import {
    fn from(value: &horned_owl::model::Import<ArcStr>) -> Self {

        Import (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&Import> for horned_owl::model::Import<ArcStr> {
    fn from(value: &Import) -> Self {
        horned_owl::model::Import::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for Import ****************/
impl FromCompatible<horned_owl::model::Import<ArcStr>> for Import {
    fn from_c(value: horned_owl::model::Import<ArcStr>) -> Self {
        Import::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Import<ArcStr>> for Import {
    fn from_c(value: &horned_owl::model::Import<ArcStr>) -> Self {
        Import::from(value)
    }
}

impl FromCompatible<Import> for horned_owl::model::Import<ArcStr> {
    fn from_c(value: Import) -> Self {
        horned_owl::model::Import::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Import> for horned_owl::model::Import<ArcStr> {
    fn from_c(value: &Import) -> Self {
        horned_owl::model::Import::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Import<ArcStr>> for Import {
    fn from(value: horned_owl::model::Import<ArcStr>) -> Self {
        Import::from(value.borrow())
    }
}

impl From<Import> for horned_owl::model::Import<ArcStr> {
    fn from(value: Import) -> Self {
        horned_owl::model::Import::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Import>> for Box<horned_owl::model::Import<ArcStr>> {
    fn from(value: &BoxWrap<Import>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Import<ArcStr>>> for BoxWrap<Import> {
    fn from(value: &Box<horned_owl::model::Import<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Import>::into(*value.clone())))
    }
}

impl From<BoxWrap<Import>> for Box<horned_owl::model::Import<ArcStr>> {
    fn from(value: BoxWrap<Import>) -> Self {
        Into::<Box<horned_owl::model::Import<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Import<ArcStr>>> for BoxWrap<Import> {
    fn from(value: Box<horned_owl::model::Import<ArcStr>>) -> Self {
        Into::<BoxWrap<Import>>::into(value.borrow())
    }
}

impl From<VecWrap<Import>> for Vec<horned_owl::model::Import<ArcStr>> {
    fn from(value: VecWrap<Import>) -> Self {
        Into::<Vec<horned_owl::model::Import<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Import<ArcStr>>> for VecWrap<Import> {
    fn from(value: Vec<horned_owl::model::Import<ArcStr>>) -> Self {
        Into::<VecWrap<Import>>::into(value.borrow())
    }
}

impl From<&VecWrap<Import>> for Vec<horned_owl::model::Import<ArcStr>> {
    fn from(value: &VecWrap<Import>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Import::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Import<ArcStr>>> for VecWrap<Import> {
    fn from(value: &Vec<horned_owl::model::Import<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Import::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Import>> for Box<horned_owl::model::Import<ArcStr>> {
    fn from_c(value: &BoxWrap<Import>) -> Self {
        Box::<horned_owl::model::Import<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Import<ArcStr>>> for BoxWrap<Import> {
    fn from_c(value: &Box<horned_owl::model::Import<ArcStr>>) -> Self {
        BoxWrap::<Import>::from(value)
    }
}
impl FromCompatible<BoxWrap<Import>> for Box<horned_owl::model::Import<ArcStr>> {
    fn from_c(value: BoxWrap<Import>) -> Self {
        Box::<horned_owl::model::Import<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Import<ArcStr>>> for BoxWrap<Import> {
    fn from_c(value: Box<horned_owl::model::Import<ArcStr>>) -> Self {
        BoxWrap::<Import>::from(value)
    }
}
impl FromCompatible<VecWrap<Import>> for Vec<horned_owl::model::Import<ArcStr>> {
    fn from_c(value: VecWrap<Import>) -> Self {
        Vec::<horned_owl::model::Import<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Import<ArcStr>>> for VecWrap<Import> {
    fn from_c(value: Vec<horned_owl::model::Import<ArcStr>>) -> Self {
        VecWrap::<Import>::from(value)
    }
}
impl FromCompatible<&VecWrap<Import>> for Vec<horned_owl::model::Import<ArcStr>> {
    fn from_c(value: &VecWrap<Import>) -> Self {
        Vec::<horned_owl::model::Import<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Import<ArcStr>>> for VecWrap<Import> {
    fn from_c(value: &Vec<horned_owl::model::Import<ArcStr>>) -> Self {
        VecWrap::<Import>::from(value)
    }
}
#[doc = concat!(
    "DeclareClass(first: Class,)",
    "\n\n",
    doc!(DeclareClass)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclareClass (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub Class,
);

#[pymethods]
impl DeclareClass {
    #[new]
    fn new(first: Class,) -> Self {
        DeclareClass (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DeclareClass<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DeclareClass<ArcStr>> for DeclareClass {
    fn from(value: &horned_owl::model::DeclareClass<ArcStr>) -> Self {

        DeclareClass (
    IntoCompatible::<Class>::into_c(&value.0),
        )
    }
}

impl From<&DeclareClass> for horned_owl::model::DeclareClass<ArcStr> {
    fn from(value: &DeclareClass) -> Self {
        horned_owl::model::DeclareClass::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DeclareClass ****************/
impl FromCompatible<horned_owl::model::DeclareClass<ArcStr>> for DeclareClass {
    fn from_c(value: horned_owl::model::DeclareClass<ArcStr>) -> Self {
        DeclareClass::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DeclareClass<ArcStr>> for DeclareClass {
    fn from_c(value: &horned_owl::model::DeclareClass<ArcStr>) -> Self {
        DeclareClass::from(value)
    }
}

impl FromCompatible<DeclareClass> for horned_owl::model::DeclareClass<ArcStr> {
    fn from_c(value: DeclareClass) -> Self {
        horned_owl::model::DeclareClass::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DeclareClass> for horned_owl::model::DeclareClass<ArcStr> {
    fn from_c(value: &DeclareClass) -> Self {
        horned_owl::model::DeclareClass::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DeclareClass<ArcStr>> for DeclareClass {
    fn from(value: horned_owl::model::DeclareClass<ArcStr>) -> Self {
        DeclareClass::from(value.borrow())
    }
}

impl From<DeclareClass> for horned_owl::model::DeclareClass<ArcStr> {
    fn from(value: DeclareClass) -> Self {
        horned_owl::model::DeclareClass::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DeclareClass>> for Box<horned_owl::model::DeclareClass<ArcStr>> {
    fn from(value: &BoxWrap<DeclareClass>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DeclareClass<ArcStr>>> for BoxWrap<DeclareClass> {
    fn from(value: &Box<horned_owl::model::DeclareClass<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DeclareClass>::into(*value.clone())))
    }
}

impl From<BoxWrap<DeclareClass>> for Box<horned_owl::model::DeclareClass<ArcStr>> {
    fn from(value: BoxWrap<DeclareClass>) -> Self {
        Into::<Box<horned_owl::model::DeclareClass<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DeclareClass<ArcStr>>> for BoxWrap<DeclareClass> {
    fn from(value: Box<horned_owl::model::DeclareClass<ArcStr>>) -> Self {
        Into::<BoxWrap<DeclareClass>>::into(value.borrow())
    }
}

impl From<VecWrap<DeclareClass>> for Vec<horned_owl::model::DeclareClass<ArcStr>> {
    fn from(value: VecWrap<DeclareClass>) -> Self {
        Into::<Vec<horned_owl::model::DeclareClass<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DeclareClass<ArcStr>>> for VecWrap<DeclareClass> {
    fn from(value: Vec<horned_owl::model::DeclareClass<ArcStr>>) -> Self {
        Into::<VecWrap<DeclareClass>>::into(value.borrow())
    }
}

impl From<&VecWrap<DeclareClass>> for Vec<horned_owl::model::DeclareClass<ArcStr>> {
    fn from(value: &VecWrap<DeclareClass>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DeclareClass::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DeclareClass<ArcStr>>> for VecWrap<DeclareClass> {
    fn from(value: &Vec<horned_owl::model::DeclareClass<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DeclareClass::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DeclareClass>> for Box<horned_owl::model::DeclareClass<ArcStr>> {
    fn from_c(value: &BoxWrap<DeclareClass>) -> Self {
        Box::<horned_owl::model::DeclareClass<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DeclareClass<ArcStr>>> for BoxWrap<DeclareClass> {
    fn from_c(value: &Box<horned_owl::model::DeclareClass<ArcStr>>) -> Self {
        BoxWrap::<DeclareClass>::from(value)
    }
}
impl FromCompatible<BoxWrap<DeclareClass>> for Box<horned_owl::model::DeclareClass<ArcStr>> {
    fn from_c(value: BoxWrap<DeclareClass>) -> Self {
        Box::<horned_owl::model::DeclareClass<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DeclareClass<ArcStr>>> for BoxWrap<DeclareClass> {
    fn from_c(value: Box<horned_owl::model::DeclareClass<ArcStr>>) -> Self {
        BoxWrap::<DeclareClass>::from(value)
    }
}
impl FromCompatible<VecWrap<DeclareClass>> for Vec<horned_owl::model::DeclareClass<ArcStr>> {
    fn from_c(value: VecWrap<DeclareClass>) -> Self {
        Vec::<horned_owl::model::DeclareClass<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DeclareClass<ArcStr>>> for VecWrap<DeclareClass> {
    fn from_c(value: Vec<horned_owl::model::DeclareClass<ArcStr>>) -> Self {
        VecWrap::<DeclareClass>::from(value)
    }
}
impl FromCompatible<&VecWrap<DeclareClass>> for Vec<horned_owl::model::DeclareClass<ArcStr>> {
    fn from_c(value: &VecWrap<DeclareClass>) -> Self {
        Vec::<horned_owl::model::DeclareClass<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DeclareClass<ArcStr>>> for VecWrap<DeclareClass> {
    fn from_c(value: &Vec<horned_owl::model::DeclareClass<ArcStr>>) -> Self {
        VecWrap::<DeclareClass>::from(value)
    }
}
#[doc = concat!(
    "DeclareObjectProperty(first: ObjectProperty,)",
    "\n\n",
    doc!(DeclareObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclareObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectProperty,
);

#[pymethods]
impl DeclareObjectProperty {
    #[new]
    fn new(first: ObjectProperty,) -> Self {
        DeclareObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DeclareObjectProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DeclareObjectProperty<ArcStr>> for DeclareObjectProperty {
    fn from(value: &horned_owl::model::DeclareObjectProperty<ArcStr>) -> Self {

        DeclareObjectProperty (
    IntoCompatible::<ObjectProperty>::into_c(&value.0),
        )
    }
}

impl From<&DeclareObjectProperty> for horned_owl::model::DeclareObjectProperty<ArcStr> {
    fn from(value: &DeclareObjectProperty) -> Self {
        horned_owl::model::DeclareObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DeclareObjectProperty ****************/
impl FromCompatible<horned_owl::model::DeclareObjectProperty<ArcStr>> for DeclareObjectProperty {
    fn from_c(value: horned_owl::model::DeclareObjectProperty<ArcStr>) -> Self {
        DeclareObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DeclareObjectProperty<ArcStr>> for DeclareObjectProperty {
    fn from_c(value: &horned_owl::model::DeclareObjectProperty<ArcStr>) -> Self {
        DeclareObjectProperty::from(value)
    }
}

impl FromCompatible<DeclareObjectProperty> for horned_owl::model::DeclareObjectProperty<ArcStr> {
    fn from_c(value: DeclareObjectProperty) -> Self {
        horned_owl::model::DeclareObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DeclareObjectProperty> for horned_owl::model::DeclareObjectProperty<ArcStr> {
    fn from_c(value: &DeclareObjectProperty) -> Self {
        horned_owl::model::DeclareObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DeclareObjectProperty<ArcStr>> for DeclareObjectProperty {
    fn from(value: horned_owl::model::DeclareObjectProperty<ArcStr>) -> Self {
        DeclareObjectProperty::from(value.borrow())
    }
}

impl From<DeclareObjectProperty> for horned_owl::model::DeclareObjectProperty<ArcStr> {
    fn from(value: DeclareObjectProperty) -> Self {
        horned_owl::model::DeclareObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DeclareObjectProperty>> for Box<horned_owl::model::DeclareObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<DeclareObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DeclareObjectProperty<ArcStr>>> for BoxWrap<DeclareObjectProperty> {
    fn from(value: &Box<horned_owl::model::DeclareObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DeclareObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<DeclareObjectProperty>> for Box<horned_owl::model::DeclareObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<DeclareObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::DeclareObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DeclareObjectProperty<ArcStr>>> for BoxWrap<DeclareObjectProperty> {
    fn from(value: Box<horned_owl::model::DeclareObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<DeclareObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<DeclareObjectProperty>> for Vec<horned_owl::model::DeclareObjectProperty<ArcStr>> {
    fn from(value: VecWrap<DeclareObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>> for VecWrap<DeclareObjectProperty> {
    fn from(value: Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<DeclareObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<DeclareObjectProperty>> for Vec<horned_owl::model::DeclareObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<DeclareObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DeclareObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>> for VecWrap<DeclareObjectProperty> {
    fn from(value: &Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DeclareObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DeclareObjectProperty>> for Box<horned_owl::model::DeclareObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<DeclareObjectProperty>) -> Self {
        Box::<horned_owl::model::DeclareObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DeclareObjectProperty<ArcStr>>> for BoxWrap<DeclareObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::DeclareObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<DeclareObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<DeclareObjectProperty>> for Box<horned_owl::model::DeclareObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<DeclareObjectProperty>) -> Self {
        Box::<horned_owl::model::DeclareObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DeclareObjectProperty<ArcStr>>> for BoxWrap<DeclareObjectProperty> {
    fn from_c(value: Box<horned_owl::model::DeclareObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<DeclareObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<DeclareObjectProperty>> for Vec<horned_owl::model::DeclareObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<DeclareObjectProperty>) -> Self {
        Vec::<horned_owl::model::DeclareObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>> for VecWrap<DeclareObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>) -> Self {
        VecWrap::<DeclareObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<DeclareObjectProperty>> for Vec<horned_owl::model::DeclareObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<DeclareObjectProperty>) -> Self {
        Vec::<horned_owl::model::DeclareObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>> for VecWrap<DeclareObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::DeclareObjectProperty<ArcStr>>) -> Self {
        VecWrap::<DeclareObjectProperty>::from(value)
    }
}
#[doc = concat!(
    "DeclareAnnotationProperty(first: AnnotationProperty,)",
    "\n\n",
    doc!(DeclareAnnotationProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclareAnnotationProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub AnnotationProperty,
);

#[pymethods]
impl DeclareAnnotationProperty {
    #[new]
    fn new(first: AnnotationProperty,) -> Self {
        DeclareAnnotationProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DeclareAnnotationProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DeclareAnnotationProperty<ArcStr>> for DeclareAnnotationProperty {
    fn from(value: &horned_owl::model::DeclareAnnotationProperty<ArcStr>) -> Self {

        DeclareAnnotationProperty (
    IntoCompatible::<AnnotationProperty>::into_c(&value.0),
        )
    }
}

impl From<&DeclareAnnotationProperty> for horned_owl::model::DeclareAnnotationProperty<ArcStr> {
    fn from(value: &DeclareAnnotationProperty) -> Self {
        horned_owl::model::DeclareAnnotationProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DeclareAnnotationProperty ****************/
impl FromCompatible<horned_owl::model::DeclareAnnotationProperty<ArcStr>> for DeclareAnnotationProperty {
    fn from_c(value: horned_owl::model::DeclareAnnotationProperty<ArcStr>) -> Self {
        DeclareAnnotationProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DeclareAnnotationProperty<ArcStr>> for DeclareAnnotationProperty {
    fn from_c(value: &horned_owl::model::DeclareAnnotationProperty<ArcStr>) -> Self {
        DeclareAnnotationProperty::from(value)
    }
}

impl FromCompatible<DeclareAnnotationProperty> for horned_owl::model::DeclareAnnotationProperty<ArcStr> {
    fn from_c(value: DeclareAnnotationProperty) -> Self {
        horned_owl::model::DeclareAnnotationProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DeclareAnnotationProperty> for horned_owl::model::DeclareAnnotationProperty<ArcStr> {
    fn from_c(value: &DeclareAnnotationProperty) -> Self {
        horned_owl::model::DeclareAnnotationProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DeclareAnnotationProperty<ArcStr>> for DeclareAnnotationProperty {
    fn from(value: horned_owl::model::DeclareAnnotationProperty<ArcStr>) -> Self {
        DeclareAnnotationProperty::from(value.borrow())
    }
}

impl From<DeclareAnnotationProperty> for horned_owl::model::DeclareAnnotationProperty<ArcStr> {
    fn from(value: DeclareAnnotationProperty) -> Self {
        horned_owl::model::DeclareAnnotationProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DeclareAnnotationProperty>> for Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>> {
    fn from(value: &BoxWrap<DeclareAnnotationProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>> for BoxWrap<DeclareAnnotationProperty> {
    fn from(value: &Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DeclareAnnotationProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<DeclareAnnotationProperty>> for Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>> {
    fn from(value: BoxWrap<DeclareAnnotationProperty>) -> Self {
        Into::<Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>> for BoxWrap<DeclareAnnotationProperty> {
    fn from(value: Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<DeclareAnnotationProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<DeclareAnnotationProperty>> for Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>> {
    fn from(value: VecWrap<DeclareAnnotationProperty>) -> Self {
        Into::<Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>> for VecWrap<DeclareAnnotationProperty> {
    fn from(value: Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>) -> Self {
        Into::<VecWrap<DeclareAnnotationProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<DeclareAnnotationProperty>> for Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>> {
    fn from(value: &VecWrap<DeclareAnnotationProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DeclareAnnotationProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>> for VecWrap<DeclareAnnotationProperty> {
    fn from(value: &Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DeclareAnnotationProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DeclareAnnotationProperty>> for Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<DeclareAnnotationProperty>) -> Self {
        Box::<horned_owl::model::DeclareAnnotationProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>> for BoxWrap<DeclareAnnotationProperty> {
    fn from_c(value: &Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>) -> Self {
        BoxWrap::<DeclareAnnotationProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<DeclareAnnotationProperty>> for Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>> {
    fn from_c(value: BoxWrap<DeclareAnnotationProperty>) -> Self {
        Box::<horned_owl::model::DeclareAnnotationProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>> for BoxWrap<DeclareAnnotationProperty> {
    fn from_c(value: Box<horned_owl::model::DeclareAnnotationProperty<ArcStr>>) -> Self {
        BoxWrap::<DeclareAnnotationProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<DeclareAnnotationProperty>> for Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>> {
    fn from_c(value: VecWrap<DeclareAnnotationProperty>) -> Self {
        Vec::<horned_owl::model::DeclareAnnotationProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>> for VecWrap<DeclareAnnotationProperty> {
    fn from_c(value: Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>) -> Self {
        VecWrap::<DeclareAnnotationProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<DeclareAnnotationProperty>> for Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>> {
    fn from_c(value: &VecWrap<DeclareAnnotationProperty>) -> Self {
        Vec::<horned_owl::model::DeclareAnnotationProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>> for VecWrap<DeclareAnnotationProperty> {
    fn from_c(value: &Vec<horned_owl::model::DeclareAnnotationProperty<ArcStr>>) -> Self {
        VecWrap::<DeclareAnnotationProperty>::from(value)
    }
}
#[doc = concat!(
    "DeclareDataProperty(first: DataProperty,)",
    "\n\n",
    doc!(DeclareDataProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclareDataProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub DataProperty,
);

#[pymethods]
impl DeclareDataProperty {
    #[new]
    fn new(first: DataProperty,) -> Self {
        DeclareDataProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DeclareDataProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DeclareDataProperty<ArcStr>> for DeclareDataProperty {
    fn from(value: &horned_owl::model::DeclareDataProperty<ArcStr>) -> Self {

        DeclareDataProperty (
    IntoCompatible::<DataProperty>::into_c(&value.0),
        )
    }
}

impl From<&DeclareDataProperty> for horned_owl::model::DeclareDataProperty<ArcStr> {
    fn from(value: &DeclareDataProperty) -> Self {
        horned_owl::model::DeclareDataProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DeclareDataProperty ****************/
impl FromCompatible<horned_owl::model::DeclareDataProperty<ArcStr>> for DeclareDataProperty {
    fn from_c(value: horned_owl::model::DeclareDataProperty<ArcStr>) -> Self {
        DeclareDataProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DeclareDataProperty<ArcStr>> for DeclareDataProperty {
    fn from_c(value: &horned_owl::model::DeclareDataProperty<ArcStr>) -> Self {
        DeclareDataProperty::from(value)
    }
}

impl FromCompatible<DeclareDataProperty> for horned_owl::model::DeclareDataProperty<ArcStr> {
    fn from_c(value: DeclareDataProperty) -> Self {
        horned_owl::model::DeclareDataProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DeclareDataProperty> for horned_owl::model::DeclareDataProperty<ArcStr> {
    fn from_c(value: &DeclareDataProperty) -> Self {
        horned_owl::model::DeclareDataProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DeclareDataProperty<ArcStr>> for DeclareDataProperty {
    fn from(value: horned_owl::model::DeclareDataProperty<ArcStr>) -> Self {
        DeclareDataProperty::from(value.borrow())
    }
}

impl From<DeclareDataProperty> for horned_owl::model::DeclareDataProperty<ArcStr> {
    fn from(value: DeclareDataProperty) -> Self {
        horned_owl::model::DeclareDataProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DeclareDataProperty>> for Box<horned_owl::model::DeclareDataProperty<ArcStr>> {
    fn from(value: &BoxWrap<DeclareDataProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DeclareDataProperty<ArcStr>>> for BoxWrap<DeclareDataProperty> {
    fn from(value: &Box<horned_owl::model::DeclareDataProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DeclareDataProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<DeclareDataProperty>> for Box<horned_owl::model::DeclareDataProperty<ArcStr>> {
    fn from(value: BoxWrap<DeclareDataProperty>) -> Self {
        Into::<Box<horned_owl::model::DeclareDataProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DeclareDataProperty<ArcStr>>> for BoxWrap<DeclareDataProperty> {
    fn from(value: Box<horned_owl::model::DeclareDataProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<DeclareDataProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<DeclareDataProperty>> for Vec<horned_owl::model::DeclareDataProperty<ArcStr>> {
    fn from(value: VecWrap<DeclareDataProperty>) -> Self {
        Into::<Vec<horned_owl::model::DeclareDataProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DeclareDataProperty<ArcStr>>> for VecWrap<DeclareDataProperty> {
    fn from(value: Vec<horned_owl::model::DeclareDataProperty<ArcStr>>) -> Self {
        Into::<VecWrap<DeclareDataProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<DeclareDataProperty>> for Vec<horned_owl::model::DeclareDataProperty<ArcStr>> {
    fn from(value: &VecWrap<DeclareDataProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DeclareDataProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DeclareDataProperty<ArcStr>>> for VecWrap<DeclareDataProperty> {
    fn from(value: &Vec<horned_owl::model::DeclareDataProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DeclareDataProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DeclareDataProperty>> for Box<horned_owl::model::DeclareDataProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<DeclareDataProperty>) -> Self {
        Box::<horned_owl::model::DeclareDataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DeclareDataProperty<ArcStr>>> for BoxWrap<DeclareDataProperty> {
    fn from_c(value: &Box<horned_owl::model::DeclareDataProperty<ArcStr>>) -> Self {
        BoxWrap::<DeclareDataProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<DeclareDataProperty>> for Box<horned_owl::model::DeclareDataProperty<ArcStr>> {
    fn from_c(value: BoxWrap<DeclareDataProperty>) -> Self {
        Box::<horned_owl::model::DeclareDataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DeclareDataProperty<ArcStr>>> for BoxWrap<DeclareDataProperty> {
    fn from_c(value: Box<horned_owl::model::DeclareDataProperty<ArcStr>>) -> Self {
        BoxWrap::<DeclareDataProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<DeclareDataProperty>> for Vec<horned_owl::model::DeclareDataProperty<ArcStr>> {
    fn from_c(value: VecWrap<DeclareDataProperty>) -> Self {
        Vec::<horned_owl::model::DeclareDataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DeclareDataProperty<ArcStr>>> for VecWrap<DeclareDataProperty> {
    fn from_c(value: Vec<horned_owl::model::DeclareDataProperty<ArcStr>>) -> Self {
        VecWrap::<DeclareDataProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<DeclareDataProperty>> for Vec<horned_owl::model::DeclareDataProperty<ArcStr>> {
    fn from_c(value: &VecWrap<DeclareDataProperty>) -> Self {
        Vec::<horned_owl::model::DeclareDataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DeclareDataProperty<ArcStr>>> for VecWrap<DeclareDataProperty> {
    fn from_c(value: &Vec<horned_owl::model::DeclareDataProperty<ArcStr>>) -> Self {
        VecWrap::<DeclareDataProperty>::from(value)
    }
}
#[doc = concat!(
    "DeclareNamedIndividual(first: NamedIndividual,)",
    "\n\n",
    doc!(DeclareNamedIndividual)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclareNamedIndividual (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub NamedIndividual,
);

#[pymethods]
impl DeclareNamedIndividual {
    #[new]
    fn new(first: NamedIndividual,) -> Self {
        DeclareNamedIndividual (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DeclareNamedIndividual<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DeclareNamedIndividual<ArcStr>> for DeclareNamedIndividual {
    fn from(value: &horned_owl::model::DeclareNamedIndividual<ArcStr>) -> Self {

        DeclareNamedIndividual (
    IntoCompatible::<NamedIndividual>::into_c(&value.0),
        )
    }
}

impl From<&DeclareNamedIndividual> for horned_owl::model::DeclareNamedIndividual<ArcStr> {
    fn from(value: &DeclareNamedIndividual) -> Self {
        horned_owl::model::DeclareNamedIndividual::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DeclareNamedIndividual ****************/
impl FromCompatible<horned_owl::model::DeclareNamedIndividual<ArcStr>> for DeclareNamedIndividual {
    fn from_c(value: horned_owl::model::DeclareNamedIndividual<ArcStr>) -> Self {
        DeclareNamedIndividual::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DeclareNamedIndividual<ArcStr>> for DeclareNamedIndividual {
    fn from_c(value: &horned_owl::model::DeclareNamedIndividual<ArcStr>) -> Self {
        DeclareNamedIndividual::from(value)
    }
}

impl FromCompatible<DeclareNamedIndividual> for horned_owl::model::DeclareNamedIndividual<ArcStr> {
    fn from_c(value: DeclareNamedIndividual) -> Self {
        horned_owl::model::DeclareNamedIndividual::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DeclareNamedIndividual> for horned_owl::model::DeclareNamedIndividual<ArcStr> {
    fn from_c(value: &DeclareNamedIndividual) -> Self {
        horned_owl::model::DeclareNamedIndividual::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DeclareNamedIndividual<ArcStr>> for DeclareNamedIndividual {
    fn from(value: horned_owl::model::DeclareNamedIndividual<ArcStr>) -> Self {
        DeclareNamedIndividual::from(value.borrow())
    }
}

impl From<DeclareNamedIndividual> for horned_owl::model::DeclareNamedIndividual<ArcStr> {
    fn from(value: DeclareNamedIndividual) -> Self {
        horned_owl::model::DeclareNamedIndividual::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DeclareNamedIndividual>> for Box<horned_owl::model::DeclareNamedIndividual<ArcStr>> {
    fn from(value: &BoxWrap<DeclareNamedIndividual>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>> for BoxWrap<DeclareNamedIndividual> {
    fn from(value: &Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DeclareNamedIndividual>::into(*value.clone())))
    }
}

impl From<BoxWrap<DeclareNamedIndividual>> for Box<horned_owl::model::DeclareNamedIndividual<ArcStr>> {
    fn from(value: BoxWrap<DeclareNamedIndividual>) -> Self {
        Into::<Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>> for BoxWrap<DeclareNamedIndividual> {
    fn from(value: Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>) -> Self {
        Into::<BoxWrap<DeclareNamedIndividual>>::into(value.borrow())
    }
}

impl From<VecWrap<DeclareNamedIndividual>> for Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>> {
    fn from(value: VecWrap<DeclareNamedIndividual>) -> Self {
        Into::<Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>> for VecWrap<DeclareNamedIndividual> {
    fn from(value: Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>) -> Self {
        Into::<VecWrap<DeclareNamedIndividual>>::into(value.borrow())
    }
}

impl From<&VecWrap<DeclareNamedIndividual>> for Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>> {
    fn from(value: &VecWrap<DeclareNamedIndividual>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DeclareNamedIndividual::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>> for VecWrap<DeclareNamedIndividual> {
    fn from(value: &Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DeclareNamedIndividual::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DeclareNamedIndividual>> for Box<horned_owl::model::DeclareNamedIndividual<ArcStr>> {
    fn from_c(value: &BoxWrap<DeclareNamedIndividual>) -> Self {
        Box::<horned_owl::model::DeclareNamedIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>> for BoxWrap<DeclareNamedIndividual> {
    fn from_c(value: &Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>) -> Self {
        BoxWrap::<DeclareNamedIndividual>::from(value)
    }
}
impl FromCompatible<BoxWrap<DeclareNamedIndividual>> for Box<horned_owl::model::DeclareNamedIndividual<ArcStr>> {
    fn from_c(value: BoxWrap<DeclareNamedIndividual>) -> Self {
        Box::<horned_owl::model::DeclareNamedIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>> for BoxWrap<DeclareNamedIndividual> {
    fn from_c(value: Box<horned_owl::model::DeclareNamedIndividual<ArcStr>>) -> Self {
        BoxWrap::<DeclareNamedIndividual>::from(value)
    }
}
impl FromCompatible<VecWrap<DeclareNamedIndividual>> for Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>> {
    fn from_c(value: VecWrap<DeclareNamedIndividual>) -> Self {
        Vec::<horned_owl::model::DeclareNamedIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>> for VecWrap<DeclareNamedIndividual> {
    fn from_c(value: Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>) -> Self {
        VecWrap::<DeclareNamedIndividual>::from(value)
    }
}
impl FromCompatible<&VecWrap<DeclareNamedIndividual>> for Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>> {
    fn from_c(value: &VecWrap<DeclareNamedIndividual>) -> Self {
        Vec::<horned_owl::model::DeclareNamedIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>> for VecWrap<DeclareNamedIndividual> {
    fn from_c(value: &Vec<horned_owl::model::DeclareNamedIndividual<ArcStr>>) -> Self {
        VecWrap::<DeclareNamedIndividual>::from(value)
    }
}
#[doc = concat!(
    "DeclareDatatype(first: Datatype,)",
    "\n\n",
    doc!(DeclareDatatype)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeclareDatatype (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub Datatype,
);

#[pymethods]
impl DeclareDatatype {
    #[new]
    fn new(first: Datatype,) -> Self {
        DeclareDatatype (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DeclareDatatype<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DeclareDatatype<ArcStr>> for DeclareDatatype {
    fn from(value: &horned_owl::model::DeclareDatatype<ArcStr>) -> Self {

        DeclareDatatype (
    IntoCompatible::<Datatype>::into_c(&value.0),
        )
    }
}

impl From<&DeclareDatatype> for horned_owl::model::DeclareDatatype<ArcStr> {
    fn from(value: &DeclareDatatype) -> Self {
        horned_owl::model::DeclareDatatype::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DeclareDatatype ****************/
impl FromCompatible<horned_owl::model::DeclareDatatype<ArcStr>> for DeclareDatatype {
    fn from_c(value: horned_owl::model::DeclareDatatype<ArcStr>) -> Self {
        DeclareDatatype::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DeclareDatatype<ArcStr>> for DeclareDatatype {
    fn from_c(value: &horned_owl::model::DeclareDatatype<ArcStr>) -> Self {
        DeclareDatatype::from(value)
    }
}

impl FromCompatible<DeclareDatatype> for horned_owl::model::DeclareDatatype<ArcStr> {
    fn from_c(value: DeclareDatatype) -> Self {
        horned_owl::model::DeclareDatatype::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DeclareDatatype> for horned_owl::model::DeclareDatatype<ArcStr> {
    fn from_c(value: &DeclareDatatype) -> Self {
        horned_owl::model::DeclareDatatype::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DeclareDatatype<ArcStr>> for DeclareDatatype {
    fn from(value: horned_owl::model::DeclareDatatype<ArcStr>) -> Self {
        DeclareDatatype::from(value.borrow())
    }
}

impl From<DeclareDatatype> for horned_owl::model::DeclareDatatype<ArcStr> {
    fn from(value: DeclareDatatype) -> Self {
        horned_owl::model::DeclareDatatype::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DeclareDatatype>> for Box<horned_owl::model::DeclareDatatype<ArcStr>> {
    fn from(value: &BoxWrap<DeclareDatatype>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DeclareDatatype<ArcStr>>> for BoxWrap<DeclareDatatype> {
    fn from(value: &Box<horned_owl::model::DeclareDatatype<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DeclareDatatype>::into(*value.clone())))
    }
}

impl From<BoxWrap<DeclareDatatype>> for Box<horned_owl::model::DeclareDatatype<ArcStr>> {
    fn from(value: BoxWrap<DeclareDatatype>) -> Self {
        Into::<Box<horned_owl::model::DeclareDatatype<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DeclareDatatype<ArcStr>>> for BoxWrap<DeclareDatatype> {
    fn from(value: Box<horned_owl::model::DeclareDatatype<ArcStr>>) -> Self {
        Into::<BoxWrap<DeclareDatatype>>::into(value.borrow())
    }
}

impl From<VecWrap<DeclareDatatype>> for Vec<horned_owl::model::DeclareDatatype<ArcStr>> {
    fn from(value: VecWrap<DeclareDatatype>) -> Self {
        Into::<Vec<horned_owl::model::DeclareDatatype<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DeclareDatatype<ArcStr>>> for VecWrap<DeclareDatatype> {
    fn from(value: Vec<horned_owl::model::DeclareDatatype<ArcStr>>) -> Self {
        Into::<VecWrap<DeclareDatatype>>::into(value.borrow())
    }
}

impl From<&VecWrap<DeclareDatatype>> for Vec<horned_owl::model::DeclareDatatype<ArcStr>> {
    fn from(value: &VecWrap<DeclareDatatype>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DeclareDatatype::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DeclareDatatype<ArcStr>>> for VecWrap<DeclareDatatype> {
    fn from(value: &Vec<horned_owl::model::DeclareDatatype<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DeclareDatatype::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DeclareDatatype>> for Box<horned_owl::model::DeclareDatatype<ArcStr>> {
    fn from_c(value: &BoxWrap<DeclareDatatype>) -> Self {
        Box::<horned_owl::model::DeclareDatatype<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DeclareDatatype<ArcStr>>> for BoxWrap<DeclareDatatype> {
    fn from_c(value: &Box<horned_owl::model::DeclareDatatype<ArcStr>>) -> Self {
        BoxWrap::<DeclareDatatype>::from(value)
    }
}
impl FromCompatible<BoxWrap<DeclareDatatype>> for Box<horned_owl::model::DeclareDatatype<ArcStr>> {
    fn from_c(value: BoxWrap<DeclareDatatype>) -> Self {
        Box::<horned_owl::model::DeclareDatatype<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DeclareDatatype<ArcStr>>> for BoxWrap<DeclareDatatype> {
    fn from_c(value: Box<horned_owl::model::DeclareDatatype<ArcStr>>) -> Self {
        BoxWrap::<DeclareDatatype>::from(value)
    }
}
impl FromCompatible<VecWrap<DeclareDatatype>> for Vec<horned_owl::model::DeclareDatatype<ArcStr>> {
    fn from_c(value: VecWrap<DeclareDatatype>) -> Self {
        Vec::<horned_owl::model::DeclareDatatype<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DeclareDatatype<ArcStr>>> for VecWrap<DeclareDatatype> {
    fn from_c(value: Vec<horned_owl::model::DeclareDatatype<ArcStr>>) -> Self {
        VecWrap::<DeclareDatatype>::from(value)
    }
}
impl FromCompatible<&VecWrap<DeclareDatatype>> for Vec<horned_owl::model::DeclareDatatype<ArcStr>> {
    fn from_c(value: &VecWrap<DeclareDatatype>) -> Self {
        Vec::<horned_owl::model::DeclareDatatype<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DeclareDatatype<ArcStr>>> for VecWrap<DeclareDatatype> {
    fn from_c(value: &Vec<horned_owl::model::DeclareDatatype<ArcStr>>) -> Self {
        VecWrap::<DeclareDatatype>::from(value)
    }
}
#[doc = concat!("SubClassOf(sub: ClassExpression,sup: ClassExpression,)",
    "\n\n",
    doc!(SubClassOf)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubClassOf {
        #[doc="sub: ClassExpression"]
        #[pyo3(get,set)]
        pub sub: ClassExpression,
    
        #[doc="sup: ClassExpression"]
        #[pyo3(get,set)]
        pub sup: ClassExpression,
    }

#[pymethods]
impl SubClassOf {
    #[new]
    fn new(sub: ClassExpression,sup: ClassExpression,) -> Self {
        SubClassOf {
                sub,
                sup,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "sub" => self.sub.clone().into_pyobject(py).map(Bound::into_any),
            "sup" => self.sup.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "sub" => {
                self.sub = value.extract()?;
                Ok(())
            },
            "sup" => {
                self.sup = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::SubClassOf<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::SubClassOf<ArcStr>> for SubClassOf {
    fn from(value: &horned_owl::model::SubClassOf<ArcStr>) -> Self {
        SubClassOf {
            sub: IntoCompatible::<ClassExpression>::into_c(value.sub.borrow()),
            sup: IntoCompatible::<ClassExpression>::into_c(value.sup.borrow()),
        }
    }
}


impl From<&SubClassOf> for horned_owl::model::SubClassOf<ArcStr> {
    fn from(value: &SubClassOf) -> Self {
        horned_owl::model::SubClassOf::<ArcStr> {
            sub: value.sub.borrow().into_c(),
            sup: value.sup.borrow().into_c(),
        }
    }
}



/**************** Base implementations for SubClassOf ****************/
impl FromCompatible<horned_owl::model::SubClassOf<ArcStr>> for SubClassOf {
    fn from_c(value: horned_owl::model::SubClassOf<ArcStr>) -> Self {
        SubClassOf::from(value)
    }
}

impl FromCompatible<&horned_owl::model::SubClassOf<ArcStr>> for SubClassOf {
    fn from_c(value: &horned_owl::model::SubClassOf<ArcStr>) -> Self {
        SubClassOf::from(value)
    }
}

impl FromCompatible<SubClassOf> for horned_owl::model::SubClassOf<ArcStr> {
    fn from_c(value: SubClassOf) -> Self {
        horned_owl::model::SubClassOf::<ArcStr>::from(value)
    }
}

impl FromCompatible<&SubClassOf> for horned_owl::model::SubClassOf<ArcStr> {
    fn from_c(value: &SubClassOf) -> Self {
        horned_owl::model::SubClassOf::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::SubClassOf<ArcStr>> for SubClassOf {
    fn from(value: horned_owl::model::SubClassOf<ArcStr>) -> Self {
        SubClassOf::from(value.borrow())
    }
}

impl From<SubClassOf> for horned_owl::model::SubClassOf<ArcStr> {
    fn from(value: SubClassOf) -> Self {
        horned_owl::model::SubClassOf::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<SubClassOf>> for Box<horned_owl::model::SubClassOf<ArcStr>> {
    fn from(value: &BoxWrap<SubClassOf>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::SubClassOf<ArcStr>>> for BoxWrap<SubClassOf> {
    fn from(value: &Box<horned_owl::model::SubClassOf<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<SubClassOf>::into(*value.clone())))
    }
}

impl From<BoxWrap<SubClassOf>> for Box<horned_owl::model::SubClassOf<ArcStr>> {
    fn from(value: BoxWrap<SubClassOf>) -> Self {
        Into::<Box<horned_owl::model::SubClassOf<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::SubClassOf<ArcStr>>> for BoxWrap<SubClassOf> {
    fn from(value: Box<horned_owl::model::SubClassOf<ArcStr>>) -> Self {
        Into::<BoxWrap<SubClassOf>>::into(value.borrow())
    }
}

impl From<VecWrap<SubClassOf>> for Vec<horned_owl::model::SubClassOf<ArcStr>> {
    fn from(value: VecWrap<SubClassOf>) -> Self {
        Into::<Vec<horned_owl::model::SubClassOf<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::SubClassOf<ArcStr>>> for VecWrap<SubClassOf> {
    fn from(value: Vec<horned_owl::model::SubClassOf<ArcStr>>) -> Self {
        Into::<VecWrap<SubClassOf>>::into(value.borrow())
    }
}

impl From<&VecWrap<SubClassOf>> for Vec<horned_owl::model::SubClassOf<ArcStr>> {
    fn from(value: &VecWrap<SubClassOf>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::SubClassOf::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::SubClassOf<ArcStr>>> for VecWrap<SubClassOf> {
    fn from(value: &Vec<horned_owl::model::SubClassOf<ArcStr>>) -> Self {
        VecWrap(value.iter().map(SubClassOf::from).collect())
    }
}

impl FromCompatible<&BoxWrap<SubClassOf>> for Box<horned_owl::model::SubClassOf<ArcStr>> {
    fn from_c(value: &BoxWrap<SubClassOf>) -> Self {
        Box::<horned_owl::model::SubClassOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::SubClassOf<ArcStr>>> for BoxWrap<SubClassOf> {
    fn from_c(value: &Box<horned_owl::model::SubClassOf<ArcStr>>) -> Self {
        BoxWrap::<SubClassOf>::from(value)
    }
}
impl FromCompatible<BoxWrap<SubClassOf>> for Box<horned_owl::model::SubClassOf<ArcStr>> {
    fn from_c(value: BoxWrap<SubClassOf>) -> Self {
        Box::<horned_owl::model::SubClassOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::SubClassOf<ArcStr>>> for BoxWrap<SubClassOf> {
    fn from_c(value: Box<horned_owl::model::SubClassOf<ArcStr>>) -> Self {
        BoxWrap::<SubClassOf>::from(value)
    }
}
impl FromCompatible<VecWrap<SubClassOf>> for Vec<horned_owl::model::SubClassOf<ArcStr>> {
    fn from_c(value: VecWrap<SubClassOf>) -> Self {
        Vec::<horned_owl::model::SubClassOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::SubClassOf<ArcStr>>> for VecWrap<SubClassOf> {
    fn from_c(value: Vec<horned_owl::model::SubClassOf<ArcStr>>) -> Self {
        VecWrap::<SubClassOf>::from(value)
    }
}
impl FromCompatible<&VecWrap<SubClassOf>> for Vec<horned_owl::model::SubClassOf<ArcStr>> {
    fn from_c(value: &VecWrap<SubClassOf>) -> Self {
        Vec::<horned_owl::model::SubClassOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::SubClassOf<ArcStr>>> for VecWrap<SubClassOf> {
    fn from_c(value: &Vec<horned_owl::model::SubClassOf<ArcStr>>) -> Self {
        VecWrap::<SubClassOf>::from(value)
    }
}
#[doc = concat!(
    "EquivalentClasses(first: typing.List[ClassExpression],)",
    "\n\n",
    doc!(EquivalentClasses)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EquivalentClasses (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub VecWrap<ClassExpression>,
);

#[pymethods]
impl EquivalentClasses {
    #[new]
    fn new(first: VecWrap<ClassExpression>,) -> Self {
        EquivalentClasses (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::EquivalentClasses<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::EquivalentClasses<ArcStr>> for EquivalentClasses {
    fn from(value: &horned_owl::model::EquivalentClasses<ArcStr>) -> Self {

        EquivalentClasses (
    IntoCompatible::<VecWrap<ClassExpression>>::into_c(&value.0),
        )
    }
}

impl From<&EquivalentClasses> for horned_owl::model::EquivalentClasses<ArcStr> {
    fn from(value: &EquivalentClasses) -> Self {
        horned_owl::model::EquivalentClasses::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for EquivalentClasses ****************/
impl FromCompatible<horned_owl::model::EquivalentClasses<ArcStr>> for EquivalentClasses {
    fn from_c(value: horned_owl::model::EquivalentClasses<ArcStr>) -> Self {
        EquivalentClasses::from(value)
    }
}

impl FromCompatible<&horned_owl::model::EquivalentClasses<ArcStr>> for EquivalentClasses {
    fn from_c(value: &horned_owl::model::EquivalentClasses<ArcStr>) -> Self {
        EquivalentClasses::from(value)
    }
}

impl FromCompatible<EquivalentClasses> for horned_owl::model::EquivalentClasses<ArcStr> {
    fn from_c(value: EquivalentClasses) -> Self {
        horned_owl::model::EquivalentClasses::<ArcStr>::from(value)
    }
}

impl FromCompatible<&EquivalentClasses> for horned_owl::model::EquivalentClasses<ArcStr> {
    fn from_c(value: &EquivalentClasses) -> Self {
        horned_owl::model::EquivalentClasses::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::EquivalentClasses<ArcStr>> for EquivalentClasses {
    fn from(value: horned_owl::model::EquivalentClasses<ArcStr>) -> Self {
        EquivalentClasses::from(value.borrow())
    }
}

impl From<EquivalentClasses> for horned_owl::model::EquivalentClasses<ArcStr> {
    fn from(value: EquivalentClasses) -> Self {
        horned_owl::model::EquivalentClasses::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<EquivalentClasses>> for Box<horned_owl::model::EquivalentClasses<ArcStr>> {
    fn from(value: &BoxWrap<EquivalentClasses>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::EquivalentClasses<ArcStr>>> for BoxWrap<EquivalentClasses> {
    fn from(value: &Box<horned_owl::model::EquivalentClasses<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<EquivalentClasses>::into(*value.clone())))
    }
}

impl From<BoxWrap<EquivalentClasses>> for Box<horned_owl::model::EquivalentClasses<ArcStr>> {
    fn from(value: BoxWrap<EquivalentClasses>) -> Self {
        Into::<Box<horned_owl::model::EquivalentClasses<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::EquivalentClasses<ArcStr>>> for BoxWrap<EquivalentClasses> {
    fn from(value: Box<horned_owl::model::EquivalentClasses<ArcStr>>) -> Self {
        Into::<BoxWrap<EquivalentClasses>>::into(value.borrow())
    }
}

impl From<VecWrap<EquivalentClasses>> for Vec<horned_owl::model::EquivalentClasses<ArcStr>> {
    fn from(value: VecWrap<EquivalentClasses>) -> Self {
        Into::<Vec<horned_owl::model::EquivalentClasses<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::EquivalentClasses<ArcStr>>> for VecWrap<EquivalentClasses> {
    fn from(value: Vec<horned_owl::model::EquivalentClasses<ArcStr>>) -> Self {
        Into::<VecWrap<EquivalentClasses>>::into(value.borrow())
    }
}

impl From<&VecWrap<EquivalentClasses>> for Vec<horned_owl::model::EquivalentClasses<ArcStr>> {
    fn from(value: &VecWrap<EquivalentClasses>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::EquivalentClasses::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::EquivalentClasses<ArcStr>>> for VecWrap<EquivalentClasses> {
    fn from(value: &Vec<horned_owl::model::EquivalentClasses<ArcStr>>) -> Self {
        VecWrap(value.iter().map(EquivalentClasses::from).collect())
    }
}

impl FromCompatible<&BoxWrap<EquivalentClasses>> for Box<horned_owl::model::EquivalentClasses<ArcStr>> {
    fn from_c(value: &BoxWrap<EquivalentClasses>) -> Self {
        Box::<horned_owl::model::EquivalentClasses<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::EquivalentClasses<ArcStr>>> for BoxWrap<EquivalentClasses> {
    fn from_c(value: &Box<horned_owl::model::EquivalentClasses<ArcStr>>) -> Self {
        BoxWrap::<EquivalentClasses>::from(value)
    }
}
impl FromCompatible<BoxWrap<EquivalentClasses>> for Box<horned_owl::model::EquivalentClasses<ArcStr>> {
    fn from_c(value: BoxWrap<EquivalentClasses>) -> Self {
        Box::<horned_owl::model::EquivalentClasses<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::EquivalentClasses<ArcStr>>> for BoxWrap<EquivalentClasses> {
    fn from_c(value: Box<horned_owl::model::EquivalentClasses<ArcStr>>) -> Self {
        BoxWrap::<EquivalentClasses>::from(value)
    }
}
impl FromCompatible<VecWrap<EquivalentClasses>> for Vec<horned_owl::model::EquivalentClasses<ArcStr>> {
    fn from_c(value: VecWrap<EquivalentClasses>) -> Self {
        Vec::<horned_owl::model::EquivalentClasses<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::EquivalentClasses<ArcStr>>> for VecWrap<EquivalentClasses> {
    fn from_c(value: Vec<horned_owl::model::EquivalentClasses<ArcStr>>) -> Self {
        VecWrap::<EquivalentClasses>::from(value)
    }
}
impl FromCompatible<&VecWrap<EquivalentClasses>> for Vec<horned_owl::model::EquivalentClasses<ArcStr>> {
    fn from_c(value: &VecWrap<EquivalentClasses>) -> Self {
        Vec::<horned_owl::model::EquivalentClasses<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::EquivalentClasses<ArcStr>>> for VecWrap<EquivalentClasses> {
    fn from_c(value: &Vec<horned_owl::model::EquivalentClasses<ArcStr>>) -> Self {
        VecWrap::<EquivalentClasses>::from(value)
    }
}
#[doc = concat!(
    "DisjointClasses(first: typing.List[ClassExpression],)",
    "\n\n",
    doc!(DisjointClasses)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisjointClasses (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub VecWrap<ClassExpression>,
);

#[pymethods]
impl DisjointClasses {
    #[new]
    fn new(first: VecWrap<ClassExpression>,) -> Self {
        DisjointClasses (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DisjointClasses<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DisjointClasses<ArcStr>> for DisjointClasses {
    fn from(value: &horned_owl::model::DisjointClasses<ArcStr>) -> Self {

        DisjointClasses (
    IntoCompatible::<VecWrap<ClassExpression>>::into_c(&value.0),
        )
    }
}

impl From<&DisjointClasses> for horned_owl::model::DisjointClasses<ArcStr> {
    fn from(value: &DisjointClasses) -> Self {
        horned_owl::model::DisjointClasses::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DisjointClasses ****************/
impl FromCompatible<horned_owl::model::DisjointClasses<ArcStr>> for DisjointClasses {
    fn from_c(value: horned_owl::model::DisjointClasses<ArcStr>) -> Self {
        DisjointClasses::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DisjointClasses<ArcStr>> for DisjointClasses {
    fn from_c(value: &horned_owl::model::DisjointClasses<ArcStr>) -> Self {
        DisjointClasses::from(value)
    }
}

impl FromCompatible<DisjointClasses> for horned_owl::model::DisjointClasses<ArcStr> {
    fn from_c(value: DisjointClasses) -> Self {
        horned_owl::model::DisjointClasses::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DisjointClasses> for horned_owl::model::DisjointClasses<ArcStr> {
    fn from_c(value: &DisjointClasses) -> Self {
        horned_owl::model::DisjointClasses::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DisjointClasses<ArcStr>> for DisjointClasses {
    fn from(value: horned_owl::model::DisjointClasses<ArcStr>) -> Self {
        DisjointClasses::from(value.borrow())
    }
}

impl From<DisjointClasses> for horned_owl::model::DisjointClasses<ArcStr> {
    fn from(value: DisjointClasses) -> Self {
        horned_owl::model::DisjointClasses::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DisjointClasses>> for Box<horned_owl::model::DisjointClasses<ArcStr>> {
    fn from(value: &BoxWrap<DisjointClasses>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DisjointClasses<ArcStr>>> for BoxWrap<DisjointClasses> {
    fn from(value: &Box<horned_owl::model::DisjointClasses<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DisjointClasses>::into(*value.clone())))
    }
}

impl From<BoxWrap<DisjointClasses>> for Box<horned_owl::model::DisjointClasses<ArcStr>> {
    fn from(value: BoxWrap<DisjointClasses>) -> Self {
        Into::<Box<horned_owl::model::DisjointClasses<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DisjointClasses<ArcStr>>> for BoxWrap<DisjointClasses> {
    fn from(value: Box<horned_owl::model::DisjointClasses<ArcStr>>) -> Self {
        Into::<BoxWrap<DisjointClasses>>::into(value.borrow())
    }
}

impl From<VecWrap<DisjointClasses>> for Vec<horned_owl::model::DisjointClasses<ArcStr>> {
    fn from(value: VecWrap<DisjointClasses>) -> Self {
        Into::<Vec<horned_owl::model::DisjointClasses<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DisjointClasses<ArcStr>>> for VecWrap<DisjointClasses> {
    fn from(value: Vec<horned_owl::model::DisjointClasses<ArcStr>>) -> Self {
        Into::<VecWrap<DisjointClasses>>::into(value.borrow())
    }
}

impl From<&VecWrap<DisjointClasses>> for Vec<horned_owl::model::DisjointClasses<ArcStr>> {
    fn from(value: &VecWrap<DisjointClasses>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DisjointClasses::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DisjointClasses<ArcStr>>> for VecWrap<DisjointClasses> {
    fn from(value: &Vec<horned_owl::model::DisjointClasses<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DisjointClasses::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DisjointClasses>> for Box<horned_owl::model::DisjointClasses<ArcStr>> {
    fn from_c(value: &BoxWrap<DisjointClasses>) -> Self {
        Box::<horned_owl::model::DisjointClasses<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DisjointClasses<ArcStr>>> for BoxWrap<DisjointClasses> {
    fn from_c(value: &Box<horned_owl::model::DisjointClasses<ArcStr>>) -> Self {
        BoxWrap::<DisjointClasses>::from(value)
    }
}
impl FromCompatible<BoxWrap<DisjointClasses>> for Box<horned_owl::model::DisjointClasses<ArcStr>> {
    fn from_c(value: BoxWrap<DisjointClasses>) -> Self {
        Box::<horned_owl::model::DisjointClasses<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DisjointClasses<ArcStr>>> for BoxWrap<DisjointClasses> {
    fn from_c(value: Box<horned_owl::model::DisjointClasses<ArcStr>>) -> Self {
        BoxWrap::<DisjointClasses>::from(value)
    }
}
impl FromCompatible<VecWrap<DisjointClasses>> for Vec<horned_owl::model::DisjointClasses<ArcStr>> {
    fn from_c(value: VecWrap<DisjointClasses>) -> Self {
        Vec::<horned_owl::model::DisjointClasses<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DisjointClasses<ArcStr>>> for VecWrap<DisjointClasses> {
    fn from_c(value: Vec<horned_owl::model::DisjointClasses<ArcStr>>) -> Self {
        VecWrap::<DisjointClasses>::from(value)
    }
}
impl FromCompatible<&VecWrap<DisjointClasses>> for Vec<horned_owl::model::DisjointClasses<ArcStr>> {
    fn from_c(value: &VecWrap<DisjointClasses>) -> Self {
        Vec::<horned_owl::model::DisjointClasses<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DisjointClasses<ArcStr>>> for VecWrap<DisjointClasses> {
    fn from_c(value: &Vec<horned_owl::model::DisjointClasses<ArcStr>>) -> Self {
        VecWrap::<DisjointClasses>::from(value)
    }
}
#[doc = concat!(
    "DisjointUnion(first: Class,second: typing.List[ClassExpression],)",
    "\n\n",
    doc!(DisjointUnion)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisjointUnion (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub Class,
    #[doc="second: "]
    #[pyo3(get,set,name="second")]
    pub VecWrap<ClassExpression>,
);

#[pymethods]
impl DisjointUnion {
    #[new]
    fn new(first: Class,second: VecWrap<ClassExpression>,) -> Self {
        DisjointUnion (first,second,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DisjointUnion<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DisjointUnion<ArcStr>> for DisjointUnion {
    fn from(value: &horned_owl::model::DisjointUnion<ArcStr>) -> Self {

        DisjointUnion (
    IntoCompatible::<Class>::into_c(&value.0),
    IntoCompatible::<VecWrap<ClassExpression>>::into_c(&value.1),
        )
    }
}

impl From<&DisjointUnion> for horned_owl::model::DisjointUnion<ArcStr> {
    fn from(value: &DisjointUnion) -> Self {
        horned_owl::model::DisjointUnion::<ArcStr> (
    IntoCompatible::into_c(&value.0),
    IntoCompatible::into_c(&value.1),
        )
    }
}



/**************** Base implementations for DisjointUnion ****************/
impl FromCompatible<horned_owl::model::DisjointUnion<ArcStr>> for DisjointUnion {
    fn from_c(value: horned_owl::model::DisjointUnion<ArcStr>) -> Self {
        DisjointUnion::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DisjointUnion<ArcStr>> for DisjointUnion {
    fn from_c(value: &horned_owl::model::DisjointUnion<ArcStr>) -> Self {
        DisjointUnion::from(value)
    }
}

impl FromCompatible<DisjointUnion> for horned_owl::model::DisjointUnion<ArcStr> {
    fn from_c(value: DisjointUnion) -> Self {
        horned_owl::model::DisjointUnion::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DisjointUnion> for horned_owl::model::DisjointUnion<ArcStr> {
    fn from_c(value: &DisjointUnion) -> Self {
        horned_owl::model::DisjointUnion::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DisjointUnion<ArcStr>> for DisjointUnion {
    fn from(value: horned_owl::model::DisjointUnion<ArcStr>) -> Self {
        DisjointUnion::from(value.borrow())
    }
}

impl From<DisjointUnion> for horned_owl::model::DisjointUnion<ArcStr> {
    fn from(value: DisjointUnion) -> Self {
        horned_owl::model::DisjointUnion::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DisjointUnion>> for Box<horned_owl::model::DisjointUnion<ArcStr>> {
    fn from(value: &BoxWrap<DisjointUnion>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DisjointUnion<ArcStr>>> for BoxWrap<DisjointUnion> {
    fn from(value: &Box<horned_owl::model::DisjointUnion<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DisjointUnion>::into(*value.clone())))
    }
}

impl From<BoxWrap<DisjointUnion>> for Box<horned_owl::model::DisjointUnion<ArcStr>> {
    fn from(value: BoxWrap<DisjointUnion>) -> Self {
        Into::<Box<horned_owl::model::DisjointUnion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DisjointUnion<ArcStr>>> for BoxWrap<DisjointUnion> {
    fn from(value: Box<horned_owl::model::DisjointUnion<ArcStr>>) -> Self {
        Into::<BoxWrap<DisjointUnion>>::into(value.borrow())
    }
}

impl From<VecWrap<DisjointUnion>> for Vec<horned_owl::model::DisjointUnion<ArcStr>> {
    fn from(value: VecWrap<DisjointUnion>) -> Self {
        Into::<Vec<horned_owl::model::DisjointUnion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DisjointUnion<ArcStr>>> for VecWrap<DisjointUnion> {
    fn from(value: Vec<horned_owl::model::DisjointUnion<ArcStr>>) -> Self {
        Into::<VecWrap<DisjointUnion>>::into(value.borrow())
    }
}

impl From<&VecWrap<DisjointUnion>> for Vec<horned_owl::model::DisjointUnion<ArcStr>> {
    fn from(value: &VecWrap<DisjointUnion>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DisjointUnion::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DisjointUnion<ArcStr>>> for VecWrap<DisjointUnion> {
    fn from(value: &Vec<horned_owl::model::DisjointUnion<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DisjointUnion::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DisjointUnion>> for Box<horned_owl::model::DisjointUnion<ArcStr>> {
    fn from_c(value: &BoxWrap<DisjointUnion>) -> Self {
        Box::<horned_owl::model::DisjointUnion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DisjointUnion<ArcStr>>> for BoxWrap<DisjointUnion> {
    fn from_c(value: &Box<horned_owl::model::DisjointUnion<ArcStr>>) -> Self {
        BoxWrap::<DisjointUnion>::from(value)
    }
}
impl FromCompatible<BoxWrap<DisjointUnion>> for Box<horned_owl::model::DisjointUnion<ArcStr>> {
    fn from_c(value: BoxWrap<DisjointUnion>) -> Self {
        Box::<horned_owl::model::DisjointUnion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DisjointUnion<ArcStr>>> for BoxWrap<DisjointUnion> {
    fn from_c(value: Box<horned_owl::model::DisjointUnion<ArcStr>>) -> Self {
        BoxWrap::<DisjointUnion>::from(value)
    }
}
impl FromCompatible<VecWrap<DisjointUnion>> for Vec<horned_owl::model::DisjointUnion<ArcStr>> {
    fn from_c(value: VecWrap<DisjointUnion>) -> Self {
        Vec::<horned_owl::model::DisjointUnion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DisjointUnion<ArcStr>>> for VecWrap<DisjointUnion> {
    fn from_c(value: Vec<horned_owl::model::DisjointUnion<ArcStr>>) -> Self {
        VecWrap::<DisjointUnion>::from(value)
    }
}
impl FromCompatible<&VecWrap<DisjointUnion>> for Vec<horned_owl::model::DisjointUnion<ArcStr>> {
    fn from_c(value: &VecWrap<DisjointUnion>) -> Self {
        Vec::<horned_owl::model::DisjointUnion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DisjointUnion<ArcStr>>> for VecWrap<DisjointUnion> {
    fn from_c(value: &Vec<horned_owl::model::DisjointUnion<ArcStr>>) -> Self {
        VecWrap::<DisjointUnion>::from(value)
    }
}
#[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SubObjectPropertyExpression {
    
        #[pyo3(transparent)]
        ObjectPropertyChain (VecWrap<ObjectPropertyExpression>),
    
        #[pyo3(transparent)]
        ObjectPropertyExpression (ObjectPropertyExpression),
    
}

impl<'py> IntoPyObject<'py> for SubObjectPropertyExpression {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
        
            SubObjectPropertyExpression::ObjectPropertyChain(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            SubObjectPropertyExpression::ObjectPropertyExpression(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
        }
    }
}

impl From<&SubObjectPropertyExpression> for horned_owl::model::SubObjectPropertyExpression<ArcStr> {
    fn from(value: &SubObjectPropertyExpression) -> Self {
        match value {
        
            SubObjectPropertyExpression::ObjectPropertyChain(inner) => horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(inner.into()),
        
            SubObjectPropertyExpression::ObjectPropertyExpression(inner) => horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(inner.into()),
        
        }
    }
}

impl From<&horned_owl::model::SubObjectPropertyExpression<ArcStr>> for SubObjectPropertyExpression {

    fn from(value: &horned_owl::model::SubObjectPropertyExpression<ArcStr>) -> Self {
        match value {
        
            horned_owl::model::SubObjectPropertyExpression::ObjectPropertyChain(inner) => SubObjectPropertyExpression::ObjectPropertyChain(inner.into()),
        
            horned_owl::model::SubObjectPropertyExpression::ObjectPropertyExpression(inner) => SubObjectPropertyExpression::ObjectPropertyExpression(inner.into()),
        
        }
    }
}


impl SubObjectPropertyExpression {
    pub fn py_def() -> String {
        "typing.Union[typing.List[ObjectPropertyExpression],m.ObjectPropertyExpression,]".into()
    }
}



/**************** Base implementations for SubObjectPropertyExpression ****************/
impl FromCompatible<horned_owl::model::SubObjectPropertyExpression<ArcStr>> for SubObjectPropertyExpression {
    fn from_c(value: horned_owl::model::SubObjectPropertyExpression<ArcStr>) -> Self {
        SubObjectPropertyExpression::from(value)
    }
}

impl FromCompatible<&horned_owl::model::SubObjectPropertyExpression<ArcStr>> for SubObjectPropertyExpression {
    fn from_c(value: &horned_owl::model::SubObjectPropertyExpression<ArcStr>) -> Self {
        SubObjectPropertyExpression::from(value)
    }
}

impl FromCompatible<SubObjectPropertyExpression> for horned_owl::model::SubObjectPropertyExpression<ArcStr> {
    fn from_c(value: SubObjectPropertyExpression) -> Self {
        horned_owl::model::SubObjectPropertyExpression::<ArcStr>::from(value)
    }
}

impl FromCompatible<&SubObjectPropertyExpression> for horned_owl::model::SubObjectPropertyExpression<ArcStr> {
    fn from_c(value: &SubObjectPropertyExpression) -> Self {
        horned_owl::model::SubObjectPropertyExpression::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::SubObjectPropertyExpression<ArcStr>> for SubObjectPropertyExpression {
    fn from(value: horned_owl::model::SubObjectPropertyExpression<ArcStr>) -> Self {
        SubObjectPropertyExpression::from(value.borrow())
    }
}

impl From<SubObjectPropertyExpression> for horned_owl::model::SubObjectPropertyExpression<ArcStr> {
    fn from(value: SubObjectPropertyExpression) -> Self {
        horned_owl::model::SubObjectPropertyExpression::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<SubObjectPropertyExpression>> for Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>> {
    fn from(value: &BoxWrap<SubObjectPropertyExpression>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>> for BoxWrap<SubObjectPropertyExpression> {
    fn from(value: &Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<SubObjectPropertyExpression>::into(*value.clone())))
    }
}

impl From<BoxWrap<SubObjectPropertyExpression>> for Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>> {
    fn from(value: BoxWrap<SubObjectPropertyExpression>) -> Self {
        Into::<Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>> for BoxWrap<SubObjectPropertyExpression> {
    fn from(value: Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>) -> Self {
        Into::<BoxWrap<SubObjectPropertyExpression>>::into(value.borrow())
    }
}

impl From<VecWrap<SubObjectPropertyExpression>> for Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>> {
    fn from(value: VecWrap<SubObjectPropertyExpression>) -> Self {
        Into::<Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>> for VecWrap<SubObjectPropertyExpression> {
    fn from(value: Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>) -> Self {
        Into::<VecWrap<SubObjectPropertyExpression>>::into(value.borrow())
    }
}

impl From<&VecWrap<SubObjectPropertyExpression>> for Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>> {
    fn from(value: &VecWrap<SubObjectPropertyExpression>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::SubObjectPropertyExpression::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>> for VecWrap<SubObjectPropertyExpression> {
    fn from(value: &Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>) -> Self {
        VecWrap(value.iter().map(SubObjectPropertyExpression::from).collect())
    }
}

impl FromCompatible<&BoxWrap<SubObjectPropertyExpression>> for Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>> {
    fn from_c(value: &BoxWrap<SubObjectPropertyExpression>) -> Self {
        Box::<horned_owl::model::SubObjectPropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>> for BoxWrap<SubObjectPropertyExpression> {
    fn from_c(value: &Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>) -> Self {
        BoxWrap::<SubObjectPropertyExpression>::from(value)
    }
}
impl FromCompatible<BoxWrap<SubObjectPropertyExpression>> for Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>> {
    fn from_c(value: BoxWrap<SubObjectPropertyExpression>) -> Self {
        Box::<horned_owl::model::SubObjectPropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>> for BoxWrap<SubObjectPropertyExpression> {
    fn from_c(value: Box<horned_owl::model::SubObjectPropertyExpression<ArcStr>>) -> Self {
        BoxWrap::<SubObjectPropertyExpression>::from(value)
    }
}
impl FromCompatible<VecWrap<SubObjectPropertyExpression>> for Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>> {
    fn from_c(value: VecWrap<SubObjectPropertyExpression>) -> Self {
        Vec::<horned_owl::model::SubObjectPropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>> for VecWrap<SubObjectPropertyExpression> {
    fn from_c(value: Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>) -> Self {
        VecWrap::<SubObjectPropertyExpression>::from(value)
    }
}
impl FromCompatible<&VecWrap<SubObjectPropertyExpression>> for Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>> {
    fn from_c(value: &VecWrap<SubObjectPropertyExpression>) -> Self {
        Vec::<horned_owl::model::SubObjectPropertyExpression<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>> for VecWrap<SubObjectPropertyExpression> {
    fn from_c(value: &Vec<horned_owl::model::SubObjectPropertyExpression<ArcStr>>) -> Self {
        VecWrap::<SubObjectPropertyExpression>::from(value)
    }
}
#[doc = concat!("SubObjectPropertyOf(sub: SubObjectPropertyExpression,sup: ObjectPropertyExpression,)",
    "\n\n",
    doc!(SubObjectPropertyOf)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubObjectPropertyOf {
        #[doc="sub: SubObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub sub: SubObjectPropertyExpression,
    
        #[doc="sup: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub sup: ObjectPropertyExpression,
    }

#[pymethods]
impl SubObjectPropertyOf {
    #[new]
    fn new(sub: SubObjectPropertyExpression,sup: ObjectPropertyExpression,) -> Self {
        SubObjectPropertyOf {
                sub,
                sup,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "sub" => self.sub.clone().into_pyobject(py).map(Bound::into_any),
            "sup" => self.sup.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "sub" => {
                self.sub = value.extract()?;
                Ok(())
            },
            "sup" => {
                self.sup = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::SubObjectPropertyOf<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::SubObjectPropertyOf<ArcStr>> for SubObjectPropertyOf {
    fn from(value: &horned_owl::model::SubObjectPropertyOf<ArcStr>) -> Self {
        SubObjectPropertyOf {
            sub: IntoCompatible::<SubObjectPropertyExpression>::into_c(value.sub.borrow()),
            sup: IntoCompatible::<ObjectPropertyExpression>::into_c(value.sup.borrow()),
        }
    }
}


impl From<&SubObjectPropertyOf> for horned_owl::model::SubObjectPropertyOf<ArcStr> {
    fn from(value: &SubObjectPropertyOf) -> Self {
        horned_owl::model::SubObjectPropertyOf::<ArcStr> {
            sub: value.sub.borrow().into_c(),
            sup: value.sup.borrow().into_c(),
        }
    }
}



/**************** Base implementations for SubObjectPropertyOf ****************/
impl FromCompatible<horned_owl::model::SubObjectPropertyOf<ArcStr>> for SubObjectPropertyOf {
    fn from_c(value: horned_owl::model::SubObjectPropertyOf<ArcStr>) -> Self {
        SubObjectPropertyOf::from(value)
    }
}

impl FromCompatible<&horned_owl::model::SubObjectPropertyOf<ArcStr>> for SubObjectPropertyOf {
    fn from_c(value: &horned_owl::model::SubObjectPropertyOf<ArcStr>) -> Self {
        SubObjectPropertyOf::from(value)
    }
}

impl FromCompatible<SubObjectPropertyOf> for horned_owl::model::SubObjectPropertyOf<ArcStr> {
    fn from_c(value: SubObjectPropertyOf) -> Self {
        horned_owl::model::SubObjectPropertyOf::<ArcStr>::from(value)
    }
}

impl FromCompatible<&SubObjectPropertyOf> for horned_owl::model::SubObjectPropertyOf<ArcStr> {
    fn from_c(value: &SubObjectPropertyOf) -> Self {
        horned_owl::model::SubObjectPropertyOf::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::SubObjectPropertyOf<ArcStr>> for SubObjectPropertyOf {
    fn from(value: horned_owl::model::SubObjectPropertyOf<ArcStr>) -> Self {
        SubObjectPropertyOf::from(value.borrow())
    }
}

impl From<SubObjectPropertyOf> for horned_owl::model::SubObjectPropertyOf<ArcStr> {
    fn from(value: SubObjectPropertyOf) -> Self {
        horned_owl::model::SubObjectPropertyOf::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<SubObjectPropertyOf>> for Box<horned_owl::model::SubObjectPropertyOf<ArcStr>> {
    fn from(value: &BoxWrap<SubObjectPropertyOf>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>> for BoxWrap<SubObjectPropertyOf> {
    fn from(value: &Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<SubObjectPropertyOf>::into(*value.clone())))
    }
}

impl From<BoxWrap<SubObjectPropertyOf>> for Box<horned_owl::model::SubObjectPropertyOf<ArcStr>> {
    fn from(value: BoxWrap<SubObjectPropertyOf>) -> Self {
        Into::<Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>> for BoxWrap<SubObjectPropertyOf> {
    fn from(value: Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>) -> Self {
        Into::<BoxWrap<SubObjectPropertyOf>>::into(value.borrow())
    }
}

impl From<VecWrap<SubObjectPropertyOf>> for Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>> {
    fn from(value: VecWrap<SubObjectPropertyOf>) -> Self {
        Into::<Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>> for VecWrap<SubObjectPropertyOf> {
    fn from(value: Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>) -> Self {
        Into::<VecWrap<SubObjectPropertyOf>>::into(value.borrow())
    }
}

impl From<&VecWrap<SubObjectPropertyOf>> for Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>> {
    fn from(value: &VecWrap<SubObjectPropertyOf>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::SubObjectPropertyOf::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>> for VecWrap<SubObjectPropertyOf> {
    fn from(value: &Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>) -> Self {
        VecWrap(value.iter().map(SubObjectPropertyOf::from).collect())
    }
}

impl FromCompatible<&BoxWrap<SubObjectPropertyOf>> for Box<horned_owl::model::SubObjectPropertyOf<ArcStr>> {
    fn from_c(value: &BoxWrap<SubObjectPropertyOf>) -> Self {
        Box::<horned_owl::model::SubObjectPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>> for BoxWrap<SubObjectPropertyOf> {
    fn from_c(value: &Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>) -> Self {
        BoxWrap::<SubObjectPropertyOf>::from(value)
    }
}
impl FromCompatible<BoxWrap<SubObjectPropertyOf>> for Box<horned_owl::model::SubObjectPropertyOf<ArcStr>> {
    fn from_c(value: BoxWrap<SubObjectPropertyOf>) -> Self {
        Box::<horned_owl::model::SubObjectPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>> for BoxWrap<SubObjectPropertyOf> {
    fn from_c(value: Box<horned_owl::model::SubObjectPropertyOf<ArcStr>>) -> Self {
        BoxWrap::<SubObjectPropertyOf>::from(value)
    }
}
impl FromCompatible<VecWrap<SubObjectPropertyOf>> for Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>> {
    fn from_c(value: VecWrap<SubObjectPropertyOf>) -> Self {
        Vec::<horned_owl::model::SubObjectPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>> for VecWrap<SubObjectPropertyOf> {
    fn from_c(value: Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>) -> Self {
        VecWrap::<SubObjectPropertyOf>::from(value)
    }
}
impl FromCompatible<&VecWrap<SubObjectPropertyOf>> for Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>> {
    fn from_c(value: &VecWrap<SubObjectPropertyOf>) -> Self {
        Vec::<horned_owl::model::SubObjectPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>> for VecWrap<SubObjectPropertyOf> {
    fn from_c(value: &Vec<horned_owl::model::SubObjectPropertyOf<ArcStr>>) -> Self {
        VecWrap::<SubObjectPropertyOf>::from(value)
    }
}
#[doc = concat!(
    "EquivalentObjectProperties(first: typing.List[ObjectPropertyExpression],)",
    "\n\n",
    doc!(EquivalentObjectProperties)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EquivalentObjectProperties (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub VecWrap<ObjectPropertyExpression>,
);

#[pymethods]
impl EquivalentObjectProperties {
    #[new]
    fn new(first: VecWrap<ObjectPropertyExpression>,) -> Self {
        EquivalentObjectProperties (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::EquivalentObjectProperties<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::EquivalentObjectProperties<ArcStr>> for EquivalentObjectProperties {
    fn from(value: &horned_owl::model::EquivalentObjectProperties<ArcStr>) -> Self {

        EquivalentObjectProperties (
    IntoCompatible::<VecWrap<ObjectPropertyExpression>>::into_c(&value.0),
        )
    }
}

impl From<&EquivalentObjectProperties> for horned_owl::model::EquivalentObjectProperties<ArcStr> {
    fn from(value: &EquivalentObjectProperties) -> Self {
        horned_owl::model::EquivalentObjectProperties::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for EquivalentObjectProperties ****************/
impl FromCompatible<horned_owl::model::EquivalentObjectProperties<ArcStr>> for EquivalentObjectProperties {
    fn from_c(value: horned_owl::model::EquivalentObjectProperties<ArcStr>) -> Self {
        EquivalentObjectProperties::from(value)
    }
}

impl FromCompatible<&horned_owl::model::EquivalentObjectProperties<ArcStr>> for EquivalentObjectProperties {
    fn from_c(value: &horned_owl::model::EquivalentObjectProperties<ArcStr>) -> Self {
        EquivalentObjectProperties::from(value)
    }
}

impl FromCompatible<EquivalentObjectProperties> for horned_owl::model::EquivalentObjectProperties<ArcStr> {
    fn from_c(value: EquivalentObjectProperties) -> Self {
        horned_owl::model::EquivalentObjectProperties::<ArcStr>::from(value)
    }
}

impl FromCompatible<&EquivalentObjectProperties> for horned_owl::model::EquivalentObjectProperties<ArcStr> {
    fn from_c(value: &EquivalentObjectProperties) -> Self {
        horned_owl::model::EquivalentObjectProperties::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::EquivalentObjectProperties<ArcStr>> for EquivalentObjectProperties {
    fn from(value: horned_owl::model::EquivalentObjectProperties<ArcStr>) -> Self {
        EquivalentObjectProperties::from(value.borrow())
    }
}

impl From<EquivalentObjectProperties> for horned_owl::model::EquivalentObjectProperties<ArcStr> {
    fn from(value: EquivalentObjectProperties) -> Self {
        horned_owl::model::EquivalentObjectProperties::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<EquivalentObjectProperties>> for Box<horned_owl::model::EquivalentObjectProperties<ArcStr>> {
    fn from(value: &BoxWrap<EquivalentObjectProperties>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>> for BoxWrap<EquivalentObjectProperties> {
    fn from(value: &Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<EquivalentObjectProperties>::into(*value.clone())))
    }
}

impl From<BoxWrap<EquivalentObjectProperties>> for Box<horned_owl::model::EquivalentObjectProperties<ArcStr>> {
    fn from(value: BoxWrap<EquivalentObjectProperties>) -> Self {
        Into::<Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>> for BoxWrap<EquivalentObjectProperties> {
    fn from(value: Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>) -> Self {
        Into::<BoxWrap<EquivalentObjectProperties>>::into(value.borrow())
    }
}

impl From<VecWrap<EquivalentObjectProperties>> for Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>> {
    fn from(value: VecWrap<EquivalentObjectProperties>) -> Self {
        Into::<Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>> for VecWrap<EquivalentObjectProperties> {
    fn from(value: Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>) -> Self {
        Into::<VecWrap<EquivalentObjectProperties>>::into(value.borrow())
    }
}

impl From<&VecWrap<EquivalentObjectProperties>> for Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>> {
    fn from(value: &VecWrap<EquivalentObjectProperties>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::EquivalentObjectProperties::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>> for VecWrap<EquivalentObjectProperties> {
    fn from(value: &Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>) -> Self {
        VecWrap(value.iter().map(EquivalentObjectProperties::from).collect())
    }
}

impl FromCompatible<&BoxWrap<EquivalentObjectProperties>> for Box<horned_owl::model::EquivalentObjectProperties<ArcStr>> {
    fn from_c(value: &BoxWrap<EquivalentObjectProperties>) -> Self {
        Box::<horned_owl::model::EquivalentObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>> for BoxWrap<EquivalentObjectProperties> {
    fn from_c(value: &Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>) -> Self {
        BoxWrap::<EquivalentObjectProperties>::from(value)
    }
}
impl FromCompatible<BoxWrap<EquivalentObjectProperties>> for Box<horned_owl::model::EquivalentObjectProperties<ArcStr>> {
    fn from_c(value: BoxWrap<EquivalentObjectProperties>) -> Self {
        Box::<horned_owl::model::EquivalentObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>> for BoxWrap<EquivalentObjectProperties> {
    fn from_c(value: Box<horned_owl::model::EquivalentObjectProperties<ArcStr>>) -> Self {
        BoxWrap::<EquivalentObjectProperties>::from(value)
    }
}
impl FromCompatible<VecWrap<EquivalentObjectProperties>> for Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>> {
    fn from_c(value: VecWrap<EquivalentObjectProperties>) -> Self {
        Vec::<horned_owl::model::EquivalentObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>> for VecWrap<EquivalentObjectProperties> {
    fn from_c(value: Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>) -> Self {
        VecWrap::<EquivalentObjectProperties>::from(value)
    }
}
impl FromCompatible<&VecWrap<EquivalentObjectProperties>> for Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>> {
    fn from_c(value: &VecWrap<EquivalentObjectProperties>) -> Self {
        Vec::<horned_owl::model::EquivalentObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>> for VecWrap<EquivalentObjectProperties> {
    fn from_c(value: &Vec<horned_owl::model::EquivalentObjectProperties<ArcStr>>) -> Self {
        VecWrap::<EquivalentObjectProperties>::from(value)
    }
}
#[doc = concat!(
    "DisjointObjectProperties(first: typing.List[ObjectPropertyExpression],)",
    "\n\n",
    doc!(DisjointObjectProperties)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisjointObjectProperties (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub VecWrap<ObjectPropertyExpression>,
);

#[pymethods]
impl DisjointObjectProperties {
    #[new]
    fn new(first: VecWrap<ObjectPropertyExpression>,) -> Self {
        DisjointObjectProperties (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DisjointObjectProperties<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DisjointObjectProperties<ArcStr>> for DisjointObjectProperties {
    fn from(value: &horned_owl::model::DisjointObjectProperties<ArcStr>) -> Self {

        DisjointObjectProperties (
    IntoCompatible::<VecWrap<ObjectPropertyExpression>>::into_c(&value.0),
        )
    }
}

impl From<&DisjointObjectProperties> for horned_owl::model::DisjointObjectProperties<ArcStr> {
    fn from(value: &DisjointObjectProperties) -> Self {
        horned_owl::model::DisjointObjectProperties::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DisjointObjectProperties ****************/
impl FromCompatible<horned_owl::model::DisjointObjectProperties<ArcStr>> for DisjointObjectProperties {
    fn from_c(value: horned_owl::model::DisjointObjectProperties<ArcStr>) -> Self {
        DisjointObjectProperties::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DisjointObjectProperties<ArcStr>> for DisjointObjectProperties {
    fn from_c(value: &horned_owl::model::DisjointObjectProperties<ArcStr>) -> Self {
        DisjointObjectProperties::from(value)
    }
}

impl FromCompatible<DisjointObjectProperties> for horned_owl::model::DisjointObjectProperties<ArcStr> {
    fn from_c(value: DisjointObjectProperties) -> Self {
        horned_owl::model::DisjointObjectProperties::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DisjointObjectProperties> for horned_owl::model::DisjointObjectProperties<ArcStr> {
    fn from_c(value: &DisjointObjectProperties) -> Self {
        horned_owl::model::DisjointObjectProperties::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DisjointObjectProperties<ArcStr>> for DisjointObjectProperties {
    fn from(value: horned_owl::model::DisjointObjectProperties<ArcStr>) -> Self {
        DisjointObjectProperties::from(value.borrow())
    }
}

impl From<DisjointObjectProperties> for horned_owl::model::DisjointObjectProperties<ArcStr> {
    fn from(value: DisjointObjectProperties) -> Self {
        horned_owl::model::DisjointObjectProperties::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DisjointObjectProperties>> for Box<horned_owl::model::DisjointObjectProperties<ArcStr>> {
    fn from(value: &BoxWrap<DisjointObjectProperties>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DisjointObjectProperties<ArcStr>>> for BoxWrap<DisjointObjectProperties> {
    fn from(value: &Box<horned_owl::model::DisjointObjectProperties<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DisjointObjectProperties>::into(*value.clone())))
    }
}

impl From<BoxWrap<DisjointObjectProperties>> for Box<horned_owl::model::DisjointObjectProperties<ArcStr>> {
    fn from(value: BoxWrap<DisjointObjectProperties>) -> Self {
        Into::<Box<horned_owl::model::DisjointObjectProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DisjointObjectProperties<ArcStr>>> for BoxWrap<DisjointObjectProperties> {
    fn from(value: Box<horned_owl::model::DisjointObjectProperties<ArcStr>>) -> Self {
        Into::<BoxWrap<DisjointObjectProperties>>::into(value.borrow())
    }
}

impl From<VecWrap<DisjointObjectProperties>> for Vec<horned_owl::model::DisjointObjectProperties<ArcStr>> {
    fn from(value: VecWrap<DisjointObjectProperties>) -> Self {
        Into::<Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>> for VecWrap<DisjointObjectProperties> {
    fn from(value: Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>) -> Self {
        Into::<VecWrap<DisjointObjectProperties>>::into(value.borrow())
    }
}

impl From<&VecWrap<DisjointObjectProperties>> for Vec<horned_owl::model::DisjointObjectProperties<ArcStr>> {
    fn from(value: &VecWrap<DisjointObjectProperties>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DisjointObjectProperties::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>> for VecWrap<DisjointObjectProperties> {
    fn from(value: &Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DisjointObjectProperties::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DisjointObjectProperties>> for Box<horned_owl::model::DisjointObjectProperties<ArcStr>> {
    fn from_c(value: &BoxWrap<DisjointObjectProperties>) -> Self {
        Box::<horned_owl::model::DisjointObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DisjointObjectProperties<ArcStr>>> for BoxWrap<DisjointObjectProperties> {
    fn from_c(value: &Box<horned_owl::model::DisjointObjectProperties<ArcStr>>) -> Self {
        BoxWrap::<DisjointObjectProperties>::from(value)
    }
}
impl FromCompatible<BoxWrap<DisjointObjectProperties>> for Box<horned_owl::model::DisjointObjectProperties<ArcStr>> {
    fn from_c(value: BoxWrap<DisjointObjectProperties>) -> Self {
        Box::<horned_owl::model::DisjointObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DisjointObjectProperties<ArcStr>>> for BoxWrap<DisjointObjectProperties> {
    fn from_c(value: Box<horned_owl::model::DisjointObjectProperties<ArcStr>>) -> Self {
        BoxWrap::<DisjointObjectProperties>::from(value)
    }
}
impl FromCompatible<VecWrap<DisjointObjectProperties>> for Vec<horned_owl::model::DisjointObjectProperties<ArcStr>> {
    fn from_c(value: VecWrap<DisjointObjectProperties>) -> Self {
        Vec::<horned_owl::model::DisjointObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>> for VecWrap<DisjointObjectProperties> {
    fn from_c(value: Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>) -> Self {
        VecWrap::<DisjointObjectProperties>::from(value)
    }
}
impl FromCompatible<&VecWrap<DisjointObjectProperties>> for Vec<horned_owl::model::DisjointObjectProperties<ArcStr>> {
    fn from_c(value: &VecWrap<DisjointObjectProperties>) -> Self {
        Vec::<horned_owl::model::DisjointObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>> for VecWrap<DisjointObjectProperties> {
    fn from_c(value: &Vec<horned_owl::model::DisjointObjectProperties<ArcStr>>) -> Self {
        VecWrap::<DisjointObjectProperties>::from(value)
    }
}
#[doc = concat!(
    "InverseObjectProperties(first: ObjectProperty,second: ObjectProperty,)",
    "\n\n",
    doc!(InverseObjectProperties)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InverseObjectProperties (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectProperty,
    #[doc="second: "]
    #[pyo3(get,set,name="second")]
    pub ObjectProperty,
);

#[pymethods]
impl InverseObjectProperties {
    #[new]
    fn new(first: ObjectProperty,second: ObjectProperty,) -> Self {
        InverseObjectProperties (first,second,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::InverseObjectProperties<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::InverseObjectProperties<ArcStr>> for InverseObjectProperties {
    fn from(value: &horned_owl::model::InverseObjectProperties<ArcStr>) -> Self {

        InverseObjectProperties (
    IntoCompatible::<ObjectProperty>::into_c(&value.0),
    IntoCompatible::<ObjectProperty>::into_c(&value.1),
        )
    }
}

impl From<&InverseObjectProperties> for horned_owl::model::InverseObjectProperties<ArcStr> {
    fn from(value: &InverseObjectProperties) -> Self {
        horned_owl::model::InverseObjectProperties::<ArcStr> (
    IntoCompatible::into_c(&value.0),
    IntoCompatible::into_c(&value.1),
        )
    }
}



/**************** Base implementations for InverseObjectProperties ****************/
impl FromCompatible<horned_owl::model::InverseObjectProperties<ArcStr>> for InverseObjectProperties {
    fn from_c(value: horned_owl::model::InverseObjectProperties<ArcStr>) -> Self {
        InverseObjectProperties::from(value)
    }
}

impl FromCompatible<&horned_owl::model::InverseObjectProperties<ArcStr>> for InverseObjectProperties {
    fn from_c(value: &horned_owl::model::InverseObjectProperties<ArcStr>) -> Self {
        InverseObjectProperties::from(value)
    }
}

impl FromCompatible<InverseObjectProperties> for horned_owl::model::InverseObjectProperties<ArcStr> {
    fn from_c(value: InverseObjectProperties) -> Self {
        horned_owl::model::InverseObjectProperties::<ArcStr>::from(value)
    }
}

impl FromCompatible<&InverseObjectProperties> for horned_owl::model::InverseObjectProperties<ArcStr> {
    fn from_c(value: &InverseObjectProperties) -> Self {
        horned_owl::model::InverseObjectProperties::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::InverseObjectProperties<ArcStr>> for InverseObjectProperties {
    fn from(value: horned_owl::model::InverseObjectProperties<ArcStr>) -> Self {
        InverseObjectProperties::from(value.borrow())
    }
}

impl From<InverseObjectProperties> for horned_owl::model::InverseObjectProperties<ArcStr> {
    fn from(value: InverseObjectProperties) -> Self {
        horned_owl::model::InverseObjectProperties::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<InverseObjectProperties>> for Box<horned_owl::model::InverseObjectProperties<ArcStr>> {
    fn from(value: &BoxWrap<InverseObjectProperties>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::InverseObjectProperties<ArcStr>>> for BoxWrap<InverseObjectProperties> {
    fn from(value: &Box<horned_owl::model::InverseObjectProperties<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<InverseObjectProperties>::into(*value.clone())))
    }
}

impl From<BoxWrap<InverseObjectProperties>> for Box<horned_owl::model::InverseObjectProperties<ArcStr>> {
    fn from(value: BoxWrap<InverseObjectProperties>) -> Self {
        Into::<Box<horned_owl::model::InverseObjectProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::InverseObjectProperties<ArcStr>>> for BoxWrap<InverseObjectProperties> {
    fn from(value: Box<horned_owl::model::InverseObjectProperties<ArcStr>>) -> Self {
        Into::<BoxWrap<InverseObjectProperties>>::into(value.borrow())
    }
}

impl From<VecWrap<InverseObjectProperties>> for Vec<horned_owl::model::InverseObjectProperties<ArcStr>> {
    fn from(value: VecWrap<InverseObjectProperties>) -> Self {
        Into::<Vec<horned_owl::model::InverseObjectProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::InverseObjectProperties<ArcStr>>> for VecWrap<InverseObjectProperties> {
    fn from(value: Vec<horned_owl::model::InverseObjectProperties<ArcStr>>) -> Self {
        Into::<VecWrap<InverseObjectProperties>>::into(value.borrow())
    }
}

impl From<&VecWrap<InverseObjectProperties>> for Vec<horned_owl::model::InverseObjectProperties<ArcStr>> {
    fn from(value: &VecWrap<InverseObjectProperties>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::InverseObjectProperties::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::InverseObjectProperties<ArcStr>>> for VecWrap<InverseObjectProperties> {
    fn from(value: &Vec<horned_owl::model::InverseObjectProperties<ArcStr>>) -> Self {
        VecWrap(value.iter().map(InverseObjectProperties::from).collect())
    }
}

impl FromCompatible<&BoxWrap<InverseObjectProperties>> for Box<horned_owl::model::InverseObjectProperties<ArcStr>> {
    fn from_c(value: &BoxWrap<InverseObjectProperties>) -> Self {
        Box::<horned_owl::model::InverseObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::InverseObjectProperties<ArcStr>>> for BoxWrap<InverseObjectProperties> {
    fn from_c(value: &Box<horned_owl::model::InverseObjectProperties<ArcStr>>) -> Self {
        BoxWrap::<InverseObjectProperties>::from(value)
    }
}
impl FromCompatible<BoxWrap<InverseObjectProperties>> for Box<horned_owl::model::InverseObjectProperties<ArcStr>> {
    fn from_c(value: BoxWrap<InverseObjectProperties>) -> Self {
        Box::<horned_owl::model::InverseObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::InverseObjectProperties<ArcStr>>> for BoxWrap<InverseObjectProperties> {
    fn from_c(value: Box<horned_owl::model::InverseObjectProperties<ArcStr>>) -> Self {
        BoxWrap::<InverseObjectProperties>::from(value)
    }
}
impl FromCompatible<VecWrap<InverseObjectProperties>> for Vec<horned_owl::model::InverseObjectProperties<ArcStr>> {
    fn from_c(value: VecWrap<InverseObjectProperties>) -> Self {
        Vec::<horned_owl::model::InverseObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::InverseObjectProperties<ArcStr>>> for VecWrap<InverseObjectProperties> {
    fn from_c(value: Vec<horned_owl::model::InverseObjectProperties<ArcStr>>) -> Self {
        VecWrap::<InverseObjectProperties>::from(value)
    }
}
impl FromCompatible<&VecWrap<InverseObjectProperties>> for Vec<horned_owl::model::InverseObjectProperties<ArcStr>> {
    fn from_c(value: &VecWrap<InverseObjectProperties>) -> Self {
        Vec::<horned_owl::model::InverseObjectProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::InverseObjectProperties<ArcStr>>> for VecWrap<InverseObjectProperties> {
    fn from_c(value: &Vec<horned_owl::model::InverseObjectProperties<ArcStr>>) -> Self {
        VecWrap::<InverseObjectProperties>::from(value)
    }
}
#[doc = concat!("ObjectPropertyDomain(ope: ObjectPropertyExpression,ce: ClassExpression,)",
    "\n\n",
    doc!(ObjectPropertyDomain)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectPropertyDomain {
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
    
        #[doc="ce: ClassExpression"]
        #[pyo3(get,set)]
        pub ce: ClassExpression,
    }

#[pymethods]
impl ObjectPropertyDomain {
    #[new]
    fn new(ope: ObjectPropertyExpression,ce: ClassExpression,) -> Self {
        ObjectPropertyDomain {
                ope,
                ce,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any),
            "ce" => self.ce.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ope" => {
                self.ope = value.extract()?;
                Ok(())
            },
            "ce" => {
                self.ce = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::ObjectPropertyDomain<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::ObjectPropertyDomain<ArcStr>> for ObjectPropertyDomain {
    fn from(value: &horned_owl::model::ObjectPropertyDomain<ArcStr>) -> Self {
        ObjectPropertyDomain {
            ope: IntoCompatible::<ObjectPropertyExpression>::into_c(value.ope.borrow()),
            ce: IntoCompatible::<ClassExpression>::into_c(value.ce.borrow()),
        }
    }
}


impl From<&ObjectPropertyDomain> for horned_owl::model::ObjectPropertyDomain<ArcStr> {
    fn from(value: &ObjectPropertyDomain) -> Self {
        horned_owl::model::ObjectPropertyDomain::<ArcStr> {
            ope: value.ope.borrow().into_c(),
            ce: value.ce.borrow().into_c(),
        }
    }
}



/**************** Base implementations for ObjectPropertyDomain ****************/
impl FromCompatible<horned_owl::model::ObjectPropertyDomain<ArcStr>> for ObjectPropertyDomain {
    fn from_c(value: horned_owl::model::ObjectPropertyDomain<ArcStr>) -> Self {
        ObjectPropertyDomain::from(value)
    }
}

impl FromCompatible<&horned_owl::model::ObjectPropertyDomain<ArcStr>> for ObjectPropertyDomain {
    fn from_c(value: &horned_owl::model::ObjectPropertyDomain<ArcStr>) -> Self {
        ObjectPropertyDomain::from(value)
    }
}

impl FromCompatible<ObjectPropertyDomain> for horned_owl::model::ObjectPropertyDomain<ArcStr> {
    fn from_c(value: ObjectPropertyDomain) -> Self {
        horned_owl::model::ObjectPropertyDomain::<ArcStr>::from(value)
    }
}

impl FromCompatible<&ObjectPropertyDomain> for horned_owl::model::ObjectPropertyDomain<ArcStr> {
    fn from_c(value: &ObjectPropertyDomain) -> Self {
        horned_owl::model::ObjectPropertyDomain::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::ObjectPropertyDomain<ArcStr>> for ObjectPropertyDomain {
    fn from(value: horned_owl::model::ObjectPropertyDomain<ArcStr>) -> Self {
        ObjectPropertyDomain::from(value.borrow())
    }
}

impl From<ObjectPropertyDomain> for horned_owl::model::ObjectPropertyDomain<ArcStr> {
    fn from(value: ObjectPropertyDomain) -> Self {
        horned_owl::model::ObjectPropertyDomain::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<ObjectPropertyDomain>> for Box<horned_owl::model::ObjectPropertyDomain<ArcStr>> {
    fn from(value: &BoxWrap<ObjectPropertyDomain>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>> for BoxWrap<ObjectPropertyDomain> {
    fn from(value: &Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<ObjectPropertyDomain>::into(*value.clone())))
    }
}

impl From<BoxWrap<ObjectPropertyDomain>> for Box<horned_owl::model::ObjectPropertyDomain<ArcStr>> {
    fn from(value: BoxWrap<ObjectPropertyDomain>) -> Self {
        Into::<Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>> for BoxWrap<ObjectPropertyDomain> {
    fn from(value: Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>) -> Self {
        Into::<BoxWrap<ObjectPropertyDomain>>::into(value.borrow())
    }
}

impl From<VecWrap<ObjectPropertyDomain>> for Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>> {
    fn from(value: VecWrap<ObjectPropertyDomain>) -> Self {
        Into::<Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>> for VecWrap<ObjectPropertyDomain> {
    fn from(value: Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>) -> Self {
        Into::<VecWrap<ObjectPropertyDomain>>::into(value.borrow())
    }
}

impl From<&VecWrap<ObjectPropertyDomain>> for Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>> {
    fn from(value: &VecWrap<ObjectPropertyDomain>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::ObjectPropertyDomain::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>> for VecWrap<ObjectPropertyDomain> {
    fn from(value: &Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>) -> Self {
        VecWrap(value.iter().map(ObjectPropertyDomain::from).collect())
    }
}

impl FromCompatible<&BoxWrap<ObjectPropertyDomain>> for Box<horned_owl::model::ObjectPropertyDomain<ArcStr>> {
    fn from_c(value: &BoxWrap<ObjectPropertyDomain>) -> Self {
        Box::<horned_owl::model::ObjectPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>> for BoxWrap<ObjectPropertyDomain> {
    fn from_c(value: &Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>) -> Self {
        BoxWrap::<ObjectPropertyDomain>::from(value)
    }
}
impl FromCompatible<BoxWrap<ObjectPropertyDomain>> for Box<horned_owl::model::ObjectPropertyDomain<ArcStr>> {
    fn from_c(value: BoxWrap<ObjectPropertyDomain>) -> Self {
        Box::<horned_owl::model::ObjectPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>> for BoxWrap<ObjectPropertyDomain> {
    fn from_c(value: Box<horned_owl::model::ObjectPropertyDomain<ArcStr>>) -> Self {
        BoxWrap::<ObjectPropertyDomain>::from(value)
    }
}
impl FromCompatible<VecWrap<ObjectPropertyDomain>> for Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>> {
    fn from_c(value: VecWrap<ObjectPropertyDomain>) -> Self {
        Vec::<horned_owl::model::ObjectPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>> for VecWrap<ObjectPropertyDomain> {
    fn from_c(value: Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>) -> Self {
        VecWrap::<ObjectPropertyDomain>::from(value)
    }
}
impl FromCompatible<&VecWrap<ObjectPropertyDomain>> for Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>> {
    fn from_c(value: &VecWrap<ObjectPropertyDomain>) -> Self {
        Vec::<horned_owl::model::ObjectPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>> for VecWrap<ObjectPropertyDomain> {
    fn from_c(value: &Vec<horned_owl::model::ObjectPropertyDomain<ArcStr>>) -> Self {
        VecWrap::<ObjectPropertyDomain>::from(value)
    }
}
#[doc = concat!("ObjectPropertyRange(ope: ObjectPropertyExpression,ce: ClassExpression,)",
    "\n\n",
    doc!(ObjectPropertyRange)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectPropertyRange {
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
    
        #[doc="ce: ClassExpression"]
        #[pyo3(get,set)]
        pub ce: ClassExpression,
    }

#[pymethods]
impl ObjectPropertyRange {
    #[new]
    fn new(ope: ObjectPropertyExpression,ce: ClassExpression,) -> Self {
        ObjectPropertyRange {
                ope,
                ce,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any),
            "ce" => self.ce.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ope" => {
                self.ope = value.extract()?;
                Ok(())
            },
            "ce" => {
                self.ce = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::ObjectPropertyRange<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::ObjectPropertyRange<ArcStr>> for ObjectPropertyRange {
    fn from(value: &horned_owl::model::ObjectPropertyRange<ArcStr>) -> Self {
        ObjectPropertyRange {
            ope: IntoCompatible::<ObjectPropertyExpression>::into_c(value.ope.borrow()),
            ce: IntoCompatible::<ClassExpression>::into_c(value.ce.borrow()),
        }
    }
}


impl From<&ObjectPropertyRange> for horned_owl::model::ObjectPropertyRange<ArcStr> {
    fn from(value: &ObjectPropertyRange) -> Self {
        horned_owl::model::ObjectPropertyRange::<ArcStr> {
            ope: value.ope.borrow().into_c(),
            ce: value.ce.borrow().into_c(),
        }
    }
}



/**************** Base implementations for ObjectPropertyRange ****************/
impl FromCompatible<horned_owl::model::ObjectPropertyRange<ArcStr>> for ObjectPropertyRange {
    fn from_c(value: horned_owl::model::ObjectPropertyRange<ArcStr>) -> Self {
        ObjectPropertyRange::from(value)
    }
}

impl FromCompatible<&horned_owl::model::ObjectPropertyRange<ArcStr>> for ObjectPropertyRange {
    fn from_c(value: &horned_owl::model::ObjectPropertyRange<ArcStr>) -> Self {
        ObjectPropertyRange::from(value)
    }
}

impl FromCompatible<ObjectPropertyRange> for horned_owl::model::ObjectPropertyRange<ArcStr> {
    fn from_c(value: ObjectPropertyRange) -> Self {
        horned_owl::model::ObjectPropertyRange::<ArcStr>::from(value)
    }
}

impl FromCompatible<&ObjectPropertyRange> for horned_owl::model::ObjectPropertyRange<ArcStr> {
    fn from_c(value: &ObjectPropertyRange) -> Self {
        horned_owl::model::ObjectPropertyRange::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::ObjectPropertyRange<ArcStr>> for ObjectPropertyRange {
    fn from(value: horned_owl::model::ObjectPropertyRange<ArcStr>) -> Self {
        ObjectPropertyRange::from(value.borrow())
    }
}

impl From<ObjectPropertyRange> for horned_owl::model::ObjectPropertyRange<ArcStr> {
    fn from(value: ObjectPropertyRange) -> Self {
        horned_owl::model::ObjectPropertyRange::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<ObjectPropertyRange>> for Box<horned_owl::model::ObjectPropertyRange<ArcStr>> {
    fn from(value: &BoxWrap<ObjectPropertyRange>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::ObjectPropertyRange<ArcStr>>> for BoxWrap<ObjectPropertyRange> {
    fn from(value: &Box<horned_owl::model::ObjectPropertyRange<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<ObjectPropertyRange>::into(*value.clone())))
    }
}

impl From<BoxWrap<ObjectPropertyRange>> for Box<horned_owl::model::ObjectPropertyRange<ArcStr>> {
    fn from(value: BoxWrap<ObjectPropertyRange>) -> Self {
        Into::<Box<horned_owl::model::ObjectPropertyRange<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::ObjectPropertyRange<ArcStr>>> for BoxWrap<ObjectPropertyRange> {
    fn from(value: Box<horned_owl::model::ObjectPropertyRange<ArcStr>>) -> Self {
        Into::<BoxWrap<ObjectPropertyRange>>::into(value.borrow())
    }
}

impl From<VecWrap<ObjectPropertyRange>> for Vec<horned_owl::model::ObjectPropertyRange<ArcStr>> {
    fn from(value: VecWrap<ObjectPropertyRange>) -> Self {
        Into::<Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>> for VecWrap<ObjectPropertyRange> {
    fn from(value: Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>) -> Self {
        Into::<VecWrap<ObjectPropertyRange>>::into(value.borrow())
    }
}

impl From<&VecWrap<ObjectPropertyRange>> for Vec<horned_owl::model::ObjectPropertyRange<ArcStr>> {
    fn from(value: &VecWrap<ObjectPropertyRange>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::ObjectPropertyRange::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>> for VecWrap<ObjectPropertyRange> {
    fn from(value: &Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>) -> Self {
        VecWrap(value.iter().map(ObjectPropertyRange::from).collect())
    }
}

impl FromCompatible<&BoxWrap<ObjectPropertyRange>> for Box<horned_owl::model::ObjectPropertyRange<ArcStr>> {
    fn from_c(value: &BoxWrap<ObjectPropertyRange>) -> Self {
        Box::<horned_owl::model::ObjectPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::ObjectPropertyRange<ArcStr>>> for BoxWrap<ObjectPropertyRange> {
    fn from_c(value: &Box<horned_owl::model::ObjectPropertyRange<ArcStr>>) -> Self {
        BoxWrap::<ObjectPropertyRange>::from(value)
    }
}
impl FromCompatible<BoxWrap<ObjectPropertyRange>> for Box<horned_owl::model::ObjectPropertyRange<ArcStr>> {
    fn from_c(value: BoxWrap<ObjectPropertyRange>) -> Self {
        Box::<horned_owl::model::ObjectPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::ObjectPropertyRange<ArcStr>>> for BoxWrap<ObjectPropertyRange> {
    fn from_c(value: Box<horned_owl::model::ObjectPropertyRange<ArcStr>>) -> Self {
        BoxWrap::<ObjectPropertyRange>::from(value)
    }
}
impl FromCompatible<VecWrap<ObjectPropertyRange>> for Vec<horned_owl::model::ObjectPropertyRange<ArcStr>> {
    fn from_c(value: VecWrap<ObjectPropertyRange>) -> Self {
        Vec::<horned_owl::model::ObjectPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>> for VecWrap<ObjectPropertyRange> {
    fn from_c(value: Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>) -> Self {
        VecWrap::<ObjectPropertyRange>::from(value)
    }
}
impl FromCompatible<&VecWrap<ObjectPropertyRange>> for Vec<horned_owl::model::ObjectPropertyRange<ArcStr>> {
    fn from_c(value: &VecWrap<ObjectPropertyRange>) -> Self {
        Vec::<horned_owl::model::ObjectPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>> for VecWrap<ObjectPropertyRange> {
    fn from_c(value: &Vec<horned_owl::model::ObjectPropertyRange<ArcStr>>) -> Self {
        VecWrap::<ObjectPropertyRange>::from(value)
    }
}
#[doc = concat!(
    "FunctionalObjectProperty(first: ObjectPropertyExpression,)",
    "\n\n",
    doc!(FunctionalObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionalObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectPropertyExpression,
);

#[pymethods]
impl FunctionalObjectProperty {
    #[new]
    fn new(first: ObjectPropertyExpression,) -> Self {
        FunctionalObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::FunctionalObjectProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::FunctionalObjectProperty<ArcStr>> for FunctionalObjectProperty {
    fn from(value: &horned_owl::model::FunctionalObjectProperty<ArcStr>) -> Self {

        FunctionalObjectProperty (
    IntoCompatible::<ObjectPropertyExpression>::into_c(&value.0),
        )
    }
}

impl From<&FunctionalObjectProperty> for horned_owl::model::FunctionalObjectProperty<ArcStr> {
    fn from(value: &FunctionalObjectProperty) -> Self {
        horned_owl::model::FunctionalObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for FunctionalObjectProperty ****************/
impl FromCompatible<horned_owl::model::FunctionalObjectProperty<ArcStr>> for FunctionalObjectProperty {
    fn from_c(value: horned_owl::model::FunctionalObjectProperty<ArcStr>) -> Self {
        FunctionalObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::FunctionalObjectProperty<ArcStr>> for FunctionalObjectProperty {
    fn from_c(value: &horned_owl::model::FunctionalObjectProperty<ArcStr>) -> Self {
        FunctionalObjectProperty::from(value)
    }
}

impl FromCompatible<FunctionalObjectProperty> for horned_owl::model::FunctionalObjectProperty<ArcStr> {
    fn from_c(value: FunctionalObjectProperty) -> Self {
        horned_owl::model::FunctionalObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&FunctionalObjectProperty> for horned_owl::model::FunctionalObjectProperty<ArcStr> {
    fn from_c(value: &FunctionalObjectProperty) -> Self {
        horned_owl::model::FunctionalObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::FunctionalObjectProperty<ArcStr>> for FunctionalObjectProperty {
    fn from(value: horned_owl::model::FunctionalObjectProperty<ArcStr>) -> Self {
        FunctionalObjectProperty::from(value.borrow())
    }
}

impl From<FunctionalObjectProperty> for horned_owl::model::FunctionalObjectProperty<ArcStr> {
    fn from(value: FunctionalObjectProperty) -> Self {
        horned_owl::model::FunctionalObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<FunctionalObjectProperty>> for Box<horned_owl::model::FunctionalObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<FunctionalObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>> for BoxWrap<FunctionalObjectProperty> {
    fn from(value: &Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<FunctionalObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<FunctionalObjectProperty>> for Box<horned_owl::model::FunctionalObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<FunctionalObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>> for BoxWrap<FunctionalObjectProperty> {
    fn from(value: Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<FunctionalObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<FunctionalObjectProperty>> for Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>> {
    fn from(value: VecWrap<FunctionalObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>> for VecWrap<FunctionalObjectProperty> {
    fn from(value: Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<FunctionalObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<FunctionalObjectProperty>> for Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<FunctionalObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::FunctionalObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>> for VecWrap<FunctionalObjectProperty> {
    fn from(value: &Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(FunctionalObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<FunctionalObjectProperty>> for Box<horned_owl::model::FunctionalObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<FunctionalObjectProperty>) -> Self {
        Box::<horned_owl::model::FunctionalObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>> for BoxWrap<FunctionalObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<FunctionalObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<FunctionalObjectProperty>> for Box<horned_owl::model::FunctionalObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<FunctionalObjectProperty>) -> Self {
        Box::<horned_owl::model::FunctionalObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>> for BoxWrap<FunctionalObjectProperty> {
    fn from_c(value: Box<horned_owl::model::FunctionalObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<FunctionalObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<FunctionalObjectProperty>> for Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<FunctionalObjectProperty>) -> Self {
        Vec::<horned_owl::model::FunctionalObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>> for VecWrap<FunctionalObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>) -> Self {
        VecWrap::<FunctionalObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<FunctionalObjectProperty>> for Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<FunctionalObjectProperty>) -> Self {
        Vec::<horned_owl::model::FunctionalObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>> for VecWrap<FunctionalObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::FunctionalObjectProperty<ArcStr>>) -> Self {
        VecWrap::<FunctionalObjectProperty>::from(value)
    }
}
#[doc = concat!(
    "InverseFunctionalObjectProperty(first: ObjectPropertyExpression,)",
    "\n\n",
    doc!(InverseFunctionalObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InverseFunctionalObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectPropertyExpression,
);

#[pymethods]
impl InverseFunctionalObjectProperty {
    #[new]
    fn new(first: ObjectPropertyExpression,) -> Self {
        InverseFunctionalObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> for InverseFunctionalObjectProperty {
    fn from(value: &horned_owl::model::InverseFunctionalObjectProperty<ArcStr>) -> Self {

        InverseFunctionalObjectProperty (
    IntoCompatible::<ObjectPropertyExpression>::into_c(&value.0),
        )
    }
}

impl From<&InverseFunctionalObjectProperty> for horned_owl::model::InverseFunctionalObjectProperty<ArcStr> {
    fn from(value: &InverseFunctionalObjectProperty) -> Self {
        horned_owl::model::InverseFunctionalObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for InverseFunctionalObjectProperty ****************/
impl FromCompatible<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> for InverseFunctionalObjectProperty {
    fn from_c(value: horned_owl::model::InverseFunctionalObjectProperty<ArcStr>) -> Self {
        InverseFunctionalObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> for InverseFunctionalObjectProperty {
    fn from_c(value: &horned_owl::model::InverseFunctionalObjectProperty<ArcStr>) -> Self {
        InverseFunctionalObjectProperty::from(value)
    }
}

impl FromCompatible<InverseFunctionalObjectProperty> for horned_owl::model::InverseFunctionalObjectProperty<ArcStr> {
    fn from_c(value: InverseFunctionalObjectProperty) -> Self {
        horned_owl::model::InverseFunctionalObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&InverseFunctionalObjectProperty> for horned_owl::model::InverseFunctionalObjectProperty<ArcStr> {
    fn from_c(value: &InverseFunctionalObjectProperty) -> Self {
        horned_owl::model::InverseFunctionalObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> for InverseFunctionalObjectProperty {
    fn from(value: horned_owl::model::InverseFunctionalObjectProperty<ArcStr>) -> Self {
        InverseFunctionalObjectProperty::from(value.borrow())
    }
}

impl From<InverseFunctionalObjectProperty> for horned_owl::model::InverseFunctionalObjectProperty<ArcStr> {
    fn from(value: InverseFunctionalObjectProperty) -> Self {
        horned_owl::model::InverseFunctionalObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<InverseFunctionalObjectProperty>> for Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<InverseFunctionalObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>> for BoxWrap<InverseFunctionalObjectProperty> {
    fn from(value: &Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<InverseFunctionalObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<InverseFunctionalObjectProperty>> for Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<InverseFunctionalObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>> for BoxWrap<InverseFunctionalObjectProperty> {
    fn from(value: Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<InverseFunctionalObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<InverseFunctionalObjectProperty>> for Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> {
    fn from(value: VecWrap<InverseFunctionalObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>> for VecWrap<InverseFunctionalObjectProperty> {
    fn from(value: Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<InverseFunctionalObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<InverseFunctionalObjectProperty>> for Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<InverseFunctionalObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::InverseFunctionalObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>> for VecWrap<InverseFunctionalObjectProperty> {
    fn from(value: &Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(InverseFunctionalObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<InverseFunctionalObjectProperty>> for Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<InverseFunctionalObjectProperty>) -> Self {
        Box::<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>> for BoxWrap<InverseFunctionalObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<InverseFunctionalObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<InverseFunctionalObjectProperty>> for Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<InverseFunctionalObjectProperty>) -> Self {
        Box::<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>> for BoxWrap<InverseFunctionalObjectProperty> {
    fn from_c(value: Box<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<InverseFunctionalObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<InverseFunctionalObjectProperty>> for Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<InverseFunctionalObjectProperty>) -> Self {
        Vec::<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>> for VecWrap<InverseFunctionalObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>) -> Self {
        VecWrap::<InverseFunctionalObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<InverseFunctionalObjectProperty>> for Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<InverseFunctionalObjectProperty>) -> Self {
        Vec::<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>> for VecWrap<InverseFunctionalObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::InverseFunctionalObjectProperty<ArcStr>>) -> Self {
        VecWrap::<InverseFunctionalObjectProperty>::from(value)
    }
}
#[doc = concat!(
    "ReflexiveObjectProperty(first: ObjectPropertyExpression,)",
    "\n\n",
    doc!(ReflexiveObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReflexiveObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectPropertyExpression,
);

#[pymethods]
impl ReflexiveObjectProperty {
    #[new]
    fn new(first: ObjectPropertyExpression,) -> Self {
        ReflexiveObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::ReflexiveObjectProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::ReflexiveObjectProperty<ArcStr>> for ReflexiveObjectProperty {
    fn from(value: &horned_owl::model::ReflexiveObjectProperty<ArcStr>) -> Self {

        ReflexiveObjectProperty (
    IntoCompatible::<ObjectPropertyExpression>::into_c(&value.0),
        )
    }
}

impl From<&ReflexiveObjectProperty> for horned_owl::model::ReflexiveObjectProperty<ArcStr> {
    fn from(value: &ReflexiveObjectProperty) -> Self {
        horned_owl::model::ReflexiveObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for ReflexiveObjectProperty ****************/
impl FromCompatible<horned_owl::model::ReflexiveObjectProperty<ArcStr>> for ReflexiveObjectProperty {
    fn from_c(value: horned_owl::model::ReflexiveObjectProperty<ArcStr>) -> Self {
        ReflexiveObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::ReflexiveObjectProperty<ArcStr>> for ReflexiveObjectProperty {
    fn from_c(value: &horned_owl::model::ReflexiveObjectProperty<ArcStr>) -> Self {
        ReflexiveObjectProperty::from(value)
    }
}

impl FromCompatible<ReflexiveObjectProperty> for horned_owl::model::ReflexiveObjectProperty<ArcStr> {
    fn from_c(value: ReflexiveObjectProperty) -> Self {
        horned_owl::model::ReflexiveObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&ReflexiveObjectProperty> for horned_owl::model::ReflexiveObjectProperty<ArcStr> {
    fn from_c(value: &ReflexiveObjectProperty) -> Self {
        horned_owl::model::ReflexiveObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::ReflexiveObjectProperty<ArcStr>> for ReflexiveObjectProperty {
    fn from(value: horned_owl::model::ReflexiveObjectProperty<ArcStr>) -> Self {
        ReflexiveObjectProperty::from(value.borrow())
    }
}

impl From<ReflexiveObjectProperty> for horned_owl::model::ReflexiveObjectProperty<ArcStr> {
    fn from(value: ReflexiveObjectProperty) -> Self {
        horned_owl::model::ReflexiveObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<ReflexiveObjectProperty>> for Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<ReflexiveObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>> for BoxWrap<ReflexiveObjectProperty> {
    fn from(value: &Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<ReflexiveObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<ReflexiveObjectProperty>> for Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<ReflexiveObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>> for BoxWrap<ReflexiveObjectProperty> {
    fn from(value: Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<ReflexiveObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<ReflexiveObjectProperty>> for Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>> {
    fn from(value: VecWrap<ReflexiveObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>> for VecWrap<ReflexiveObjectProperty> {
    fn from(value: Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<ReflexiveObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<ReflexiveObjectProperty>> for Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<ReflexiveObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::ReflexiveObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>> for VecWrap<ReflexiveObjectProperty> {
    fn from(value: &Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(ReflexiveObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<ReflexiveObjectProperty>> for Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<ReflexiveObjectProperty>) -> Self {
        Box::<horned_owl::model::ReflexiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>> for BoxWrap<ReflexiveObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<ReflexiveObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<ReflexiveObjectProperty>> for Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<ReflexiveObjectProperty>) -> Self {
        Box::<horned_owl::model::ReflexiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>> for BoxWrap<ReflexiveObjectProperty> {
    fn from_c(value: Box<horned_owl::model::ReflexiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<ReflexiveObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<ReflexiveObjectProperty>> for Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<ReflexiveObjectProperty>) -> Self {
        Vec::<horned_owl::model::ReflexiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>> for VecWrap<ReflexiveObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>) -> Self {
        VecWrap::<ReflexiveObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<ReflexiveObjectProperty>> for Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<ReflexiveObjectProperty>) -> Self {
        Vec::<horned_owl::model::ReflexiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>> for VecWrap<ReflexiveObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::ReflexiveObjectProperty<ArcStr>>) -> Self {
        VecWrap::<ReflexiveObjectProperty>::from(value)
    }
}
#[doc = concat!(
    "IrreflexiveObjectProperty(first: ObjectPropertyExpression,)",
    "\n\n",
    doc!(IrreflexiveObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IrreflexiveObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectPropertyExpression,
);

#[pymethods]
impl IrreflexiveObjectProperty {
    #[new]
    fn new(first: ObjectPropertyExpression,) -> Self {
        IrreflexiveObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::IrreflexiveObjectProperty<ArcStr>> for IrreflexiveObjectProperty {
    fn from(value: &horned_owl::model::IrreflexiveObjectProperty<ArcStr>) -> Self {

        IrreflexiveObjectProperty (
    IntoCompatible::<ObjectPropertyExpression>::into_c(&value.0),
        )
    }
}

impl From<&IrreflexiveObjectProperty> for horned_owl::model::IrreflexiveObjectProperty<ArcStr> {
    fn from(value: &IrreflexiveObjectProperty) -> Self {
        horned_owl::model::IrreflexiveObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for IrreflexiveObjectProperty ****************/
impl FromCompatible<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> for IrreflexiveObjectProperty {
    fn from_c(value: horned_owl::model::IrreflexiveObjectProperty<ArcStr>) -> Self {
        IrreflexiveObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::IrreflexiveObjectProperty<ArcStr>> for IrreflexiveObjectProperty {
    fn from_c(value: &horned_owl::model::IrreflexiveObjectProperty<ArcStr>) -> Self {
        IrreflexiveObjectProperty::from(value)
    }
}

impl FromCompatible<IrreflexiveObjectProperty> for horned_owl::model::IrreflexiveObjectProperty<ArcStr> {
    fn from_c(value: IrreflexiveObjectProperty) -> Self {
        horned_owl::model::IrreflexiveObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&IrreflexiveObjectProperty> for horned_owl::model::IrreflexiveObjectProperty<ArcStr> {
    fn from_c(value: &IrreflexiveObjectProperty) -> Self {
        horned_owl::model::IrreflexiveObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> for IrreflexiveObjectProperty {
    fn from(value: horned_owl::model::IrreflexiveObjectProperty<ArcStr>) -> Self {
        IrreflexiveObjectProperty::from(value.borrow())
    }
}

impl From<IrreflexiveObjectProperty> for horned_owl::model::IrreflexiveObjectProperty<ArcStr> {
    fn from(value: IrreflexiveObjectProperty) -> Self {
        horned_owl::model::IrreflexiveObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<IrreflexiveObjectProperty>> for Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<IrreflexiveObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>> for BoxWrap<IrreflexiveObjectProperty> {
    fn from(value: &Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<IrreflexiveObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<IrreflexiveObjectProperty>> for Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<IrreflexiveObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>> for BoxWrap<IrreflexiveObjectProperty> {
    fn from(value: Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<IrreflexiveObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<IrreflexiveObjectProperty>> for Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> {
    fn from(value: VecWrap<IrreflexiveObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>> for VecWrap<IrreflexiveObjectProperty> {
    fn from(value: Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<IrreflexiveObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<IrreflexiveObjectProperty>> for Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<IrreflexiveObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::IrreflexiveObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>> for VecWrap<IrreflexiveObjectProperty> {
    fn from(value: &Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(IrreflexiveObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<IrreflexiveObjectProperty>> for Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<IrreflexiveObjectProperty>) -> Self {
        Box::<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>> for BoxWrap<IrreflexiveObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<IrreflexiveObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<IrreflexiveObjectProperty>> for Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<IrreflexiveObjectProperty>) -> Self {
        Box::<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>> for BoxWrap<IrreflexiveObjectProperty> {
    fn from_c(value: Box<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<IrreflexiveObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<IrreflexiveObjectProperty>> for Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<IrreflexiveObjectProperty>) -> Self {
        Vec::<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>> for VecWrap<IrreflexiveObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>) -> Self {
        VecWrap::<IrreflexiveObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<IrreflexiveObjectProperty>> for Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<IrreflexiveObjectProperty>) -> Self {
        Vec::<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>> for VecWrap<IrreflexiveObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::IrreflexiveObjectProperty<ArcStr>>) -> Self {
        VecWrap::<IrreflexiveObjectProperty>::from(value)
    }
}
#[doc = concat!(
    "SymmetricObjectProperty(first: ObjectPropertyExpression,)",
    "\n\n",
    doc!(SymmetricObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SymmetricObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectPropertyExpression,
);

#[pymethods]
impl SymmetricObjectProperty {
    #[new]
    fn new(first: ObjectPropertyExpression,) -> Self {
        SymmetricObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::SymmetricObjectProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::SymmetricObjectProperty<ArcStr>> for SymmetricObjectProperty {
    fn from(value: &horned_owl::model::SymmetricObjectProperty<ArcStr>) -> Self {

        SymmetricObjectProperty (
    IntoCompatible::<ObjectPropertyExpression>::into_c(&value.0),
        )
    }
}

impl From<&SymmetricObjectProperty> for horned_owl::model::SymmetricObjectProperty<ArcStr> {
    fn from(value: &SymmetricObjectProperty) -> Self {
        horned_owl::model::SymmetricObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for SymmetricObjectProperty ****************/
impl FromCompatible<horned_owl::model::SymmetricObjectProperty<ArcStr>> for SymmetricObjectProperty {
    fn from_c(value: horned_owl::model::SymmetricObjectProperty<ArcStr>) -> Self {
        SymmetricObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::SymmetricObjectProperty<ArcStr>> for SymmetricObjectProperty {
    fn from_c(value: &horned_owl::model::SymmetricObjectProperty<ArcStr>) -> Self {
        SymmetricObjectProperty::from(value)
    }
}

impl FromCompatible<SymmetricObjectProperty> for horned_owl::model::SymmetricObjectProperty<ArcStr> {
    fn from_c(value: SymmetricObjectProperty) -> Self {
        horned_owl::model::SymmetricObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&SymmetricObjectProperty> for horned_owl::model::SymmetricObjectProperty<ArcStr> {
    fn from_c(value: &SymmetricObjectProperty) -> Self {
        horned_owl::model::SymmetricObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::SymmetricObjectProperty<ArcStr>> for SymmetricObjectProperty {
    fn from(value: horned_owl::model::SymmetricObjectProperty<ArcStr>) -> Self {
        SymmetricObjectProperty::from(value.borrow())
    }
}

impl From<SymmetricObjectProperty> for horned_owl::model::SymmetricObjectProperty<ArcStr> {
    fn from(value: SymmetricObjectProperty) -> Self {
        horned_owl::model::SymmetricObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<SymmetricObjectProperty>> for Box<horned_owl::model::SymmetricObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<SymmetricObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>> for BoxWrap<SymmetricObjectProperty> {
    fn from(value: &Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<SymmetricObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<SymmetricObjectProperty>> for Box<horned_owl::model::SymmetricObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<SymmetricObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>> for BoxWrap<SymmetricObjectProperty> {
    fn from(value: Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<SymmetricObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<SymmetricObjectProperty>> for Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>> {
    fn from(value: VecWrap<SymmetricObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>> for VecWrap<SymmetricObjectProperty> {
    fn from(value: Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<SymmetricObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<SymmetricObjectProperty>> for Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<SymmetricObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::SymmetricObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>> for VecWrap<SymmetricObjectProperty> {
    fn from(value: &Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(SymmetricObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<SymmetricObjectProperty>> for Box<horned_owl::model::SymmetricObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<SymmetricObjectProperty>) -> Self {
        Box::<horned_owl::model::SymmetricObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>> for BoxWrap<SymmetricObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<SymmetricObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<SymmetricObjectProperty>> for Box<horned_owl::model::SymmetricObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<SymmetricObjectProperty>) -> Self {
        Box::<horned_owl::model::SymmetricObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>> for BoxWrap<SymmetricObjectProperty> {
    fn from_c(value: Box<horned_owl::model::SymmetricObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<SymmetricObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<SymmetricObjectProperty>> for Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<SymmetricObjectProperty>) -> Self {
        Vec::<horned_owl::model::SymmetricObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>> for VecWrap<SymmetricObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>) -> Self {
        VecWrap::<SymmetricObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<SymmetricObjectProperty>> for Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<SymmetricObjectProperty>) -> Self {
        Vec::<horned_owl::model::SymmetricObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>> for VecWrap<SymmetricObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::SymmetricObjectProperty<ArcStr>>) -> Self {
        VecWrap::<SymmetricObjectProperty>::from(value)
    }
}
#[doc = concat!(
    "AsymmetricObjectProperty(first: ObjectPropertyExpression,)",
    "\n\n",
    doc!(AsymmetricObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsymmetricObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectPropertyExpression,
);

#[pymethods]
impl AsymmetricObjectProperty {
    #[new]
    fn new(first: ObjectPropertyExpression,) -> Self {
        AsymmetricObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::AsymmetricObjectProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::AsymmetricObjectProperty<ArcStr>> for AsymmetricObjectProperty {
    fn from(value: &horned_owl::model::AsymmetricObjectProperty<ArcStr>) -> Self {

        AsymmetricObjectProperty (
    IntoCompatible::<ObjectPropertyExpression>::into_c(&value.0),
        )
    }
}

impl From<&AsymmetricObjectProperty> for horned_owl::model::AsymmetricObjectProperty<ArcStr> {
    fn from(value: &AsymmetricObjectProperty) -> Self {
        horned_owl::model::AsymmetricObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for AsymmetricObjectProperty ****************/
impl FromCompatible<horned_owl::model::AsymmetricObjectProperty<ArcStr>> for AsymmetricObjectProperty {
    fn from_c(value: horned_owl::model::AsymmetricObjectProperty<ArcStr>) -> Self {
        AsymmetricObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AsymmetricObjectProperty<ArcStr>> for AsymmetricObjectProperty {
    fn from_c(value: &horned_owl::model::AsymmetricObjectProperty<ArcStr>) -> Self {
        AsymmetricObjectProperty::from(value)
    }
}

impl FromCompatible<AsymmetricObjectProperty> for horned_owl::model::AsymmetricObjectProperty<ArcStr> {
    fn from_c(value: AsymmetricObjectProperty) -> Self {
        horned_owl::model::AsymmetricObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AsymmetricObjectProperty> for horned_owl::model::AsymmetricObjectProperty<ArcStr> {
    fn from_c(value: &AsymmetricObjectProperty) -> Self {
        horned_owl::model::AsymmetricObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AsymmetricObjectProperty<ArcStr>> for AsymmetricObjectProperty {
    fn from(value: horned_owl::model::AsymmetricObjectProperty<ArcStr>) -> Self {
        AsymmetricObjectProperty::from(value.borrow())
    }
}

impl From<AsymmetricObjectProperty> for horned_owl::model::AsymmetricObjectProperty<ArcStr> {
    fn from(value: AsymmetricObjectProperty) -> Self {
        horned_owl::model::AsymmetricObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AsymmetricObjectProperty>> for Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<AsymmetricObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>> for BoxWrap<AsymmetricObjectProperty> {
    fn from(value: &Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AsymmetricObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<AsymmetricObjectProperty>> for Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<AsymmetricObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>> for BoxWrap<AsymmetricObjectProperty> {
    fn from(value: Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<AsymmetricObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<AsymmetricObjectProperty>> for Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>> {
    fn from(value: VecWrap<AsymmetricObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>> for VecWrap<AsymmetricObjectProperty> {
    fn from(value: Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<AsymmetricObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<AsymmetricObjectProperty>> for Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<AsymmetricObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AsymmetricObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>> for VecWrap<AsymmetricObjectProperty> {
    fn from(value: &Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AsymmetricObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AsymmetricObjectProperty>> for Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<AsymmetricObjectProperty>) -> Self {
        Box::<horned_owl::model::AsymmetricObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>> for BoxWrap<AsymmetricObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<AsymmetricObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<AsymmetricObjectProperty>> for Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<AsymmetricObjectProperty>) -> Self {
        Box::<horned_owl::model::AsymmetricObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>> for BoxWrap<AsymmetricObjectProperty> {
    fn from_c(value: Box<horned_owl::model::AsymmetricObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<AsymmetricObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<AsymmetricObjectProperty>> for Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<AsymmetricObjectProperty>) -> Self {
        Vec::<horned_owl::model::AsymmetricObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>> for VecWrap<AsymmetricObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>) -> Self {
        VecWrap::<AsymmetricObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<AsymmetricObjectProperty>> for Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<AsymmetricObjectProperty>) -> Self {
        Vec::<horned_owl::model::AsymmetricObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>> for VecWrap<AsymmetricObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::AsymmetricObjectProperty<ArcStr>>) -> Self {
        VecWrap::<AsymmetricObjectProperty>::from(value)
    }
}
#[doc = concat!(
    "TransitiveObjectProperty(first: ObjectPropertyExpression,)",
    "\n\n",
    doc!(TransitiveObjectProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransitiveObjectProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub ObjectPropertyExpression,
);

#[pymethods]
impl TransitiveObjectProperty {
    #[new]
    fn new(first: ObjectPropertyExpression,) -> Self {
        TransitiveObjectProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::TransitiveObjectProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::TransitiveObjectProperty<ArcStr>> for TransitiveObjectProperty {
    fn from(value: &horned_owl::model::TransitiveObjectProperty<ArcStr>) -> Self {

        TransitiveObjectProperty (
    IntoCompatible::<ObjectPropertyExpression>::into_c(&value.0),
        )
    }
}

impl From<&TransitiveObjectProperty> for horned_owl::model::TransitiveObjectProperty<ArcStr> {
    fn from(value: &TransitiveObjectProperty) -> Self {
        horned_owl::model::TransitiveObjectProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for TransitiveObjectProperty ****************/
impl FromCompatible<horned_owl::model::TransitiveObjectProperty<ArcStr>> for TransitiveObjectProperty {
    fn from_c(value: horned_owl::model::TransitiveObjectProperty<ArcStr>) -> Self {
        TransitiveObjectProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::TransitiveObjectProperty<ArcStr>> for TransitiveObjectProperty {
    fn from_c(value: &horned_owl::model::TransitiveObjectProperty<ArcStr>) -> Self {
        TransitiveObjectProperty::from(value)
    }
}

impl FromCompatible<TransitiveObjectProperty> for horned_owl::model::TransitiveObjectProperty<ArcStr> {
    fn from_c(value: TransitiveObjectProperty) -> Self {
        horned_owl::model::TransitiveObjectProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&TransitiveObjectProperty> for horned_owl::model::TransitiveObjectProperty<ArcStr> {
    fn from_c(value: &TransitiveObjectProperty) -> Self {
        horned_owl::model::TransitiveObjectProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::TransitiveObjectProperty<ArcStr>> for TransitiveObjectProperty {
    fn from(value: horned_owl::model::TransitiveObjectProperty<ArcStr>) -> Self {
        TransitiveObjectProperty::from(value.borrow())
    }
}

impl From<TransitiveObjectProperty> for horned_owl::model::TransitiveObjectProperty<ArcStr> {
    fn from(value: TransitiveObjectProperty) -> Self {
        horned_owl::model::TransitiveObjectProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<TransitiveObjectProperty>> for Box<horned_owl::model::TransitiveObjectProperty<ArcStr>> {
    fn from(value: &BoxWrap<TransitiveObjectProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>> for BoxWrap<TransitiveObjectProperty> {
    fn from(value: &Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<TransitiveObjectProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<TransitiveObjectProperty>> for Box<horned_owl::model::TransitiveObjectProperty<ArcStr>> {
    fn from(value: BoxWrap<TransitiveObjectProperty>) -> Self {
        Into::<Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>> for BoxWrap<TransitiveObjectProperty> {
    fn from(value: Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<TransitiveObjectProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<TransitiveObjectProperty>> for Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>> {
    fn from(value: VecWrap<TransitiveObjectProperty>) -> Self {
        Into::<Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>> for VecWrap<TransitiveObjectProperty> {
    fn from(value: Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>) -> Self {
        Into::<VecWrap<TransitiveObjectProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<TransitiveObjectProperty>> for Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>> {
    fn from(value: &VecWrap<TransitiveObjectProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::TransitiveObjectProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>> for VecWrap<TransitiveObjectProperty> {
    fn from(value: &Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(TransitiveObjectProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<TransitiveObjectProperty>> for Box<horned_owl::model::TransitiveObjectProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<TransitiveObjectProperty>) -> Self {
        Box::<horned_owl::model::TransitiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>> for BoxWrap<TransitiveObjectProperty> {
    fn from_c(value: &Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<TransitiveObjectProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<TransitiveObjectProperty>> for Box<horned_owl::model::TransitiveObjectProperty<ArcStr>> {
    fn from_c(value: BoxWrap<TransitiveObjectProperty>) -> Self {
        Box::<horned_owl::model::TransitiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>> for BoxWrap<TransitiveObjectProperty> {
    fn from_c(value: Box<horned_owl::model::TransitiveObjectProperty<ArcStr>>) -> Self {
        BoxWrap::<TransitiveObjectProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<TransitiveObjectProperty>> for Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>> {
    fn from_c(value: VecWrap<TransitiveObjectProperty>) -> Self {
        Vec::<horned_owl::model::TransitiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>> for VecWrap<TransitiveObjectProperty> {
    fn from_c(value: Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>) -> Self {
        VecWrap::<TransitiveObjectProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<TransitiveObjectProperty>> for Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>> {
    fn from_c(value: &VecWrap<TransitiveObjectProperty>) -> Self {
        Vec::<horned_owl::model::TransitiveObjectProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>> for VecWrap<TransitiveObjectProperty> {
    fn from_c(value: &Vec<horned_owl::model::TransitiveObjectProperty<ArcStr>>) -> Self {
        VecWrap::<TransitiveObjectProperty>::from(value)
    }
}
#[doc = concat!("SubDataPropertyOf(sub: DataProperty,sup: DataProperty,)",
    "\n\n",
    doc!(SubDataPropertyOf)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubDataPropertyOf {
        #[doc="sub: DataProperty"]
        #[pyo3(get,set)]
        pub sub: DataProperty,
    
        #[doc="sup: DataProperty"]
        #[pyo3(get,set)]
        pub sup: DataProperty,
    }

#[pymethods]
impl SubDataPropertyOf {
    #[new]
    fn new(sub: DataProperty,sup: DataProperty,) -> Self {
        SubDataPropertyOf {
                sub,
                sup,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "sub" => self.sub.clone().into_pyobject(py).map(Bound::into_any),
            "sup" => self.sup.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "sub" => {
                self.sub = value.extract()?;
                Ok(())
            },
            "sup" => {
                self.sup = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::SubDataPropertyOf<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::SubDataPropertyOf<ArcStr>> for SubDataPropertyOf {
    fn from(value: &horned_owl::model::SubDataPropertyOf<ArcStr>) -> Self {
        SubDataPropertyOf {
            sub: IntoCompatible::<DataProperty>::into_c(value.sub.borrow()),
            sup: IntoCompatible::<DataProperty>::into_c(value.sup.borrow()),
        }
    }
}


impl From<&SubDataPropertyOf> for horned_owl::model::SubDataPropertyOf<ArcStr> {
    fn from(value: &SubDataPropertyOf) -> Self {
        horned_owl::model::SubDataPropertyOf::<ArcStr> {
            sub: value.sub.borrow().into_c(),
            sup: value.sup.borrow().into_c(),
        }
    }
}



/**************** Base implementations for SubDataPropertyOf ****************/
impl FromCompatible<horned_owl::model::SubDataPropertyOf<ArcStr>> for SubDataPropertyOf {
    fn from_c(value: horned_owl::model::SubDataPropertyOf<ArcStr>) -> Self {
        SubDataPropertyOf::from(value)
    }
}

impl FromCompatible<&horned_owl::model::SubDataPropertyOf<ArcStr>> for SubDataPropertyOf {
    fn from_c(value: &horned_owl::model::SubDataPropertyOf<ArcStr>) -> Self {
        SubDataPropertyOf::from(value)
    }
}

impl FromCompatible<SubDataPropertyOf> for horned_owl::model::SubDataPropertyOf<ArcStr> {
    fn from_c(value: SubDataPropertyOf) -> Self {
        horned_owl::model::SubDataPropertyOf::<ArcStr>::from(value)
    }
}

impl FromCompatible<&SubDataPropertyOf> for horned_owl::model::SubDataPropertyOf<ArcStr> {
    fn from_c(value: &SubDataPropertyOf) -> Self {
        horned_owl::model::SubDataPropertyOf::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::SubDataPropertyOf<ArcStr>> for SubDataPropertyOf {
    fn from(value: horned_owl::model::SubDataPropertyOf<ArcStr>) -> Self {
        SubDataPropertyOf::from(value.borrow())
    }
}

impl From<SubDataPropertyOf> for horned_owl::model::SubDataPropertyOf<ArcStr> {
    fn from(value: SubDataPropertyOf) -> Self {
        horned_owl::model::SubDataPropertyOf::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<SubDataPropertyOf>> for Box<horned_owl::model::SubDataPropertyOf<ArcStr>> {
    fn from(value: &BoxWrap<SubDataPropertyOf>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::SubDataPropertyOf<ArcStr>>> for BoxWrap<SubDataPropertyOf> {
    fn from(value: &Box<horned_owl::model::SubDataPropertyOf<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<SubDataPropertyOf>::into(*value.clone())))
    }
}

impl From<BoxWrap<SubDataPropertyOf>> for Box<horned_owl::model::SubDataPropertyOf<ArcStr>> {
    fn from(value: BoxWrap<SubDataPropertyOf>) -> Self {
        Into::<Box<horned_owl::model::SubDataPropertyOf<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::SubDataPropertyOf<ArcStr>>> for BoxWrap<SubDataPropertyOf> {
    fn from(value: Box<horned_owl::model::SubDataPropertyOf<ArcStr>>) -> Self {
        Into::<BoxWrap<SubDataPropertyOf>>::into(value.borrow())
    }
}

impl From<VecWrap<SubDataPropertyOf>> for Vec<horned_owl::model::SubDataPropertyOf<ArcStr>> {
    fn from(value: VecWrap<SubDataPropertyOf>) -> Self {
        Into::<Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>> for VecWrap<SubDataPropertyOf> {
    fn from(value: Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>) -> Self {
        Into::<VecWrap<SubDataPropertyOf>>::into(value.borrow())
    }
}

impl From<&VecWrap<SubDataPropertyOf>> for Vec<horned_owl::model::SubDataPropertyOf<ArcStr>> {
    fn from(value: &VecWrap<SubDataPropertyOf>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::SubDataPropertyOf::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>> for VecWrap<SubDataPropertyOf> {
    fn from(value: &Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>) -> Self {
        VecWrap(value.iter().map(SubDataPropertyOf::from).collect())
    }
}

impl FromCompatible<&BoxWrap<SubDataPropertyOf>> for Box<horned_owl::model::SubDataPropertyOf<ArcStr>> {
    fn from_c(value: &BoxWrap<SubDataPropertyOf>) -> Self {
        Box::<horned_owl::model::SubDataPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::SubDataPropertyOf<ArcStr>>> for BoxWrap<SubDataPropertyOf> {
    fn from_c(value: &Box<horned_owl::model::SubDataPropertyOf<ArcStr>>) -> Self {
        BoxWrap::<SubDataPropertyOf>::from(value)
    }
}
impl FromCompatible<BoxWrap<SubDataPropertyOf>> for Box<horned_owl::model::SubDataPropertyOf<ArcStr>> {
    fn from_c(value: BoxWrap<SubDataPropertyOf>) -> Self {
        Box::<horned_owl::model::SubDataPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::SubDataPropertyOf<ArcStr>>> for BoxWrap<SubDataPropertyOf> {
    fn from_c(value: Box<horned_owl::model::SubDataPropertyOf<ArcStr>>) -> Self {
        BoxWrap::<SubDataPropertyOf>::from(value)
    }
}
impl FromCompatible<VecWrap<SubDataPropertyOf>> for Vec<horned_owl::model::SubDataPropertyOf<ArcStr>> {
    fn from_c(value: VecWrap<SubDataPropertyOf>) -> Self {
        Vec::<horned_owl::model::SubDataPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>> for VecWrap<SubDataPropertyOf> {
    fn from_c(value: Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>) -> Self {
        VecWrap::<SubDataPropertyOf>::from(value)
    }
}
impl FromCompatible<&VecWrap<SubDataPropertyOf>> for Vec<horned_owl::model::SubDataPropertyOf<ArcStr>> {
    fn from_c(value: &VecWrap<SubDataPropertyOf>) -> Self {
        Vec::<horned_owl::model::SubDataPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>> for VecWrap<SubDataPropertyOf> {
    fn from_c(value: &Vec<horned_owl::model::SubDataPropertyOf<ArcStr>>) -> Self {
        VecWrap::<SubDataPropertyOf>::from(value)
    }
}
#[doc = concat!(
    "EquivalentDataProperties(first: typing.List[DataProperty],)",
    "\n\n",
    doc!(EquivalentDataProperties)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EquivalentDataProperties (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub VecWrap<DataProperty>,
);

#[pymethods]
impl EquivalentDataProperties {
    #[new]
    fn new(first: VecWrap<DataProperty>,) -> Self {
        EquivalentDataProperties (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::EquivalentDataProperties<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::EquivalentDataProperties<ArcStr>> for EquivalentDataProperties {
    fn from(value: &horned_owl::model::EquivalentDataProperties<ArcStr>) -> Self {

        EquivalentDataProperties (
    IntoCompatible::<VecWrap<DataProperty>>::into_c(&value.0),
        )
    }
}

impl From<&EquivalentDataProperties> for horned_owl::model::EquivalentDataProperties<ArcStr> {
    fn from(value: &EquivalentDataProperties) -> Self {
        horned_owl::model::EquivalentDataProperties::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for EquivalentDataProperties ****************/
impl FromCompatible<horned_owl::model::EquivalentDataProperties<ArcStr>> for EquivalentDataProperties {
    fn from_c(value: horned_owl::model::EquivalentDataProperties<ArcStr>) -> Self {
        EquivalentDataProperties::from(value)
    }
}

impl FromCompatible<&horned_owl::model::EquivalentDataProperties<ArcStr>> for EquivalentDataProperties {
    fn from_c(value: &horned_owl::model::EquivalentDataProperties<ArcStr>) -> Self {
        EquivalentDataProperties::from(value)
    }
}

impl FromCompatible<EquivalentDataProperties> for horned_owl::model::EquivalentDataProperties<ArcStr> {
    fn from_c(value: EquivalentDataProperties) -> Self {
        horned_owl::model::EquivalentDataProperties::<ArcStr>::from(value)
    }
}

impl FromCompatible<&EquivalentDataProperties> for horned_owl::model::EquivalentDataProperties<ArcStr> {
    fn from_c(value: &EquivalentDataProperties) -> Self {
        horned_owl::model::EquivalentDataProperties::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::EquivalentDataProperties<ArcStr>> for EquivalentDataProperties {
    fn from(value: horned_owl::model::EquivalentDataProperties<ArcStr>) -> Self {
        EquivalentDataProperties::from(value.borrow())
    }
}

impl From<EquivalentDataProperties> for horned_owl::model::EquivalentDataProperties<ArcStr> {
    fn from(value: EquivalentDataProperties) -> Self {
        horned_owl::model::EquivalentDataProperties::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<EquivalentDataProperties>> for Box<horned_owl::model::EquivalentDataProperties<ArcStr>> {
    fn from(value: &BoxWrap<EquivalentDataProperties>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::EquivalentDataProperties<ArcStr>>> for BoxWrap<EquivalentDataProperties> {
    fn from(value: &Box<horned_owl::model::EquivalentDataProperties<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<EquivalentDataProperties>::into(*value.clone())))
    }
}

impl From<BoxWrap<EquivalentDataProperties>> for Box<horned_owl::model::EquivalentDataProperties<ArcStr>> {
    fn from(value: BoxWrap<EquivalentDataProperties>) -> Self {
        Into::<Box<horned_owl::model::EquivalentDataProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::EquivalentDataProperties<ArcStr>>> for BoxWrap<EquivalentDataProperties> {
    fn from(value: Box<horned_owl::model::EquivalentDataProperties<ArcStr>>) -> Self {
        Into::<BoxWrap<EquivalentDataProperties>>::into(value.borrow())
    }
}

impl From<VecWrap<EquivalentDataProperties>> for Vec<horned_owl::model::EquivalentDataProperties<ArcStr>> {
    fn from(value: VecWrap<EquivalentDataProperties>) -> Self {
        Into::<Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>> for VecWrap<EquivalentDataProperties> {
    fn from(value: Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>) -> Self {
        Into::<VecWrap<EquivalentDataProperties>>::into(value.borrow())
    }
}

impl From<&VecWrap<EquivalentDataProperties>> for Vec<horned_owl::model::EquivalentDataProperties<ArcStr>> {
    fn from(value: &VecWrap<EquivalentDataProperties>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::EquivalentDataProperties::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>> for VecWrap<EquivalentDataProperties> {
    fn from(value: &Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>) -> Self {
        VecWrap(value.iter().map(EquivalentDataProperties::from).collect())
    }
}

impl FromCompatible<&BoxWrap<EquivalentDataProperties>> for Box<horned_owl::model::EquivalentDataProperties<ArcStr>> {
    fn from_c(value: &BoxWrap<EquivalentDataProperties>) -> Self {
        Box::<horned_owl::model::EquivalentDataProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::EquivalentDataProperties<ArcStr>>> for BoxWrap<EquivalentDataProperties> {
    fn from_c(value: &Box<horned_owl::model::EquivalentDataProperties<ArcStr>>) -> Self {
        BoxWrap::<EquivalentDataProperties>::from(value)
    }
}
impl FromCompatible<BoxWrap<EquivalentDataProperties>> for Box<horned_owl::model::EquivalentDataProperties<ArcStr>> {
    fn from_c(value: BoxWrap<EquivalentDataProperties>) -> Self {
        Box::<horned_owl::model::EquivalentDataProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::EquivalentDataProperties<ArcStr>>> for BoxWrap<EquivalentDataProperties> {
    fn from_c(value: Box<horned_owl::model::EquivalentDataProperties<ArcStr>>) -> Self {
        BoxWrap::<EquivalentDataProperties>::from(value)
    }
}
impl FromCompatible<VecWrap<EquivalentDataProperties>> for Vec<horned_owl::model::EquivalentDataProperties<ArcStr>> {
    fn from_c(value: VecWrap<EquivalentDataProperties>) -> Self {
        Vec::<horned_owl::model::EquivalentDataProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>> for VecWrap<EquivalentDataProperties> {
    fn from_c(value: Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>) -> Self {
        VecWrap::<EquivalentDataProperties>::from(value)
    }
}
impl FromCompatible<&VecWrap<EquivalentDataProperties>> for Vec<horned_owl::model::EquivalentDataProperties<ArcStr>> {
    fn from_c(value: &VecWrap<EquivalentDataProperties>) -> Self {
        Vec::<horned_owl::model::EquivalentDataProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>> for VecWrap<EquivalentDataProperties> {
    fn from_c(value: &Vec<horned_owl::model::EquivalentDataProperties<ArcStr>>) -> Self {
        VecWrap::<EquivalentDataProperties>::from(value)
    }
}
#[doc = concat!(
    "DisjointDataProperties(first: typing.List[DataProperty],)",
    "\n\n",
    doc!(DisjointDataProperties)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisjointDataProperties (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub VecWrap<DataProperty>,
);

#[pymethods]
impl DisjointDataProperties {
    #[new]
    fn new(first: VecWrap<DataProperty>,) -> Self {
        DisjointDataProperties (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DisjointDataProperties<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DisjointDataProperties<ArcStr>> for DisjointDataProperties {
    fn from(value: &horned_owl::model::DisjointDataProperties<ArcStr>) -> Self {

        DisjointDataProperties (
    IntoCompatible::<VecWrap<DataProperty>>::into_c(&value.0),
        )
    }
}

impl From<&DisjointDataProperties> for horned_owl::model::DisjointDataProperties<ArcStr> {
    fn from(value: &DisjointDataProperties) -> Self {
        horned_owl::model::DisjointDataProperties::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DisjointDataProperties ****************/
impl FromCompatible<horned_owl::model::DisjointDataProperties<ArcStr>> for DisjointDataProperties {
    fn from_c(value: horned_owl::model::DisjointDataProperties<ArcStr>) -> Self {
        DisjointDataProperties::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DisjointDataProperties<ArcStr>> for DisjointDataProperties {
    fn from_c(value: &horned_owl::model::DisjointDataProperties<ArcStr>) -> Self {
        DisjointDataProperties::from(value)
    }
}

impl FromCompatible<DisjointDataProperties> for horned_owl::model::DisjointDataProperties<ArcStr> {
    fn from_c(value: DisjointDataProperties) -> Self {
        horned_owl::model::DisjointDataProperties::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DisjointDataProperties> for horned_owl::model::DisjointDataProperties<ArcStr> {
    fn from_c(value: &DisjointDataProperties) -> Self {
        horned_owl::model::DisjointDataProperties::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DisjointDataProperties<ArcStr>> for DisjointDataProperties {
    fn from(value: horned_owl::model::DisjointDataProperties<ArcStr>) -> Self {
        DisjointDataProperties::from(value.borrow())
    }
}

impl From<DisjointDataProperties> for horned_owl::model::DisjointDataProperties<ArcStr> {
    fn from(value: DisjointDataProperties) -> Self {
        horned_owl::model::DisjointDataProperties::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DisjointDataProperties>> for Box<horned_owl::model::DisjointDataProperties<ArcStr>> {
    fn from(value: &BoxWrap<DisjointDataProperties>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DisjointDataProperties<ArcStr>>> for BoxWrap<DisjointDataProperties> {
    fn from(value: &Box<horned_owl::model::DisjointDataProperties<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DisjointDataProperties>::into(*value.clone())))
    }
}

impl From<BoxWrap<DisjointDataProperties>> for Box<horned_owl::model::DisjointDataProperties<ArcStr>> {
    fn from(value: BoxWrap<DisjointDataProperties>) -> Self {
        Into::<Box<horned_owl::model::DisjointDataProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DisjointDataProperties<ArcStr>>> for BoxWrap<DisjointDataProperties> {
    fn from(value: Box<horned_owl::model::DisjointDataProperties<ArcStr>>) -> Self {
        Into::<BoxWrap<DisjointDataProperties>>::into(value.borrow())
    }
}

impl From<VecWrap<DisjointDataProperties>> for Vec<horned_owl::model::DisjointDataProperties<ArcStr>> {
    fn from(value: VecWrap<DisjointDataProperties>) -> Self {
        Into::<Vec<horned_owl::model::DisjointDataProperties<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DisjointDataProperties<ArcStr>>> for VecWrap<DisjointDataProperties> {
    fn from(value: Vec<horned_owl::model::DisjointDataProperties<ArcStr>>) -> Self {
        Into::<VecWrap<DisjointDataProperties>>::into(value.borrow())
    }
}

impl From<&VecWrap<DisjointDataProperties>> for Vec<horned_owl::model::DisjointDataProperties<ArcStr>> {
    fn from(value: &VecWrap<DisjointDataProperties>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DisjointDataProperties::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DisjointDataProperties<ArcStr>>> for VecWrap<DisjointDataProperties> {
    fn from(value: &Vec<horned_owl::model::DisjointDataProperties<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DisjointDataProperties::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DisjointDataProperties>> for Box<horned_owl::model::DisjointDataProperties<ArcStr>> {
    fn from_c(value: &BoxWrap<DisjointDataProperties>) -> Self {
        Box::<horned_owl::model::DisjointDataProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DisjointDataProperties<ArcStr>>> for BoxWrap<DisjointDataProperties> {
    fn from_c(value: &Box<horned_owl::model::DisjointDataProperties<ArcStr>>) -> Self {
        BoxWrap::<DisjointDataProperties>::from(value)
    }
}
impl FromCompatible<BoxWrap<DisjointDataProperties>> for Box<horned_owl::model::DisjointDataProperties<ArcStr>> {
    fn from_c(value: BoxWrap<DisjointDataProperties>) -> Self {
        Box::<horned_owl::model::DisjointDataProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DisjointDataProperties<ArcStr>>> for BoxWrap<DisjointDataProperties> {
    fn from_c(value: Box<horned_owl::model::DisjointDataProperties<ArcStr>>) -> Self {
        BoxWrap::<DisjointDataProperties>::from(value)
    }
}
impl FromCompatible<VecWrap<DisjointDataProperties>> for Vec<horned_owl::model::DisjointDataProperties<ArcStr>> {
    fn from_c(value: VecWrap<DisjointDataProperties>) -> Self {
        Vec::<horned_owl::model::DisjointDataProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DisjointDataProperties<ArcStr>>> for VecWrap<DisjointDataProperties> {
    fn from_c(value: Vec<horned_owl::model::DisjointDataProperties<ArcStr>>) -> Self {
        VecWrap::<DisjointDataProperties>::from(value)
    }
}
impl FromCompatible<&VecWrap<DisjointDataProperties>> for Vec<horned_owl::model::DisjointDataProperties<ArcStr>> {
    fn from_c(value: &VecWrap<DisjointDataProperties>) -> Self {
        Vec::<horned_owl::model::DisjointDataProperties<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DisjointDataProperties<ArcStr>>> for VecWrap<DisjointDataProperties> {
    fn from_c(value: &Vec<horned_owl::model::DisjointDataProperties<ArcStr>>) -> Self {
        VecWrap::<DisjointDataProperties>::from(value)
    }
}
#[doc = concat!("DataPropertyDomain(dp: DataProperty,ce: ClassExpression,)",
    "\n\n",
    doc!(DataPropertyDomain)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataPropertyDomain {
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
    
        #[doc="ce: ClassExpression"]
        #[pyo3(get,set)]
        pub ce: ClassExpression,
    }

#[pymethods]
impl DataPropertyDomain {
    #[new]
    fn new(dp: DataProperty,ce: ClassExpression,) -> Self {
        DataPropertyDomain {
                dp,
                ce,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any),
            "ce" => self.ce.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "dp" => {
                self.dp = value.extract()?;
                Ok(())
            },
            "ce" => {
                self.ce = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DataPropertyDomain<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DataPropertyDomain<ArcStr>> for DataPropertyDomain {
    fn from(value: &horned_owl::model::DataPropertyDomain<ArcStr>) -> Self {
        DataPropertyDomain {
            dp: IntoCompatible::<DataProperty>::into_c(value.dp.borrow()),
            ce: IntoCompatible::<ClassExpression>::into_c(value.ce.borrow()),
        }
    }
}


impl From<&DataPropertyDomain> for horned_owl::model::DataPropertyDomain<ArcStr> {
    fn from(value: &DataPropertyDomain) -> Self {
        horned_owl::model::DataPropertyDomain::<ArcStr> {
            dp: value.dp.borrow().into_c(),
            ce: value.ce.borrow().into_c(),
        }
    }
}



/**************** Base implementations for DataPropertyDomain ****************/
impl FromCompatible<horned_owl::model::DataPropertyDomain<ArcStr>> for DataPropertyDomain {
    fn from_c(value: horned_owl::model::DataPropertyDomain<ArcStr>) -> Self {
        DataPropertyDomain::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DataPropertyDomain<ArcStr>> for DataPropertyDomain {
    fn from_c(value: &horned_owl::model::DataPropertyDomain<ArcStr>) -> Self {
        DataPropertyDomain::from(value)
    }
}

impl FromCompatible<DataPropertyDomain> for horned_owl::model::DataPropertyDomain<ArcStr> {
    fn from_c(value: DataPropertyDomain) -> Self {
        horned_owl::model::DataPropertyDomain::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DataPropertyDomain> for horned_owl::model::DataPropertyDomain<ArcStr> {
    fn from_c(value: &DataPropertyDomain) -> Self {
        horned_owl::model::DataPropertyDomain::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DataPropertyDomain<ArcStr>> for DataPropertyDomain {
    fn from(value: horned_owl::model::DataPropertyDomain<ArcStr>) -> Self {
        DataPropertyDomain::from(value.borrow())
    }
}

impl From<DataPropertyDomain> for horned_owl::model::DataPropertyDomain<ArcStr> {
    fn from(value: DataPropertyDomain) -> Self {
        horned_owl::model::DataPropertyDomain::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DataPropertyDomain>> for Box<horned_owl::model::DataPropertyDomain<ArcStr>> {
    fn from(value: &BoxWrap<DataPropertyDomain>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DataPropertyDomain<ArcStr>>> for BoxWrap<DataPropertyDomain> {
    fn from(value: &Box<horned_owl::model::DataPropertyDomain<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DataPropertyDomain>::into(*value.clone())))
    }
}

impl From<BoxWrap<DataPropertyDomain>> for Box<horned_owl::model::DataPropertyDomain<ArcStr>> {
    fn from(value: BoxWrap<DataPropertyDomain>) -> Self {
        Into::<Box<horned_owl::model::DataPropertyDomain<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DataPropertyDomain<ArcStr>>> for BoxWrap<DataPropertyDomain> {
    fn from(value: Box<horned_owl::model::DataPropertyDomain<ArcStr>>) -> Self {
        Into::<BoxWrap<DataPropertyDomain>>::into(value.borrow())
    }
}

impl From<VecWrap<DataPropertyDomain>> for Vec<horned_owl::model::DataPropertyDomain<ArcStr>> {
    fn from(value: VecWrap<DataPropertyDomain>) -> Self {
        Into::<Vec<horned_owl::model::DataPropertyDomain<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DataPropertyDomain<ArcStr>>> for VecWrap<DataPropertyDomain> {
    fn from(value: Vec<horned_owl::model::DataPropertyDomain<ArcStr>>) -> Self {
        Into::<VecWrap<DataPropertyDomain>>::into(value.borrow())
    }
}

impl From<&VecWrap<DataPropertyDomain>> for Vec<horned_owl::model::DataPropertyDomain<ArcStr>> {
    fn from(value: &VecWrap<DataPropertyDomain>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DataPropertyDomain::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DataPropertyDomain<ArcStr>>> for VecWrap<DataPropertyDomain> {
    fn from(value: &Vec<horned_owl::model::DataPropertyDomain<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DataPropertyDomain::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DataPropertyDomain>> for Box<horned_owl::model::DataPropertyDomain<ArcStr>> {
    fn from_c(value: &BoxWrap<DataPropertyDomain>) -> Self {
        Box::<horned_owl::model::DataPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DataPropertyDomain<ArcStr>>> for BoxWrap<DataPropertyDomain> {
    fn from_c(value: &Box<horned_owl::model::DataPropertyDomain<ArcStr>>) -> Self {
        BoxWrap::<DataPropertyDomain>::from(value)
    }
}
impl FromCompatible<BoxWrap<DataPropertyDomain>> for Box<horned_owl::model::DataPropertyDomain<ArcStr>> {
    fn from_c(value: BoxWrap<DataPropertyDomain>) -> Self {
        Box::<horned_owl::model::DataPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DataPropertyDomain<ArcStr>>> for BoxWrap<DataPropertyDomain> {
    fn from_c(value: Box<horned_owl::model::DataPropertyDomain<ArcStr>>) -> Self {
        BoxWrap::<DataPropertyDomain>::from(value)
    }
}
impl FromCompatible<VecWrap<DataPropertyDomain>> for Vec<horned_owl::model::DataPropertyDomain<ArcStr>> {
    fn from_c(value: VecWrap<DataPropertyDomain>) -> Self {
        Vec::<horned_owl::model::DataPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DataPropertyDomain<ArcStr>>> for VecWrap<DataPropertyDomain> {
    fn from_c(value: Vec<horned_owl::model::DataPropertyDomain<ArcStr>>) -> Self {
        VecWrap::<DataPropertyDomain>::from(value)
    }
}
impl FromCompatible<&VecWrap<DataPropertyDomain>> for Vec<horned_owl::model::DataPropertyDomain<ArcStr>> {
    fn from_c(value: &VecWrap<DataPropertyDomain>) -> Self {
        Vec::<horned_owl::model::DataPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DataPropertyDomain<ArcStr>>> for VecWrap<DataPropertyDomain> {
    fn from_c(value: &Vec<horned_owl::model::DataPropertyDomain<ArcStr>>) -> Self {
        VecWrap::<DataPropertyDomain>::from(value)
    }
}
#[doc = concat!("DataPropertyRange(dp: DataProperty,dr: DataRange,)",
    "\n\n",
    doc!(DataPropertyRange)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataPropertyRange {
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
    
        #[doc="dr: DataRange"]
        #[pyo3(get,set)]
        pub dr: DataRange,
    }

#[pymethods]
impl DataPropertyRange {
    #[new]
    fn new(dp: DataProperty,dr: DataRange,) -> Self {
        DataPropertyRange {
                dp,
                dr,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any),
            "dr" => self.dr.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "dp" => {
                self.dp = value.extract()?;
                Ok(())
            },
            "dr" => {
                self.dr = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DataPropertyRange<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DataPropertyRange<ArcStr>> for DataPropertyRange {
    fn from(value: &horned_owl::model::DataPropertyRange<ArcStr>) -> Self {
        DataPropertyRange {
            dp: IntoCompatible::<DataProperty>::into_c(value.dp.borrow()),
            dr: IntoCompatible::<DataRange>::into_c(value.dr.borrow()),
        }
    }
}


impl From<&DataPropertyRange> for horned_owl::model::DataPropertyRange<ArcStr> {
    fn from(value: &DataPropertyRange) -> Self {
        horned_owl::model::DataPropertyRange::<ArcStr> {
            dp: value.dp.borrow().into_c(),
            dr: value.dr.borrow().into_c(),
        }
    }
}



/**************** Base implementations for DataPropertyRange ****************/
impl FromCompatible<horned_owl::model::DataPropertyRange<ArcStr>> for DataPropertyRange {
    fn from_c(value: horned_owl::model::DataPropertyRange<ArcStr>) -> Self {
        DataPropertyRange::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DataPropertyRange<ArcStr>> for DataPropertyRange {
    fn from_c(value: &horned_owl::model::DataPropertyRange<ArcStr>) -> Self {
        DataPropertyRange::from(value)
    }
}

impl FromCompatible<DataPropertyRange> for horned_owl::model::DataPropertyRange<ArcStr> {
    fn from_c(value: DataPropertyRange) -> Self {
        horned_owl::model::DataPropertyRange::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DataPropertyRange> for horned_owl::model::DataPropertyRange<ArcStr> {
    fn from_c(value: &DataPropertyRange) -> Self {
        horned_owl::model::DataPropertyRange::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DataPropertyRange<ArcStr>> for DataPropertyRange {
    fn from(value: horned_owl::model::DataPropertyRange<ArcStr>) -> Self {
        DataPropertyRange::from(value.borrow())
    }
}

impl From<DataPropertyRange> for horned_owl::model::DataPropertyRange<ArcStr> {
    fn from(value: DataPropertyRange) -> Self {
        horned_owl::model::DataPropertyRange::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DataPropertyRange>> for Box<horned_owl::model::DataPropertyRange<ArcStr>> {
    fn from(value: &BoxWrap<DataPropertyRange>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DataPropertyRange<ArcStr>>> for BoxWrap<DataPropertyRange> {
    fn from(value: &Box<horned_owl::model::DataPropertyRange<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DataPropertyRange>::into(*value.clone())))
    }
}

impl From<BoxWrap<DataPropertyRange>> for Box<horned_owl::model::DataPropertyRange<ArcStr>> {
    fn from(value: BoxWrap<DataPropertyRange>) -> Self {
        Into::<Box<horned_owl::model::DataPropertyRange<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DataPropertyRange<ArcStr>>> for BoxWrap<DataPropertyRange> {
    fn from(value: Box<horned_owl::model::DataPropertyRange<ArcStr>>) -> Self {
        Into::<BoxWrap<DataPropertyRange>>::into(value.borrow())
    }
}

impl From<VecWrap<DataPropertyRange>> for Vec<horned_owl::model::DataPropertyRange<ArcStr>> {
    fn from(value: VecWrap<DataPropertyRange>) -> Self {
        Into::<Vec<horned_owl::model::DataPropertyRange<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DataPropertyRange<ArcStr>>> for VecWrap<DataPropertyRange> {
    fn from(value: Vec<horned_owl::model::DataPropertyRange<ArcStr>>) -> Self {
        Into::<VecWrap<DataPropertyRange>>::into(value.borrow())
    }
}

impl From<&VecWrap<DataPropertyRange>> for Vec<horned_owl::model::DataPropertyRange<ArcStr>> {
    fn from(value: &VecWrap<DataPropertyRange>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DataPropertyRange::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DataPropertyRange<ArcStr>>> for VecWrap<DataPropertyRange> {
    fn from(value: &Vec<horned_owl::model::DataPropertyRange<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DataPropertyRange::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DataPropertyRange>> for Box<horned_owl::model::DataPropertyRange<ArcStr>> {
    fn from_c(value: &BoxWrap<DataPropertyRange>) -> Self {
        Box::<horned_owl::model::DataPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DataPropertyRange<ArcStr>>> for BoxWrap<DataPropertyRange> {
    fn from_c(value: &Box<horned_owl::model::DataPropertyRange<ArcStr>>) -> Self {
        BoxWrap::<DataPropertyRange>::from(value)
    }
}
impl FromCompatible<BoxWrap<DataPropertyRange>> for Box<horned_owl::model::DataPropertyRange<ArcStr>> {
    fn from_c(value: BoxWrap<DataPropertyRange>) -> Self {
        Box::<horned_owl::model::DataPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DataPropertyRange<ArcStr>>> for BoxWrap<DataPropertyRange> {
    fn from_c(value: Box<horned_owl::model::DataPropertyRange<ArcStr>>) -> Self {
        BoxWrap::<DataPropertyRange>::from(value)
    }
}
impl FromCompatible<VecWrap<DataPropertyRange>> for Vec<horned_owl::model::DataPropertyRange<ArcStr>> {
    fn from_c(value: VecWrap<DataPropertyRange>) -> Self {
        Vec::<horned_owl::model::DataPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DataPropertyRange<ArcStr>>> for VecWrap<DataPropertyRange> {
    fn from_c(value: Vec<horned_owl::model::DataPropertyRange<ArcStr>>) -> Self {
        VecWrap::<DataPropertyRange>::from(value)
    }
}
impl FromCompatible<&VecWrap<DataPropertyRange>> for Vec<horned_owl::model::DataPropertyRange<ArcStr>> {
    fn from_c(value: &VecWrap<DataPropertyRange>) -> Self {
        Vec::<horned_owl::model::DataPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DataPropertyRange<ArcStr>>> for VecWrap<DataPropertyRange> {
    fn from_c(value: &Vec<horned_owl::model::DataPropertyRange<ArcStr>>) -> Self {
        VecWrap::<DataPropertyRange>::from(value)
    }
}
#[doc = concat!(
    "FunctionalDataProperty(first: DataProperty,)",
    "\n\n",
    doc!(FunctionalDataProperty)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FunctionalDataProperty (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub DataProperty,
);

#[pymethods]
impl FunctionalDataProperty {
    #[new]
    fn new(first: DataProperty,) -> Self {
        FunctionalDataProperty (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::FunctionalDataProperty<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::FunctionalDataProperty<ArcStr>> for FunctionalDataProperty {
    fn from(value: &horned_owl::model::FunctionalDataProperty<ArcStr>) -> Self {

        FunctionalDataProperty (
    IntoCompatible::<DataProperty>::into_c(&value.0),
        )
    }
}

impl From<&FunctionalDataProperty> for horned_owl::model::FunctionalDataProperty<ArcStr> {
    fn from(value: &FunctionalDataProperty) -> Self {
        horned_owl::model::FunctionalDataProperty::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for FunctionalDataProperty ****************/
impl FromCompatible<horned_owl::model::FunctionalDataProperty<ArcStr>> for FunctionalDataProperty {
    fn from_c(value: horned_owl::model::FunctionalDataProperty<ArcStr>) -> Self {
        FunctionalDataProperty::from(value)
    }
}

impl FromCompatible<&horned_owl::model::FunctionalDataProperty<ArcStr>> for FunctionalDataProperty {
    fn from_c(value: &horned_owl::model::FunctionalDataProperty<ArcStr>) -> Self {
        FunctionalDataProperty::from(value)
    }
}

impl FromCompatible<FunctionalDataProperty> for horned_owl::model::FunctionalDataProperty<ArcStr> {
    fn from_c(value: FunctionalDataProperty) -> Self {
        horned_owl::model::FunctionalDataProperty::<ArcStr>::from(value)
    }
}

impl FromCompatible<&FunctionalDataProperty> for horned_owl::model::FunctionalDataProperty<ArcStr> {
    fn from_c(value: &FunctionalDataProperty) -> Self {
        horned_owl::model::FunctionalDataProperty::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::FunctionalDataProperty<ArcStr>> for FunctionalDataProperty {
    fn from(value: horned_owl::model::FunctionalDataProperty<ArcStr>) -> Self {
        FunctionalDataProperty::from(value.borrow())
    }
}

impl From<FunctionalDataProperty> for horned_owl::model::FunctionalDataProperty<ArcStr> {
    fn from(value: FunctionalDataProperty) -> Self {
        horned_owl::model::FunctionalDataProperty::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<FunctionalDataProperty>> for Box<horned_owl::model::FunctionalDataProperty<ArcStr>> {
    fn from(value: &BoxWrap<FunctionalDataProperty>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::FunctionalDataProperty<ArcStr>>> for BoxWrap<FunctionalDataProperty> {
    fn from(value: &Box<horned_owl::model::FunctionalDataProperty<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<FunctionalDataProperty>::into(*value.clone())))
    }
}

impl From<BoxWrap<FunctionalDataProperty>> for Box<horned_owl::model::FunctionalDataProperty<ArcStr>> {
    fn from(value: BoxWrap<FunctionalDataProperty>) -> Self {
        Into::<Box<horned_owl::model::FunctionalDataProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::FunctionalDataProperty<ArcStr>>> for BoxWrap<FunctionalDataProperty> {
    fn from(value: Box<horned_owl::model::FunctionalDataProperty<ArcStr>>) -> Self {
        Into::<BoxWrap<FunctionalDataProperty>>::into(value.borrow())
    }
}

impl From<VecWrap<FunctionalDataProperty>> for Vec<horned_owl::model::FunctionalDataProperty<ArcStr>> {
    fn from(value: VecWrap<FunctionalDataProperty>) -> Self {
        Into::<Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>> for VecWrap<FunctionalDataProperty> {
    fn from(value: Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>) -> Self {
        Into::<VecWrap<FunctionalDataProperty>>::into(value.borrow())
    }
}

impl From<&VecWrap<FunctionalDataProperty>> for Vec<horned_owl::model::FunctionalDataProperty<ArcStr>> {
    fn from(value: &VecWrap<FunctionalDataProperty>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::FunctionalDataProperty::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>> for VecWrap<FunctionalDataProperty> {
    fn from(value: &Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>) -> Self {
        VecWrap(value.iter().map(FunctionalDataProperty::from).collect())
    }
}

impl FromCompatible<&BoxWrap<FunctionalDataProperty>> for Box<horned_owl::model::FunctionalDataProperty<ArcStr>> {
    fn from_c(value: &BoxWrap<FunctionalDataProperty>) -> Self {
        Box::<horned_owl::model::FunctionalDataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::FunctionalDataProperty<ArcStr>>> for BoxWrap<FunctionalDataProperty> {
    fn from_c(value: &Box<horned_owl::model::FunctionalDataProperty<ArcStr>>) -> Self {
        BoxWrap::<FunctionalDataProperty>::from(value)
    }
}
impl FromCompatible<BoxWrap<FunctionalDataProperty>> for Box<horned_owl::model::FunctionalDataProperty<ArcStr>> {
    fn from_c(value: BoxWrap<FunctionalDataProperty>) -> Self {
        Box::<horned_owl::model::FunctionalDataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::FunctionalDataProperty<ArcStr>>> for BoxWrap<FunctionalDataProperty> {
    fn from_c(value: Box<horned_owl::model::FunctionalDataProperty<ArcStr>>) -> Self {
        BoxWrap::<FunctionalDataProperty>::from(value)
    }
}
impl FromCompatible<VecWrap<FunctionalDataProperty>> for Vec<horned_owl::model::FunctionalDataProperty<ArcStr>> {
    fn from_c(value: VecWrap<FunctionalDataProperty>) -> Self {
        Vec::<horned_owl::model::FunctionalDataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>> for VecWrap<FunctionalDataProperty> {
    fn from_c(value: Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>) -> Self {
        VecWrap::<FunctionalDataProperty>::from(value)
    }
}
impl FromCompatible<&VecWrap<FunctionalDataProperty>> for Vec<horned_owl::model::FunctionalDataProperty<ArcStr>> {
    fn from_c(value: &VecWrap<FunctionalDataProperty>) -> Self {
        Vec::<horned_owl::model::FunctionalDataProperty<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>> for VecWrap<FunctionalDataProperty> {
    fn from_c(value: &Vec<horned_owl::model::FunctionalDataProperty<ArcStr>>) -> Self {
        VecWrap::<FunctionalDataProperty>::from(value)
    }
}
#[doc = concat!("DatatypeDefinition(kind: Datatype,range: DataRange,)",
    "\n\n",
    doc!(DatatypeDefinition)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DatatypeDefinition {
        #[doc="kind: Datatype"]
        #[pyo3(get,set)]
        pub kind: Datatype,
    
        #[doc="range: DataRange"]
        #[pyo3(get,set)]
        pub range: DataRange,
    }

#[pymethods]
impl DatatypeDefinition {
    #[new]
    fn new(kind: Datatype,range: DataRange,) -> Self {
        DatatypeDefinition {
                kind,
                range,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "kind" => self.kind.clone().into_pyobject(py).map(Bound::into_any),
            "range" => self.range.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "kind" => {
                self.kind = value.extract()?;
                Ok(())
            },
            "range" => {
                self.range = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DatatypeDefinition<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DatatypeDefinition<ArcStr>> for DatatypeDefinition {
    fn from(value: &horned_owl::model::DatatypeDefinition<ArcStr>) -> Self {
        DatatypeDefinition {
            kind: IntoCompatible::<Datatype>::into_c(value.kind.borrow()),
            range: IntoCompatible::<DataRange>::into_c(value.range.borrow()),
        }
    }
}


impl From<&DatatypeDefinition> for horned_owl::model::DatatypeDefinition<ArcStr> {
    fn from(value: &DatatypeDefinition) -> Self {
        horned_owl::model::DatatypeDefinition::<ArcStr> {
            kind: value.kind.borrow().into_c(),
            range: value.range.borrow().into_c(),
        }
    }
}



/**************** Base implementations for DatatypeDefinition ****************/
impl FromCompatible<horned_owl::model::DatatypeDefinition<ArcStr>> for DatatypeDefinition {
    fn from_c(value: horned_owl::model::DatatypeDefinition<ArcStr>) -> Self {
        DatatypeDefinition::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DatatypeDefinition<ArcStr>> for DatatypeDefinition {
    fn from_c(value: &horned_owl::model::DatatypeDefinition<ArcStr>) -> Self {
        DatatypeDefinition::from(value)
    }
}

impl FromCompatible<DatatypeDefinition> for horned_owl::model::DatatypeDefinition<ArcStr> {
    fn from_c(value: DatatypeDefinition) -> Self {
        horned_owl::model::DatatypeDefinition::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DatatypeDefinition> for horned_owl::model::DatatypeDefinition<ArcStr> {
    fn from_c(value: &DatatypeDefinition) -> Self {
        horned_owl::model::DatatypeDefinition::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DatatypeDefinition<ArcStr>> for DatatypeDefinition {
    fn from(value: horned_owl::model::DatatypeDefinition<ArcStr>) -> Self {
        DatatypeDefinition::from(value.borrow())
    }
}

impl From<DatatypeDefinition> for horned_owl::model::DatatypeDefinition<ArcStr> {
    fn from(value: DatatypeDefinition) -> Self {
        horned_owl::model::DatatypeDefinition::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DatatypeDefinition>> for Box<horned_owl::model::DatatypeDefinition<ArcStr>> {
    fn from(value: &BoxWrap<DatatypeDefinition>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DatatypeDefinition<ArcStr>>> for BoxWrap<DatatypeDefinition> {
    fn from(value: &Box<horned_owl::model::DatatypeDefinition<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DatatypeDefinition>::into(*value.clone())))
    }
}

impl From<BoxWrap<DatatypeDefinition>> for Box<horned_owl::model::DatatypeDefinition<ArcStr>> {
    fn from(value: BoxWrap<DatatypeDefinition>) -> Self {
        Into::<Box<horned_owl::model::DatatypeDefinition<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DatatypeDefinition<ArcStr>>> for BoxWrap<DatatypeDefinition> {
    fn from(value: Box<horned_owl::model::DatatypeDefinition<ArcStr>>) -> Self {
        Into::<BoxWrap<DatatypeDefinition>>::into(value.borrow())
    }
}

impl From<VecWrap<DatatypeDefinition>> for Vec<horned_owl::model::DatatypeDefinition<ArcStr>> {
    fn from(value: VecWrap<DatatypeDefinition>) -> Self {
        Into::<Vec<horned_owl::model::DatatypeDefinition<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DatatypeDefinition<ArcStr>>> for VecWrap<DatatypeDefinition> {
    fn from(value: Vec<horned_owl::model::DatatypeDefinition<ArcStr>>) -> Self {
        Into::<VecWrap<DatatypeDefinition>>::into(value.borrow())
    }
}

impl From<&VecWrap<DatatypeDefinition>> for Vec<horned_owl::model::DatatypeDefinition<ArcStr>> {
    fn from(value: &VecWrap<DatatypeDefinition>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DatatypeDefinition::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DatatypeDefinition<ArcStr>>> for VecWrap<DatatypeDefinition> {
    fn from(value: &Vec<horned_owl::model::DatatypeDefinition<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DatatypeDefinition::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DatatypeDefinition>> for Box<horned_owl::model::DatatypeDefinition<ArcStr>> {
    fn from_c(value: &BoxWrap<DatatypeDefinition>) -> Self {
        Box::<horned_owl::model::DatatypeDefinition<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DatatypeDefinition<ArcStr>>> for BoxWrap<DatatypeDefinition> {
    fn from_c(value: &Box<horned_owl::model::DatatypeDefinition<ArcStr>>) -> Self {
        BoxWrap::<DatatypeDefinition>::from(value)
    }
}
impl FromCompatible<BoxWrap<DatatypeDefinition>> for Box<horned_owl::model::DatatypeDefinition<ArcStr>> {
    fn from_c(value: BoxWrap<DatatypeDefinition>) -> Self {
        Box::<horned_owl::model::DatatypeDefinition<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DatatypeDefinition<ArcStr>>> for BoxWrap<DatatypeDefinition> {
    fn from_c(value: Box<horned_owl::model::DatatypeDefinition<ArcStr>>) -> Self {
        BoxWrap::<DatatypeDefinition>::from(value)
    }
}
impl FromCompatible<VecWrap<DatatypeDefinition>> for Vec<horned_owl::model::DatatypeDefinition<ArcStr>> {
    fn from_c(value: VecWrap<DatatypeDefinition>) -> Self {
        Vec::<horned_owl::model::DatatypeDefinition<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DatatypeDefinition<ArcStr>>> for VecWrap<DatatypeDefinition> {
    fn from_c(value: Vec<horned_owl::model::DatatypeDefinition<ArcStr>>) -> Self {
        VecWrap::<DatatypeDefinition>::from(value)
    }
}
impl FromCompatible<&VecWrap<DatatypeDefinition>> for Vec<horned_owl::model::DatatypeDefinition<ArcStr>> {
    fn from_c(value: &VecWrap<DatatypeDefinition>) -> Self {
        Vec::<horned_owl::model::DatatypeDefinition<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DatatypeDefinition<ArcStr>>> for VecWrap<DatatypeDefinition> {
    fn from_c(value: &Vec<horned_owl::model::DatatypeDefinition<ArcStr>>) -> Self {
        VecWrap::<DatatypeDefinition>::from(value)
    }
}
#[doc = concat!("HasKey(ce: ClassExpression,vpe: typing.List[PropertyExpression],)",
    "\n\n",
    doc!(HasKey)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HasKey {
        #[doc="ce: ClassExpression"]
        #[pyo3(get,set)]
        pub ce: ClassExpression,
    
        #[doc="vpe: typing.List[PropertyExpression]"]
        #[pyo3(get,set)]
        pub vpe: VecWrap<PropertyExpression>,
    }

#[pymethods]
impl HasKey {
    #[new]
    fn new(ce: ClassExpression,vpe: VecWrap<PropertyExpression>,) -> Self {
        HasKey {
                ce,
                vpe,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ce" => self.ce.clone().into_pyobject(py).map(Bound::into_any),
            "vpe" => self.vpe.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ce" => {
                self.ce = value.extract()?;
                Ok(())
            },
            "vpe" => {
                self.vpe = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::HasKey<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::HasKey<ArcStr>> for HasKey {
    fn from(value: &horned_owl::model::HasKey<ArcStr>) -> Self {
        HasKey {
            ce: IntoCompatible::<ClassExpression>::into_c(value.ce.borrow()),
            vpe: IntoCompatible::<VecWrap<PropertyExpression>>::into_c(value.vpe.borrow()),
        }
    }
}


impl From<&HasKey> for horned_owl::model::HasKey<ArcStr> {
    fn from(value: &HasKey) -> Self {
        horned_owl::model::HasKey::<ArcStr> {
            ce: value.ce.borrow().into_c(),
            vpe: value.vpe.borrow().into_c(),
        }
    }
}



/**************** Base implementations for HasKey ****************/
impl FromCompatible<horned_owl::model::HasKey<ArcStr>> for HasKey {
    fn from_c(value: horned_owl::model::HasKey<ArcStr>) -> Self {
        HasKey::from(value)
    }
}

impl FromCompatible<&horned_owl::model::HasKey<ArcStr>> for HasKey {
    fn from_c(value: &horned_owl::model::HasKey<ArcStr>) -> Self {
        HasKey::from(value)
    }
}

impl FromCompatible<HasKey> for horned_owl::model::HasKey<ArcStr> {
    fn from_c(value: HasKey) -> Self {
        horned_owl::model::HasKey::<ArcStr>::from(value)
    }
}

impl FromCompatible<&HasKey> for horned_owl::model::HasKey<ArcStr> {
    fn from_c(value: &HasKey) -> Self {
        horned_owl::model::HasKey::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::HasKey<ArcStr>> for HasKey {
    fn from(value: horned_owl::model::HasKey<ArcStr>) -> Self {
        HasKey::from(value.borrow())
    }
}

impl From<HasKey> for horned_owl::model::HasKey<ArcStr> {
    fn from(value: HasKey) -> Self {
        horned_owl::model::HasKey::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<HasKey>> for Box<horned_owl::model::HasKey<ArcStr>> {
    fn from(value: &BoxWrap<HasKey>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::HasKey<ArcStr>>> for BoxWrap<HasKey> {
    fn from(value: &Box<horned_owl::model::HasKey<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<HasKey>::into(*value.clone())))
    }
}

impl From<BoxWrap<HasKey>> for Box<horned_owl::model::HasKey<ArcStr>> {
    fn from(value: BoxWrap<HasKey>) -> Self {
        Into::<Box<horned_owl::model::HasKey<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::HasKey<ArcStr>>> for BoxWrap<HasKey> {
    fn from(value: Box<horned_owl::model::HasKey<ArcStr>>) -> Self {
        Into::<BoxWrap<HasKey>>::into(value.borrow())
    }
}

impl From<VecWrap<HasKey>> for Vec<horned_owl::model::HasKey<ArcStr>> {
    fn from(value: VecWrap<HasKey>) -> Self {
        Into::<Vec<horned_owl::model::HasKey<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::HasKey<ArcStr>>> for VecWrap<HasKey> {
    fn from(value: Vec<horned_owl::model::HasKey<ArcStr>>) -> Self {
        Into::<VecWrap<HasKey>>::into(value.borrow())
    }
}

impl From<&VecWrap<HasKey>> for Vec<horned_owl::model::HasKey<ArcStr>> {
    fn from(value: &VecWrap<HasKey>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::HasKey::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::HasKey<ArcStr>>> for VecWrap<HasKey> {
    fn from(value: &Vec<horned_owl::model::HasKey<ArcStr>>) -> Self {
        VecWrap(value.iter().map(HasKey::from).collect())
    }
}

impl FromCompatible<&BoxWrap<HasKey>> for Box<horned_owl::model::HasKey<ArcStr>> {
    fn from_c(value: &BoxWrap<HasKey>) -> Self {
        Box::<horned_owl::model::HasKey<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::HasKey<ArcStr>>> for BoxWrap<HasKey> {
    fn from_c(value: &Box<horned_owl::model::HasKey<ArcStr>>) -> Self {
        BoxWrap::<HasKey>::from(value)
    }
}
impl FromCompatible<BoxWrap<HasKey>> for Box<horned_owl::model::HasKey<ArcStr>> {
    fn from_c(value: BoxWrap<HasKey>) -> Self {
        Box::<horned_owl::model::HasKey<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::HasKey<ArcStr>>> for BoxWrap<HasKey> {
    fn from_c(value: Box<horned_owl::model::HasKey<ArcStr>>) -> Self {
        BoxWrap::<HasKey>::from(value)
    }
}
impl FromCompatible<VecWrap<HasKey>> for Vec<horned_owl::model::HasKey<ArcStr>> {
    fn from_c(value: VecWrap<HasKey>) -> Self {
        Vec::<horned_owl::model::HasKey<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::HasKey<ArcStr>>> for VecWrap<HasKey> {
    fn from_c(value: Vec<horned_owl::model::HasKey<ArcStr>>) -> Self {
        VecWrap::<HasKey>::from(value)
    }
}
impl FromCompatible<&VecWrap<HasKey>> for Vec<horned_owl::model::HasKey<ArcStr>> {
    fn from_c(value: &VecWrap<HasKey>) -> Self {
        Vec::<horned_owl::model::HasKey<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::HasKey<ArcStr>>> for VecWrap<HasKey> {
    fn from_c(value: &Vec<horned_owl::model::HasKey<ArcStr>>) -> Self {
        VecWrap::<HasKey>::from(value)
    }
}
#[doc = concat!(
    "SameIndividual(first: typing.List[Individual],)",
    "\n\n",
    doc!(SameIndividual)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SameIndividual (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub VecWrap<Individual>,
);

#[pymethods]
impl SameIndividual {
    #[new]
    fn new(first: VecWrap<Individual>,) -> Self {
        SameIndividual (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::SameIndividual<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::SameIndividual<ArcStr>> for SameIndividual {
    fn from(value: &horned_owl::model::SameIndividual<ArcStr>) -> Self {

        SameIndividual (
    IntoCompatible::<VecWrap<Individual>>::into_c(&value.0),
        )
    }
}

impl From<&SameIndividual> for horned_owl::model::SameIndividual<ArcStr> {
    fn from(value: &SameIndividual) -> Self {
        horned_owl::model::SameIndividual::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for SameIndividual ****************/
impl FromCompatible<horned_owl::model::SameIndividual<ArcStr>> for SameIndividual {
    fn from_c(value: horned_owl::model::SameIndividual<ArcStr>) -> Self {
        SameIndividual::from(value)
    }
}

impl FromCompatible<&horned_owl::model::SameIndividual<ArcStr>> for SameIndividual {
    fn from_c(value: &horned_owl::model::SameIndividual<ArcStr>) -> Self {
        SameIndividual::from(value)
    }
}

impl FromCompatible<SameIndividual> for horned_owl::model::SameIndividual<ArcStr> {
    fn from_c(value: SameIndividual) -> Self {
        horned_owl::model::SameIndividual::<ArcStr>::from(value)
    }
}

impl FromCompatible<&SameIndividual> for horned_owl::model::SameIndividual<ArcStr> {
    fn from_c(value: &SameIndividual) -> Self {
        horned_owl::model::SameIndividual::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::SameIndividual<ArcStr>> for SameIndividual {
    fn from(value: horned_owl::model::SameIndividual<ArcStr>) -> Self {
        SameIndividual::from(value.borrow())
    }
}

impl From<SameIndividual> for horned_owl::model::SameIndividual<ArcStr> {
    fn from(value: SameIndividual) -> Self {
        horned_owl::model::SameIndividual::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<SameIndividual>> for Box<horned_owl::model::SameIndividual<ArcStr>> {
    fn from(value: &BoxWrap<SameIndividual>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::SameIndividual<ArcStr>>> for BoxWrap<SameIndividual> {
    fn from(value: &Box<horned_owl::model::SameIndividual<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<SameIndividual>::into(*value.clone())))
    }
}

impl From<BoxWrap<SameIndividual>> for Box<horned_owl::model::SameIndividual<ArcStr>> {
    fn from(value: BoxWrap<SameIndividual>) -> Self {
        Into::<Box<horned_owl::model::SameIndividual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::SameIndividual<ArcStr>>> for BoxWrap<SameIndividual> {
    fn from(value: Box<horned_owl::model::SameIndividual<ArcStr>>) -> Self {
        Into::<BoxWrap<SameIndividual>>::into(value.borrow())
    }
}

impl From<VecWrap<SameIndividual>> for Vec<horned_owl::model::SameIndividual<ArcStr>> {
    fn from(value: VecWrap<SameIndividual>) -> Self {
        Into::<Vec<horned_owl::model::SameIndividual<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::SameIndividual<ArcStr>>> for VecWrap<SameIndividual> {
    fn from(value: Vec<horned_owl::model::SameIndividual<ArcStr>>) -> Self {
        Into::<VecWrap<SameIndividual>>::into(value.borrow())
    }
}

impl From<&VecWrap<SameIndividual>> for Vec<horned_owl::model::SameIndividual<ArcStr>> {
    fn from(value: &VecWrap<SameIndividual>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::SameIndividual::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::SameIndividual<ArcStr>>> for VecWrap<SameIndividual> {
    fn from(value: &Vec<horned_owl::model::SameIndividual<ArcStr>>) -> Self {
        VecWrap(value.iter().map(SameIndividual::from).collect())
    }
}

impl FromCompatible<&BoxWrap<SameIndividual>> for Box<horned_owl::model::SameIndividual<ArcStr>> {
    fn from_c(value: &BoxWrap<SameIndividual>) -> Self {
        Box::<horned_owl::model::SameIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::SameIndividual<ArcStr>>> for BoxWrap<SameIndividual> {
    fn from_c(value: &Box<horned_owl::model::SameIndividual<ArcStr>>) -> Self {
        BoxWrap::<SameIndividual>::from(value)
    }
}
impl FromCompatible<BoxWrap<SameIndividual>> for Box<horned_owl::model::SameIndividual<ArcStr>> {
    fn from_c(value: BoxWrap<SameIndividual>) -> Self {
        Box::<horned_owl::model::SameIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::SameIndividual<ArcStr>>> for BoxWrap<SameIndividual> {
    fn from_c(value: Box<horned_owl::model::SameIndividual<ArcStr>>) -> Self {
        BoxWrap::<SameIndividual>::from(value)
    }
}
impl FromCompatible<VecWrap<SameIndividual>> for Vec<horned_owl::model::SameIndividual<ArcStr>> {
    fn from_c(value: VecWrap<SameIndividual>) -> Self {
        Vec::<horned_owl::model::SameIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::SameIndividual<ArcStr>>> for VecWrap<SameIndividual> {
    fn from_c(value: Vec<horned_owl::model::SameIndividual<ArcStr>>) -> Self {
        VecWrap::<SameIndividual>::from(value)
    }
}
impl FromCompatible<&VecWrap<SameIndividual>> for Vec<horned_owl::model::SameIndividual<ArcStr>> {
    fn from_c(value: &VecWrap<SameIndividual>) -> Self {
        Vec::<horned_owl::model::SameIndividual<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::SameIndividual<ArcStr>>> for VecWrap<SameIndividual> {
    fn from_c(value: &Vec<horned_owl::model::SameIndividual<ArcStr>>) -> Self {
        VecWrap::<SameIndividual>::from(value)
    }
}
#[doc = concat!(
    "DifferentIndividuals(first: typing.List[Individual],)",
    "\n\n",
    doc!(DifferentIndividuals)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DifferentIndividuals (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub VecWrap<Individual>,
);

#[pymethods]
impl DifferentIndividuals {
    #[new]
    fn new(first: VecWrap<Individual>,) -> Self {
        DifferentIndividuals (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DifferentIndividuals<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DifferentIndividuals<ArcStr>> for DifferentIndividuals {
    fn from(value: &horned_owl::model::DifferentIndividuals<ArcStr>) -> Self {

        DifferentIndividuals (
    IntoCompatible::<VecWrap<Individual>>::into_c(&value.0),
        )
    }
}

impl From<&DifferentIndividuals> for horned_owl::model::DifferentIndividuals<ArcStr> {
    fn from(value: &DifferentIndividuals) -> Self {
        horned_owl::model::DifferentIndividuals::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DifferentIndividuals ****************/
impl FromCompatible<horned_owl::model::DifferentIndividuals<ArcStr>> for DifferentIndividuals {
    fn from_c(value: horned_owl::model::DifferentIndividuals<ArcStr>) -> Self {
        DifferentIndividuals::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DifferentIndividuals<ArcStr>> for DifferentIndividuals {
    fn from_c(value: &horned_owl::model::DifferentIndividuals<ArcStr>) -> Self {
        DifferentIndividuals::from(value)
    }
}

impl FromCompatible<DifferentIndividuals> for horned_owl::model::DifferentIndividuals<ArcStr> {
    fn from_c(value: DifferentIndividuals) -> Self {
        horned_owl::model::DifferentIndividuals::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DifferentIndividuals> for horned_owl::model::DifferentIndividuals<ArcStr> {
    fn from_c(value: &DifferentIndividuals) -> Self {
        horned_owl::model::DifferentIndividuals::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DifferentIndividuals<ArcStr>> for DifferentIndividuals {
    fn from(value: horned_owl::model::DifferentIndividuals<ArcStr>) -> Self {
        DifferentIndividuals::from(value.borrow())
    }
}

impl From<DifferentIndividuals> for horned_owl::model::DifferentIndividuals<ArcStr> {
    fn from(value: DifferentIndividuals) -> Self {
        horned_owl::model::DifferentIndividuals::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DifferentIndividuals>> for Box<horned_owl::model::DifferentIndividuals<ArcStr>> {
    fn from(value: &BoxWrap<DifferentIndividuals>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DifferentIndividuals<ArcStr>>> for BoxWrap<DifferentIndividuals> {
    fn from(value: &Box<horned_owl::model::DifferentIndividuals<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DifferentIndividuals>::into(*value.clone())))
    }
}

impl From<BoxWrap<DifferentIndividuals>> for Box<horned_owl::model::DifferentIndividuals<ArcStr>> {
    fn from(value: BoxWrap<DifferentIndividuals>) -> Self {
        Into::<Box<horned_owl::model::DifferentIndividuals<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DifferentIndividuals<ArcStr>>> for BoxWrap<DifferentIndividuals> {
    fn from(value: Box<horned_owl::model::DifferentIndividuals<ArcStr>>) -> Self {
        Into::<BoxWrap<DifferentIndividuals>>::into(value.borrow())
    }
}

impl From<VecWrap<DifferentIndividuals>> for Vec<horned_owl::model::DifferentIndividuals<ArcStr>> {
    fn from(value: VecWrap<DifferentIndividuals>) -> Self {
        Into::<Vec<horned_owl::model::DifferentIndividuals<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DifferentIndividuals<ArcStr>>> for VecWrap<DifferentIndividuals> {
    fn from(value: Vec<horned_owl::model::DifferentIndividuals<ArcStr>>) -> Self {
        Into::<VecWrap<DifferentIndividuals>>::into(value.borrow())
    }
}

impl From<&VecWrap<DifferentIndividuals>> for Vec<horned_owl::model::DifferentIndividuals<ArcStr>> {
    fn from(value: &VecWrap<DifferentIndividuals>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DifferentIndividuals::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DifferentIndividuals<ArcStr>>> for VecWrap<DifferentIndividuals> {
    fn from(value: &Vec<horned_owl::model::DifferentIndividuals<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DifferentIndividuals::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DifferentIndividuals>> for Box<horned_owl::model::DifferentIndividuals<ArcStr>> {
    fn from_c(value: &BoxWrap<DifferentIndividuals>) -> Self {
        Box::<horned_owl::model::DifferentIndividuals<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DifferentIndividuals<ArcStr>>> for BoxWrap<DifferentIndividuals> {
    fn from_c(value: &Box<horned_owl::model::DifferentIndividuals<ArcStr>>) -> Self {
        BoxWrap::<DifferentIndividuals>::from(value)
    }
}
impl FromCompatible<BoxWrap<DifferentIndividuals>> for Box<horned_owl::model::DifferentIndividuals<ArcStr>> {
    fn from_c(value: BoxWrap<DifferentIndividuals>) -> Self {
        Box::<horned_owl::model::DifferentIndividuals<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DifferentIndividuals<ArcStr>>> for BoxWrap<DifferentIndividuals> {
    fn from_c(value: Box<horned_owl::model::DifferentIndividuals<ArcStr>>) -> Self {
        BoxWrap::<DifferentIndividuals>::from(value)
    }
}
impl FromCompatible<VecWrap<DifferentIndividuals>> for Vec<horned_owl::model::DifferentIndividuals<ArcStr>> {
    fn from_c(value: VecWrap<DifferentIndividuals>) -> Self {
        Vec::<horned_owl::model::DifferentIndividuals<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DifferentIndividuals<ArcStr>>> for VecWrap<DifferentIndividuals> {
    fn from_c(value: Vec<horned_owl::model::DifferentIndividuals<ArcStr>>) -> Self {
        VecWrap::<DifferentIndividuals>::from(value)
    }
}
impl FromCompatible<&VecWrap<DifferentIndividuals>> for Vec<horned_owl::model::DifferentIndividuals<ArcStr>> {
    fn from_c(value: &VecWrap<DifferentIndividuals>) -> Self {
        Vec::<horned_owl::model::DifferentIndividuals<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DifferentIndividuals<ArcStr>>> for VecWrap<DifferentIndividuals> {
    fn from_c(value: &Vec<horned_owl::model::DifferentIndividuals<ArcStr>>) -> Self {
        VecWrap::<DifferentIndividuals>::from(value)
    }
}
#[doc = concat!("ClassAssertion(ce: ClassExpression,i: Individual,)",
    "\n\n",
    doc!(ClassAssertion)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassAssertion {
        #[doc="ce: ClassExpression"]
        #[pyo3(get,set)]
        pub ce: ClassExpression,
    
        #[doc="i: Individual"]
        #[pyo3(get,set)]
        pub i: Individual,
    }

#[pymethods]
impl ClassAssertion {
    #[new]
    fn new(ce: ClassExpression,i: Individual,) -> Self {
        ClassAssertion {
                ce,
                i,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ce" => self.ce.clone().into_pyobject(py).map(Bound::into_any),
            "i" => self.i.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ce" => {
                self.ce = value.extract()?;
                Ok(())
            },
            "i" => {
                self.i = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::ClassAssertion<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::ClassAssertion<ArcStr>> for ClassAssertion {
    fn from(value: &horned_owl::model::ClassAssertion<ArcStr>) -> Self {
        ClassAssertion {
            ce: IntoCompatible::<ClassExpression>::into_c(value.ce.borrow()),
            i: IntoCompatible::<Individual>::into_c(value.i.borrow()),
        }
    }
}


impl From<&ClassAssertion> for horned_owl::model::ClassAssertion<ArcStr> {
    fn from(value: &ClassAssertion) -> Self {
        horned_owl::model::ClassAssertion::<ArcStr> {
            ce: value.ce.borrow().into_c(),
            i: value.i.borrow().into_c(),
        }
    }
}



/**************** Base implementations for ClassAssertion ****************/
impl FromCompatible<horned_owl::model::ClassAssertion<ArcStr>> for ClassAssertion {
    fn from_c(value: horned_owl::model::ClassAssertion<ArcStr>) -> Self {
        ClassAssertion::from(value)
    }
}

impl FromCompatible<&horned_owl::model::ClassAssertion<ArcStr>> for ClassAssertion {
    fn from_c(value: &horned_owl::model::ClassAssertion<ArcStr>) -> Self {
        ClassAssertion::from(value)
    }
}

impl FromCompatible<ClassAssertion> for horned_owl::model::ClassAssertion<ArcStr> {
    fn from_c(value: ClassAssertion) -> Self {
        horned_owl::model::ClassAssertion::<ArcStr>::from(value)
    }
}

impl FromCompatible<&ClassAssertion> for horned_owl::model::ClassAssertion<ArcStr> {
    fn from_c(value: &ClassAssertion) -> Self {
        horned_owl::model::ClassAssertion::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::ClassAssertion<ArcStr>> for ClassAssertion {
    fn from(value: horned_owl::model::ClassAssertion<ArcStr>) -> Self {
        ClassAssertion::from(value.borrow())
    }
}

impl From<ClassAssertion> for horned_owl::model::ClassAssertion<ArcStr> {
    fn from(value: ClassAssertion) -> Self {
        horned_owl::model::ClassAssertion::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<ClassAssertion>> for Box<horned_owl::model::ClassAssertion<ArcStr>> {
    fn from(value: &BoxWrap<ClassAssertion>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::ClassAssertion<ArcStr>>> for BoxWrap<ClassAssertion> {
    fn from(value: &Box<horned_owl::model::ClassAssertion<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<ClassAssertion>::into(*value.clone())))
    }
}

impl From<BoxWrap<ClassAssertion>> for Box<horned_owl::model::ClassAssertion<ArcStr>> {
    fn from(value: BoxWrap<ClassAssertion>) -> Self {
        Into::<Box<horned_owl::model::ClassAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::ClassAssertion<ArcStr>>> for BoxWrap<ClassAssertion> {
    fn from(value: Box<horned_owl::model::ClassAssertion<ArcStr>>) -> Self {
        Into::<BoxWrap<ClassAssertion>>::into(value.borrow())
    }
}

impl From<VecWrap<ClassAssertion>> for Vec<horned_owl::model::ClassAssertion<ArcStr>> {
    fn from(value: VecWrap<ClassAssertion>) -> Self {
        Into::<Vec<horned_owl::model::ClassAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::ClassAssertion<ArcStr>>> for VecWrap<ClassAssertion> {
    fn from(value: Vec<horned_owl::model::ClassAssertion<ArcStr>>) -> Self {
        Into::<VecWrap<ClassAssertion>>::into(value.borrow())
    }
}

impl From<&VecWrap<ClassAssertion>> for Vec<horned_owl::model::ClassAssertion<ArcStr>> {
    fn from(value: &VecWrap<ClassAssertion>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::ClassAssertion::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::ClassAssertion<ArcStr>>> for VecWrap<ClassAssertion> {
    fn from(value: &Vec<horned_owl::model::ClassAssertion<ArcStr>>) -> Self {
        VecWrap(value.iter().map(ClassAssertion::from).collect())
    }
}

impl FromCompatible<&BoxWrap<ClassAssertion>> for Box<horned_owl::model::ClassAssertion<ArcStr>> {
    fn from_c(value: &BoxWrap<ClassAssertion>) -> Self {
        Box::<horned_owl::model::ClassAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::ClassAssertion<ArcStr>>> for BoxWrap<ClassAssertion> {
    fn from_c(value: &Box<horned_owl::model::ClassAssertion<ArcStr>>) -> Self {
        BoxWrap::<ClassAssertion>::from(value)
    }
}
impl FromCompatible<BoxWrap<ClassAssertion>> for Box<horned_owl::model::ClassAssertion<ArcStr>> {
    fn from_c(value: BoxWrap<ClassAssertion>) -> Self {
        Box::<horned_owl::model::ClassAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::ClassAssertion<ArcStr>>> for BoxWrap<ClassAssertion> {
    fn from_c(value: Box<horned_owl::model::ClassAssertion<ArcStr>>) -> Self {
        BoxWrap::<ClassAssertion>::from(value)
    }
}
impl FromCompatible<VecWrap<ClassAssertion>> for Vec<horned_owl::model::ClassAssertion<ArcStr>> {
    fn from_c(value: VecWrap<ClassAssertion>) -> Self {
        Vec::<horned_owl::model::ClassAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::ClassAssertion<ArcStr>>> for VecWrap<ClassAssertion> {
    fn from_c(value: Vec<horned_owl::model::ClassAssertion<ArcStr>>) -> Self {
        VecWrap::<ClassAssertion>::from(value)
    }
}
impl FromCompatible<&VecWrap<ClassAssertion>> for Vec<horned_owl::model::ClassAssertion<ArcStr>> {
    fn from_c(value: &VecWrap<ClassAssertion>) -> Self {
        Vec::<horned_owl::model::ClassAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::ClassAssertion<ArcStr>>> for VecWrap<ClassAssertion> {
    fn from_c(value: &Vec<horned_owl::model::ClassAssertion<ArcStr>>) -> Self {
        VecWrap::<ClassAssertion>::from(value)
    }
}
#[doc = concat!("ObjectPropertyAssertion(ope: ObjectPropertyExpression,source: Individual,target: Individual,)",
    "\n\n",
    doc!(ObjectPropertyAssertion)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectPropertyAssertion {
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
    
        #[doc="source: Individual"]
        #[pyo3(get,set)]
        pub source: Individual,
    
        #[doc="target: Individual"]
        #[pyo3(get,set)]
        pub target: Individual,
    }

#[pymethods]
impl ObjectPropertyAssertion {
    #[new]
    fn new(ope: ObjectPropertyExpression,source: Individual,target: Individual,) -> Self {
        ObjectPropertyAssertion {
                ope,
                source,
                target,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any),
            "source" => self.source.clone().into_pyobject(py).map(Bound::into_any),
            "target" => self.target.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ope" => {
                self.ope = value.extract()?;
                Ok(())
            },
            "source" => {
                self.source = value.extract()?;
                Ok(())
            },
            "target" => {
                self.target = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::ObjectPropertyAssertion<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::ObjectPropertyAssertion<ArcStr>> for ObjectPropertyAssertion {
    fn from(value: &horned_owl::model::ObjectPropertyAssertion<ArcStr>) -> Self {
        ObjectPropertyAssertion {
            ope: IntoCompatible::<ObjectPropertyExpression>::into_c(value.ope.borrow()),
            source: IntoCompatible::<Individual>::into_c(value.from.borrow()),
            target: IntoCompatible::<Individual>::into_c(value.to.borrow()),
        }
    }
}


impl From<&ObjectPropertyAssertion> for horned_owl::model::ObjectPropertyAssertion<ArcStr> {
    fn from(value: &ObjectPropertyAssertion) -> Self {
        horned_owl::model::ObjectPropertyAssertion::<ArcStr> {
            ope: value.ope.borrow().into_c(),
            from: value.source.borrow().into_c(),
            to: value.target.borrow().into_c(),
        }
    }
}



/**************** Base implementations for ObjectPropertyAssertion ****************/
impl FromCompatible<horned_owl::model::ObjectPropertyAssertion<ArcStr>> for ObjectPropertyAssertion {
    fn from_c(value: horned_owl::model::ObjectPropertyAssertion<ArcStr>) -> Self {
        ObjectPropertyAssertion::from(value)
    }
}

impl FromCompatible<&horned_owl::model::ObjectPropertyAssertion<ArcStr>> for ObjectPropertyAssertion {
    fn from_c(value: &horned_owl::model::ObjectPropertyAssertion<ArcStr>) -> Self {
        ObjectPropertyAssertion::from(value)
    }
}

impl FromCompatible<ObjectPropertyAssertion> for horned_owl::model::ObjectPropertyAssertion<ArcStr> {
    fn from_c(value: ObjectPropertyAssertion) -> Self {
        horned_owl::model::ObjectPropertyAssertion::<ArcStr>::from(value)
    }
}

impl FromCompatible<&ObjectPropertyAssertion> for horned_owl::model::ObjectPropertyAssertion<ArcStr> {
    fn from_c(value: &ObjectPropertyAssertion) -> Self {
        horned_owl::model::ObjectPropertyAssertion::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::ObjectPropertyAssertion<ArcStr>> for ObjectPropertyAssertion {
    fn from(value: horned_owl::model::ObjectPropertyAssertion<ArcStr>) -> Self {
        ObjectPropertyAssertion::from(value.borrow())
    }
}

impl From<ObjectPropertyAssertion> for horned_owl::model::ObjectPropertyAssertion<ArcStr> {
    fn from(value: ObjectPropertyAssertion) -> Self {
        horned_owl::model::ObjectPropertyAssertion::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<ObjectPropertyAssertion>> for Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>> {
    fn from(value: &BoxWrap<ObjectPropertyAssertion>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>> for BoxWrap<ObjectPropertyAssertion> {
    fn from(value: &Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<ObjectPropertyAssertion>::into(*value.clone())))
    }
}

impl From<BoxWrap<ObjectPropertyAssertion>> for Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>> {
    fn from(value: BoxWrap<ObjectPropertyAssertion>) -> Self {
        Into::<Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>> for BoxWrap<ObjectPropertyAssertion> {
    fn from(value: Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>) -> Self {
        Into::<BoxWrap<ObjectPropertyAssertion>>::into(value.borrow())
    }
}

impl From<VecWrap<ObjectPropertyAssertion>> for Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>> {
    fn from(value: VecWrap<ObjectPropertyAssertion>) -> Self {
        Into::<Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>> for VecWrap<ObjectPropertyAssertion> {
    fn from(value: Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>) -> Self {
        Into::<VecWrap<ObjectPropertyAssertion>>::into(value.borrow())
    }
}

impl From<&VecWrap<ObjectPropertyAssertion>> for Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>> {
    fn from(value: &VecWrap<ObjectPropertyAssertion>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::ObjectPropertyAssertion::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>> for VecWrap<ObjectPropertyAssertion> {
    fn from(value: &Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>) -> Self {
        VecWrap(value.iter().map(ObjectPropertyAssertion::from).collect())
    }
}

impl FromCompatible<&BoxWrap<ObjectPropertyAssertion>> for Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>> {
    fn from_c(value: &BoxWrap<ObjectPropertyAssertion>) -> Self {
        Box::<horned_owl::model::ObjectPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>> for BoxWrap<ObjectPropertyAssertion> {
    fn from_c(value: &Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap::<ObjectPropertyAssertion>::from(value)
    }
}
impl FromCompatible<BoxWrap<ObjectPropertyAssertion>> for Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>> {
    fn from_c(value: BoxWrap<ObjectPropertyAssertion>) -> Self {
        Box::<horned_owl::model::ObjectPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>> for BoxWrap<ObjectPropertyAssertion> {
    fn from_c(value: Box<horned_owl::model::ObjectPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap::<ObjectPropertyAssertion>::from(value)
    }
}
impl FromCompatible<VecWrap<ObjectPropertyAssertion>> for Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>> {
    fn from_c(value: VecWrap<ObjectPropertyAssertion>) -> Self {
        Vec::<horned_owl::model::ObjectPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>> for VecWrap<ObjectPropertyAssertion> {
    fn from_c(value: Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>) -> Self {
        VecWrap::<ObjectPropertyAssertion>::from(value)
    }
}
impl FromCompatible<&VecWrap<ObjectPropertyAssertion>> for Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>> {
    fn from_c(value: &VecWrap<ObjectPropertyAssertion>) -> Self {
        Vec::<horned_owl::model::ObjectPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>> for VecWrap<ObjectPropertyAssertion> {
    fn from_c(value: &Vec<horned_owl::model::ObjectPropertyAssertion<ArcStr>>) -> Self {
        VecWrap::<ObjectPropertyAssertion>::from(value)
    }
}
#[doc = concat!("NegativeObjectPropertyAssertion(ope: ObjectPropertyExpression,source: Individual,target: Individual,)",
    "\n\n",
    doc!(NegativeObjectPropertyAssertion)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NegativeObjectPropertyAssertion {
        #[doc="ope: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub ope: ObjectPropertyExpression,
    
        #[doc="source: Individual"]
        #[pyo3(get,set)]
        pub source: Individual,
    
        #[doc="target: Individual"]
        #[pyo3(get,set)]
        pub target: Individual,
    }

#[pymethods]
impl NegativeObjectPropertyAssertion {
    #[new]
    fn new(ope: ObjectPropertyExpression,source: Individual,target: Individual,) -> Self {
        NegativeObjectPropertyAssertion {
                ope,
                source,
                target,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ope" => self.ope.clone().into_pyobject(py).map(Bound::into_any),
            "source" => self.source.clone().into_pyobject(py).map(Bound::into_any),
            "target" => self.target.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ope" => {
                self.ope = value.extract()?;
                Ok(())
            },
            "source" => {
                self.source = value.extract()?;
                Ok(())
            },
            "target" => {
                self.target = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> for NegativeObjectPropertyAssertion {
    fn from(value: &horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>) -> Self {
        NegativeObjectPropertyAssertion {
            ope: IntoCompatible::<ObjectPropertyExpression>::into_c(value.ope.borrow()),
            source: IntoCompatible::<Individual>::into_c(value.from.borrow()),
            target: IntoCompatible::<Individual>::into_c(value.to.borrow()),
        }
    }
}


impl From<&NegativeObjectPropertyAssertion> for horned_owl::model::NegativeObjectPropertyAssertion<ArcStr> {
    fn from(value: &NegativeObjectPropertyAssertion) -> Self {
        horned_owl::model::NegativeObjectPropertyAssertion::<ArcStr> {
            ope: value.ope.borrow().into_c(),
            from: value.source.borrow().into_c(),
            to: value.target.borrow().into_c(),
        }
    }
}



/**************** Base implementations for NegativeObjectPropertyAssertion ****************/
impl FromCompatible<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> for NegativeObjectPropertyAssertion {
    fn from_c(value: horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>) -> Self {
        NegativeObjectPropertyAssertion::from(value)
    }
}

impl FromCompatible<&horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> for NegativeObjectPropertyAssertion {
    fn from_c(value: &horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>) -> Self {
        NegativeObjectPropertyAssertion::from(value)
    }
}

impl FromCompatible<NegativeObjectPropertyAssertion> for horned_owl::model::NegativeObjectPropertyAssertion<ArcStr> {
    fn from_c(value: NegativeObjectPropertyAssertion) -> Self {
        horned_owl::model::NegativeObjectPropertyAssertion::<ArcStr>::from(value)
    }
}

impl FromCompatible<&NegativeObjectPropertyAssertion> for horned_owl::model::NegativeObjectPropertyAssertion<ArcStr> {
    fn from_c(value: &NegativeObjectPropertyAssertion) -> Self {
        horned_owl::model::NegativeObjectPropertyAssertion::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> for NegativeObjectPropertyAssertion {
    fn from(value: horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>) -> Self {
        NegativeObjectPropertyAssertion::from(value.borrow())
    }
}

impl From<NegativeObjectPropertyAssertion> for horned_owl::model::NegativeObjectPropertyAssertion<ArcStr> {
    fn from(value: NegativeObjectPropertyAssertion) -> Self {
        horned_owl::model::NegativeObjectPropertyAssertion::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<NegativeObjectPropertyAssertion>> for Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> {
    fn from(value: &BoxWrap<NegativeObjectPropertyAssertion>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>> for BoxWrap<NegativeObjectPropertyAssertion> {
    fn from(value: &Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<NegativeObjectPropertyAssertion>::into(*value.clone())))
    }
}

impl From<BoxWrap<NegativeObjectPropertyAssertion>> for Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> {
    fn from(value: BoxWrap<NegativeObjectPropertyAssertion>) -> Self {
        Into::<Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>> for BoxWrap<NegativeObjectPropertyAssertion> {
    fn from(value: Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>) -> Self {
        Into::<BoxWrap<NegativeObjectPropertyAssertion>>::into(value.borrow())
    }
}

impl From<VecWrap<NegativeObjectPropertyAssertion>> for Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> {
    fn from(value: VecWrap<NegativeObjectPropertyAssertion>) -> Self {
        Into::<Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>> for VecWrap<NegativeObjectPropertyAssertion> {
    fn from(value: Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>) -> Self {
        Into::<VecWrap<NegativeObjectPropertyAssertion>>::into(value.borrow())
    }
}

impl From<&VecWrap<NegativeObjectPropertyAssertion>> for Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> {
    fn from(value: &VecWrap<NegativeObjectPropertyAssertion>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::NegativeObjectPropertyAssertion::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>> for VecWrap<NegativeObjectPropertyAssertion> {
    fn from(value: &Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>) -> Self {
        VecWrap(value.iter().map(NegativeObjectPropertyAssertion::from).collect())
    }
}

impl FromCompatible<&BoxWrap<NegativeObjectPropertyAssertion>> for Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> {
    fn from_c(value: &BoxWrap<NegativeObjectPropertyAssertion>) -> Self {
        Box::<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>> for BoxWrap<NegativeObjectPropertyAssertion> {
    fn from_c(value: &Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap::<NegativeObjectPropertyAssertion>::from(value)
    }
}
impl FromCompatible<BoxWrap<NegativeObjectPropertyAssertion>> for Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> {
    fn from_c(value: BoxWrap<NegativeObjectPropertyAssertion>) -> Self {
        Box::<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>> for BoxWrap<NegativeObjectPropertyAssertion> {
    fn from_c(value: Box<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap::<NegativeObjectPropertyAssertion>::from(value)
    }
}
impl FromCompatible<VecWrap<NegativeObjectPropertyAssertion>> for Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> {
    fn from_c(value: VecWrap<NegativeObjectPropertyAssertion>) -> Self {
        Vec::<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>> for VecWrap<NegativeObjectPropertyAssertion> {
    fn from_c(value: Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>) -> Self {
        VecWrap::<NegativeObjectPropertyAssertion>::from(value)
    }
}
impl FromCompatible<&VecWrap<NegativeObjectPropertyAssertion>> for Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>> {
    fn from_c(value: &VecWrap<NegativeObjectPropertyAssertion>) -> Self {
        Vec::<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>> for VecWrap<NegativeObjectPropertyAssertion> {
    fn from_c(value: &Vec<horned_owl::model::NegativeObjectPropertyAssertion<ArcStr>>) -> Self {
        VecWrap::<NegativeObjectPropertyAssertion>::from(value)
    }
}
#[doc = concat!("DataPropertyAssertion(dp: DataProperty,source: Individual,target: Literal,)",
    "\n\n",
    doc!(DataPropertyAssertion)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataPropertyAssertion {
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
    
        #[doc="source: Individual"]
        #[pyo3(get,set)]
        pub source: Individual,
    
        #[doc="target: Literal"]
        #[pyo3(get,set)]
        pub target: Literal,
    }

#[pymethods]
impl DataPropertyAssertion {
    #[new]
    fn new(dp: DataProperty,source: Individual,target: Literal,) -> Self {
        DataPropertyAssertion {
                dp,
                source,
                target,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any),
            "source" => self.source.clone().into_pyobject(py).map(Bound::into_any),
            "target" => self.target.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "dp" => {
                self.dp = value.extract()?;
                Ok(())
            },
            "source" => {
                self.source = value.extract()?;
                Ok(())
            },
            "target" => {
                self.target = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DataPropertyAssertion<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DataPropertyAssertion<ArcStr>> for DataPropertyAssertion {
    fn from(value: &horned_owl::model::DataPropertyAssertion<ArcStr>) -> Self {
        DataPropertyAssertion {
            dp: IntoCompatible::<DataProperty>::into_c(value.dp.borrow()),
            source: IntoCompatible::<Individual>::into_c(value.from.borrow()),
            target: IntoCompatible::<Literal>::into_c(value.to.borrow()),
        }
    }
}


impl From<&DataPropertyAssertion> for horned_owl::model::DataPropertyAssertion<ArcStr> {
    fn from(value: &DataPropertyAssertion) -> Self {
        horned_owl::model::DataPropertyAssertion::<ArcStr> {
            dp: value.dp.borrow().into_c(),
            from: value.source.borrow().into_c(),
            to: value.target.borrow().into_c(),
        }
    }
}



/**************** Base implementations for DataPropertyAssertion ****************/
impl FromCompatible<horned_owl::model::DataPropertyAssertion<ArcStr>> for DataPropertyAssertion {
    fn from_c(value: horned_owl::model::DataPropertyAssertion<ArcStr>) -> Self {
        DataPropertyAssertion::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DataPropertyAssertion<ArcStr>> for DataPropertyAssertion {
    fn from_c(value: &horned_owl::model::DataPropertyAssertion<ArcStr>) -> Self {
        DataPropertyAssertion::from(value)
    }
}

impl FromCompatible<DataPropertyAssertion> for horned_owl::model::DataPropertyAssertion<ArcStr> {
    fn from_c(value: DataPropertyAssertion) -> Self {
        horned_owl::model::DataPropertyAssertion::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DataPropertyAssertion> for horned_owl::model::DataPropertyAssertion<ArcStr> {
    fn from_c(value: &DataPropertyAssertion) -> Self {
        horned_owl::model::DataPropertyAssertion::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DataPropertyAssertion<ArcStr>> for DataPropertyAssertion {
    fn from(value: horned_owl::model::DataPropertyAssertion<ArcStr>) -> Self {
        DataPropertyAssertion::from(value.borrow())
    }
}

impl From<DataPropertyAssertion> for horned_owl::model::DataPropertyAssertion<ArcStr> {
    fn from(value: DataPropertyAssertion) -> Self {
        horned_owl::model::DataPropertyAssertion::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DataPropertyAssertion>> for Box<horned_owl::model::DataPropertyAssertion<ArcStr>> {
    fn from(value: &BoxWrap<DataPropertyAssertion>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DataPropertyAssertion<ArcStr>>> for BoxWrap<DataPropertyAssertion> {
    fn from(value: &Box<horned_owl::model::DataPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DataPropertyAssertion>::into(*value.clone())))
    }
}

impl From<BoxWrap<DataPropertyAssertion>> for Box<horned_owl::model::DataPropertyAssertion<ArcStr>> {
    fn from(value: BoxWrap<DataPropertyAssertion>) -> Self {
        Into::<Box<horned_owl::model::DataPropertyAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DataPropertyAssertion<ArcStr>>> for BoxWrap<DataPropertyAssertion> {
    fn from(value: Box<horned_owl::model::DataPropertyAssertion<ArcStr>>) -> Self {
        Into::<BoxWrap<DataPropertyAssertion>>::into(value.borrow())
    }
}

impl From<VecWrap<DataPropertyAssertion>> for Vec<horned_owl::model::DataPropertyAssertion<ArcStr>> {
    fn from(value: VecWrap<DataPropertyAssertion>) -> Self {
        Into::<Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>> for VecWrap<DataPropertyAssertion> {
    fn from(value: Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>) -> Self {
        Into::<VecWrap<DataPropertyAssertion>>::into(value.borrow())
    }
}

impl From<&VecWrap<DataPropertyAssertion>> for Vec<horned_owl::model::DataPropertyAssertion<ArcStr>> {
    fn from(value: &VecWrap<DataPropertyAssertion>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DataPropertyAssertion::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>> for VecWrap<DataPropertyAssertion> {
    fn from(value: &Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DataPropertyAssertion::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DataPropertyAssertion>> for Box<horned_owl::model::DataPropertyAssertion<ArcStr>> {
    fn from_c(value: &BoxWrap<DataPropertyAssertion>) -> Self {
        Box::<horned_owl::model::DataPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DataPropertyAssertion<ArcStr>>> for BoxWrap<DataPropertyAssertion> {
    fn from_c(value: &Box<horned_owl::model::DataPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap::<DataPropertyAssertion>::from(value)
    }
}
impl FromCompatible<BoxWrap<DataPropertyAssertion>> for Box<horned_owl::model::DataPropertyAssertion<ArcStr>> {
    fn from_c(value: BoxWrap<DataPropertyAssertion>) -> Self {
        Box::<horned_owl::model::DataPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DataPropertyAssertion<ArcStr>>> for BoxWrap<DataPropertyAssertion> {
    fn from_c(value: Box<horned_owl::model::DataPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap::<DataPropertyAssertion>::from(value)
    }
}
impl FromCompatible<VecWrap<DataPropertyAssertion>> for Vec<horned_owl::model::DataPropertyAssertion<ArcStr>> {
    fn from_c(value: VecWrap<DataPropertyAssertion>) -> Self {
        Vec::<horned_owl::model::DataPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>> for VecWrap<DataPropertyAssertion> {
    fn from_c(value: Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>) -> Self {
        VecWrap::<DataPropertyAssertion>::from(value)
    }
}
impl FromCompatible<&VecWrap<DataPropertyAssertion>> for Vec<horned_owl::model::DataPropertyAssertion<ArcStr>> {
    fn from_c(value: &VecWrap<DataPropertyAssertion>) -> Self {
        Vec::<horned_owl::model::DataPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>> for VecWrap<DataPropertyAssertion> {
    fn from_c(value: &Vec<horned_owl::model::DataPropertyAssertion<ArcStr>>) -> Self {
        VecWrap::<DataPropertyAssertion>::from(value)
    }
}
#[doc = concat!("NegativeDataPropertyAssertion(dp: DataProperty,source: Individual,target: Literal,)",
    "\n\n",
    doc!(NegativeDataPropertyAssertion)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NegativeDataPropertyAssertion {
        #[doc="dp: DataProperty"]
        #[pyo3(get,set)]
        pub dp: DataProperty,
    
        #[doc="source: Individual"]
        #[pyo3(get,set)]
        pub source: Individual,
    
        #[doc="target: Literal"]
        #[pyo3(get,set)]
        pub target: Literal,
    }

#[pymethods]
impl NegativeDataPropertyAssertion {
    #[new]
    fn new(dp: DataProperty,source: Individual,target: Literal,) -> Self {
        NegativeDataPropertyAssertion {
                dp,
                source,
                target,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "dp" => self.dp.clone().into_pyobject(py).map(Bound::into_any),
            "source" => self.source.clone().into_pyobject(py).map(Bound::into_any),
            "target" => self.target.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "dp" => {
                self.dp = value.extract()?;
                Ok(())
            },
            "source" => {
                self.source = value.extract()?;
                Ok(())
            },
            "target" => {
                self.target = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> for NegativeDataPropertyAssertion {
    fn from(value: &horned_owl::model::NegativeDataPropertyAssertion<ArcStr>) -> Self {
        NegativeDataPropertyAssertion {
            dp: IntoCompatible::<DataProperty>::into_c(value.dp.borrow()),
            source: IntoCompatible::<Individual>::into_c(value.from.borrow()),
            target: IntoCompatible::<Literal>::into_c(value.to.borrow()),
        }
    }
}


impl From<&NegativeDataPropertyAssertion> for horned_owl::model::NegativeDataPropertyAssertion<ArcStr> {
    fn from(value: &NegativeDataPropertyAssertion) -> Self {
        horned_owl::model::NegativeDataPropertyAssertion::<ArcStr> {
            dp: value.dp.borrow().into_c(),
            from: value.source.borrow().into_c(),
            to: value.target.borrow().into_c(),
        }
    }
}



/**************** Base implementations for NegativeDataPropertyAssertion ****************/
impl FromCompatible<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> for NegativeDataPropertyAssertion {
    fn from_c(value: horned_owl::model::NegativeDataPropertyAssertion<ArcStr>) -> Self {
        NegativeDataPropertyAssertion::from(value)
    }
}

impl FromCompatible<&horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> for NegativeDataPropertyAssertion {
    fn from_c(value: &horned_owl::model::NegativeDataPropertyAssertion<ArcStr>) -> Self {
        NegativeDataPropertyAssertion::from(value)
    }
}

impl FromCompatible<NegativeDataPropertyAssertion> for horned_owl::model::NegativeDataPropertyAssertion<ArcStr> {
    fn from_c(value: NegativeDataPropertyAssertion) -> Self {
        horned_owl::model::NegativeDataPropertyAssertion::<ArcStr>::from(value)
    }
}

impl FromCompatible<&NegativeDataPropertyAssertion> for horned_owl::model::NegativeDataPropertyAssertion<ArcStr> {
    fn from_c(value: &NegativeDataPropertyAssertion) -> Self {
        horned_owl::model::NegativeDataPropertyAssertion::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> for NegativeDataPropertyAssertion {
    fn from(value: horned_owl::model::NegativeDataPropertyAssertion<ArcStr>) -> Self {
        NegativeDataPropertyAssertion::from(value.borrow())
    }
}

impl From<NegativeDataPropertyAssertion> for horned_owl::model::NegativeDataPropertyAssertion<ArcStr> {
    fn from(value: NegativeDataPropertyAssertion) -> Self {
        horned_owl::model::NegativeDataPropertyAssertion::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<NegativeDataPropertyAssertion>> for Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> {
    fn from(value: &BoxWrap<NegativeDataPropertyAssertion>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>> for BoxWrap<NegativeDataPropertyAssertion> {
    fn from(value: &Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<NegativeDataPropertyAssertion>::into(*value.clone())))
    }
}

impl From<BoxWrap<NegativeDataPropertyAssertion>> for Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> {
    fn from(value: BoxWrap<NegativeDataPropertyAssertion>) -> Self {
        Into::<Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>> for BoxWrap<NegativeDataPropertyAssertion> {
    fn from(value: Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>) -> Self {
        Into::<BoxWrap<NegativeDataPropertyAssertion>>::into(value.borrow())
    }
}

impl From<VecWrap<NegativeDataPropertyAssertion>> for Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> {
    fn from(value: VecWrap<NegativeDataPropertyAssertion>) -> Self {
        Into::<Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>> for VecWrap<NegativeDataPropertyAssertion> {
    fn from(value: Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>) -> Self {
        Into::<VecWrap<NegativeDataPropertyAssertion>>::into(value.borrow())
    }
}

impl From<&VecWrap<NegativeDataPropertyAssertion>> for Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> {
    fn from(value: &VecWrap<NegativeDataPropertyAssertion>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::NegativeDataPropertyAssertion::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>> for VecWrap<NegativeDataPropertyAssertion> {
    fn from(value: &Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>) -> Self {
        VecWrap(value.iter().map(NegativeDataPropertyAssertion::from).collect())
    }
}

impl FromCompatible<&BoxWrap<NegativeDataPropertyAssertion>> for Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> {
    fn from_c(value: &BoxWrap<NegativeDataPropertyAssertion>) -> Self {
        Box::<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>> for BoxWrap<NegativeDataPropertyAssertion> {
    fn from_c(value: &Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap::<NegativeDataPropertyAssertion>::from(value)
    }
}
impl FromCompatible<BoxWrap<NegativeDataPropertyAssertion>> for Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> {
    fn from_c(value: BoxWrap<NegativeDataPropertyAssertion>) -> Self {
        Box::<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>> for BoxWrap<NegativeDataPropertyAssertion> {
    fn from_c(value: Box<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>) -> Self {
        BoxWrap::<NegativeDataPropertyAssertion>::from(value)
    }
}
impl FromCompatible<VecWrap<NegativeDataPropertyAssertion>> for Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> {
    fn from_c(value: VecWrap<NegativeDataPropertyAssertion>) -> Self {
        Vec::<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>> for VecWrap<NegativeDataPropertyAssertion> {
    fn from_c(value: Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>) -> Self {
        VecWrap::<NegativeDataPropertyAssertion>::from(value)
    }
}
impl FromCompatible<&VecWrap<NegativeDataPropertyAssertion>> for Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>> {
    fn from_c(value: &VecWrap<NegativeDataPropertyAssertion>) -> Self {
        Vec::<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>> for VecWrap<NegativeDataPropertyAssertion> {
    fn from_c(value: &Vec<horned_owl::model::NegativeDataPropertyAssertion<ArcStr>>) -> Self {
        VecWrap::<NegativeDataPropertyAssertion>::from(value)
    }
}
#[doc = concat!("AnnotationAssertion(subject: AnnotationSubject,ann: Annotation,)",
    "\n\n",
    doc!(AnnotationAssertion)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnotationAssertion {
        #[doc="subject: AnnotationSubject"]
        #[pyo3(get,set)]
        pub subject: AnnotationSubject,
    
        #[doc="ann: Annotation"]
        #[pyo3(get,set)]
        pub ann: Annotation,
    }

#[pymethods]
impl AnnotationAssertion {
    #[new]
    fn new(subject: AnnotationSubject,ann: Annotation,) -> Self {
        AnnotationAssertion {
                subject,
                ann,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "subject" => self.subject.clone().into_pyobject(py).map(Bound::into_any),
            "ann" => self.ann.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "subject" => {
                self.subject = value.extract()?;
                Ok(())
            },
            "ann" => {
                self.ann = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::AnnotationAssertion<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::AnnotationAssertion<ArcStr>> for AnnotationAssertion {
    fn from(value: &horned_owl::model::AnnotationAssertion<ArcStr>) -> Self {
        AnnotationAssertion {
            subject: IntoCompatible::<AnnotationSubject>::into_c(value.subject.borrow()),
            ann: IntoCompatible::<Annotation>::into_c(value.ann.borrow()),
        }
    }
}


impl From<&AnnotationAssertion> for horned_owl::model::AnnotationAssertion<ArcStr> {
    fn from(value: &AnnotationAssertion) -> Self {
        horned_owl::model::AnnotationAssertion::<ArcStr> {
            subject: value.subject.borrow().into_c(),
            ann: value.ann.borrow().into_c(),
        }
    }
}



/**************** Base implementations for AnnotationAssertion ****************/
impl FromCompatible<horned_owl::model::AnnotationAssertion<ArcStr>> for AnnotationAssertion {
    fn from_c(value: horned_owl::model::AnnotationAssertion<ArcStr>) -> Self {
        AnnotationAssertion::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AnnotationAssertion<ArcStr>> for AnnotationAssertion {
    fn from_c(value: &horned_owl::model::AnnotationAssertion<ArcStr>) -> Self {
        AnnotationAssertion::from(value)
    }
}

impl FromCompatible<AnnotationAssertion> for horned_owl::model::AnnotationAssertion<ArcStr> {
    fn from_c(value: AnnotationAssertion) -> Self {
        horned_owl::model::AnnotationAssertion::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AnnotationAssertion> for horned_owl::model::AnnotationAssertion<ArcStr> {
    fn from_c(value: &AnnotationAssertion) -> Self {
        horned_owl::model::AnnotationAssertion::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AnnotationAssertion<ArcStr>> for AnnotationAssertion {
    fn from(value: horned_owl::model::AnnotationAssertion<ArcStr>) -> Self {
        AnnotationAssertion::from(value.borrow())
    }
}

impl From<AnnotationAssertion> for horned_owl::model::AnnotationAssertion<ArcStr> {
    fn from(value: AnnotationAssertion) -> Self {
        horned_owl::model::AnnotationAssertion::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AnnotationAssertion>> for Box<horned_owl::model::AnnotationAssertion<ArcStr>> {
    fn from(value: &BoxWrap<AnnotationAssertion>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AnnotationAssertion<ArcStr>>> for BoxWrap<AnnotationAssertion> {
    fn from(value: &Box<horned_owl::model::AnnotationAssertion<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AnnotationAssertion>::into(*value.clone())))
    }
}

impl From<BoxWrap<AnnotationAssertion>> for Box<horned_owl::model::AnnotationAssertion<ArcStr>> {
    fn from(value: BoxWrap<AnnotationAssertion>) -> Self {
        Into::<Box<horned_owl::model::AnnotationAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AnnotationAssertion<ArcStr>>> for BoxWrap<AnnotationAssertion> {
    fn from(value: Box<horned_owl::model::AnnotationAssertion<ArcStr>>) -> Self {
        Into::<BoxWrap<AnnotationAssertion>>::into(value.borrow())
    }
}

impl From<VecWrap<AnnotationAssertion>> for Vec<horned_owl::model::AnnotationAssertion<ArcStr>> {
    fn from(value: VecWrap<AnnotationAssertion>) -> Self {
        Into::<Vec<horned_owl::model::AnnotationAssertion<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AnnotationAssertion<ArcStr>>> for VecWrap<AnnotationAssertion> {
    fn from(value: Vec<horned_owl::model::AnnotationAssertion<ArcStr>>) -> Self {
        Into::<VecWrap<AnnotationAssertion>>::into(value.borrow())
    }
}

impl From<&VecWrap<AnnotationAssertion>> for Vec<horned_owl::model::AnnotationAssertion<ArcStr>> {
    fn from(value: &VecWrap<AnnotationAssertion>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AnnotationAssertion::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AnnotationAssertion<ArcStr>>> for VecWrap<AnnotationAssertion> {
    fn from(value: &Vec<horned_owl::model::AnnotationAssertion<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AnnotationAssertion::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AnnotationAssertion>> for Box<horned_owl::model::AnnotationAssertion<ArcStr>> {
    fn from_c(value: &BoxWrap<AnnotationAssertion>) -> Self {
        Box::<horned_owl::model::AnnotationAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AnnotationAssertion<ArcStr>>> for BoxWrap<AnnotationAssertion> {
    fn from_c(value: &Box<horned_owl::model::AnnotationAssertion<ArcStr>>) -> Self {
        BoxWrap::<AnnotationAssertion>::from(value)
    }
}
impl FromCompatible<BoxWrap<AnnotationAssertion>> for Box<horned_owl::model::AnnotationAssertion<ArcStr>> {
    fn from_c(value: BoxWrap<AnnotationAssertion>) -> Self {
        Box::<horned_owl::model::AnnotationAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AnnotationAssertion<ArcStr>>> for BoxWrap<AnnotationAssertion> {
    fn from_c(value: Box<horned_owl::model::AnnotationAssertion<ArcStr>>) -> Self {
        BoxWrap::<AnnotationAssertion>::from(value)
    }
}
impl FromCompatible<VecWrap<AnnotationAssertion>> for Vec<horned_owl::model::AnnotationAssertion<ArcStr>> {
    fn from_c(value: VecWrap<AnnotationAssertion>) -> Self {
        Vec::<horned_owl::model::AnnotationAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AnnotationAssertion<ArcStr>>> for VecWrap<AnnotationAssertion> {
    fn from_c(value: Vec<horned_owl::model::AnnotationAssertion<ArcStr>>) -> Self {
        VecWrap::<AnnotationAssertion>::from(value)
    }
}
impl FromCompatible<&VecWrap<AnnotationAssertion>> for Vec<horned_owl::model::AnnotationAssertion<ArcStr>> {
    fn from_c(value: &VecWrap<AnnotationAssertion>) -> Self {
        Vec::<horned_owl::model::AnnotationAssertion<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AnnotationAssertion<ArcStr>>> for VecWrap<AnnotationAssertion> {
    fn from_c(value: &Vec<horned_owl::model::AnnotationAssertion<ArcStr>>) -> Self {
        VecWrap::<AnnotationAssertion>::from(value)
    }
}
#[doc = concat!("SubAnnotationPropertyOf(sub: AnnotationProperty,sup: AnnotationProperty,)",
    "\n\n",
    doc!(SubAnnotationPropertyOf)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubAnnotationPropertyOf {
        #[doc="sub: AnnotationProperty"]
        #[pyo3(get,set)]
        pub sub: AnnotationProperty,
    
        #[doc="sup: AnnotationProperty"]
        #[pyo3(get,set)]
        pub sup: AnnotationProperty,
    }

#[pymethods]
impl SubAnnotationPropertyOf {
    #[new]
    fn new(sub: AnnotationProperty,sup: AnnotationProperty,) -> Self {
        SubAnnotationPropertyOf {
                sub,
                sup,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "sub" => self.sub.clone().into_pyobject(py).map(Bound::into_any),
            "sup" => self.sup.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "sub" => {
                self.sub = value.extract()?;
                Ok(())
            },
            "sup" => {
                self.sup = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::SubAnnotationPropertyOf<ArcStr>> for SubAnnotationPropertyOf {
    fn from(value: &horned_owl::model::SubAnnotationPropertyOf<ArcStr>) -> Self {
        SubAnnotationPropertyOf {
            sub: IntoCompatible::<AnnotationProperty>::into_c(value.sub.borrow()),
            sup: IntoCompatible::<AnnotationProperty>::into_c(value.sup.borrow()),
        }
    }
}


impl From<&SubAnnotationPropertyOf> for horned_owl::model::SubAnnotationPropertyOf<ArcStr> {
    fn from(value: &SubAnnotationPropertyOf) -> Self {
        horned_owl::model::SubAnnotationPropertyOf::<ArcStr> {
            sub: value.sub.borrow().into_c(),
            sup: value.sup.borrow().into_c(),
        }
    }
}



/**************** Base implementations for SubAnnotationPropertyOf ****************/
impl FromCompatible<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> for SubAnnotationPropertyOf {
    fn from_c(value: horned_owl::model::SubAnnotationPropertyOf<ArcStr>) -> Self {
        SubAnnotationPropertyOf::from(value)
    }
}

impl FromCompatible<&horned_owl::model::SubAnnotationPropertyOf<ArcStr>> for SubAnnotationPropertyOf {
    fn from_c(value: &horned_owl::model::SubAnnotationPropertyOf<ArcStr>) -> Self {
        SubAnnotationPropertyOf::from(value)
    }
}

impl FromCompatible<SubAnnotationPropertyOf> for horned_owl::model::SubAnnotationPropertyOf<ArcStr> {
    fn from_c(value: SubAnnotationPropertyOf) -> Self {
        horned_owl::model::SubAnnotationPropertyOf::<ArcStr>::from(value)
    }
}

impl FromCompatible<&SubAnnotationPropertyOf> for horned_owl::model::SubAnnotationPropertyOf<ArcStr> {
    fn from_c(value: &SubAnnotationPropertyOf) -> Self {
        horned_owl::model::SubAnnotationPropertyOf::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> for SubAnnotationPropertyOf {
    fn from(value: horned_owl::model::SubAnnotationPropertyOf<ArcStr>) -> Self {
        SubAnnotationPropertyOf::from(value.borrow())
    }
}

impl From<SubAnnotationPropertyOf> for horned_owl::model::SubAnnotationPropertyOf<ArcStr> {
    fn from(value: SubAnnotationPropertyOf) -> Self {
        horned_owl::model::SubAnnotationPropertyOf::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<SubAnnotationPropertyOf>> for Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> {
    fn from(value: &BoxWrap<SubAnnotationPropertyOf>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>> for BoxWrap<SubAnnotationPropertyOf> {
    fn from(value: &Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<SubAnnotationPropertyOf>::into(*value.clone())))
    }
}

impl From<BoxWrap<SubAnnotationPropertyOf>> for Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> {
    fn from(value: BoxWrap<SubAnnotationPropertyOf>) -> Self {
        Into::<Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>> for BoxWrap<SubAnnotationPropertyOf> {
    fn from(value: Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>) -> Self {
        Into::<BoxWrap<SubAnnotationPropertyOf>>::into(value.borrow())
    }
}

impl From<VecWrap<SubAnnotationPropertyOf>> for Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> {
    fn from(value: VecWrap<SubAnnotationPropertyOf>) -> Self {
        Into::<Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>> for VecWrap<SubAnnotationPropertyOf> {
    fn from(value: Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>) -> Self {
        Into::<VecWrap<SubAnnotationPropertyOf>>::into(value.borrow())
    }
}

impl From<&VecWrap<SubAnnotationPropertyOf>> for Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> {
    fn from(value: &VecWrap<SubAnnotationPropertyOf>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::SubAnnotationPropertyOf::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>> for VecWrap<SubAnnotationPropertyOf> {
    fn from(value: &Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>) -> Self {
        VecWrap(value.iter().map(SubAnnotationPropertyOf::from).collect())
    }
}

impl FromCompatible<&BoxWrap<SubAnnotationPropertyOf>> for Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> {
    fn from_c(value: &BoxWrap<SubAnnotationPropertyOf>) -> Self {
        Box::<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>> for BoxWrap<SubAnnotationPropertyOf> {
    fn from_c(value: &Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>) -> Self {
        BoxWrap::<SubAnnotationPropertyOf>::from(value)
    }
}
impl FromCompatible<BoxWrap<SubAnnotationPropertyOf>> for Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> {
    fn from_c(value: BoxWrap<SubAnnotationPropertyOf>) -> Self {
        Box::<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>> for BoxWrap<SubAnnotationPropertyOf> {
    fn from_c(value: Box<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>) -> Self {
        BoxWrap::<SubAnnotationPropertyOf>::from(value)
    }
}
impl FromCompatible<VecWrap<SubAnnotationPropertyOf>> for Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> {
    fn from_c(value: VecWrap<SubAnnotationPropertyOf>) -> Self {
        Vec::<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>> for VecWrap<SubAnnotationPropertyOf> {
    fn from_c(value: Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>) -> Self {
        VecWrap::<SubAnnotationPropertyOf>::from(value)
    }
}
impl FromCompatible<&VecWrap<SubAnnotationPropertyOf>> for Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>> {
    fn from_c(value: &VecWrap<SubAnnotationPropertyOf>) -> Self {
        Vec::<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>> for VecWrap<SubAnnotationPropertyOf> {
    fn from_c(value: &Vec<horned_owl::model::SubAnnotationPropertyOf<ArcStr>>) -> Self {
        VecWrap::<SubAnnotationPropertyOf>::from(value)
    }
}
#[doc = concat!("AnnotationPropertyDomain(ap: AnnotationProperty,iri: IRI,)",
    "\n\n",
    doc!(AnnotationPropertyDomain)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnotationPropertyDomain {
        #[doc="ap: AnnotationProperty"]
        #[pyo3(get,set)]
        pub ap: AnnotationProperty,
    
        #[doc="iri: IRI"]
        #[pyo3(get,set)]
        pub iri: IRI,
    }

#[pymethods]
impl AnnotationPropertyDomain {
    #[new]
    fn new(ap: AnnotationProperty,iri: IRI,) -> Self {
        AnnotationPropertyDomain {
                ap,
                iri,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ap" => self.ap.clone().into_pyobject(py).map(Bound::into_any),
            "iri" => self.iri.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ap" => {
                self.ap = value.extract()?;
                Ok(())
            },
            "iri" => {
                self.iri = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::AnnotationPropertyDomain<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::AnnotationPropertyDomain<ArcStr>> for AnnotationPropertyDomain {
    fn from(value: &horned_owl::model::AnnotationPropertyDomain<ArcStr>) -> Self {
        AnnotationPropertyDomain {
            ap: IntoCompatible::<AnnotationProperty>::into_c(value.ap.borrow()),
            iri: IntoCompatible::<IRI>::into_c(value.iri.borrow()),
        }
    }
}


impl From<&AnnotationPropertyDomain> for horned_owl::model::AnnotationPropertyDomain<ArcStr> {
    fn from(value: &AnnotationPropertyDomain) -> Self {
        horned_owl::model::AnnotationPropertyDomain::<ArcStr> {
            ap: value.ap.borrow().into_c(),
            iri: value.iri.borrow().into_c(),
        }
    }
}



/**************** Base implementations for AnnotationPropertyDomain ****************/
impl FromCompatible<horned_owl::model::AnnotationPropertyDomain<ArcStr>> for AnnotationPropertyDomain {
    fn from_c(value: horned_owl::model::AnnotationPropertyDomain<ArcStr>) -> Self {
        AnnotationPropertyDomain::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AnnotationPropertyDomain<ArcStr>> for AnnotationPropertyDomain {
    fn from_c(value: &horned_owl::model::AnnotationPropertyDomain<ArcStr>) -> Self {
        AnnotationPropertyDomain::from(value)
    }
}

impl FromCompatible<AnnotationPropertyDomain> for horned_owl::model::AnnotationPropertyDomain<ArcStr> {
    fn from_c(value: AnnotationPropertyDomain) -> Self {
        horned_owl::model::AnnotationPropertyDomain::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AnnotationPropertyDomain> for horned_owl::model::AnnotationPropertyDomain<ArcStr> {
    fn from_c(value: &AnnotationPropertyDomain) -> Self {
        horned_owl::model::AnnotationPropertyDomain::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AnnotationPropertyDomain<ArcStr>> for AnnotationPropertyDomain {
    fn from(value: horned_owl::model::AnnotationPropertyDomain<ArcStr>) -> Self {
        AnnotationPropertyDomain::from(value.borrow())
    }
}

impl From<AnnotationPropertyDomain> for horned_owl::model::AnnotationPropertyDomain<ArcStr> {
    fn from(value: AnnotationPropertyDomain) -> Self {
        horned_owl::model::AnnotationPropertyDomain::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AnnotationPropertyDomain>> for Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>> {
    fn from(value: &BoxWrap<AnnotationPropertyDomain>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>> for BoxWrap<AnnotationPropertyDomain> {
    fn from(value: &Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AnnotationPropertyDomain>::into(*value.clone())))
    }
}

impl From<BoxWrap<AnnotationPropertyDomain>> for Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>> {
    fn from(value: BoxWrap<AnnotationPropertyDomain>) -> Self {
        Into::<Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>> for BoxWrap<AnnotationPropertyDomain> {
    fn from(value: Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>) -> Self {
        Into::<BoxWrap<AnnotationPropertyDomain>>::into(value.borrow())
    }
}

impl From<VecWrap<AnnotationPropertyDomain>> for Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>> {
    fn from(value: VecWrap<AnnotationPropertyDomain>) -> Self {
        Into::<Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>> for VecWrap<AnnotationPropertyDomain> {
    fn from(value: Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>) -> Self {
        Into::<VecWrap<AnnotationPropertyDomain>>::into(value.borrow())
    }
}

impl From<&VecWrap<AnnotationPropertyDomain>> for Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>> {
    fn from(value: &VecWrap<AnnotationPropertyDomain>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AnnotationPropertyDomain::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>> for VecWrap<AnnotationPropertyDomain> {
    fn from(value: &Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AnnotationPropertyDomain::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AnnotationPropertyDomain>> for Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>> {
    fn from_c(value: &BoxWrap<AnnotationPropertyDomain>) -> Self {
        Box::<horned_owl::model::AnnotationPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>> for BoxWrap<AnnotationPropertyDomain> {
    fn from_c(value: &Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>) -> Self {
        BoxWrap::<AnnotationPropertyDomain>::from(value)
    }
}
impl FromCompatible<BoxWrap<AnnotationPropertyDomain>> for Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>> {
    fn from_c(value: BoxWrap<AnnotationPropertyDomain>) -> Self {
        Box::<horned_owl::model::AnnotationPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>> for BoxWrap<AnnotationPropertyDomain> {
    fn from_c(value: Box<horned_owl::model::AnnotationPropertyDomain<ArcStr>>) -> Self {
        BoxWrap::<AnnotationPropertyDomain>::from(value)
    }
}
impl FromCompatible<VecWrap<AnnotationPropertyDomain>> for Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>> {
    fn from_c(value: VecWrap<AnnotationPropertyDomain>) -> Self {
        Vec::<horned_owl::model::AnnotationPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>> for VecWrap<AnnotationPropertyDomain> {
    fn from_c(value: Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>) -> Self {
        VecWrap::<AnnotationPropertyDomain>::from(value)
    }
}
impl FromCompatible<&VecWrap<AnnotationPropertyDomain>> for Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>> {
    fn from_c(value: &VecWrap<AnnotationPropertyDomain>) -> Self {
        Vec::<horned_owl::model::AnnotationPropertyDomain<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>> for VecWrap<AnnotationPropertyDomain> {
    fn from_c(value: &Vec<horned_owl::model::AnnotationPropertyDomain<ArcStr>>) -> Self {
        VecWrap::<AnnotationPropertyDomain>::from(value)
    }
}
#[doc = concat!("AnnotationPropertyRange(ap: AnnotationProperty,iri: IRI,)",
    "\n\n",
    doc!(AnnotationPropertyRange)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnotationPropertyRange {
        #[doc="ap: AnnotationProperty"]
        #[pyo3(get,set)]
        pub ap: AnnotationProperty,
    
        #[doc="iri: IRI"]
        #[pyo3(get,set)]
        pub iri: IRI,
    }

#[pymethods]
impl AnnotationPropertyRange {
    #[new]
    fn new(ap: AnnotationProperty,iri: IRI,) -> Self {
        AnnotationPropertyRange {
                ap,
                iri,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "ap" => self.ap.clone().into_pyobject(py).map(Bound::into_any),
            "iri" => self.iri.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "ap" => {
                self.ap = value.extract()?;
                Ok(())
            },
            "iri" => {
                self.iri = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::AnnotationPropertyRange<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::AnnotationPropertyRange<ArcStr>> for AnnotationPropertyRange {
    fn from(value: &horned_owl::model::AnnotationPropertyRange<ArcStr>) -> Self {
        AnnotationPropertyRange {
            ap: IntoCompatible::<AnnotationProperty>::into_c(value.ap.borrow()),
            iri: IntoCompatible::<IRI>::into_c(value.iri.borrow()),
        }
    }
}


impl From<&AnnotationPropertyRange> for horned_owl::model::AnnotationPropertyRange<ArcStr> {
    fn from(value: &AnnotationPropertyRange) -> Self {
        horned_owl::model::AnnotationPropertyRange::<ArcStr> {
            ap: value.ap.borrow().into_c(),
            iri: value.iri.borrow().into_c(),
        }
    }
}



/**************** Base implementations for AnnotationPropertyRange ****************/
impl FromCompatible<horned_owl::model::AnnotationPropertyRange<ArcStr>> for AnnotationPropertyRange {
    fn from_c(value: horned_owl::model::AnnotationPropertyRange<ArcStr>) -> Self {
        AnnotationPropertyRange::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AnnotationPropertyRange<ArcStr>> for AnnotationPropertyRange {
    fn from_c(value: &horned_owl::model::AnnotationPropertyRange<ArcStr>) -> Self {
        AnnotationPropertyRange::from(value)
    }
}

impl FromCompatible<AnnotationPropertyRange> for horned_owl::model::AnnotationPropertyRange<ArcStr> {
    fn from_c(value: AnnotationPropertyRange) -> Self {
        horned_owl::model::AnnotationPropertyRange::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AnnotationPropertyRange> for horned_owl::model::AnnotationPropertyRange<ArcStr> {
    fn from_c(value: &AnnotationPropertyRange) -> Self {
        horned_owl::model::AnnotationPropertyRange::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AnnotationPropertyRange<ArcStr>> for AnnotationPropertyRange {
    fn from(value: horned_owl::model::AnnotationPropertyRange<ArcStr>) -> Self {
        AnnotationPropertyRange::from(value.borrow())
    }
}

impl From<AnnotationPropertyRange> for horned_owl::model::AnnotationPropertyRange<ArcStr> {
    fn from(value: AnnotationPropertyRange) -> Self {
        horned_owl::model::AnnotationPropertyRange::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AnnotationPropertyRange>> for Box<horned_owl::model::AnnotationPropertyRange<ArcStr>> {
    fn from(value: &BoxWrap<AnnotationPropertyRange>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>> for BoxWrap<AnnotationPropertyRange> {
    fn from(value: &Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AnnotationPropertyRange>::into(*value.clone())))
    }
}

impl From<BoxWrap<AnnotationPropertyRange>> for Box<horned_owl::model::AnnotationPropertyRange<ArcStr>> {
    fn from(value: BoxWrap<AnnotationPropertyRange>) -> Self {
        Into::<Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>> for BoxWrap<AnnotationPropertyRange> {
    fn from(value: Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>) -> Self {
        Into::<BoxWrap<AnnotationPropertyRange>>::into(value.borrow())
    }
}

impl From<VecWrap<AnnotationPropertyRange>> for Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>> {
    fn from(value: VecWrap<AnnotationPropertyRange>) -> Self {
        Into::<Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>> for VecWrap<AnnotationPropertyRange> {
    fn from(value: Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>) -> Self {
        Into::<VecWrap<AnnotationPropertyRange>>::into(value.borrow())
    }
}

impl From<&VecWrap<AnnotationPropertyRange>> for Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>> {
    fn from(value: &VecWrap<AnnotationPropertyRange>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AnnotationPropertyRange::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>> for VecWrap<AnnotationPropertyRange> {
    fn from(value: &Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AnnotationPropertyRange::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AnnotationPropertyRange>> for Box<horned_owl::model::AnnotationPropertyRange<ArcStr>> {
    fn from_c(value: &BoxWrap<AnnotationPropertyRange>) -> Self {
        Box::<horned_owl::model::AnnotationPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>> for BoxWrap<AnnotationPropertyRange> {
    fn from_c(value: &Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>) -> Self {
        BoxWrap::<AnnotationPropertyRange>::from(value)
    }
}
impl FromCompatible<BoxWrap<AnnotationPropertyRange>> for Box<horned_owl::model::AnnotationPropertyRange<ArcStr>> {
    fn from_c(value: BoxWrap<AnnotationPropertyRange>) -> Self {
        Box::<horned_owl::model::AnnotationPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>> for BoxWrap<AnnotationPropertyRange> {
    fn from_c(value: Box<horned_owl::model::AnnotationPropertyRange<ArcStr>>) -> Self {
        BoxWrap::<AnnotationPropertyRange>::from(value)
    }
}
impl FromCompatible<VecWrap<AnnotationPropertyRange>> for Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>> {
    fn from_c(value: VecWrap<AnnotationPropertyRange>) -> Self {
        Vec::<horned_owl::model::AnnotationPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>> for VecWrap<AnnotationPropertyRange> {
    fn from_c(value: Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>) -> Self {
        VecWrap::<AnnotationPropertyRange>::from(value)
    }
}
impl FromCompatible<&VecWrap<AnnotationPropertyRange>> for Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>> {
    fn from_c(value: &VecWrap<AnnotationPropertyRange>) -> Self {
        Vec::<horned_owl::model::AnnotationPropertyRange<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>> for VecWrap<AnnotationPropertyRange> {
    fn from_c(value: &Vec<horned_owl::model::AnnotationPropertyRange<ArcStr>>) -> Self {
        VecWrap::<AnnotationPropertyRange>::from(value)
    }
}
#[doc = concat!(
    "DocIRI(first: IRI,)",
    "\n\n",
    doc!(DocIRI)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DocIRI (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl DocIRI {
    #[new]
    fn new(first: IRI,) -> Self {
        DocIRI (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::DocIRI<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::DocIRI<ArcStr>> for DocIRI {
    fn from(value: &horned_owl::model::DocIRI<ArcStr>) -> Self {

        DocIRI (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&DocIRI> for horned_owl::model::DocIRI<ArcStr> {
    fn from(value: &DocIRI) -> Self {
        horned_owl::model::DocIRI::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for DocIRI ****************/
impl FromCompatible<horned_owl::model::DocIRI<ArcStr>> for DocIRI {
    fn from_c(value: horned_owl::model::DocIRI<ArcStr>) -> Self {
        DocIRI::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DocIRI<ArcStr>> for DocIRI {
    fn from_c(value: &horned_owl::model::DocIRI<ArcStr>) -> Self {
        DocIRI::from(value)
    }
}

impl FromCompatible<DocIRI> for horned_owl::model::DocIRI<ArcStr> {
    fn from_c(value: DocIRI) -> Self {
        horned_owl::model::DocIRI::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DocIRI> for horned_owl::model::DocIRI<ArcStr> {
    fn from_c(value: &DocIRI) -> Self {
        horned_owl::model::DocIRI::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DocIRI<ArcStr>> for DocIRI {
    fn from(value: horned_owl::model::DocIRI<ArcStr>) -> Self {
        DocIRI::from(value.borrow())
    }
}

impl From<DocIRI> for horned_owl::model::DocIRI<ArcStr> {
    fn from(value: DocIRI) -> Self {
        horned_owl::model::DocIRI::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DocIRI>> for Box<horned_owl::model::DocIRI<ArcStr>> {
    fn from(value: &BoxWrap<DocIRI>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DocIRI<ArcStr>>> for BoxWrap<DocIRI> {
    fn from(value: &Box<horned_owl::model::DocIRI<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DocIRI>::into(*value.clone())))
    }
}

impl From<BoxWrap<DocIRI>> for Box<horned_owl::model::DocIRI<ArcStr>> {
    fn from(value: BoxWrap<DocIRI>) -> Self {
        Into::<Box<horned_owl::model::DocIRI<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DocIRI<ArcStr>>> for BoxWrap<DocIRI> {
    fn from(value: Box<horned_owl::model::DocIRI<ArcStr>>) -> Self {
        Into::<BoxWrap<DocIRI>>::into(value.borrow())
    }
}

impl From<VecWrap<DocIRI>> for Vec<horned_owl::model::DocIRI<ArcStr>> {
    fn from(value: VecWrap<DocIRI>) -> Self {
        Into::<Vec<horned_owl::model::DocIRI<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DocIRI<ArcStr>>> for VecWrap<DocIRI> {
    fn from(value: Vec<horned_owl::model::DocIRI<ArcStr>>) -> Self {
        Into::<VecWrap<DocIRI>>::into(value.borrow())
    }
}

impl From<&VecWrap<DocIRI>> for Vec<horned_owl::model::DocIRI<ArcStr>> {
    fn from(value: &VecWrap<DocIRI>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DocIRI::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DocIRI<ArcStr>>> for VecWrap<DocIRI> {
    fn from(value: &Vec<horned_owl::model::DocIRI<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DocIRI::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DocIRI>> for Box<horned_owl::model::DocIRI<ArcStr>> {
    fn from_c(value: &BoxWrap<DocIRI>) -> Self {
        Box::<horned_owl::model::DocIRI<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DocIRI<ArcStr>>> for BoxWrap<DocIRI> {
    fn from_c(value: &Box<horned_owl::model::DocIRI<ArcStr>>) -> Self {
        BoxWrap::<DocIRI>::from(value)
    }
}
impl FromCompatible<BoxWrap<DocIRI>> for Box<horned_owl::model::DocIRI<ArcStr>> {
    fn from_c(value: BoxWrap<DocIRI>) -> Self {
        Box::<horned_owl::model::DocIRI<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DocIRI<ArcStr>>> for BoxWrap<DocIRI> {
    fn from_c(value: Box<horned_owl::model::DocIRI<ArcStr>>) -> Self {
        BoxWrap::<DocIRI>::from(value)
    }
}
impl FromCompatible<VecWrap<DocIRI>> for Vec<horned_owl::model::DocIRI<ArcStr>> {
    fn from_c(value: VecWrap<DocIRI>) -> Self {
        Vec::<horned_owl::model::DocIRI<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DocIRI<ArcStr>>> for VecWrap<DocIRI> {
    fn from_c(value: Vec<horned_owl::model::DocIRI<ArcStr>>) -> Self {
        VecWrap::<DocIRI>::from(value)
    }
}
impl FromCompatible<&VecWrap<DocIRI>> for Vec<horned_owl::model::DocIRI<ArcStr>> {
    fn from_c(value: &VecWrap<DocIRI>) -> Self {
        Vec::<horned_owl::model::DocIRI<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DocIRI<ArcStr>>> for VecWrap<DocIRI> {
    fn from_c(value: &Vec<horned_owl::model::DocIRI<ArcStr>>) -> Self {
        VecWrap::<DocIRI>::from(value)
    }
}
#[doc = concat!("OntologyID(iri: typing.Optional[IRI],viri: typing.Optional[IRI],)",
    "\n\n",
    doc!(OntologyID)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OntologyID {
        #[doc="iri: typing.Optional[IRI]"]
        #[pyo3(get,set)]
        pub iri: Option<IRI>,
    
        #[doc="viri: typing.Optional[IRI]"]
        #[pyo3(get,set)]
        pub viri: Option<IRI>,
    }

#[pymethods]
impl OntologyID {
    #[new]
    fn new(iri: Option<IRI>,viri: Option<IRI>,) -> Self {
        OntologyID {
                iri,
                viri,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "iri" => self.iri.clone().into_pyobject(py).map(Bound::into_any),
            "viri" => self.viri.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "iri" => {
                self.iri = value.extract()?;
                Ok(())
            },
            "viri" => {
                self.viri = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::OntologyID<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::OntologyID<ArcStr>> for OntologyID {
    fn from(value: &horned_owl::model::OntologyID<ArcStr>) -> Self {
        OntologyID {
            iri: IntoCompatible::<Option<IRI>>::into_c(value.iri.borrow()),
            viri: IntoCompatible::<Option<IRI>>::into_c(value.viri.borrow()),
        }
    }
}


impl From<&OntologyID> for horned_owl::model::OntologyID<ArcStr> {
    fn from(value: &OntologyID) -> Self {
        horned_owl::model::OntologyID::<ArcStr> {
            iri: value.iri.borrow().into_c(),
            viri: value.viri.borrow().into_c(),
        }
    }
}



/**************** Base implementations for OntologyID ****************/
impl FromCompatible<horned_owl::model::OntologyID<ArcStr>> for OntologyID {
    fn from_c(value: horned_owl::model::OntologyID<ArcStr>) -> Self {
        OntologyID::from(value)
    }
}

impl FromCompatible<&horned_owl::model::OntologyID<ArcStr>> for OntologyID {
    fn from_c(value: &horned_owl::model::OntologyID<ArcStr>) -> Self {
        OntologyID::from(value)
    }
}

impl FromCompatible<OntologyID> for horned_owl::model::OntologyID<ArcStr> {
    fn from_c(value: OntologyID) -> Self {
        horned_owl::model::OntologyID::<ArcStr>::from(value)
    }
}

impl FromCompatible<&OntologyID> for horned_owl::model::OntologyID<ArcStr> {
    fn from_c(value: &OntologyID) -> Self {
        horned_owl::model::OntologyID::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::OntologyID<ArcStr>> for OntologyID {
    fn from(value: horned_owl::model::OntologyID<ArcStr>) -> Self {
        OntologyID::from(value.borrow())
    }
}

impl From<OntologyID> for horned_owl::model::OntologyID<ArcStr> {
    fn from(value: OntologyID) -> Self {
        horned_owl::model::OntologyID::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<OntologyID>> for Box<horned_owl::model::OntologyID<ArcStr>> {
    fn from(value: &BoxWrap<OntologyID>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::OntologyID<ArcStr>>> for BoxWrap<OntologyID> {
    fn from(value: &Box<horned_owl::model::OntologyID<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<OntologyID>::into(*value.clone())))
    }
}

impl From<BoxWrap<OntologyID>> for Box<horned_owl::model::OntologyID<ArcStr>> {
    fn from(value: BoxWrap<OntologyID>) -> Self {
        Into::<Box<horned_owl::model::OntologyID<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::OntologyID<ArcStr>>> for BoxWrap<OntologyID> {
    fn from(value: Box<horned_owl::model::OntologyID<ArcStr>>) -> Self {
        Into::<BoxWrap<OntologyID>>::into(value.borrow())
    }
}

impl From<VecWrap<OntologyID>> for Vec<horned_owl::model::OntologyID<ArcStr>> {
    fn from(value: VecWrap<OntologyID>) -> Self {
        Into::<Vec<horned_owl::model::OntologyID<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::OntologyID<ArcStr>>> for VecWrap<OntologyID> {
    fn from(value: Vec<horned_owl::model::OntologyID<ArcStr>>) -> Self {
        Into::<VecWrap<OntologyID>>::into(value.borrow())
    }
}

impl From<&VecWrap<OntologyID>> for Vec<horned_owl::model::OntologyID<ArcStr>> {
    fn from(value: &VecWrap<OntologyID>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::OntologyID::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::OntologyID<ArcStr>>> for VecWrap<OntologyID> {
    fn from(value: &Vec<horned_owl::model::OntologyID<ArcStr>>) -> Self {
        VecWrap(value.iter().map(OntologyID::from).collect())
    }
}

impl FromCompatible<&BoxWrap<OntologyID>> for Box<horned_owl::model::OntologyID<ArcStr>> {
    fn from_c(value: &BoxWrap<OntologyID>) -> Self {
        Box::<horned_owl::model::OntologyID<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::OntologyID<ArcStr>>> for BoxWrap<OntologyID> {
    fn from_c(value: &Box<horned_owl::model::OntologyID<ArcStr>>) -> Self {
        BoxWrap::<OntologyID>::from(value)
    }
}
impl FromCompatible<BoxWrap<OntologyID>> for Box<horned_owl::model::OntologyID<ArcStr>> {
    fn from_c(value: BoxWrap<OntologyID>) -> Self {
        Box::<horned_owl::model::OntologyID<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::OntologyID<ArcStr>>> for BoxWrap<OntologyID> {
    fn from_c(value: Box<horned_owl::model::OntologyID<ArcStr>>) -> Self {
        BoxWrap::<OntologyID>::from(value)
    }
}
impl FromCompatible<VecWrap<OntologyID>> for Vec<horned_owl::model::OntologyID<ArcStr>> {
    fn from_c(value: VecWrap<OntologyID>) -> Self {
        Vec::<horned_owl::model::OntologyID<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::OntologyID<ArcStr>>> for VecWrap<OntologyID> {
    fn from_c(value: Vec<horned_owl::model::OntologyID<ArcStr>>) -> Self {
        VecWrap::<OntologyID>::from(value)
    }
}
impl FromCompatible<&VecWrap<OntologyID>> for Vec<horned_owl::model::OntologyID<ArcStr>> {
    fn from_c(value: &VecWrap<OntologyID>) -> Self {
        Vec::<horned_owl::model::OntologyID<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::OntologyID<ArcStr>>> for VecWrap<OntologyID> {
    fn from_c(value: &Vec<horned_owl::model::OntologyID<ArcStr>>) -> Self {
        VecWrap::<OntologyID>::from(value)
    }
}
#[doc = concat!(
    "Variable(first: IRI,)",
    "\n\n",
    doc!(Variable)
)]
#[pyclass(module="pyhornedowl.model")]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Variable (
    #[doc="first: "]
    #[pyo3(get,set,name="first")]
    pub IRI,
);

#[pymethods]
impl Variable {
    #[new]
    fn new(first: IRI,) -> Self {
        Variable (first,)
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::Variable<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::Variable<ArcStr>> for Variable {
    fn from(value: &horned_owl::model::Variable<ArcStr>) -> Self {

        Variable (
    IntoCompatible::<IRI>::into_c(&value.0),
        )
    }
}

impl From<&Variable> for horned_owl::model::Variable<ArcStr> {
    fn from(value: &Variable) -> Self {
        horned_owl::model::Variable::<ArcStr> (
    IntoCompatible::into_c(&value.0),
        )
    }
}



/**************** Base implementations for Variable ****************/
impl FromCompatible<horned_owl::model::Variable<ArcStr>> for Variable {
    fn from_c(value: horned_owl::model::Variable<ArcStr>) -> Self {
        Variable::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Variable<ArcStr>> for Variable {
    fn from_c(value: &horned_owl::model::Variable<ArcStr>) -> Self {
        Variable::from(value)
    }
}

impl FromCompatible<Variable> for horned_owl::model::Variable<ArcStr> {
    fn from_c(value: Variable) -> Self {
        horned_owl::model::Variable::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Variable> for horned_owl::model::Variable<ArcStr> {
    fn from_c(value: &Variable) -> Self {
        horned_owl::model::Variable::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Variable<ArcStr>> for Variable {
    fn from(value: horned_owl::model::Variable<ArcStr>) -> Self {
        Variable::from(value.borrow())
    }
}

impl From<Variable> for horned_owl::model::Variable<ArcStr> {
    fn from(value: Variable) -> Self {
        horned_owl::model::Variable::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Variable>> for Box<horned_owl::model::Variable<ArcStr>> {
    fn from(value: &BoxWrap<Variable>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Variable<ArcStr>>> for BoxWrap<Variable> {
    fn from(value: &Box<horned_owl::model::Variable<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Variable>::into(*value.clone())))
    }
}

impl From<BoxWrap<Variable>> for Box<horned_owl::model::Variable<ArcStr>> {
    fn from(value: BoxWrap<Variable>) -> Self {
        Into::<Box<horned_owl::model::Variable<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Variable<ArcStr>>> for BoxWrap<Variable> {
    fn from(value: Box<horned_owl::model::Variable<ArcStr>>) -> Self {
        Into::<BoxWrap<Variable>>::into(value.borrow())
    }
}

impl From<VecWrap<Variable>> for Vec<horned_owl::model::Variable<ArcStr>> {
    fn from(value: VecWrap<Variable>) -> Self {
        Into::<Vec<horned_owl::model::Variable<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Variable<ArcStr>>> for VecWrap<Variable> {
    fn from(value: Vec<horned_owl::model::Variable<ArcStr>>) -> Self {
        Into::<VecWrap<Variable>>::into(value.borrow())
    }
}

impl From<&VecWrap<Variable>> for Vec<horned_owl::model::Variable<ArcStr>> {
    fn from(value: &VecWrap<Variable>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Variable::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Variable<ArcStr>>> for VecWrap<Variable> {
    fn from(value: &Vec<horned_owl::model::Variable<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Variable::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Variable>> for Box<horned_owl::model::Variable<ArcStr>> {
    fn from_c(value: &BoxWrap<Variable>) -> Self {
        Box::<horned_owl::model::Variable<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Variable<ArcStr>>> for BoxWrap<Variable> {
    fn from_c(value: &Box<horned_owl::model::Variable<ArcStr>>) -> Self {
        BoxWrap::<Variable>::from(value)
    }
}
impl FromCompatible<BoxWrap<Variable>> for Box<horned_owl::model::Variable<ArcStr>> {
    fn from_c(value: BoxWrap<Variable>) -> Self {
        Box::<horned_owl::model::Variable<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Variable<ArcStr>>> for BoxWrap<Variable> {
    fn from_c(value: Box<horned_owl::model::Variable<ArcStr>>) -> Self {
        BoxWrap::<Variable>::from(value)
    }
}
impl FromCompatible<VecWrap<Variable>> for Vec<horned_owl::model::Variable<ArcStr>> {
    fn from_c(value: VecWrap<Variable>) -> Self {
        Vec::<horned_owl::model::Variable<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Variable<ArcStr>>> for VecWrap<Variable> {
    fn from_c(value: Vec<horned_owl::model::Variable<ArcStr>>) -> Self {
        VecWrap::<Variable>::from(value)
    }
}
impl FromCompatible<&VecWrap<Variable>> for Vec<horned_owl::model::Variable<ArcStr>> {
    fn from_c(value: &VecWrap<Variable>) -> Self {
        Vec::<horned_owl::model::Variable<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Variable<ArcStr>>> for VecWrap<Variable> {
    fn from_c(value: &Vec<horned_owl::model::Variable<ArcStr>>) -> Self {
        VecWrap::<Variable>::from(value)
    }
}
#[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DArgument {
    
        #[pyo3(transparent)]
        Literal (Literal),
    
        #[pyo3(transparent)]
        Variable (Variable),
    
}

impl<'py> IntoPyObject<'py> for DArgument {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
        
            DArgument::Literal(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            DArgument::Variable(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
        }
    }
}

impl From<&DArgument> for horned_owl::model::DArgument<ArcStr> {
    fn from(value: &DArgument) -> Self {
        match value {
        
            DArgument::Literal(inner) => horned_owl::model::DArgument::Literal(inner.into()),
        
            DArgument::Variable(inner) => horned_owl::model::DArgument::Variable(inner.into()),
        
        }
    }
}

impl From<&horned_owl::model::DArgument<ArcStr>> for DArgument {

    fn from(value: &horned_owl::model::DArgument<ArcStr>) -> Self {
        match value {
        
            horned_owl::model::DArgument::Literal(inner) => DArgument::Literal(inner.into()),
        
            horned_owl::model::DArgument::Variable(inner) => DArgument::Variable(inner.into()),
        
        }
    }
}


impl DArgument {
    pub fn py_def() -> String {
        "typing.Union[m.Literal,m.Variable,]".into()
    }
}



/**************** Base implementations for DArgument ****************/
impl FromCompatible<horned_owl::model::DArgument<ArcStr>> for DArgument {
    fn from_c(value: horned_owl::model::DArgument<ArcStr>) -> Self {
        DArgument::from(value)
    }
}

impl FromCompatible<&horned_owl::model::DArgument<ArcStr>> for DArgument {
    fn from_c(value: &horned_owl::model::DArgument<ArcStr>) -> Self {
        DArgument::from(value)
    }
}

impl FromCompatible<DArgument> for horned_owl::model::DArgument<ArcStr> {
    fn from_c(value: DArgument) -> Self {
        horned_owl::model::DArgument::<ArcStr>::from(value)
    }
}

impl FromCompatible<&DArgument> for horned_owl::model::DArgument<ArcStr> {
    fn from_c(value: &DArgument) -> Self {
        horned_owl::model::DArgument::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::DArgument<ArcStr>> for DArgument {
    fn from(value: horned_owl::model::DArgument<ArcStr>) -> Self {
        DArgument::from(value.borrow())
    }
}

impl From<DArgument> for horned_owl::model::DArgument<ArcStr> {
    fn from(value: DArgument) -> Self {
        horned_owl::model::DArgument::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<DArgument>> for Box<horned_owl::model::DArgument<ArcStr>> {
    fn from(value: &BoxWrap<DArgument>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::DArgument<ArcStr>>> for BoxWrap<DArgument> {
    fn from(value: &Box<horned_owl::model::DArgument<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<DArgument>::into(*value.clone())))
    }
}

impl From<BoxWrap<DArgument>> for Box<horned_owl::model::DArgument<ArcStr>> {
    fn from(value: BoxWrap<DArgument>) -> Self {
        Into::<Box<horned_owl::model::DArgument<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::DArgument<ArcStr>>> for BoxWrap<DArgument> {
    fn from(value: Box<horned_owl::model::DArgument<ArcStr>>) -> Self {
        Into::<BoxWrap<DArgument>>::into(value.borrow())
    }
}

impl From<VecWrap<DArgument>> for Vec<horned_owl::model::DArgument<ArcStr>> {
    fn from(value: VecWrap<DArgument>) -> Self {
        Into::<Vec<horned_owl::model::DArgument<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::DArgument<ArcStr>>> for VecWrap<DArgument> {
    fn from(value: Vec<horned_owl::model::DArgument<ArcStr>>) -> Self {
        Into::<VecWrap<DArgument>>::into(value.borrow())
    }
}

impl From<&VecWrap<DArgument>> for Vec<horned_owl::model::DArgument<ArcStr>> {
    fn from(value: &VecWrap<DArgument>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::DArgument::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::DArgument<ArcStr>>> for VecWrap<DArgument> {
    fn from(value: &Vec<horned_owl::model::DArgument<ArcStr>>) -> Self {
        VecWrap(value.iter().map(DArgument::from).collect())
    }
}

impl FromCompatible<&BoxWrap<DArgument>> for Box<horned_owl::model::DArgument<ArcStr>> {
    fn from_c(value: &BoxWrap<DArgument>) -> Self {
        Box::<horned_owl::model::DArgument<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::DArgument<ArcStr>>> for BoxWrap<DArgument> {
    fn from_c(value: &Box<horned_owl::model::DArgument<ArcStr>>) -> Self {
        BoxWrap::<DArgument>::from(value)
    }
}
impl FromCompatible<BoxWrap<DArgument>> for Box<horned_owl::model::DArgument<ArcStr>> {
    fn from_c(value: BoxWrap<DArgument>) -> Self {
        Box::<horned_owl::model::DArgument<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::DArgument<ArcStr>>> for BoxWrap<DArgument> {
    fn from_c(value: Box<horned_owl::model::DArgument<ArcStr>>) -> Self {
        BoxWrap::<DArgument>::from(value)
    }
}
impl FromCompatible<VecWrap<DArgument>> for Vec<horned_owl::model::DArgument<ArcStr>> {
    fn from_c(value: VecWrap<DArgument>) -> Self {
        Vec::<horned_owl::model::DArgument<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::DArgument<ArcStr>>> for VecWrap<DArgument> {
    fn from_c(value: Vec<horned_owl::model::DArgument<ArcStr>>) -> Self {
        VecWrap::<DArgument>::from(value)
    }
}
impl FromCompatible<&VecWrap<DArgument>> for Vec<horned_owl::model::DArgument<ArcStr>> {
    fn from_c(value: &VecWrap<DArgument>) -> Self {
        Vec::<horned_owl::model::DArgument<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::DArgument<ArcStr>>> for VecWrap<DArgument> {
    fn from_c(value: &Vec<horned_owl::model::DArgument<ArcStr>>) -> Self {
        VecWrap::<DArgument>::from(value)
    }
}
#[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IArgument {
    
        #[pyo3(transparent)]
        Individual (Individual),
    
        #[pyo3(transparent)]
        Variable (Variable),
    
}

impl<'py> IntoPyObject<'py> for IArgument {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
        
            IArgument::Individual(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            IArgument::Variable(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
        }
    }
}

impl From<&IArgument> for horned_owl::model::IArgument<ArcStr> {
    fn from(value: &IArgument) -> Self {
        match value {
        
            IArgument::Individual(inner) => horned_owl::model::IArgument::Individual(inner.into()),
        
            IArgument::Variable(inner) => horned_owl::model::IArgument::Variable(inner.into()),
        
        }
    }
}

impl From<&horned_owl::model::IArgument<ArcStr>> for IArgument {

    fn from(value: &horned_owl::model::IArgument<ArcStr>) -> Self {
        match value {
        
            horned_owl::model::IArgument::Individual(inner) => IArgument::Individual(inner.into()),
        
            horned_owl::model::IArgument::Variable(inner) => IArgument::Variable(inner.into()),
        
        }
    }
}


impl IArgument {
    pub fn py_def() -> String {
        "typing.Union[m.Individual,m.Variable,]".into()
    }
}



/**************** Base implementations for IArgument ****************/
impl FromCompatible<horned_owl::model::IArgument<ArcStr>> for IArgument {
    fn from_c(value: horned_owl::model::IArgument<ArcStr>) -> Self {
        IArgument::from(value)
    }
}

impl FromCompatible<&horned_owl::model::IArgument<ArcStr>> for IArgument {
    fn from_c(value: &horned_owl::model::IArgument<ArcStr>) -> Self {
        IArgument::from(value)
    }
}

impl FromCompatible<IArgument> for horned_owl::model::IArgument<ArcStr> {
    fn from_c(value: IArgument) -> Self {
        horned_owl::model::IArgument::<ArcStr>::from(value)
    }
}

impl FromCompatible<&IArgument> for horned_owl::model::IArgument<ArcStr> {
    fn from_c(value: &IArgument) -> Self {
        horned_owl::model::IArgument::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::IArgument<ArcStr>> for IArgument {
    fn from(value: horned_owl::model::IArgument<ArcStr>) -> Self {
        IArgument::from(value.borrow())
    }
}

impl From<IArgument> for horned_owl::model::IArgument<ArcStr> {
    fn from(value: IArgument) -> Self {
        horned_owl::model::IArgument::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<IArgument>> for Box<horned_owl::model::IArgument<ArcStr>> {
    fn from(value: &BoxWrap<IArgument>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::IArgument<ArcStr>>> for BoxWrap<IArgument> {
    fn from(value: &Box<horned_owl::model::IArgument<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<IArgument>::into(*value.clone())))
    }
}

impl From<BoxWrap<IArgument>> for Box<horned_owl::model::IArgument<ArcStr>> {
    fn from(value: BoxWrap<IArgument>) -> Self {
        Into::<Box<horned_owl::model::IArgument<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::IArgument<ArcStr>>> for BoxWrap<IArgument> {
    fn from(value: Box<horned_owl::model::IArgument<ArcStr>>) -> Self {
        Into::<BoxWrap<IArgument>>::into(value.borrow())
    }
}

impl From<VecWrap<IArgument>> for Vec<horned_owl::model::IArgument<ArcStr>> {
    fn from(value: VecWrap<IArgument>) -> Self {
        Into::<Vec<horned_owl::model::IArgument<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::IArgument<ArcStr>>> for VecWrap<IArgument> {
    fn from(value: Vec<horned_owl::model::IArgument<ArcStr>>) -> Self {
        Into::<VecWrap<IArgument>>::into(value.borrow())
    }
}

impl From<&VecWrap<IArgument>> for Vec<horned_owl::model::IArgument<ArcStr>> {
    fn from(value: &VecWrap<IArgument>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::IArgument::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::IArgument<ArcStr>>> for VecWrap<IArgument> {
    fn from(value: &Vec<horned_owl::model::IArgument<ArcStr>>) -> Self {
        VecWrap(value.iter().map(IArgument::from).collect())
    }
}

impl FromCompatible<&BoxWrap<IArgument>> for Box<horned_owl::model::IArgument<ArcStr>> {
    fn from_c(value: &BoxWrap<IArgument>) -> Self {
        Box::<horned_owl::model::IArgument<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::IArgument<ArcStr>>> for BoxWrap<IArgument> {
    fn from_c(value: &Box<horned_owl::model::IArgument<ArcStr>>) -> Self {
        BoxWrap::<IArgument>::from(value)
    }
}
impl FromCompatible<BoxWrap<IArgument>> for Box<horned_owl::model::IArgument<ArcStr>> {
    fn from_c(value: BoxWrap<IArgument>) -> Self {
        Box::<horned_owl::model::IArgument<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::IArgument<ArcStr>>> for BoxWrap<IArgument> {
    fn from_c(value: Box<horned_owl::model::IArgument<ArcStr>>) -> Self {
        BoxWrap::<IArgument>::from(value)
    }
}
impl FromCompatible<VecWrap<IArgument>> for Vec<horned_owl::model::IArgument<ArcStr>> {
    fn from_c(value: VecWrap<IArgument>) -> Self {
        Vec::<horned_owl::model::IArgument<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::IArgument<ArcStr>>> for VecWrap<IArgument> {
    fn from_c(value: Vec<horned_owl::model::IArgument<ArcStr>>) -> Self {
        VecWrap::<IArgument>::from(value)
    }
}
impl FromCompatible<&VecWrap<IArgument>> for Vec<horned_owl::model::IArgument<ArcStr>> {
    fn from_c(value: &VecWrap<IArgument>) -> Self {
        Vec::<horned_owl::model::IArgument<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::IArgument<ArcStr>>> for VecWrap<IArgument> {
    fn from_c(value: &Vec<horned_owl::model::IArgument<ArcStr>>) -> Self {
        VecWrap::<IArgument>::from(value)
    }
}
/**************** ENUM implementation for Atom ****************/
#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Atom_Inner {
	BuiltInAtom(BuiltInAtom),
	ClassAtom(ClassAtom),
	DataPropertyAtom(DataPropertyAtom),
	DataRangeAtom(DataRangeAtom),
	DifferentIndividualsAtom(DifferentIndividualsAtom),
	ObjectPropertyAtom(ObjectPropertyAtom),
	SameIndividualAtom(SameIndividualAtom),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom(Atom_Inner);


/**************** ENUM VARIANTS for Atom ****************/

    /**************** ENUM VARIANT BuiltInAtom for Atom ****************/
    #[doc = concat!("BuiltInAtom(pred: IRIargs: typing.List[DArgument]",
        "\n\n",doc!(BuiltInAtom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BuiltInAtom{
        #[doc="pred: IRI"]
        #[pyo3(get,set)]
        pub pred: IRI,
        #[doc="args: typing.List[DArgument]"]
        #[pyo3(get,set)]
        pub args: VecWrap<DArgument>,}

    impl From<BuiltInAtom> for Atom {
        fn from(value: BuiltInAtom) -> Self {
            Atom(Atom_Inner::BuiltInAtom(value))
        }
    }

    #[pymethods]
    impl BuiltInAtom {
        #[new]
        fn new(pred: IRI,args: VecWrap<DArgument>,) -> Self {
            BuiltInAtom{
                pred,
                args,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"pred" => self.pred.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"args" => self.args.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"pred" => {
                    self.pred = value.extract()?;
                Ok(())
                },
            	"args" => {
                    self.args = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Atom<ArcStr>>::into(Into::<Atom>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT ClassAtom for Atom ****************/
    #[doc = concat!("ClassAtom(pred: ClassExpressionarg: IArgument",
        "\n\n",doc!(ClassAtom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClassAtom{
        #[doc="pred: ClassExpression"]
        #[pyo3(get,set)]
        pub pred: ClassExpression,
        #[doc="arg: IArgument"]
        #[pyo3(get,set)]
        pub arg: IArgument,}

    impl From<ClassAtom> for Atom {
        fn from(value: ClassAtom) -> Self {
            Atom(Atom_Inner::ClassAtom(value))
        }
    }

    #[pymethods]
    impl ClassAtom {
        #[new]
        fn new(pred: ClassExpression,arg: IArgument,) -> Self {
            ClassAtom{
                pred,
                arg,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"pred" => self.pred.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"arg" => self.arg.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"pred" => {
                    self.pred = value.extract()?;
                Ok(())
                },
            	"arg" => {
                    self.arg = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Atom<ArcStr>>::into(Into::<Atom>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT DataPropertyAtom for Atom ****************/
    #[doc = concat!("DataPropertyAtom(pred: DataPropertyargs: typing.Tuple[DArgument,DArgument]",
        "\n\n",doc!(DataPropertyAtom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataPropertyAtom{
        #[doc="pred: DataProperty"]
        #[pyo3(get,set)]
        pub pred: DataProperty,
        #[doc="args: typing.Tuple[DArgument,DArgument]"]
        #[pyo3(get,set)]
        pub args: (DArgument,DArgument),}

    impl From<DataPropertyAtom> for Atom {
        fn from(value: DataPropertyAtom) -> Self {
            Atom(Atom_Inner::DataPropertyAtom(value))
        }
    }

    #[pymethods]
    impl DataPropertyAtom {
        #[new]
        fn new(pred: DataProperty,args: (DArgument,DArgument),) -> Self {
            DataPropertyAtom{
                pred,
                args,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"pred" => self.pred.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"args" => self.args.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"pred" => {
                    self.pred = value.extract()?;
                Ok(())
                },
            	"args" => {
                    self.args = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Atom<ArcStr>>::into(Into::<Atom>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT DataRangeAtom for Atom ****************/
    #[doc = concat!("DataRangeAtom(pred: DataRangearg: DArgument",
        "\n\n",doc!(DataRangeAtom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataRangeAtom{
        #[doc="pred: DataRange"]
        #[pyo3(get,set)]
        pub pred: DataRange,
        #[doc="arg: DArgument"]
        #[pyo3(get,set)]
        pub arg: DArgument,}

    impl From<DataRangeAtom> for Atom {
        fn from(value: DataRangeAtom) -> Self {
            Atom(Atom_Inner::DataRangeAtom(value))
        }
    }

    #[pymethods]
    impl DataRangeAtom {
        #[new]
        fn new(pred: DataRange,arg: DArgument,) -> Self {
            DataRangeAtom{
                pred,
                arg,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"pred" => self.pred.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"arg" => self.arg.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"pred" => {
                    self.pred = value.extract()?;
                Ok(())
                },
            	"arg" => {
                    self.arg = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Atom<ArcStr>>::into(Into::<Atom>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT DifferentIndividualsAtom for Atom ****************/
    #[doc = concat!("DifferentIndividualsAtom(first: IArgumentsecond: IArgument",
        "\n\n",doc!(DifferentIndividualsAtom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DifferentIndividualsAtom(
        #[pyo3(get,set,name="first")]
        pub IArgument,
        #[pyo3(get,set,name="second")]
        pub IArgument,
    );

    impl From<DifferentIndividualsAtom> for Atom {
        fn from(value: DifferentIndividualsAtom) -> Self {
            Atom(Atom_Inner::DifferentIndividualsAtom(value))
        }
    }

    #[pymethods]
    impl DifferentIndividualsAtom {
        #[new]
        fn new(first: IArgument,second: IArgument,) -> Self {
            DifferentIndividualsAtom(
                first,
                second,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"second" => self.1.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
            	"second" => {
                    self.1 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Atom<ArcStr>>::into(Into::<Atom>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT ObjectPropertyAtom for Atom ****************/
    #[doc = concat!("ObjectPropertyAtom(pred: ObjectPropertyExpressionargs: typing.Tuple[IArgument,IArgument]",
        "\n\n",doc!(ObjectPropertyAtom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ObjectPropertyAtom{
        #[doc="pred: ObjectPropertyExpression"]
        #[pyo3(get,set)]
        pub pred: ObjectPropertyExpression,
        #[doc="args: typing.Tuple[IArgument,IArgument]"]
        #[pyo3(get,set)]
        pub args: (IArgument,IArgument),}

    impl From<ObjectPropertyAtom> for Atom {
        fn from(value: ObjectPropertyAtom) -> Self {
            Atom(Atom_Inner::ObjectPropertyAtom(value))
        }
    }

    #[pymethods]
    impl ObjectPropertyAtom {
        #[new]
        fn new(pred: ObjectPropertyExpression,args: (IArgument,IArgument),) -> Self {
            ObjectPropertyAtom{
                pred,
                args,}
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"pred" => self.pred.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"args" => self.args.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"pred" => {
                    self.pred = value.extract()?;
                Ok(())
                },
            	"args" => {
                    self.args = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Atom<ArcStr>>::into(Into::<Atom>::into(self.clone())).as_functional().to_string()
        }
    }

    


    /**************** ENUM VARIANT SameIndividualAtom for Atom ****************/
    #[doc = concat!("SameIndividualAtom(first: IArgumentsecond: IArgument",
        "\n\n",doc!(SameIndividualAtom))]
    #[allow(non_camel_case_types)]
    #[pyclass(module="pyhornedowl.model")]
    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SameIndividualAtom(
        #[pyo3(get,set,name="first")]
        pub IArgument,
        #[pyo3(get,set,name="second")]
        pub IArgument,
    );

    impl From<SameIndividualAtom> for Atom {
        fn from(value: SameIndividualAtom) -> Self {
            Atom(Atom_Inner::SameIndividualAtom(value))
        }
    }

    #[pymethods]
    impl SameIndividualAtom {
        #[new]
        fn new(first: IArgument,second: IArgument,) -> Self {
            SameIndividualAtom(
                first,
                second,
            )
        }


        fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
            match name {
            	"first" => self.0.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
            	"second" => self.1.clone().into_pyobject(py).map(Bound::into_any).map_err(PyErr::from),
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
            match name {
            	"first" => {
                    self.0 = value.extract()?;
                Ok(())
                },
            	"second" => {
                    self.1 = value.extract()?;
                Ok(())
                },
                &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
            }
        }

        fn __hash__(&self) -> u64 {
            let mut s = DefaultHasher::new();
            Hash::hash(&self, &mut s);
            s.finish()
        }

        fn __eq__(&self, other: &Self) -> bool {
            self == other
        }

        fn __str__(&self) -> String {
            Into::<horned_owl::model::Atom<ArcStr>>::into(Into::<Atom>::into(self.clone())).as_functional().to_string()
        }
    }

    



impl From<&horned_owl::model::Atom<ArcStr>> for Atom {
    fn from(value: &horned_owl::model::Atom<ArcStr>) -> Self {
        match value {horned_owl::model::Atom::BuiltInAtom::<ArcStr>{
                    pred,
args
                } => Atom(Atom_Inner::BuiltInAtom(BuiltInAtom{pred: IntoCompatible::<IRI>::into_c(pred),args: IntoCompatible::<VecWrap<DArgument>>::into_c(args),})),horned_owl::model::Atom::ClassAtom::<ArcStr>{
                    pred,
arg
                } => Atom(Atom_Inner::ClassAtom(ClassAtom{pred: IntoCompatible::<ClassExpression>::into_c(pred),arg: IntoCompatible::<IArgument>::into_c(arg),})),horned_owl::model::Atom::DataPropertyAtom::<ArcStr>{
                    pred,
args
                } => Atom(Atom_Inner::DataPropertyAtom(DataPropertyAtom{pred: IntoCompatible::<DataProperty>::into_c(pred),args: IntoCompatible::<(DArgument,DArgument)>::into_c(args),})),horned_owl::model::Atom::DataRangeAtom::<ArcStr>{
                    pred,
arg
                } => Atom(Atom_Inner::DataRangeAtom(DataRangeAtom{pred: IntoCompatible::<DataRange>::into_c(pred),arg: IntoCompatible::<DArgument>::into_c(arg),})),
                horned_owl::model::Atom::DifferentIndividualsAtom::<ArcStr>(first, second) =>
                    Atom(Atom_Inner::DifferentIndividualsAtom(DifferentIndividualsAtom((first).into(),(second).into(),))),horned_owl::model::Atom::ObjectPropertyAtom::<ArcStr>{
                    pred,
args
                } => Atom(Atom_Inner::ObjectPropertyAtom(ObjectPropertyAtom{pred: IntoCompatible::<ObjectPropertyExpression>::into_c(pred),args: IntoCompatible::<(IArgument,IArgument)>::into_c(args),})),
                horned_owl::model::Atom::SameIndividualAtom::<ArcStr>(first, second) =>
                    Atom(Atom_Inner::SameIndividualAtom(SameIndividualAtom((first).into(),(second).into(),))),
        }
    }
}

impl From<&Atom> for horned_owl::model::Atom<ArcStr> {
    fn from(value: &Atom) -> Self {
        match value.0.borrow() {
                Atom_Inner::BuiltInAtom(BuiltInAtom{
                    pred, args
                }) => horned_owl::model::Atom::<ArcStr>::BuiltInAtom{
                	pred: pred.into_c(),
                	args: args.into_c(),
                },
                Atom_Inner::ClassAtom(ClassAtom{
                    pred, arg
                }) => horned_owl::model::Atom::<ArcStr>::ClassAtom{
                	pred: pred.into_c(),
                	arg: arg.into_c(),
                },
                Atom_Inner::DataPropertyAtom(DataPropertyAtom{
                    pred, args
                }) => horned_owl::model::Atom::<ArcStr>::DataPropertyAtom{
                	pred: pred.into_c(),
                	args: args.into_c(),
                },
                Atom_Inner::DataRangeAtom(DataRangeAtom{
                    pred, arg
                }) => horned_owl::model::Atom::<ArcStr>::DataRangeAtom{
                	pred: pred.into_c(),
                	arg: arg.into_c(),
                },
                Atom_Inner::DifferentIndividualsAtom(DifferentIndividualsAtom(first, second)) =>
                horned_owl::model::Atom::<ArcStr>::DifferentIndividualsAtom(
                    first.into_c(),
                    second.into_c(),
                ),
                Atom_Inner::ObjectPropertyAtom(ObjectPropertyAtom{
                    pred, args
                }) => horned_owl::model::Atom::<ArcStr>::ObjectPropertyAtom{
                	pred: pred.into_c(),
                	args: args.into_c(),
                },
                Atom_Inner::SameIndividualAtom(SameIndividualAtom(first, second)) =>
                horned_owl::model::Atom::<ArcStr>::SameIndividualAtom(
                    first.into_c(),
                    second.into_c(),
                ),
        }
    }
}

impl<'py> IntoPyObject<'py> for Atom {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = pyo3::PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        match self.0 {
            
                Atom_Inner::BuiltInAtom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                Atom_Inner::ClassAtom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                Atom_Inner::DataPropertyAtom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                Atom_Inner::DataRangeAtom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                Atom_Inner::DifferentIndividualsAtom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                Atom_Inner::ObjectPropertyAtom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
                Atom_Inner::SameIndividualAtom(val) => {
                    val.into_pyobject(py).map(Bound::into_any)
                },
            
        }
    }

}

impl <'py> FromPyObject<'py> for Atom {
    fn extract_bound(ob: &Bound<'py, pyo3::PyAny>) -> pyo3::PyResult<Self> {
            {
                let r = BuiltInAtom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Atom_Inner::BuiltInAtom(local);
                    return Ok(Atom(inner));
                }
            }
            {
                let r = ClassAtom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Atom_Inner::ClassAtom(local);
                    return Ok(Atom(inner));
                }
            }
            {
                let r = DataPropertyAtom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Atom_Inner::DataPropertyAtom(local);
                    return Ok(Atom(inner));
                }
            }
            {
                let r = DataRangeAtom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Atom_Inner::DataRangeAtom(local);
                    return Ok(Atom(inner));
                }
            }
            {
                let r = DifferentIndividualsAtom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Atom_Inner::DifferentIndividualsAtom(local);
                    return Ok(Atom(inner));
                }
            }
            {
                let r = ObjectPropertyAtom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Atom_Inner::ObjectPropertyAtom(local);
                    return Ok(Atom(inner));
                }
            }
            {
                let r = SameIndividualAtom::extract_bound(ob);
                if let Ok(local) = r {
                    let inner = Atom_Inner::SameIndividualAtom(local);
                    return Ok(Atom(inner));
                }
            }

        Err(pyo3::PyErr::new::<pyo3::exceptions::PyTypeError, _>("Object cannot be converted to Atom"))
    }
}

impl Atom {
    pub fn py_def() -> String {
        "typing.Union[m.BuiltInAtom,m.ClassAtom,m.DataPropertyAtom,m.DataRangeAtom,m.DifferentIndividualsAtom,m.ObjectPropertyAtom,m.SameIndividualAtom,]".into()
    }
}



/**************** Base implementations for Atom ****************/
impl FromCompatible<horned_owl::model::Atom<ArcStr>> for Atom {
    fn from_c(value: horned_owl::model::Atom<ArcStr>) -> Self {
        Atom::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Atom<ArcStr>> for Atom {
    fn from_c(value: &horned_owl::model::Atom<ArcStr>) -> Self {
        Atom::from(value)
    }
}

impl FromCompatible<Atom> for horned_owl::model::Atom<ArcStr> {
    fn from_c(value: Atom) -> Self {
        horned_owl::model::Atom::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Atom> for horned_owl::model::Atom<ArcStr> {
    fn from_c(value: &Atom) -> Self {
        horned_owl::model::Atom::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Atom<ArcStr>> for Atom {
    fn from(value: horned_owl::model::Atom<ArcStr>) -> Self {
        Atom::from(value.borrow())
    }
}

impl From<Atom> for horned_owl::model::Atom<ArcStr> {
    fn from(value: Atom) -> Self {
        horned_owl::model::Atom::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Atom>> for Box<horned_owl::model::Atom<ArcStr>> {
    fn from(value: &BoxWrap<Atom>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Atom<ArcStr>>> for BoxWrap<Atom> {
    fn from(value: &Box<horned_owl::model::Atom<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Atom>::into(*value.clone())))
    }
}

impl From<BoxWrap<Atom>> for Box<horned_owl::model::Atom<ArcStr>> {
    fn from(value: BoxWrap<Atom>) -> Self {
        Into::<Box<horned_owl::model::Atom<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Atom<ArcStr>>> for BoxWrap<Atom> {
    fn from(value: Box<horned_owl::model::Atom<ArcStr>>) -> Self {
        Into::<BoxWrap<Atom>>::into(value.borrow())
    }
}

impl From<VecWrap<Atom>> for Vec<horned_owl::model::Atom<ArcStr>> {
    fn from(value: VecWrap<Atom>) -> Self {
        Into::<Vec<horned_owl::model::Atom<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Atom<ArcStr>>> for VecWrap<Atom> {
    fn from(value: Vec<horned_owl::model::Atom<ArcStr>>) -> Self {
        Into::<VecWrap<Atom>>::into(value.borrow())
    }
}

impl From<&VecWrap<Atom>> for Vec<horned_owl::model::Atom<ArcStr>> {
    fn from(value: &VecWrap<Atom>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Atom::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Atom<ArcStr>>> for VecWrap<Atom> {
    fn from(value: &Vec<horned_owl::model::Atom<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Atom::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Atom>> for Box<horned_owl::model::Atom<ArcStr>> {
    fn from_c(value: &BoxWrap<Atom>) -> Self {
        Box::<horned_owl::model::Atom<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Atom<ArcStr>>> for BoxWrap<Atom> {
    fn from_c(value: &Box<horned_owl::model::Atom<ArcStr>>) -> Self {
        BoxWrap::<Atom>::from(value)
    }
}
impl FromCompatible<BoxWrap<Atom>> for Box<horned_owl::model::Atom<ArcStr>> {
    fn from_c(value: BoxWrap<Atom>) -> Self {
        Box::<horned_owl::model::Atom<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Atom<ArcStr>>> for BoxWrap<Atom> {
    fn from_c(value: Box<horned_owl::model::Atom<ArcStr>>) -> Self {
        BoxWrap::<Atom>::from(value)
    }
}
impl FromCompatible<VecWrap<Atom>> for Vec<horned_owl::model::Atom<ArcStr>> {
    fn from_c(value: VecWrap<Atom>) -> Self {
        Vec::<horned_owl::model::Atom<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Atom<ArcStr>>> for VecWrap<Atom> {
    fn from_c(value: Vec<horned_owl::model::Atom<ArcStr>>) -> Self {
        VecWrap::<Atom>::from(value)
    }
}
impl FromCompatible<&VecWrap<Atom>> for Vec<horned_owl::model::Atom<ArcStr>> {
    fn from_c(value: &VecWrap<Atom>) -> Self {
        Vec::<horned_owl::model::Atom<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Atom<ArcStr>>> for VecWrap<Atom> {
    fn from_c(value: &Vec<horned_owl::model::Atom<ArcStr>>) -> Self {
        VecWrap::<Atom>::from(value)
    }
}
#[doc = concat!("Rule(head: typing.List[Atom],body: typing.List[Atom],)",
    "\n\n",
    doc!(Rule)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rule {
        #[doc="head: typing.List[Atom]"]
        #[pyo3(get,set)]
        pub head: VecWrap<Atom>,
    
        #[doc="body: typing.List[Atom]"]
        #[pyo3(get,set)]
        pub body: VecWrap<Atom>,
    }

#[pymethods]
impl Rule {
    #[new]
    fn new(head: VecWrap<Atom>,body: VecWrap<Atom>,) -> Self {
        Rule {
                head,
                body,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "head" => self.head.clone().into_pyobject(py).map(Bound::into_any),
            "body" => self.body.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "head" => {
                self.head = value.extract()?;
                Ok(())
            },
            "body" => {
                self.body = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::Rule<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::Rule<ArcStr>> for Rule {
    fn from(value: &horned_owl::model::Rule<ArcStr>) -> Self {
        Rule {
            head: IntoCompatible::<VecWrap<Atom>>::into_c(value.head.borrow()),
            body: IntoCompatible::<VecWrap<Atom>>::into_c(value.body.borrow()),
        }
    }
}


impl From<&Rule> for horned_owl::model::Rule<ArcStr> {
    fn from(value: &Rule) -> Self {
        horned_owl::model::Rule::<ArcStr> {
            head: value.head.borrow().into_c(),
            body: value.body.borrow().into_c(),
        }
    }
}



/**************** Base implementations for Rule ****************/
impl FromCompatible<horned_owl::model::Rule<ArcStr>> for Rule {
    fn from_c(value: horned_owl::model::Rule<ArcStr>) -> Self {
        Rule::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Rule<ArcStr>> for Rule {
    fn from_c(value: &horned_owl::model::Rule<ArcStr>) -> Self {
        Rule::from(value)
    }
}

impl FromCompatible<Rule> for horned_owl::model::Rule<ArcStr> {
    fn from_c(value: Rule) -> Self {
        horned_owl::model::Rule::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Rule> for horned_owl::model::Rule<ArcStr> {
    fn from_c(value: &Rule) -> Self {
        horned_owl::model::Rule::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Rule<ArcStr>> for Rule {
    fn from(value: horned_owl::model::Rule<ArcStr>) -> Self {
        Rule::from(value.borrow())
    }
}

impl From<Rule> for horned_owl::model::Rule<ArcStr> {
    fn from(value: Rule) -> Self {
        horned_owl::model::Rule::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Rule>> for Box<horned_owl::model::Rule<ArcStr>> {
    fn from(value: &BoxWrap<Rule>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Rule<ArcStr>>> for BoxWrap<Rule> {
    fn from(value: &Box<horned_owl::model::Rule<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Rule>::into(*value.clone())))
    }
}

impl From<BoxWrap<Rule>> for Box<horned_owl::model::Rule<ArcStr>> {
    fn from(value: BoxWrap<Rule>) -> Self {
        Into::<Box<horned_owl::model::Rule<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Rule<ArcStr>>> for BoxWrap<Rule> {
    fn from(value: Box<horned_owl::model::Rule<ArcStr>>) -> Self {
        Into::<BoxWrap<Rule>>::into(value.borrow())
    }
}

impl From<VecWrap<Rule>> for Vec<horned_owl::model::Rule<ArcStr>> {
    fn from(value: VecWrap<Rule>) -> Self {
        Into::<Vec<horned_owl::model::Rule<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Rule<ArcStr>>> for VecWrap<Rule> {
    fn from(value: Vec<horned_owl::model::Rule<ArcStr>>) -> Self {
        Into::<VecWrap<Rule>>::into(value.borrow())
    }
}

impl From<&VecWrap<Rule>> for Vec<horned_owl::model::Rule<ArcStr>> {
    fn from(value: &VecWrap<Rule>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Rule::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Rule<ArcStr>>> for VecWrap<Rule> {
    fn from(value: &Vec<horned_owl::model::Rule<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Rule::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Rule>> for Box<horned_owl::model::Rule<ArcStr>> {
    fn from_c(value: &BoxWrap<Rule>) -> Self {
        Box::<horned_owl::model::Rule<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Rule<ArcStr>>> for BoxWrap<Rule> {
    fn from_c(value: &Box<horned_owl::model::Rule<ArcStr>>) -> Self {
        BoxWrap::<Rule>::from(value)
    }
}
impl FromCompatible<BoxWrap<Rule>> for Box<horned_owl::model::Rule<ArcStr>> {
    fn from_c(value: BoxWrap<Rule>) -> Self {
        Box::<horned_owl::model::Rule<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Rule<ArcStr>>> for BoxWrap<Rule> {
    fn from_c(value: Box<horned_owl::model::Rule<ArcStr>>) -> Self {
        BoxWrap::<Rule>::from(value)
    }
}
impl FromCompatible<VecWrap<Rule>> for Vec<horned_owl::model::Rule<ArcStr>> {
    fn from_c(value: VecWrap<Rule>) -> Self {
        Vec::<horned_owl::model::Rule<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Rule<ArcStr>>> for VecWrap<Rule> {
    fn from_c(value: Vec<horned_owl::model::Rule<ArcStr>>) -> Self {
        VecWrap::<Rule>::from(value)
    }
}
impl FromCompatible<&VecWrap<Rule>> for Vec<horned_owl::model::Rule<ArcStr>> {
    fn from_c(value: &VecWrap<Rule>) -> Self {
        Vec::<horned_owl::model::Rule<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Rule<ArcStr>>> for VecWrap<Rule> {
    fn from_c(value: &Vec<horned_owl::model::Rule<ArcStr>>) -> Self {
        VecWrap::<Rule>::from(value)
    }
}
#[derive(Debug, FromPyObject, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Component {
    
        #[pyo3(transparent)]
        OntologyID (OntologyID),
    
        #[pyo3(transparent)]
        DocIRI (DocIRI),
    
        #[pyo3(transparent)]
        OntologyAnnotation (OntologyAnnotation),
    
        #[pyo3(transparent)]
        Import (Import),
    
        #[pyo3(transparent)]
        DeclareClass (DeclareClass),
    
        #[pyo3(transparent)]
        DeclareObjectProperty (DeclareObjectProperty),
    
        #[pyo3(transparent)]
        DeclareAnnotationProperty (DeclareAnnotationProperty),
    
        #[pyo3(transparent)]
        DeclareDataProperty (DeclareDataProperty),
    
        #[pyo3(transparent)]
        DeclareNamedIndividual (DeclareNamedIndividual),
    
        #[pyo3(transparent)]
        DeclareDatatype (DeclareDatatype),
    
        #[pyo3(transparent)]
        SubClassOf (SubClassOf),
    
        #[pyo3(transparent)]
        EquivalentClasses (EquivalentClasses),
    
        #[pyo3(transparent)]
        DisjointClasses (DisjointClasses),
    
        #[pyo3(transparent)]
        DisjointUnion (DisjointUnion),
    
        #[pyo3(transparent)]
        SubObjectPropertyOf (SubObjectPropertyOf),
    
        #[pyo3(transparent)]
        EquivalentObjectProperties (EquivalentObjectProperties),
    
        #[pyo3(transparent)]
        DisjointObjectProperties (DisjointObjectProperties),
    
        #[pyo3(transparent)]
        InverseObjectProperties (InverseObjectProperties),
    
        #[pyo3(transparent)]
        ObjectPropertyDomain (ObjectPropertyDomain),
    
        #[pyo3(transparent)]
        ObjectPropertyRange (ObjectPropertyRange),
    
        #[pyo3(transparent)]
        FunctionalObjectProperty (FunctionalObjectProperty),
    
        #[pyo3(transparent)]
        InverseFunctionalObjectProperty (InverseFunctionalObjectProperty),
    
        #[pyo3(transparent)]
        ReflexiveObjectProperty (ReflexiveObjectProperty),
    
        #[pyo3(transparent)]
        IrreflexiveObjectProperty (IrreflexiveObjectProperty),
    
        #[pyo3(transparent)]
        SymmetricObjectProperty (SymmetricObjectProperty),
    
        #[pyo3(transparent)]
        AsymmetricObjectProperty (AsymmetricObjectProperty),
    
        #[pyo3(transparent)]
        TransitiveObjectProperty (TransitiveObjectProperty),
    
        #[pyo3(transparent)]
        SubDataPropertyOf (SubDataPropertyOf),
    
        #[pyo3(transparent)]
        EquivalentDataProperties (EquivalentDataProperties),
    
        #[pyo3(transparent)]
        DisjointDataProperties (DisjointDataProperties),
    
        #[pyo3(transparent)]
        DataPropertyDomain (DataPropertyDomain),
    
        #[pyo3(transparent)]
        DataPropertyRange (DataPropertyRange),
    
        #[pyo3(transparent)]
        FunctionalDataProperty (FunctionalDataProperty),
    
        #[pyo3(transparent)]
        DatatypeDefinition (DatatypeDefinition),
    
        #[pyo3(transparent)]
        HasKey (HasKey),
    
        #[pyo3(transparent)]
        SameIndividual (SameIndividual),
    
        #[pyo3(transparent)]
        DifferentIndividuals (DifferentIndividuals),
    
        #[pyo3(transparent)]
        ClassAssertion (ClassAssertion),
    
        #[pyo3(transparent)]
        ObjectPropertyAssertion (ObjectPropertyAssertion),
    
        #[pyo3(transparent)]
        NegativeObjectPropertyAssertion (NegativeObjectPropertyAssertion),
    
        #[pyo3(transparent)]
        DataPropertyAssertion (DataPropertyAssertion),
    
        #[pyo3(transparent)]
        NegativeDataPropertyAssertion (NegativeDataPropertyAssertion),
    
        #[pyo3(transparent)]
        AnnotationAssertion (AnnotationAssertion),
    
        #[pyo3(transparent)]
        SubAnnotationPropertyOf (SubAnnotationPropertyOf),
    
        #[pyo3(transparent)]
        AnnotationPropertyDomain (AnnotationPropertyDomain),
    
        #[pyo3(transparent)]
        AnnotationPropertyRange (AnnotationPropertyRange),
    
        #[pyo3(transparent)]
        Rule (Rule),
    
}

impl<'py> IntoPyObject<'py> for Component {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
        
            Component::OntologyID(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DocIRI(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::OntologyAnnotation(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::Import(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DeclareClass(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DeclareObjectProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DeclareAnnotationProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DeclareDataProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DeclareNamedIndividual(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DeclareDatatype(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::SubClassOf(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::EquivalentClasses(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DisjointClasses(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DisjointUnion(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::SubObjectPropertyOf(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::EquivalentObjectProperties(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DisjointObjectProperties(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::InverseObjectProperties(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::ObjectPropertyDomain(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::ObjectPropertyRange(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::FunctionalObjectProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::InverseFunctionalObjectProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::ReflexiveObjectProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::IrreflexiveObjectProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::SymmetricObjectProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::AsymmetricObjectProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::TransitiveObjectProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::SubDataPropertyOf(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::EquivalentDataProperties(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DisjointDataProperties(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DataPropertyDomain(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DataPropertyRange(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::FunctionalDataProperty(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DatatypeDefinition(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::HasKey(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::SameIndividual(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DifferentIndividuals(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::ClassAssertion(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::ObjectPropertyAssertion(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::NegativeObjectPropertyAssertion(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::DataPropertyAssertion(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::NegativeDataPropertyAssertion(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::AnnotationAssertion(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::SubAnnotationPropertyOf(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::AnnotationPropertyDomain(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::AnnotationPropertyRange(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
            Component::Rule(inner) => inner.into_pyobject(py).map(Bound::into_any),
        
        }
    }
}

impl From<&Component> for horned_owl::model::Component<ArcStr> {
    fn from(value: &Component) -> Self {
        match value {
        
            Component::OntologyID(inner) => horned_owl::model::Component::OntologyID(inner.into()),
        
            Component::DocIRI(inner) => horned_owl::model::Component::DocIRI(inner.into()),
        
            Component::OntologyAnnotation(inner) => horned_owl::model::Component::OntologyAnnotation(inner.into()),
        
            Component::Import(inner) => horned_owl::model::Component::Import(inner.into()),
        
            Component::DeclareClass(inner) => horned_owl::model::Component::DeclareClass(inner.into()),
        
            Component::DeclareObjectProperty(inner) => horned_owl::model::Component::DeclareObjectProperty(inner.into()),
        
            Component::DeclareAnnotationProperty(inner) => horned_owl::model::Component::DeclareAnnotationProperty(inner.into()),
        
            Component::DeclareDataProperty(inner) => horned_owl::model::Component::DeclareDataProperty(inner.into()),
        
            Component::DeclareNamedIndividual(inner) => horned_owl::model::Component::DeclareNamedIndividual(inner.into()),
        
            Component::DeclareDatatype(inner) => horned_owl::model::Component::DeclareDatatype(inner.into()),
        
            Component::SubClassOf(inner) => horned_owl::model::Component::SubClassOf(inner.into()),
        
            Component::EquivalentClasses(inner) => horned_owl::model::Component::EquivalentClasses(inner.into()),
        
            Component::DisjointClasses(inner) => horned_owl::model::Component::DisjointClasses(inner.into()),
        
            Component::DisjointUnion(inner) => horned_owl::model::Component::DisjointUnion(inner.into()),
        
            Component::SubObjectPropertyOf(inner) => horned_owl::model::Component::SubObjectPropertyOf(inner.into()),
        
            Component::EquivalentObjectProperties(inner) => horned_owl::model::Component::EquivalentObjectProperties(inner.into()),
        
            Component::DisjointObjectProperties(inner) => horned_owl::model::Component::DisjointObjectProperties(inner.into()),
        
            Component::InverseObjectProperties(inner) => horned_owl::model::Component::InverseObjectProperties(inner.into()),
        
            Component::ObjectPropertyDomain(inner) => horned_owl::model::Component::ObjectPropertyDomain(inner.into()),
        
            Component::ObjectPropertyRange(inner) => horned_owl::model::Component::ObjectPropertyRange(inner.into()),
        
            Component::FunctionalObjectProperty(inner) => horned_owl::model::Component::FunctionalObjectProperty(inner.into()),
        
            Component::InverseFunctionalObjectProperty(inner) => horned_owl::model::Component::InverseFunctionalObjectProperty(inner.into()),
        
            Component::ReflexiveObjectProperty(inner) => horned_owl::model::Component::ReflexiveObjectProperty(inner.into()),
        
            Component::IrreflexiveObjectProperty(inner) => horned_owl::model::Component::IrreflexiveObjectProperty(inner.into()),
        
            Component::SymmetricObjectProperty(inner) => horned_owl::model::Component::SymmetricObjectProperty(inner.into()),
        
            Component::AsymmetricObjectProperty(inner) => horned_owl::model::Component::AsymmetricObjectProperty(inner.into()),
        
            Component::TransitiveObjectProperty(inner) => horned_owl::model::Component::TransitiveObjectProperty(inner.into()),
        
            Component::SubDataPropertyOf(inner) => horned_owl::model::Component::SubDataPropertyOf(inner.into()),
        
            Component::EquivalentDataProperties(inner) => horned_owl::model::Component::EquivalentDataProperties(inner.into()),
        
            Component::DisjointDataProperties(inner) => horned_owl::model::Component::DisjointDataProperties(inner.into()),
        
            Component::DataPropertyDomain(inner) => horned_owl::model::Component::DataPropertyDomain(inner.into()),
        
            Component::DataPropertyRange(inner) => horned_owl::model::Component::DataPropertyRange(inner.into()),
        
            Component::FunctionalDataProperty(inner) => horned_owl::model::Component::FunctionalDataProperty(inner.into()),
        
            Component::DatatypeDefinition(inner) => horned_owl::model::Component::DatatypeDefinition(inner.into()),
        
            Component::HasKey(inner) => horned_owl::model::Component::HasKey(inner.into()),
        
            Component::SameIndividual(inner) => horned_owl::model::Component::SameIndividual(inner.into()),
        
            Component::DifferentIndividuals(inner) => horned_owl::model::Component::DifferentIndividuals(inner.into()),
        
            Component::ClassAssertion(inner) => horned_owl::model::Component::ClassAssertion(inner.into()),
        
            Component::ObjectPropertyAssertion(inner) => horned_owl::model::Component::ObjectPropertyAssertion(inner.into()),
        
            Component::NegativeObjectPropertyAssertion(inner) => horned_owl::model::Component::NegativeObjectPropertyAssertion(inner.into()),
        
            Component::DataPropertyAssertion(inner) => horned_owl::model::Component::DataPropertyAssertion(inner.into()),
        
            Component::NegativeDataPropertyAssertion(inner) => horned_owl::model::Component::NegativeDataPropertyAssertion(inner.into()),
        
            Component::AnnotationAssertion(inner) => horned_owl::model::Component::AnnotationAssertion(inner.into()),
        
            Component::SubAnnotationPropertyOf(inner) => horned_owl::model::Component::SubAnnotationPropertyOf(inner.into()),
        
            Component::AnnotationPropertyDomain(inner) => horned_owl::model::Component::AnnotationPropertyDomain(inner.into()),
        
            Component::AnnotationPropertyRange(inner) => horned_owl::model::Component::AnnotationPropertyRange(inner.into()),
        
            Component::Rule(inner) => horned_owl::model::Component::Rule(inner.into()),
        
        }
    }
}

impl From<&horned_owl::model::Component<ArcStr>> for Component {

    fn from(value: &horned_owl::model::Component<ArcStr>) -> Self {
        match value {
        
            horned_owl::model::Component::OntologyID(inner) => Component::OntologyID(inner.into()),
        
            horned_owl::model::Component::DocIRI(inner) => Component::DocIRI(inner.into()),
        
            horned_owl::model::Component::OntologyAnnotation(inner) => Component::OntologyAnnotation(inner.into()),
        
            horned_owl::model::Component::Import(inner) => Component::Import(inner.into()),
        
            horned_owl::model::Component::DeclareClass(inner) => Component::DeclareClass(inner.into()),
        
            horned_owl::model::Component::DeclareObjectProperty(inner) => Component::DeclareObjectProperty(inner.into()),
        
            horned_owl::model::Component::DeclareAnnotationProperty(inner) => Component::DeclareAnnotationProperty(inner.into()),
        
            horned_owl::model::Component::DeclareDataProperty(inner) => Component::DeclareDataProperty(inner.into()),
        
            horned_owl::model::Component::DeclareNamedIndividual(inner) => Component::DeclareNamedIndividual(inner.into()),
        
            horned_owl::model::Component::DeclareDatatype(inner) => Component::DeclareDatatype(inner.into()),
        
            horned_owl::model::Component::SubClassOf(inner) => Component::SubClassOf(inner.into()),
        
            horned_owl::model::Component::EquivalentClasses(inner) => Component::EquivalentClasses(inner.into()),
        
            horned_owl::model::Component::DisjointClasses(inner) => Component::DisjointClasses(inner.into()),
        
            horned_owl::model::Component::DisjointUnion(inner) => Component::DisjointUnion(inner.into()),
        
            horned_owl::model::Component::SubObjectPropertyOf(inner) => Component::SubObjectPropertyOf(inner.into()),
        
            horned_owl::model::Component::EquivalentObjectProperties(inner) => Component::EquivalentObjectProperties(inner.into()),
        
            horned_owl::model::Component::DisjointObjectProperties(inner) => Component::DisjointObjectProperties(inner.into()),
        
            horned_owl::model::Component::InverseObjectProperties(inner) => Component::InverseObjectProperties(inner.into()),
        
            horned_owl::model::Component::ObjectPropertyDomain(inner) => Component::ObjectPropertyDomain(inner.into()),
        
            horned_owl::model::Component::ObjectPropertyRange(inner) => Component::ObjectPropertyRange(inner.into()),
        
            horned_owl::model::Component::FunctionalObjectProperty(inner) => Component::FunctionalObjectProperty(inner.into()),
        
            horned_owl::model::Component::InverseFunctionalObjectProperty(inner) => Component::InverseFunctionalObjectProperty(inner.into()),
        
            horned_owl::model::Component::ReflexiveObjectProperty(inner) => Component::ReflexiveObjectProperty(inner.into()),
        
            horned_owl::model::Component::IrreflexiveObjectProperty(inner) => Component::IrreflexiveObjectProperty(inner.into()),
        
            horned_owl::model::Component::SymmetricObjectProperty(inner) => Component::SymmetricObjectProperty(inner.into()),
        
            horned_owl::model::Component::AsymmetricObjectProperty(inner) => Component::AsymmetricObjectProperty(inner.into()),
        
            horned_owl::model::Component::TransitiveObjectProperty(inner) => Component::TransitiveObjectProperty(inner.into()),
        
            horned_owl::model::Component::SubDataPropertyOf(inner) => Component::SubDataPropertyOf(inner.into()),
        
            horned_owl::model::Component::EquivalentDataProperties(inner) => Component::EquivalentDataProperties(inner.into()),
        
            horned_owl::model::Component::DisjointDataProperties(inner) => Component::DisjointDataProperties(inner.into()),
        
            horned_owl::model::Component::DataPropertyDomain(inner) => Component::DataPropertyDomain(inner.into()),
        
            horned_owl::model::Component::DataPropertyRange(inner) => Component::DataPropertyRange(inner.into()),
        
            horned_owl::model::Component::FunctionalDataProperty(inner) => Component::FunctionalDataProperty(inner.into()),
        
            horned_owl::model::Component::DatatypeDefinition(inner) => Component::DatatypeDefinition(inner.into()),
        
            horned_owl::model::Component::HasKey(inner) => Component::HasKey(inner.into()),
        
            horned_owl::model::Component::SameIndividual(inner) => Component::SameIndividual(inner.into()),
        
            horned_owl::model::Component::DifferentIndividuals(inner) => Component::DifferentIndividuals(inner.into()),
        
            horned_owl::model::Component::ClassAssertion(inner) => Component::ClassAssertion(inner.into()),
        
            horned_owl::model::Component::ObjectPropertyAssertion(inner) => Component::ObjectPropertyAssertion(inner.into()),
        
            horned_owl::model::Component::NegativeObjectPropertyAssertion(inner) => Component::NegativeObjectPropertyAssertion(inner.into()),
        
            horned_owl::model::Component::DataPropertyAssertion(inner) => Component::DataPropertyAssertion(inner.into()),
        
            horned_owl::model::Component::NegativeDataPropertyAssertion(inner) => Component::NegativeDataPropertyAssertion(inner.into()),
        
            horned_owl::model::Component::AnnotationAssertion(inner) => Component::AnnotationAssertion(inner.into()),
        
            horned_owl::model::Component::SubAnnotationPropertyOf(inner) => Component::SubAnnotationPropertyOf(inner.into()),
        
            horned_owl::model::Component::AnnotationPropertyDomain(inner) => Component::AnnotationPropertyDomain(inner.into()),
        
            horned_owl::model::Component::AnnotationPropertyRange(inner) => Component::AnnotationPropertyRange(inner.into()),
        
            horned_owl::model::Component::Rule(inner) => Component::Rule(inner.into()),
        
        }
    }
}


impl Component {
    pub fn py_def() -> String {
        "typing.Union[m.OntologyID,m.DocIRI,m.OntologyAnnotation,m.Import,m.DeclareClass,m.DeclareObjectProperty,m.DeclareAnnotationProperty,m.DeclareDataProperty,m.DeclareNamedIndividual,m.DeclareDatatype,m.SubClassOf,m.EquivalentClasses,m.DisjointClasses,m.DisjointUnion,m.SubObjectPropertyOf,m.EquivalentObjectProperties,m.DisjointObjectProperties,m.InverseObjectProperties,m.ObjectPropertyDomain,m.ObjectPropertyRange,m.FunctionalObjectProperty,m.InverseFunctionalObjectProperty,m.ReflexiveObjectProperty,m.IrreflexiveObjectProperty,m.SymmetricObjectProperty,m.AsymmetricObjectProperty,m.TransitiveObjectProperty,m.SubDataPropertyOf,m.EquivalentDataProperties,m.DisjointDataProperties,m.DataPropertyDomain,m.DataPropertyRange,m.FunctionalDataProperty,m.DatatypeDefinition,m.HasKey,m.SameIndividual,m.DifferentIndividuals,m.ClassAssertion,m.ObjectPropertyAssertion,m.NegativeObjectPropertyAssertion,m.DataPropertyAssertion,m.NegativeDataPropertyAssertion,m.AnnotationAssertion,m.SubAnnotationPropertyOf,m.AnnotationPropertyDomain,m.AnnotationPropertyRange,m.Rule,]".into()
    }
}



/**************** Base implementations for Component ****************/
impl FromCompatible<horned_owl::model::Component<ArcStr>> for Component {
    fn from_c(value: horned_owl::model::Component<ArcStr>) -> Self {
        Component::from(value)
    }
}

impl FromCompatible<&horned_owl::model::Component<ArcStr>> for Component {
    fn from_c(value: &horned_owl::model::Component<ArcStr>) -> Self {
        Component::from(value)
    }
}

impl FromCompatible<Component> for horned_owl::model::Component<ArcStr> {
    fn from_c(value: Component) -> Self {
        horned_owl::model::Component::<ArcStr>::from(value)
    }
}

impl FromCompatible<&Component> for horned_owl::model::Component<ArcStr> {
    fn from_c(value: &Component) -> Self {
        horned_owl::model::Component::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::Component<ArcStr>> for Component {
    fn from(value: horned_owl::model::Component<ArcStr>) -> Self {
        Component::from(value.borrow())
    }
}

impl From<Component> for horned_owl::model::Component<ArcStr> {
    fn from(value: Component) -> Self {
        horned_owl::model::Component::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<Component>> for Box<horned_owl::model::Component<ArcStr>> {
    fn from(value: &BoxWrap<Component>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::Component<ArcStr>>> for BoxWrap<Component> {
    fn from(value: &Box<horned_owl::model::Component<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<Component>::into(*value.clone())))
    }
}

impl From<BoxWrap<Component>> for Box<horned_owl::model::Component<ArcStr>> {
    fn from(value: BoxWrap<Component>) -> Self {
        Into::<Box<horned_owl::model::Component<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::Component<ArcStr>>> for BoxWrap<Component> {
    fn from(value: Box<horned_owl::model::Component<ArcStr>>) -> Self {
        Into::<BoxWrap<Component>>::into(value.borrow())
    }
}

impl From<VecWrap<Component>> for Vec<horned_owl::model::Component<ArcStr>> {
    fn from(value: VecWrap<Component>) -> Self {
        Into::<Vec<horned_owl::model::Component<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::Component<ArcStr>>> for VecWrap<Component> {
    fn from(value: Vec<horned_owl::model::Component<ArcStr>>) -> Self {
        Into::<VecWrap<Component>>::into(value.borrow())
    }
}

impl From<&VecWrap<Component>> for Vec<horned_owl::model::Component<ArcStr>> {
    fn from(value: &VecWrap<Component>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::Component::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::Component<ArcStr>>> for VecWrap<Component> {
    fn from(value: &Vec<horned_owl::model::Component<ArcStr>>) -> Self {
        VecWrap(value.iter().map(Component::from).collect())
    }
}

impl FromCompatible<&BoxWrap<Component>> for Box<horned_owl::model::Component<ArcStr>> {
    fn from_c(value: &BoxWrap<Component>) -> Self {
        Box::<horned_owl::model::Component<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::Component<ArcStr>>> for BoxWrap<Component> {
    fn from_c(value: &Box<horned_owl::model::Component<ArcStr>>) -> Self {
        BoxWrap::<Component>::from(value)
    }
}
impl FromCompatible<BoxWrap<Component>> for Box<horned_owl::model::Component<ArcStr>> {
    fn from_c(value: BoxWrap<Component>) -> Self {
        Box::<horned_owl::model::Component<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::Component<ArcStr>>> for BoxWrap<Component> {
    fn from_c(value: Box<horned_owl::model::Component<ArcStr>>) -> Self {
        BoxWrap::<Component>::from(value)
    }
}
impl FromCompatible<VecWrap<Component>> for Vec<horned_owl::model::Component<ArcStr>> {
    fn from_c(value: VecWrap<Component>) -> Self {
        Vec::<horned_owl::model::Component<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::Component<ArcStr>>> for VecWrap<Component> {
    fn from_c(value: Vec<horned_owl::model::Component<ArcStr>>) -> Self {
        VecWrap::<Component>::from(value)
    }
}
impl FromCompatible<&VecWrap<Component>> for Vec<horned_owl::model::Component<ArcStr>> {
    fn from_c(value: &VecWrap<Component>) -> Self {
        Vec::<horned_owl::model::Component<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::Component<ArcStr>>> for VecWrap<Component> {
    fn from_c(value: &Vec<horned_owl::model::Component<ArcStr>>) -> Self {
        VecWrap::<Component>::from(value)
    }
}
#[doc = concat!("AnnotatedComponent(component: Component,ann: typing.Set[Annotation],)",
    "\n\n",
    doc!(AnnotatedComponent)
)]
#[pyclass(module="pyhornedowl.model",mapping)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AnnotatedComponent {
        #[doc="component: Component"]
        #[pyo3(get,set)]
        pub component: Component,
    
        #[doc="ann: typing.Set[Annotation]"]
        #[pyo3(get,set)]
        pub ann: BTreeSetWrap<Annotation>,
    }

#[pymethods]
impl AnnotatedComponent {
    #[new]
    fn new(component: Component,ann: BTreeSetWrap<Annotation>,) -> Self {
        AnnotatedComponent {
                component,
                ann,
        }
    }

    fn __getitem__<'py>(&self, py: Python<'py>, name: &str) -> PyResult<Bound<'py, PyAny>> {
        match name {
            "component" => self.component.clone().into_pyobject(py).map(Bound::into_any),
            "ann" => self.ann.clone().into_pyobject(py).map(Bound::into_any),
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __setitem__(&mut self, name: &str, value: &Bound<'_, PyAny>) -> PyResult<()> {
        match name {
            "component" => {
                self.component = value.extract()?;
                Ok(())
            },
            "ann" => {
                self.ann = value.extract()?;
                Ok(())
            },
            &_ => Err(PyKeyError::new_err(format!("The field '{}' does not exist.", name)))
        }
    }

    fn __hash__(&self) -> u64 {
        let mut s = DefaultHasher::new();
        Hash::hash(&self, &mut s);
        s.finish()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self == other
    }
    fn __str__(&self) -> String {
        Into::<horned_owl::model::AnnotatedComponent<ArcStr>>::into(self.clone()).as_functional().to_string()
    }
}

impl From<&horned_owl::model::AnnotatedComponent<ArcStr>> for AnnotatedComponent {
    fn from(value: &horned_owl::model::AnnotatedComponent<ArcStr>) -> Self {
        AnnotatedComponent {
            component: IntoCompatible::<Component>::into_c(value.component.borrow()),
            ann: IntoCompatible::<BTreeSetWrap<Annotation>>::into_c(value.ann.borrow()),
        }
    }
}


impl From<&AnnotatedComponent> for horned_owl::model::AnnotatedComponent<ArcStr> {
    fn from(value: &AnnotatedComponent) -> Self {
        horned_owl::model::AnnotatedComponent::<ArcStr> {
            component: value.component.borrow().into_c(),
            ann: value.ann.borrow().into_c(),
        }
    }
}



/**************** Base implementations for AnnotatedComponent ****************/
impl FromCompatible<horned_owl::model::AnnotatedComponent<ArcStr>> for AnnotatedComponent {
    fn from_c(value: horned_owl::model::AnnotatedComponent<ArcStr>) -> Self {
        AnnotatedComponent::from(value)
    }
}

impl FromCompatible<&horned_owl::model::AnnotatedComponent<ArcStr>> for AnnotatedComponent {
    fn from_c(value: &horned_owl::model::AnnotatedComponent<ArcStr>) -> Self {
        AnnotatedComponent::from(value)
    }
}

impl FromCompatible<AnnotatedComponent> for horned_owl::model::AnnotatedComponent<ArcStr> {
    fn from_c(value: AnnotatedComponent) -> Self {
        horned_owl::model::AnnotatedComponent::<ArcStr>::from(value)
    }
}

impl FromCompatible<&AnnotatedComponent> for horned_owl::model::AnnotatedComponent<ArcStr> {
    fn from_c(value: &AnnotatedComponent) -> Self {
        horned_owl::model::AnnotatedComponent::<ArcStr>::from(value)
    }
}

impl From<horned_owl::model::AnnotatedComponent<ArcStr>> for AnnotatedComponent {
    fn from(value: horned_owl::model::AnnotatedComponent<ArcStr>) -> Self {
        AnnotatedComponent::from(value.borrow())
    }
}

impl From<AnnotatedComponent> for horned_owl::model::AnnotatedComponent<ArcStr> {
    fn from(value: AnnotatedComponent) -> Self {
        horned_owl::model::AnnotatedComponent::<ArcStr>::from(value.borrow())
    }
}

impl From<&BoxWrap<AnnotatedComponent>> for Box<horned_owl::model::AnnotatedComponent<ArcStr>> {
    fn from(value: &BoxWrap<AnnotatedComponent>) -> Self {
        Box::new((*value.0.clone()).into())
    }
}

impl From<&Box<horned_owl::model::AnnotatedComponent<ArcStr>>> for BoxWrap<AnnotatedComponent> {
    fn from(value: &Box<horned_owl::model::AnnotatedComponent<ArcStr>>) -> Self {
        BoxWrap(Box::new(Into::<AnnotatedComponent>::into(*value.clone())))
    }
}

impl From<BoxWrap<AnnotatedComponent>> for Box<horned_owl::model::AnnotatedComponent<ArcStr>> {
    fn from(value: BoxWrap<AnnotatedComponent>) -> Self {
        Into::<Box<horned_owl::model::AnnotatedComponent<ArcStr>>>::into(value.borrow())
    }
}

impl From<Box<horned_owl::model::AnnotatedComponent<ArcStr>>> for BoxWrap<AnnotatedComponent> {
    fn from(value: Box<horned_owl::model::AnnotatedComponent<ArcStr>>) -> Self {
        Into::<BoxWrap<AnnotatedComponent>>::into(value.borrow())
    }
}

impl From<VecWrap<AnnotatedComponent>> for Vec<horned_owl::model::AnnotatedComponent<ArcStr>> {
    fn from(value: VecWrap<AnnotatedComponent>) -> Self {
        Into::<Vec<horned_owl::model::AnnotatedComponent<ArcStr>>>::into(value.borrow())
    }
}

impl From<Vec<horned_owl::model::AnnotatedComponent<ArcStr>>> for VecWrap<AnnotatedComponent> {
    fn from(value: Vec<horned_owl::model::AnnotatedComponent<ArcStr>>) -> Self {
        Into::<VecWrap<AnnotatedComponent>>::into(value.borrow())
    }
}

impl From<&VecWrap<AnnotatedComponent>> for Vec<horned_owl::model::AnnotatedComponent<ArcStr>> {
    fn from(value: &VecWrap<AnnotatedComponent>) -> Self {
        value
            .0
            .iter()
            .map(horned_owl::model::AnnotatedComponent::<ArcStr>::from)
            .collect()
    }
}
impl From<&Vec<horned_owl::model::AnnotatedComponent<ArcStr>>> for VecWrap<AnnotatedComponent> {
    fn from(value: &Vec<horned_owl::model::AnnotatedComponent<ArcStr>>) -> Self {
        VecWrap(value.iter().map(AnnotatedComponent::from).collect())
    }
}

impl FromCompatible<&BoxWrap<AnnotatedComponent>> for Box<horned_owl::model::AnnotatedComponent<ArcStr>> {
    fn from_c(value: &BoxWrap<AnnotatedComponent>) -> Self {
        Box::<horned_owl::model::AnnotatedComponent<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Box<horned_owl::model::AnnotatedComponent<ArcStr>>> for BoxWrap<AnnotatedComponent> {
    fn from_c(value: &Box<horned_owl::model::AnnotatedComponent<ArcStr>>) -> Self {
        BoxWrap::<AnnotatedComponent>::from(value)
    }
}
impl FromCompatible<BoxWrap<AnnotatedComponent>> for Box<horned_owl::model::AnnotatedComponent<ArcStr>> {
    fn from_c(value: BoxWrap<AnnotatedComponent>) -> Self {
        Box::<horned_owl::model::AnnotatedComponent<ArcStr>>::from(value)
    }
}
impl FromCompatible<Box<horned_owl::model::AnnotatedComponent<ArcStr>>> for BoxWrap<AnnotatedComponent> {
    fn from_c(value: Box<horned_owl::model::AnnotatedComponent<ArcStr>>) -> Self {
        BoxWrap::<AnnotatedComponent>::from(value)
    }
}
impl FromCompatible<VecWrap<AnnotatedComponent>> for Vec<horned_owl::model::AnnotatedComponent<ArcStr>> {
    fn from_c(value: VecWrap<AnnotatedComponent>) -> Self {
        Vec::<horned_owl::model::AnnotatedComponent<ArcStr>>::from(value)
    }
}
impl FromCompatible<Vec<horned_owl::model::AnnotatedComponent<ArcStr>>> for VecWrap<AnnotatedComponent> {
    fn from_c(value: Vec<horned_owl::model::AnnotatedComponent<ArcStr>>) -> Self {
        VecWrap::<AnnotatedComponent>::from(value)
    }
}
impl FromCompatible<&VecWrap<AnnotatedComponent>> for Vec<horned_owl::model::AnnotatedComponent<ArcStr>> {
    fn from_c(value: &VecWrap<AnnotatedComponent>) -> Self {
        Vec::<horned_owl::model::AnnotatedComponent<ArcStr>>::from(value)
    }
}
impl FromCompatible<&Vec<horned_owl::model::AnnotatedComponent<ArcStr>>> for VecWrap<AnnotatedComponent> {
    fn from_c(value: &Vec<horned_owl::model::AnnotatedComponent<ArcStr>>) -> Self {
        VecWrap::<AnnotatedComponent>::from(value)
    }
}
