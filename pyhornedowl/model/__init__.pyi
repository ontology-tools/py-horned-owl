import typing
from typing import *

class IRI:
    parse: Any
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

class Class:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    def __str__(self) -> str:
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

class AnonymousIndividual:
    first: str

    def __init__(self,first: str,/):
        ...


    def __str__(self) -> str:
        ...
    ...

class NamedIndividual:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    def __str__(self) -> str:
        ...
    ...

class ObjectProperty:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    def __str__(self) -> str:
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

class Datatype:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    def __str__(self) -> str:
        ...
    ...

class DataProperty:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    def __str__(self) -> str:
        ...
    ...

class FacetRestriction:
    f: Facet
    l: Literal

    def __init__(self,f: Facet,l: Literal,/):
        ...

    ...

Individual = typing.Union[AnonymousIndividual,NamedIndividual,]


class InverseObjectProperty:
    first: ObjectProperty

    def __init__(self,first: ObjectProperty,):
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

ObjectPropertyExpression = typing.Union[ObjectProperty,InverseObjectProperty,]


class SimpleLiteral:
    literal: str

    def __init__(self,literal: str,):
        ...
    

    ...
class LanguageLiteral:
    literal: str
    lang: str

    def __init__(self,literal: str,lang: str,):
        ...
    

    ...
class DatatypeLiteral:
    literal: str
    datatype_iri: IRI

    def __init__(self,literal: str,datatype_iri: IRI,):
        ...
    

    ...

Literal = typing.Union[SimpleLiteral,LanguageLiteral,DatatypeLiteral,]


class DataIntersectionOf:
    first: typing.List[DataRange]

    def __init__(self,first: typing.List[DataRange],):
        ...
    

    ...
class DataUnionOf:
    first: typing.List[DataRange]

    def __init__(self,first: typing.List[DataRange],):
        ...
    

    ...
class DataComplementOf:
    first: DataRange

    def __init__(self,first: DataRange,):
        ...
    

    ...
class DataOneOf:
    first: typing.List[Literal]

    def __init__(self,first: typing.List[Literal],):
        ...
    

    ...
class DatatypeRestriction:
    first: Datatype
    second: typing.List[FacetRestriction]

    def __init__(self,first: Datatype,second: typing.List[FacetRestriction],):
        ...
    

    ...

DataRange = typing.Union[Datatype,DataIntersectionOf,DataUnionOf,DataComplementOf,DataOneOf,DatatypeRestriction,]


class ObjectIntersectionOf:
    first: typing.List[ClassExpression]

    def __init__(self,first: typing.List[ClassExpression],):
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

    def __init__(self,first: typing.List[ClassExpression],):
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

    def __init__(self,first: ClassExpression,):
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

    def __init__(self,first: typing.List[Individual],):
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

    def __init__(self,ope: ObjectPropertyExpression,bce: ClassExpression,):
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

    def __init__(self,ope: ObjectPropertyExpression,bce: ClassExpression,):
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

    def __init__(self,ope: ObjectPropertyExpression,i: Individual,):
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

    def __init__(self,first: ObjectPropertyExpression,):
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

    def __init__(self,n: int,ope: ObjectPropertyExpression,bce: ClassExpression,):
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

    def __init__(self,n: int,ope: ObjectPropertyExpression,bce: ClassExpression,):
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

    def __init__(self,n: int,ope: ObjectPropertyExpression,bce: ClassExpression,):
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

    def __init__(self,dp: DataProperty,dr: DataRange,):
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

    def __init__(self,dp: DataProperty,dr: DataRange,):
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

    def __init__(self,dp: DataProperty,l: Literal,):
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

    def __init__(self,n: int,dp: DataProperty,dr: DataRange,):
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

    def __init__(self,n: int,dp: DataProperty,dr: DataRange,):
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

    def __init__(self,n: int,dp: DataProperty,dr: DataRange,):
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

ClassExpression = typing.Union[Class,ObjectIntersectionOf,ObjectUnionOf,ObjectComplementOf,ObjectOneOf,ObjectSomeValuesFrom,ObjectAllValuesFrom,ObjectHasValue,ObjectHasSelf,ObjectMinCardinality,ObjectMaxCardinality,ObjectExactCardinality,DataSomeValuesFrom,DataAllValuesFrom,DataHasValue,DataMinCardinality,DataMaxCardinality,DataExactCardinality,]

PropertyExpression = typing.Union[ObjectPropertyExpression,DataProperty,AnnotationProperty,]

AnnotationSubject = typing.Union[IRI,AnonymousIndividual,]

class AnnotationProperty:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    ...

AnnotationValue = typing.Union[Literal,IRI,AnonymousIndividual,]

class Annotation:
    ap: AnnotationProperty
    av: AnnotationValue

    def __init__(self,ap: AnnotationProperty,av: AnnotationValue,/):
        ...

    ...

class OntologyAnnotation:
    first: Annotation

    def __init__(self,first: Annotation,/):
        ...


    ...

class Import:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    ...

class DeclareClass:
    first: Class

    def __init__(self,first: Class,/):
        ...


    ...

class DeclareObjectProperty:
    first: ObjectProperty

    def __init__(self,first: ObjectProperty,/):
        ...


    ...

class DeclareAnnotationProperty:
    first: AnnotationProperty

    def __init__(self,first: AnnotationProperty,/):
        ...


    ...

class DeclareDataProperty:
    first: DataProperty

    def __init__(self,first: DataProperty,/):
        ...


    ...

class DeclareNamedIndividual:
    first: NamedIndividual

    def __init__(self,first: NamedIndividual,/):
        ...


    ...

class DeclareDatatype:
    first: Datatype

    def __init__(self,first: Datatype,/):
        ...


    ...

class SubClassOf:
    sub: ClassExpression
    sup: ClassExpression

    def __init__(self,sub: ClassExpression,sup: ClassExpression,/):
        ...

    ...

class EquivalentClasses:
    first: typing.List[ClassExpression]

    def __init__(self,first: typing.List[ClassExpression],/):
        ...


    ...

class DisjointClasses:
    first: typing.List[ClassExpression]

    def __init__(self,first: typing.List[ClassExpression],/):
        ...


    ...

class DisjointUnion:
    first: Class
    second: typing.List[ClassExpression]

    def __init__(self,first: Class,second: typing.List[ClassExpression],/):
        ...


    ...

SubObjectPropertyExpression = typing.Union[typing.List[ObjectPropertyExpression],ObjectPropertyExpression,]

class SubObjectPropertyOf:
    sub: SubObjectPropertyExpression
    sup: ObjectPropertyExpression

    def __init__(self,sub: SubObjectPropertyExpression,sup: ObjectPropertyExpression,/):
        ...

    ...

class EquivalentObjectProperties:
    first: typing.List[ObjectPropertyExpression]

    def __init__(self,first: typing.List[ObjectPropertyExpression],/):
        ...


    ...

class DisjointObjectProperties:
    first: typing.List[ObjectPropertyExpression]

    def __init__(self,first: typing.List[ObjectPropertyExpression],/):
        ...


    ...

class InverseObjectProperties:
    first: ObjectProperty
    second: ObjectProperty

    def __init__(self,first: ObjectProperty,second: ObjectProperty,/):
        ...


    ...

class ObjectPropertyDomain:
    ope: ObjectPropertyExpression
    ce: ClassExpression

    def __init__(self,ope: ObjectPropertyExpression,ce: ClassExpression,/):
        ...

    ...

class ObjectPropertyRange:
    ope: ObjectPropertyExpression
    ce: ClassExpression

    def __init__(self,ope: ObjectPropertyExpression,ce: ClassExpression,/):
        ...

    ...

class FunctionalObjectProperty:
    first: ObjectPropertyExpression

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class InverseFunctionalObjectProperty:
    first: ObjectPropertyExpression

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class ReflexiveObjectProperty:
    first: ObjectPropertyExpression

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class IrreflexiveObjectProperty:
    first: ObjectPropertyExpression

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class SymmetricObjectProperty:
    first: ObjectPropertyExpression

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class AsymmetricObjectProperty:
    first: ObjectPropertyExpression

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class TransitiveObjectProperty:
    first: ObjectPropertyExpression

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class SubDataPropertyOf:
    sub: DataProperty
    sup: DataProperty

    def __init__(self,sub: DataProperty,sup: DataProperty,/):
        ...

    ...

class EquivalentDataProperties:
    first: typing.List[DataProperty]

    def __init__(self,first: typing.List[DataProperty],/):
        ...


    ...

class DisjointDataProperties:
    first: typing.List[DataProperty]

    def __init__(self,first: typing.List[DataProperty],/):
        ...


    ...

class DataPropertyDomain:
    dp: DataProperty
    ce: ClassExpression

    def __init__(self,dp: DataProperty,ce: ClassExpression,/):
        ...

    ...

class DataPropertyRange:
    dp: DataProperty
    dr: DataRange

    def __init__(self,dp: DataProperty,dr: DataRange,/):
        ...

    ...

class FunctionalDataProperty:
    first: DataProperty

    def __init__(self,first: DataProperty,/):
        ...


    ...

class DatatypeDefinition:
    kind: Datatype
    range: DataRange

    def __init__(self,kind: Datatype,range: DataRange,/):
        ...

    ...

class HasKey:
    ce: ClassExpression
    vpe: typing.List[PropertyExpression]

    def __init__(self,ce: ClassExpression,vpe: typing.List[PropertyExpression],/):
        ...

    ...

class SameIndividual:
    first: typing.List[Individual]

    def __init__(self,first: typing.List[Individual],/):
        ...


    ...

class DifferentIndividuals:
    first: typing.List[Individual]

    def __init__(self,first: typing.List[Individual],/):
        ...


    ...

class ClassAssertion:
    ce: ClassExpression
    i: Individual

    def __init__(self,ce: ClassExpression,i: Individual,/):
        ...

    ...

class ObjectPropertyAssertion:
    ope: ObjectPropertyExpression
    source: Individual
    target: Individual

    def __init__(self,ope: ObjectPropertyExpression,source: Individual,target: Individual,/):
        ...

    ...

class NegativeObjectPropertyAssertion:
    ope: ObjectPropertyExpression
    source: Individual
    target: Individual

    def __init__(self,ope: ObjectPropertyExpression,source: Individual,target: Individual,/):
        ...

    ...

class DataPropertyAssertion:
    dp: DataProperty
    source: Individual
    target: Literal

    def __init__(self,dp: DataProperty,source: Individual,target: Literal,/):
        ...

    ...

class NegativeDataPropertyAssertion:
    dp: DataProperty
    source: Individual
    target: Literal

    def __init__(self,dp: DataProperty,source: Individual,target: Literal,/):
        ...

    ...

class AnnotationAssertion:
    subject: AnnotationSubject
    ann: Annotation

    def __init__(self,subject: AnnotationSubject,ann: Annotation,/):
        ...

    ...

class SubAnnotationPropertyOf:
    sub: AnnotationProperty
    sup: AnnotationProperty

    def __init__(self,sub: AnnotationProperty,sup: AnnotationProperty,/):
        ...

    ...

class AnnotationPropertyDomain:
    ap: AnnotationProperty
    iri: IRI

    def __init__(self,ap: AnnotationProperty,iri: IRI,/):
        ...

    ...

class AnnotationPropertyRange:
    ap: AnnotationProperty
    iri: IRI

    def __init__(self,ap: AnnotationProperty,iri: IRI,/):
        ...

    ...

class DocIRI:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    ...

class OntologyID:
    iri: typing.Optional[IRI]
    viri: typing.Optional[IRI]

    def __init__(self,iri: typing.Optional[IRI],viri: typing.Optional[IRI],/):
        ...

    ...

class Variable:
    first: IRI

    def __init__(self,first: IRI,/):
        ...


    ...

DArgument = typing.Union[Literal,Variable,]

IArgument = typing.Union[Individual,Variable,]


class BuiltInAtom:
    pred: IRI
    args: typing.List[DArgument]

    def __init__(self,pred: IRI,args: typing.List[DArgument],):
        ...
    

    ...
class ClassAtom:
    pred: ClassExpression
    arg: IArgument

    def __init__(self,pred: ClassExpression,arg: IArgument,):
        ...
    

    ...
class DataPropertyAtom:
    pred: DataProperty
    args: typing.Tuple[DArgument,DArgument]

    def __init__(self,pred: DataProperty,args: typing.Tuple[DArgument,DArgument],):
        ...
    

    ...
class DataRangeAtom:
    pred: DataRange
    arg: DArgument

    def __init__(self,pred: DataRange,arg: DArgument,):
        ...
    

    ...
class DifferentIndividualsAtom:
    first: IArgument
    second: IArgument

    def __init__(self,first: IArgument,second: IArgument,):
        ...
    

    ...
class ObjectPropertyAtom:
    pred: ObjectPropertyExpression
    args: typing.Tuple[IArgument,IArgument]

    def __init__(self,pred: ObjectPropertyExpression,args: typing.Tuple[IArgument,IArgument],):
        ...
    

    ...
class SameIndividualAtom:
    first: IArgument
    second: IArgument

    def __init__(self,first: IArgument,second: IArgument,):
        ...
    

    ...

Atom = typing.Union[BuiltInAtom,ClassAtom,DataPropertyAtom,DataRangeAtom,DifferentIndividualsAtom,ObjectPropertyAtom,SameIndividualAtom,]

class Rule:
    head: typing.List[Atom]
    body: typing.List[Atom]

    def __init__(self,head: typing.List[Atom],body: typing.List[Atom],/):
        ...

    ...

Component = typing.Union[OntologyID,DocIRI,OntologyAnnotation,Import,DeclareClass,DeclareObjectProperty,DeclareAnnotationProperty,DeclareDataProperty,DeclareNamedIndividual,DeclareDatatype,SubClassOf,EquivalentClasses,DisjointClasses,DisjointUnion,SubObjectPropertyOf,EquivalentObjectProperties,DisjointObjectProperties,InverseObjectProperties,ObjectPropertyDomain,ObjectPropertyRange,FunctionalObjectProperty,InverseFunctionalObjectProperty,ReflexiveObjectProperty,IrreflexiveObjectProperty,SymmetricObjectProperty,AsymmetricObjectProperty,TransitiveObjectProperty,SubDataPropertyOf,EquivalentDataProperties,DisjointDataProperties,DataPropertyDomain,DataPropertyRange,FunctionalDataProperty,DatatypeDefinition,HasKey,SameIndividual,DifferentIndividuals,ClassAssertion,ObjectPropertyAssertion,NegativeObjectPropertyAssertion,DataPropertyAssertion,NegativeDataPropertyAssertion,AnnotationAssertion,SubAnnotationPropertyOf,AnnotationPropertyDomain,AnnotationPropertyRange,Rule,]

class AnnotatedComponent:
    component: Component
    ann: typing.Set[Annotation]

    def __init__(self,component: Component,ann: typing.Set[Annotation],/):
        ...

    ...

