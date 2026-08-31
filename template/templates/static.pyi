import typing
from typing import *

from .. import PrefixMapping

class IRI:
    parse: Any

    def serialize(self, serialization: typing.Literal['omn', 'ofn']='omn', prefix_mapping: typing.Optional[PrefixMapping]=None) -> str:
        """
        Renders this element on its own, in OWL 2 Manchester (`omn`) or OWL functional
        (`ofn`) syntax. The per-element counterpart to `PyIndexedOntology.save_to_string`.
        """
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

    def serialize(self, serialization: typing.Literal['ofn']='ofn', prefix_mapping: typing.Optional[PrefixMapping]=None) -> str:
        """
        Renders this element on its own, in OWL functional (`ofn`) syntax.

        horned-owl has no Manchester rendering for a facet: Manchester writes it as an
        operator inside the datatype restriction that holds it.
        """
        ...
    

IRIParam = Union[str, IRI, Tuple[str, bool]]
"""
One of the following:
- A string representing an IRI, e.g. "https://example.com/A"
- A string representing a CURIE, e.g. "ex:A", which will be expanded using the prefix mapping
- An IRI object
- A tuple of a string and a boolean, where the boolean indicates whether the string is an absolute IRI (True) or a CURIE (False)
"""