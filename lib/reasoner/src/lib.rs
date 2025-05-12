/// For a reasoner to be loadable by py-horned-owl:
/// 1. The crate must be a cdynlib
/// 2. The crate must expose a function `#[no_mangle] pub fn create_reasoner() -> Box<dyn Reasoner>` which creates an instance of the reasoner.
/// 3. The reasoner must implement the `Reasoner` trait. All methods are optional so the implementation can be based on the reasoners capabilities.

use horned_owl::model::{ArcStr, Component};
use horned_owl::ontology::set::SetOntology;
use std::collections::HashSet;
use std::fmt::{Debug};

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

