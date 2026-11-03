# Private names only besides the model, which `import *` exports.
import typing as _typing

from .pyhornedowl.model import *


@_typing.runtime_checkable
class NamedEntity(_typing.Protocol):
    def __owl_entity_iri__(self) -> IRI: ...


IRIParam = IRI | NamedEntity | str | tuple[str, bool]
PropertyExpression = ObjectPropertyExpression | DataProperty | AnnotationProperty
SubObjectPropertyExpression = ObjectPropertyExpression | list[ObjectPropertyExpression]
AnnotationSubject = IRI | AnonymousIndividual
AnnotationValue = Literal | IRI | AnonymousIndividual
DArgument = Literal | Variable
IArgument = Individual | Variable
