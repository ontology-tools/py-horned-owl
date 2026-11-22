use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue,
    ArcStr, Build, Component, ComponentKind, Literal, MutableOntology, IRI,
};
use horned_owl::vocab::AnnotationBuiltIn;
use pyo3::prelude::*;
use std::collections::{BTreeSet, HashSet};

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

    /// Gets all annotated value for an entity and annotation property.
    ///
    /// Note: The order is neither necessarily the same as in the ontology neither is it stable.
    /// Get all annotation values with `Ontology.get_annotations`.
    #[pyo3(signature = (subject, property = None))]
    pub fn get_annotations(
        &mut self,
        py: Python<'_>,
        subject: model::AnnotationSubject,
        property: Option<model::IRIParam>,
    ) -> PyResult<Vec<model::AnnotationValue>> {
        let subject = subject.into();
        let property = property
            .map(|p| p.into_iri(&self.mapping.borrow(py).0, &self.build.read().unwrap()))
            .transpose()?;

        self.annotation_assertions(&subject, property.as_ref())
            .into_iter()
            .map(|a| Ok(a.ann.av.into()))
            .collect()
    }

    /// Adds an annotation assertion. String values are literals; use IRI or
    /// a named entity for an IRI value. Returns whether it was newly inserted.
    #[pyo3(signature = (entity, property, value, *, annotations = None))]
    pub fn add_annotation(
        &mut self,
        py: Python<'_>,
        entity: model::AnnotationSubject,
        property: model::IRIParam,
        value: model::AnnotationValue,
        annotations: Option<HashSet<model::Annotation>>,
    ) -> PyResult<bool> {
        let subject = entity.into();
        let property = into_iri!(self, py, property);
        Ok(self.insert(AnnotatedComponent {
            component: Component::AnnotationAssertion(AnnotationAssertion {
                subject,
                ann: Annotation {
                    ap: horned_owl::model::AnnotationProperty(property),
                    av: value.into(),
                    ann: BTreeSet::new(),
                },
            }),
            ann: annotations
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect(),
        }))
    }

    /// Removes all annotation assertions matching subject, property and value,
    /// regardless of assertion/axiom annotations and returns the number of removed axioms.
    pub fn remove_annotation(
        &mut self,
        py: Python<'_>,
        entity: model::AnnotationSubject,
        property: model::IRIParam,
        value: model::AnnotationValue,
    ) -> PyResult<usize> {
        let subject = entity.into();
        let property = into_iri!(self, py, property);
        let value: AnnotationValue<ArcStr> = value.into();

        let candidates: Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>> =
            if let Some(index) = &self.component_index {
                Box::new(index.component_for_kind(ComponentKind::AnnotationAssertion))
            } else {
                Box::new(self.iter())
            };

        let matches = candidates
            .filter(|a| {
                matches!(&a.component, Component::AnnotationAssertion(ax)
                if ax.subject == subject && ax.ann.ap.0 == property && ax.ann.av == value)
            })
            .cloned()
            .collect::<Vec<_>>();

        Ok(matches.iter().filter(|a| self.remove(a)).count())
    }

    /// Chooses the first matching preferred language, breaking ties lexically.
    /// An empty language selects untagged strings. No implicit fallback.
    #[pyo3(signature = (entity, languages = None, *, default = None))]
    pub fn get_label(
        &self,
        entity: model::AnnotationSubject,
        languages: Option<Vec<String>>,
        default: Option<String>,
    ) -> PyResult<Option<String>> {
        let subject = entity.into();
        let labels = self.labels(&subject);
        let languages = languages.unwrap_or_else(|| vec![String::new()]);
        for language in languages.iter() {
            if let Some((text, _)) = labels
                .iter()
                .find(|(_, lang)| lang.eq_ignore_ascii_case(language))
            {
                return Ok(Some(text.clone()));
            }
        }
        Ok(default)
    }
}

impl PyIndexedOntology {
    fn labels(&self, subject: &AnnotationSubject<ArcStr>) -> HashSet<(String, String)> {
        let label = Build::new_arc().iri(AnnotationBuiltIn::Label);
        self.annotation_assertions(subject, Some(&label))
            .into_iter()
            .filter_map(|ax| match &ax.ann.av {
                AnnotationValue::Literal(Literal::Simple { literal }) => {
                    Some((literal.clone(), String::new()))
                }
                AnnotationValue::Literal(Literal::Language { literal, lang }) => {
                    Some((literal.clone(), lang.to_ascii_lowercase()))
                }
                AnnotationValue::Literal(Literal::Datatype {
                    literal,
                    datatype_iri,
                }) if datatype_iri.as_ref() == "http://www.w3.org/2001/XMLSchema#string" => {
                    Some((literal.clone(), String::new()))
                }
                _ => None,
            })
            .collect()
    }

    fn annotation_assertions(
        &self,
        subject: &AnnotationSubject<ArcStr>,
        property: Option<&IRI<ArcStr>>,
    ) -> Vec<AnnotationAssertion<ArcStr>> {
        let candidates: Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>> =
            if let Some(index) = &self.component_index {
                Box::new(index.component_for_kind(ComponentKind::AnnotationAssertion))
            } else {
                Box::new(self.iter())
            };

        candidates
            .filter_map(|a| {
                let Component::AnnotationAssertion(ax) = &a.component else {
                    return None;
                };
                if &ax.subject == subject && property.map_or(true, |p| ax.ann.ap.is(p)) {
                    Some(ax.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
