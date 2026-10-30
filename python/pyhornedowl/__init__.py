# Private names only besides the API, which `import *` exports.
import sys as _sys

from . import pyhornedowl as _native

for _name in ("model", "reasoning", "profile"):
    _sys.modules[f"{__name__}.pyhornedowl.{_name}"] = getattr(_native, _name)

from .pyhornedowl import *

PyIndexedOntology = Ontology

del model, reasoning, profile
from . import model as model, reasoning as reasoning, profile as profile
