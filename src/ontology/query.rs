use std::collections::HashSet;

use horned_owl::{
    model::{
        AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue,
        ArcStr, Class, ClassExpression, Component, ComponentKind, HigherKinded, Kinded, Literal,
        SubClassOf, IRI,
    },
    visitor::immutable::Walk,
    vocab::{AnnotationBuiltIn, OWL},
};
use pyhornedowlreasoner::{PyReasoner, Reasoner};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::{
    model::{self, protocol::named_entity},
    signature::{Signature, SignatureCollector},
    structural_reasoner::StructuralReasoner,
    to_py_err, IndexCreationStrategy, PyIndexedOntology,
};

macro_rules! entity_query {
    ($s:ident, $kind:expr, $comp:pat => $comp_var:expr) => {{
        //Get the declaration axioms
        let entities = if let Some(ref mut component_index) = &mut $s.component_index {
            Box::new(component_index.component_for_kind($kind))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&$s.set_index).into_iter())
        };

        let entities = entities
            .filter_map(|aax| match &aax.component {
                $comp => Some($comp_var.0 .0.to_string()),
                _ => None,
            })
            .collect();

        Ok(entities)
    }};
}

#[pymethods]
impl PyIndexedOntology {
    /// Returns the IRI of a term by its label if it exists.
    ///
    /// If the term does not have a label, `None` is returned.
    pub fn get_iri_for_label(&mut self, label: String) -> PyResult<Option<String>> {
        let components = if let Some(ref mut component_index) = &mut self.component_index {
            Box::new(component_index.annotation_assertion())
                as Box<dyn Iterator<Item = &AnnotationAssertion<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter().filter_map(|c| match c {
                AnnotatedComponent {
                    component: Component::AnnotationAssertion(a),
                    ..
                } => Some(a),
                _ => None,
            }))
        };

        let mut labels = components.filter_map(|ax| match ax {
            AnnotationAssertion {
                subject: AnnotationSubject::IRI(subj),
                ann:
                    Annotation {
                        ap,
                        av:
                            AnnotationValue::Literal(
                                Literal::Simple { literal } | Literal::Language { literal, .. },
                            ),
                        ann,
                    },
            } if *literal == label
                && AnnotationBuiltIn::Label.underlying().eq(&ap.0.to_string()) =>
            {
                Some(subj)
            }
            _ => None,
        });

        Ok(labels.next().map(|i| i.to_string()))
    }

    /// Gets all asserted (named) direct subclasses of a class.
    #[pyo3[name="get_subclasses", signature = (iri)]]
    pub fn py_get_subclasses(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<HashSet<String>> {
        let iri: IRI<ArcStr> = into_iri!(self, py, iri);

        let classes: HashSet<String> = self
            .get_subclasses(&iri)
            .into_iter()
            .filter_map(|ce| match ce {
                ClassExpression::Class(Class(i)) => Some(i.to_string()),
                _ => None,
            })
            .collect();

        Ok(classes)
    }

    /// Gets all asserted (named) direct superclasses of a class.
    #[pyo3[name="get_superclasses", signature = (iri)]]
    pub fn py_get_superclasses(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<HashSet<String>> {
        let iri: IRI<ArcStr> = into_iri!(self, py, iri);

        let classes: HashSet<String> = self
            .get_superclasses(&iri)
            .into_iter()
            .filter_map(|ce| match ce {
                ClassExpression::Class(Class(i)) => Some(i.to_string()),
                _ => None,
            })
            .collect();

        Ok(classes)
    }

    /// Gets all (named) root classes, i.e. all classes with no superclasses (except owl:Thing).
    pub fn get_root_classes(&mut self) -> PyResult<HashSet<String>> {
        let owl_thing = self
            .build
            .read()
            .map_err(to_py_err!("Cannot get build instance!"))?
            .class(OWL::Thing);

        let component_index = match self.component_index.as_ref() {
            Some(c) => c,
            _ => {
                self.build_component_index();
                self.component_index
                    .as_ref()
                    .ok_or_else(|| PyValueError::new_err("Component index could not be created!"))?
            }
        };

        Ok(
            StructuralReasoner::get_direct_subclasses_of_iri(component_index, &owl_thing)
                .map(|c| c.0.to_string())
                .collect(),
        )
    }

    /// Returns the IRIs of all declared classes in the ontology.
    pub fn get_classes(&mut self) -> PyResult<HashSet<String>> {
        entity_query!(self, ComponentKind::DeclareClass, Component::DeclareClass(x) => x)
    }

    /// Returns the IRIs of all declared datatypes in the ontology.
    pub fn get_datatypes(&mut self) -> PyResult<HashSet<String>> {
        entity_query!(self, ComponentKind::DeclareDatatype, Component::DeclareDatatype(x) => x)
    }

    /// Returns the IRIs of all declared object properties in the ontology.
    pub fn get_object_properties(&mut self) -> PyResult<HashSet<String>> {
        entity_query!(self, ComponentKind::DeclareObjectProperty, Component::DeclareObjectProperty(x) => x)
    }

    /// Returns the IRIs of all declared annotation properties in the ontology.
    pub fn get_annotation_properties(&mut self) -> PyResult<HashSet<String>> {
        entity_query!(self, ComponentKind::DeclareAnnotationProperty, Component::DeclareAnnotationProperty(x) => x)
    }

    /// Returns the IRIs of all declared data properties in the ontology.
    pub fn get_data_properties(&mut self) -> PyResult<HashSet<String>> {
        entity_query!(self, ComponentKind::DeclareDataProperty, Component::DeclareDataProperty(x) => x)
    }

    /// Returns the IRIs of all declared named individuals in the ontology.
    pub fn get_named_individuals(&mut self) -> PyResult<HashSet<String>> {
        entity_query!(self, ComponentKind::DeclareNamedIndividual, Component::DeclareNamedIndividual(x) => x)
    }

    /// Calcuates the signature of the ontology.
    pub fn signature(&self) -> Signature {
        Signature::collect(self.iter())
    }

    /// Gets all axioms for an entity.
    #[pyo3[signature = (iri)]]
    pub fn get_axioms_for_iri(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<Vec<model::AnnotatedComponent>> {
        let iri: IRI<ArcStr> = into_iri!(self, py, iri);

        let iri_index = if let Some(index) = &self.iri_index {
            Some(index)
        } else {
            if self.index_strategy == IndexCreationStrategy::OnQuery {
                self.build_iri_index();
            }
            if let Some(index) = &self.iri_index {
                Some(index)
            } else {
                None
            }
        };

        if let Some(iri_index) = iri_index {
            let axioms = iri_index
                .component_for_iri(&iri)
                .filter_map(|a| {
                    if a.is_axiom() {
                        Some(model::AnnotatedComponent::from(a))
                    } else {
                        None
                    }
                })
                .collect();

            Ok(axioms)
        } else {
            Err(PyValueError::new_err("IRI index not yet build!"))
        }
    }

    /// Gets all components (axiom, swrl, and meta component) for an entity.
    #[pyo3[signature = (iri)]]
    pub fn get_components_for_iri(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<Vec<model::AnnotatedComponent>> {
        let iri: IRI<ArcStr> = into_iri!(self, py, iri);

        let iri_index = if let Some(index) = &self.iri_index {
            Some(index)
        } else {
            if self.index_strategy == IndexCreationStrategy::OnQuery {
                self.build_iri_index();
            }
            self.iri_index.as_ref()
        };

        if let Some(iri_index) = iri_index {
            let components = iri_index
                .component_for_iri(&iri)
                .map(model::AnnotatedComponent::from)
                .collect();

            Ok(components)
        } else {
            Err(PyValueError::new_err("IRI index not yet build!"))
        }
    }

    /// Returns all axioms of the ontology.
    pub fn get_axioms(&mut self) -> PyResult<Vec<model::AnnotatedComponent>> {
        let r = (&self.set_index)
            .into_iter()
            .filter_map(|a| {
                if a.is_axiom() {
                    Some(model::AnnotatedComponent::from(a.clone()))
                } else {
                    None
                }
            })
            .collect();

        Ok(r)
    }

    /// Returns all axioms of the ontology.
    pub fn get_components(&mut self) -> PyResult<Vec<model::AnnotatedComponent>> {
        let r = (&self.set_index)
            .into_iter()
            .map(model::AnnotatedComponent::from)
            .collect();

        Ok(r)
    }

    /// Gets all direct and indirect subclasses of a class.
    ///
    /// .. deprecated::
    ///     Use a reasoner instead. See :doc:`reasoner` for details.
    #[pyo3(signature = (parent_iri))]
    pub fn get_descendants(
        &self,
        py: Python<'_>,
        parent_iri: model::IRIParam,
    ) -> PyResult<HashSet<String>> {
        PyErr::warn(
            py,
            &py.get_type::<pyo3::exceptions::PyDeprecationWarning>(),
            c"get_descendants is deprecated, use a reasoner instead. See the 'reasoner' documentation.",
            1,
        )?;
        let parent_class: Class<ArcStr> = self.class_(py, parent_iri)?.into();
        let parent_class: ClassExpression<ArcStr> = parent_class.into();

        // Use structural reasoner for hierarchy traversal
        let reasoner = self.create_structural_reasoner();
        let descendants = reasoner
            .get_subclasses(&parent_class)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(descendants.into_iter().map(|c| c.to_string()).collect())
    }

    /// Gets all direct and indirect super classes of a class.
    ///
    /// .. deprecated::
    ///     Use a reasoner instead. See :doc:`reasoner` for details.
    #[pyo3(signature = (child_iri))]
    pub fn get_ancestors(
        &self,
        py: Python<'_>,
        child_iri: model::IRIParam,
    ) -> PyResult<HashSet<String>> {
        PyErr::warn(
            py,
            &py.get_type::<pyo3::exceptions::PyDeprecationWarning>(),
            c"get_ancestors is deprecated, use a reasoner instead. See the 'reasoner' documentation.",
            1,
        )?;
        let child: Class<ArcStr> = self.class_(py, child_iri)?.into();
        let child: ClassExpression<ArcStr> = child.into();

        // Use structural reasoner for hierarchy traversal
        let reasoner = self.create_structural_reasoner();
        let ancestors = reasoner
            .get_superclasses(&child)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;

        Ok(ancestors.into_iter().map(|c| c.to_string()).collect())
    }

    /// Checks exact membership by default. A bare axiom means no annotations.
    /// With ignore_annotations=True, compares only the axiom body.
    #[pyo3(signature = (axiom, *, ignore_annotations = false))]
    pub fn contains_axiom(
        &self,
        axiom: model::AnnotatedComponent,
        ignore_annotations: bool,
    ) -> bool {
        let axiom: AnnotatedComponent<ArcStr> = axiom.into();
        if ignore_annotations {
            self.iter().any(|a| a.component == axiom.component)
        } else {
            self.set_index.contains(&axiom)
        }
    }

    /// Checks whether an entity occurs in the ontology.
    #[pyo3(signature = (entity: "NamedEntity"))]
    pub fn contains_entity(&self, entity: &Bound<'_, PyAny>) -> PyResult<bool> {
        let (kind, iri) = named_entity(entity)?;
        let iri: IRI<ArcStr> = iri.into();
        let candidates: Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>> =
            if kind != model::EntityKind::Datatype {
                if let Some(index) = &self.iri_index {
                    Box::new(index.component_for_iri(&iri))
                } else {
                    Box::new(self.iter())
                }
            } else {
                // Simple and language literals imply datatypes whose IRIs are
                // not explicitly present in the IRI index.
                Box::new(self.iter())
            };
        let mut walk = Walk::new(SignatureCollector::looking_for(kind, iri));
        for component in candidates {
            walk.annotated_component(component);
            if walk.as_mut_visit().found {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Checks whether an entity has an explicit declaration axiom.
    #[pyo3(signature = (entity: "NamedEntity"))]
    pub fn is_declared(&self, entity: &Bound<'_, PyAny>) -> PyResult<bool> {
        let (kind, iri) = named_entity(entity)?;

        let iri: IRI<ArcStr> = iri.into();

        let mut candidates: Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>> =
            if let Some(index) = &self.component_index {
                Box::new(index.component_for_kind(kind.into()))
            } else {
                Box::new(self.iter())
            };

        macro_rules! match_axioms {
            ($matching:expr, $(($declare:ident, $entity:ident)),*) => (
                match $matching {
                    $((Component::$declare(decl), model::EntityKind::$entity) => decl.0.is(&iri),)*
                    _ => false
                }
            );
        }

        Ok(candidates.any(|component| {
            match_axioms!(
                (&component.component, kind),
                (DeclareClass, Class),
                (DeclareObjectProperty, ObjectProperty),
                (DeclareDataProperty, DataProperty),
                (DeclareAnnotationProperty, AnnotationProperty),
                (DeclareNamedIndividual, NamedIndividual),
                (DeclareDatatype, Datatype)
            )
        }))
    }
}

impl PyIndexedOntology {
    fn create_structural_reasoner(&self) -> StructuralReasoner {
        StructuralReasoner::create_reasoner(self.into())
    }

    pub fn get_subclasses(&mut self, iri: &IRI<ArcStr>) -> HashSet<ClassExpression<ArcStr>> {
        let subclassof_axioms: Box<dyn std::iter::Iterator<Item = &AnnotatedComponent<ArcStr>>> =
            if let Some(iri_index) = self.get_iri_index() {
                let components = iri_index.component_for_iri(iri);
                Box::new(components.filter(|a| a.kind() == ComponentKind::SubClassOf))
            } else if let Some(component_index) = self.get_component_index() {
                Box::new(component_index.component_for_kind(ComponentKind::SubClassOf))
            } else {
                Box::new(
                    (&self.set_index)
                        .into_iter()
                        .filter(|a| a.kind() == ComponentKind::SubClassOf),
                )
            };

        subclassof_axioms
            .filter_map(|a| match &a.component {
                Component::SubClassOf(SubClassOf {
                    sub,
                    sup: ClassExpression::Class(Class(sup)),
                }) if sup == iri => Some(sub.clone()),
                _ => None,
            })
            .collect()
    }

    pub fn get_superclasses(&mut self, iri: &IRI<ArcStr>) -> HashSet<ClassExpression<ArcStr>> {
        let subclassof_axioms: Box<dyn std::iter::Iterator<Item = &AnnotatedComponent<ArcStr>>> =
            if let Some(iri_index) = self.get_iri_index() {
                let components = iri_index.component_for_iri(iri);
                Box::new(components.filter(|a| a.kind() == ComponentKind::SubClassOf))
            } else if let Some(component_index) = self.get_component_index() {
                Box::new(component_index.component_for_kind(ComponentKind::SubClassOf))
            } else {
                Box::new(
                    (&self.set_index)
                        .into_iter()
                        .filter(|a| a.kind() == ComponentKind::SubClassOf),
                )
            };

        subclassof_axioms
            .filter_map(|a| match &a.component {
                Component::SubClassOf(SubClassOf {
                    sub: ClassExpression::Class(Class(sub)),
                    sup,
                }) if sub == iri => Some(sup.clone()),
                _ => None,
            })
            .collect()
    }
}
