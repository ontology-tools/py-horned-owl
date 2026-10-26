//! The named entities, which an IRI identifies, and individuals.

use py_horned_owl_macros::{model, model_enum};

use crate::model::protocol::entity_from_str;
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

// A `str` names an individual.
#[model_enum(wraps, coerce = entity_from_str::<NamedIndividual>, accepts = [String])]
pub enum Individual {
    Anonymous(AnonymousIndividual),
    Named(NamedIndividual),
}
