"""Tests for OWL 2 profile conformance checking (pyhornedowl.profile)."""
import pytest

import pyhornedowl
from pyhornedowl.model import Class, IRI, DeclareClass, ObjectUnionOf, SubClassOf
from pyhornedowl.profile import (
    Profile,
    ProfileReport,
    ProfileViolation,
    UseOfNonSubClassExpression,
    UseOfUndeclaredClass,
    check_profile,
    conformant_profiles,
)

EX = "https://example.com/"


def _ontology(*names: str) -> pyhornedowl.PyIndexedOntology:
    onto = pyhornedowl.PyIndexedOntology()
    onto.prefix_mapping.add_default_prefix_names()
    onto.prefix_mapping.add_prefix("", EX)
    for n in names:
        onto.add_component(DeclareClass(Class(IRI.parse(EX + n))))
    return onto


def simple_ontology() -> pyhornedowl.PyIndexedOntology:
    """Two named classes with a single SubClassOf — conformant to every profile."""
    onto = _ontology("A", "B")
    onto.add_component(SubClassOf(Class(IRI.parse(EX + "A")), Class(IRI.parse(EX + "B"))))
    return onto


def union_ontology() -> pyhornedowl.PyIndexedOntology:
    """`A or B SubClassOf C` — legal in OWL 2 DL and RL, but not in EL or QL."""
    onto = _ontology("A", "B", "C")
    union = ObjectUnionOf([Class(IRI.parse(EX + "A")), Class(IRI.parse(EX + "B"))])
    onto.add_component(SubClassOf(union, Class(IRI.parse(EX + "C"))))
    return onto


def test_conformant_profiles_lists_profiles():
    profiles = conformant_profiles(simple_ontology())
    # A simple named-subclass ontology conforms to OWL 2 DL and all three tractable profiles.
    assert set(profiles) == {Profile.OWL2DL, Profile.EL, Profile.QL, Profile.RL}


def test_check_profile_returns_report():
    r = check_profile(simple_ontology(), "EL")
    assert isinstance(r, ProfileReport)
    assert r.profile == Profile.EL
    assert r.conformant is True
    assert len(r) == 0
    assert r.violations == []
    assert r.violations_by_kind == {}


def test_check_profile_accepts_profile_enum():
    assert check_profile(simple_ontology(), Profile.QL).profile == Profile.QL


def test_check_profile_is_case_insensitive_and_dl_alias():
    assert check_profile(simple_ontology(), "el").profile == Profile.EL
    assert check_profile(simple_ontology(), "dl").profile == Profile.OWL2DL


def test_check_profile_rejects_unknown_profile():
    with pytest.raises(ValueError):
        check_profile(simple_ontology(), "bogus")


def test_violations_are_typed_subclasses():
    onto = union_ontology()
    assert conformant_profiles(onto) == [Profile.OWL2DL, Profile.RL]

    r = check_profile(onto, Profile.EL)
    assert r.conformant is False
    assert r.violations_by_kind == {"UseOfNonSubClassExpression": 1}

    (v,) = r.violations
    assert isinstance(v, UseOfNonSubClassExpression)
    assert isinstance(v, ProfileViolation)
    # The base class carries the offending axiom, the subclass its own payload.
    assert v.axiom.component == SubClassOf(
        ObjectUnionOf([Class(IRI.parse(EX + "A")), Class(IRI.parse(EX + "B"))]),
        Class(IRI.parse(EX + "C")),
    )
    assert v.ce == ObjectUnionOf([Class(IRI.parse(EX + "A")), Class(IRI.parse(EX + "B"))])
    assert "subclass position" in v.message
    assert str(v) == v.message
    assert repr(v).startswith("UseOfNonSubClassExpression(axiom=")


def test_violation_exposes_only_the_fields_it_has():
    # An undeclared class is not attributable to a single axiom, so
    # UseOfUndeclaredClass carries an `iri` and no `axiom` at all.
    onto = pyhornedowl.PyIndexedOntology()
    onto.prefix_mapping.add_default_prefix_names()
    onto.add_component(
        SubClassOf(Class(IRI.parse(EX + "A")), Class(IRI.parse(EX + "B")))
    )

    violations = check_profile(onto, Profile.OWL2DL).violations
    undeclared = [v for v in violations if isinstance(v, UseOfUndeclaredClass)]
    assert undeclared, [type(v).__name__ for v in violations]
    assert not hasattr(undeclared[0], "axiom")
    assert {str(v.iri) for v in undeclared} == {EX + "A", EX + "B"}
    assert repr(undeclared[0]).startswith("UseOfUndeclaredClass(iri=")


def test_base_class_carries_no_data():
    # ProfileViolation is a pure type collector: no fields of its own.
    assert not hasattr(ProfileViolation, "axiom")
    assert not hasattr(ProfileViolation, "message")
