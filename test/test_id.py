import pytest
import pyhornedowl
from test_base import simple_ontology


def test_id_from_iri_empty():
    o = simple_ontology()
    o.prefix_mapping.add_prefix("ex", "https://example.com/")

    expected = "ex:A"
    actual = o.get_id_for_iri("https://example.com/A")

    # The empty prefix was inserted earlier, hence it will be used to define the ID
    assert expected != actual


def test_id_from_absolute():
    o = pyhornedowl.PyIndexedOntology()
    o.prefix_mapping.add_prefix("EX", "https://example.com/")

    expected = "EX:A"
    actual = o.get_id_for_iri("https://example.com/A")

    assert expected == actual


def test_id_from_curie_empty_prefix():
    o = pyhornedowl.PyIndexedOntology()
    o.prefix_mapping.add_prefix("", "https://example.com/")

    expected = "A"
    actual = o.get_id_for_iri(":A")

    assert expected == actual


def test_id_from_curie_defined_prefix():
    o = pyhornedowl.PyIndexedOntology()
    o.prefix_mapping.add_prefix("EX", "https://example.com/")

    expected = "EX:A"
    actual = o.get_id_for_iri("EX:A")

    assert expected == actual


def test_iri_from_id():
    o = simple_ontology()
    o.prefix_mapping.add_prefix("ex", "https://example.com/")

    expected = "https://example.com/A"
    actual = o.get_iri_for_id("ex:A")

    assert expected == actual
