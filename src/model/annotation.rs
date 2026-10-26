use py_horned_owl_macros::{model, model_union};

use crate::wrappers::BTreeSetWrap;

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

// A `str` is a literal, not an IRI: annotations are mostly labels and
// comments.
#[model_union]
pub enum AnnotationValue {
    Literal(Literal),
    IRI(IRI),
    AnonymousIndividual(AnonymousIndividual),
}

#[model_union]
pub enum AnnotationSubject {
    IRI(IRI),
    AnonymousIndividual(AnonymousIndividual),
}
