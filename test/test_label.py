import pytest
from test_base import simple_ontology
from pyhornedowl.model import SimpleLiteral


def test_label_for_iri_existing():
    o = simple_ontology()

    expected = "https://example.com/A"
    actual = o.get_iri_for_label("ClassA")

    assert expected == actual


def test_label_for_iri_not_existing():
    o = simple_ontology()

    expected = None
    actual = o.get_iri_for_label("ClassZ")

    assert expected == actual


def test_update_label():
    o = simple_ontology()

    expected = "NewClassA"
    o.set_label(":A", expected)

    actual = o.get_label(o.class_(":A"))

    assert expected == actual

    all_labels = o.get_annotations(o.class_(":A"), "rdfs:label")
    assert [SimpleLiteral(expected)] == all_labels


def test_add_label():
    o = simple_ontology()

    expected = "ClassC"
    o.set_label(":C", expected)

    actual = o.get_label(o.class_(":C"))

    assert expected == actual
