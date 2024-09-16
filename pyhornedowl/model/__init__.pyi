import typing
from typing import *
from typing_extensions import deprecated

class Class:
    first: Any
    ...

class ObjectIntersectionOf:
    first: Any
    ...

class ObjectUnionOf:
    first: Any
    ...

class ObjectComplementOf:
    first: Any
    ...

class ObjectOneOf:
    first: Any
    ...

class ObjectSomeValuesFrom:
    ope: Any
    bce: Any
    ...

class ObjectAllValuesFrom:
    bce: Any
    ope: Any
    ...

class ObjectHasValue:
    i: Any
    ope: Any
    ...

class ObjectHasSelf:
    first: Any
    ...

class ObjectMinCardinality:
    ope: Any
    n: Any
    bce: Any
    ...

class ObjectMaxCardinality:
    bce: Any
    n: Any
    ope: Any
    ...

class ObjectExactCardinality:
    bce: Any
    ope: Any
    n: Any
    ...

class DataSomeValuesFrom:
    dr: Any
    dp: Any
    ...

class DataAllValuesFrom:
    dp: Any
    dr: Any
    ...

class DataHasValue:
    dp: Any
    l: Any
    ...

class DataMinCardinality:
    n: Any
    dp: Any
    dr: Any
    ...

class DataMaxCardinality:
    dr: Any
    dp: Any
    n: Any
    ...

class DataExactCardinality:
    dr: Any
    n: Any
    dp: Any
    ...

class Datatype:
    first: Any
    ...

class DataIntersectionOf:
    first: Any
    ...

class DataUnionOf:
    first: Any
    ...

class DataComplementOf:
    first: Any
    ...

class DataOneOf:
    first: Any
    ...

class DatatypeRestriction:
    first: Any
    second: Any
    ...

class SimpleLiteral:
    literal: Any
    ...

class LanguageLiteral:
    literal: Any
    lang: Any
    ...

class DatatypeLiteral:
    datatype_iri: Any
    literal: Any
    ...

class ObjectProperty:
    some: Any
    only: Any
    has_value: Any
    has_self: Any
    min: Any
    max: Any
    exact: Any
    first: Any
    ...

class InverseObjectProperty:
    some: Any
    only: Any
    has_value: Any
    has_self: Any
    min: Any
    max: Any
    exact: Any
    first: Any
    ...

class AnnotatedComponent:
    component: Any
    ann: Any
    ...

class Annotation:
    ap: Any
    av: Any
    ...

class AnnotationAssertion:
    subject: Any
    ann: Any
    ...

class AnnotationProperty:
    first: Any
    ...

class AnnotationPropertyDomain:
    iri: Any
    ap: Any
    ...

class AnnotationPropertyRange:
    ap: Any
    iri: Any
    ...

class AnonymousIndividual:
    first: Any
    ...

class AsymmetricObjectProperty:
    first: Any
    ...

class ClassAssertion:
    ce: Any
    i: Any
    ...

class DataProperty:
    first: Any
    ...

class DataPropertyAssertion:
    dp: Any
    to: Any
    ...

class DataPropertyDomain:
    ce: Any
    dp: Any
    ...

class DataPropertyRange:
    dp: Any
    dr: Any
    ...

class DatatypeDefinition:
    kind: Any
    range: Any
    ...

class DeclareAnnotationProperty:
    first: Any
    ...

class DeclareClass:
    first: Any
    ...

class DeclareDataProperty:
    first: Any
    ...

class DeclareDatatype:
    first: Any
    ...

class DeclareNamedIndividual:
    first: Any
    ...

class DeclareObjectProperty:
    first: Any
    ...

class DifferentIndividuals:
    first: Any
    ...

class DisjointClasses:
    first: Any
    ...

class DisjointDataProperties:
    first: Any
    ...

class DisjointObjectProperties:
    first: Any
    ...

class DisjointUnion:
    second: Any
    first: Any
    ...

class EquivalentClasses:
    first: Any
    ...

class EquivalentDataProperties:
    first: Any
    ...

class EquivalentObjectProperties:
    first: Any
    ...

class FacetRestriction:
    l: Any
    f: Any
    ...

class FunctionalDataProperty:
    first: Any
    ...

class FunctionalObjectProperty:
    first: Any
    ...

class HasKey:
    ce: Any
    vpe: Any
    ...

class IRI:
    parse: Any
    ...

class Import:
    first: Any
    ...

class InverseFunctionalObjectProperty:
    first: Any
    ...

class InverseObjectProperties:
    second: Any
    first: Any
    ...

class IrreflexiveObjectProperty:
    first: Any
    ...

class NamedIndividual:
    first: Any
    ...

class NegativeDataPropertyAssertion:
    dp: Any
    to: Any
    ...

class NegativeObjectPropertyAssertion:
    ope: Any
    to: Any
    ...

class ObjectPropertyAssertion:
    to: Any
    ope: Any
    ...

class ObjectPropertyDomain:
    ope: Any
    ce: Any
    ...

class ObjectPropertyRange:
    ope: Any
    ce: Any
    ...

class OntologyAnnotation:
    first: Any
    ...

class ReflexiveObjectProperty:
    first: Any
    ...

class SameIndividual:
    first: Any
    ...

class SubAnnotationPropertyOf:
    sup: Any
    sub: Any
    ...

class SubClassOf:
    sup: Any
    sub: Any
    ...

class SubDataPropertyOf:
    sub: Any
    sup: Any
    ...

class SubObjectPropertyOf:
    sub: Any
    sup: Any
    ...

class SymmetricObjectProperty:
    first: Any
    ...

class TransitiveObjectProperty:
    first: Any
    ...

class OntologyID:
    iri: Any
    viri: Any
    ...

class DocIRI:
    first: Any
    ...

class Rule:
    body: Any
    head: Any
    ...

class Variable:
    first: Any
    ...

class BuiltInAtom:
    args: Any
    pred: Any
    ...

class ClassAtom:
    arg: Any
    pred: Any
    ...

class DataPropertyAtom:
    args: Any
    pred: Any
    ...

class DataRangeAtom:
    arg: Any
    pred: Any
    ...

class DifferentIndividualsAtom:
    first: Any
    second: Any
    ...

class ObjectPropertyAtom:
    pred: Any
    args: Any
    ...

class SameIndividualAtom:
    second: Any
    first: Any
    ...

class Facet:
    Length: Any
    MinLength: Any
    MaxLength: Any
    Pattern: Any
    MinInclusive: Any
    MinExclusive: Any
    MaxInclusive: Any
    MaxExclusive: Any
    TotalDigits: Any
    FractionDigits: Any
    LangRange: Any
    ...

ClassExpression = typing.Union[ObjectIntersectionOf, ObjectUnionOf, ObjectComplementOf, ObjectOneOf, ObjectSomeValuesFrom, ObjectAllValuesFrom, ObjectHasValue, ObjectHasSelf, ObjectMinCardinality, ObjectMaxCardinality, ObjectExactCardinality, DataSomeValuesFrom, DataAllValuesFrom, DataHasValue, DataMinCardinality, DataMaxCardinality, DataExactCardinality, Class]
ObjectPropertyExpression = typing.Union[InverseObjectProperty, ObjectProperty]
SubObjectPropertyExpression = typing.Union[typing.List[typing.Union[InverseObjectProperty, ObjectProperty]], InverseObjectProperty, ObjectProperty]
Literal = typing.Union[SimpleLiteral, LanguageLiteral, DatatypeLiteral]
DataRange = typing.Union[DataIntersectionOf, DataUnionOf, DataComplementOf, DataOneOf, DatatypeRestriction, Datatype]
Individual = typing.Union[AnonymousIndividual, NamedIndividual]
PropertyExpression = typing.Union[InverseObjectProperty, ObjectProperty, DataProperty, AnnotationProperty]
AnnotationSubject = typing.Union[IRI, AnonymousIndividual]
AnnotationValue = typing.Union[SimpleLiteral, LanguageLiteral, DatatypeLiteral, IRI, AnonymousIndividual]
Component = typing.Union[OntologyID, DocIRI, OntologyAnnotation, Import, DeclareClass, DeclareObjectProperty, DeclareAnnotationProperty, DeclareDataProperty, DeclareNamedIndividual, DeclareDatatype, SubClassOf, EquivalentClasses, DisjointClasses, DisjointUnion, SubObjectPropertyOf, EquivalentObjectProperties, DisjointObjectProperties, InverseObjectProperties, ObjectPropertyDomain, ObjectPropertyRange, FunctionalObjectProperty, InverseFunctionalObjectProperty, ReflexiveObjectProperty, IrreflexiveObjectProperty, SymmetricObjectProperty, AsymmetricObjectProperty, TransitiveObjectProperty, SubDataPropertyOf, EquivalentDataProperties, DisjointDataProperties, DataPropertyDomain, DataPropertyRange, FunctionalDataProperty, DatatypeDefinition, HasKey, SameIndividual, DifferentIndividuals, ClassAssertion, ObjectPropertyAssertion, NegativeObjectPropertyAssertion, DataPropertyAssertion, NegativeDataPropertyAssertion, AnnotationAssertion, SubAnnotationPropertyOf, AnnotationPropertyDomain, AnnotationPropertyRange, Rule]
Atom = typing.Union[BuiltInAtom, ClassAtom, DataPropertyAtom, DataRangeAtom, DifferentIndividualsAtom, ObjectPropertyAtom, SameIndividualAtom]
IArgument = typing.Union[AnonymousIndividual, NamedIndividual, Variable]
DArgument = typing.Union[SimpleLiteral, LanguageLiteral, DatatypeLiteral, Variable]

