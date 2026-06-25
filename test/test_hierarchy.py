import pytest
from test_base import OWL_NOTHING, OWL_THING, simple_ontology


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

    expected = {"https://example.com/A", "https://example.com/B", OWL_THING}
    actual = o.get_ancestors(":D")

    assert expected == actual


def test_multiple_descendants():
    o = simple_ontology()

    expected = {"https://example.com/B", "https://example.com/D"}
    actual = o.get_descendants(":A")

    assert expected == actual


# owl:Thing tests (top-level class)

def test_subclasses_of_thing():
    """owl:Thing subclasses should not do inference and only return subclasses explicitly declared as subclasses of owl:Thing"""
    o = simple_ontology()

    # A and C have no superclass but are not an asserted subclass of owl:Thing
    expected = {"https://example.com/D"}
    actual = o.get_subclasses(OWL_THING)

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
    actual = o.get_ancestors(OWL_THING)

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
    actual = o.get_descendants(OWL_THING)

    assert expected == actual


# owl:Nothing tests (bottom-level class)

def test_subclasses_of_nothing():
    """owl:Nothing has no subclasses"""
    o = simple_ontology()

    expected = set()
    actual = o.get_subclasses(OWL_NOTHING)

    assert expected == actual


def test_superclasses_of_nothing():
    """owl:Nothing superclasses - no special handling, returns empty for undeclared class"""
    o = simple_ontology()

    expected = set()
    actual = o.get_superclasses(OWL_NOTHING)

    assert expected == actual


def test_ancestors_of_nothing():
    """owl:Nothing ancestors - bottom Element of the hierarchy, should have all other classes as ancestors"""
    o = simple_ontology()

    expected = o.get_classes()
    actual = o.get_ancestors(OWL_NOTHING)

    assert expected == actual


def test_descendants_of_nothing():
    """owl:Nothing has no descendants"""
    o = simple_ontology()

    expected = set()
    actual = o.get_descendants(OWL_NOTHING)

    assert expected == actual

def test_root_classes():
    o = simple_ontology()

    expected = {"https://example.com/A", "https://example.com/C"}
    actual = o.get_root_classes()

    assert expected == actual
