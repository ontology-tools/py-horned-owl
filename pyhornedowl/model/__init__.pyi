import typing
from typing import (Any)
from typing_extensions import deprecated

class Class:
    first: IRI
    def __init__(self, first: IRI):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectIntersectionOf:
    first: typing.List[ClassExpression]
    def __init__(self, first: typing.List[ClassExpression]):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectUnionOf:
    first: typing.List[ClassExpression]
    def __init__(self, first: typing.List[ClassExpression]):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectComplementOf:
    first: ClassExpression
    def __init__(self, first: ClassExpression):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectOneOf:
    first: typing.List[Individual]
    def __init__(self, first: typing.List[Individual]):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectSomeValuesFrom:
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectAllValuesFrom:
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectHasValue:
    ope: ObjectPropertyExpression
    i: Individual
    def __init__(self, ope: ObjectPropertyExpression, i: Individual):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectHasSelf:
    first: ObjectPropertyExpression
    def __init__(self, first: ObjectPropertyExpression):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectMinCardinality:
    n: int
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, n: int, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectMaxCardinality:
    n: int
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, n: int, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class ObjectExactCardinality:
    n: int
    ope: ObjectPropertyExpression
    bce: ClassExpression
    def __init__(self, n: int, ope: ObjectPropertyExpression, bce: ClassExpression):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class DataSomeValuesFrom:
    dp: DataProperty
    dr: DataRange
    def __init__(self, dp: DataProperty, dr: DataRange):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class DataAllValuesFrom:
    dp: DataProperty
    dr: DataRange
    def __init__(self, dp: DataProperty, dr: DataRange):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class DataHasValue:
    dp: DataProperty
    l: Literal
    def __init__(self, dp: DataProperty, l: Literal):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class DataMinCardinality:
    n: int
    dp: DataProperty
    dr: DataRange
    def __init__(self, n: int, dp: DataProperty, dr: DataRange):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class DataMaxCardinality:
    n: int
    dp: DataProperty
    dr: DataRange
    def __init__(self, n: int, dp: DataProperty, dr: DataRange):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class DataExactCardinality:
    n: int
    dp: DataProperty
    dr: DataRange
    def __init__(self, n: int, dp: DataProperty, dr: DataRange):
        ...

    def __and__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Intersection of two class expressions"""
        ...

    def __or__(self, ce: ClassExpression) -> ObjectIntersectionOf:
        """Union of two class expressions"""
        ...

    def __invert__(self) -> ObjectIntersectionOf:
        """Complement of a class expression"""
        ...
    ...

class Datatype:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class DataIntersectionOf:
    first: typing.List[DataRange]
    def __init__(self, first: typing.List[DataRange]):
        ...
    ...

class DataUnionOf:
    first: typing.List[DataRange]
    def __init__(self, first: typing.List[DataRange]):
        ...
    ...

class DataComplementOf:
    first: DataRange
    def __init__(self, first: DataRange):
        ...
    ...

class DataOneOf:
    first: typing.List[Literal]
    def __init__(self, first: typing.List[Literal]):
        ...
    ...

class DatatypeRestriction:
    first: Datatype
    second: typing.List[FacetRestriction]
    def __init__(self, first: Datatype, second: typing.List[FacetRestriction]):
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

    def some(self, ce: ClassExpression) -> ObjectSomeValuesFrom:
        """Existentional relationship"""
        ...

    def only(self, ce: ClassExpression) -> ObjectAllValuesFrom:
        """Universal relationship"""
        ...

    def has_value(self, individual: Individual) -> ObjectHasValue:
        """Existential relationship to an individual"""
        ...

    def has_self(self) -> ObjectHasSelf:
        """Individuals with relation to themselves"""
        ...

    def min(self, n: int, ce: ClassExpression) -> ObjectMinCardinality:
        """Minimum cardinality relationship"""
        ...

    def max(self, n: int, ce: ClassExpression) -> ObjectMaxCardinality:
        """Maximum cardinality relationship"""
        ...

    def exact(self, n: int, ce: ClassExpression) -> ObjectExactCardinality:
        """Exact cardinality relationship"""
        ...

    def __invert__(self) -> ObjectPropertyExpression:
        """Inverse of object property expression"""
        ...
    ...

class InverseObjectProperty:
    first: ObjectProperty
    def __init__(self, first: ObjectProperty):
        ...

    def some(self, ce: ClassExpression) -> ObjectSomeValuesFrom:
        """Existentional relationship"""
        ...

    def only(self, ce: ClassExpression) -> ObjectAllValuesFrom:
        """Universal relationship"""
        ...

    def has_value(self, individual: Individual) -> ObjectHasValue:
        """Existential relationship to an individual"""
        ...

    def has_self(self) -> ObjectHasSelf:
        """Individuals with relation to themselves"""
        ...

    def min(self, n: int, ce: ClassExpression) -> ObjectMinCardinality:
        """Minimum cardinality relationship"""
        ...

    def max(self, n: int, ce: ClassExpression) -> ObjectMaxCardinality:
        """Maximum cardinality relationship"""
        ...

    def exact(self, n: int, ce: ClassExpression) -> ObjectExactCardinality:
        """Exact cardinality relationship"""
        ...

    def __invert__(self) -> ObjectPropertyExpression:
        """Inverse of object property expression"""
        ...
    ...

class AnnotatedComponent:
    component: Component
    ann: typing.Set[Annotation]
    def __init__(self, component: Component, ann: typing.Set[Annotation]):
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
    from_: Individual
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
    first: typing.List[Individual]
    def __init__(self, first: typing.List[Individual]):
        ...
    ...

class DisjointClasses:
    first: typing.List[ClassExpression]
    def __init__(self, first: typing.List[ClassExpression]):
        ...
    ...

class DisjointDataProperties:
    first: typing.List[DataProperty]
    def __init__(self, first: typing.List[DataProperty]):
        ...
    ...

class DisjointObjectProperties:
    first: typing.List[ObjectPropertyExpression]
    def __init__(self, first: typing.List[ObjectPropertyExpression]):
        ...
    ...

class DisjointUnion:
    first: Class
    second: typing.List[ClassExpression]
    def __init__(self, first: Class, second: typing.List[ClassExpression]):
        ...
    ...

class EquivalentClasses:
    first: typing.List[ClassExpression]
    def __init__(self, first: typing.List[ClassExpression]):
        ...
    ...

class EquivalentDataProperties:
    first: typing.List[DataProperty]
    def __init__(self, first: typing.List[DataProperty]):
        ...
    ...

class EquivalentObjectProperties:
    first: typing.List[ObjectPropertyExpression]
    def __init__(self, first: typing.List[ObjectPropertyExpression]):
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
    vpe: typing.List[PropertyExpression]
    def __init__(self, ce: ClassExpression, vpe: typing.List[PropertyExpression]):
        ...
    ...

class IRI:
    parse: Any
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
    from_: Individual
    to: Literal
    def __init__(self, dp: DataProperty, from_: Individual, to: Literal):
        ...
    ...

class NegativeObjectPropertyAssertion:
    ope: ObjectPropertyExpression
    from_: Individual
    to: Individual
    def __init__(self, ope: ObjectPropertyExpression, from_: Individual, to: Individual):
        ...
    ...

class ObjectPropertyAssertion:
    ope: ObjectPropertyExpression
    from_: Individual
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
    first: typing.List[Individual]
    def __init__(self, first: typing.List[Individual]):
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

class OntologyID:
    iri: typing.Optional[IRI]
    viri: typing.Optional[IRI]
    def __init__(self, iri: typing.Optional[IRI], viri: typing.Optional[IRI]):
        ...
    ...

class DocIRI:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class Rule:
    head: typing.List[Atom]
    body: typing.List[Atom]
    def __init__(self, head: typing.List[Atom], body: typing.List[Atom]):
        ...
    ...

class Variable:
    first: IRI
    def __init__(self, first: IRI):
        ...
    ...

class BuiltInAtom:
    pred: IRI
    args: typing.List[DArgument]
    def __init__(self, pred: IRI, args: typing.List[DArgument]):
        ...
    ...

class ClassAtom:
    pred: ClassExpression
    arg: IArgument
    def __init__(self, pred: ClassExpression, arg: IArgument):
        ...
    ...

class DataPropertyAtom:
    pred: DataProperty
    args: typing.Tuple[DArgument, DArgument]
    def __init__(self, pred: DataProperty, args: typing.Tuple[DArgument, DArgument]):
        ...
    ...

class DataRangeAtom:
    pred: DataRange
    arg: DArgument
    def __init__(self, pred: DataRange, arg: DArgument):
        ...
    ...

class DifferentIndividualsAtom:
    first: IArgument
    second: IArgument
    def __init__(self, first: IArgument, second: IArgument):
        ...
    ...

class ObjectPropertyAtom:
    pred: ObjectPropertyExpression
    args: typing.Tuple[IArgument, IArgument]
    def __init__(self, pred: ObjectPropertyExpression, args: typing.Tuple[IArgument, IArgument]):
        ...
    ...

class SameIndividualAtom:
    first: IArgument
    second: IArgument
    def __init__(self, first: IArgument, second: IArgument):
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

