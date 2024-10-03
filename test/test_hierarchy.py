import unittest

from test_base import simple_ontology


class HierarchyTestCase(unittest.TestCase):
    def test_no_subclass(self):
        o = simple_ontology()

        expected = set()
        actual = o.get_subclasses(":C")

        self.assertSetEqual(expected, actual)

    def test_no_superclass(self):
        o = simple_ontology()

        expected = set()
        actual = o.get_superclasses(":C")

        self.assertSetEqual(expected, actual)

    def test_direct_subclass(self):
        o = simple_ontology()

        expected = {"https://example.com/B"}
        actual = o.get_subclasses(":A")

        self.assertSetEqual(expected, actual)

    def test_direct_superclass(self):
        o = simple_ontology()

        expected = {"https://example.com/A"}
        actual = o.get_superclasses(":B")

        self.assertSetEqual(expected, actual)

    def test_no_ancestors(self):
        o = simple_ontology()

        expected = set()
        actual = o.get_ancestors(":A")

        self.assertSetEqual(expected, actual)

    def test_no_descendants(self):
        o = simple_ontology()

        expected = set()
        actual = o.get_descendants(":C")

        self.assertSetEqual(expected, actual)

    def test_single_ancestors(self):
        o = simple_ontology()

        expected = {"https://example.com/A"}
        actual = o.get_ancestors(":B")

        self.assertSetEqual(expected, actual)

    def test_single_descendants(self):
        o = simple_ontology()

        expected = {"https://example.com/D"}
        actual = o.get_descendants(":B")

        self.assertSetEqual(expected, actual)

    def test_multiple_ancestors(self):
        o = simple_ontology()

        expected = {"https://example.com/A", "https://example.com/B"}
        actual = o.get_ancestors(":D")

        self.assertSetEqual(expected, actual)

    def test_multiple_descendants(self):
        o = simple_ontology()

        expected = {"https://example.com/B", "https://example.com/D"}
        actual = o.get_descendants(":A")

        self.assertSetEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
