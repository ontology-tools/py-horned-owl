/// For a reasoner to be loadable by py-horned-owl:
/// 1. The crate must be a cdynlib
/// 2. The crate must expose a function `#[no_mangle] pub fn create_reasoner() -> Box<dyn Reasoner>` which creates an instance of the reasoner.
/// 3. The reasoner must implement the `Reasoner` trait. All methods are optional so the implementation can be based on the reasoners capabilities.
use horned_owl::model::{
    ArcAnnotatedComponent, ArcStr, ClassExpression, Component, ForIRI, ObjectPropertyExpression,
};
use horned_owl::ontology::indexed::{ForIndex, OntologyIndex};
use horned_owl::ontology::set::SetOntology;
use std::collections::HashSet;
use std::fmt::Debug;

#[macro_export]
macro_rules! export_py_reasoner {
    ($reasoner:ty) => {
        #[unsafe(no_mangle)]
        pub fn create_reasoner(
            ontology: SetOntology<ArcStr>,
        ) -> Box<dyn Reasoner<ArcStr, ArcAnnotatedComponent>> {
            Box::new(<$reasoner as PyReasoner>::create_reasoner(ontology))
        }
    };
}

#[derive(Debug, Default)]
pub enum ReasonerError {
    #[default]
    NotImplemented,
    Horned(horned_owl::error::HornedError),
    Other(String),
}

impl From<horned_owl::error::HornedError> for ReasonerError {
    fn from(value: horned_owl::error::HornedError) -> Self {
        ReasonerError::Horned(value)
    }
}

pub trait PyReasoner: Reasoner<ArcStr, ArcAnnotatedComponent> {
    fn create_reasoner(ontology: SetOntology<ArcStr>) -> Self;
}

pub trait Reasoner<A: ForIRI, AA: ForIndex<A>>: OntologyIndex<A, AA> {
    fn get_name(&self) -> String;

    fn get_version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    /// Applies the pending changes to the reasoner.
    fn flush(&mut self) -> Result<(), ReasonerError>;

    fn inferred_axioms(&self) -> Box<dyn Iterator<Item = Component<A>>>;

    fn is_consistent(&self) -> Result<bool, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn is_entailed(&self, _cmp: &Component<A>) -> Result<bool, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn is_satifisable(&self, _cmp: &ClassExpression<A>) -> Result<bool, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_unsatisfiable_classes(&self) -> Result<HashSet<ClassExpression<A>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_subclasses<'a>(
        &'a self,
        _cmp: &'a ClassExpression<A>,
    ) -> Result<Box<dyn Iterator<Item = ClassExpression<ArcStr>> + 'a>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_superclasses(
        &self,
        _cmp: &ClassExpression<A>,
    ) -> Result<HashSet<ClassExpression<A>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_equivalent_classes(
        &self,
        _cmp: &ClassExpression<A>,
    ) -> Result<HashSet<ClassExpression<A>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_disjoint_classes(
        &self,
        _cmp: &ClassExpression<A>,
    ) -> Result<HashSet<ClassExpression<A>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_subobjectproperties(
        &self,
        _cmp: &ObjectPropertyExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_superobjectproperties(
        &self,
        _cmp: &ObjectPropertyExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_equivalent_objectproperties(
        &self,
        _cmp: &ObjectPropertyExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_disjoint_objectproperties(
        &self,
        _cmp: &ObjectPropertyExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_inverse_objectproperties(
        &self,
        _cmp: &ObjectPropertyExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }
}
