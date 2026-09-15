"""Tests for OWL 2 profile conformance checking (pyhornedowl.profile)."""
import pytest

import pyhornedowl
from pyhornedowl.model import Class, IRI, DeclareClass, SubClassOf
from pyhornedowl.profile import conformant_profiles, check_profile, ProfileReport

EX = "https://example.com/"


def simple_ontology() -> pyhornedowl.PyIndexedOntology:
    """Two named classes with a single SubClassOf — conformant to every profile."""
    onto = pyhornedowl.PyIndexedOntology()
    onto.prefix_mapping.add_default_prefix_names()
    onto.prefix_mapping.add_prefix("", EX)
    a = Class(IRI.parse(EX + "A"))
    b = Class(IRI.parse(EX + "B"))
    onto.add_component(DeclareClass(a))
    onto.add_component(DeclareClass(b))
    onto.add_component(SubClassOf(a, b))
    return onto


def test_conformant_profiles_lists_profiles():
    profiles = conformant_profiles(simple_ontology())
    # A simple named-subclass ontology conforms to OWL 2 DL and all three tractable profiles.
    assert set(profiles) == {"OWL2DL", "EL", "QL", "RL"}


def test_check_profile_returns_report():
    r = check_profile(simple_ontology(), "EL")
    assert isinstance(r, ProfileReport)
    assert r.profile == "EL"
    assert r.conformant is True
    assert r.num_violations == 0
    assert r.violations_by_kind == {}


def test_check_profile_is_case_insensitive_and_dl_alias():
    assert check_profile(simple_ontology(), "el").profile == "EL"
    assert check_profile(simple_ontology(), "dl").profile == "OWL2DL"


def test_check_profile_rejects_unknown_profile():
    with pytest.raises(ValueError):
        check_profile(simple_ontology(), "bogus")
