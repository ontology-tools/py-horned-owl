use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use curie::Curie;
use horned_owl::io::rdf::reader::ConcreteRDFOntology;
use horned_owl::io::ResourceType;
use horned_owl::model::{
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationSubject, AnnotationValue,
    ArcAnnotatedComponent, ArcStr, Build, Class, ClassExpression, Component, ComponentKind, ForIRI,
    HigherKinded, Literal, MutableOntology, Ontology, OntologyID, SubClassOf, IRI,
};
use horned_owl::ontology::component_mapped::{
    ArcComponentMappedOntology, ComponentMappedIndex, ComponentMappedOntology,
};
use horned_owl::ontology::indexed::OntologyIndex;
use horned_owl::ontology::iri_mapped::IRIMappedIndex;
use horned_owl::ontology::set::{SetIndex, SetOntology};
use horned_owl::vocab::AnnotationBuiltIn;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{
    pyclass, pyfunction, pymethods, Bound, Py, PyAny, PyObject, PyResult, Python, ToPyObject,
};

use crate::prefix_mapping::PrefixMapping;
use crate::wrappers::BTreeSetWrap;
use crate::{guess_serialization, model, parse_serialization, to_py_err};

#[pyclass]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
/// Values to indicate when to build the additional indexes.
///
/// OnLoad: Create the additional indexes when the ontology is loaded
/// OnQuery: Create the additional indexes only when they are needed
/// Explicit: Only create the additional indexes when explicity requested
pub enum IndexCreationStrategy {
    /// Create the additional indexes when the ontology is loaded
    OnLoad,

    /// Create the additional indexes only when they are needed
    OnQuery,

    /// Only create the additional indexes when explicity requested
    Explicit,
}

impl Default for IndexCreationStrategy {
    fn default() -> Self {
        IndexCreationStrategy::OnQuery
    }
}

/// Represents a loaded ontology.
#[pyclass]
pub struct PyIndexedOntology {
    //State variables private to Rust, exposed through methods to Python
    pub labels_to_iris: HashMap<String, IRI<ArcStr>>,

    pub classes_to_subclasses: HashMap<IRI<ArcStr>, HashSet<IRI<ArcStr>>>,
    //axiom typed index would give subclass axioms
    pub classes_to_superclasses: HashMap<IRI<ArcStr>, HashSet<IRI<ArcStr>>>,

    //The primary store of the axioms is a Horned OWL indexed ontology
    pub iri_index: Option<IRIMappedIndex<ArcStr, ArcAnnotatedComponent>>,
    pub component_index: Option<ComponentMappedIndex<ArcStr, ArcAnnotatedComponent>>,
    pub set_index: SetIndex<ArcStr, ArcAnnotatedComponent>,
    //Need this for converting IRIs to IDs and for saving again afterwards
    pub mapping: Py<PrefixMapping>,
    pub build: Build<ArcStr>,

    pub index_strategy: IndexCreationStrategy,
}

impl Default for PyIndexedOntology {
    fn default() -> Self {
        Python::with_gil(|py| PyIndexedOntology {
            labels_to_iris: Default::default(),
            classes_to_subclasses: Default::default(),
            classes_to_superclasses: Default::default(),
            iri_index: None,
            component_index: None,
            set_index: Default::default(),
            mapping: Py::new(py, PrefixMapping::default())
                .expect("Unable to create default prefix mapping"),
            build: Build::new_arc(),
            index_strategy: IndexCreationStrategy::OnQuery,
        })
    }
}

impl<A: ForIRI> From<&PyIndexedOntology> for SetOntology<A>
where
    AnnotatedComponent<A>: From<AnnotatedComponent<ArcStr>>,
{
    fn from(ont: &PyIndexedOntology) -> Self {
        let mut o = SetOntology::<A>::new();
        for comp in &ont.set_index {
            o.insert::<AnnotatedComponent<A>>(comp.clone().into());
        }
        o
    }
}

impl Clone for PyIndexedOntology {
    fn clone(&self) -> Self {
        let set = SetOntology::from(self);
        Self::from_set_ontology(set, self.index_strategy)
    }
}

impl Ontology<ArcStr> for PyIndexedOntology {}
impl MutableOntology<ArcStr> for PyIndexedOntology {
    fn insert<AA>(&mut self, ax: AA) -> bool
    where
        AA: Into<AnnotatedComponent<ArcStr>>,
    {
        let ax: ArcAnnotatedComponent = ax.into().into();
        if let Some(ref mut iri_index) = &mut self.iri_index {
            iri_index.index_insert(ax.clone());
        }
        if let Some(ref mut component_index) = &mut self.component_index {
            component_index.index_insert(ax.clone());
        }
        self.set_index.index_insert(ax)
    }

    fn take(&mut self, ax: &AnnotatedComponent<ArcStr>) -> Option<AnnotatedComponent<ArcStr>> {
        if let Some(ref mut iri_index) = &mut self.iri_index {
            iri_index.index_take(ax);
        }
        if let Some(ref mut component_index) = &mut self.component_index {
            component_index.index_take(ax);
        }
        self.set_index.index_take(ax)
    }

    fn remove(&mut self, ax: &AnnotatedComponent<ArcStr>) -> bool {
        if let Some(ref mut iri_index) = &mut self.iri_index {
            iri_index.index_remove(ax);
        }
        if let Some(ref mut component_index) = &mut self.component_index {
            component_index.index_remove(ax);
        }
        self.set_index.index_remove(ax)
    }
}

#[pymethods]
impl PyIndexedOntology {
    #[new]
    #[pyo3(signature = (index_strategy = IndexCreationStrategy::OnQuery))]
    pub fn new(index_strategy: IndexCreationStrategy) -> Self {
        let mut s = Self::default();

        if index_strategy == IndexCreationStrategy::OnLoad {
            s.iri_index = Default::default();
            s.component_index = Default::default();
        }

        s.index_strategy = index_strategy;

        s
    }

    /// get_id_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Optional[str]
    ///
    /// Gets the ID of term by it IRI.
    ///
    /// If the term does not have an ID, `None` is returned.
    #[pyo3[signature = (iri, *, iri_is_absolute = None)]]
    pub fn get_id_for_iri(
        &mut self,
        py: Python<'_>,
        iri: String,
        iri_is_absolute: Option<bool>,
    ) -> PyResult<Option<String>> {
        let iri: String = self.iri(py, iri, iri_is_absolute)?.into();
        let mapping = self.mapping.borrow_mut(py);

        let res = mapping.0.shrink_iri(iri.as_str());

        if let Ok(curie) = res {
            Ok(Some(curie.to_string()))
        } else {
            //Return null
            Ok(None)
        }
    }

    /// get_iri_for_id(self, id: str) -> Optional[str]
    ///
    /// Gets the IRI of a term by its ID.
    ///
    /// If the term does not have an IRI, `None` is returned.
    pub fn get_iri_for_id(&mut self, py: Python<'_>, id: String) -> PyResult<PyObject> {
        let idparts: Vec<&str> = id.split(":").collect();

        if idparts.len() == 2 {
            let curie = Curie::new(Some(idparts[0]), idparts[1]);

            let mapping = self.mapping.borrow_mut(py);
            let res = mapping.0.expand_curie(&curie);

            if let Ok(iri) = res {
                Ok(iri.to_string().to_object(py))
            } else {
                //Return null
                Ok(().to_object(py))
            }
        } else {
            //Not a CURIE, at least not of the form PREFIX:NUMBER
            Ok(().to_object(py))
        }
    }

    /// prefix_mapping: PrefixMapping
    ///
    /// The prefix mapping
    #[getter]
    pub fn get_prefix_mapping<'py>(&self, py: Python<'py>) -> PyResult<&Bound<'py, PrefixMapping>> {
        let mapping = self.mapping.bind(py);
        Ok(mapping)
    }

    /// add_prefix_mapping(self, iriprefix: str, mappedid: str) -> None
    ///
    /// Adds the prefix `iriprefix`.
    pub fn add_prefix_mapping<'py>(
        &mut self,
        py: Python<'py>,
        iriprefix: String,
        mappedid: String,
    ) -> PyResult<()> {
        self.mapping.borrow_mut(py).add_prefix(iriprefix, mappedid)
    }

    /// set_label(self, iri: str, label: str, *, absolute: Optional[bool] = None) -> None
    ///
    /// Sets the label of a term by iri.
    ///
    /// Adds an or updates the `AnnotationAssertion` axiom for `rdfs:label`.
    #[pyo3[signature = (iri, label, *, absolute = None)]]
    pub fn set_label(
        &mut self,
        py: Python<'_>,
        iri: String,
        label: String,
        absolute: Option<bool>,
    ) -> PyResult<()> {
        let iri: IRI<ArcStr> = self.iri(py, iri, absolute)?.into();

        let ax1: AnnotatedComponent<ArcStr> = Component::AnnotationAssertion(AnnotationAssertion {
            subject: iri.clone().into(),
            ann: Annotation {
                ap: self.build.annotation_property(AnnotationBuiltIn::Label),
                av: AnnotationValue::Literal(Literal::Simple {
                    literal: label.clone(),
                }),
            },
        })
        .into();

        let components = if let Some(ref mut iri_index) = &mut self.iri_index {
            Box::new(iri_index.component_for_iri(&iri))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter())
        };

        //If we already have a label, update it:
        let old_ax = &components
            .filter_map(|aax: &AnnotatedComponent<ArcStr>| match &aax.component {
                Component::AnnotationAssertion(AnnotationAssertion {
                    subject: AnnotationSubject::IRI(subj),
                    ann,
                }) if subj == &iri => match ann {
                    Annotation {
                        ap,
                        av: AnnotationValue::Literal(Literal::Simple { literal: _old }),
                    } => {
                        if AnnotationBuiltIn::Label.to_string().eq(&ap.0.to_string()) {
                            Some(aax.clone())
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            })
            .next();

        if let Some(old_ax) = old_ax {
            self.take(old_ax);
        }

        self.insert(ax1);
        Ok(())
    }

    /// get_iri_for_label(self, label: str) -> Optional[str]
    ///
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
                        av: AnnotationValue::Literal(Literal::Simple { literal }),
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

    /// get_iri(self) -> Optional[str]
    ///
    /// Returns the ontology iri, if it exists.
    pub fn get_iri(&mut self) -> PyResult<Option<model::IRI>> {
        Ok(self
            .get_id()
            .and_then(|x| x.iri.as_ref())
            .map(model::IRI::from))
    }

    /// get_version_iri(self) -> Optional[str]
    ///
    /// Returns the ontologys version iri, if it exists.
    pub fn get_version_iri(&mut self) -> PyResult<Option<model::IRI>> {
        Ok(self
            .get_id()
            .and_then(|x| x.viri.as_ref())
            .map(model::IRI::from))
    }

    /// get_subclasses(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Set[str]
    ///
    /// Gets all subclasses of an entity.
    #[pyo3[name="get_subclasses", signature = (iri, iri_is_absolute = None)]]
    pub fn py_get_subclasses(
        &mut self,
        py: Python<'_>,
        iri: String,
        iri_is_absolute: Option<bool>,
    ) -> PyResult<HashSet<String>> {
        let iri: IRI<ArcStr> = self.iri(py, iri, iri_is_absolute)?.into();

        let classes: HashSet<String> = self
            .get_subclasses(&iri)
            .into_iter()
            .map(|i| i.to_string())
            .collect();

        Ok(classes)
    }

    /// get_superclasses(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Set[str]
    ///
    /// Gets all superclasses of an entity.
    #[pyo3[name="get_superclasses", signature = (iri, iri_is_absolute = None)]]
    pub fn py_get_superclasses(
        &mut self,
        py: Python<'_>,
        iri: String,
        iri_is_absolute: Option<bool>,
    ) -> PyResult<HashSet<String>> {
        let iri: IRI<ArcStr> = self.iri(py, iri, iri_is_absolute)?.into();

        let classes: HashSet<String> = self
            .get_superclasses(&iri)
            .into_iter()
            .map(|i| i.to_string())
            .collect();

        Ok(classes)
    }

    /// get_classes(self) -> Set[str]
    ///
    /// Returns the IRIs of all declared classes in the ontology.
    pub fn get_classes(&mut self) -> PyResult<HashSet<String>> {
        //Get the DeclareClass axioms
        let classes = if let Some(ref mut component_index) = &mut self.component_index {
            Box::new(component_index.component_for_kind(ComponentKind::DeclareClass))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter())
        };

        let classes = classes
            .filter_map(|aax| match &aax.component {
                Component::DeclareClass(dc) => Some(dc.0 .0.to_string()),
                _ => None,
            })
            .collect();

        Ok(classes)
    }

    /// get_object_properties(self) -> Set[str]
    ///
    /// Returns the IRIs of all declared object properties in the ontology.
    pub fn get_object_properties(&mut self) -> PyResult<HashSet<String>> {
        //Get the DeclareObjectProperty axioms
        let object_properties = if let Some(ref mut component_index) = &mut self.component_index {
            Box::new(component_index.component_for_kind(ComponentKind::DeclareObjectProperty))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter())
        };

        let object_properties: HashSet<String> = object_properties
            .filter_map(|aax| match aax.clone().component {
                Component::DeclareObjectProperty(dop) => Some(dop.0 .0.to_string()),
                _ => None,
            })
            .collect();
        Ok(object_properties)
    }

    /// get_annotation(self, class_iri: str, ann_iri: str, *, class_iri_is_absolute: Optional[bool] = None, ann_iri_is_absolute: Optional[bool]=None) -> Optional[str]
    ///
    /// Gets the first annotated value for an entity and annotation property.
    ///
    /// Note: If there are multiple annotation axioms for the queried entity and annotation property,
    /// the order is neither necessarily the same as in the ontology neither is it stable.
    /// Get all annotation values with `PyIndexedOntology.get_annotations`.
    #[pyo3[signature = (class_iri, ann_iri, *, class_iri_is_absolute = None, ann_iri_is_absolute = None)]]
    pub fn get_annotation(
        &mut self,
        py: Python<'_>,
        class_iri: String,
        ann_iri: String,
        class_iri_is_absolute: Option<bool>,
        ann_iri_is_absolute: Option<bool>,
    ) -> PyResult<Option<String>> {
        self.get_annotations(
            py,
            class_iri,
            ann_iri,
            class_iri_is_absolute,
            ann_iri_is_absolute,
        )
        .map(|x| x.first().map(Into::into))
    }

    /// get_annotations(self, class_iri: str, ann_iri: str, *, class_iri_is_absolute: Optional[bool] = None, ann_iri_is_absolute: Optional[bool]=None) -> List[str]
    ///
    /// Gets all annotated value for an entity and annotation property.
    ///
    /// Note: The order is neither necessarily the same as in the ontology neither is it stable.
    /// Get all annotation values with `PyIndexedOntology.get_annotations`.
    #[pyo3[signature = (class_iri, ann_iri, *, class_iri_is_absolute = None, ann_iri_is_absolute = None)]]
    pub fn get_annotations(
        &mut self,
        py: Python<'_>,
        class_iri: String,
        ann_iri: String,
        class_iri_is_absolute: Option<bool>,
        ann_iri_is_absolute: Option<bool>,
    ) -> PyResult<Vec<String>> {
        let class_iri: IRI<ArcStr> = self.iri(py, class_iri, class_iri_is_absolute)?.into();
        let ann_iri: IRI<ArcStr> = self.iri(py, ann_iri, ann_iri_is_absolute)?.into();

        let components = if let Some(iri_index) = &self.iri_index {
            Box::new(iri_index.component_for_iri(&class_iri))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter())
        };

        let literal_values = components
            .filter_map(|aax: &AnnotatedComponent<ArcStr>| {
                match &aax.component {
                    Component::AnnotationAssertion(AnnotationAssertion { subject: AnnotationSubject::IRI(s), ann }) if &class_iri == s => {
                        match ann {
                            Annotation { ap, av: AnnotationValue::Literal(Literal::Simple { literal }) } => {
                                if ann_iri.eq(&ap.0) { Some(literal.clone()) } else { None }
                            }
                            //Language { literal: String, lang: String },
                            Annotation { ap, av: AnnotationValue::Literal(Literal::Language { literal, lang: _ }) } => {
                                if ann_iri.eq(&ap.0) { Some(literal.clone()) } else { None }
                            }
                            //Datatype { literal: String, datatype_iri: IRI },
                            Annotation { ap, av: AnnotationValue::Literal(Literal::Datatype { literal, datatype_iri: _ }) } => {
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

    /// save_to_string(self, serialization: typing.Literal['owl', 'rdf','ofn', 'owx']) -> str
    ///
    /// Saves the ontology to a UTF8 string.
    pub fn save_to_string(&mut self, py: Python<'_>, serialization: &str) -> PyResult<String> {
        let serialization = parse_serialization(serialization)?;

        let mut writer = Vec::<u8>::new();

        self.save_to_buf(py, &mut writer, serialization)?;

        String::from_utf8(writer).map_err(to_py_err!("Failed to save ontology to UTF-8"))
    }

    /// save_to_file(self, file_name: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> None
    ///
    /// Saves the ontology to disk. If no serialization is given it is guessed by the file extension.
    /// Defaults to OWL/XML
    #[pyo3(signature = (file_name, serialization = None))]
    pub fn save_to_file(
        &mut self,
        py: Python<'_>,
        file_name: String,
        serialization: Option<&str>,
    ) -> PyResult<()> {
        let serialization = guess_serialization(&file_name, serialization)?;
        let mut file = File::create(file_name)?;

        self.save_to_buf(py, &mut file, serialization)
    }

    /// get_axioms_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> List[model.AnnotatedComponent]
    ///
    /// Gets all axioms for an entity.
    #[pyo3[signature = (iri, *, iri_is_absolute = None)]]
    pub fn get_axioms_for_iri(
        &mut self,
        py: Python<'_>,
        iri: String,
        iri_is_absolute: Option<bool>,
    ) -> PyResult<Vec<model::AnnotatedComponent>> {
        let iri: IRI<ArcStr> = self.iri(py, iri, iri_is_absolute)?.into();

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
            return Err(PyValueError::new_err("IRI index not yet build!"));
        }
    }

    /// get_components_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> List[model.AnnotatedComponent]
    ///
    /// Gets all components (axiom, swrl, and meta component) for an entity.
    #[pyo3[signature = (iri, *, iri_is_absolute = None)]]
    pub fn get_components_for_iri(
        &mut self,
        py: Python<'_>,
        iri: String,
        iri_is_absolute: Option<bool>,
    ) -> PyResult<Vec<model::AnnotatedComponent>> {
        let iri: IRI<ArcStr> = self.iri(py, iri, iri_is_absolute)?.into();

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
            let components = iri_index
                .component_for_iri(&iri)
                .map(model::AnnotatedComponent::from)
                .collect();

            Ok(components)
        } else {
            Err(PyValueError::new_err("IRI index not yet build!"))
        }
    }

    /// get_axioms(self) -> List[model.AnnotatedComponent]
    ///
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

    /// get_components(self) -> List[model.AnnotatedComponent]
    ///
    /// Returns all axioms of the ontology.
    pub fn get_components(&mut self) -> PyResult<Vec<model::AnnotatedComponent>> {
        let r = (&self.set_index)
            .into_iter()
            .map(model::AnnotatedComponent::from)
            .collect();

        Ok(r)
    }

    /// add_component(self, component: model.Component, annotations: Optional[Union[List[model.Annotation]|Set[model.Annotation]]]=None) -> None
    ///
    /// Adds an axiom to the ontology with optional annotations.
    #[pyo3(signature = (component, annotations = None))]
    pub fn add_component(
        &mut self,
        component: model::Component,
        annotations: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<()> {
        let ann: BTreeSetWrap<model::Annotation> = match annotations {
            Some(a) => a
                .extract::<BTreeSet<model::Annotation>>()
                .or_else(|_| {
                    Ok::<BTreeSet<model::Annotation>, PyErr>(
                        a.extract::<Vec<model::Annotation>>()?.into_iter().collect(),
                    )
                })?
                .into(),
            None => BTreeSet::new(),
        }
        .into();

        let annotated_component: AnnotatedComponent<ArcStr> =
            model::AnnotatedComponent { component, ann }.into();
        self.insert(annotated_component);

        Ok(())
    }

    /// add_axiom(self, ax: model.Component, annotations: Optional[Union[List[model.Annotation]|Set[model.Annotation]]]=None) -> None
    ///
    /// Synonym for `add_component`
    #[pyo3(signature = (ax, annotations = None))]
    pub fn add_axiom(
        &mut self,
        ax: model::Component,
        annotations: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<()> {
        self.add_component(ax, annotations)
    }

    /// remove_component(self, component: model.Component) -> bool
    ///
    /// Removes a component from the ontology.
    pub fn remove_component(&mut self, component: model::Component) -> PyResult<bool> {
        let ax: Component<Arc<str>> = component.into();
        let annotated = (&self.set_index)
            .into_iter()
            .find(|a| a.component == ax)
            .ok_or(PyValueError::new_err("args"))?
            .to_owned();
        Ok(self.remove(&annotated))
    }

    /// remove_axiom(self, ax: model.Component) ->  bool
    ///
    /// Synonym for `remove_component`
    pub fn remove_axiom(&mut self, ax: model::Component) -> PyResult<bool> {
        self.remove_component(ax)
    }

    /// iri(self, iri: str, *, absolute: Optional[bool] = True) -> model.IRI
    ///
    /// Creates a new IRI from string.
    ///
    /// Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
    /// If `absolute` is None it is guessed by the occurrence of `"://"` in the IRI whether the iri
    /// is absolute or not.
    #[pyo3[signature = (iri, *, absolute = true)]]
    pub fn iri(&self, py: Python<'_>, iri: String, absolute: Option<bool>) -> PyResult<model::IRI> {
        let absolute = absolute.unwrap_or_else(|| iri.contains("://"));
        let r = if absolute {
            model::IRI::new(iri, &self.build)
        } else {
            self.curie(py, iri)?
        };
        Ok(r)
    }

    /// curie(self, iri: str) -> model.IRI
    ///
    /// Creates a new IRI from CURIE string.
    ///
    /// Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
    pub fn curie(&self, py: Python<'_>, curie: String) -> PyResult<model::IRI> {
        let mapping = self.mapping.borrow_mut(py);
        let iri = mapping
            .0
            .expand_curie_string(&curie)
            .map_err(to_py_err!("Invalid curie"))?;
        Ok(model::IRI::new(iri, &self.build))
    }

    /// clazz(self, iri: str, *, absolute: Optional[bool]=None) -> model.Class
    ///
    /// Convenience method to create a Class from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn clazz(
        &self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<model::Class> {
        Ok(model::Class(self.iri(py, iri, absolute)?))
    }

    /// declare_class(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(Class(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_class(
        &mut self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<bool> {
        Ok(self.declare::<horned_owl::model::Class<ArcStr>>(self.clazz(py, iri, absolute)?.into()))
    }

    /// object_property(self, iri: str, *, absolute: Optional[bool]=None) -> model.ObjectProperty
    ///
    /// Convenience method to create an ObjectProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn object_property(
        &self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<model::ObjectProperty> {
        Ok(model::ObjectProperty(self.iri(py, iri, absolute)?))
    }

    /// declare_object_property(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(ObjectProperty(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_object_property(
        &mut self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<bool> {
        Ok(self.declare::<horned_owl::model::ObjectProperty<ArcStr>>(
            self.object_property(py, iri, absolute)?.into(),
        ))
    }

    /// data_property(self, iri: str, *, absolute: Optional[bool]=None) -> model.DataProperty
    ///
    /// Convenience method to create a DataProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn data_property(
        &self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<model::DataProperty> {
        Ok(model::DataProperty(self.iri(py, iri, absolute)?))
    }

    /// declare_data_property(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(DataProperty(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_data_property(
        &mut self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<bool> {
        Ok(self.declare::<horned_owl::model::DataProperty<ArcStr>>(
            self.data_property(py, iri, absolute)?.into(),
        ))
    }

    /// annotation_property(self, iri: str, *, absolute: Optional[bool]=None) -> model.AnnotationProperty
    ///
    /// Convenience method to create an annotationProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn annotation_property(
        &self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<model::AnnotationProperty> {
        Ok(model::AnnotationProperty(self.iri(py, iri, absolute)?))
    }

    /// declare_annotation_property(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(annotationProperty(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_annotation_property(
        &mut self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<bool> {
        Ok(
            self.declare::<horned_owl::model::AnnotationProperty<ArcStr>>(
                self.annotation_property(py, iri, absolute)?.into(),
            ),
        )
    }

    /// named_individual(self, iri: str, *, absolute: Optional[bool]=None) -> model.NamedIndividual
    ///
    /// Convenience method to create a NamedIndividual from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn named_individual(
        &self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<model::NamedIndividual> {
        Ok(model::NamedIndividual(self.iri(py, iri, absolute)?))
    }

    /// declare_individual(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(NamedIndividual(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_individual(
        &mut self,
        py: Python<'_>,
        iri: String,
        absolute: Option<bool>,
    ) -> PyResult<bool> {
        Ok(self.declare::<horned_owl::model::NamedIndividual<ArcStr>>(
            self.named_individual(py, iri, absolute)?.into(),
        ))
    }

    /// anonymous_individual(self, iri: str) -> model.AnonymousIndividual
    ///
    /// Convenience method to create an AnonymousIndividual from a string.
    pub fn anonymous_individual(&self, name: String) -> model::AnonymousIndividual {
        model::AnonymousIndividual(name.into())
    }

    /// get_descendants(self, parent: str, iri_is_absolute: Optional[bool] = None) -> Set[str]
    ///
    /// Gets all direct and indirect subclasses of a class.
    #[pyo3[signature = (parent_iri, *, iri_is_absolute = None)]]
    pub fn get_descendants(
        &self,
        py: Python<'_>,
        parent_iri: String,
        iri_is_absolute: Option<bool>,
    ) -> PyResult<HashSet<String>> {
        let mut descendants = HashSet::new();
        let parent_iri: IRI<ArcStr> = self.iri(py, parent_iri, iri_is_absolute)?.into();

        self.recurse_descendants(&parent_iri, &mut descendants);

        Ok(descendants.into_iter().map(|c| c.to_string()).collect())
    }

    /// get_ancestors(self, child: str, iri_is_absolute: Optional[bool] = None) -> Set[str]
    ///
    /// Gets all direct and indirect super classes of a class.
    #[pyo3[signature = (child_iri, *, iri_is_absolute = None)]]
    pub fn get_ancestors(
        &self,
        py: Python<'_>,
        child_iri: String,
        iri_is_absolute: Option<bool>,
    ) -> PyResult<HashSet<String>> {
        let mut ancestors = HashSet::new();

        let child_iri: IRI<ArcStr> = self.iri(py, child_iri, iri_is_absolute)?.into();

        self.recurse_ancestors(&child_iri, &mut ancestors);

        Ok(ancestors.into_iter().map(|c| c.to_string()).collect())
    }

    /// build_iri_index(self) -> None
    ///
    /// Builds an index by iri (IRIMappedIndex).
    pub fn build_iri_index(&mut self) -> () {
        if self.iri_index.is_some() {
            return;
        }

        let mut iri_index = IRIMappedIndex::<ArcStr, ArcAnnotatedComponent>::new();

        for c in self.set_index.iter() {
            iri_index.index_insert(c.clone());
        }

        self.iri_index = Some(iri_index);
    }

    /// component_index(self) -> None
    ///
    /// Builds an index by component kind (ComponentMappedIndex).
    pub fn build_component_index(&mut self) {
        if self.component_index.is_some() {
            return;
        }

        let mut component_index = ComponentMappedIndex::<ArcStr, ArcAnnotatedComponent>::new();

        for c in self.set_index.iter() {
            component_index.index_insert(c.clone());
        }

        self.component_index = Some(component_index);
    }

    /// build_indexes(self) -> None
    ///
    /// Builds indexes to allow (a quicker) access to axioms and entities.
    pub fn build_indexes(&mut self) {
        match (&self.iri_index, &self.component_index) {
            (Some(_), Some(_)) => return,
            (Some(_), None) => return self.build_component_index(),
            (None, Some(_)) => return self.build_iri_index(),
            _ => {}
        }

        let mut component_index = ComponentMappedIndex::<ArcStr, ArcAnnotatedComponent>::new();
        let mut iri_index = IRIMappedIndex::<ArcStr, ArcAnnotatedComponent>::new();

        for c in self.set_index.iter() {
            component_index.index_insert(c.clone());
            iri_index.index_insert(c.clone());
        }

        self.component_index = Some(component_index);
        self.iri_index = Some(iri_index);
    }
}

impl PyIndexedOntology {
    pub fn get_subclasses(&self, iri: &IRI<ArcStr>) -> HashSet<IRI<ArcStr>> {
        let subclass_axioms = if let Some(ref component_index) = &self.component_index {
            Box::new(component_index.component_for_kind(ComponentKind::SubClassOf))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter())
        };

        subclass_axioms
            .filter_map(|aax| match &aax.component {
                Component::SubClassOf(SubClassOf {
                    sub: ClassExpression::Class(Class(sub)),
                    sup: ClassExpression::Class(Class(sup)),
                }) if sup == iri => Some(sub.clone()),
                _ => None,
            })
            .collect()
    }

    pub fn get_superclasses(&self, iri: &IRI<ArcStr>) -> HashSet<IRI<ArcStr>> {
        let subclass_axioms = if let Some(ref component_index) = &self.component_index {
            Box::new(component_index.component_for_kind(ComponentKind::SubClassOf))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter())
        };

        subclass_axioms
            .filter_map(|aax| match &aax.component {
                Component::SubClassOf(SubClassOf {
                    sub: ClassExpression::Class(Class(sub)),
                    sup: ClassExpression::Class(Class(sup)),
                }) if sub == iri => Some(sup.clone()),
                _ => None,
            })
            .collect()
    }

    fn recurse_descendants(
        &self,
        superclass: &IRI<ArcStr>,
        descendants: &mut HashSet<IRI<ArcStr>>,
    ) {
        let subclasses = self.get_subclasses(superclass);

        for cls in subclasses.into_iter() {
            self.recurse_descendants(&cls, descendants);
            descendants.insert(cls);
        }
    }

    fn recurse_ancestors(&self, subclass: &IRI<ArcStr>, ancestors: &mut HashSet<IRI<ArcStr>>) {
        let superclasses = self.get_superclasses(subclass);

        for cls in superclasses.into_iter() {
            self.recurse_ancestors(&cls, ancestors);
            ancestors.insert(cls);
        }
    }

    fn get_id(&mut self) -> Option<&OntologyID<ArcStr>> {
        let components = if let Some(component_index) = &self.component_index {
            Box::new(component_index.component_for_kind(ComponentKind::OntologyID))
                as Box<dyn Iterator<Item = &AnnotatedComponent<ArcStr>>>
        } else {
            Box::new((&self.set_index).into_iter())
        };

        components
            .filter_map(|c| match c {
                horned_owl::model::AnnotatedComponent::<ArcStr> {
                    component: Component::OntologyID(id),
                    ann: _,
                } => Some(id),
                _ => None,
            })
            .next()
    }

    pub fn from_set_ontology(
        value: SetOntology<ArcStr>,
        index_strategy: IndexCreationStrategy,
    ) -> Self {
        let mut pio = Self::new(index_strategy);

        for cmp in value {
            pio.insert(cmp);
        }

        pio
    }

    pub fn from_rdf_ontology(
        value: ConcreteRDFOntology<ArcStr, ArcAnnotatedComponent>,
        index_strategy: IndexCreationStrategy,
    ) -> Self {
        let mut pio = Self::new(index_strategy);
        let (set_index, _, _) = value.index();

        pio.set_index = set_index;

        pio
    }

    fn save_to_buf<W: Write>(
        &mut self,
        py: Python<'_>,
        w: &mut W,
        serialization: ResourceType,
    ) -> PyResult<()> {
        let mut file = w;
        let mut amo: ArcComponentMappedOntology = ComponentMappedOntology::new_arc();

        //Copy the components into an ComponentMappedOntology as that is what horned owl writes
        for component in (&self.set_index).into_iter() {
            amo.insert(component.clone());
        }

        let mapping = self.mapping.borrow(py);

        let result = match serialization {
            ResourceType::OFN => {
                horned_owl::io::ofn::writer::write(&mut file, &amo, Some(&mapping.0))
            }
            ResourceType::OWX => {
                horned_owl::io::owx::writer::write(&mut file, &amo, Some(&mapping.0))
            }
            ResourceType::RDF => horned_owl::io::rdf::writer::write(&mut file, &amo),
        };

        result.map_err(to_py_err!("Problem saving the ontology to a file"))
    }
}

impl From<PyIndexedOntology> for SetOntology<ArcStr> {
    fn from(value: PyIndexedOntology) -> Self {
        let mut o = SetOntology::<ArcStr>::new();
        for cmp in value.set_index.into_iter() {
            o.insert(cmp);
        }
        o
    }
}

/// @deprecated("please use `PyIndexedOntology.get_descendants` instead")
/// get_descendants(onto: PyIndexedOntology, parent: str) -> Set[str]
///
/// Gets all direct and indirect subclasses of a class.
#[pyfunction]
pub fn get_descendants(
    py: Python<'_>,
    onto: &PyIndexedOntology,
    parent: String,
) -> PyResult<HashSet<String>> {
    onto.get_descendants(py, parent, Some(true))
}

/// @deprecated("please use `PyIndexedOntology.get_ancestors` instead")
/// get_ancestors(onto: PyIndexedOntology, child: str) -> Set[str]
///
/// Gets all direct and indirect super classes of a class.
#[pyfunction]
pub fn get_ancestors(
    py: Python<'_>,
    onto: &PyIndexedOntology,
    child: String,
) -> PyResult<HashSet<String>> {
    onto.get_ancestors(py, child, Some(true))
}
