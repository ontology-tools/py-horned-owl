use std::{borrow::Borrow, collections::{BTreeSet,hash_map::DefaultHasher}, sync::Arc};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use horned_owl::io::ofn::writer::AsFunctional;

use horned_owl::model::ArcStr;
use pyo3::{exceptions::PyKeyError, prelude::*, PyObject, types::PyType};

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