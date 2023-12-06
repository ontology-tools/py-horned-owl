from ..pyhornedowl import model
import typing

Class = model.Class
ObjectIntersectionOf = model.ObjectIntersectionOf
ObjectUnionOf = model.ObjectUnionOf
ObjectComplementOf = model.ObjectComplementOf
ObjectOneOf = model.ObjectOneOf
ObjectSomeValuesFrom = model.ObjectSomeValuesFrom
ObjectAllValuesFrom = model.ObjectAllValuesFrom
ObjectHasValue = model.ObjectHasValue
ObjectHasSelf = model.ObjectHasSelf
ObjectMinCardinality = model.ObjectMinCardinality
ObjectMaxCardinality = model.ObjectMaxCardinality
ObjectExactCardinality = model.ObjectExactCardinality
DataSomeValuesFrom = model.DataSomeValuesFrom
DataAllValuesFrom = model.DataAllValuesFrom
DataHasValue = model.DataHasValue
DataMinCardinality = model.DataMinCardinality
DataMaxCardinality = model.DataMaxCardinality
DataExactCardinality = model.DataExactCardinality
Datatype = model.Datatype
DataIntersectionOf = model.DataIntersectionOf
DataUnionOf = model.DataUnionOf
DataComplementOf = model.DataComplementOf
DataOneOf = model.DataOneOf
DatatypeRestriction = model.DatatypeRestriction
SimpleLiteral = model.SimpleLiteral
LanguageLiteral = model.LanguageLiteral
DatatypeLiteral = model.DatatypeLiteral
ObjectProperty = model.ObjectProperty
InverseObjectProperty = model.InverseObjectProperty
AnnotatedAxiom = model.AnnotatedAxiom
Annotation = model.Annotation
AnnotationAssertion = model.AnnotationAssertion
AnnotationProperty = model.AnnotationProperty
AnnotationPropertyDomain = model.AnnotationPropertyDomain
AnnotationPropertyRange = model.AnnotationPropertyRange
AnonymousIndividual = model.AnonymousIndividual
AsymmetricObjectProperty = model.AsymmetricObjectProperty
ClassAssertion = model.ClassAssertion
DataProperty = model.DataProperty
DataPropertyAssertion = model.DataPropertyAssertion
DataPropertyDomain = model.DataPropertyDomain
DataPropertyRange = model.DataPropertyRange
DatatypeDefinition = model.DatatypeDefinition
DeclareAnnotationProperty = model.DeclareAnnotationProperty
DeclareClass = model.DeclareClass
DeclareDataProperty = model.DeclareDataProperty
DeclareDatatype = model.DeclareDatatype
DeclareNamedIndividual = model.DeclareNamedIndividual
DeclareObjectProperty = model.DeclareObjectProperty
DifferentIndividuals = model.DifferentIndividuals
DisjointClasses = model.DisjointClasses
DisjointDataProperties = model.DisjointDataProperties
DisjointObjectProperties = model.DisjointObjectProperties
DisjointUnion = model.DisjointUnion
EquivalentClasses = model.EquivalentClasses
EquivalentDataProperties = model.EquivalentDataProperties
EquivalentObjectProperties = model.EquivalentObjectProperties
FacetRestriction = model.FacetRestriction
FunctionalDataProperty = model.FunctionalDataProperty
FunctionalObjectProperty = model.FunctionalObjectProperty
HasKey = model.HasKey
IRI = model.IRI
Import = model.Import
InverseFunctionalObjectProperty = model.InverseFunctionalObjectProperty
InverseObjectProperties = model.InverseObjectProperties
IrreflexiveObjectProperty = model.IrreflexiveObjectProperty
NamedIndividual = model.NamedIndividual
NegativeDataPropertyAssertion = model.NegativeDataPropertyAssertion
NegativeObjectPropertyAssertion = model.NegativeObjectPropertyAssertion
ObjectPropertyAssertion = model.ObjectPropertyAssertion
ObjectPropertyDomain = model.ObjectPropertyDomain
ObjectPropertyRange = model.ObjectPropertyRange
OntologyAnnotation = model.OntologyAnnotation
ReflexiveObjectProperty = model.ReflexiveObjectProperty
SameIndividual = model.SameIndividual
SubAnnotationPropertyOf = model.SubAnnotationPropertyOf
SubClassOf = model.SubClassOf
SubDataPropertyOf = model.SubDataPropertyOf
SubObjectPropertyOf = model.SubObjectPropertyOf
SymmetricObjectProperty = model.SymmetricObjectProperty
TransitiveObjectProperty = model.TransitiveObjectProperty
Facet = model.Facet
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


__all__ = ['Class', 'ObjectIntersectionOf', 'ObjectUnionOf', 'ObjectComplementOf', 'ObjectOneOf', 'ObjectSomeValuesFrom', 'ObjectAllValuesFrom', 'ObjectHasValue', 'ObjectHasSelf', 'ObjectMinCardinality', 'ObjectMaxCardinality', 'ObjectExactCardinality', 'DataSomeValuesFrom', 'DataAllValuesFrom', 'DataHasValue', 'DataMinCardinality', 'DataMaxCardinality', 'DataExactCardinality', 'Datatype', 'DataIntersectionOf', 'DataUnionOf', 'DataComplementOf', 'DataOneOf', 'DatatypeRestriction', 'SimpleLiteral', 'LanguageLiteral', 'DatatypeLiteral', 'ObjectProperty', 'InverseObjectProperty', 'AnnotatedAxiom', 'Annotation', 'AnnotationAssertion', 'AnnotationProperty', 'AnnotationPropertyDomain', 'AnnotationPropertyRange', 'AnonymousIndividual', 'AsymmetricObjectProperty', 'ClassAssertion', 'DataProperty', 'DataPropertyAssertion', 'DataPropertyDomain', 'DataPropertyRange', 'DatatypeDefinition', 'DeclareAnnotationProperty', 'DeclareClass', 'DeclareDataProperty', 'DeclareDatatype', 'DeclareNamedIndividual', 'DeclareObjectProperty', 'DifferentIndividuals', 'DisjointClasses', 'DisjointDataProperties', 'DisjointObjectProperties', 'DisjointUnion', 'EquivalentClasses', 'EquivalentDataProperties', 'EquivalentObjectProperties', 'FacetRestriction', 'FunctionalDataProperty', 'FunctionalObjectProperty', 'HasKey', 'IRI', 'Import', 'InverseFunctionalObjectProperty', 'InverseObjectProperties', 'IrreflexiveObjectProperty', 'NamedIndividual', 'NegativeDataPropertyAssertion', 'NegativeObjectPropertyAssertion', 'ObjectPropertyAssertion', 'ObjectPropertyDomain', 'ObjectPropertyRange', 'OntologyAnnotation', 'ReflexiveObjectProperty', 'SameIndividual', 'SubAnnotationPropertyOf', 'SubClassOf', 'SubDataPropertyOf', 'SubObjectPropertyOf', 'SymmetricObjectProperty', 'TransitiveObjectProperty', 'Facet', 'ClassExpression', 'ObjectPropertyExpression', 'Literal', 'DataRange', 'Individual', 'PropertyExpression', 'AnnotationSubject', 'AnnotationValue', 'SubObjectPropertyExpression', 'Axiom']