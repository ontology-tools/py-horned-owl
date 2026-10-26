//! SWRL rules' atoms and their arguments.

use py_horned_owl_macros::{model, model_enum, model_union};

use crate::wrappers::VecWrap;

use super::{
    ClassExpression, DataProperty, DataRange, Individual, Literal, ObjectPropertyExpression, IRI,
};

#[model(render = direct)]
pub struct Variable {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model_union]
pub enum IArgument {
    Individual(Individual),
    Variable(Variable),
}

#[model_union]
pub enum DArgument {
    Literal(Literal),
    Variable(Variable),
}

#[model_enum]
pub enum Atom {
    BuiltInAtom(BuiltInAtom),
    ClassAtom(ClassAtom),
    DataPropertyAtom(DataPropertyAtom),
    DataRangeAtom(DataRangeAtom),
    DifferentIndividualsAtom(DifferentIndividualsAtom),
    ObjectPropertyAtom(ObjectPropertyAtom),
    SameIndividualAtom(SameIndividualAtom),
}

#[model(variant_of = Atom)]
pub struct BuiltInAtom {
    #[model(horned = pred)]
    pub predicate: IRI,
    #[model(horned = args)]
    pub arguments: VecWrap<DArgument>,
}

#[model(variant_of = Atom)]
pub struct ClassAtom {
    #[model(horned = pred)]
    pub predicate: ClassExpression,
    #[model(horned = arg)]
    pub argument: IArgument,
}

#[model(variant_of = Atom)]
pub struct DataPropertyAtom {
    #[model(horned = pred)]
    pub predicate: DataProperty,
    #[model(horned = args)]
    pub arguments: (DArgument, DArgument),
}

#[model(variant_of = Atom)]
pub struct DataRangeAtom {
    #[model(horned = pred)]
    pub predicate: DataRange,
    #[model(horned = arg)]
    pub argument: DArgument,
}

#[model(variant_of = Atom)]
pub struct DifferentIndividualsAtom {
    #[model(horned = 0)]
    pub first: IArgument,
    #[model(horned = 1)]
    pub second: IArgument,
}

#[model(variant_of = Atom)]
pub struct ObjectPropertyAtom {
    #[model(horned = pred)]
    pub predicate: ObjectPropertyExpression,
    #[model(horned = args)]
    pub arguments: (IArgument, IArgument),
}

#[model(variant_of = Atom)]
pub struct SameIndividualAtom {
    #[model(horned = 0)]
    pub first: IArgument,
    #[model(horned = 1)]
    pub second: IArgument,
}
