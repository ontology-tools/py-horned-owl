import unittest

from pyhornedowl.model import *
from test_base import simple_ontology


class ModifyTestCase(unittest.TestCase):
    def test_add_collect_all_axioms(self):
        o = simple_ontology()

        axioms = set(o.get_axioms())
        axiom = EquivalentClasses([o.clazz(":A"), o.clazz(":B")])

        o.add_axiom(axiom)

        expected = {*axioms, AnnotatedComponent(axiom, set())}
        actual = set(o.get_axioms())

        self.assertSetEqual(expected, actual)

    def test_add_collect_by_iri(self):
        o = simple_ontology()

        axioms = set(o.get_axioms_for_iri(":A"))
        axiom = EquivalentClasses([o.clazz(":A"), o.clazz(":B")])

        o.add_axiom(axiom)

        expected = {*axioms, AnnotatedComponent(axiom, set())}
        actual = set(o.get_axioms_for_iri(":A"))

        self.assertSetEqual(expected, actual)

    def test_remove_collect_all_axioms(self):
        o = simple_ontology()

        axioms = set(o.get_axioms())
        axiom = SubClassOf(o.clazz(":B"), o.clazz(":A"))

        removed = o.remove_axiom(axiom)
        self.assertTrue(removed, "Axiom was not removed!")

        expected = axioms - {AnnotatedComponent(axiom, set())}
        actual = set(o.get_axioms())

        self.assertSetEqual(expected, actual)

    def test_remove_collect_by_iri(self):
        o = simple_ontology()

        all_axioms = set(o.get_axioms())
        axioms = set(o.get_components_for_iri(":A"))
        axiom = SubClassOf(o.clazz(":B"), o.clazz(":A"))
        ann_axiom = AnnotatedComponent(axiom, set())

        removed = o.remove_axiom(axiom)
        
        self.assertTrue(removed, "Axiom was not removed!")
        
        expected = all_axioms - {ann_axiom}
        actual = set(o.get_axioms())
        self.assertSetEqual(expected, actual, "Axiom not removed from all axioms")

        self.skipTest("Removing axioms from the IRI index is not done properly in HornedOWL at the moment. https://github.com/phillord/horned-owl/pull/121")
        expected = axioms - {ann_axiom}
        actual = set(o.get_components_for_iri(":A"))
        self.assertSetEqual(expected, actual, "Axiom not removed from IRI index")

    def test_add_component_with_set_annos(self):
        o = simple_ontology()

        axiom = SubClassOf(o.clazz(":B"), o.clazz(":A"))

        annos0 = {Annotation(o.annotation_property(":a"), SimpleLiteral("Test"))}
        ann_axiom0 = AnnotatedComponent(axiom, annos0)
        o.add_axiom(axiom, annos0)
        assert(ann_axiom0 in o.get_axioms())

        annos1 = {Annotation(o.annotation_property(":a"), SimpleLiteral("Test"))}
        ann_axiom1 = AnnotatedComponent(axiom, annos1)
        o.add_axiom(axiom, annos1)
        assert(ann_axiom1 in o.get_axioms())

        annos2 = {
            Annotation(o.annotation_property(":a"), SimpleLiteral("Test1")),
            Annotation(o.annotation_property(":b"), SimpleLiteral("Test2")),
        }
        ann_axiom2 = AnnotatedComponent(axiom, annos2)
        o.add_axiom(axiom, annos2)
        assert(ann_axiom2 in o.get_axioms())

    def test_add_component_with_list_annos(self):
        o = simple_ontology()

        axiom = SubClassOf(o.clazz(":B"), o.clazz(":A"))

        annos0 = [Annotation(o.annotation_property(":a"), SimpleLiteral("Test"))]
        ann_axiom0 = AnnotatedComponent(axiom, set(annos0))
        o.add_axiom(axiom, annos0)
        assert(ann_axiom0 in o.get_axioms())

        annos1 = [Annotation(o.annotation_property(":a"), SimpleLiteral("Test"))]
        ann_axiom1 = AnnotatedComponent(axiom, set(annos1))
        o.add_axiom(axiom, annos1)
        assert(ann_axiom1 in o.get_axioms())

        annos2 = [
            Annotation(o.annotation_property(":a"), SimpleLiteral("Test1")),
            Annotation(o.annotation_property(":b"), SimpleLiteral("Test2")),
        ]
        ann_axiom2 = AnnotatedComponent(axiom, set(annos2))
        o.add_axiom(axiom, annos2)
        assert(ann_axiom2 in o.get_axioms())






if __name__ == '__main__':
    unittest.main()
