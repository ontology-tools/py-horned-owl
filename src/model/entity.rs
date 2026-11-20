//! The named entities, which an IRI identifies, and individuals.

use horned_owl::model::ArcStr;
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

/// The kind of a named OWL entity, independent of its IRI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[pyclass(module = "pyhornedowl.model", frozen, eq, hash, from_py_object)]
pub enum EntityKind {
    Class,
    NamedIndividual,
    ObjectProperty,
    DataProperty,
    AnnotationProperty,
    Datatype,
}

impl EntityKind {
    pub fn index(self) -> usize {
        self as usize
    }

    pub const ALL: [Self; 6] = [
        Self::Class,
        Self::NamedIndividual,
        Self::ObjectProperty,
        Self::DataProperty,
        Self::AnnotationProperty,
        Self::Datatype,
    ];
}

impl From<horned_owl::model::NamedOWLEntityKind> for EntityKind {
    fn from(kind: horned_owl::model::NamedOWLEntityKind) -> Self {
        match kind {
            horned_owl::model::NamedOWLEntityKind::Class => Self::Class,
            horned_owl::model::NamedOWLEntityKind::NamedIndividual => Self::NamedIndividual,
            horned_owl::model::NamedOWLEntityKind::ObjectProperty => Self::ObjectProperty,
            horned_owl::model::NamedOWLEntityKind::DataProperty => Self::DataProperty,
            horned_owl::model::NamedOWLEntityKind::AnnotationProperty => Self::AnnotationProperty,
            horned_owl::model::NamedOWLEntityKind::Datatype => Self::Datatype,
        }
    }
}

impl From<EntityKind> for horned_owl::model::NamedEntityKind {
    fn from(kind: EntityKind) -> Self {
        match kind {
            EntityKind::Class => Self::Class,
            EntityKind::NamedIndividual => Self::NamedIndividual,
            EntityKind::ObjectProperty => Self::ObjectProperty,
            EntityKind::DataProperty => Self::DataProperty,
            EntityKind::AnnotationProperty => Self::AnnotationProperty,
            EntityKind::Datatype => Self::Datatype,
        }
    }
}

impl From<EntityKind> for horned_owl::model::ComponentKind {
    fn from(kind: EntityKind) -> Self {
        match kind {
            EntityKind::Class => Self::DeclareClass,
            EntityKind::ObjectProperty => Self::DeclareObjectProperty,
            EntityKind::DataProperty => Self::DeclareDataProperty,
            EntityKind::AnnotationProperty => Self::DeclareAnnotationProperty,
            EntityKind::NamedIndividual => Self::DeclareNamedIndividual,
            EntityKind::Datatype => Self::DeclareDatatype,
        }
    }
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

macro_rules! impl_entity_from_iri {
    ($entity:ident) => {
        impl From<horned_owl::model::IRI<ArcStr>> for $entity {
            fn from(iri: horned_owl::model::IRI<ArcStr>) -> Self {
                Self { iri: iri.into() }
            }
        }

        impl From<IRI> for horned_owl::model::$entity<ArcStr> {
            fn from(value: IRI) -> Self {
                horned_owl::model::$entity::from(value.0)
            }
        }
    };
}

impl_entity_from_iri!(Class);
impl_entity_from_iri!(ObjectProperty);
impl_entity_from_iri!(DataProperty);
impl_entity_from_iri!(AnnotationProperty);
impl_entity_from_iri!(Datatype);
impl_entity_from_iri!(NamedIndividual);
