"""Per-element rendering of axioms and class expressions (`pyhornedowl.write_snippet`)."""

import pytest

import pyhornedowl
from pyhornedowl.model import (
    AnnotatedComponent,
    Annotation,
    AnnotationProperty,
    Class,
    ClassAssertion,
    DeclareClass,
    IRI,
    NamedIndividual,
    ObjectProperty,
    ObjectSomeValuesFrom,
    SimpleLiteral,
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

    assert pyhornedowl.write_snippet(ax) == f"<{A}> SubClassOf <{B}>"


def test_renders_existential_restriction():
    ax = SubClassOf(
        sub=c(A),
        sup=ObjectSomeValuesFrom(ope=ObjectProperty(IRI.parse(R)), bce=c(B)),
    )

    assert pyhornedowl.write_snippet(ax) == f"<{A}> SubClassOf <{R}> some <{B}>"


def test_renders_class_expression_alone():
    ce = ObjectSomeValuesFrom(ope=ObjectProperty(IRI.parse(R)), bce=c(B))

    assert pyhornedowl.write_snippet(ce) == f"<{R}> some <{B}>"


def test_abbreviates_with_prefix_mapping():
    onto = pyhornedowl.open_ontology(res("simple.owl"))
    onto.add_prefix_mapping("ex", "http://example.com/")
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert pyhornedowl.write_snippet(ax, "omn", onto.prefix_mapping) == "ex:A SubClassOf ex:B"


def test_accepts_annotated_component_from_get_axioms():
    onto = simple_ontology()

    rendered = [pyhornedowl.write_snippet(ac) for ac in onto.get_axioms()]

    assert rendered, "expected at least one axiom"
    assert all(isinstance(s, str) and s for s in rendered)


def test_renders_every_axiom_of_a_real_ontology():
    onto = pyhornedowl.open_ontology(res("simple.owl"))

    for ac in onto.get_axioms():
        assert pyhornedowl.write_snippet(ac)


def test_rejects_unsupported_argument():
    with pytest.raises(ValueError):
        pyhornedowl.write_snippet("not a model element")


def test_renders_functional_syntax():
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert pyhornedowl.write_snippet(ax, "ofn") == f"SubClassOf(<{A}> <{B}>)"


def test_functional_alias():
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert pyhornedowl.write_snippet(ax, "functional") == pyhornedowl.write_snippet(ax, "ofn")


def test_manchester_alias():
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert pyhornedowl.write_snippet(ax, "manchester") == pyhornedowl.write_snippet(ax, "omn")


def test_defaults_to_manchester():
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert pyhornedowl.write_snippet(ax) == pyhornedowl.write_snippet(ax, "omn")


@pytest.mark.parametrize("serialization", ["owx", "rdf", "obo", "nonsense"])
def test_rejects_serializations_without_a_per_element_writer(serialization):
    ax = SubClassOf(sub=c(A), sup=c(B))

    with pytest.raises(ValueError, match="per-element"):
        pyhornedowl.write_snippet(ax, serialization)


def test_functional_keeps_axiom_annotations_that_manchester_drops():
    # An annotation ON an axiom, which is not the same thing as an AnnotationAssertion.
    ac = AnnotatedComponent(
        SubClassOf(sub=c(A), sup=c(B)),
        {
            Annotation(
                AnnotationProperty(IRI.parse("http://www.w3.org/2000/01/rdf-schema#comment")),
                SimpleLiteral("why"),
                set(),
            )
        },
    )

    # AsFunctional has an AnnotatedComponent impl; AsManchester does not, so Manchester
    # renders the bare component. Documented on write_snippet.
    assert "Annotation(" in pyhornedowl.write_snippet(ac, "ofn")
    assert pyhornedowl.write_snippet(ac, "omn") == f"<{A}> SubClassOf <{B}>"
