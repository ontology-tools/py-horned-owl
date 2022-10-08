use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{PyString,PyList};
use pyo3::exceptions::PyValueError;
use pyo3::PyDowncastError;
use std::io::BufReader;
use std::fs::File;

use horned_owl::vocab::{AnnotationBuiltIn,WithIRI};
use horned_owl::model::*;
use horned_owl::ontology::iri_mapped::IRIMappedOntology;
use horned_owl::ontology::axiom_mapped::AxiomMappedOntology;
//use horned_owl::ontology::declaration_mapped::DeclarationMappedIndex;
//use horned_owl::ontology::logically_equal::LogicallyEqualIndex;
use horned_owl::io::rdf::reader::IncompleteParse;
use horned_owl::io::{ParserConfiguration, RDFParserConfiguration};
//use horned_owl::ontology::indexed::ThreeIndexedOntology;
//use horned_owl::ontology::set::SetIndex;
use horned_owl::ontology::set::SetOntology;
use horned_owl::error::HornedError;
use horned_owl::io::rdf::reader::RDFOntology;
use horned_owl::ontology::axiom_mapped::ArcAxiomMappedOntology;
use horned_owl::ontology::iri_mapped::ArcIRIMappedOntology;

use curie::{PrefixMapping,Curie};

use std::collections::HashSet;
use std::collections::HashMap;
use std::time::Instant;
use std::default::Default;
use std::path::Path;
use std::ops::Deref;
//use failure::Error;
use std::sync::Arc;
use std::borrow::Borrow;

#[pyclass]
#[derive(Clone,Default,Debug,PartialEq)]
struct SimpleAxiomContent {
    #[pyo3(get,set)]
    str_val: Option<String>,
    #[pyo3(get,set)]
    ax_val: Option<PySimpleAxiom>,
}

impl SimpleAxiomContent {
    fn parse(ax: Vec<PyObject>, py: Python ) -> Vec<SimpleAxiomContent> {
        let els: Vec<SimpleAxiomContent> = ax.into_iter().map(|aax: PyObject| {
            //Is aax a simple string or itelf another list? One of these should work.
            let strax: Result<&PyString,PyDowncastError> = aax.as_ref(py).downcast::<PyString>();
            let lstax: Result<&PyList,PyDowncastError> = aax.as_ref(py).downcast::<PyList>();

            if let Ok(str_val) = strax {
                SimpleAxiomContent{str_val:Some(str_val.to_string()),ax_val:None}
            } else if let Ok(list_val) = lstax {
                let pyeles: Vec<PyObject> = list_val.extract().unwrap();
                let eles: Vec<SimpleAxiomContent> = SimpleAxiomContent::parse(pyeles,py);
                SimpleAxiomContent{str_val:None,ax_val:Some(PySimpleAxiom{elements:eles})}
            } else {
                println!("Got an unparseable value: {:?}",aax);
                panic!("Unparseable axiom sent from Python to Rust.");
            }
        }
        ).collect();

        els
    }
}

impl From<String> for SimpleAxiomContent {
    fn from(s: String) -> Self {
        SimpleAxiomContent { str_val: Some(s), ax_val: None }
    }
}

impl From<&str> for SimpleAxiomContent {
    fn from(s: &str) -> Self {
        SimpleAxiomContent { str_val: Some(s.to_string()), ax_val: None }
    }
}

impl From<SimpleAxiomContent> for String {
    fn from(s: SimpleAxiomContent) -> String {
        s.str_val.unwrap()
    }
}

impl From<&SimpleAxiomContent> for String {
    fn from(s: &SimpleAxiomContent) -> String {
        s.str_val.as_ref().unwrap().clone()
    }
}

impl Borrow<str> for SimpleAxiomContent {
    fn borrow(&self) -> &str {
        self.str_val.as_ref().unwrap().as_ref()
    }
}

impl Borrow<str> for &SimpleAxiomContent {
    fn borrow(&self) -> &str {
        self.str_val.as_ref().unwrap().as_ref()
    }
}

impl From<PySimpleAxiom> for SimpleAxiomContent {
    fn from(item: PySimpleAxiom) -> Self {
        SimpleAxiomContent { str_val: None, ax_val: Some(item) }
    }
}


#[pyclass]
#[derive(Clone,Default,Debug,PartialEq)]
struct PySimpleAxiom {
    elements: Vec<SimpleAxiomContent>,
}

//impl FromPyObject for PySimpleAxiom {
//    fn extract(ob: PyObject) -> PyResult<PySimpleAxiom> {
//        let elements: Vec<SimpleAxiomContent> = Vec::extract(ob);
//        PySimpleAxiom(elements)
//    }
//}

impl ToPyObject for PySimpleAxiom {

    fn to_object(&self, py: Python<'_>) -> PyObject {

        //if just one object, don't send a whole list
        if self.elements.len() ==1 {
            let ele = self.elements.iter().next().unwrap();
            if ele.str_val.is_some() {
                ele.str_val.as_ref().unwrap().to_object(py)
            } else if ele.ax_val.is_some() {
                ele.ax_val.as_ref().unwrap().to_object(py)
            } else {
                ().to_object(py)
            }
        } else { // More than one
            let list = PyList::empty(py);

            for ele in self.elements.iter() {
                if ele.str_val.is_some() {
                    list.append(ele.str_val.as_ref().unwrap().to_object(py));
                } else {
                    if ele.ax_val.is_some() {
                        list.append(ele.ax_val.as_ref().unwrap().to_object(py));
                    }
                }
            };

            list.into()
        }
    }
}

impl From<&ObjectPropertyExpression<ArcStr>> for PySimpleAxiom {
    fn from(ope: &ObjectPropertyExpression<ArcStr>) -> PySimpleAxiom {
        let mut pyax = PySimpleAxiom::default();

        match ope {
            ObjectPropertyExpression::ObjectProperty(p) => {
                pyax.elements.push("ObjectProperty".into());
                pyax.elements.push(p.0.to_string().into());
            },
            ObjectPropertyExpression::InverseObjectProperty(p) => {
                pyax.elements.push("InverseObjectProperty".into());
                pyax.elements.push(p.0.to_string().into());
            }
        }

        pyax
    }
}

impl From<&ClassExpression<ArcStr>> for PySimpleAxiom {
    fn from(ce: &ClassExpression<ArcStr>) -> PySimpleAxiom {
        let mut pyax = PySimpleAxiom::default();

            match ce {
                ClassExpression::Class(c) => {
                    pyax.elements.push(c.0.to_string().into());
                },
                ClassExpression::ObjectIntersectionOf(clsses) => {
                    pyax.elements.push("ObjectIntersectionOf".into());
                    for ele in clsses {
                        pyax.elements.push(PySimpleAxiom::from(ele).into());
                    }
                },
                ClassExpression::ObjectUnionOf(clsses) => {
                    pyax.elements.push("ObjectUnionOf".into());
                    for ele in clsses {
                        pyax.elements.push(PySimpleAxiom::from(ele).into());
                    }
                }
                ClassExpression::ObjectComplementOf(ce) => {
                    pyax.elements.push("ObjectComplementOf".into());
                    pyax.elements.push(PySimpleAxiom::from(&(**ce)).into());
                },
                ClassExpression::ObjectSomeValuesFrom{ope,bce} => {
                    pyax.elements.push("ObjectSomeValuesFrom".into());
                    pyax.elements.push(PySimpleAxiom::from(ope).into());
                    pyax.elements.push(PySimpleAxiom::from(&(**bce)).into());
                },
                ClassExpression::ObjectAllValuesFrom{ope,bce} => {
                    pyax.elements.push("ObjectAllValuesFrom".into());
                    pyax.elements.push(PySimpleAxiom::from(ope).into());
                    pyax.elements.push(PySimpleAxiom::from(&(**bce)).into());
                },
                _ => ()
            }
        pyax
    }
}

impl From<&SimpleAxiomContent> for ClassExpression<ArcStr> {
    fn from(ce: &SimpleAxiomContent) -> ClassExpression<ArcStr> {
        let b = Build::new_arc();

        if let Some(axval) = &ce.ax_val {
            //Parse axiom into class expression
            //It has some elements, the first of which should be the type of the expression
            let mut eles = axval.elements.iter();
            let cename: String = eles.next().unwrap().into();

            match &cename[..] {
                "ObjectSomeValuesFrom" | "ObjectAllValuesFrom" => {
                    //First an object property expression
                    let objpe: ObjectPropertyExpression<ArcStr> = eles.next().unwrap().into();
                    //Then its target
                    let objptar = eles.next().unwrap();

                    match &cename[..] {
                        "ObjectSomeValuesFrom" => {
                            ClassExpression::ObjectSomeValuesFrom{
                                            ope: objpe,
                                       bce: b.class(objptar).into()
                            }
                        },
                        "ObjectAllValuesFrom" => {
                            ClassExpression::ObjectAllValuesFrom{
                                            ope: objpe,
                                       bce: b.class(objptar).into()
                            }
                        },
                        _ => {panic!("Class expression not supported.")}
                    }
                },
                "ObjectIntersectionOf" | "ObjectUnionOf" => {
                    //The rest of the list should have classes that remain in the list
                    let clsses: Vec<ClassExpression<ArcStr>> = eles.map(|clss| b.class(clss).into()).collect();
                    match &cename[..] {
                        "ObjectIntersectionOf" => {
                            ClassExpression::ObjectIntersectionOf (clsses)
                        },
                        "ObjectUnionOf" => {
                            ClassExpression::ObjectUnionOf (clsses)
                        },
                        _ => {panic!{"Class expression not supported."}}
                    }
                },
                "ObjectComplementOf" => {
                    let objctar = eles.next().unwrap();
                    ClassExpression::ObjectComplementOf(b.class(objctar).into())
                },
                "ObjectHasSelf" => {
                    let objprp = eles.next().unwrap();
                    ClassExpression::ObjectHasSelf(objprp.into())
                },
                _ => {
                    println!("Class expression name: {:?} not supported.",cename);
                    panic!("Class expression not supported.")
                }
            }
        } else if let Some(strval) = &ce.str_val {
            //Parse string value into a simple class
            ClassExpression::Class(Class(b.iri(strval.clone())))
        } else {
            panic!("Unparseable class expression")
        }
    }
}

impl From<&SimpleAxiomContent> for ObjectPropertyExpression<ArcStr> {
    fn from(ope: &SimpleAxiomContent) -> ObjectPropertyExpression<ArcStr> {
        let b = Build::new();

        if let Some(axval) = &ope.ax_val {
            //Parse axiom into object property expression
            //It has some elements, the first of which should be the type of the expression
            let mut eles = axval.elements.iter();
            let opename: String = eles.next().unwrap().into();

            match &opename[..] {
                "ObjectProperty" => {
                    let objprp = eles.next().unwrap();
                    ObjectPropertyExpression::ObjectProperty(b.object_property(objprp.clone()))
                },
                "InverseObjectProperty" => {
                    let objprp = eles.next().unwrap();
                    ObjectPropertyExpression::InverseObjectProperty(b.object_property(objprp.clone()))
                },
                _ => {
                    panic!("Object property expression not supported.")
                }
            }
        } else if let Some(strval) = &ope.str_val {
            //Parse string value into a simple object property expression
            ObjectPropertyExpression::ObjectProperty(ObjectProperty(b.iri(strval.clone())))
        } else {
            panic!("Unparseable object property expression")
        }
    }
}

impl From<PySimpleAxiom> for Axiom<ArcStr> {
    fn from(ax: PySimpleAxiom) -> Axiom<ArcStr> {
        let b = Build::new_arc();
        //we expect a List with elements
        let mut eles = ax.elements.iter();
        //The first of which is the axiom type
        let axtype: String = eles.next().unwrap().into();

        let resax : Axiom<ArcStr> = match &axtype[..] {
            "DeclareClass" => {
                //next is going to be an IRI of the class being declared
                let clsiri: String = eles.next().unwrap().into();
                Axiom::DeclareClass(DeclareClass(Class(b.iri(clsiri.clone()))))
            },
            "DeclareObjectProperty" => {
                let objpiri: String = eles.next().unwrap().into();
                Axiom::DeclareObjectProperty(DeclareObjectProperty(ObjectProperty(b.iri(objpiri.clone()))))
            }
            "DeclareNamedIndividual" => {
                let indiri: String = eles.next().unwrap().into();
                Axiom::DeclareNamedIndividual(DeclareNamedIndividual(NamedIndividual(b.iri(indiri.clone()))))
            },
            "DeclareDatatype" => {
                let dtiri: String = eles.next().unwrap().into();
                Axiom::DeclareDatatype(DeclareDatatype(Datatype(b.iri(dtiri.clone()))))
            },
            "DeclareDataProperty" => {
                let dtiri: String = eles.next().unwrap().into();
                Axiom::DeclareDataProperty(DeclareDataProperty(DataProperty(b.iri(dtiri.clone()))))
            },
            "DeclareAnnotationProperty" => {
                let dtiri: String = eles.next().unwrap().into();
                Axiom::DeclareAnnotationProperty(DeclareAnnotationProperty(AnnotationProperty(b.iri(dtiri.clone()))))
            },
            "SubClassOf" => {
                //next is going to be an IRI for the class that is the subclass
                let subiri: String = eles.next().unwrap().into();
                let subce: ClassExpression<ArcStr> = ClassExpression::Class(Class(b.iri(subiri.clone())));
                //then either class expression for the superclass, or another list to iterate over.
                let ce: &SimpleAxiomContent = eles.next().unwrap();

                //Parse a class expression from the simple axiom content
                let supce: ClassExpression<ArcStr> = ce.into();

                //Create the Axiom
                Axiom::SubClassOf(SubClassOf{sup:supce,sub:subce})

            },
            "EquivalentClasses" => {
                //Next is going to be a set of class expressions
                let clsses: Vec<ClassExpression<ArcStr>> = eles.map(|clss| clss.into()).collect();

                //Create the Axiom
                Axiom::EquivalentClasses(EquivalentClasses(clsses))

            },
            "DisjointClasses" => {
                //Next is going to be a set of class expressions
                let clsses: Vec<ClassExpression<ArcStr>> = eles.map(|clss| clss.into()).collect();

                //Create the Axiom
                Axiom::DisjointClasses(DisjointClasses(clsses))

            }
            //TODO add other axiom types here.
            "AnnotationAssertion" => {
                let subiri: String = eles.next().unwrap().into();
                let apiri: String = eles.next().unwrap().into();
                let annstr: String = eles.next().unwrap().into();

                Axiom::AnnotationAssertion(AnnotationAssertion{
                    subject: AnnotationSubject::IRI (b.iri(subiri)), //TODO cope with anonymous individual as well?
                        ann: Annotation{ap: AnnotationProperty(b.iri(apiri))
                            ,av: AnnotationValue::Literal(Literal::Simple{literal:annstr})}})
            },
            _ => {
                panic!("Unknown axiom type {:}",axtype);
            },
        };
        //

        resax
    }
}

impl From<&Axiom<ArcStr>> for PySimpleAxiom {

    fn from(aax: &Axiom<ArcStr>) -> PySimpleAxiom {
        let mut pyax = PySimpleAxiom::default();
        pyax.elements.push(format!("{:?}",aax.kind()).into());

        match aax {
            Axiom::DeclareClass(DeclareClass(dc)) => {
                pyax.elements.push( dc.0.to_string().into() );
            },
            Axiom::DeclareObjectProperty(DeclareObjectProperty(dp)) => {
                pyax.elements.push( dp.0.to_string().into() );
            },
            Axiom::DeclareNamedIndividual(DeclareNamedIndividual(ni)) => {
                pyax.elements.push( ni.0.to_string().into() );
            },
            Axiom::SubClassOf(SubClassOf{sup,sub}) => {
                pyax.elements.push( PySimpleAxiom::from(sub).into() );
                pyax.elements.push( PySimpleAxiom::from(sup).into() );
            },
            Axiom::AnnotationAssertion(AnnotationAssertion{subject,ann:Annotation{ap,av}}) => {
                pyax.elements.push( subject.to_string().into() );
                pyax.elements.push( ap.0.to_string().into() );
                let av: String = match av {
                    AnnotationValue::Literal(lit) => lit.literal().to_string(),
                    AnnotationValue::IRI(iri) => iri.to_string(),
                };
                pyax.elements.push( av.into() );
            },
            Axiom::EquivalentClasses(EquivalentClasses(clsses)) => {
                for ele in clsses {
                    pyax.elements.push( PySimpleAxiom::from(ele).into() );
                }
            }
            _ => ()
        }

        pyax
    }
}

#[pyclass]
struct PyIndexedOntology {

    //State variables private to Rust, exposed through methods to Python
    labels_to_iris: HashMap<String,IRI<ArcStr>>,

    classes_to_subclasses: HashMap<IRI<ArcStr>,HashSet<IRI<ArcStr>>>, //axiom typed index would give subclass axioms
    classes_to_superclasses: HashMap<IRI<ArcStr>,HashSet<IRI<ArcStr>>>,

    //The primary store of the axioms is a Horned OWL indexed ontology
    ontology: ArcIRIMappedOntology,
    //Need this for converting IRIs to IDs and for saving again afterwards
    mapping: PrefixMapping,
}

impl Default for PyIndexedOntology {
    fn default() -> Self {
        PyIndexedOntology {
            labels_to_iris: Default::default(),
            classes_to_subclasses: Default::default(),
            classes_to_superclasses: Default::default(),
            ontology: ArcIRIMappedOntology::new_arc(),
            mapping: Default::default()
        }
    }
}

#[pymethods]
impl PyIndexedOntology {
    fn get_id_for_iri(&mut self, iri: String) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let res = self.mapping.shrink_iri(&iri);

        if let Ok(curie) = res {
            Ok(curie.to_string().to_object(py))
        } else {  //Return null
            Ok(().to_object(py))
        }
    }

    fn get_iri_for_id(&mut self, id: String) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let idparts: Vec<&str> = id.split(":").collect();

        if idparts.len()==2 {
            let curie = Curie::new(Some(idparts[0]), idparts[1]);

            let res = self.mapping.expand_curie(&curie);

            if let Ok(iri) = res {
                Ok(iri.to_string().to_object(py))
            } else {  //Return null
                Ok(().to_object(py))
            }
        } else { //Not a CURIE, at least not of the form PREFIX:NUMBER
            Ok(().to_object(py))
        }

    }

    fn add_prefix_mapping(&mut self, iriprefix: String, mappedid: String) -> PyResult<()> {
        let result = self.mapping.add_prefix(&iriprefix, &mappedid);
        if let Ok(()) = result {
            Ok(())
        } else {
            Err(PyValueError::new_err("Error - prefix is invalid."))
        }
    }

    fn set_label(&mut self, iri: String, label: String) -> PyResult<()> {
        let b = Build::new_arc();
        let iri = b.iri(iri);

        let ax1:AnnotatedAxiom<ArcStr> =
            Axiom::AnnotationAssertion(
                AnnotationAssertion{subject: iri.clone().into(),
                    ann: Annotation{ap: b.annotation_property(AnnotationBuiltIn::LABEL.iri_s().clone()),
                    av: AnnotationValue::Literal(
                        Literal::Simple{literal:label.clone()})}}).into();

        //If we already have a label, update it:
        let old_ax = &self.ontology.axiom_for_iri(&iri).filter_map(|aax: &AnnotatedAxiom<ArcStr>| {
            match &aax.axiom {
                Axiom::AnnotationAssertion(AnnotationAssertion{subject:_subj,ann}) => {
                        match ann {
                            Annotation {ap, av:  AnnotationValue::Literal(Literal::Simple{literal:_old}) } => {
                                if AnnotationBuiltIn::LABEL.iri_s().eq(&ap.0.to_string()) {
                                    Some(aax.clone())
                                } else {
                                    None
                                }
                            },
                            _ => None,
                        }
                    },
                    _ => None,
                }
        }).next();

        if let Some(old_ax) = old_ax {
            self.ontology.update_axiom(old_ax, ax1);
        } else {
        //If no label already, just add one
            self.ontology.insert(ax1);
        }
        Ok(())
    }

    fn get_iri_for_label(&mut self, label: String) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let iri_value = &self.labels_to_iris.get(&label);
        if let Some(iri_value) = iri_value {
            Ok(iri_value.to_string().to_object(py))
        } else {
            Ok(().to_object(py))
        }
    }

    fn get_subclasses(&mut self, iri: String) -> PyResult<HashSet<String>> {
        let b = Build::new_arc();
        let iri = b.iri(iri);

        let subclasses = self.classes_to_subclasses.get(&iri);
        if let Some(subclss) = subclasses {
            let subclasses : HashSet<String> = subclss.iter().map(|sc| sc.to_string()).collect();
            Ok(subclasses)
        } else {
            Ok(HashSet::new())
        }
    }

    fn get_superclasses(&mut self, iri: String) -> PyResult<HashSet<String>> {
        let b = Build::new_arc();
        let iri = b.iri(iri);

        let superclasses  = self.classes_to_superclasses.get(&iri);
        if let Some(superclss) = superclasses {
            let superclasses : HashSet<String> = superclss
                   .iter().map(|sc| sc.to_string()).collect();
            Ok(superclasses)
        } else {
            Ok(HashSet::new())
        }
    }

    fn get_classes(&mut self) -> PyResult<HashSet<String>> {
        //Get the DeclareClass axioms
        let classes = self.ontology.axiom_for_kind(AxiomKind::DeclareClass);

        let classes : HashSet<String> = classes
                        .filter_map(|aax| {
                            match aax.clone().axiom {
                                    Axiom::DeclareClass(dc) => {
                                        Some(dc.0.0.to_string())
                                    },
                                    _ => None
                                }
                        }).collect();
        Ok(classes)
    }

    fn get_object_properties(&mut self) -> PyResult<HashSet<String>> {
        //Get the DeclareObjectProperty axioms
        let object_properties = self.ontology.axiom_for_kind(AxiomKind::DeclareObjectProperty);

        let object_properties : HashSet<String> = object_properties
                        .filter_map(|aax| {
                            match aax.clone().axiom {
                                    Axiom::DeclareObjectProperty(dop) => {
                                        Some(dop.0.0.to_string())
                                    },
                                    _ => None
                                }
                        }).collect();
        Ok(object_properties)
    }
    
    fn get_annotation(&mut self, class_iri: String, ann_iri: String) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let annots = self.get_annotations(class_iri, ann_iri);

        let mut literal_value = ().to_object(py);

        if let Ok(literal_values) = annots {
            if !literal_values.is_empty() {
                literal_value = literal_values.into_iter().next().to_object(py);
        } }

        Ok(literal_value)
    }

    fn get_annotations(&mut self, class_iri: String, ann_iri: String) -> PyResult<Vec<String>> {
        let b = Build::new_arc();
        let iri = b.iri(class_iri);

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

    fn save_to_file(&mut self, file_name: String) -> PyResult<()>{
        let before = Instant::now();

        let mut file = File::create(file_name)?;
        let mut amo : ArcAxiomMappedOntology = AxiomMappedOntology::new_arc();
        //Copy the axioms into an AxiomMappedOntology as that is what horned owl writes
        for aax in self.ontology.iter() {
            amo.insert(aax.clone());
        }
        let time_middle = before.elapsed().as_secs();
        println!("Finished preparing ontology for saving in {:?} seconds.", time_middle);
        let before = Instant::now();

        let result = horned_owl::io::owx::writer::write(&mut file, &amo, Some(&self.mapping));

        let time_after = before.elapsed().as_secs();
        println!("Finished saving ontology to file in  {:?} seconds.", time_after);

        match result {
            Ok(()) => Ok(()),
            Err(error) => panic!("Problem saving the ontology to a file: {:?}", error),
        }
    }

    fn get_axioms_for_iri(&mut self, iri: String) -> PyResult<Vec<PyObject>> {
        let b = Build::new();
        let iri = b.iri(iri);

        let gil = Python::acquire_gil();
        let py = gil.python();

        let axioms = self.ontology.axiom_for_iri(&iri)
                                .filter_map(|aax: &AnnotatedAxiom<ArcStr>| {
                                    Some(PySimpleAxiom::from(&aax.axiom))
                                }).map(|aax: PySimpleAxiom| {aax.to_object(py)}).collect();

        Ok(axioms)
    }

    fn get_axioms(&mut self) -> PyResult<Vec<PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let axioms = self.ontology.iter()
                                .filter_map(|aax: &AnnotatedAxiom<ArcStr>| {
                                    Some(PySimpleAxiom::from(&aax.axiom))
                                }).map(|aax: PySimpleAxiom| {aax.to_object(py)}).collect();

        Ok(axioms)
    }

    fn add_axiom(&mut self, ax: Vec<PyObject>) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let els: Vec<SimpleAxiomContent> = SimpleAxiomContent::parse(ax,py);

        let ax: Axiom<ArcStr> = PySimpleAxiom{elements:els}.into();

        self.ontology.insert(ax);

        Ok(())
    }

    fn remove_axiom(&mut self, ax: Vec<PyObject>) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let els: Vec<SimpleAxiomContent> = SimpleAxiomContent::parse(ax,py);

        let ax: Axiom<ArcStr> = PySimpleAxiom{elements:els}.into();

        self.ontology.remove(&ax.into());

        Ok(())
    }
}

impl PyIndexedOntology {
    fn insert(&mut self, ax: &AnnotatedAxiom<ArcStr>) -> () {
        let b = Build::new();

        match ax.kind() {
            AxiomKind::AnnotationAssertion => {
                match ax.clone().axiom {
                    Axiom::AnnotationAssertion(AnnotationAssertion{subject,ann}) => {
                        match ann {
                            Annotation {ap, av:  AnnotationValue::Literal(Literal::Simple{literal}) } => {
                                if AnnotationBuiltIn::LABEL.iri_s().eq(&ap.0.to_string()) {
                                    &self.labels_to_iris.insert(literal.clone(),b.iri(subject.deref()));
                                }
                            },
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
            AxiomKind::SubClassOf => {
                match ax.clone().axiom {
                    Axiom::SubClassOf(SubClassOf{sup,sub}) => {
                        match sup {
                            ClassExpression::Class(c) => {
                                match sub {
                                    ClassExpression::Class(d) => { //Direct subclasses only
                                        &self.classes_to_subclasses.entry(c.0.clone()).or_insert(HashSet::new()).insert(d.0.clone());
                                        &self.classes_to_superclasses.entry(d.0.clone()).or_insert(HashSet::new()).insert(c.0.clone());
                                    },
                                    _ => ()
                                }
                            },
                        _ => ()
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
    }

    fn from(iro: IRIMappedOntology<ArcStr,Arc<AnnotatedAxiom<ArcStr>>>) -> PyIndexedOntology {
        let mut ino = PyIndexedOntology::default();

        for ax in iro.iter() {
            ino.insert(&ax);
        }

        ino.ontology = iro;

        ino
    }

}

fn open_ontology_owx(ontology: &str) -> Result<(SetOntology<ArcStr>,PrefixMapping),HornedError> {
    let b = Build::new_arc();

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

fn open_ontology_rdf(ontology: &str) ->
        Result<(RDFOntology<ArcStr, Arc<AnnotatedAxiom<ArcStr>>>,
            IncompleteParse<Arc<str>>), HornedError> {
    let b = Build::new_arc();

    let r = if Path::new(&ontology).exists() {
        let file = File::open(ontology).ok().unwrap();
        let mut f = BufReader::new(file);
        horned_owl::io::rdf::reader::read_with_build(&mut f, &b, ParserConfiguration {
            rdf: RDFParserConfiguration{lax:true},
            ..Default::default()
        })
    } else {
        //just try to parse the string
        let str_val = ontology.as_bytes();
        let mut f = BufReader::new(str_val);
        horned_owl::io::rdf::reader::read_with_build(&mut f, &b, ParserConfiguration {
            rdf: RDFParserConfiguration{lax:true},
            ..Default::default()
        })
    };
    r
}

#[pyfunction]
fn open_ontology(ontology: &PyString) -> PyResult<PyIndexedOntology> {
    let before = Instant::now();

    let ontology: String = ontology.extract().unwrap();

    let result = if ontology.ends_with("owx") {
        let r = open_ontology_owx(&ontology);
        //println!("Got result {:?}",r);
        if r.is_ok() {
            let (o,m) = r.ok().unwrap();
            //println!("Got ontology from owx {:?}",o);
            //println!("About to build indexes");
            let iro = IRIMappedOntology::from(o);
            let mut lo =  PyIndexedOntology::from(iro);
            lo.mapping = m; //Needed when saving
            Ok(lo)
        } else {
            Err(PyValueError::new_err("Unable to open ontology"))
        }
    } else if ontology.ends_with("owl"){
        let r2 = open_ontology_rdf(&ontology);
        if r2.is_ok() {
            let (o,p) = r2.ok().unwrap();
            //println!("Got ontology from rdf {:?}",o);
            let so = SetOntology::from(o);
            let iro = IRIMappedOntology::from(so);
            let mut lo =  PyIndexedOntology::from(iro);
            Ok(lo)
        } else {
            Err(PyValueError::new_err("Unable to open ontology"))
        }
    } else { // No recognised suffix, maybe it is a string value, just try to parse
        let r = open_ontology_owx(&ontology);
        if r.is_ok() {
            let (o,m) = r.ok().unwrap();
            //println!("Got ontology from owx {:?}",o);
            //println!("About to build indexes");
            let iro = IRIMappedOntology::from(o);
            let mut lo =  PyIndexedOntology::from(iro);
            lo.mapping = m; //Needed when saving
            Ok(lo)
        } else {
            let r2 = open_ontology_rdf(&ontology);
            if r2.is_ok() {
                let (o,p) = r2.ok().unwrap();
                //println!("Got ontology from rdf {:?}",o);
                let so = SetOntology::from(o);
                let iro = IRIMappedOntology::from(so);
                let mut lo =  PyIndexedOntology::from(iro);
                Ok(lo)
            } else {
                Err(PyValueError::new_err("Unable to open ontology"))
            }

        }

    };
    result
}

#[pyfunction]
fn get_descendants(onto: &PyIndexedOntology, parent: &PyString) -> PyResult<HashSet<String>> {
    let mut descendants = HashSet::new();
    let parent: String = parent.extract().unwrap();

    let b = Build::new();
    let parentiri = b.iri(parent);

    recurse_descendants(onto, &parentiri, &mut descendants);

    Ok(descendants)
}

fn recurse_descendants(onto : &PyIndexedOntology, superclass: &IRI<ArcStr>, descendants: &mut HashSet<String>) {
    descendants.insert(superclass.into());
    if onto.classes_to_subclasses.contains_key(superclass) {
        for cls2 in &mut onto.classes_to_subclasses[superclass].iter() {
            recurse_descendants(onto, cls2, descendants);
        }
    }
}

#[pyfunction]
fn get_ancestors(onto: &PyIndexedOntology, child: &PyString) -> PyResult<HashSet<String>> {
    let mut ancestors = HashSet::new();
    let child: String = child.extract().unwrap();

    let b = Build::new();
    let childiri = b.iri(child);

    recurse_ancestors(onto, &childiri, &mut ancestors);

    Ok(ancestors)
}

fn recurse_ancestors(onto : &PyIndexedOntology, subclass: &IRI<ArcStr>, ancestors: &mut HashSet<String>) {
    ancestors.insert(subclass.into());
    if onto.classes_to_superclasses.contains_key(subclass) {
        for cls2 in &mut onto.classes_to_superclasses[subclass].iter() {
            recurse_ancestors(onto, cls2, ancestors);
        }
    }
}

#[pymodule]
fn pyhornedowl(_py:Python, m:&PyModule) -> PyResult<()> {
    m.add_class::<PyIndexedOntology>()?;

    m.add_function(wrap_pyfunction!(open_ontology,m)?)?;
    m.add_function(wrap_pyfunction!(get_descendants,m)?)?;
    m.add_function(wrap_pyfunction!(get_ancestors,m)?)?;

    Ok(())
}
