use crate::{model, PyIndexedOntology};
use horned_owl::model::{AnnotatedComponent, ArcStr, MutableOntology};
use pyo3::prelude::*;

#[pymethods]
impl PyIndexedOntology {
    /// Adds a component. Returns whether it was new.
    ///
    /// With ignore_annotations=True, skips insertion if the body exists, regardless of its annotations.
    #[pyo3(signature = (component, *, ignore_annotations = false))]
    pub fn add_component(
        &mut self,
        component: model::AnnotatedComponent,
        ignore_annotations: bool,
    ) -> bool {
        let component: AnnotatedComponent<ArcStr> = component.into();
        if ignore_annotations && self.iter().any(|a| a.component == component.component) {
            return false;
        }
        self.insert(component)
    }

    /// Synonym for add_component.
    #[pyo3(signature = (ax, *, ignore_annotations = false))]
    pub fn add_axiom(&mut self, ax: model::AnnotatedComponent, ignore_annotations: bool) -> bool {
        self.add_component(ax, ignore_annotations)
    }

    /// synonym for add_components.
    #[pyo3(signature = (axioms, *, ignore_annotations = false))]
    pub fn add_axioms(
        &mut self,
        axioms: Vec<model::AnnotatedComponent>,
        ignore_annotations: bool,
    ) -> usize {
        self.add_components(axioms, ignore_annotations)
    }

    /// Adds components and returns the number newly inserted.
    ///
    /// With ignore_annotations=True, component which are already present
    /// are skipped.
    #[pyo3(signature = (components, *, ignore_annotations = false))]
    pub fn add_components(
        &mut self,
        components: Vec<model::AnnotatedComponent>,
        ignore_annotations: bool,
    ) -> usize {
        components
            .into_iter()
            .map(|a| usize::from(self.add_component(a, ignore_annotations)))
            .sum()
    }

    /// Removes a component from the ontology and returns whether it was present.
    ///
    /// With ignore_annotations=True, all components with the same unannotated body 
    /// are removed, regardless of the annotations
    #[pyo3(signature = (component, *, ignore_annotations = false))]
    pub fn remove_component(
        &mut self,
        component: model::AnnotatedComponent,
        ignore_annotations: bool,
    ) -> bool {
        self.remove_matching(component, ignore_annotations) > 0
    }

    /// Synonym for remove_component.
    #[pyo3(signature = (ax, *, ignore_annotations = false))]
    pub fn remove_axiom(
        &mut self,
        ax: model::AnnotatedComponent,
        ignore_annotations: bool,
    ) -> bool {
        self.remove_component(ax, ignore_annotations)
    }

    /// Removes components from the ontology and returns the number of components removed.
    ///
    /// With ignore_annotations=True, components are removed if their unannotated body matches.
    #[pyo3(signature = (components, *, ignore_annotations = false))]
    pub fn remove_components(
        &mut self,
        components: Vec<model::AnnotatedComponent>,
        ignore_annotations: bool,
    ) -> usize {
        components
            .into_iter()
            .map(|a| self.remove_matching(a, ignore_annotations))
            .sum()
    }

    /// Convenience method to add a Declare(Class(iri)) axiom.
    #[pyo3(signature = (iri))]
    pub fn declare_class(&mut self, py: Python<'_>, iri: model::IRIParam) -> PyResult<bool> {
        Ok(self.declare::<horned_owl::model::Class<ArcStr>>(self.class_(py, iri)?.into()))
    }

    /// Convenience method to add a Declare(ObjectProperty(iri)) axiom.
    #[pyo3(signature = (iri))]
    pub fn declare_object_property(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<bool> {
        Ok(self.declare::<horned_owl::model::ObjectProperty<ArcStr>>(
            self.object_property(py, iri)?.into(),
        ))
    }

    /// Convenience method to add a Declare(DataProperty(iri)) axiom.
    #[pyo3(signature = (iri))]
    pub fn declare_data_property(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<bool> {
        Ok(self.declare::<horned_owl::model::DataProperty<ArcStr>>(
            self.data_property(py, iri)?.into(),
        ))
    }

    /// Convenience method to add a Declare(annotationProperty(iri)) axiom.
    #[pyo3(signature = (iri))]
    pub fn declare_annotation_property(
        &mut self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<bool> {
        Ok(
            self.declare::<horned_owl::model::AnnotationProperty<ArcStr>>(
                self.annotation_property(py, iri)?.into(),
            ),
        )
    }

    /// Convenience method to add a Declare(NamedIndividual(iri)) axiom.
    #[pyo3(signature = (iri))]
    pub fn declare_individual(&mut self, py: Python<'_>, iri: model::IRIParam) -> PyResult<bool> {
        Ok(self.declare::<horned_owl::model::NamedIndividual<ArcStr>>(
            self.named_individual(py, iri)?.into(),
        ))
    }

}

impl PyIndexedOntology {
    pub(super) fn remove_matching(
        &mut self,
        axiom: model::AnnotatedComponent,
        ignore_annotations: bool,
    ) -> usize {
        let axiom: AnnotatedComponent<ArcStr> = axiom.into();
        if !ignore_annotations {
            return usize::from(self.remove(&axiom));
        }
        let matches: Vec<_> = self
            .iter()
            .filter(|a| a.component == axiom.component)
            .cloned()
            .collect();
        matches.iter().filter(|a| self.remove(a)).count()
    }
}
