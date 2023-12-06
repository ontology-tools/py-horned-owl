import typing
from typing import *

class Class:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class ObjectIntersectionOf:
    first: list[ClassExpression]
    def __init__(self, first: list[ClassExpression]):
        ...
    ...

class ObjectUnionOf:
    first: list[ClassExpression]
    def __init__(self, first: list[ClassExpression]):
        ...
    ...

class ObjectComplementOf:
    first: ClassExpression
    def __init__(self, first: ClassExpression):
        ...
    ...

class ObjectOneOf:
    first: list[Individual]
    def __init__(self, first: list[Individual]):
        ...
    ...

class ObjectSomeValuesFrom:
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...
    ...

class ObjectAllValuesFrom:
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...
    ...

class ObjectHasValue:
    ope: ObjectPropertyExpression
    i: Individual
    def __init__(self, ope: ObjectPropertyExpression, i: Individual):
        ...
    ...

class ObjectHasSelf:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...
    ...

class ObjectMinCardinality:
    n: int
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, n: int, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...
    ...

class ObjectMaxCardinality:
    n: int
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, n: int, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...
    ...

class ObjectExactCardinality:
    n: int
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, n: int, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...
    ...

class DataSomeValuesFrom:
    dp: DataProperty
    dr: DataRange
    def __init__(self, dp: DataProperty, dr: DataRange):
        ...
    ...

class DataAllValuesFrom:
    dp: DataProperty
    dr: DataRange
    def __init__(self, dp: DataProperty, dr: DataRange):
        ...
    ...

class DataHasValue:
    dp: DataProperty
    l: Literal
    def __init__(self, dp: DataProperty, l: Literal):
        ...
    ...

class DataMinCardinality:
    n: int
    dp: DataProperty
    dr: DataRange
    def __init__(self, n: int, dp: DataProperty, dr: DataRange):
        ...
    ...

class DataMaxCardinality:
    n: int
    dp: DataProperty
    dr: DataRange
    def __init__(self, n: int, dp: DataProperty, dr: DataRange):
        ...
    ...

class DataExactCardinality:
    n: int
    dp: DataProperty
    dr: DataRange
    def __init__(self, n: int, dp: DataProperty, dr: DataRange):
        ...
    ...

class Datatype:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class DataIntersectionOf:
    first: list[DataRange]
    def __init__(self, first: list[DataRange]):
        ...
    ...

class DataUnionOf:
    first: list[DataRange]
    def __init__(self, first: list[DataRange]):
        ...
    ...

class DataComplementOf:
    first: DataRange
    def __init__(self, first: DataRange):
        ...
    ...

class DataOneOf:
    first: list[Literal]
    def __init__(self, first: list[Literal]):
        ...
    ...

class DatatypeRestriction:
    first: Datatype
    second: list[FacetRestriction]
    def __init__(self, first: Datatype, second: list[FacetRestriction]):
        ...
    ...

class SimpleLiteral:
    literal: str
    def __init__(self, literal: str):
        ...
    ...

class LanguageLiteral:
    literal: str
    lang: str
    def __init__(self, literal: str, lang: str):
        ...
    ...

class DatatypeLiteral:
    literal: str
    datatype_iri: IRI
    def __init__(self, literal: str, datatype_iri: IRI):
        ...
    ...

class ObjectProperty:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class InverseObjectProperty:
    first: ObjectProperty
    def __init__(self, first: ObjectProperty):
        ...
    ...

class AnnotatedAxiom:
    axiom: Axiom
    ann: set[Annotation]
    def __init__(self, axiom: Axiom, ann: set[Annotation]):
        ...
    ...

class Annotation:
    ap: AnnotationProperty
    av: AnnotationValue
    def __init__(self, ap: AnnotationProperty, av: AnnotationValue):
        ...
    ...

class AnnotationAssertion:
    subject: AnnotationSubject
    ann: Annotation
    def __init__(self, subject: AnnotationSubject, ann: Annotation):
        ...
    ...

class AnnotationProperty:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class AnnotationPropertyDomain:
    ap: AnnotationProperty
    iri: IRI
    def __init__(self, ap: AnnotationProperty, iri: IRI):
        ...
    ...

class AnnotationPropertyRange:
    ap: AnnotationProperty
    iri: IRI
    def __init__(self, ap: AnnotationProperty, iri: IRI):
        ...
    ...

class AnonymousIndividual:
    first: str
    def __init__(self, first: str):
        ...
    ...

class AsymmetricObjectProperty:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...
    ...

class ClassAssertion:
    ce: ClassExpression
    i: Individual
    def __init__(self, ce: ClassExpression, i: Individual):
        ...
    ...

class DataProperty:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class DataPropertyAssertion:
    dp: DataProperty
    "from": Individual
    to: Literal
    def __init__(self, dp: DataProperty, from_: Individual, to: Literal):
        ...
    ...

class DataPropertyDomain:
    dp: DataProperty
    ce: ClassExpression
    def __init__(self, dp: DataProperty, ce: ClassExpression):
        ...
    ...

class DataPropertyRange:
    dp: DataProperty
    dr: DataRange
    def __init__(self, dp: DataProperty, dr: DataRange):
        ...
    ...

class DatatypeDefinition:
    kind: Datatype
    range: DataRange
    def __init__(self, kind: Datatype, range: DataRange):
        ...
    ...

class DeclareAnnotationProperty:
    first: AnnotationProperty
    def __init__(self, first: AnnotationProperty):
        ...
    ...

class DeclareClass:
    first: Class
    def __init__(self, first: Class):
        ...
    ...

class DeclareDataProperty:
    first: DataProperty
    def __init__(self, first: DataProperty):
        ...
    ...

class DeclareDatatype:
    first: Datatype
    def __init__(self, first: Datatype):
        ...
    ...

class DeclareNamedIndividual:
    first: NamedIndividual
    def __init__(self, first: NamedIndividual):
        ...
    ...

class DeclareObjectProperty:
    first: ObjectProperty
    def __init__(self, first: ObjectProperty):
        ...
    ...

class DifferentIndividuals:
    first: list[Individual]
    def __init__(self, first: list[Individual]):
        ...
    ...

class DisjointClasses:
    first: list[ClassExpression]
    def __init__(self, first: list[ClassExpression]):
        ...
    ...

class DisjointDataProperties:
    first: list[DataProperty]
    def __init__(self, first: list[DataProperty]):
        ...
    ...

class DisjointObjectProperties:
    first: list[ObjectPropertyExpression]
    def __init__(self, first: list[ObjectPropertyExpression]):
        ...
    ...

class DisjointUnion:
    first: Class
    second: list[ClassExpression]
    def __init__(self, first: Class, second: list[ClassExpression]):
        ...
    ...

class EquivalentClasses:
    first: list[ClassExpression]
    def __init__(self, first: list[ClassExpression]):
        ...
    ...

class EquivalentDataProperties:
    first: list[DataProperty]
    def __init__(self, first: list[DataProperty]):
        ...
    ...

class EquivalentObjectProperties:
    first: list[ObjectPropertyExpression]
    def __init__(self, first: list[ObjectPropertyExpression]):
        ...
    ...

class FacetRestriction:
    f: Facet
    l: Literal
    def __init__(self, f: Facet, l: Literal):
        ...
    ...

class FunctionalDataProperty:
    first: DataProperty
    def __init__(self, first: DataProperty):
        ...
    ...

class FunctionalObjectProperty:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...
    ...

class HasKey:
    ce: ClassExpression
    vpe: list[PropertyExpression]
    def __init__(self, ce: ClassExpression, vpe: list[PropertyExpression]):
        ...
    ...

class IRI:
    parse: Any
    def __init__(self, parse):
        ...
    ...

class Import:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class InverseFunctionalObjectProperty:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...
    ...

class InverseObjectProperties:
    first: ObjectProperty
    second: ObjectProperty
    def __init__(self, first: ObjectProperty, second: ObjectProperty):
        ...
    ...

class IrreflexiveObjectProperty:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...
    ...

class NamedIndividual:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class NegativeDataPropertyAssertion:
    dp: DataProperty
    # from: Individual
    to: Literal
    def __init__(self, dp: DataProperty, from_: Individual, to: Literal):
        ...
    ...

class NegativeObjectPropertyAssertion:
    ope: ObjectPropertyExpression
    "from": Individual
    to: Individual
    def __init__(self, ope: ObjectPropertyExpression, from_: Individual, to: Individual):
        ...
    ...

class ObjectPropertyAssertion:
    ope: ObjectPropertyExpression
    "from": Individual
    to: Individual
    def __init__(self, ope: ObjectPropertyExpression, from_: Individual, to: Individual):
        ...
    ...

class ObjectPropertyDomain:
    ope: ObjectPropertyExpression
    ce: ClassExpression
    def __init__(self, ope: ObjectPropertyExpression, ce: ClassExpression):
        ...
    ...

class ObjectPropertyRange:
    ope: ObjectPropertyExpression
    ce: ClassExpression
    def __init__(self, ope: ObjectPropertyExpression, ce: ClassExpression):
        ...
    ...

class OntologyAnnotation:
    first: Annotation
    def __init__(self, first: Annotation):
        ...
    ...

class ReflexiveObjectProperty:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...
    ...

class SameIndividual:
    first: list[Individual]
    def __init__(self, first: list[Individual]):
        ...
    ...

class SubAnnotationPropertyOf:
    sup: AnnotationProperty
    sub: AnnotationProperty
    def __init__(self, sup: AnnotationProperty, sub: AnnotationProperty):
        ...
    ...

class SubClassOf:
    sup: ClassExpression
    sub: ClassExpression
    def __init__(self, sup: ClassExpression, sub: ClassExpression):
        ...
    ...

class SubDataPropertyOf:
    sup: DataProperty
    sub: DataProperty
    def __init__(self, sup: DataProperty, sub: DataProperty):
        ...
    ...

class SubObjectPropertyOf:
    sup: ObjectPropertyExpression
    sub: SubObjectPropertyExpression
    def __init__(self, sup: ObjectPropertyExpression, sub: SubObjectPropertyExpression):
        ...
    ...

class SymmetricObjectProperty:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...
    ...

class TransitiveObjectProperty:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...
    ...

class Facet:
    Length: Facet
    MinLength: Facet
    MaxLength: Facet
    Pattern: Facet
    MinInclusive: Facet
    MinExclusive: Facet
    MaxInclusive: Facet
    MaxExclusive: Facet
    TotalDigits: Facet
    FractionDigits: Facet
    LangRange: Facet

ClassExpression = typing.Union[ObjectIntersectionOf, ObjectUnionOf, ObjectComplementOf, ObjectOneOf, ObjectSomeValuesFrom, ObjectAllValuesFrom, ObjectHasValue, ObjectHasSelf, ObjectMinCardinality, ObjectMaxCardinality, ObjectExactCardinality, DataSomeValuesFrom, DataAllValuesFrom, DataHasValue, DataMinCardinality, DataMaxCardinality, DataExactCardinality, Class]

ObjectPropertyExpression = typing.Union[InverseObjectProperty, ObjectProperty]

Literal = typing.Union[SimpleLiteral, LanguageLiteral, DatatypeLiteral]

DataRange = typing.Union[DataIntersectionOf, DataUnionOf, DataComplementOf, DataOneOf, DatatypeRestriction, Datatype]

Individual = typing.Union[AnonymousIndividual, NamedIndividual]

PropertyExpression = typing.Union[ObjectPropertyExpression, DataProperty, AnnotationProperty]

AnnotationSubject = typing.Union[IRI, AnonymousIndividual]

AnnotationValue = typing.Union[Literal, IRI]

SubObjectPropertyExpression = typing.Union[list[ObjectPropertyExpression], ObjectPropertyExpression]

Axiom = typing.Union[OntologyAnnotation, Import, DeclareClass, DeclareObjectProperty, DeclareAnnotationProperty, DeclareDataProperty, DeclareNamedIndividual, DeclareDatatype, SubClassOf, EquivalentClasses, DisjointClasses, DisjointUnion, SubObjectPropertyOf, EquivalentObjectProperties, DisjointObjectProperties, InverseObjectProperties, ObjectPropertyDomain, ObjectPropertyRange, FunctionalObjectProperty, InverseFunctionalObjectProperty, ReflexiveObjectProperty, IrreflexiveObjectProperty, SymmetricObjectProperty, AsymmetricObjectProperty, TransitiveObjectProperty, SubDataPropertyOf, EquivalentDataProperties, DisjointDataProperties, DataPropertyDomain, DataPropertyRange, FunctionalDataProperty, DatatypeDefinition, HasKey, SameIndividual, DifferentIndividuals, ClassAssertion, ObjectPropertyAssertion, NegativeObjectPropertyAssertion, DataPropertyAssertion, NegativeDataPropertyAssertion, AnnotationAssertion, SubAnnotationPropertyOf, AnnotationPropertyDomain, AnnotationPropertyRange]


