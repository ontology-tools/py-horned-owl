"""`element.serialize(...)`, the method form of `pyhornedowl.write_snippet`."""

import pytest

import pyhornedowl
import pyhornedowl.model
from pyhornedowl.model import (
    AnnotatedComponent,
    Annotation,
    AnnotationProperty,
    Class,
    Facet,
    FacetRestriction,
    IRI,
    ObjectProperty,
    ObjectSomeValuesFrom,
    SimpleLiteral,
    SubClassOf,
)
from test_base import res

A = "http://example.com/A"
B = "http://example.com/B"
R = "http://example.com/r"


def c(iri: str) -> Class:
    return Class(IRI.parse(iri))


# Classes horned-owl cannot render in Manchester syntax on their own: they are
# neither an OWL entity nor a component, so Manchester only writes them inside
# the element that holds them. Mirrors NO_MANCHESTER in scripts/build_model.py.
NO_MANCHESTER = [
    Annotation(AnnotationProperty(IRI.parse(R)), SimpleLiteral("v"), set()),
    AnnotationProperty(IRI.parse(R)),
    FacetRestriction(f=Facet.MinInclusive, l=SimpleLiteral("1")),
    Facet.MinInclusive,
]


def test_renders_an_axiom():
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert ax.serialize("omn") == f"<{A}> SubClassOf <{B}>"
    assert ax.serialize("ofn") == f"SubClassOf(<{A}> <{B}>)"


def test_defaults_to_functional():
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert ax.serialize() == ax.serialize("ofn")


def test_renders_a_class_expression():
    ce = ObjectSomeValuesFrom(ope=ObjectProperty(IRI.parse(R)), bce=c(B))

    assert ce.serialize("omn") == f"<{R}> some <{B}>"


def test_renders_an_entity_and_an_iri():
    assert c(A).serialize("omn") == f"<{A}>"
    assert IRI.parse(A).serialize("omn") == f"<{A}>"


def test_abbreviates_with_prefix_mapping():
    onto = pyhornedowl.open_ontology(res("simple.owl"))
    onto.add_prefix_mapping("ex", "http://example.com/")
    ax = SubClassOf(sub=c(A), sup=c(B))

    assert ax.serialize("omn", onto.prefix_mapping) == "ex:A SubClassOf ex:B"
    assert ax.serialize("ofn", onto.prefix_mapping) == "SubClassOf(ex:A ex:B)"


def test_agrees_with_write_snippet():
    onto = pyhornedowl.open_ontology(res("simple.owl"))

    for ac in onto.get_axioms():
        for serialization in ("omn", "ofn"):
            assert ac.serialize(serialization) == pyhornedowl.write_snippet(
                ac, serialization
            )


def test_functional_keeps_axiom_annotations_that_manchester_drops():
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

    assert "Annotation(" in ac.serialize("ofn")
    assert ac.serialize("omn") == f"<{A}> SubClassOf <{B}>"


@pytest.mark.parametrize("serialization", ["owx", "rdf", "obo", "nonsense"])
def test_rejects_serializations_without_a_per_element_writer(serialization):
    ax = SubClassOf(sub=c(A), sup=c(B))

    with pytest.raises(ValueError, match="per-element"):
        ax.serialize(serialization)


@pytest.mark.parametrize("element", NO_MANCHESTER, ids=lambda e: type(e).__name__)
def test_elements_without_a_manchester_rendering_say_so(element):
    with pytest.raises(ValueError, match="no Manchester rendering"):
        element.serialize("omn")

    assert element.serialize("ofn")


def test_every_model_class_has_serialize():
    without = [
        name
        for name, obj in vars(pyhornedowl.model).items()
        if isinstance(obj, type) and not name.startswith("_") and not hasattr(obj, "serialize")
    ]

    assert without == []
