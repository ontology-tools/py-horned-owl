import pytest
from test_base import OWL_NOTHING, OWL_THING, simple_ontology

from pyhornedowl.reasoning import create_structural_reasoner


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
    r = create_structural_reasoner(o)

    expected = set()
    actual = r.get_superclasses(o.class_(":A"))

    assert expected == actual


def test_no_descendants():
    o = simple_ontology()
    r = create_structural_reasoner(o)

    expected = set()
    actual = r.get_subclasses(o.class_(":C"))

    assert expected == actual


def test_single_ancestors():
    o = simple_ontology()
    r = create_structural_reasoner(o)

    expected = {o.class_("https://example.com/A")}
    actual = r.get_superclasses(o.class_(":B"))

    assert expected == actual


def test_single_descendants():
    o = simple_ontology()
    r = create_structural_reasoner(o)

    expected = {o.class_("https://example.com/D")}
    actual = r.get_subclasses(o.class_(":B"))

    assert expected == actual


def test_multiple_ancestors():
    o = simple_ontology()
    r = create_structural_reasoner(o)

    expected = {
        o.class_("https://example.com/A"),
        o.class_("https://example.com/B"),
        o.class_(OWL_THING),
    }
    actual = r.get_superclasses(o.class_(":D"))

    assert expected == actual


def test_multiple_descendants():
    o = simple_ontology()
    r = create_structural_reasoner(o)

    expected = {
        o.class_("https://example.com/B"),
        o.class_("https://example.com/D"),
    }
    actual = r.get_subclasses(o.class_(":A"))

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
    r = create_structural_reasoner(o)

    expected = set()
    actual = r.get_superclasses(o.class_(OWL_THING))

    assert expected == actual


def test_descendants_of_thing():
    """owl:Thing descendants should return all classes"""
    o = simple_ontology()
    r = create_structural_reasoner(o)

    expected = {
        o.class_("https://example.com/A"),
        o.class_("https://example.com/B"),
        o.class_("https://example.com/C"),
        o.class_("https://example.com/D"),
    }
    actual = r.get_subclasses(o.class_(OWL_THING))

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
    r = create_structural_reasoner(o)

    expected = {o.class_(c) for c in o.get_classes()}
    actual = r.get_superclasses(o.class_(OWL_NOTHING))

    assert expected == actual


def test_descendants_of_nothing():
    """owl:Nothing has no descendants"""
    o = simple_ontology()
    r = create_structural_reasoner(o)

    expected = set()
    actual = r.get_subclasses(o.class_(OWL_NOTHING))

    assert expected == actual


def test_root_classes():
    o = simple_ontology()

    expected = {"https://example.com/A", "https://example.com/C"}
    actual = o.get_root_classes()

    assert expected == actual
