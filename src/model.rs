use std::collections::BTreeSet;

use horned_owl::model::ArcStr;
use pyo3::{prelude::*, types::IntoPyDict};

pub use crate::model_generated::*;
use crate::wrappers::BTreeSetWrap;

macro_rules! add_type_alias {
    ($py:ident, $module:ident, $($name:ident),*) => {
        {
            let locals = [("typing", &$py.import_bound("typing")?), ("m", &$module)].into_py_dict_bound($py);

            let mut code: String;
            let mut ta: Bound<'_, PyAny>;

            $(
                code = $name::py_def();
                ta = $py.eval_bound(&code, None, Some(&locals))?;
                locals.set_item(stringify!($name), &ta)?;
                ta.setattr("__doc__",  doc!($name))?;
                $module.add(stringify!($name), &ta)?;
            )*
        }
    };
}


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

impl<'source> FromPyObject<'source> for BTreeSetWrap<Annotation> {
    fn extract(ob: &'source pyo3::PyAny) -> pyo3::PyResult<Self> {
        ob.extract::<BTreeSet<Annotation>>()
            .map(BTreeSetWrap::<Annotation>)
    }
}

impl IntoPy<pyo3::PyObject> for BTreeSetWrap<Annotation> {
    fn into_py(self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.0.into_py(py)
    }
}

pub fn py_module(py: Python<'_>) -> PyResult<Bound<PyModule>> {
    let module = PyModule::new_bound(py, "model")?;

    // To get all members to export on the documentation website for horned_ows::model execute the following javascript command
    // console.log([...(await Promise.all(Array.from(document.querySelectorAll("a.enum")).filter(x => ["ClassExpression", "ObjectPropertyExpression", "Literal", "DataRange", ""].indexOf(x.innerText) >= 0).map(async a => { html = await(await fetch(a.href)).text(); doc = document.createElement("html"); doc.innerHTML=html; return Array.from(doc.querySelectorAll(".variant")).map(x => x.id.replace("variant.", "")); }))).flatMap(arr => arr.map(x => `module.add_class::<${ x }>()?;`)), ...Array.from(document.querySelectorAll("a.struct")).map(x=>x.innerText).filter(x => ["Build", "OntologyID"].indexOf(x) < 0).map(x => `module.add_class::<${ x }>()?;`)].join("\n"))
    module.add_class::<Class>()?;
    module.add_class::<ObjectIntersectionOf>()?;
    module.add_class::<ObjectUnionOf>()?;
    module.add_class::<ObjectComplementOf>()?;
    module.add_class::<ObjectOneOf>()?;
    module.add_class::<ObjectSomeValuesFrom>()?;
    module.add_class::<ObjectAllValuesFrom>()?;
    module.add_class::<ObjectHasValue>()?;
    module.add_class::<ObjectHasSelf>()?;
    module.add_class::<ObjectMinCardinality>()?;
    module.add_class::<ObjectMaxCardinality>()?;
    module.add_class::<ObjectExactCardinality>()?;
    module.add_class::<DataSomeValuesFrom>()?;
    module.add_class::<DataAllValuesFrom>()?;
    module.add_class::<DataHasValue>()?;
    module.add_class::<DataMinCardinality>()?;
    module.add_class::<DataMaxCardinality>()?;
    module.add_class::<DataExactCardinality>()?;
    module.add_class::<Datatype>()?;
    module.add_class::<DataIntersectionOf>()?;
    module.add_class::<DataUnionOf>()?;
    module.add_class::<DataComplementOf>()?;
    module.add_class::<DataOneOf>()?;
    module.add_class::<DatatypeRestriction>()?;
    module.add_class::<SimpleLiteral>()?;
    module.add_class::<LanguageLiteral>()?;
    module.add_class::<DatatypeLiteral>()?;
    module.add_class::<ObjectProperty>()?;
    module.add_class::<InverseObjectProperty>()?;
    module.add_class::<AnnotatedComponent>()?;
    module.add_class::<Annotation>()?;
    module.add_class::<AnnotationAssertion>()?;
    module.add_class::<AnnotationProperty>()?;
    module.add_class::<AnnotationPropertyDomain>()?;
    module.add_class::<AnnotationPropertyRange>()?;
    module.add_class::<AnonymousIndividual>()?;
    module.add_class::<AsymmetricObjectProperty>()?;
    module.add_class::<Class>()?;
    module.add_class::<ClassAssertion>()?;
    module.add_class::<DataProperty>()?;
    module.add_class::<DataPropertyAssertion>()?;
    module.add_class::<DataPropertyDomain>()?;
    module.add_class::<DataPropertyRange>()?;
    module.add_class::<Datatype>()?;
    module.add_class::<DatatypeDefinition>()?;
    module.add_class::<DeclareAnnotationProperty>()?;
    module.add_class::<DeclareClass>()?;
    module.add_class::<DeclareDataProperty>()?;
    module.add_class::<DeclareDatatype>()?;
    module.add_class::<DeclareNamedIndividual>()?;
    module.add_class::<DeclareObjectProperty>()?;
    module.add_class::<DifferentIndividuals>()?;
    module.add_class::<DisjointClasses>()?;
    module.add_class::<DisjointDataProperties>()?;
    module.add_class::<DisjointObjectProperties>()?;
    module.add_class::<DisjointUnion>()?;
    module.add_class::<EquivalentClasses>()?;
    module.add_class::<EquivalentDataProperties>()?;
    module.add_class::<EquivalentObjectProperties>()?;
    module.add_class::<FacetRestriction>()?;
    module.add_class::<FunctionalDataProperty>()?;
    module.add_class::<FunctionalObjectProperty>()?;
    module.add_class::<HasKey>()?;
    module.add_class::<IRI>()?;
    module.add_class::<Import>()?;
    module.add_class::<InverseFunctionalObjectProperty>()?;
    module.add_class::<InverseObjectProperties>()?;
    module.add_class::<IrreflexiveObjectProperty>()?;
    module.add_class::<NamedIndividual>()?;
    module.add_class::<NegativeDataPropertyAssertion>()?;
    module.add_class::<NegativeObjectPropertyAssertion>()?;
    module.add_class::<ObjectProperty>()?;
    module.add_class::<ObjectPropertyAssertion>()?;
    module.add_class::<ObjectPropertyDomain>()?;
    module.add_class::<ObjectPropertyRange>()?;
    module.add_class::<OntologyAnnotation>()?;
    module.add_class::<ReflexiveObjectProperty>()?;
    module.add_class::<SameIndividual>()?;
    module.add_class::<SubAnnotationPropertyOf>()?;
    module.add_class::<SubClassOf>()?;
    module.add_class::<SubDataPropertyOf>()?;
    module.add_class::<SubObjectPropertyOf>()?;
    module.add_class::<SymmetricObjectProperty>()?;
    module.add_class::<TransitiveObjectProperty>()?;
    module.add_class::<OntologyID>()?;
    module.add_class::<DocIRI>()?;
    module.add_class::<Rule>()?;
    module.add_class::<Variable>()?;
    module.add_class::<BuiltInAtom>()?;
    module.add_class::<ClassAtom>()?;
    module.add_class::<DataPropertyAtom>()?;
    module.add_class::<DataRangeAtom>()?;
    module.add_class::<DifferentIndividualsAtom>()?;
    module.add_class::<ObjectPropertyAtom>()?;
    module.add_class::<SameIndividualAtom>()?;

    module.add_class::<Facet>()?;

    add_type_alias!(py, module,
        ClassExpression,
        ObjectPropertyExpression,
        SubObjectPropertyExpression,
        Literal,
        DataRange,
        Individual,
        PropertyExpression,
        AnnotationSubject,
        AnnotationValue,
        Component,
        Atom,
        IArgument,
        DArgument
    );

    Ok(module)
}


