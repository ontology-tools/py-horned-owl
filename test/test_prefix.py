import unittest

import pyhornedowl


class PrefixTestCase(unittest.TestCase):

    def test_missing_prefix(self):
        o = pyhornedowl.PyIndexedOntology()

        with self.assertRaises(ValueError) as context:
            c = o.clazz("ex:A")

        self.assertEqual(context.exception.args[0], "Invalid curie: Invalid")

    def test_prefix(self):
        o = pyhornedowl.PyIndexedOntology()
        o.prefix_mapping.add_prefix("ex", "https://example.com/")

        c = o.clazz("ex:A")

        self.assertEqual(str(c), "https://example.com/A")

    def test_default_prefix_fail(self):
        o = pyhornedowl.PyIndexedOntology()

        with self.assertRaises(ValueError) as context:
            c = o.clazz("A")

        self.assertEqual(context.exception.args[0], "Invalid curie: MissingDefault")

    def test_default_prefix(self):
        o = pyhornedowl.PyIndexedOntology()
        o.prefix_mapping.add_prefix("", "https://example.com/")

        c = o.clazz("A")

        self.assertEqual(str(c), "https://example.com/A")


if __name__ == '__main__':
    unittest.main()
