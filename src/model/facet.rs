use std::borrow::Borrow;

use horned_owl::io::ofn::writer::AsFunctional;
use horned_owl::model::ArcStr;
use pyo3::prelude::*;

use crate::wrappers::*;

impl FromCounterpart<horned_owl::vocab::Facet> for Facet {
    fn from_counterpart(value: &horned_owl::vocab::Facet) -> Self {
        Facet::from(value)
    }
}

impl FromCounterpart<Facet> for horned_owl::vocab::Facet {
    fn from_counterpart(value: &Facet) -> Self {
        horned_owl::vocab::Facet::from(value)
    }
}

#[doc = doc!(Facet)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(module = "pyhornedowl.model", frozen, eq, ord, hash, from_py_object)]
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
    fn __reduce__<'py>(
        slf: &Bound<'py, Self>,
    ) -> PyResult<(Bound<'py, PyAny>, (Bound<'py, pyo3::types::PyType>, String))> {
        let getattr = slf.py().import("builtins")?.getattr("getattr")?;
        Ok((
            getattr,
            (slf.py().get_type::<Self>(), format!("{:?}", slf.get())),
        ))
    }

    /// Renders this element on its own, in OWL functional (`"ofn"`) syntax.
    ///
    /// horned-owl has no Manchester rendering for a facet: Manchester writes it
    /// as an operator inside the datatype restriction that holds it.
    #[pyo3(signature = (serialization = None, prefix_mapping = None))]
    pub fn serialize(
        &self,
        serialization: Option<LiteralStr<FunctionalSyntax>>,
        prefix_mapping: Option<&crate::prefix_mapping::PrefixMapping>,
    ) -> PyResult<String> {
        let value = horned_owl::vocab::Facet::from(self);

        Ok(
            match crate::snippet::parse_syntax(serialization.as_deref().unwrap_or("ofn"))? {
                crate::snippet::SnippetSyntax::Manchester => {
                    return Err(crate::snippet::no_manchester_rendering("Facet"))
                }
                // Facet is not generic over `A`, so `AsFunctional<A>` needs naming.
                crate::snippet::SnippetSyntax::Functional => match prefix_mapping {
                    Some(pm) => AsFunctional::<ArcStr>::as_functional_with_prefixes(&value, &pm.0)
                        .to_string(),
                    None => AsFunctional::<ArcStr>::as_functional(&value).to_string(),
                },
            },
        )
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
