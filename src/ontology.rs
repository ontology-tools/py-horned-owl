use crate::prefix_mapping::PrefixMapping;
use crate::reasoning::DynamicLoadedReasoner;
use crate::wrappers::{LiteralStr, Serializations};
use crate::{model, parse_serialization, to_py_err};
use horned_owl::io::rdf::reader::ConcreteRDFOntology;
use horned_owl::io::InputFormat;
use horned_owl::model::{
    AnnotatedComponent, ArcAnnotatedComponent, ArcStr, Build, Component, ComponentKind, ForIRI,
    MutableOntology, Ontology, OntologyID, IRI,
};
use horned_owl::ontology::component_mapped::{
    ArcComponentMappedOntology, ComponentMappedIndex, ComponentMappedOntology,
};
use horned_owl::ontology::indexed::OntologyIndex;
use horned_owl::ontology::iri_mapped::IRIMappedIndex;
use horned_owl::ontology::set::{SetIndex, SetOntology};
use oxrdfio::RdfFormat;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::{pyclass, pymethods, Py, PyResult, Python};
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::iter::FusedIterator;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

macro_rules! into_iri {
    ($s:ident, $py:ident, $iri:ident) => {
        $iri.into_iri(&$s.mapping.borrow($py).0, &$s.build.read().unwrap())?
    };
}

mod annotations;
mod iri;
mod modify;
mod query;

#[pyclass(from_py_object)]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Copy)]
/// Values to indicate when to build the additional indexes.
///
/// :cvar OnLoad: Create the additional indexes when the ontology is loaded
/// :cvar OnQuery: Create the additional indexes only when they are needed
/// :cvar Explicit: Only create the additional indexes when explicity requested
#[derive(Default)]
pub enum IndexCreationStrategy {
    /// Create the additional indexes when the ontology is loaded
    OnLoad,

    /// Create the additional indexes only when they are needed
    #[default]
    OnQuery,

    /// Only create the additional indexes when explicity requested
    Explicit,
}

/// Represents a loaded ontology.
#[pyclass(name = "Ontology", module = "pyhornedowl", from_py_object)]
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
    pub build: RwLock<Build<ArcStr>>,

    pub index_strategy: IndexCreationStrategy,

    pub reasoners: Vec<crate::reasoning::PyReasoner>,
}

unsafe impl Sync for PyIndexedOntology {}
unsafe impl Send for PyIndexedOntology {}

impl Default for PyIndexedOntology {
    fn default() -> Self {
        Python::attach(|py| PyIndexedOntology {
            labels_to_iris: Default::default(),
            classes_to_subclasses: Default::default(),
            classes_to_superclasses: Default::default(),
            iri_index: None,
            component_index: None,
            set_index: Default::default(),
            mapping: Py::new(py, PrefixMapping::default())
                .expect("Unable to create default prefix mapping"),
            build: RwLock::new(Build::new_arc()),
            index_strategy: IndexCreationStrategy::OnQuery,
            reasoners: vec![],
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

impl Ontology<ArcStr> for PyIndexedOntology {
    type ComponentIter<'c> = PyOntIter<'c>;

    fn iter(&self) -> Self::ComponentIter<'_> {
        PyIndexedOntology::iter(self)
    }
}

impl MutableOntology<ArcStr> for PyIndexedOntology {
    fn insert<AA>(&mut self, ax: AA) -> bool
    where
        AA: Into<AnnotatedComponent<ArcStr>>,
    {
        let ax: ArcAnnotatedComponent = ax.into().into();
        if self.set_index.contains(&ax) {
            return false;
        }
        if let Some(ref mut iri_index) = &mut self.iri_index {
            iri_index.index_insert(ax.clone());
        }
        if let Some(ref mut component_index) = &mut self.component_index {
            component_index.index_insert(ax.clone());
        }
        for reasoner in &mut self.reasoners {
            reasoner.0.lock().unwrap().0.index_insert(ax.clone());
        }

        self.set_index.index_insert(ax)
    }

    fn take(&mut self, ax: &AnnotatedComponent<ArcStr>) -> Option<AnnotatedComponent<ArcStr>> {
        if !self.set_index.contains(ax) {
            return None;
        }
        if let Some(ref mut iri_index) = &mut self.iri_index {
            iri_index.index_take(ax);
        }
        if let Some(ref mut component_index) = &mut self.component_index {
            component_index.index_take(ax);
        }
        for reasoner in &self.reasoners {
            reasoner.0.lock().unwrap().0.index_take(ax);
        }

        self.set_index.index_take(ax)
    }

    fn remove(&mut self, ax: &AnnotatedComponent<ArcStr>) -> bool {
        if !self.set_index.contains(ax) {
            return false;
        }
        if let Some(ref mut iri_index) = &mut self.iri_index {
            iri_index.index_remove(ax);
        }
        if let Some(ref mut component_index) = &mut self.component_index {
            component_index.index_remove(ax);
        }
        for reasoner in &mut self.reasoners {
            reasoner.0.lock().unwrap().0.index_remove(ax);
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

    /// Returns the ontology iri, if it exists.
    pub fn get_iri(&mut self) -> PyResult<Option<model::IRI>> {
        Ok(self
            .get_id()
            .and_then(|x| x.iri.as_ref())
            .map(model::IRI::from))
    }

    /// Returns the ontologys version iri, if it exists.
    pub fn get_version_iri(&mut self) -> PyResult<Option<model::IRI>> {
        Ok(self
            .get_id()
            .and_then(|x| x.viri.as_ref())
            .map(model::IRI::from))
    }

    /// Saves the ontology to a UTF8 string.
    pub fn save_to_string(
        &mut self,
        py: Python<'_>,
        serialization: LiteralStr<Serializations>,
    ) -> PyResult<String> {
        let serialization = parse_serialization(&serialization)?;

        let mut writer = Vec::<u8>::new();

        self.save_to_buf(py, &mut writer, serialization)?;

        String::from_utf8(writer).map_err(to_py_err!("Failed to save ontology to UTF-8"))
    }

    /// Saves the ontology to disk. If no serialization is given it is guessed by the file extension.
    /// Defaults to OWL/XML
    #[pyo3(signature = (file_name, serialization = None))]
    pub fn save_to_file(
        &mut self,
        py: Python<'_>,
        file_name: String,
        serialization: Option<LiteralStr<Serializations>>,
    ) -> PyResult<()> {
        let serialization = serialization
            .as_deref()
            .map(parse_serialization)
            .or_else(|| {
                Path::new(&file_name)
                    .extension()
                    .and_then(OsStr::to_str)
                    .map(parse_serialization)
            })
            .transpose()?
            .unwrap_or(InputFormat::Rdf(Some(oxrdfio::RdfFormat::RdfXml)));
        let mut file = File::create(file_name)?;

        self.save_to_buf(py, &mut file, serialization)
    }

    /// Creates a new IRI from string.
    ///
    /// Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
    #[pyo3(signature = (iri))]
    pub fn iri(&self, py: Python<'_>, iri: model::IRIParam) -> PyResult<model::IRI> {
        Ok(into_iri!(self, py, iri).into())
    }

    /// Creates a new IRI from CURIE string.
    ///
    /// Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
    pub fn curie(&self, py: Python<'_>, curie: String) -> PyResult<model::IRI> {
        let mapping = self.mapping.borrow_mut(py);
        let build = self
            .build
            .read()
            .map_err(to_py_err!("Failed get build instance!"))?;
        let iri = mapping
            .0
            .expand_curie_string(&curie)
            .map_err(to_py_err!("Invalid curie"))?;
        Ok(model::IRI::new(iri, &build))
    }

    /// Convenience method to create a Class from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    ///
    /// .. deprecated::
    ///     Use `Ontology.class_` instead
    #[pyo3(signature = (iri))]
    pub fn clazz(&self, py: Python<'_>, iri: model::IRIParam) -> PyResult<model::Class> {
        PyErr::warn(
            py,
            &py.get_type::<pyo3::exceptions::PyDeprecationWarning>(),
            c"`clazz` is deprecated, use a `class_` instead.",
            1,
        )?;
        self.class_(py, iri)
    }

    /// Convenience method to create a Class from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3(signature = (iri))]
    pub fn class_(&self, py: Python<'_>, iri: model::IRIParam) -> PyResult<model::Class> {
        Ok(model::Class {
            iri: into_iri!(self, py, iri).into(),
        })
    }

    /// Convenience method to create an ObjectProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3(signature = (iri))]
    pub fn object_property(
        &self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<model::ObjectProperty> {
        Ok(model::ObjectProperty {
            iri: into_iri!(self, py, iri).into(),
        })
    }

    /// Convenience method to create a DataProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3(signature = (iri))]
    pub fn data_property(
        &self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<model::DataProperty> {
        Ok(model::DataProperty {
            iri: into_iri!(self, py, iri).into(),
        })
    }

    /// Convenience method to create a datatype from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    pub fn datatype(&self, py: Python<'_>, iri: model::IRIParam) -> PyResult<model::Datatype> {
        Ok(model::Datatype {
            iri: into_iri!(self, py, iri).into(),
        })
    }

    /// Convenience method to create an annotationProperty from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3(signature = (iri))]
    pub fn annotation_property(
        &self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<model::AnnotationProperty> {
        Ok(model::AnnotationProperty {
            iri: into_iri!(self, py, iri).into(),
        })
    }

    /// Convenience method to create a NamedIndividual from an IRI.
    ///
    /// Uses the `iri` method to cache native IRI instances.
    #[pyo3(signature = (iri))]
    pub fn named_individual(
        &self,
        py: Python<'_>,
        iri: model::IRIParam,
    ) -> PyResult<model::NamedIndividual> {
        Ok(model::NamedIndividual {
            iri: into_iri!(self, py, iri).into(),
        })
    }

    /// Convenience method to create an AnonymousIndividual from a string.
    pub fn anonymous_individual(&self, name: String) -> model::AnonymousIndividual {
        model::AnonymousIndividual { name: name.into() }
    }

    /// Builds an index by iri (IRIMappedIndex).
    pub fn build_iri_index(&mut self) {
        if self.iri_index.is_some() {
            return;
        }

        let mut iri_index = IRIMappedIndex::<ArcStr, ArcAnnotatedComponent>::new();

        for c in self.set_index.iter() {
            iri_index.index_insert(c.clone());
        }

        self.iri_index = Some(iri_index);
    }

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
    fn get_component_index(
        &mut self,
    ) -> Option<&ComponentMappedIndex<ArcStr, ArcAnnotatedComponent>> {
        if self.component_index.is_none() && self.index_strategy == IndexCreationStrategy::OnQuery {
            self.build_component_index();
        }

        self.component_index.as_ref()
    }

    fn get_iri_index(&mut self) -> Option<&IRIMappedIndex<ArcStr, ArcAnnotatedComponent>> {
        if self.iri_index.is_none() && self.index_strategy == IndexCreationStrategy::OnQuery {
            self.build_iri_index();
        }

        self.iri_index.as_ref()
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
        serialization: InputFormat,
    ) -> PyResult<()> {
        let mut file = w;
        let mut amo: ArcComponentMappedOntology = ComponentMappedOntology::new_arc();

        //Copy the components into an ComponentMappedOntology as that is what horned owl writes
        for component in (&self.set_index).into_iter() {
            amo.insert(component.clone());
        }

        let mapping = self.mapping.borrow(py);

        let result = match serialization {
            InputFormat::OFN => {
                horned_owl::io::ofn::writer::write(&mut file, &amo, Some(&mapping.0))
            }
            InputFormat::OWX => {
                horned_owl::io::owx::writer::write(&mut file, &amo, Some(&mapping.0))
            }
            InputFormat::Rdf(f) => horned_owl::io::rdf::writer::write_to_rdf_format(
                &mut file,
                &amo,
                f.unwrap_or(RdfFormat::RdfXml).file_extension(),
            ),
            InputFormat::OMN => {
                horned_owl::io::omn::writer::write(&mut file, &amo, Some(&mapping.0))
            }
            InputFormat::OBO => {
                horned_owl::io::obo::writer::write(&mut file, &amo, Some(&mapping.0))
            }
            InputFormat::Guess => {
                return Err(PyValueError::new_err(
                    "The serialization cannot be guessed when writing",
                ))
            }
        };

        result
            .map(|_| ())
            .map_err(to_py_err!("Problem saving the ontology to a file"))
    }

    pub fn add_reasoner(
        &mut self,
        reasoner: DynamicLoadedReasoner,
    ) -> crate::reasoning::PyReasoner {
        let r = crate::reasoning::PyReasoner(Arc::new(Mutex::new(reasoner)));
        self.reasoners.push(r.clone());
        r
    }

    pub fn iter(&self) -> PyOntIter<'_> {
        PyOntIter(Box::new(self.set_index.iter().map(|m| m.as_ref())))
    }
}

pub struct PyOntIter<'a>(Box<dyn Iterator<Item = &'a AnnotatedComponent<ArcStr>> + 'a>);

impl<'a> Iterator for PyOntIter<'a> {
    type Item = &'a AnnotatedComponent<ArcStr>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl FusedIterator for PyOntIter<'_> {}

impl<'a> IntoIterator for &'a PyIndexedOntology {
    type Item = &'a AnnotatedComponent<ArcStr>;
    type IntoIter = PyOntIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct PyOntIntoIter(
    std::iter::Map<
        std::collections::hash_set::IntoIter<Arc<AnnotatedComponent<ArcStr>>>,
        fn(Arc<AnnotatedComponent<ArcStr>>) -> AnnotatedComponent<ArcStr>,
    >,
);

impl Iterator for PyOntIntoIter {
    type Item = AnnotatedComponent<ArcStr>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl FusedIterator for PyOntIntoIter {}

impl ExactSizeIterator for PyOntIntoIter {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for PyIndexedOntology {
    type Item = AnnotatedComponent<ArcStr>;
    type IntoIter = PyOntIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        PyOntIntoIter(self.set_index.into_iter())
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
