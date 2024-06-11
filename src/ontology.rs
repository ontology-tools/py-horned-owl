use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs::File;
use std::ops::Deref;
use std::sync::Arc;

use curie::{Curie, PrefixMapping};
use horned_owl::io::ResourceType;
use horned_owl::model::{AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationValue, ArcStr, Build, ClassExpression, Component, ComponentKind, HigherKinded, IRI, Kinded, Literal, MutableOntology, OntologyID, SubClassOf};
use horned_owl::ontology::component_mapped::{ArcComponentMappedOntology, ComponentMappedOntology};
use horned_owl::ontology::iri_mapped::{ArcIRIMappedOntology, IRIMappedOntology};
use horned_owl::vocab::AnnotationBuiltIn;
use pyo3::{pyclass, pyfunction, pymethods, PyObject, PyResult, Python, ToPyObject};
use pyo3::exceptions::PyValueError;

use crate::{guess_serialization, model, to_py_err};
use crate::model::AnonymousIndividual;

/// Represents a loaded ontology.
#[pyclass]
pub struct PyIndexedOntology {
    //State variables private to Rust, exposed through methods to Python
    pub labels_to_iris: HashMap<String, IRI<ArcStr>>,

    pub classes_to_subclasses: HashMap<IRI<ArcStr>, HashSet<IRI<ArcStr>>>,
    //axiom typed index would give subclass axioms
    pub classes_to_superclasses: HashMap<IRI<ArcStr>, HashSet<IRI<ArcStr>>>,

    //The primary store of the axioms is a Horned OWL indexed ontology
    pub ontology: ArcIRIMappedOntology,
    //Need this for converting IRIs to IDs and for saving again afterwards
    pub mapping: PrefixMapping,
    pub build: Build<ArcStr>,
}

impl Default for PyIndexedOntology {
    fn default() -> Self {
        PyIndexedOntology {
            labels_to_iris: Default::default(),
            classes_to_subclasses: Default::default(),
            classes_to_superclasses: Default::default(),
            ontology: ArcIRIMappedOntology::new_arc(),
            mapping: Default::default(),
            build: Build::new_arc(),
        }
    }
}

#[pymethods]
impl PyIndexedOntology {
    #[new]
    pub fn new() -> Self {
        Default::default()
    }

    /// add_default_prefix_names(self) -> None
    ///
    /// Adds the prefix for rdf, rdfs, xsd, and owl
    pub fn add_default_prefix_names(&mut self) -> PyResult<()> {
        self.mapping.add_prefix("rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#")
            .map_err(to_py_err!("Error while adding predefined prefix 'rdf'"))?;
        self.mapping.add_prefix("rdfs", "http://www.w3.org/2000/01/rdf-schema#")
            .map_err(to_py_err!("Error while adding predefined prefix 'rdfs'"))?;
        self.mapping.add_prefix("xsd", "http://www.w3.org/2001/XMLSchema#")
            .map_err(to_py_err!("Error while adding predefined prefix 'xsd'"))?;
        self.mapping.add_prefix("owl", "http://www.w3.org/2002/07/owl#")
            .map_err(to_py_err!("Error while adding predefined prefix 'owl'"))?;

        Ok(())
    }

    /// get_id_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Optional[str]
    ///
    /// Gets the ID of term by it IRI.
    ///
    /// If the term does not have an ID, `None` is returned.
    #[pyo3[signature = (iri, *, iri_is_absolute = None)]]
    pub fn get_id_for_iri(&mut self, iri: String, iri_is_absolute: Option<bool>) -> PyResult<Option<String>> {
        let iri: String = self.iri(iri, iri_is_absolute)?.into();
        let res = self.mapping.shrink_iri(iri.as_str());

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
    pub fn get_iri_for_id(&mut self, py: Python, id: String) -> PyResult<PyObject> {
        let idparts: Vec<&str> = id.split(":").collect();

        if idparts.len() == 2 {
            let curie = Curie::new(Some(idparts[0]), idparts[1]);

            let res = self.mapping.expand_curie(&curie);

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

    /// add_prefix_mapping(self, iriprefix: str, mappedid: str) -> None
    ///
    /// Adds the prefix `iriprefix`.
    pub fn add_prefix_mapping(&mut self, iriprefix: String, mappedid: String) -> PyResult<()> {
        let result = self.mapping.add_prefix(&iriprefix, &mappedid);
        let _ = result.map_err(to_py_err!("Error - prefix is invalid."))?;

        if iriprefix.is_empty() {
            self.mapping.set_default(mappedid.as_str());
        }

        Ok(())
    }

    /// set_label(self, iri: str, label: str, *, absolute: Optional[bool] = None) -> None
    ///
    /// Sets the label of a term by iri.
    ///
    /// Adds an or updates the `AnnotationAssertion` axiom for `rdfs:label`.
    #[pyo3[signature = (iri, label, *, absolute = None)]]
    pub fn set_label(&mut self, iri: String, label: String, absolute: Option<bool>) -> PyResult<()> {
        let iri: IRI<ArcStr> = self.iri(iri, absolute)?.into();

        let ax1: AnnotatedComponent<ArcStr> = Component::AnnotationAssertion(AnnotationAssertion {
            subject: iri.clone().into(),
            ann: Annotation {
                ap: self
                    .build
                    .annotation_property(AnnotationBuiltIn::Label),
                av: AnnotationValue::Literal(Literal::Simple {
                    literal: label.clone(),
                }),
            },
        })
            .into();

        //If we already have a label, update it:
        let old_ax = &self
            .ontology
            .components_for_iri(&iri)
            .filter_map(|aax: &AnnotatedComponent<ArcStr>| match &aax.component {
                Component::AnnotationAssertion(AnnotationAssertion {
                                                   subject: _subj,
                                                   ann,
                                               }) => match ann {
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
            self.ontology.update_axiom(old_ax, ax1);
        } else {
            //If no label already, just add one
            self.ontology.insert(ax1);
        }
        Ok(())
    }

    /// get_iri_for_label(self, label: str) -> Optional[str]
    ///
    /// Returns the IRI of a term by its label if it exists.
    ///
    /// If the term does not have a label, `None` is returned.
    pub fn get_iri_for_label(&mut self, label: String) -> PyResult<Option<String>> {
        Ok(self.labels_to_iris.get(&label).map(String::from))
    }

    /// get_iri(self) -> Optional[str]
    ///
    /// Returns the ontology iri, if it exists.
    pub fn get_iri(&mut self) -> PyResult<Option<model::IRI>> {
        Ok(self.get_id().and_then(|x| x.iri.as_ref()).map(model::IRI::from))
    }

    /// get_version_iri(self) -> Optional[str]
    ///
    /// Returns the ontologys version iri, if it exists.
    pub fn get_version_iri(&mut self) -> PyResult<Option<model::IRI>> {
        Ok(self.get_id().and_then(|x| x.viri.as_ref()).map(model::IRI::from))
    }

    /// get_subclasses(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Set[str]
    ///
    /// Gets all subclasses of an entity.
    #[pyo3[signature = (iri, iri_is_absolute = None)]]
    pub fn get_subclasses(&mut self, iri: String, iri_is_absolute: Option<bool>) -> PyResult<HashSet<String>> {
        let iri: IRI<ArcStr> = self.iri(iri, iri_is_absolute)?.into();

        let subclasses = self.classes_to_subclasses.get(&iri);
        if let Some(subclss) = subclasses {
            let subclasses: HashSet<String> = subclss.iter().map(|sc| sc.to_string()).collect();
            Ok(subclasses)
        } else {
            Ok(HashSet::new())
        }
    }

    /// get_superclasses(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Set[str]
    ///
    /// Gets all superclasses of an entity.
    #[pyo3[signature = (iri, iri_is_absolute = None)]]
    pub fn get_superclasses(&mut self, iri: String, iri_is_absolute: Option<bool>) -> PyResult<HashSet<String>> {
        let iri: IRI<ArcStr> = self.iri(iri, iri_is_absolute)?.into();

        let superclasses = self.classes_to_superclasses.get(&iri);
        if let Some(superclss) = superclasses {
            let superclasses: HashSet<String> = superclss.iter().map(|sc| sc.to_string()).collect();
            Ok(superclasses)
        } else {
            Ok(HashSet::new())
        }
    }

    /// get_classes(self) -> Set[str]
    ///
    /// Returns the IRIs of all declared classes in the ontology.
    pub fn get_classes(&mut self) -> PyResult<HashSet<String>> {
        //Get the DeclareClass axioms
        let classes = self.ontology.component_for_kind(ComponentKind::DeclareClass);

        let classes: HashSet<String> = classes
            .filter_map(|aax| match aax.clone().component {
                Component::DeclareClass(dc) => Some(dc.0.0.to_string()),
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
        let object_properties = self
            .ontology
            .component_for_kind(ComponentKind::DeclareObjectProperty);

        let object_properties: HashSet<String> = object_properties
            .filter_map(|aax| match aax.clone().component {
                Component::DeclareObjectProperty(dop) => Some(dop.0.0.to_string()),
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
        class_iri: String,
        ann_iri: String,
        class_iri_is_absolute: Option<bool>,
        ann_iri_is_absolute: Option<bool>,
    ) -> PyResult<Option<String>> {
        self.get_annotations(class_iri, ann_iri, class_iri_is_absolute, ann_iri_is_absolute)
            .map(|x| x.first().map(Into::into))
    }

    /// get_annotations(self, class_iri: str, ann_iri: str, *, class_iri_is_absolute: Optional[bool] = None, ann_iri_is_absolute: Optional[bool]=None) -> List[str]
    ///
    /// Gets all annotated value for an entity and annotation property.
    ///
    /// Note: The order is neither necessarily the same as in the ontology neither is it stable.
    /// Get all annotation values with `PyIndexedOntology.get_annotations`.
    #[pyo3[signature = (class_iri, ann_iri, *, class_iri_is_absolute = None, ann_iri_is_absolute = None)]]
    pub fn get_annotations(&mut self, class_iri: String, ann_iri: String, class_iri_is_absolute: Option<bool>, ann_iri_is_absolute: Option<bool>) -> PyResult<Vec<String>> {
        let class_iri: IRI<ArcStr> = self.iri(class_iri, class_iri_is_absolute)?.into();
        let ann_iri: IRI<ArcStr> = self.iri(ann_iri, ann_iri_is_absolute)?.into();
        let literal_values: Vec<String> = self.ontology.components_for_iri(&class_iri)
            .filter_map(|aax: &AnnotatedComponent<ArcStr>| {
                match &aax.component {
                    Component::AnnotationAssertion(AnnotationAssertion { subject: _, ann }) => {
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

    /// save_to_file(self, file_name: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> None
    ///
    /// Saves the ontology to disk. If no serialization is given it is guessed by the file extension.
    /// Defaults to OWL/XML
    #[pyo3(signature = (file_name, serialization = None))]
    pub fn save_to_file(&mut self, file_name: String, serialization: Option<&str>) -> PyResult<()> {
        let serialization = guess_serialization(&file_name, serialization)?;

        let mut file = File::create(file_name)?;
        let mut amo: ArcComponentMappedOntology = ComponentMappedOntology::new_arc();

        //Copy the components into an ComponentMappedOntology as that is what horned owl writes
        for component in self.ontology.iter() {
            amo.insert(component.clone());
        }

        let result = match serialization {
            ResourceType::OFN => horned_owl::io::ofn::writer::write(&mut file, &amo, Some(&self.mapping)),
            ResourceType::OWX => horned_owl::io::owx::writer::write(&mut file, &amo, Some(&self.mapping)),
            ResourceType::RDF => horned_owl::io::rdf::writer::write(&mut file, &amo)
        };

        result.map_err(to_py_err!("Problem saving the ontology to a file"))
    }

    /// get_axioms_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> List[model.AnnotatedComponent]
    ///
    /// Gets all axioms for an entity.
    #[pyo3[signature = (iri, *, iri_is_absolute = None)]]
    pub fn get_axioms_for_iri(&mut self, iri: String, iri_is_absolute: Option<bool>) -> PyResult<Vec<model::AnnotatedComponent>> {
        let iri: IRI<ArcStr> = self.iri(iri, iri_is_absolute)?.into();

        let axioms = self
            .ontology
            .components_for_iri(&iri)
            .filter_map(|a|
                if a.is_axiom() {
                    Some(model::AnnotatedComponent::from(a))
                } else { None })
            .collect();

        Ok(axioms)
    }

    /// get_components_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> List[model.AnnotatedComponent]
    ///
    /// Gets all components (axiom, swrl, and meta component) for an entity.
    #[pyo3[signature = (iri, *, iri_is_absolute = None)]]
    pub fn get_components_for_iri(&mut self, iri: String, iri_is_absolute: Option<bool>) -> PyResult<Vec<model::AnnotatedComponent>> {
        let iri: IRI<ArcStr> = self.iri(iri, iri_is_absolute)?.into();

        let components = self
            .ontology
            .components_for_iri(&iri)
            .map(model::AnnotatedComponent::from)
            .collect();

        Ok(components)
    }

    /// get_axioms(self) -> List[model.AnnotatedComponent]
    ///
    /// Returns all axioms of the ontology.
    pub fn get_axioms(&mut self) -> PyResult<Vec<model::AnnotatedComponent>> {
        let r = self
            .ontology
            .iter()

            .filter_map(|a|
                if a.is_axiom() {
                    Some(model::AnnotatedComponent::from(a.clone()))
                } else {
                    None
                }
            )
            .collect();

        Ok(r)
    }


    /// get_components(self) -> List[model.AnnotatedComponent]
    ///
    /// Returns all axioms of the ontology.
    pub fn get_components(&mut self) -> PyResult<Vec<model::AnnotatedComponent>> {
        let r = self
            .ontology
            .iter()

            .map(model::AnnotatedComponent::from)
            .collect();

        Ok(r)
    }

    /// add_component(self, component: model.Component, annotations: Optional[List[model.Annotation]]=None) -> None
    ///
    /// Adds an axiom to the ontology with optional annotations.
    #[pyo3(signature = (component, annotations = None))]
    pub fn add_component(
        &mut self,
        component: model::Component,
        annotations: Option<BTreeSet<model::Annotation>>,
    ) -> PyResult<()> {
        let ann: model::BTreeSetWrap<model::Annotation> = annotations.unwrap_or(BTreeSet::new()).into();
        let annotated_component: AnnotatedComponent<ArcStr> = model::AnnotatedComponent {
            component,
            ann,
        }.into();
        self.insert(&annotated_component);
        self.ontology.insert(annotated_component);

        Ok(())
    }

    /// add_axiom(self, ax: model.Component, annotations: Optional[List[model.Annotation]]=None) -> None
    ///
    /// Synonym for `add_component`
    #[pyo3(signature = (ax, annotations = None))]
    pub fn add_axiom(
        &mut self,
        ax: model::Component,
        annotations: Option<BTreeSet<model::Annotation>>,
    ) -> PyResult<()> {
        self.add_component(ax, annotations)
    }

    /// remove_component(self, component: model.Component) -> None
    ///
    /// Removes a component from the ontology.
    pub fn remove_component(&mut self, component: model::Component) -> PyResult<()> {
        let ax: Component<Arc<str>> = component.into();
        let annotated = self
            .ontology
            .iter()
            .find(|a| a.component == ax)
            .ok_or(PyValueError::new_err("args"))?
            .to_owned();
        self.ontology.remove(&annotated);

        Ok(())
    }


    /// remove_axiom(self, ax: model.Component) -> None
    ///
    /// Synonym for `remove_component`
    pub fn remove_axiom(&mut self, ax: model::Component) -> PyResult<()> {
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
    pub fn iri(&self, iri: String, absolute: Option<bool>) -> PyResult<model::IRI> {
        let absolute = absolute.unwrap_or_else(|| iri.contains("://"));
        let r = if absolute {
            model::IRI::new(iri, &self.build)
        } else {
            self.curie(iri)?
        };
        Ok(r)
    }


    /// curie(self, iri: str) -> model.IRI
    ///
    /// Creates a new IRI from CURIE string.
    ///
    /// Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
    pub fn curie(&self, curie: String) -> PyResult<model::IRI> {
        let iri = self.mapping.expand_curie_string(&curie).map_err(to_py_err!("Invalid curie"))?;
        Ok(model::IRI::new(iri, &self.build))
    }

    /// clazz(self, iri: str, *, absolute: Optional[bool]=None) -> model.Class
    ///
    /// Convenience method to create a Class from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn clazz(&self, iri: String, absolute: Option<bool>) -> PyResult<model::Class> {
        Ok(model::Class(self.iri(iri, absolute)?))
    }

    /// declare_class(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(Class(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_class(&mut self, iri: String, absolute: Option<bool>) -> PyResult<bool> {
        Ok(self.ontology.declare::<horned_owl::model::Class<ArcStr>>(self.clazz(iri, absolute)?.into()))
    }

    /// object_property(self, iri: str, *, absolute: Optional[bool]=None) -> model.ObjectProperty
    ///
    /// Convenience method to create an ObjectProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn object_property(&self, iri: String, absolute: Option<bool>) -> PyResult<model::ObjectProperty> {
        Ok(model::ObjectProperty(self.iri(iri, absolute)?))
    }

    /// declare_object_property(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(ObjectProperty(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_object_property(&mut self, iri: String, absolute: Option<bool>) -> PyResult<bool> {
        Ok(self.ontology.declare::<horned_owl::model::ObjectProperty<ArcStr>>(self.object_property(iri, absolute)?.into()))
    }

    /// data_property(self, iri: str, *, absolute: Optional[bool]=None) -> model.DataProperty
    ///
    /// Convenience method to create a DataProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn data_property(&self, iri: String, absolute: Option<bool>) -> PyResult<model::DataProperty> {
        Ok(model::DataProperty(self.iri(iri, absolute)?))
    }

    /// declare_data_property(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(DataProperty(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_data_property(&mut self, iri: String, absolute: Option<bool>) -> PyResult<bool> {
        Ok(self.ontology.declare::<horned_owl::model::DataProperty<ArcStr>>(self.data_property(iri, absolute)?.into()))
    }

    /// annotation_property(self, iri: str, *, absolute: Optional[bool]=None) -> model.annotationProperty
    ///
    /// Convenience method to create an annotationProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn annotation_property(&self, iri: String, absolute: Option<bool>) -> PyResult<model::AnnotationProperty> {
        Ok(model::AnnotationProperty(self.iri(iri, absolute)?))
    }

    /// declare_annotation_property(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(annotationProperty(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_annotation_property(&mut self, iri: String, absolute: Option<bool>) -> PyResult<bool> {
        Ok(self.ontology.declare::<horned_owl::model::AnnotationProperty<ArcStr>>(self.annotation_property(iri, absolute)?.into()))
    }

    /// named_individual(self, iri: str, *, absolute: Optional[bool]=None) -> model.NamedIndividual
    ///
    /// Convenience method to create a NamedIndividual from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn named_individual(&self, iri: String, absolute: Option<bool>) -> PyResult<model::NamedIndividual> {
        Ok(model::NamedIndividual(self.iri(iri, absolute)?))
    }

    /// declare_individual(self, iri: str, *, absolute: Optional[bool]=None) -> bool
    ///
    /// Convenience method to add a Declare(NamedIndividual(iri)) axiom.
    #[pyo3[signature = (iri, *, absolute = None)]]
    pub fn declare_individual(&mut self, iri: String, absolute: Option<bool>) -> PyResult<bool> {
        Ok(self.ontology.declare::<horned_owl::model::NamedIndividual<ArcStr>>(self.named_individual(iri, absolute)?.into()))
    }


    /// anonymous_individual(self, iri: str) -> model.AnonymousIndividual
    ///
    /// Convenience method to create an AnonymousIndividual from a string.
    pub fn anonymous_individual(&self, name: String) -> model::AnonymousIndividual {
        AnonymousIndividual(name.into())
    }


    /// get_descendants(self, parent: str, iri_is_absolute: Optional[bool] = None) -> Set[str]
    ///
    /// Gets all direct and indirect subclasses of a class.
    #[pyo3[signature = (parent_iri, *, iri_is_absolute = None)]]
    pub fn get_descendants(&self, parent_iri: String, iri_is_absolute: Option<bool>) -> PyResult<HashSet<String>> {
        let mut descendants = HashSet::new();
        let parent_iri: IRI<ArcStr> = self.iri(parent_iri, iri_is_absolute)?.into();

        self.recurse_descendants(&parent_iri, &mut descendants);

        Ok(descendants)
    }

    /// get_ancestors(onto: PyIndexedOntology, child: str, iri_is_absolute: Optional[bool] = None) -> Set[str]
    ///
    /// Gets all direct and indirect super classes of a class.
    #[pyo3[signature = (child_iri, *, iri_is_absolute = None)]]
    pub fn get_ancestors(&self, child_iri: String, iri_is_absolute: Option<bool>) -> PyResult<HashSet<String>> {
        let mut ancestors = HashSet::new();

        let child_iri: IRI<ArcStr> = self.iri(child_iri, iri_is_absolute)?.into();

        self.recurse_ancestors(&child_iri, &mut ancestors);

        Ok(ancestors)
    }
}

impl PyIndexedOntology {
    fn recurse_descendants(
        &self,
        superclass: &IRI<ArcStr>,
        descendants: &mut HashSet<String>,
    ) {
        descendants.insert(superclass.into());
        if self.classes_to_subclasses.contains_key(superclass) {
            for cls2 in &mut self.classes_to_subclasses[superclass].iter() {
                self.recurse_descendants(cls2, descendants);
            }
        }
    }

    fn recurse_ancestors(
        &self,
        subclass: &IRI<ArcStr>,
        ancestors: &mut HashSet<String>,
    ) {
        ancestors.insert(subclass.into());
        if self.classes_to_superclasses.contains_key(subclass) {
            for cls2 in &mut self.classes_to_superclasses[subclass].iter() {
                self.recurse_ancestors(cls2, ancestors);
            }
        }
    }

    pub fn insert(&mut self, ax: &AnnotatedComponent<ArcStr>) -> () {
        let b = Build::new();

        match ax.kind() {
            ComponentKind::AnnotationAssertion => match ax.clone().component {
                Component::AnnotationAssertion(AnnotationAssertion { subject, ann }) => match ann {
                    Annotation {
                        ap,
                        av: AnnotationValue::Literal(Literal::Simple { literal }),
                    } => {
                        if AnnotationBuiltIn::Label.underlying().eq(&ap.0.to_string()) {
                            let _ = &self
                                .labels_to_iris
                                .insert(literal.clone(), b.iri(subject.deref()));
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            ComponentKind::SubClassOf => {
                match ax.clone().component {
                    Component::SubClassOf(SubClassOf { sup, sub }) => {
                        match sup {
                            ClassExpression::Class(c) => {
                                match sub {
                                    ClassExpression::Class(d) => {
                                        //Direct subclasses only
                                        let _ = &self
                                            .classes_to_subclasses
                                            .entry(c.0.clone())
                                            .or_insert(HashSet::new())
                                            .insert(d.0.clone());
                                        let _ = &self
                                            .classes_to_superclasses
                                            .entry(d.0.clone())
                                            .or_insert(HashSet::new())
                                            .insert(c.0.clone());
                                    }
                                    _ => (),
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    pub fn from(iro: IRIMappedOntology<ArcStr, Arc<AnnotatedComponent<ArcStr>>>) -> PyIndexedOntology {
        let mut ino = PyIndexedOntology::default();

        for ax in iro.iter() {
            ino.insert(&ax);
        }

        ino.ontology = iro;

        ino
    }

    fn get_id(&mut self) -> Option<&OntologyID<ArcStr>> {
        match self.ontology.component_for_kind(ComponentKind::OntologyID).next() {
            Some(horned_owl::model::AnnotatedComponent::<ArcStr> {
                     component: Component::OntologyID(id),
                     ann: _
                 }) => Some(id),
            _ => None
        }
    }
}

/// get_descendants(onto: PyIndexedOntology, parent: str) -> Set[str]
///
/// DEPRECATED: please use `PyIndexedOntology::get_descendants` instead
/// Gets all direct and indirect subclasses of a class.
#[pyfunction]
pub fn get_descendants(onto: &PyIndexedOntology, parent: String) -> PyResult<HashSet<String>> {
    onto.get_descendants(parent, Some(true))
}


/// get_ancestors(onto: PyIndexedOntology, child: str) -> Set[str]
///
/// DEPRECATED: please use `PyIndexedOntology::get_ancestors` instead
/// Gets all direct and indirect super classes of a class.
#[pyfunction]
pub fn get_ancestors(onto: &PyIndexedOntology, child: String) -> PyResult<HashSet<String>> {
    onto.get_ancestors(child, Some(true))
}
