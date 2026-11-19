use crate::{PyIndexedOntology, model, wrappers::BTreeSetWrap};
use horned_owl::model::{AnnotatedComponent, ArcStr, Component, MutableOntology};
use pyo3::{exceptions::PyValueError, prelude::*};

#[pymethods]
impl PyIndexedOntology {


    /// Adds an axiom to the ontology with optional annotations.
    #[pyo3(signature = (component, annotations = None))]
    pub fn add_component(
        &mut self,
        component: model::Component,
        annotations: Option<BTreeSetWrap<model::Annotation>>,
    ) -> PyResult<()> {
        let ann = annotations.unwrap_or_default();

        let annotated_component: AnnotatedComponent<ArcStr> = model::AnnotatedComponent {
            component,
            annotations: ann,
        }
        .into();
        self.insert(annotated_component);

        Ok(())
    }

    /// Synonym for `add_component`
    #[pyo3(signature = (ax, annotations = None))]
    pub fn add_axiom(
        &mut self,
        ax: model::Component,
        annotations: Option<BTreeSetWrap<model::Annotation>>,
    ) -> PyResult<()> {
        self.add_component(ax, annotations)
    }

    /// Removes a component from the ontology.
    pub fn remove_component(&mut self, component: model::Component) -> PyResult<bool> {
        let ax: Component<ArcStr> = component.into();
        let annotated = (&self.set_index)
            .into_iter()
            .find(|a| a.component == ax)
            .ok_or(PyValueError::new_err("args"))?
            .to_owned();
        Ok(self.remove(&annotated))
    }

    /// Synonym for `remove_component`
    pub fn remove_axiom(&mut self, ax: model::Component) -> PyResult<bool> {
        self.remove_component(ax)
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