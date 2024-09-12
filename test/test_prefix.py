import typing
import unittest

import pyhornedowl
from test_base import r

SERIALIZATIONS: typing.List[typing.Literal['ofn', 'owx', 'owl']] = ['ofn', 'owx', 'owl']


class PrefixTestCase(unittest.TestCase):
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
