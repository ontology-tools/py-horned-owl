import typing
import unittest
from tempfile import NamedTemporaryFile

import pyhornedowl
from test_base import r, res, simple_ontology

SERIALIZATIONS: typing.List[typing.Literal['ofn', 'owx', 'owl']] = ['ofn', 'owx', 'owl']


class IOTestCase(unittest.TestCase):

    def assertOntologiesEqual(self, actual, expected):
        self.assertSetEqual(set(expected.get_axioms()), set(actual.get_axioms()), "Axioms do not match!")
        self.assertEqual(expected.get_iri(), actual.get_iri(), "Ontology IRIs do not match!")

    @unittest.skip("Functional syntax parser parses others formats without errors others panic")
    def test_load_simple_from_string_generic_guess(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                content = res(f"simple.{s}")

                actual = pyhornedowl.open_ontology(content)
                expected = simple_ontology()

                self.assertOntologiesEqual(actual, expected)

    def test_load_simple_from_string_generic_explicit(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                content = res(f"simple.{s}")

                actual = pyhornedowl.open_ontology(content, s)
                expected = simple_ontology()

                self.assertOntologiesEqual(actual, expected)

    def test_load_simple_from_string_explicit(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                content = res(f"simple.{s}")

                actual = pyhornedowl.open_ontology_from_string(content, s)
                expected = simple_ontology()

                self.assertOntologiesEqual(actual, expected)

    @unittest.skip("Functional syntax parser parses others formats without errors others panic")
    def test_load_simple_from_string_guess_parser(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                content = res(f"simple.{s}")

                actual = pyhornedowl.open_ontology_from_string(content)
                expected = simple_ontology()

                self.assertOntologiesEqual(actual, expected)

    def test_load_simple_from_file_generic_guess(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                actual = pyhornedowl.open_ontology(r(f'simple.{s}'))
                expected = simple_ontology()

                self.assertOntologiesEqual(actual, expected)

    def test_load_simple_from_file_generic_explicit(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                actual = pyhornedowl.open_ontology(r(f'simple.{s}'), s)
                expected = simple_ontology()

                self.assertOntologiesEqual(actual, expected)

    def test_load_simple_from_file_guess_ext(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                actual = pyhornedowl.open_ontology_from_file(r(f'simple.{s}'))
                expected = simple_ontology()

                self.assertOntologiesEqual(actual, expected)

    def test_load_simple_from_file_explicit(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                actual = pyhornedowl.open_ontology_from_file(r(f'simple.{s}.raw'), s)
                expected = simple_ontology()

                self.assertOntologiesEqual(actual, expected)

    def test_write_simple_guess_ext(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                with NamedTemporaryFile(suffix=f".{s}") as f:
                    original = simple_ontology()
                    original.save_to_file(f.name)

                    actual = pyhornedowl.open_ontology_from_file(f.name, s)

                    self.assertOntologiesEqual(original, actual)

    def test_write_simple_explicit(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                with NamedTemporaryFile(suffix=f".{s}.raw") as f:
                    original = simple_ontology()
                    original.save_to_file(f.name, s)

                    actual = pyhornedowl.open_ontology_from_file(f.name, s)

                    self.assertOntologiesEqual(original, actual)


if __name__ == '__main__':
    unittest.main()
