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