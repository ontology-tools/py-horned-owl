import pytest
import pyhornedowl


def test_missing_prefix():
    o = pyhornedowl.PyIndexedOntology()

    with pytest.raises(ValueError) as context:
        c = o.class_("ex:A")

    assert context.value.args[0] == "Cannot expand CURIE ex:A"


def test_prefix():
    o = pyhornedowl.PyIndexedOntology()
    o.prefix_mapping.add_prefix("ex", "https://example.com/")

    c = o.class_("ex:A")

    assert str(c) == "https://example.com/A"


def test_default_prefix_fail():
    o = pyhornedowl.PyIndexedOntology()

    with pytest.raises(ValueError) as context:
        c = o.class_("A")

    assert context.value.args[0] == "Cannot expand CURIE :A"


def test_default_prefix():
    o = pyhornedowl.PyIndexedOntology()
    o.prefix_mapping.add_prefix("", "https://example.com/")

    c = o.class_("A")

    assert str(c) == "https://example.com/A"
