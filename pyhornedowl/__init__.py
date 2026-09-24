import sys

from . import pyhornedowl as _native

# PyO3 submodules cannot be imported by their dotted path. Registering them lets
# the subpackages below re-export them with `from ..pyhornedowl.<name> import *`.
for _name in ("model", "reasoning", "profile"):
    sys.modules[f"{__name__}.pyhornedowl.{_name}"] = getattr(_native, _name)

from . import model, profile, reasoning
