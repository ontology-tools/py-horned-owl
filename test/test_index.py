import pytest
import pyhornedowl
from pyhornedowl import IndexCreationStrategy
from pyhornedowl.model import *
from test_base import r, RDFS_LABEL


def test_default_build_on_query():
    o = pyhornedowl.open_ontology_from_file(r(f'simple.owl'))
    actual = o.get_axioms_for_iri('https://example.com/A')

    assert {AnnotatedComponent(a, set()) for a in {
        DeclareClass(o.clazz("https://example.com/A")),
        SubClassOf(
            o.clazz("https://example.com/B"),
            o.clazz("https://example.com/A")),
        AnnotationAssertion(
            o.iri("https://example.com/A"),
            Annotation(AnnotationProperty(o.iri(RDFS_LABEL)),
                       SimpleLiteral("ClassA"))),
    }} == set(actual)


def test_explicit_fail():
    o = pyhornedowl.open_ontology_from_file(r(f'simple.owl'), index_strategy=IndexCreationStrategy.Explicit)

    with pytest.raises(ValueError):
        o.get_axioms_for_iri("https://example.com/A")


def test_explicit():
    o = pyhornedowl.open_ontology_from_file(r(f'simple.owl'), index_strategy=IndexCreationStrategy.Explicit)

    o.build_indexes()

    actual = o.get_axioms_for_iri("https://example.com/A")

    assert {AnnotatedComponent(a, set()) for a in {
        DeclareClass(o.clazz("https://example.com/A")),
        SubClassOf(
            o.clazz("https://example.com/B"),
            o.clazz("https://example.com/A")),
        AnnotationAssertion(
            o.iri("https://example.com/A"),
            Annotation(AnnotationProperty(o.iri(RDFS_LABEL)),
                       SimpleLiteral("ClassA"))),
    }} == set(actual)


def test_on_load():
    o = pyhornedowl.open_ontology_from_file(r(f'simple.owl'), index_strategy=IndexCreationStrategy.OnLoad)

    actual = o.get_axioms_for_iri("https://example.com/A")

    assert {AnnotatedComponent(a, set()) for a in {
        DeclareClass(o.clazz("https://example.com/A")),
        SubClassOf(
            o.clazz("https://example.com/B"),
            o.clazz("https://example.com/A")),
        AnnotationAssertion(
            o.iri("https://example.com/A"),
            Annotation(AnnotationProperty(o.iri(RDFS_LABEL)),
                       SimpleLiteral("ClassA"))),
    }} == set(actual)
