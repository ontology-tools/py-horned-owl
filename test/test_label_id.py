import unittest

from test_base import simple_ontology


class LabelTestCase(unittest.TestCase):
    def test_label_for_iri_existing(self):
        o = simple_ontology()

        expected = "https://example.com/A"
        actual = o.get_iri_for_label("ClassA")

        self.assertEqual(expected, actual)

    def test_label_for_iri_not_existing(self):
        o = simple_ontology()

        expected = None
        actual = o.get_iri_for_label("C")

        self.assertEqual(expected, actual)


if __name__ == '__main__':
    unittest.main()
