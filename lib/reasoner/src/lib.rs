/// For a reasoner to be loadable by py-horned-owl:
/// 1. The crate must be a cdynlib
/// 2. The crate must expose a function `#[no_mangle] pub fn create_reasoner() -> Box<dyn Reasoner>` which creates an instance of the reasoner.
/// 3. The reasoner must implement the `Reasoner` trait. All methods are optional so the implementation can be based on the reasoners capabilities.
use horned_owl::model::{AnnotatedComponent, ArcStr, ClassExpression, Component};
use horned_owl::ontology::indexed::OntologyIndex;
use horned_owl::ontology::set::SetOntology;
use std::collections::HashSet;
use std::fmt::Debug;


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

pub trait Reasoner {
    fn classify(
        &self,
        _ontology: &SetOntology<ArcStr>,
    ) -> Result<SetOntology<ArcStr>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn infer(
        &self,
        _ontology: &SetOntology<ArcStr>,
    ) -> Result<HashSet<Component<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn consistency(&self, _ontology: &SetOntology<ArcStr>) -> Result<bool, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn destroy(&self) {}
}

pub trait Reasoner2: OntologyIndex<ArcStr, AnnotatedComponent<ArcStr>> {
    fn get_name(&self) -> String;
    fn get_version(&self) -> String;
    fn get_ontology(&self) -> SetOntology<ArcStr>;

    fn is_consistent(&self) -> Result<bool, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn is_entailed(&self, _cmp: &Component<ArcStr>) -> Result<bool, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn is_satifisable(&self, _cmp: &ClassExpression<ArcStr>) -> Result<bool, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_unsatisfiable_classes(&self) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_subclasses(
        &self,
        _cmp: &ClassExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_superclasses(
        &self,
        _cmp: &ClassExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_equivalent_classes(
        &self,
        _cmp: &ClassExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    fn get_disjoint_classes(
        &self,
        _cmp: &ClassExpression<ArcStr>,
    ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
        Err(ReasonerError::NotImplemented)
    }

    // fn get_subobjectproperties(
    //     &self,
    //     _cmp: &ObjectPropertyExpression<ArcStr>,
    // ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
    //     Err(ReasonerError::NotImplemented)
    // }
    //
    //
    // fn get_superobjectproperties(
    //     &self,
    //     _cmp: &ObjectPropertyExpression<ArcStr>,
    // ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
    //     Err(ReasonerError::NotImplemented)
    // }
    //
    // fn get_equivalent_objectproperties(
    //     &self,
    //     _cmp: &ObjectPropertyExpression<ArcStr>,
    // ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
    //     Err(ReasonerError::NotImplemented)
    // }
    //
    // fn get_disjoint_objectproperties(
    //     &self,
    //     _cmp: &ObjectPropertyExpression<ArcStr>,
    // ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
    //     Err(ReasonerError::NotImplemented)
    // }
    //
    // fn get_inverse_objectproperties(
    //     &self,
    //     _cmp: &ObjectPropertyExpression<ArcStr>,
    // ) -> Result<HashSet<ClassExpression<ArcStr>>, ReasonerError> {
    //     Err(ReasonerError::NotImplemented)
    // }
}
