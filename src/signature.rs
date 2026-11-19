//! Compact, immutable signature snapshots with lazily constructed Python entities.

use crate::model::protocol::named_entity;
use crate::model::{
    AnnotationProperty, Class, DataProperty, Datatype, EntityKind, NamedIndividual,
    ObjectProperty,
};
use horned_owl::model::{ArcStr, IRI};
use horned_owl::visitor::immutable::{Visit, Walk};
use pyo3::exceptions::{PyStopIteration, PyTypeError};
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;
use std::collections::{HashMap, HashSet};

/// An immutable snapshot of an ontology's signature.
///
/// :ivar classes: Read-only set of classes.
/// :ivar named_individuals: Read-only set of named individuals.
/// :ivar object_properties: Read-only set of object properties.
/// :ivar data_properties: Read-only set of data properties.
/// :ivar annotation_properties: Read-only set of annotation properties.
/// :ivar datatypes: Read-only set of datatypes.
#[pyclass(module = "pyhornedowl", frozen)]
#[derive(Default)]
pub struct Signature {
    classes: HashSet<IRI<ArcStr>>,
    named_individuals: HashSet<IRI<ArcStr>>,
    object_properties: HashSet<IRI<ArcStr>>,
    data_properties: HashSet<IRI<ArcStr>>,
    annotation_properties: HashSet<IRI<ArcStr>>,
    datatypes: HashSet<IRI<ArcStr>>,
}

macro_rules! sig_entity_getter {
    ($field:expr) => {
        $field
            .iter()
            .map(|iri| From::from(iri.clone()))
            .collect::<HashSet<_>>()
    };
}

#[pymethods]
impl Signature {
    fn __len__(&self) -> usize {
        self.classes.len()
            + self.named_individuals.len()
            + self.object_properties.len()
            + self.data_properties.len()
            + self.annotation_properties.len()
            + self.datatypes.len()
    }
    fn __contains__(&self, entity: &Bound<'_, PyAny>) -> PyResult<bool> {
        match named_entity(entity) {
            Ok((kind, iri)) => Ok(match kind {
                EntityKind::Class => self.classes.contains(&iri.0),
                EntityKind::NamedIndividual => self.named_individuals.contains(&iri.0),
                EntityKind::ObjectProperty => self.object_properties.contains(&iri.0),
                EntityKind::DataProperty => self.data_properties.contains(&iri.0),
                EntityKind::AnnotationProperty => self.annotation_properties.contains(&iri.0),
                EntityKind::Datatype => self.datatypes.contains(&iri.0),
            }),
            Err(e) if e.is_instance_of::<PyTypeError>(entity.py()) => Ok(false),
            Err(e) => Err(e),
        }
    }
    fn __iter__(&self) -> SignatureIterator {
        macro_rules! collect {
            ($set1:expr, $kind1:expr, $($set:expr, $kind:expr,)*) => {
                $set1.iter().map(|iri| ($kind1, iri.clone()))
                $(.chain($set.iter().map(|iri| ($kind, iri.clone()))))*
                .collect::<Vec<_>>()
                .into_iter()
            };
        }

        SignatureIterator {
            inner: collect!(
                &self.classes,
                EntityKind::Class,
                &self.named_individuals,
                EntityKind::NamedIndividual,
                &self.object_properties,
                EntityKind::ObjectProperty,
                &self.data_properties,
                EntityKind::DataProperty,
                &self.annotation_properties,
                EntityKind::AnnotationProperty,
                &self.datatypes,
                EntityKind::Datatype,
            ),
        }
    }

    fn classes(&self) -> HashSet<Class> {
        sig_entity_getter!(self.classes)
    }

    fn named_individuals(&self) -> HashSet<NamedIndividual> {
        sig_entity_getter!(self.named_individuals)
    }

    fn object_properties(&self) -> HashSet<ObjectProperty> {
        sig_entity_getter!(self.object_properties)
    }

    fn data_properties(&self) -> HashSet<DataProperty> {
        sig_entity_getter!(self.data_properties)
    }

    fn annotation_properties(&self) -> HashSet<AnnotationProperty> {
        sig_entity_getter!(self.annotation_properties)
    }

    fn datatypes(&self) -> HashSet<Datatype> {
        sig_entity_getter!(self.datatypes)
    }
}

impl Signature {
    pub(crate) fn collect<'a>(
        components: impl Iterator<Item = &'a horned_owl::model::AnnotatedComponent<ArcStr>>,
    ) -> Self {
        let mut walk = Walk::new(SignatureCollector::default());
        for component in components {
            walk.annotated_component(component);
        }
        walk.into_visit().finish()
    }
}

/// Iterates over a snapshot in entity-kind order and then lexical IRI order.
#[pyclass(module = "pyhornedowl")]
pub struct SignatureIterator {
    inner: std::vec::IntoIter<(EntityKind, IRI<ArcStr>)>,
}

#[pymethods]
impl SignatureIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.inner.next().map_or_else(
            || Err(PyStopIteration::new_err(())),
            |(kind, iri)| match kind {
                EntityKind::Class => Class::from(iri).into_bound_py_any(py),
                EntityKind::NamedIndividual => NamedIndividual::from(iri).into_bound_py_any(py),
                EntityKind::ObjectProperty => ObjectProperty::from(iri).into_bound_py_any(py),
                EntityKind::DataProperty => DataProperty::from(iri).into_bound_py_any(py),
                EntityKind::AnnotationProperty => {
                    AnnotationProperty::from(iri).into_bound_py_any(py)
                }
                EntityKind::Datatype => Datatype::from(iri).into_bound_py_any(py),
            },
        )
    }
}

/// The same visitor serves collection and early-exit membership queries.
#[derive(Default)]
pub(crate) struct SignatureCollector {
    data: HashMap<EntityKind, HashSet<IRI<ArcStr>>>,
    target: Option<(EntityKind, IRI<ArcStr>)>,
    pub(crate) found: bool,
}

impl SignatureCollector {
    pub(crate) fn looking_for(kind: EntityKind, iri: IRI<ArcStr>) -> Self {
        Self {
            target: Some((kind, iri)),
            ..Self::default()
        }
    }
    fn add(&mut self, kind: EntityKind, iri: &IRI<ArcStr>) {
        if let Some((target_kind, target_iri)) = &self.target {
            self.found |= *target_kind == kind && target_iri == iri;
        } else {
            self.data
                .entry(kind)
                .or_insert_with(HashSet::new)
                .insert(iri.clone());
        }
    }
    pub(crate) fn finish(mut self) -> Signature {
        Signature {
            classes: self.data.remove(&EntityKind::Class).unwrap_or_default(),
            named_individuals: self
                .data
                .remove(&EntityKind::NamedIndividual)
                .unwrap_or_default(),
            object_properties: self
                .data
                .remove(&EntityKind::ObjectProperty)
                .unwrap_or_default(),
            data_properties: self
                .data
                .remove(&EntityKind::DataProperty)
                .unwrap_or_default(),
            annotation_properties: self
                .data
                .remove(&EntityKind::AnnotationProperty)
                .unwrap_or_default(),
            datatypes: self.data.remove(&EntityKind::Datatype).unwrap_or_default(),
        }
    }
}

impl Visit<ArcStr> for SignatureCollector {
    fn visit_class(&mut self, value: &horned_owl::model::Class<ArcStr>) {
        self.add(EntityKind::Class, &value.0);
    }
    fn visit_named_individual(&mut self, value: &horned_owl::model::NamedIndividual<ArcStr>) {
        self.add(EntityKind::NamedIndividual, &value.0);
    }
    fn visit_object_property(&mut self, value: &horned_owl::model::ObjectProperty<ArcStr>) {
        self.add(EntityKind::ObjectProperty, &value.0);
    }
    fn visit_data_property(&mut self, value: &horned_owl::model::DataProperty<ArcStr>) {
        self.add(EntityKind::DataProperty, &value.0);
    }
    fn visit_annotation_property(&mut self, value: &horned_owl::model::AnnotationProperty<ArcStr>) {
        self.add(EntityKind::AnnotationProperty, &value.0);
    }
    fn visit_datatype(&mut self, value: &horned_owl::model::Datatype<ArcStr>) {
        self.add(EntityKind::Datatype, &value.0);
    }
    fn visit_literal(&mut self, value: &horned_owl::model::Literal<ArcStr>) {
        use horned_owl::model::Literal;
        let iri = match value {
            Literal::Datatype { datatype_iri, .. } => datatype_iri.clone(),
            Literal::Simple { .. } => crate::vocab::XSD::string().into(),
            Literal::Language { .. } => crate::vocab::RDF::langString().into(),
        };
        self.add(EntityKind::Datatype, &iri);
    }
}
