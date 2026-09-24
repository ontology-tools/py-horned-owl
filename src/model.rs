use std::collections::BTreeSet;

use curie::Curie;
use horned_owl::model::{ArcStr, Build};
use pyo3::{inspect::PyStaticExpr, prelude::*, type_hint_union};

pub use crate::model_generated::*;
use crate::wrappers::BTreeSetWrap;

impl From<&BTreeSet<horned_owl::model::Annotation<ArcStr>>> for BTreeSetWrap<Annotation> {
    fn from(value: &BTreeSet<horned_owl::model::Annotation<ArcStr>>) -> Self {
        BTreeSetWrap(value.iter().map(From::from).collect())
    }
}

impl From<&BTreeSetWrap<Annotation>> for BTreeSet<horned_owl::model::Annotation<ArcStr>> {
    fn from(value: &BTreeSetWrap<Annotation>) -> Self {
        value.0.iter().map(From::from).collect()
    }
}

#[derive(Debug, Clone)]
pub enum IRIParam {
    IRI(IRI),
    StrIri(String),
    Curie(Option<String>, String),
}

impl IRIParam {
    pub fn into_iri(
        self,
        prefix_mapping: &curie::PrefixMapping,
        build: &Build<ArcStr>,
    ) -> PyResult<horned_owl::model::IRI<ArcStr>> {
        match self {
            IRIParam::IRI(iri) => Ok(iri.into()),
            IRIParam::StrIri(s) => Ok(build.iri(s.clone()).into()),
            IRIParam::Curie(prefix, reference) => {
                if let Ok(expanded) =
                    prefix_mapping.expand_curie(&Curie::new(prefix.as_deref(), reference.as_str()))
                {
                    Ok(build.iri(expanded).into())
                } else {
                    Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Cannot expand CURIE {}:{}",
                        prefix.as_deref().unwrap_or(""),
                        reference
                    )))
                }
            }
        }
    }
}

impl<'py> FromPyObject<'_, 'py> for IRIParam {
    const INPUT_TYPE: PyStaticExpr = type_hint_union!(
        <IRI as pyo3::PyTypeInfo>::TYPE_HINT,
        <(String, bool)>::INPUT_TYPE,
        String::INPUT_TYPE
    );
    type Error = PyErr;

    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(a) = ob.extract::<IRI>() {
            return Ok(IRIParam::IRI(a));
        }

        if let Ok((str, is_absolute)) = ob.extract::<(String, bool)>() {
            if is_absolute {
                return Ok(IRIParam::StrIri(str));
            } else {
                let (prefix, reference) = if let Some((first, second)) = str.split_once(':') {
                    (Some(first.to_owned()), second.to_owned())
                } else {
                    (None, str.to_owned())
                };
                return Ok(IRIParam::Curie(prefix, reference));
            }
        }

        if let Ok(s) = ob.extract::<String>() {
            if s.contains("://") {
                // looks like an IRI
                return Ok(IRIParam::StrIri(s));
            } else if let Some((first, second)) = s.split_once(':') {
                // looks like a CURIE
                return Ok(IRIParam::Curie(Some(first.to_owned()), second.to_owned()));
            } else {
                return Ok(IRIParam::Curie(None, s.to_owned()));
            }
        }

        // Fallback: wrong type
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "expected IRI instance or IRI string",
        ))
    }
}

#[pymodule(name = "model")]
pub mod py_model {
    #[pymodule_export]
    use super::{
        AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationProperty,
        AnnotationPropertyDomain, AnnotationPropertyRange, AnonymousIndividual,
        AsymmetricObjectProperty, BuiltInAtom, Class, ClassAssertion, ClassAtom, DataAllValuesFrom,
        DataComplementOf, DataExactCardinality, DataHasValue, DataIntersectionOf,
        DataMaxCardinality, DataMinCardinality, DataOneOf, DataProperty, DataPropertyAssertion,
        DataPropertyAtom, DataPropertyDomain, DataPropertyRange, DataRangeAtom, DataSomeValuesFrom,
        DataUnionOf, Datatype, DatatypeDefinition, DatatypeLiteral, DatatypeRestriction,
        DeclareAnnotationProperty, DeclareClass, DeclareDataProperty, DeclareDatatype,
        DeclareNamedIndividual, DeclareObjectProperty, DifferentIndividuals,
        DifferentIndividualsAtom, DisjointClasses, DisjointDataProperties,
        DisjointObjectProperties, DisjointUnion, DocIRI, EquivalentClasses,
        EquivalentDataProperties, EquivalentObjectProperties, Facet, FacetRestriction,
        FunctionalDataProperty, FunctionalObjectProperty, HasKey, Import,
        InverseFunctionalObjectProperty, InverseObjectProperties, InverseObjectProperty,
        IrreflexiveObjectProperty, LanguageLiteral, NamedIndividual, NegativeDataPropertyAssertion,
        NegativeObjectPropertyAssertion, ObjectAllValuesFrom, ObjectComplementOf,
        ObjectExactCardinality, ObjectHasSelf, ObjectHasValue, ObjectIntersectionOf,
        ObjectMaxCardinality, ObjectMinCardinality, ObjectOneOf, ObjectProperty,
        ObjectPropertyAssertion, ObjectPropertyAtom, ObjectPropertyDomain, ObjectPropertyRange,
        ObjectSomeValuesFrom, ObjectUnionOf, OntologyAnnotation, OntologyID,
        ReflexiveObjectProperty, Rule, SameIndividual, SameIndividualAtom, SimpleLiteral,
        SubAnnotationPropertyOf, SubClassOf, SubDataPropertyOf, SubObjectPropertyOf,
        SymmetricObjectProperty, TransitiveObjectProperty, Variable, IRI,
    };
}
