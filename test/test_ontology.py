"""Tests for the ontology."""

import unittest

import pyhornedowl
from pyhornedowl.model import *

from test_base import RDFS_LABEL


class OntologyTestCase(unittest.TestCase):
    """Tests for the ontology."""

    def setUp(self) -> None:
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

        self.o = onto

    def test_get_object_properties(self) -> None:
        """Test getting object properties."""
        self.assertEqual(
            {"https://example.com/E"},
            self.o.get_object_properties(),
        )

    def test_get_data_properties(self) -> None:
        """Test getting data properties."""
        self.assertEqual(
            {"https://example.com/F"},
            self.o.get_data_properties(),
        )

    def test_get_annotation_properties(self) -> None:
        """Test getting annotation properties."""
        self.assertEqual(
            {RDFS_LABEL},
            self.o.get_annotation_properties(),
        )

    def test_get_classes(self) -> None:
        """Test getting classes."""
        self.assertEqual(
            {f"https://example.com/{x}" for x in "ABCD"},
            self.o.get_classes(),
        )

    def test_named_individuals(self) -> None:
        """Test getting named individuals."""
        self.assertEqual(
            {"https://example.com/G"},
            self.o.get_named_individuals(),
        )


if __name__ == '__main__':
    unittest.main()
