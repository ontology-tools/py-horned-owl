import pytest
import pyhornedowl


def test_missing_prefix():
    o = pyhornedowl.PyIndexedOntology()

    with pytest.raises(ValueError) as context:
        c = o.clazz("ex:A")

    assert context.value.args[0] == "Invalid curie: Invalid"


def test_prefix():
    o = pyhornedowl.PyIndexedOntology()
    o.prefix_mapping.add_prefix("ex", "https://example.com/")

    c = o.clazz("ex:A")

    assert str(c) == "https://example.com/A"


def test_default_prefix_fail():
    o = pyhornedowl.PyIndexedOntology()

    with pytest.raises(ValueError) as context:
        c = o.clazz("A")

    assert context.value.args[0] == "Invalid curie: MissingDefault"


def test_default_prefix():
    o = pyhornedowl.PyIndexedOntology()
    o.prefix_mapping.add_prefix("", "https://example.com/")

    c = o.clazz("A")

    assert str(c) == "https://example.com/A"
