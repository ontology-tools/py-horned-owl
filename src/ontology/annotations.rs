use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue, ArcStr, Component, IRI, Literal, MutableOntology,
};
use horned_owl::vocab::AnnotationBuiltIn;
use pyo3::prelude::*;
use std::collections::BTreeSet;

use crate::{model, PyIndexedOntology};

#[pymethods]
impl PyIndexedOntology {
    /// Replaces all literal rdfs:label assertions in one language scope.
    /// The default scope is untagged literals; other languages are preserved.
    /// Language matching is case-insensitive and exact. Existing axiom
    /// annotations in the replaced scope are removed with their assertions.
    #[pyo3(signature = (iri, label, *, language = None))]
    pub fn set_label(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
        label: String,
        language: Option<String>,
    ) -> PyResult<()> {
        let iri: IRI<ArcStr> = into_iri!(self, py, iri);
        let language = language.unwrap_or_default();
        let matches: Vec<_> = self
            .iter()
            .filter(|a| {
                matches!(&a.component, Component::AnnotationAssertion(ax)
                if ax.subject == AnnotationSubject::IRI(iri.clone())
                && ax.ann.ap.0.as_ref() == AnnotationBuiltIn::Label.underlying()
                && match &ax.ann.av {
                    AnnotationValue::Literal(Literal::Language { lang, .. }) =>
                        !language.is_empty() && lang.eq_ignore_ascii_case(&language),
                    AnnotationValue::Literal(Literal::Simple { .. } | Literal::Datatype { .. }) =>
                        language.is_empty(),
                    _ => false,
                })
            })
            .cloned()
            .collect();
        for old in matches {
            self.remove(&old);
        }
        let literal = if language.is_empty() {
            Literal::Simple { literal: label }
        } else {
            Literal::Language {
                literal: label,
                lang: language,
            }
        };
        let ap = self
            .build
            .read()
            .unwrap()
            .annotation_property(AnnotationBuiltIn::Label);
        self.insert(Component::AnnotationAssertion(AnnotationAssertion {
            subject: iri.into(),
            ann: Annotation {
                ap,
                av: AnnotationValue::Literal(literal),
                ann: BTreeSet::new(),
            },
        }));
        Ok(())
    }

    /// Gets the first annotated value for an entity and annotation property.
    ///
    /// Note: If there are multiple annotation axioms for the queried entity and annotation property,
    /// the order is neither necessarily the same as in the ontology neither is it stable.
    /// Get all annotation values with `Ontology.get_annotations`.
    #[pyo3(signature = (entity_iri, ann_iri))]
    pub fn get_annotation(
        &mut self,
        py: Python<'_>,
        entity_iri: model::IRIParam,
        ann_iri: model::IRIParam,
    ) -> PyResult<Option<String>> {
        self.get_annotations(py, entity_iri, ann_iri)
            .map(|x| x.first().map(Into::into))
    }

    /// Gets all annotated value for an entity and annotation property.
    ///
    /// Note: The order is neither necessarily the same as in the ontology neither is it stable.
    /// Get all annotation values with `Ontology.get_annotations`.
    #[pyo3(signature = (entity_iri, ann_iri))]
    pub fn get_annotations(
        &mut self,
        py: Python<'_>,
        entity_iri: model::IRIParam,
        ann_iri: model::IRIParam,
    ) -> PyResult<Vec<String>> {
        let entity_iri: IRI<ArcStr> = into_iri!(self, py, entity_iri);
        let ann_iri: IRI<ArcStr> = into_iri!(self, py, ann_iri);

        let components = if let Some(iri_index) = &self.iri_index {
            Box::new(iri_index.component_for_iri(&entity_iri))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter())
        };

        let literal_values = components
            .filter_map(|aax: &AnnotatedComponent<ArcStr>| {
                match &aax.component {
                    Component::AnnotationAssertion(AnnotationAssertion { subject: AnnotationSubject::IRI(s), ann }) if &entity_iri == s => {
                        match ann {
                            Annotation { ap, av: AnnotationValue::Literal(Literal::Simple { literal }), .. } => {
                                if ann_iri.eq(&ap.0) { Some(literal.clone()) } else { None }
                            }
                            //Language { literal: String, lang: String },
                            Annotation { ap, av: AnnotationValue::Literal(Literal::Language { literal, lang: _ }), .. } => {
                                if ann_iri.eq(&ap.0) { Some(literal.clone()) } else { None }
                            }
                            //Datatype { literal: String, datatype_iri: IRI },
                            Annotation { ap, av: AnnotationValue::Literal(Literal::Datatype { literal, datatype_iri: _ }), .. } => {
                                if ann_iri.eq(&ap.0) { Some(literal.clone()) } else { None }
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }).collect();

        Ok(literal_values)
    }
}
