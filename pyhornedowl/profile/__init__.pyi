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


def check_profile(ontology: PyIndexedOntology, profile: typing.Union[Profile, Literal["DL", "EL", "QL", "RL"]]) -> ProfileReport:
    """
    Checks `ontology` against a single profile and returns a `ProfileReport` with
    conformance and the violations found.
    
    :param PyIndexedOntology ontology: the ontology to check
    :param typing.Union[Profile, Literal["DL", "EL", "QL", "RL"]] profile: the profile to check against, either a `Profile` member or one of the strings "DL"/"OWL2DL", "EL", "QL", "RL" (case-insensitive)
    """
    ...


class Violation:
    """
    Base class for profile violations
    """

class Profile:
    """
    OWL 2 profiles
    
    """
    EL: typing.Self
    """
    The [OWL 2 EL profile](https://www.w3.org/TR/owl2-profiles/#OWL_2_EL).
    """
    OWL2DL: typing.Self
    """
    The [OWL 2 DL profile](https://www.w3.org/TR/owl2-syntax/#Global_Restrictions_on_Axioms_in_OWL_2_DL).
    """
    QL: typing.Self
    """
    The [OWL 2 QL profile](https://www.w3.org/TR/owl2-profiles/#OWL_2_QL).
    """
    RL: typing.Self
    """
    The [OWL 2 RL profile](https://www.w3.org/TR/owl2-profiles/#OWL_2_RL).
    """

class ProfileReport:
    """
    The result of checking an ontology against one OWL 2 profile.
    """
    conformant: bool
    """
    True if the ontology has no violations of this profile.
    """

    profile: Profile
    """
    The profile this report is for.
    """

    violations: typing.List[Violation]
    """
    Every violation found, as `Violation` subclass instances.
    """


class UseOfNonAtomicClassExpression(Violation):
    """
    A class expression is not *atomic* where OWL 2 DL requires one to be.
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


class UseOfNonSubClassExpression(Violation):
    """
    `ce` is not legal in the profile's subclass-position grammar.
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


class UseOfNonSuperClassExpression(Violation):
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


class UseOfIllegalClassExpression(Violation):
    """
    A class expression is not legal in OWL 2 DL at all, in any position.
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


class UseOfClassExpressionWithTooFewOperands(Violation):
    """
    `ObjectUnionOf`/`ObjectIntersectionOf` built with fewer than the two
    operands OWL 2 DL requires for these n-ary constructors.
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


class UseOfDataRangeWithTooFewOperands(Violation):
    """
    `DataUnionOf`/`DataIntersectionOf` built with fewer than two operands.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    dr: DataRange
    """
    The offending data range.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfBuiltinDatatypeInDatatypeDefinition(Violation):
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


class UseOfNonSimplePropertyInObjectHasSelf(Violation):
    """
    `ObjectHasSelf` is given a composite (non-simple) object property.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """

    ope: ObjectPropertyExpression
    """
    The composite (non-simple) object property expression.
    """


class UseOfNonSimplePropertyInCardinalityRestriction(Violation):
    """
    An object-cardinality restriction is given a composite (non-simple)
    object property.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """

    ope: ObjectPropertyExpression
    """
    The composite (non-simple) object property expression.
    """


class UseOfNonSimplePropertyInDisjointPropertiesAxiom(Violation):
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


class UseOfNonSimplePropertyInIrreflexivePropertyAxiom(Violation):
    """
    `IrreflexiveObjectProperty` is given a composite (non-simple) object
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


class UseOfNonSimplePropertyInAsymmetricPropertyAxiom(Violation):
    """
    `AsymmetricObjectProperty` is given a composite (non-simple) object
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


class UseOfNonSimplePropertyInFunctionalPropertyAxiom(Violation):
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


class UseOfNonSimplePropertyInInverseFunctionalPropertyAxiom(Violation):
    """
    `InverseFunctionalObjectProperty` is given a composite (non-simple)
    object property.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfPropertyInChainCausingCycle(Violation):
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


class UseOfUndeclaredClass(Violation):
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


class UseOfUndeclaredObjectProperty(Violation):
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


class UseOfUndeclaredDataProperty(Violation):
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


class UseOfUndeclaredAnnotationProperty(Violation):
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


class UseOfUndeclaredDatatype(Violation):
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


class UseOfIllegalPunning(Violation):
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


class UseOfReservedVocabulary(Violation):
    """
    A reserved `rdf:`/`rdfs:`/`owl:` structural vocabulary term is the
    subject of a `Declare*` axiom.
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
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """


class UseOfObjectPropertyInverse(Violation):
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


class UseOfIllegalAxiomKind(Violation):
    """
    This whole axiom *kind* is not permitted in the profile, regardless of
    its content -- e.g. `FunctionalObjectProperty` in EL.
    """
    axiom: AnnotatedComponent
    """
    The axiom the violation was found in.
    """

    message: str
    """
    A human readable description of the violation.
    """

    reason: str
    """
    Which profile rule the axiom kind falls foul of.
    """


