import sys

from . import pyhornedowl as _native

for _name in ("model", "reasoning", "profile"):
    sys.modules[f"{__name__}.pyhornedowl.{_name}"] = getattr(_native, _name)

from .pyhornedowl import *

PyIndexedOntology = Ontology

del model, reasoning, profile
from . import model as model, reasoning as reasoning, profile as profile
