import typing
from typing import *
from .. import PyIndexedOntology
from ..model import *

def conformant_profiles(ontology: PyIndexedOntology) -> List[str]:
    """
    Returns every OWL 2 profile the ontology conforms to, in declaration order
    (OWL2DL, EL, QL, RL). The profiles overlap, so more than one may be returned.
    """
    ...


def check_profile(ontology: PyIndexedOntology, profile: str) -> ProfileReport:
    """
    Checks `ontology` against a single profile ("DL"/"OWL2DL", "EL", "QL", "RL",
    case-insensitive) and returns a `ProfileReport` with conformance and
    per-kind violation counts.
    """
    ...


class ProfileReport:
    """
    The result of checking an ontology against one OWL 2 profile.
    """

