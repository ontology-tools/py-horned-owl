import unittest

from test.test_base import simple_ontology


class IdTestCase(unittest.TestCase):
    def test_id_from_iri_empty(self):
        o = simple_ontology()
        o.add_prefix_mapping("ex", "https://example.com/")

        expected = "ex:A"
        actual = o.get_id_for_iri("https://example.com/A")

        # The empty prefix was inserted earlier, hence it will be used to define the ID
        self.assertNotEqual(expected, actual)

    def test_id_from_absolute(self):
        o = simple_ontology()
        o.add_prefix_mapping("ex", "https://example.com/")

        expected = "A"
        actual = o.get_id_for_iri("https://example.com/A")

        self.assertEqual(expected, actual)

    def test_id_from_curie_empty_prefix(self):
        o = simple_ontology()
        o.add_prefix_mapping("ex", "https://example.com/")

        expected = "A"
        actual = o.get_id_for_iri("A")

        self.assertEqual(expected, actual)

    def test_id_from_curie_defined_prefix(self):
        o = simple_ontology()
        o.add_prefix_mapping("ex", "https://example.com/")

        expected = "A"
        actual = o.get_id_for_iri("ex:A")

        self.assertEqual(expected, actual)

    def test_iri_from_id(self):
        o = simple_ontology()
        o.add_prefix_mapping("ex", "https://example.com/")

        expected = "https://example.com/A"
        actual = o.get_iri_for_id("ex:A")

        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
