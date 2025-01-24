import unittest

import pyhornedowl
from pyhornedowl import IndexCreationStrategy
from pyhornedowl.model import *
from test_base import r, RDFS_LABEL


class IndexTestCase(unittest.TestCase):

    def test_default_build_on_query(self):
        o = pyhornedowl.open_ontology_from_file(r(f'simple.owl'))
        actual = o.get_axioms_for_iri('https://example.com/A')

        self.assertSetEqual({AnnotatedComponent(a, set()) for a in {
            DeclareClass(o.clazz("https://example.com/A")),
            SubClassOf(
                o.clazz("https://example.com/B"),
                o.clazz("https://example.com/A")),
            AnnotationAssertion(
                o.iri("https://example.com/A"),
                Annotation(AnnotationProperty(o.iri(RDFS_LABEL)),
                           SimpleLiteral("ClassA"))),
        }}, set(actual))

    def test_explicit_fail(self):
        o = pyhornedowl.open_ontology_from_file(r(f'simple.owl'), index_strategy=IndexCreationStrategy.Explicit)

        with self.assertRaises(ValueError):
            o.get_axioms_for_iri("https://example.com/A")

    def test_explicit(self):
        o = pyhornedowl.open_ontology_from_file(r(f'simple.owl'), index_strategy=IndexCreationStrategy.Explicit)

        o.build_indexes()

        actual = o.get_axioms_for_iri("https://example.com/A")

        self.assertSetEqual({AnnotatedComponent(a, set()) for a in {
            DeclareClass(o.clazz("https://example.com/A")),
            SubClassOf(
                o.clazz("https://example.com/B"),
                o.clazz("https://example.com/A")),
            AnnotationAssertion(
                o.iri("https://example.com/A"),
                Annotation(AnnotationProperty(o.iri(RDFS_LABEL)),
                           SimpleLiteral("ClassA"))),
        }}, set(actual))

    def test_on_load(self):
        o = pyhornedowl.open_ontology_from_file(r(f'simple.owl'), index_strategy=IndexCreationStrategy.OnLoad)

        actual = o.get_axioms_for_iri("https://example.com/A")

        self.assertSetEqual({AnnotatedComponent(a, set()) for a in {
            DeclareClass(o.clazz("https://example.com/A")),
            SubClassOf(
                o.clazz("https://example.com/B"),
                o.clazz("https://example.com/A")),
            AnnotationAssertion(
                o.iri("https://example.com/A"),
                Annotation(AnnotationProperty(o.iri(RDFS_LABEL)),
                           SimpleLiteral("ClassA"))),
        }}, set(actual))


if __name__ == '__main__':
    unittest.main()
