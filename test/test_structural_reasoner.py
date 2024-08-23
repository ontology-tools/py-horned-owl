import unittest

from test_base import hierarchical_ontology


class StructuralReasonerTestCase(unittest.TestCase):
    def test_direct_superclasses_roots(self):
        o = hierarchical_ontology()
        roots_a = o.get_superclasses(":A")
        roots_b = o.get_superclasses(":B")

        self.assertSetEqual(set(), roots_a)
        self.assertSetEqual(set(), roots_b)

    def test_direct_superclasses_1st_level(self):
        o = hierarchical_ontology()

        expected = [(k, {o.clazz(c) for c in v}) for k, v in
            [(":A1", {":A"}),
                    (":A2", {":A"}),
                    (":AB", {":A", ":B"}),
                    (":B1", {":B"}),
                    (":B2", {":B"})]]

        actual = [(k, o.get_superclasses(k)) for (k, v) in expected]

        for e, a in zip(expected, actual):
            self.assertSetEqual(e[1], a[1], e[0])


if __name__ == '__main__':
    unittest.main()
