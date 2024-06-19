import unittest

from pyhornedowl import PyIndexedOntology
from pyhornedowl.model import ObjectComplementOf, ObjectIntersectionOf, ObjectUnionOf, InverseObjectProperty, \
    ObjectSomeValuesFrom


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


if __name__ == '__main__':
    unittest.main()
