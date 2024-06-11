import unittest

from pyhornedowl import PyIndexedOntology
from pyhornedowl.model import *
from test_base import simple_ontology, RDFS_LABEL


class IRITestCase(unittest.TestCase):

    def test_absolute_iri(self):
        o = PyIndexedOntology()
        o.add_prefix_mapping("example", "https://example.com/test#")

        expected = "https://example.com/test#A"
        self.assertEqual(expected, str(o.iri(expected)))
        self.assertEqual(expected, str(o.iri(expected, absolute=True)))

    def test_expand_curie(self):
        o = PyIndexedOntology()
        o.add_prefix_mapping("example", "https://example.com/test#")

        self.assertEqual("https://example.com/test#A", str(o.curie("example:A")))
        self.assertEqual("https://example.com/test#A", str(o.iri("example:A", absolute=False)))

    def test_expand_curie_empty_prefix(self):
        o = PyIndexedOntology()
        o.add_prefix_mapping("", "https://example.com/test#")

        self.assertEqual("https://example.com/test#A", str(o.curie(":A")))
        self.assertEqual("https://example.com/test#A", str(o.curie("A")))

    def test_expand_curie_unknown_prefix(self):
        o = PyIndexedOntology()

        with self.assertRaises(ValueError):
            o.curie("example:A")

    def test_iri_guess_curie(self):
        o = PyIndexedOntology()
        o.add_prefix_mapping("ex", "https://example.com/test#")

        expected = "https://example.com/test#A"

        self.assertEqual(expected, str(o.iri("ex:A", absolute=None)))
        self.assertEqual(expected, str(o.clazz("ex:A", absolute=None)))

    def test_iri_guess_absolute(self):
        o = PyIndexedOntology()
        o.add_prefix_mapping("ex", "https://example.com/test#")

        expected = "https://example.com/test#A"

        self.assertEqual(expected, str(o.iri(expected, absolute=None)))
        self.assertEqual(expected, str(o.clazz(expected, absolute=None)))

    def test_find_by_absolute(self):
        o = simple_ontology()

        expected = {AnnotatedComponent(x, set()) for x in {
            DeclareClass(o.clazz("https://example.com/A")),
            (SubClassOf(
                o.clazz("https://example.com/A"),
                o.clazz("https://example.com/B"))),
            AnnotationAssertion(o.iri("https://example.com/A"),
                                Annotation(o.annotation_property(RDFS_LABEL), SimpleLiteral("ClassA")))
        }}
        actual = o.get_axioms_for_iri("https://example.com/A", iri_is_absolute=True)

        self.assertSetEqual(expected, set(actual))

    def test_find_by_curie(self):
        o = simple_ontology()

        expected = {AnnotatedComponent(x, set()) for x in {
            DeclareClass(o.clazz("https://example.com/A")),
            (SubClassOf(
                o.clazz("https://example.com/A"),
                o.clazz("https://example.com/B"))),
            AnnotationAssertion(o.iri("https://example.com/A"),
                                Annotation(o.annotation_property(RDFS_LABEL), SimpleLiteral("ClassA")))
        }}
        actual = o.get_axioms_for_iri("A", iri_is_absolute=False)

        self.assertSetEqual(expected, set(actual))

    def test_find_by_absolute_guess(self):
        o = simple_ontology()

        expected = {AnnotatedComponent(x, set()) for x in {
            DeclareClass(o.clazz("https://example.com/A")),
            (SubClassOf(
                o.clazz("https://example.com/A"),
                o.clazz("https://example.com/B"))),
            AnnotationAssertion(o.iri("https://example.com/A"),
                                Annotation(o.annotation_property(RDFS_LABEL), SimpleLiteral("ClassA")))
        }}
        actual = o.get_axioms_for_iri("https://example.com/A")

        self.assertSetEqual(expected, set(actual))

    def test_find_by_curie_guess(self):
        o = simple_ontology()

        expected = {AnnotatedComponent(x, set()) for x in {
            DeclareClass(o.clazz("https://example.com/A")),
            (SubClassOf(
                o.clazz("https://example.com/A"),
                o.clazz("https://example.com/B"))),
            AnnotationAssertion(o.iri("https://example.com/A"),
                                Annotation(o.annotation_property(RDFS_LABEL), SimpleLiteral("ClassA")))
        }}
        actual = o.get_axioms_for_iri("A")

        self.assertSetEqual(expected, set(actual))


if __name__ == '__main__':
    unittest.main()
