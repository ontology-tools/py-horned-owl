# The classes live in the native module; this package adds what PyO3 cannot
# express: the union type aliases and the NamedEntity protocol.
import typing

from ..pyhornedowl.model import *

@typing.runtime_checkable
class NamedEntity(typing.Protocol):
    """A named OWL entity, such as a `Class`.

    Native entities and external wrappers opt in through
    `__owl_entity_iri__()`, which must return an `IRI`. An ordinary `iri`
    attribute alone does not make an object a named entity.

    A protocol rather than a base class, because PyO3 classes have a single
    base class and the entities already have theirs (`Class` is a
    `ClassExpression`). Runtime protocol checks test method presence;
    conversion also validates its return value.
    """

    def __owl_entity_iri__(self) -> IRI: ...


IRIParam = typing.Union[IRI, NamedEntity, str, typing.Tuple[str, bool]]


# The unions of model types that are not a class hierarchy, because their
# members belong to other hierarchies already. Mirrors the `#[model_union]`
# enums in src/model/.
PropertyExpression = typing.Union[ObjectPropertyExpression, DataProperty, AnnotationProperty]
SubObjectPropertyExpression = typing.Union[ObjectPropertyExpression, typing.List[ObjectPropertyExpression]]
AnnotationSubject = typing.Union[IRI, AnonymousIndividual]
AnnotationValue = typing.Union[Literal, IRI, AnonymousIndividual]
DArgument = typing.Union[Literal, Variable]
IArgument = typing.Union[Individual, Variable]
