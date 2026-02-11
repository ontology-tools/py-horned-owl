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


# owl:Thing tests (top-level class)

def test_subclasses_of_thing():
    """owl:Thing subclasses should return all root classes (classes without superclass)"""
    o = simple_ontology()

    # A and C have no superclass, so they are direct subclasses of owl:Thing
    expected = o.get_classes()
    actual = o.get_subclasses("owl:Thing")

    assert expected == actual


def test_superclasses_of_thing():
    """owl:Thing has no superclass"""
    o = simple_ontology()

    expected = set()
    actual = o.get_superclasses("owl:Thing")

    assert expected == actual


def test_ancestors_of_thing():
    """owl:Thing has no ancestors"""
    o = simple_ontology()

    expected = set()
    actual = o.get_ancestors("owl:Thing")

    assert expected == actual


def test_descendants_of_thing():
    """owl:Thing descendants should return all classes"""
    o = simple_ontology()

    expected = {
        "https://example.com/A",
        "https://example.com/B",
        "https://example.com/C",
        "https://example.com/D",
    }
    actual = o.get_descendants("owl:Thing")

    assert expected == actual


# owl:Nothing tests (bottom-level class)

def test_subclasses_of_nothing():
    """owl:Nothing has no subclasses"""
    o = simple_ontology()

    expected = set()
    actual = o.get_subclasses("owl:Nothing")

    assert expected == actual


def test_superclasses_of_nothing():
    """owl:Nothing superclasses - no special handling, returns empty for undeclared class"""
    o = simple_ontology()

    expected = set()
    actual = o.get_superclasses("owl:Nothing")

    assert expected == actual


def test_ancestors_of_nothing():
    """owl:Nothing ancestors - bottom Element of the hierarchy, should have all other classes as ancestors"""
    o = simple_ontology()

    expected = o.get_classes()
    actual = o.get_ancestors("owl:Nothing")

    assert expected == actual


def test_descendants_of_nothing():
    """owl:Nothing has no descendants"""
    o = simple_ontology()

    expected = set()
    actual = o.get_descendants("owl:Nothing")

    assert expected == actual

