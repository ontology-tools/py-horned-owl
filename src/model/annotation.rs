use std::collections::BTreeSet;

use horned_owl::model::ArcStr;
use py_horned_owl_macros::{model, model_union};

use crate::wrappers::{BTreeSetWrap, FromCompatible};

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

impl From<&BTreeSet<horned_owl::model::Annotation<ArcStr>>> for BTreeSetWrap<Annotation> {
    fn from(value: &BTreeSet<horned_owl::model::Annotation<ArcStr>>) -> Self {
        BTreeSetWrap(value.iter().map(From::from).collect())
    }
}

impl From<&BTreeSetWrap<Annotation>> for BTreeSet<horned_owl::model::Annotation<ArcStr>> {
    fn from(value: &BTreeSetWrap<Annotation>) -> Self {
        value.0.iter().map(From::from).collect()
    }
}

impl FromCompatible<&BTreeSet<horned_owl::model::Annotation<ArcStr>>> for BTreeSetWrap<Annotation> {
    fn from_c(value: &BTreeSet<horned_owl::model::Annotation<ArcStr>>) -> Self {
        value.into()
    }
}

impl FromCompatible<&BTreeSetWrap<Annotation>> for BTreeSet<horned_owl::model::Annotation<ArcStr>> {
    fn from_c(value: &BTreeSetWrap<Annotation>) -> Self {
        value.into()
    }
}
