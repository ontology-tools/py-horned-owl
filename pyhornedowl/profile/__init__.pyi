import typing
from typing import *
from .. import PyIndexedOntology
from ..model import *

def conformant_profiles(ontology: PyIndexedOntology) -> List[Profile]:
    """
    Returns every OWL 2 profile the ontology conforms to, in declaration order
    (OWL2DL, EL, QL, RL). The profiles overlap, so more than one may be returned.
    
    :param PyIndexedOntology ontology: the ontology to check
    """
    ...


def check_profile(ontology: PyIndexedOntology, profile: Profile) -> ProfileReport:
    """
    Checks `ontology` against a single profile and returns a `ProfileReport` with
    conformance and the violations found.
    
    :param PyIndexedOntology ontology: the ontology to check
    :param Profile profile: the profile to check against, either a `Profile` member or one of the strings "DL"/"OWL2DL", "EL", "QL", "RL" (case-insensitive)
    """
    ...


class Violation:
    """
    Marker base class of every profile violation, mirroring the OWL API's
    `OWLProfileViolation`.
    
    Carries no data of its own: it exists so that every violation is one type in
    Python. The payload lives on the subclasses, each of which exposes only the
    fields its own violation actually has.
    """

class Profile:
    """
    An OWL 2 profile to check conformance against.
    
    The profiles overlap rather than forming a linear ladder, so an ontology can
    conform to several at once. Wherever one is expected, its name is also
    accepted as a case-insensitive string.
    
    """
    OWL2DL: typing.Self
    """
    Plain OWL 2 DL -- the global restrictions, and a prerequisite for the three sub-profiles
    """
    EL: typing.Self
    """
    The OWL 2 EL profile
    """
    QL: typing.Self
    """
    The OWL 2 QL profile
    """
    RL: typing.Self
    """
    The OWL 2 RL profile
    """

class ProfileReport:
    """
    The result of checking an ontology against one OWL 2 profile.
    
    :ivar profile: The profile this report is for.
    :vartype profile: Profile
    :ivar conformant: True if the ontology has no violations of this profile.
    :vartype conformant: bool
    :ivar violations: Every violation found, as `Violation` subclass instances.
    :vartype violations: typing.List[Violation]
    """
    violations: typing.List[Violation]
    """
    Every violation found, as `Violation` subclass instances.
    """

    conformant: bool
    """
    True if the ontology has no violations of this profile.
    """

    profile: Profile
    """
    The profile this report is for.
    """


class UseOfNonAtomicClassExpression(Violation):
    """
    A class expression is not *atomic* where OWL 2 DL requires one to be.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar ce: The offending class expression.
    :vartype ce: ClassExpression
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    ce: ClassExpression
    """
    The offending class expression.
    """


class UseOfNonSubClassExpression(Violation):
    """
    `ce` is not legal in the profile's subclass-position grammar.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar ce: The offending class expression.
    :vartype ce: ClassExpression
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    ce: ClassExpression
    """
    The offending class expression.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfNonSuperClassExpression(Violation):
    """
    `ce` is not legal in the profile's superclass-position grammar.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar ce: The offending class expression.
    :vartype ce: ClassExpression
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    ce: ClassExpression
    """
    The offending class expression.
    """

    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfIllegalClassExpression(Violation):
    """
    A class expression is not legal in OWL 2 DL at all, in any position.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar ce: The offending class expression.
    :vartype ce: ClassExpression
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    ce: ClassExpression
    """
    The offending class expression.
    """


class UseOfClassExpressionWithTooFewOperands(Violation):
    """
    `ObjectUnionOf`/`ObjectIntersectionOf` built with fewer than the two
    operands OWL 2 DL requires for these n-ary constructors.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar ce: The offending class expression.
    :vartype ce: ClassExpression
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    ce: ClassExpression
    """
    The offending class expression.
    """


class UseOfDataRangeWithTooFewOperands(Violation):
    """
    `DataUnionOf`/`DataIntersectionOf` built with fewer than two operands.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar dr: The offending data range.
    :vartype dr: DataRange
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    dr: DataRange
    """
    The offending data range.
    """


class UseOfBuiltinDatatypeInDatatypeDefinition(Violation):
    """
    A `DatatypeDefinition` whose subject is itself a built-in XSD/OWL 2
    datatype -- `DatatypeDefinition` introduces a *new* datatype, it cannot
    redefine an existing built-in one.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfNonSimplePropertyInObjectHasSelf(Violation):
    """
    `ObjectHasSelf` is given a composite (non-simple) object property.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar ope: The composite (non-simple) object property expression.
    :vartype ope: ObjectPropertyExpression
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    ope: ObjectPropertyExpression
    """
    The composite (non-simple) object property expression.
    """


class UseOfNonSimplePropertyInCardinalityRestriction(Violation):
    """
    An object-cardinality restriction is given a composite (non-simple)
    object property.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar ope: The composite (non-simple) object property expression.
    :vartype ope: ObjectPropertyExpression
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    ope: ObjectPropertyExpression
    """
    The composite (non-simple) object property expression.
    """

    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfNonSimplePropertyInDisjointPropertiesAxiom(Violation):
    """
    `DisjointObjectProperties` includes a composite (non-simple) object
    property.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfNonSimplePropertyInIrreflexivePropertyAxiom(Violation):
    """
    `IrreflexiveObjectProperty` is given a composite (non-simple) object
    property.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfNonSimplePropertyInAsymmetricPropertyAxiom(Violation):
    """
    `AsymmetricObjectProperty` is given a composite (non-simple) object
    property.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfNonSimplePropertyInFunctionalPropertyAxiom(Violation):
    """
    `FunctionalObjectProperty` is given a composite (non-simple) object
    property.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfNonSimplePropertyInInverseFunctionalPropertyAxiom(Violation):
    """
    `InverseFunctionalObjectProperty` is given a composite (non-simple)
    object property.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfPropertyInChainCausingCycle(Violation):
    """
    A cycle in the role hierarchy's property-chain graph. Spans potentially
    many chain axioms, so -- unlike most violations -- it has no `axiom`.
    
    :ivar cycle: The properties forming the cycle, the first repeated as the last to close the loop.
    :vartype cycle: typing.List[ObjectProperty]
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    cycle: typing.List[ObjectProperty]
    """
    The properties forming the cycle, the first repeated as the last to close the loop.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfUndeclaredClass(Violation):
    """
    `iri` is used as a class without a `DeclareClass` axiom.
    
    :ivar iri: The IRI the violation is about.
    :vartype iri: IRI
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfUndeclaredObjectProperty(Violation):
    """
    `iri` is used as an object property without a `DeclareObjectProperty`
    axiom.
    
    :ivar iri: The IRI the violation is about.
    :vartype iri: IRI
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    iri: IRI
    """
    The IRI the violation is about.
    """


class UseOfUndeclaredDataProperty(Violation):
    """
    `iri` is used as a data property without a `DeclareDataProperty` axiom.
    
    :ivar iri: The IRI the violation is about.
    :vartype iri: IRI
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfUndeclaredAnnotationProperty(Violation):
    """
    `iri` is used as an annotation property without a
    `DeclareAnnotationProperty` axiom.
    
    :ivar iri: The IRI the violation is about.
    :vartype iri: IRI
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    iri: IRI
    """
    The IRI the violation is about.
    """


class UseOfUndeclaredDatatype(Violation):
    """
    `iri` is used as a datatype without a `DeclareDatatype` axiom.
    
    :ivar iri: The IRI the violation is about.
    :vartype iri: IRI
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    iri: IRI
    """
    The IRI the violation is about.
    """


class UseOfIllegalPunning(Violation):
    """
    `iri` is declared as more than one mutually-exclusive entity kind.
    
    :ivar iri: The IRI the violation is about.
    :vartype iri: IRI
    :ivar kinds: The mutually-exclusive entity kinds the IRI is declared as.
    :vartype kinds: typing.List[str]
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """

    kinds: typing.List[str]
    """
    The mutually-exclusive entity kinds the IRI is declared as.
    """


class UseOfReservedVocabulary(Violation):
    """
    A reserved `rdf:`/`rdfs:`/`owl:` structural vocabulary term is the
    subject of a `Declare*` axiom.
    
    :ivar iri: The IRI the violation is about.
    :vartype iri: IRI
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfDataOneOfWithMultipleLiterals(Violation):
    """
    EL-specific: `DataOneOf` with more than one literal.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfObjectPropertyInverse(Violation):
    """
    EL-specific: object property inverses are not permitted at all.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfIllegalAxiomKind(Violation):
    """
    This whole axiom *kind* is not permitted in the profile, regardless of
    its content -- e.g. `FunctionalObjectProperty` in EL.
    
    :ivar axiom: The axiom the violation was found in.
    :vartype axiom: AnnotatedComponent
    :ivar reason: Which profile rule the axiom kind falls foul of.
    :vartype reason: str
    :ivar message: A human readable description of the violation.
    :vartype message: str
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    reason: str
    """
    Which profile rule the axiom kind falls foul of.
    """


