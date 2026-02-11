import pytest
from pyhornedowl import PyIndexedOntology
from pyhornedowl.model import *
from test_base import simple_ontology, RDFS_LABEL


def test_absolute_iri():
    o = PyIndexedOntology()
    o.prefix_mapping.add_prefix("example", "https://example.com/test#")

    expected = "https://example.com/test#A"
    assert expected == str(o.iri(expected))
    assert expected == str(o.iri((expected, True)))


def test_expand_curie():
    o = PyIndexedOntology()
    o.prefix_mapping.add_prefix("example", "https://example.com/test#")

    assert "https://example.com/test#A" == str(o.curie("example:A"))
    assert "https://example.com/test#A" == str(o.iri(("example:A", False)))


def test_expand_curie_empty_prefix():
    o = PyIndexedOntology()
    o.prefix_mapping.add_prefix("", "https://example.com/test#")

    assert "https://example.com/test#A" == str(o.curie(":A"))


def test_expand_curie_unknown_prefix():
    o = PyIndexedOntology()

    with pytest.raises(ValueError):
        o.curie("example:A")


def test_iri_guess_curie():
    o = PyIndexedOntology()
    o.prefix_mapping.add_prefix("ex", "https://example.com/test#")

    expected = "https://example.com/test#A"

    assert expected == str(o.iri("ex:A"))
    assert expected == str(o.clazz("ex:A"))


def test_iri_guess_absolute():
    o = PyIndexedOntology()
    o.prefix_mapping.add_prefix("ex", "https://example.com/test#")

    expected = "https://example.com/test#A"

    assert expected == str(o.iri(expected))
    assert expected == str(o.clazz(expected))


def test_find_by_absolute():
    o = simple_ontology()

    expected = {AnnotatedComponent(x, set()) for x in {
        DeclareClass(o.clazz("https://example.com/A")),
        (SubClassOf(
            o.clazz("https://example.com/B"),
            o.clazz("https://example.com/A"))),
        AnnotationAssertion(o.iri("https://example.com/A"),
                            Annotation(o.annotation_property(RDFS_LABEL), SimpleLiteral("ClassA")))
    }}
    actual = o.get_axioms_for_iri(("https://example.com/A", True))

    assert expected == set(actual)


def test_find_by_curie():
    o = simple_ontology()

    expected = {AnnotatedComponent(x, set()) for x in {
        DeclareClass(o.clazz("https://example.com/A")),
        (SubClassOf(
            o.clazz("https://example.com/B"),
            o.clazz("https://example.com/A"))),
        AnnotationAssertion(o.iri("https://example.com/A"),
                            Annotation(o.annotation_property(RDFS_LABEL), SimpleLiteral("ClassA")))
    }}
    actual = o.get_axioms_for_iri((":A", False))

    assert expected == set(actual)


def test_find_by_absolute_guess():
    o = simple_ontology()

    expected = {AnnotatedComponent(x, set()) for x in {
        DeclareClass(o.clazz("https://example.com/A")),
        (SubClassOf(
            o.clazz("https://example.com/B"),
            o.clazz("https://example.com/A"))),
        AnnotationAssertion(o.iri("https://example.com/A"),
                            Annotation(o.annotation_property(RDFS_LABEL), SimpleLiteral("ClassA")))
    }}
    actual = o.get_axioms_for_iri("https://example.com/A")

    assert expected == set(actual)


def test_find_by_curie_guess():
    o = simple_ontology()

    expected = {AnnotatedComponent(x, set()) for x in {
        DeclareClass(o.clazz("https://example.com/A")),
        (SubClassOf(
            o.clazz("https://example.com/B"),
            o.clazz("https://example.com/A"))),
        AnnotationAssertion(o.iri("https://example.com/A"),
                            Annotation(o.annotation_property(RDFS_LABEL), SimpleLiteral("ClassA")))
    }}
    actual = o.get_axioms_for_iri(":A")

    assert expected == set(actual)
