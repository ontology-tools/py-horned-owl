use py_horned_owl_macros::{model, model_union};

use crate::wrappers::BTreeSetWrap;

use super::entity::anonymous_from_python;
use super::{AnnotationProperty, AnonymousIndividual, Literal, IRI};

#[model(render = functional)]
pub struct Annotation {
    #[model(horned = ap)]
    pub property: AnnotationProperty,
    #[model(horned = av)]
    pub value: AnnotationValue,
    #[model(horned = ann, default = BTreeSetWrap::default())]
    pub annotations: BTreeSetWrap<Annotation>,
}

// Ordinary strings are literals; `_:` labels identify anonymous individuals.
#[model_union(coerce = anonymous_from_python)]
pub enum AnnotationValue {
    Literal(Literal),
    IRI(IRI),
    AnonymousIndividual(AnonymousIndividual),
}

#[model_union(coerce = anonymous_from_python)]
pub enum AnnotationSubject {
    IRI(IRI),
    AnonymousIndividual(AnonymousIndividual),
}

impl From<AnonymousIndividual> for AnnotationValue {
    fn from(value: AnonymousIndividual) -> Self {
        Self::AnonymousIndividual(value)
    }
}

impl From<AnonymousIndividual> for AnnotationSubject {
    fn from(value: AnonymousIndividual) -> Self {
        Self::AnonymousIndividual(value)
    }
}
