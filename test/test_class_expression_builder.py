import unittest

from pyhornedowl import PyIndexedOntology
from pyhornedowl.model import *


class ClassExpressionBuilderTestCase(unittest.TestCase):
    def test_ce_not(self):
        o = PyIndexedOntology()
        c = o.clazz("https://example.com/A")

        expected = ObjectComplementOf(c)
        actual = ~c

        self.assertEqual(expected, actual)

    def test_ce_not_complex(self):
        o = PyIndexedOntology()
        c = ObjectUnionOf([o.clazz("https://example.com/A"), o.clazz("https://example.com/B")])

        expected = ObjectComplementOf(c)
        actual = ~c

        self.assertEqual(expected, actual)

    def test_ce_and(self):
        o = PyIndexedOntology()
        c1 = o.clazz("https://example.com/A")
        c2 = o.clazz("https://example.com/B")

        expected = ObjectIntersectionOf([c1, c2])
        actual = c1 & c2

        self.assertEqual(expected, actual)

    def test_ce_or(self):
        o = PyIndexedOntology()
        c1 = o.clazz("https://example.com/A")
        c2 = o.clazz("https://example.com/B")

        expected = ObjectUnionOf([c1, c2])
        actual = c1 | c2

        self.assertEqual(expected, actual)

    def test_op_not(self):
        o = PyIndexedOntology()
        op = o.object_property("https://example.com/op")

        expected = InverseObjectProperty(op)
        actual = ~op

        self.assertEqual(expected, actual)

    def test_ce_some(self):
        o = PyIndexedOntology()
        c = o.clazz("https://example.com/A")
        op = o.object_property("https://example.com/op")

        expected = ObjectSomeValuesFrom(op, c)
        actual = op.some(c)

        self.assertEqual(expected, actual)

    def test_ce_only(self):
        o = PyIndexedOntology()
        c = o.clazz("https://example.com/A")
        op = o.object_property("https://example.com/op")

        expected = ObjectAllValuesFrom(op, c)
        actual = op.only(c)

        self.assertEqual(expected, actual)

    def test_ce_min(self):
        o = PyIndexedOntology()
        c = o.clazz("https://example.com/A")
        op = o.object_property("https://example.com/op")

        expected = ObjectMinCardinality(2, op, c)
        actual = op.min(2, c)

        self.assertEqual(expected, actual)

    def test_ce_max(self):
        o = PyIndexedOntology()
        c = o.clazz("https://example.com/A")
        op = o.object_property("https://example.com/op")

        expected = ObjectMaxCardinality(2, op, c)
        actual = op.max(2, c)

        self.assertEqual(expected, actual)

    def test_ce_exact(self):
        o = PyIndexedOntology()
        c = o.clazz("https://example.com/A")
        op = o.object_property("https://example.com/op")

        expected = ObjectExactCardinality(2, op, c)
        actual = op.exact(2, c)

        self.assertEqual(expected, actual)

    def test_ce_has_value(self):
        o = PyIndexedOntology()
        op = o.object_property("https://example.com/op")
        i = o.named_individual("https://example.com/I")

        expected = ObjectHasValue(op, i)
        actual = op.has_value(i)

        self.assertEqual(expected, actual)

    def test_ce_has_self(self):
        o = PyIndexedOntology()
        op = o.object_property("https://example.com/op")

        expected = ObjectHasSelf(op)
        actual = op.has_self()

        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
