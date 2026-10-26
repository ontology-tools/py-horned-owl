//! The components of an ontology: its axioms, its metadata and SWRL rules.

use horned_owl::model::ArcStr;
use py_horned_owl_macros::{model, model_enum};
use pyo3::prelude::*;

use crate::wrappers::{BTreeSetWrap, LiteralStr, SnippetSyntaxes, VecWrap};

use super::{
    Annotation, AnnotationProperty, AnnotationSubject, Atom, Class, ClassExpression, DataProperty,
    DataRange, Datatype, Individual, Literal, NamedIndividual, ObjectProperty,
    ObjectPropertyExpression, PropertyExpression, SubObjectPropertyExpression, IRI,
};

#[model_enum(wraps)]
pub enum Component {
    OntologyID(OntologyID),
    DocIRI(DocIRI),
    OntologyAnnotation(OntologyAnnotation),
    Import(Import),
    DeclareClass(DeclareClass),
    DeclareObjectProperty(DeclareObjectProperty),
    DeclareAnnotationProperty(DeclareAnnotationProperty),
    DeclareDataProperty(DeclareDataProperty),
    DeclareNamedIndividual(DeclareNamedIndividual),
    DeclareDatatype(DeclareDatatype),
    SubClassOf(SubClassOf),
    EquivalentClasses(EquivalentClasses),
    DisjointClasses(DisjointClasses),
    DisjointUnion(DisjointUnion),
    SubObjectPropertyOf(SubObjectPropertyOf),
    EquivalentObjectProperties(EquivalentObjectProperties),
    DisjointObjectProperties(DisjointObjectProperties),
    InverseObjectProperties(InverseObjectProperties),
    ObjectPropertyDomain(ObjectPropertyDomain),
    ObjectPropertyRange(ObjectPropertyRange),
    FunctionalObjectProperty(FunctionalObjectProperty),
    InverseFunctionalObjectProperty(InverseFunctionalObjectProperty),
    ReflexiveObjectProperty(ReflexiveObjectProperty),
    IrreflexiveObjectProperty(IrreflexiveObjectProperty),
    SymmetricObjectProperty(SymmetricObjectProperty),
    AsymmetricObjectProperty(AsymmetricObjectProperty),
    TransitiveObjectProperty(TransitiveObjectProperty),
    SubDataPropertyOf(SubDataPropertyOf),
    EquivalentDataProperties(EquivalentDataProperties),
    DisjointDataProperties(DisjointDataProperties),
    DataPropertyDomain(DataPropertyDomain),
    DataPropertyRange(DataPropertyRange),
    FunctionalDataProperty(FunctionalDataProperty),
    DatatypeDefinition(DatatypeDefinition),
    HasKey(HasKey),
    SameIndividual(SameIndividual),
    DifferentIndividuals(DifferentIndividuals),
    ClassAssertion(ClassAssertion),
    ObjectPropertyAssertion(ObjectPropertyAssertion),
    NegativeObjectPropertyAssertion(NegativeObjectPropertyAssertion),
    DataPropertyAssertion(DataPropertyAssertion),
    NegativeDataPropertyAssertion(NegativeDataPropertyAssertion),
    AnnotationAssertion(AnnotationAssertion),
    SubAnnotationPropertyOf(SubAnnotationPropertyOf),
    AnnotationPropertyDomain(AnnotationPropertyDomain),
    AnnotationPropertyRange(AnnotationPropertyRange),
    Rule(Rule),
}

#[model(extends = ComponentBase)]
pub struct OntologyID {
    #[model(default = None)]
    pub iri: Option<IRI>,
    #[model(horned = viri, default = None)]
    pub version_iri: Option<IRI>,
}

#[model(extends = ComponentBase)]
pub struct DocIRI {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model(extends = ComponentBase)]
pub struct OntologyAnnotation {
    #[model(horned = 0)]
    pub annotation: Annotation,
}

#[model(extends = ComponentBase)]
pub struct Import {
    #[model(horned = 0)]
    pub iri: IRI,
}

#[model(extends = ComponentBase)]
pub struct DeclareClass {
    #[model(horned = 0)]
    pub entity: Class,
}

#[model(extends = ComponentBase)]
pub struct DeclareObjectProperty {
    #[model(horned = 0)]
    pub entity: ObjectProperty,
}

#[model(extends = ComponentBase)]
pub struct DeclareAnnotationProperty {
    #[model(horned = 0)]
    pub entity: AnnotationProperty,
}

#[model(extends = ComponentBase)]
pub struct DeclareDataProperty {
    #[model(horned = 0)]
    pub entity: DataProperty,
}

#[model(extends = ComponentBase)]
pub struct DeclareNamedIndividual {
    #[model(horned = 0)]
    pub entity: NamedIndividual,
}

#[model(extends = ComponentBase)]
pub struct DeclareDatatype {
    #[model(horned = 0)]
    pub entity: Datatype,
}

#[model(extends = ComponentBase)]
pub struct SubClassOf {
    #[model(horned = sub)]
    pub sub_class: ClassExpression,
    #[model(horned = sup)]
    pub super_class: ClassExpression,
}

#[model(extends = ComponentBase)]
pub struct EquivalentClasses {
    #[model(horned = 0)]
    pub class_expressions: VecWrap<ClassExpression>,
}

#[model(extends = ComponentBase)]
pub struct DisjointClasses {
    #[model(horned = 0)]
    pub class_expressions: VecWrap<ClassExpression>,
}

#[model(extends = ComponentBase)]
pub struct DisjointUnion {
    #[model(horned = 0)]
    pub class_: Class,
    #[model(horned = 1)]
    pub class_expressions: VecWrap<ClassExpression>,
}

#[model(extends = ComponentBase)]
pub struct SubObjectPropertyOf {
    #[model(horned = sub)]
    pub sub_property: SubObjectPropertyExpression,
    #[model(horned = sup)]
    pub super_property: ObjectPropertyExpression,
}

#[model(extends = ComponentBase)]
pub struct EquivalentObjectProperties {
    #[model(horned = 0)]
    pub object_property_expressions: VecWrap<ObjectPropertyExpression>,
}

#[model(extends = ComponentBase)]
pub struct DisjointObjectProperties {
    #[model(horned = 0)]
    pub object_property_expressions: VecWrap<ObjectPropertyExpression>,
}

#[model(extends = ComponentBase)]
pub struct InverseObjectProperties {
    #[model(horned = 0)]
    pub first: ObjectProperty,
    #[model(horned = 1)]
    pub second: ObjectProperty,
}

#[model(extends = ComponentBase)]
pub struct ObjectPropertyDomain {
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = ce)]
    pub class_expression: ClassExpression,
}

#[model(extends = ComponentBase)]
pub struct ObjectPropertyRange {
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = ce)]
    pub class_expression: ClassExpression,
}

#[model(extends = ComponentBase)]
pub struct FunctionalObjectProperty {
    #[model(horned = 0)]
    pub object_property_expression: ObjectPropertyExpression,
}

#[model(extends = ComponentBase)]
pub struct InverseFunctionalObjectProperty {
    #[model(horned = 0)]
    pub object_property_expression: ObjectPropertyExpression,
}

#[model(extends = ComponentBase)]
pub struct ReflexiveObjectProperty {
    #[model(horned = 0)]
    pub object_property_expression: ObjectPropertyExpression,
}

#[model(extends = ComponentBase)]
pub struct IrreflexiveObjectProperty {
    #[model(horned = 0)]
    pub object_property_expression: ObjectPropertyExpression,
}

#[model(extends = ComponentBase)]
pub struct SymmetricObjectProperty {
    #[model(horned = 0)]
    pub object_property_expression: ObjectPropertyExpression,
}

#[model(extends = ComponentBase)]
pub struct AsymmetricObjectProperty {
    #[model(horned = 0)]
    pub object_property_expression: ObjectPropertyExpression,
}

#[model(extends = ComponentBase)]
pub struct TransitiveObjectProperty {
    #[model(horned = 0)]
    pub object_property_expression: ObjectPropertyExpression,
}

#[model(extends = ComponentBase)]
pub struct SubDataPropertyOf {
    #[model(horned = sub)]
    pub sub_property: DataProperty,
    #[model(horned = sup)]
    pub super_property: DataProperty,
}

#[model(extends = ComponentBase)]
pub struct EquivalentDataProperties {
    #[model(horned = 0)]
    pub data_properties: VecWrap<DataProperty>,
}

#[model(extends = ComponentBase)]
pub struct DisjointDataProperties {
    #[model(horned = 0)]
    pub data_properties: VecWrap<DataProperty>,
}

#[model(extends = ComponentBase)]
pub struct DataPropertyDomain {
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = ce)]
    pub class_expression: ClassExpression,
}

#[model(extends = ComponentBase)]
pub struct DataPropertyRange {
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = dr)]
    pub data_range: DataRange,
}

#[model(extends = ComponentBase)]
pub struct FunctionalDataProperty {
    #[model(horned = 0)]
    pub data_property: DataProperty,
}

#[model(extends = ComponentBase)]
pub struct DatatypeDefinition {
    #[model(horned = kind)]
    pub datatype: Datatype,
    #[model(horned = range)]
    pub data_range: DataRange,
}

#[model(extends = ComponentBase)]
pub struct HasKey {
    #[model(horned = ce)]
    pub class_expression: ClassExpression,
    #[model(horned = vpe)]
    pub property_expressions: VecWrap<PropertyExpression>,
}

#[model(extends = ComponentBase)]
pub struct SameIndividual {
    #[model(horned = 0)]
    pub individuals: VecWrap<Individual>,
}

#[model(extends = ComponentBase)]
pub struct DifferentIndividuals {
    #[model(horned = 0)]
    pub individuals: VecWrap<Individual>,
}

#[model(extends = ComponentBase)]
pub struct ClassAssertion {
    #[model(horned = ce)]
    pub class_expression: ClassExpression,
    #[model(horned = i)]
    pub individual: Individual,
}

#[model(extends = ComponentBase)]
pub struct ObjectPropertyAssertion {
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = from)]
    pub source: Individual,
    #[model(horned = to)]
    pub target: Individual,
}

#[model(extends = ComponentBase)]
pub struct NegativeObjectPropertyAssertion {
    #[model(horned = ope)]
    pub object_property_expression: ObjectPropertyExpression,
    #[model(horned = from)]
    pub source: Individual,
    #[model(horned = to)]
    pub target: Individual,
}

#[model(extends = ComponentBase)]
pub struct DataPropertyAssertion {
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = from)]
    pub source: Individual,
    #[model(horned = to)]
    pub target: Literal,
}

#[model(extends = ComponentBase)]
pub struct NegativeDataPropertyAssertion {
    #[model(horned = dp)]
    pub data_property: DataProperty,
    #[model(horned = from)]
    pub source: Individual,
    #[model(horned = to)]
    pub target: Literal,
}

#[model(extends = ComponentBase)]
pub struct AnnotationAssertion {
    pub subject: AnnotationSubject,
    #[model(horned = ann)]
    pub annotation: Annotation,
}

#[model(extends = ComponentBase)]
pub struct SubAnnotationPropertyOf {
    #[model(horned = sub)]
    pub sub_property: AnnotationProperty,
    #[model(horned = sup)]
    pub super_property: AnnotationProperty,
}

#[model(extends = ComponentBase)]
pub struct AnnotationPropertyDomain {
    #[model(horned = ap)]
    pub annotation_property: AnnotationProperty,
    #[model(horned = iri)]
    pub domain: IRI,
}

#[model(extends = ComponentBase)]
pub struct AnnotationPropertyRange {
    #[model(horned = ap)]
    pub annotation_property: AnnotationProperty,
    #[model(horned = iri)]
    pub range: IRI,
}

#[model(extends = ComponentBase)]
pub struct Rule {
    pub head: VecWrap<Atom>,
    pub body: VecWrap<Atom>,
}

#[model]
pub struct AnnotatedComponent {
    pub component: Component,
    #[model(horned = ann, default = BTreeSetWrap::default())]
    pub annotations: BTreeSetWrap<Annotation>,
}

#[pymethods]
impl AnnotatedComponent {
    /// Renders this element on its own, in OWL functional (`"ofn"`, the
    /// default) or OWL 2 Manchester (`"omn"`) syntax. The per-element
    /// counterpart to `PyIndexedOntology.save_to_string`.
    ///
    /// Manchester syntax renders only the component, without its annotations.
    #[pyo3(signature = (serialization = None, prefix_mapping = None))]
    fn serialize(
        &self,
        serialization: Option<LiteralStr<SnippetSyntaxes>>,
        prefix_mapping: Option<&crate::prefix_mapping::PrefixMapping>,
    ) -> PyResult<String> {
        let value = horned_owl::model::AnnotatedComponent::<ArcStr>::from(self);
        Ok(
            match crate::snippet::parse_syntax(serialization.as_deref().unwrap_or("ofn"))? {
                // AsManchester has no AnnotatedComponent impl.
                crate::snippet::SnippetSyntax::Manchester => {
                    crate::as_omn!(value.component, prefix_mapping)
                }
                crate::snippet::SnippetSyntax::Functional => crate::as_ofn!(value, prefix_mapping),
            },
        )
    }

    fn __str__(&self) -> PyResult<String> {
        self.serialize(None, None)
    }
}
