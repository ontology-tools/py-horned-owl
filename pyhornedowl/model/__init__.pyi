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
    

IRIParam = Union[str, IRI, Tuple[str, bool]]
"""
One of the following:
- A string representing an IRI, e.g. "https://example.com/A"
- A string representing a CURIE, e.g. "ex:A", which will be expanded using the prefix mapping
- An IRI object
- A tuple of a string and a boolean, where the boolean indicates whether the string is an absolute IRI (True) or a CURIE (False)
"""

class Class:
    first: IRI

    __match_args__ = ("first", )

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

    __match_args__ = ("first", )

    def __init__(self,first: str,/):
        ...


    def __str__(self) -> str:
        ...
    ...

class NamedIndividual:
    first: IRI

    __match_args__ = ("first", )

    def __init__(self,first: IRI,/):
        ...


    def __str__(self) -> str:
        ...
    ...

class ObjectProperty:
    first: IRI

    __match_args__ = ("first", )

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

    __match_args__ = ("first", )

    def __init__(self,first: IRI,/):
        ...


    def __str__(self) -> str:
        ...
    ...

class DataProperty:
    first: IRI

    __match_args__ = ("first", )

    def __init__(self,first: IRI,/):
        ...


    def __str__(self) -> str:
        ...
    ...

class FacetRestriction:
    f: Facet
    l: Literal
    __match_args__ = ("f", "l", )

    def __init__(self,f: Facet,l: Literal,/):
        ...

    ...

Individual = typing.Union[AnonymousIndividual,NamedIndividual,]


class InverseObjectProperty:
    first: ObjectProperty

    __match_args__ = ("first", )


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

    __match_args__ = ("literal", )


    def __init__(self,literal: str,):
        ...
    

    ...
class LanguageLiteral:
    literal: str
    lang: str

    __match_args__ = ("literal", "lang", )


    def __init__(self,literal: str,lang: str,):
        ...
    

    ...
class DatatypeLiteral:
    literal: str
    datatype_iri: IRI

    __match_args__ = ("literal", "datatype_iri", )


    def __init__(self,literal: str,datatype_iri: IRI,):
        ...
    

    ...

Literal = typing.Union[SimpleLiteral,LanguageLiteral,DatatypeLiteral,]


class DataIntersectionOf:
    first: typing.List[DataRange]

    __match_args__ = ("first", )


    def __init__(self,first: typing.List[DataRange],):
        ...
    

    ...
class DataUnionOf:
    first: typing.List[DataRange]

    __match_args__ = ("first", )


    def __init__(self,first: typing.List[DataRange],):
        ...
    

    ...
class DataComplementOf:
    first: DataRange

    __match_args__ = ("first", )


    def __init__(self,first: DataRange,):
        ...
    

    ...
class DataOneOf:
    first: typing.List[Literal]

    __match_args__ = ("first", )


    def __init__(self,first: typing.List[Literal],):
        ...
    

    ...
class DatatypeRestriction:
    first: Datatype
    second: typing.List[FacetRestriction]

    __match_args__ = ("first", "second", )


    def __init__(self,first: Datatype,second: typing.List[FacetRestriction],):
        ...
    

    ...

DataRange = typing.Union[Datatype,DataIntersectionOf,DataUnionOf,DataComplementOf,DataOneOf,DatatypeRestriction,]


class ObjectIntersectionOf:
    first: typing.List[ClassExpression]

    __match_args__ = ("first", )


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

    __match_args__ = ("first", )


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

    __match_args__ = ("first", )


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

    __match_args__ = ("first", )


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

    __match_args__ = ("ope", "bce", )


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

    __match_args__ = ("ope", "bce", )


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

    __match_args__ = ("ope", "i", )


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

    __match_args__ = ("first", )


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

    __match_args__ = ("n", "ope", "bce", )


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

    __match_args__ = ("n", "ope", "bce", )


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

    __match_args__ = ("n", "ope", "bce", )


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

    __match_args__ = ("dp", "dr", )


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

    __match_args__ = ("dp", "dr", )


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

    __match_args__ = ("dp", "l", )


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

    __match_args__ = ("n", "dp", "dr", )


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

    __match_args__ = ("n", "dp", "dr", )


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

    __match_args__ = ("n", "dp", "dr", )


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

    __match_args__ = ("first", )

    def __init__(self,first: IRI,/):
        ...


    ...

AnnotationValue = typing.Union[Literal,IRI,AnonymousIndividual,]

class Annotation:
    ap: AnnotationProperty
    av: AnnotationValue
    __match_args__ = ("ap", "av", )

    def __init__(self,ap: AnnotationProperty,av: AnnotationValue,/):
        ...

    ...

class OntologyAnnotation:
    first: Annotation

    __match_args__ = ("first", )

    def __init__(self,first: Annotation,/):
        ...


    ...

class Import:
    first: IRI

    __match_args__ = ("first", )

    def __init__(self,first: IRI,/):
        ...


    ...

class DeclareClass:
    first: Class

    __match_args__ = ("first", )

    def __init__(self,first: Class,/):
        ...


    ...

class DeclareObjectProperty:
    first: ObjectProperty

    __match_args__ = ("first", )

    def __init__(self,first: ObjectProperty,/):
        ...


    ...

class DeclareAnnotationProperty:
    first: AnnotationProperty

    __match_args__ = ("first", )

    def __init__(self,first: AnnotationProperty,/):
        ...


    ...

class DeclareDataProperty:
    first: DataProperty

    __match_args__ = ("first", )

    def __init__(self,first: DataProperty,/):
        ...


    ...

class DeclareNamedIndividual:
    first: NamedIndividual

    __match_args__ = ("first", )

    def __init__(self,first: NamedIndividual,/):
        ...


    ...

class DeclareDatatype:
    first: Datatype

    __match_args__ = ("first", )

    def __init__(self,first: Datatype,/):
        ...


    ...

class SubClassOf:
    sub: ClassExpression
    sup: ClassExpression
    __match_args__ = ("sub", "sup", )

    def __init__(self,sub: ClassExpression,sup: ClassExpression,/):
        ...

    ...

class EquivalentClasses:
    first: typing.List[ClassExpression]

    __match_args__ = ("first", )

    def __init__(self,first: typing.List[ClassExpression],/):
        ...


    ...

class DisjointClasses:
    first: typing.List[ClassExpression]

    __match_args__ = ("first", )

    def __init__(self,first: typing.List[ClassExpression],/):
        ...


    ...

class DisjointUnion:
    first: Class
    second: typing.List[ClassExpression]

    __match_args__ = ("first", "second", )

    def __init__(self,first: Class,second: typing.List[ClassExpression],/):
        ...


    ...

SubObjectPropertyExpression = typing.Union[typing.List[ObjectPropertyExpression],ObjectPropertyExpression,]

class SubObjectPropertyOf:
    sub: SubObjectPropertyExpression
    sup: ObjectPropertyExpression
    __match_args__ = ("sub", "sup", )

    def __init__(self,sub: SubObjectPropertyExpression,sup: ObjectPropertyExpression,/):
        ...

    ...

class EquivalentObjectProperties:
    first: typing.List[ObjectPropertyExpression]

    __match_args__ = ("first", )

    def __init__(self,first: typing.List[ObjectPropertyExpression],/):
        ...


    ...

class DisjointObjectProperties:
    first: typing.List[ObjectPropertyExpression]

    __match_args__ = ("first", )

    def __init__(self,first: typing.List[ObjectPropertyExpression],/):
        ...


    ...

class InverseObjectProperties:
    first: ObjectProperty
    second: ObjectProperty

    __match_args__ = ("first", "second", )

    def __init__(self,first: ObjectProperty,second: ObjectProperty,/):
        ...


    ...

class ObjectPropertyDomain:
    ope: ObjectPropertyExpression
    ce: ClassExpression
    __match_args__ = ("ope", "ce", )

    def __init__(self,ope: ObjectPropertyExpression,ce: ClassExpression,/):
        ...

    ...

class ObjectPropertyRange:
    ope: ObjectPropertyExpression
    ce: ClassExpression
    __match_args__ = ("ope", "ce", )

    def __init__(self,ope: ObjectPropertyExpression,ce: ClassExpression,/):
        ...

    ...

class FunctionalObjectProperty:
    first: ObjectPropertyExpression

    __match_args__ = ("first", )

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class InverseFunctionalObjectProperty:
    first: ObjectPropertyExpression

    __match_args__ = ("first", )

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class ReflexiveObjectProperty:
    first: ObjectPropertyExpression

    __match_args__ = ("first", )

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class IrreflexiveObjectProperty:
    first: ObjectPropertyExpression

    __match_args__ = ("first", )

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class SymmetricObjectProperty:
    first: ObjectPropertyExpression

    __match_args__ = ("first", )

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class AsymmetricObjectProperty:
    first: ObjectPropertyExpression

    __match_args__ = ("first", )

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class TransitiveObjectProperty:
    first: ObjectPropertyExpression

    __match_args__ = ("first", )

    def __init__(self,first: ObjectPropertyExpression,/):
        ...


    ...

class SubDataPropertyOf:
    sub: DataProperty
    sup: DataProperty
    __match_args__ = ("sub", "sup", )

    def __init__(self,sub: DataProperty,sup: DataProperty,/):
        ...

    ...

class EquivalentDataProperties:
    first: typing.List[DataProperty]

    __match_args__ = ("first", )

    def __init__(self,first: typing.List[DataProperty],/):
        ...


    ...

class DisjointDataProperties:
    first: typing.List[DataProperty]

    __match_args__ = ("first", )

    def __init__(self,first: typing.List[DataProperty],/):
        ...


    ...

class DataPropertyDomain:
    dp: DataProperty
    ce: ClassExpression
    __match_args__ = ("dp", "ce", )

    def __init__(self,dp: DataProperty,ce: ClassExpression,/):
        ...

    ...

class DataPropertyRange:
    dp: DataProperty
    dr: DataRange
    __match_args__ = ("dp", "dr", )

    def __init__(self,dp: DataProperty,dr: DataRange,/):
        ...

    ...

class FunctionalDataProperty:
    first: DataProperty

    __match_args__ = ("first", )

    def __init__(self,first: DataProperty,/):
        ...


    ...

class DatatypeDefinition:
    kind: Datatype
    range: DataRange
    __match_args__ = ("kind", "range", )

    def __init__(self,kind: Datatype,range: DataRange,/):
        ...

    ...

class HasKey:
    ce: ClassExpression
    vpe: typing.List[PropertyExpression]
    __match_args__ = ("ce", "vpe", )

    def __init__(self,ce: ClassExpression,vpe: typing.List[PropertyExpression],/):
        ...

    ...

class SameIndividual:
    first: typing.List[Individual]

    __match_args__ = ("first", )

    def __init__(self,first: typing.List[Individual],/):
        ...


    ...

class DifferentIndividuals:
    first: typing.List[Individual]

    __match_args__ = ("first", )

    def __init__(self,first: typing.List[Individual],/):
        ...


    ...

class ClassAssertion:
    ce: ClassExpression
    i: Individual
    __match_args__ = ("ce", "i", )

    def __init__(self,ce: ClassExpression,i: Individual,/):
        ...

    ...

class ObjectPropertyAssertion:
    ope: ObjectPropertyExpression
    source: Individual
    target: Individual
    __match_args__ = ("ope", "from", "to", )

    def __init__(self,ope: ObjectPropertyExpression,source: Individual,target: Individual,/):
        ...

    ...

class NegativeObjectPropertyAssertion:
    ope: ObjectPropertyExpression
    source: Individual
    target: Individual
    __match_args__ = ("ope", "from", "to", )

    def __init__(self,ope: ObjectPropertyExpression,source: Individual,target: Individual,/):
        ...

    ...

class DataPropertyAssertion:
    dp: DataProperty
    source: Individual
    target: Literal
    __match_args__ = ("dp", "from", "to", )

    def __init__(self,dp: DataProperty,source: Individual,target: Literal,/):
        ...

    ...

class NegativeDataPropertyAssertion:
    dp: DataProperty
    source: Individual
    target: Literal
    __match_args__ = ("dp", "from", "to", )

    def __init__(self,dp: DataProperty,source: Individual,target: Literal,/):
        ...

    ...

class AnnotationAssertion:
    subject: AnnotationSubject
    ann: Annotation
    __match_args__ = ("subject", "ann", )

    def __init__(self,subject: AnnotationSubject,ann: Annotation,/):
        ...

    ...

class SubAnnotationPropertyOf:
    sub: AnnotationProperty
    sup: AnnotationProperty
    __match_args__ = ("sub", "sup", )

    def __init__(self,sub: AnnotationProperty,sup: AnnotationProperty,/):
        ...

    ...

class AnnotationPropertyDomain:
    ap: AnnotationProperty
    iri: IRI
    __match_args__ = ("ap", "iri", )

    def __init__(self,ap: AnnotationProperty,iri: IRI,/):
        ...

    ...

class AnnotationPropertyRange:
    ap: AnnotationProperty
    iri: IRI
    __match_args__ = ("ap", "iri", )

    def __init__(self,ap: AnnotationProperty,iri: IRI,/):
        ...

    ...

class DocIRI:
    first: IRI

    __match_args__ = ("first", )

    def __init__(self,first: IRI,/):
        ...


    ...

class OntologyID:
    iri: typing.Optional[IRI]
    viri: typing.Optional[IRI]
    __match_args__ = ("iri", "viri", )

    def __init__(self,iri: typing.Optional[IRI],viri: typing.Optional[IRI],/):
        ...

    ...

class Variable:
    first: IRI

    __match_args__ = ("first", )

    def __init__(self,first: IRI,/):
        ...


    ...

DArgument = typing.Union[Literal,Variable,]

IArgument = typing.Union[Individual,Variable,]


class BuiltInAtom:
    pred: IRI
    args: typing.List[DArgument]

    __match_args__ = ("pred", "args", )


    def __init__(self,pred: IRI,args: typing.List[DArgument],):
        ...
    

    ...
class ClassAtom:
    pred: ClassExpression
    arg: IArgument

    __match_args__ = ("pred", "arg", )


    def __init__(self,pred: ClassExpression,arg: IArgument,):
        ...
    

    ...
class DataPropertyAtom:
    pred: DataProperty
    args: typing.Tuple[DArgument,DArgument]

    __match_args__ = ("pred", "args", )


    def __init__(self,pred: DataProperty,args: typing.Tuple[DArgument,DArgument],):
        ...
    

    ...
class DataRangeAtom:
    pred: DataRange
    arg: DArgument

    __match_args__ = ("pred", "arg", )


    def __init__(self,pred: DataRange,arg: DArgument,):
        ...
    

    ...
class DifferentIndividualsAtom:
    first: IArgument
    second: IArgument

    __match_args__ = ("first", "second", )


    def __init__(self,first: IArgument,second: IArgument,):
        ...
    

    ...
class ObjectPropertyAtom:
    pred: ObjectPropertyExpression
    args: typing.Tuple[IArgument,IArgument]

    __match_args__ = ("pred", "args", )


    def __init__(self,pred: ObjectPropertyExpression,args: typing.Tuple[IArgument,IArgument],):
        ...
    

    ...
class SameIndividualAtom:
    first: IArgument
    second: IArgument

    __match_args__ = ("first", "second", )


    def __init__(self,first: IArgument,second: IArgument,):
        ...
    

    ...

Atom = typing.Union[BuiltInAtom,ClassAtom,DataPropertyAtom,DataRangeAtom,DifferentIndividualsAtom,ObjectPropertyAtom,SameIndividualAtom,]

class Rule:
    head: typing.List[Atom]
    body: typing.List[Atom]
    __match_args__ = ("head", "body", )

    def __init__(self,head: typing.List[Atom],body: typing.List[Atom],/):
        ...

    ...

Component = typing.Union[OntologyID,DocIRI,OntologyAnnotation,Import,DeclareClass,DeclareObjectProperty,DeclareAnnotationProperty,DeclareDataProperty,DeclareNamedIndividual,DeclareDatatype,SubClassOf,EquivalentClasses,DisjointClasses,DisjointUnion,SubObjectPropertyOf,EquivalentObjectProperties,DisjointObjectProperties,InverseObjectProperties,ObjectPropertyDomain,ObjectPropertyRange,FunctionalObjectProperty,InverseFunctionalObjectProperty,ReflexiveObjectProperty,IrreflexiveObjectProperty,SymmetricObjectProperty,AsymmetricObjectProperty,TransitiveObjectProperty,SubDataPropertyOf,EquivalentDataProperties,DisjointDataProperties,DataPropertyDomain,DataPropertyRange,FunctionalDataProperty,DatatypeDefinition,HasKey,SameIndividual,DifferentIndividuals,ClassAssertion,ObjectPropertyAssertion,NegativeObjectPropertyAssertion,DataPropertyAssertion,NegativeDataPropertyAssertion,AnnotationAssertion,SubAnnotationPropertyOf,AnnotationPropertyDomain,AnnotationPropertyRange,Rule,]

class AnnotatedComponent:
    component: Component
    ann: typing.Set[Annotation]
    __match_args__ = ("component", "ann", )

    def __init__(self,component: Component,ann: typing.Set[Annotation],/):
        ...

    ...

