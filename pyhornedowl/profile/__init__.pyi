import typing
from typing import *
from .. import PyIndexedOntology
from ..model import *

def conformant_profiles(ontology: PyIndexedOntology) -> List[Profile]:
    """
    Returns every OWL 2 profile the ontology conforms to, in declaration order
    (OWL2DL, EL, QL, RL). The profiles overlap, so more than one may be returned.
    """
    ...


def check_profile(ontology: PyIndexedOntology, profile: Profile) -> ProfileReport:
    """
    Checks `ontology` against a single profile -- a `Profile` member or one of
    the strings "DL"/"OWL2DL", "EL", "QL", "RL" (case-insensitive) -- and
    returns a `ProfileReport` with conformance and the violations found.
    """
    ...


class Profile:
    OWL2DL: typing.Self
    EL: typing.Self
    QL: typing.Self
    RL: typing.Self

class ProfileReport:
    """
    The result of checking an ontology against one OWL 2 profile.
    """
    def __len__(self, /):
        ...

    violations: typing.List[ProfileViolation]
    """
    Every violation found, as `ProfileViolation` subclass instances.
    """

    conformant: bool
    """
    True if the ontology has no violations of this profile.
    """

    profile: Profile
    """
    The profile this report is for.
    """

    violations_by_kind: typing.Dict[str, int]
    """
    Violation counts grouped by violation class name, e.g.
    `{"UseOfNonSubClassExpression": 3}`.
    """


class ProfileViolation:
    """
    Marker base class of every profile violation, mirroring the OWL API's
    `OWLProfileViolation`.
    
    Carries no data of its own: it exists so that every violation is one type in
    Python (`isinstance(v, ProfileViolation)`, `List[ProfileViolation]`). The
    payload lives on the subclasses, each of which exposes only the fields its
    own violation actually has -- an `axiom` getter, for instance, appears only
    where the violation is attributable to a single axiom.
    """

class UseOfNonAtomicClassExpression(ProfileViolation):
    """
    A class expression is not *atomic* where OWL 2 DL requires one to be.
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


class UseOfNonSubClassExpression(ProfileViolation):
    """
    `ce` is not legal in the profile's subclass-position grammar.
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


class UseOfNonSuperClassExpression(ProfileViolation):
    """
    `ce` is not legal in the profile's superclass-position grammar.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    ce: ClassExpression
    """
    The offending class expression.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfIllegalClassExpression(ProfileViolation):
    """
    A class expression is not legal in OWL 2 DL at all, in any position.
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


class UseOfClassExpressionWithTooFewOperands(ProfileViolation):
    """
    `ObjectUnionOf`/`ObjectIntersectionOf` built with fewer than the two
    operands OWL 2 DL requires for these n-ary constructors.
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


class UseOfDataRangeWithTooFewOperands(ProfileViolation):
    """
    `DataUnionOf`/`DataIntersectionOf` built with fewer than two operands.
    """
    message: str
    """
    A human readable description of the violation.
    """

    dr: DataRange
    """
    The offending data range.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfBuiltinDatatypeInDatatypeDefinition(ProfileViolation):
    """
    A `DatatypeDefinition` whose subject is itself a built-in XSD/OWL 2
    datatype -- `DatatypeDefinition` introduces a *new* datatype, it cannot
    redefine an existing built-in one.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfNonSimplePropertyInObjectHasSelf(ProfileViolation):
    """
    `ObjectHasSelf` is given a composite (non-simple) object property.
    """
    message: str
    """
    A human readable description of the violation.
    """

    ope: ObjectPropertyExpression
    """
    The composite (non-simple) object property expression.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfNonSimplePropertyInCardinalityRestriction(ProfileViolation):
    """
    An object-cardinality restriction is given a composite (non-simple)
    object property.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    ope: ObjectPropertyExpression
    """
    The composite (non-simple) object property expression.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfNonSimplePropertyInDisjointPropertiesAxiom(ProfileViolation):
    """
    `DisjointObjectProperties` includes a composite (non-simple) object
    property.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfNonSimplePropertyInIrreflexivePropertyAxiom(ProfileViolation):
    """
    `IrreflexiveObjectProperty` is given a composite (non-simple) object
    property.
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfNonSimplePropertyInAsymmetricPropertyAxiom(ProfileViolation):
    """
    `AsymmetricObjectProperty` is given a composite (non-simple) object
    property.
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfNonSimplePropertyInFunctionalPropertyAxiom(ProfileViolation):
    """
    `FunctionalObjectProperty` is given a composite (non-simple) object
    property.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfNonSimplePropertyInInverseFunctionalPropertyAxiom(ProfileViolation):
    """
    `InverseFunctionalObjectProperty` is given a composite (non-simple)
    object property.
    """
    message: str
    """
    A human readable description of the violation.
    """

    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """


class UseOfPropertyInChainCausingCycle(ProfileViolation):
    """
    A cycle in the role hierarchy's property-chain graph. Spans potentially
    many chain axioms, so -- unlike most violations -- it has no `axiom`.
    """
    cycle: typing.List[ObjectProperty]
    """
    The properties forming the cycle, the first repeated as the last to close the loop.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfUndeclaredClass(ProfileViolation):
    """
    `iri` is used as a class without a `DeclareClass` axiom.
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfUndeclaredObjectProperty(ProfileViolation):
    """
    `iri` is used as an object property without a `DeclareObjectProperty`
    axiom.
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfUndeclaredDataProperty(ProfileViolation):
    """
    `iri` is used as a data property without a `DeclareDataProperty` axiom.
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfUndeclaredAnnotationProperty(ProfileViolation):
    """
    `iri` is used as an annotation property without a
    `DeclareAnnotationProperty` axiom.
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfUndeclaredDatatype(ProfileViolation):
    """
    `iri` is used as a datatype without a `DeclareDatatype` axiom.
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfIllegalPunning(ProfileViolation):
    """
    `iri` is declared as more than one mutually-exclusive entity kind.
    """
    iri: IRI
    """
    The IRI the violation is about.
    """

    kinds: typing.List[str]
    """
    The mutually-exclusive entity kinds the IRI is declared as.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfReservedVocabulary(ProfileViolation):
    """
    A reserved `rdf:`/`rdfs:`/`owl:` structural vocabulary term is the
    subject of a `Declare*` axiom.
    """
    message: str
    """
    A human readable description of the violation.
    """

    iri: IRI
    """
    The IRI the violation is about.
    """


class UseOfDataOneOfWithMultipleLiterals(ProfileViolation):
    """
    EL-specific: `DataOneOf` with more than one literal.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfObjectPropertyInverse(ProfileViolation):
    """
    EL-specific: object property inverses are not permitted at all.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfIllegalAxiomKind(ProfileViolation):
    """
    This whole axiom *kind* is not permitted in the profile, regardless of
    its content -- e.g. `FunctionalObjectProperty` in EL.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    reason: str
    """
    Which profile rule the axiom kind falls foul of.
    """

    message: str
    """
    A human readable description of the violation.
    """


