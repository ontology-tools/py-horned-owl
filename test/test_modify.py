import unittest

from pyhornedowl.model import *
from test_base import simple_ontology


class ModifyTestCase(unittest.TestCase):
    def test_add_collect_all_axioms(self):
        o = simple_ontology()

        axioms = set(o.get_axioms())
        axiom = EquivalentClasses([o.clazz("A"), o.clazz("B")])

        o.add_axiom(axiom)

        expected = {*axioms, AnnotatedComponent(axiom, set())}
        actual = set(o.get_axioms())

        self.assertSetEqual(expected, actual)

    def test_add_collect_by_iri(self):
        o = simple_ontology()

        axioms = set(o.get_axioms_for_iri("A"))
        axiom = EquivalentClasses([o.clazz("A"), o.clazz("B")])

        o.add_axiom(axiom)

        expected = {*axioms, AnnotatedComponent(axiom, set())}
        actual = set(o.get_axioms_for_iri("A"))

        self.assertSetEqual(expected, actual)

    def test_remove_collect_all_axioms(self):
        o = simple_ontology()

        axioms = set(o.get_axioms())
        axiom = SubClassOf(o.clazz("A"), o.clazz("B"))

        o.remove_axiom(axiom)

        expected = axioms - {AnnotatedComponent(axiom, set())}
        actual = set(o.get_axioms())

        self.assertSetEqual(expected, actual)

    def test_remove_collect_by_iri(self):
        o = simple_ontology()

        axioms = set(o.get_axioms_for_iri("A"))
        axiom = SubClassOf(o.clazz("A"), o.clazz("B"))

        o.remove_axiom(axiom)

        expected = axioms - {AnnotatedComponent(axiom, set())}
        actual = set(o.get_axioms_for_iri("A"))

        self.assertSetEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
