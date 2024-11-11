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

    def test_load_simple_from_string_explicit_wrong_serialization(self):
        content = res(f"simple.owl")
        with self.assertRaises(ValueError):
            pyhornedowl.open_ontology(content, "something")

        with self.assertRaises(TypeError):
            pyhornedowl.open_ontology(content, 42)

    @unittest.skip("Horned parsers currently panic")
    def test_load_simple_from_string_implicit_fail(self):
        content = "Lorem ipsum dolor sit amet"
        with self.assertRaises(ValueError):
            o = pyhornedowl.open_ontology(content)
            pass

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
                    original.prefix_mapping.remove_prefix("")
                    original.save_to_file(f.name)

                    actual = pyhornedowl.open_ontology_from_file(f.name, s)

                    self.assertOntologiesEqual(original, actual)

    def test_write_simple_explicit(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                with NamedTemporaryFile(suffix=f".{s}.raw") as f:
                    original = simple_ontology()
                    original.prefix_mapping.remove_prefix("")
                    original.save_to_file(f.name, s)

                    actual = pyhornedowl.open_ontology_from_file(f.name, s)

                    self.assertOntologiesEqual(original, actual)
                    
    def test_write_simple_to_string(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                original = simple_ontology()
                original.prefix_mapping.remove_prefix("")
                o = original.save_to_string(s)

                actual = pyhornedowl.open_ontology_from_string(o, s)

                self.assertOntologiesEqual(original, actual)


    def test_prefix(self):
        for s in SERIALIZATIONS:
            with self.subTest(serialization=s):
                if s == 'owl':
                    self.skipTest('RDF/XML parser does not return prefixes')

                o = pyhornedowl.open_ontology(r(f'prefix.{s}'), s)

                actual = set(iter(o.prefix_mapping))
                expected = set({"": "https://example.com/",
                                "ex1": "https://example.com/1",
                                "ex2": "https://example.com/2",
                                "ex3": "https://example.com/3",
                                "rdf": "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
                                "xsd": "http://www.w3.org/2001/XMLSchema#",
                                "rdfs": "http://www.w3.org/2000/01/rdf-schema#",
                                "owl": "http://www.w3.org/2002/07/owl#",
                                "xml": "http://www.w3.org/XML/1998/namespace",
                                }.items())

                self.assertSetEqual(expected, actual)

if __name__ == '__main__':
    unittest.main()
