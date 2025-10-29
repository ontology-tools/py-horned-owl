"""Tests for the ontology."""

import pytest
import pyhornedowl
from pyhornedowl.model import *

from test_base import RDFS_LABEL


@pytest.fixture
def o():
    """Set up the test case with an ontology."""
    components = [
        DeclareClass(Class(IRI.parse("https://example.com/A"))),
        DeclareClass(Class(IRI.parse("https://example.com/B"))),
        DeclareClass(Class(IRI.parse("https://example.com/C"))),
        DeclareClass(Class(IRI.parse("https://example.com/D"))),
        DeclareAnnotationProperty(AnnotationProperty(IRI.parse(RDFS_LABEL))),
        DeclareObjectProperty(ObjectProperty(IRI.parse("https://example.com/E"))),
        DeclareDataProperty(DataProperty(IRI.parse("https://example.com/F"))),
        DeclareNamedIndividual(NamedIndividual(IRI.parse("https://example.com/G"))),
    ]

    onto = pyhornedowl.PyIndexedOntology()
    onto.prefix_mapping.add_default_prefix_names()
    onto.prefix_mapping.add_prefix("", "https://example.com/")

    for component in components:
        onto.add_component(component)

    return onto


def test_get_object_properties(o) -> None:
    """Test getting object properties."""
    assert {"https://example.com/E"} == o.get_object_properties()


def test_get_data_properties(o) -> None:
    """Test getting data properties."""
    assert {"https://example.com/F"} == o.get_data_properties()


def test_get_annotation_properties(o) -> None:
    """Test getting annotation properties."""
    assert {RDFS_LABEL} == o.get_annotation_properties()


def test_get_classes(o) -> None:
    """Test getting classes."""
    assert {f"https://example.com/{x}" for x in "ABCD"} == o.get_classes()


def test_named_individuals(o) -> None:
    """Test getting named individuals."""
    assert {"https://example.com/G"} == o.get_named_individuals()
