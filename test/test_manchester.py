"""Per-element Manchester syntax rendering (`pyhornedowl.to_manchester`)."""

import pytest

import pyhornedowl
from pyhornedowl.model import (
    Class,
    ClassAssertion,
    DeclareClass,
    IRI,
    NamedIndividual,
    ObjectProperty,
    ObjectSomeValuesFrom,
    SubClassOf,
)
from test_base import r, res, simple_ontology


def c(iri: str) -> Class:
    return Class(IRI.parse(iri))


A = "http://example.com/A"
B = "http://example.com/B"
R = "http://example.com/r"


def test_renders_named_subclass_axiom():
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert pyhornedowl.to_manchester(ax) == f"<{A}> SubClassOf <{B}>"


def test_renders_existential_restriction():
    ax = SubClassOf(
        sub=c(A),
        sup=ObjectSomeValuesFrom(ope=ObjectProperty(IRI.parse(R)), bce=c(B)),
    )

    assert pyhornedowl.to_manchester(ax) == f"<{A}> SubClassOf <{R}> some <{B}>"


def test_renders_class_expression_alone():
    ce = ObjectSomeValuesFrom(ope=ObjectProperty(IRI.parse(R)), bce=c(B))

    assert pyhornedowl.to_manchester(ce) == f"<{R}> some <{B}>"


def test_abbreviates_with_prefix_mapping():
    onto = pyhornedowl.open_ontology(res("simple.owl"))
    onto.add_prefix_mapping("ex", "http://example.com/")
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert pyhornedowl.to_manchester(ax, onto.prefix_mapping) == "ex:A SubClassOf ex:B"


def test_accepts_annotated_component_from_get_axioms():
    onto = simple_ontology()

    rendered = [pyhornedowl.to_manchester(ac) for ac in onto.get_axioms()]

    assert rendered, "expected at least one axiom"
    assert all(isinstance(s, str) and s for s in rendered)


def test_renders_every_axiom_of_a_real_ontology():
    onto = pyhornedowl.open_ontology(res("simple.owl"))

    for ac in onto.get_axioms():
        assert pyhornedowl.to_manchester(ac)


def test_rejects_unsupported_argument():
    with pytest.raises(ValueError):
        pyhornedowl.to_manchester("not a model element")
