use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pyo3::wrap_pyfunction;
use std::fs::File;
use std::io::BufReader;

#[macro_use]
mod doc;
mod model;

use horned_owl::model::*;
use horned_owl::ontology::axiom_mapped::AxiomMappedOntology;
use horned_owl::ontology::iri_mapped::IRIMappedOntology;
use horned_owl::vocab::{AnnotationBuiltIn, WithIRI};
//use horned_owl::ontology::declaration_mapped::DeclarationMappedIndex;
//use horned_owl::ontology::logically_equal::LogicallyEqualIndex;
use horned_owl::io::rdf::reader::IncompleteParse;
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration};
//use horned_owl::ontology::indexed::ThreeIndexedOntology;
//use horned_owl::ontology::set::SetIndex;
use horned_owl::error::HornedError;
use horned_owl::io::rdf::reader::RDFOntology;
use horned_owl::ontology::axiom_mapped::ArcAxiomMappedOntology;
use horned_owl::ontology::iri_mapped::ArcIRIMappedOntology;
use horned_owl::ontology::set::SetOntology;

use curie::{Curie, PrefixMapping};

use std::collections::HashSet;
use std::collections::{BTreeSet, HashMap};
use std::default::Default;
use std::ops::Deref;
use std::path::Path;
use std::time::Instant;
//use failure::Error;
use std::sync::Arc;


/// Represents a loaded ontology.
#[pyclass]
struct PyIndexedOntology {
    //State variables private to Rust, exposed through methods to Python
    labels_to_iris: HashMap<String, IRI<ArcStr>>,

    classes_to_subclasses: HashMap<IRI<ArcStr>, HashSet<IRI<ArcStr>>>, //axiom typed index would give subclass axioms
    classes_to_superclasses: HashMap<IRI<ArcStr>, HashSet<IRI<ArcStr>>>,

    //The primary store of the axioms is a Horned OWL indexed ontology
    ontology: ArcIRIMappedOntology,
    //Need this for converting IRIs to IDs and for saving again afterwards
    mapping: PrefixMapping,
    build: Build<ArcStr>,
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
    /// get_id_for_iri(self, iri: str) -> Optional[str]
    /// 
    /// Gets the ID of term by it IRI.
    /// 
    /// If the term does not have an ID, `None` is returned.
    fn get_id_for_iri(&mut self, py: Python, iri: String) -> PyResult<PyObject> {
        let res = self.mapping.shrink_iri(&iri);

        if let Ok(curie) = res {
            Ok(curie.to_string().to_object(py))
        } else {
            //Return null
            Ok(().to_object(py))
        }
    }

    /// get_iri_for_id(self, id: str) -> Optional[str]
    /// 
    /// Gets the IRI of a term by its ID.
    /// 
    /// If the term does not have an IRI, `None` is returned.
    fn get_iri_for_id(&mut self, py: Python, id: String) -> PyResult<PyObject> {
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
    fn add_prefix_mapping(&mut self, iriprefix: String, mappedid: String) -> PyResult<()> {
        let result = self.mapping.add_prefix(&iriprefix, &mappedid);
        if let Ok(()) = result {
            Ok(())
        } else {
            Err(PyValueError::new_err("Error - prefix is invalid."))
        }
    }

    /// set_label(self, iri: str, label: str) -> None
    /// 
    /// Sets the label of a term by iri.
    /// 
    /// Adds an or updates the `AnnotationAssertion` axiom for `rdfs:label`.
    fn set_label(&mut self, iri: String, label: String) -> PyResult<()> {
        let iri = self.build.iri(iri);

        let ax1: AnnotatedAxiom<ArcStr> = Axiom::AnnotationAssertion(AnnotationAssertion {
            subject: iri.clone().into(),
            ann: Annotation {
                ap: self
                    .build
                    .annotation_property(AnnotationBuiltIn::LABEL.iri_s().clone()),
                av: AnnotationValue::Literal(Literal::Simple {
                    literal: label.clone(),
                }),
            },
        })
        .into();

        //If we already have a label, update it:
        let old_ax = &self
            .ontology
            .axiom_for_iri(&iri)
            .filter_map(|aax: &AnnotatedAxiom<ArcStr>| match &aax.axiom {
                Axiom::AnnotationAssertion(AnnotationAssertion {
                    subject: _subj,
                    ann,
                }) => match ann {
                    Annotation {
                        ap,
                        av: AnnotationValue::Literal(Literal::Simple { literal: _old }),
                    } => {
                        if AnnotationBuiltIn::LABEL.iri_s().eq(&ap.0.to_string()) {
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
    fn get_iri_for_label(&mut self, py: Python, label: String) -> PyResult<PyObject> {
        let iri_value = &self.labels_to_iris.get(&label);
        if let Some(iri_value) = iri_value {
            Ok(iri_value.to_string().to_object(py))
        } else {
            Ok(().to_object(py))
        }
    }

    /// get_iri(self) -> Optional[str]
    /// 
    /// Returns the ontology iri, if it exists.
    fn get_iri(&mut self, py: Python) -> PyResult<PyObject> {
        let iri_value = &self.ontology.id().iri.as_ref();
        if let Some(iri_value) = iri_value {
            Ok(iri_value.to_string().to_object(py))
        } else {
            Ok(().to_object(py))
        }
    }

    /// get_version_iri(self) -> Optional[str]
    /// 
    /// Returns the ontologys version iri, if it exists.
    fn get_version_iri(&mut self, py: Python) -> PyResult<PyObject> {
        let iri_value = &self.ontology.id().viri.as_ref();
        if let Some(iri_value) = iri_value {
            Ok(iri_value.to_string().to_object(py))
        } else {
            Ok(().to_object(py))
        }
    }

    /// get_subclasses(self, iri: str) -> Set[str]
    /// 
    /// Gets all subclasses of an entity.
    fn get_subclasses(&mut self, iri: String) -> PyResult<HashSet<String>> {
        let iri = self.build.iri(iri);

        let subclasses = self.classes_to_subclasses.get(&iri);
        if let Some(subclss) = subclasses {
            let subclasses: HashSet<String> = subclss.iter().map(|sc| sc.to_string()).collect();
            Ok(subclasses)
        } else {
            Ok(HashSet::new())
        }
    }

    /// get_superclasses(self, iri: str) -> Set[str]
    /// 
    /// Gets all superclasses of an entity.
    fn get_superclasses(&mut self, iri: String) -> PyResult<HashSet<String>> {
        let iri = self.build.iri(iri);

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
    fn get_classes(&mut self) -> PyResult<HashSet<String>> {
        //Get the DeclareClass axioms
        let classes = self.ontology.axiom_for_kind(AxiomKind::DeclareClass);

        let classes: HashSet<String> = classes
            .filter_map(|aax| match aax.clone().axiom {
                Axiom::DeclareClass(dc) => Some(dc.0 .0.to_string()),
                _ => None,
            })
            .collect();
        Ok(classes)
    }

    /// get_object_properties(self) -> Set[str]
    /// 
    /// Returns the IRIs of all declared object properties in the ontology.
    fn get_object_properties(&mut self) -> PyResult<HashSet<String>> {
        //Get the DeclareObjectProperty axioms
        let object_properties = self
            .ontology
            .axiom_for_kind(AxiomKind::DeclareObjectProperty);

        let object_properties: HashSet<String> = object_properties
            .filter_map(|aax| match aax.clone().axiom {
                Axiom::DeclareObjectProperty(dop) => Some(dop.0 .0.to_string()),
                _ => None,
            })
            .collect();
        Ok(object_properties)
    }

    /// get_annotation(self, class_iri: str, ann_iri: str) -> Optional[str]
    /// 
    /// Gets the first annotated value for an entity and annotation property.
    /// 
    /// Note: If there are multiple annotation axioms for the queried entity and annotation property,
    /// the order is neither necessarily the same as in the ontology neither is it stable.
    /// Get all annotation values with `PyIndexedOntology.get_annotations`.
    fn get_annotation(
        &mut self,
        py: Python,
        class_iri: String,
        ann_iri: String,
    ) -> PyResult<PyObject> {
        let annots = self.get_annotations(class_iri, ann_iri);

        let mut literal_value = ().to_object(py);

        if let Ok(literal_values) = annots {
            if !literal_values.is_empty() {
                literal_value = literal_values.into_iter().next().to_object(py);
            }
        }

        Ok(literal_value)
    }

    /// get_annotations(self, class_iri: str, ann_iri: str) -> List[str]
    /// 
    /// Gets all annotated value for an entity and annotation property.
    /// 
    /// Note: The order is neither necessarily the same as in the ontology neither is it stable.
    /// Get all annotation values with `PyIndexedOntology.get_annotations`.
    fn get_annotations(&mut self, class_iri: String, ann_iri: String) -> PyResult<Vec<String>> {
        let iri = self.build.iri(class_iri);

        let literal_values : Vec<String> = self.ontology.axiom_for_iri(&iri)
                                .filter_map(|aax: &AnnotatedAxiom<ArcStr>| {
            match &aax.axiom {
                Axiom::AnnotationAssertion(AnnotationAssertion{subject:_,ann}) => {
                        match ann {
                            Annotation {ap, av:  AnnotationValue::Literal(Literal::Simple{literal}) } => {
                                if ann_iri.eq(&ap.0.to_string()) {
                                    Some(literal.clone())
                                } else {
                                    None
                                }
                            },
                            //Language { literal: String, lang: String },
                            Annotation {ap, av:  AnnotationValue::Literal(Literal::Language{literal, lang:_}) } => {
                                if ann_iri.eq(&ap.0.to_string()) {
                                    Some(literal.clone())
                                } else {
                                    None
                                }
                            },
                            //Datatype { literal: String, datatype_iri: IRI },
                            Annotation {ap, av:  AnnotationValue::Literal(Literal::Datatype{literal, datatype_iri:_}) } => {
                                if ann_iri.eq(&ap.0.to_string()) {
                                    Some(literal.clone())
                                } else {
                                    None
                                }
                            },
                            _ => None,
                        }
                    },
                    _ => None,
                }
        }).collect();

        Ok(literal_values)
    }

    /// save_to_file(self, file_name: str) -> None
    /// 
    /// Saves the ontology to disk in owx format.
    fn save_to_file(&mut self, file_name: String) -> PyResult<()> {
        let before = Instant::now();

        let mut file = File::create(file_name)?;
        let mut amo: ArcAxiomMappedOntology = AxiomMappedOntology::new_arc();
        let oid = &self.ontology.id().clone();

        amo.mut_id().iri = oid.iri.clone();
        amo.mut_id().viri = oid.viri.clone();
        //Copy the axioms into an AxiomMappedOntology as that is what horned owl writes
        for aax in self.ontology.iter() {
            amo.insert(aax.clone());
        }
        let time_middle = before.elapsed().as_secs();
        println!(
            "Finished preparing ontology for saving in {:?} seconds.",
            time_middle
        );
        let before = Instant::now();

        let result = horned_owl::io::owx::writer::write(&mut file, &amo, Some(&self.mapping));

        let time_after = before.elapsed().as_secs();
        println!(
            "Finished saving ontology to file in  {:?} seconds.",
            time_after
        );

        match result {
            Ok(()) => Ok(()),
            Err(error) => panic!("Problem saving the ontology to a file: {:?}", error),
        }
    }

    /// get_axioms_for_iri(self, iri: str) -> List[model.AnnotatedAxiom]
    /// 
    /// Gets all axioms for an entity.
    fn get_axioms_for_iri(&mut self, py: Python<'_>, iri: String) -> PyResult<Vec<PyObject>> {
        let b = Build::new();
        let iri = b.iri(iri);

        let axioms = self
            .ontology
            .axiom_for_iri(&iri)
            .map(|a| model::AnnotatedAxiom::from(a))
            .map(|a: model::AnnotatedAxiom| a.into_py(py) )
            .collect();

        Ok(axioms)
    }

    /// get_axioms(self) -> List[model.AnnotatedAxioms]
    /// 
    /// Returns all axioms of the ontology.
    fn get_axioms(&mut self, py: Python) -> PyResult<Vec<PyObject>> {
        let r = self
            .ontology
            .iter()
            .map(|a| a.clone().into())
            .map(|a: model::AnnotatedAxiom| a.into_py(py))
            .collect();

        Ok(r)
    }

    /// add_axiom(self, ax: model.Axiom, annotations: Optional[List[model.Annotation]]) -> None
    /// 
    /// Adds an axiom to the ontology with optional annotations.
    fn add_axiom(
        &mut self,
        ax: model::Axiom,
        annotations: Option<BTreeSet<model::Annotation>>,
    ) -> PyResult<()> {
        let ann: model::BTreeSetWrap<model::Annotation> = annotations.unwrap_or(BTreeSet::new()).into();
        let annotated_axiom = model::AnnotatedAxiom {
            axiom: ax,
            ann,
        };
        self.ontology.insert(annotated_axiom);

        Ok(())
    }

    /// remove_axiom(self, ax: model.Axiom) -> None
    /// 
    /// Removes an axiom from the ontology.
    fn remove_axiom(&mut self, ax: model::Axiom) -> PyResult<()> {
        let ax: Axiom<Arc<str>> = ax.into();
        let annotated = self
            .ontology
            .iter()
            .find(|a| a.axiom == ax)
            .ok_or(PyValueError::new_err("args"))?
            .to_owned();
        self.ontology.remove(&annotated);

        Ok(())
    }

    /// iri(self, iri: str) -> model.IRI
    /// 
    /// Creates an new IRI from string.
    /// 
    /// Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
    fn iri(&self, iri: String) -> model::IRI {
        model::IRI::new(iri, &self.build)
    }
}
impl PyIndexedOntology {
    fn insert(&mut self, ax: &AnnotatedAxiom<ArcStr>) -> () {
        let b = Build::new();

        match ax.kind() {
            AxiomKind::AnnotationAssertion => match ax.clone().axiom {
                Axiom::AnnotationAssertion(AnnotationAssertion { subject, ann }) => match ann {
                    Annotation {
                        ap,
                        av: AnnotationValue::Literal(Literal::Simple { literal }),
                    } => {
                        if AnnotationBuiltIn::LABEL.iri_s().eq(&ap.0.to_string()) {
                            let _ = &self
                                .labels_to_iris
                                .insert(literal.clone(), b.iri(subject.deref()));
                        }
                    }
                    _ => (),
                },
                _ => (),
            },
            AxiomKind::SubClassOf => {
                match ax.clone().axiom {
                    Axiom::SubClassOf(SubClassOf { sup, sub }) => {
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

    fn from(iro: IRIMappedOntology<ArcStr, Arc<AnnotatedAxiom<ArcStr>>>) -> PyIndexedOntology {
        let mut ino = PyIndexedOntology::default();

        for ax in iro.iter() {
            ino.insert(&ax);
        }

        ino.ontology = iro;

        ino
    }
}

fn open_ontology_owx(
    ontology: &str,
    b: &Build<Arc<str>>,
) -> Result<(SetOntology<ArcStr>, PrefixMapping), HornedError> {
    let r = if Path::new(&ontology).exists() {
        let file = File::open(ontology).ok().unwrap();
        let mut f = BufReader::new(file);
        horned_owl::io::owx::reader::read_with_build(&mut f, &b)
    } else {
        //just try to parse the string
        let str_val = ontology.as_bytes();
        let mut f = BufReader::new(str_val);
        horned_owl::io::owx::reader::read_with_build(&mut f, &b)
    };
    r
}

fn open_ontology_rdf(
    ontology: &str,
    b: &Build<Arc<str>>,
) -> Result<
    (
        RDFOntology<ArcStr, Arc<AnnotatedAxiom<ArcStr>>>,
        IncompleteParse<Arc<str>>,
    ),
    HornedError,
> {
    let r = if Path::new(&ontology).exists() {
        let file = File::open(ontology).ok().unwrap();
        let mut f = BufReader::new(file);
        horned_owl::io::rdf::reader::read_with_build(&mut f, &b, ParserConfiguration::default())
    } else {
        //just try to parse the string
        let str_val = ontology.as_bytes();
        let mut f = BufReader::new(str_val);
        horned_owl::io::rdf::reader::read_with_build(
            &mut f,
            &b,
            ParserConfiguration {
                rdf: RDFParserConfiguration { lax: true },
                ..Default::default()
            },
        )
    };
    r
}


/// open_ontology(ontology: str) -> PyIndexedOntology
/// 
/// Opens an ontology from a path or plain text.
/// 
/// If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interepreted as an ontology in either owx or owl format.
/// Note: Only .owl and .owx files are currently supported.
#[pyfunction]
fn open_ontology(ontology: &PyString) -> PyResult<PyIndexedOntology> {
    let ontology: String = ontology.extract().unwrap();

    let b = Build::new_arc();

    let result = if ontology.ends_with("owx") {
        let r = open_ontology_owx(&ontology, &b);
        //println!("Got result {:?}",r);
        if r.is_ok() {
            let (o, m) = r.ok().unwrap();
            //println!("Got ontology from owx {:?}",o);
            //println!("About to build indexes");
            let iro = IRIMappedOntology::from(o);
            let mut lo = PyIndexedOntology::from(iro);
            lo.mapping = m; //Needed when saving
            Ok(lo)
        } else {
            Err(PyValueError::new_err("Unable to open ontology"))
        }
    } else if ontology.ends_with("owl") {
        let r2 = open_ontology_rdf(&ontology, &b);
        if r2.is_ok() {
            let (o, _) = r2.ok().unwrap();
            //println!("Got ontology from rdf {:?}",o);
            let so = SetOntology::from(o);
            let iro = IRIMappedOntology::from(so);
            let lo = PyIndexedOntology::from(iro);
            Ok(lo)
        } else {
            Err(PyValueError::new_err("Unable to open ontology"))
        }
    } else {
        // No recognised suffix, maybe it is a string value, just try to parse
        let r = open_ontology_owx(&ontology, &b);
        if r.is_ok() {
            let (o, m) = r.ok().unwrap();
            //println!("Got ontology from owx {:?}",o);
            //println!("About to build indexes");
            let iro = IRIMappedOntology::from(o);
            let mut lo = PyIndexedOntology::from(iro);
            lo.mapping = m; //Needed when saving
            Ok(lo)
        } else {
            let r2 = open_ontology_rdf(&ontology, &b);
            if r2.is_ok() {
                let (o, _) = r2.ok().unwrap();
                //println!("Got ontology from rdf {:?}",o);
                let so = SetOntology::from(o);
                let iro = IRIMappedOntology::from(so);
                let lo = PyIndexedOntology::from(iro);
                Ok(lo)
            } else {
                Err(PyValueError::new_err("Unable to open ontology"))
            }
        }
    };
    result
}

/// get_descendants(onto: PyIndexedOntology, parent: str) -> Set[str]
/// 
/// Gets all direct and indirect subclasses of an class.
#[pyfunction]
fn get_descendants(onto: &PyIndexedOntology, parent: &PyString) -> PyResult<HashSet<String>> {
    let mut descendants = HashSet::new();
    let parent: String = parent.extract().unwrap();

    let b = Build::new();
    let parentiri = b.iri(parent);

    recurse_descendants(onto, &parentiri, &mut descendants);

    Ok(descendants)
}

fn recurse_descendants(
    onto: &PyIndexedOntology,
    superclass: &IRI<ArcStr>,
    descendants: &mut HashSet<String>,
) {
    descendants.insert(superclass.into());
    if onto.classes_to_subclasses.contains_key(superclass) {
        for cls2 in &mut onto.classes_to_subclasses[superclass].iter() {
            recurse_descendants(onto, cls2, descendants);
        }
    }
}

/// get_ancestors(onto: PyIndexedOntology, child: str) -> Set[str]
/// 
/// Gets all direct and indirect super classes of a class.
#[pyfunction]
fn get_ancestors(onto: &PyIndexedOntology, child: &PyString) -> PyResult<HashSet<String>> {
    let mut ancestors = HashSet::new();
    let child: String = child.extract().unwrap();

    let b = Build::new();
    let childiri = b.iri(child);

    recurse_ancestors(onto, &childiri, &mut ancestors);

    Ok(ancestors)
}

fn recurse_ancestors(
    onto: &PyIndexedOntology,
    subclass: &IRI<ArcStr>,
    ancestors: &mut HashSet<String>,
) {
    ancestors.insert(subclass.into());
    if onto.classes_to_superclasses.contains_key(subclass) {
        for cls2 in &mut onto.classes_to_superclasses[subclass].iter() {
            recurse_ancestors(onto, cls2, ancestors);
        }
    }
}

#[pymodule]
fn pyhornedowl(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyIndexedOntology>()?;

    m.add_function(wrap_pyfunction!(open_ontology, m)?)?;
    m.add_function(wrap_pyfunction!(get_descendants, m)?)?;
    m.add_function(wrap_pyfunction!(get_ancestors, m)?)?;

    let model_sub_module = model::py_module(py)?;
    m.add_submodule(model_sub_module)?;

    Ok(())
}
