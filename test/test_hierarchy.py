import pytest
from test_base import simple_ontology


def test_no_subclass():
    o = simple_ontology()

    expected = set()
    actual = o.get_subclasses(":C")

    assert expected == actual


def test_no_superclass():
    o = simple_ontology()

    expected = set()
    actual = o.get_superclasses(":C")

    assert expected == actual


def test_direct_subclass():
    o = simple_ontology()

    expected = {"https://example.com/B"}
    actual = o.get_subclasses(":A")

    assert expected == actual


def test_direct_superclass():
    o = simple_ontology()

    expected = {"https://example.com/A"}
    actual = o.get_superclasses(":B")

    assert expected == actual


def test_no_ancestors():
    o = simple_ontology()

    expected = set()
    actual = o.get_ancestors(":A")

    assert expected == actual


def test_no_descendants():
    o = simple_ontology()

    expected = set()
    actual = o.get_descendants(":C")

    assert expected == actual


def test_single_ancestors():
    o = simple_ontology()

    expected = {"https://example.com/A"}
    actual = o.get_ancestors(":B")

    assert expected == actual


def test_single_descendants():
    o = simple_ontology()

    expected = {"https://example.com/D"}
    actual = o.get_descendants(":B")

    assert expected == actual


def test_multiple_ancestors():
    o = simple_ontology()

    expected = {"https://example.com/A", "https://example.com/B"}
    actual = o.get_ancestors(":D")

    assert expected == actual


def test_multiple_descendants():
    o = simple_ontology()

    expected = {"https://example.com/B", "https://example.com/D"}
    actual = o.get_descendants(":A")

    assert expected == actual
