//! The named entities, which an IRI identifies, and individuals.

use py_horned_owl_macros::{model, model_enum};
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::model::protocol::{entity_from_iri, IRI_OR_STR};
use crate::wrappers::StringWrapper;

use super::{ClassExpressionBase, DataRangeBase, ObjectPropertyExpressionBase, IRI};

#[model(entity, extends = ClassExpressionBase)]
pub struct Class {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model(entity, extends = ObjectPropertyExpressionBase)]
pub struct ObjectProperty {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model(entity, render = direct)]
pub struct DataProperty {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model(entity, render = functional)]
pub struct AnnotationProperty {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model(entity, extends = DataRangeBase)]
pub struct Datatype {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model(entity, extends = IndividualBase)]
pub struct NamedIndividual {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model(extends = IndividualBase, str = name)]
pub struct AnonymousIndividual {
    #[model(horned = 0)]
    pub name: StringWrapper,
}

pub(crate) fn anonymous_from_python(
    ob: &Bound<'_, PyAny>,
) -> PyResult<Option<AnonymousIndividual>> {
    if let Ok(s) = ob.cast::<PyString>() {
        let name = s.to_str()?;
        if name.starts_with("_:") {
            return Ok(Some(AnonymousIndividual {
                name: name.to_owned().into(),
            }));
        }
    }
    Ok(None)
}

fn individual_from_python(ob: &Bound<'_, PyAny>) -> PyResult<Option<Individual>> {
    if let Some(value) = anonymous_from_python(ob)? {
        return Ok(Some(value.into()));
    }
    Ok(entity_from_iri::<NamedIndividual>(ob)?.map(Into::into))
}

#[model_enum(wraps, coerce = individual_from_python, accepts = [IRI_OR_STR])]
pub enum Individual {
    Anonymous(AnonymousIndividual),
    Named(NamedIndividual),
}
