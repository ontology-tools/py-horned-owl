# Private names only besides the API, which `import *` exports.
import sys as _sys
import typing as _typing
import warnings as _warnings

from . import pyhornedowl as _native

for _name in ("model", "reasoning", "profile"):
    _sys.modules[f"{__name__}.pyhornedowl.{_name}"] = getattr(_native, _name)

from .pyhornedowl import *

if _typing.TYPE_CHECKING:
    PyIndexedOntology = Ontology
else:
    def __getattr__(name: str):
        if name == "PyIndexedOntology":
            _warnings.warn(
                "PyIndexedOntology was renamed to Ontology and will be removed in a future version",
                DeprecationWarning,
                stacklevel=2,
            )
            return Ontology
        raise AttributeError(f"module {__name__!r} has no attribute {name!r}")

del model, reasoning, profile
from . import model as model, reasoning as reasoning, profile as profile
