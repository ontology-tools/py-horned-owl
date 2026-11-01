//! The named entities, which an IRI identifies, and individuals.

use py_horned_owl_macros::{model, model_enum};

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

// An `IRI` or a `str` names an individual.
#[model_enum(wraps, coerce = entity_from_iri::<NamedIndividual>, accepts = [IRI_OR_STR])]
pub enum Individual {
    Anonymous(AnonymousIndividual),
    Named(NamedIndividual),
}
