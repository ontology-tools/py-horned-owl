import typing
from tempfile import NamedTemporaryFile

import pytest
import pyhornedowl
from test_base import r, res, simple_ontology

SERIALIZATIONS: typing.List[typing.Literal['ofn', 'owx', 'owl']] = ['ofn', 'owx', 'owl']


def assert_ontologies_equal(actual, expected):
    assert set(expected.get_axioms()) == set(actual.get_axioms()), "Axioms do not match!"
    assert expected.get_iri() == actual.get_iri(), "Ontology IRIs do not match!"


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_load_simple_from_string_generic_guess(s):
    content = res(f"simple.{s}")

    actual = pyhornedowl.open_ontology(content)
    expected = simple_ontology()

    assert_ontologies_equal(actual, expected)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_load_simple_from_string_generic_explicit(s):
    content = res(f"simple.{s}")

    actual = pyhornedowl.open_ontology(content, s)
    expected = simple_ontology()

    assert_ontologies_equal(actual, expected)


def test_load_simple_from_string_explicit_wrong_serialization():
    content = res(f"simple.owl")
    with pytest.raises(ValueError):
        pyhornedowl.open_ontology(content, "something")

    with pytest.raises(TypeError):
        pyhornedowl.open_ontology(content, 42)


@pytest.mark.skip(reason="Horned parsers currently panic")
def test_load_simple_from_string_implicit_fail():
    content = "Lorem ipsum dolor sit amet"
    with pytest.raises(ValueError):
        o = pyhornedowl.open_ontology(content)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_load_simple_from_string_explicit(s):
    content = res(f"simple.{s}")

    actual = pyhornedowl.open_ontology_from_string(content, s)
    expected = simple_ontology()

    assert_ontologies_equal(actual, expected)


@pytest.mark.skip(reason="Functional syntax parser parses others formats without errors others panic")
@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_load_simple_from_string_guess_parser(s):
    content = res(f"simple.{s}")

    actual = pyhornedowl.open_ontology_from_string(content)
    expected = simple_ontology()

    assert_ontologies_equal(actual, expected)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_load_simple_from_file_generic_guess(s):
    actual = pyhornedowl.open_ontology(r(f'simple.{s}'))
    expected = simple_ontology()

    assert_ontologies_equal(actual, expected)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_load_simple_from_file_generic_explicit(s):
    actual = pyhornedowl.open_ontology(r(f'simple.{s}'), s)
    expected = simple_ontology()

    assert_ontologies_equal(actual, expected)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_load_simple_from_file_guess_ext(s):
    actual = pyhornedowl.open_ontology_from_file(r(f'simple.{s}'))
    expected = simple_ontology()

    assert_ontologies_equal(actual, expected)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_load_simple_from_file_explicit(s):
    actual = pyhornedowl.open_ontology_from_file(r(f'simple.{s}.raw'), s)
    expected = simple_ontology()

    assert_ontologies_equal(actual, expected)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_write_simple_guess_ext(s):
    with NamedTemporaryFile(suffix=f".{s}") as f:
        original = simple_ontology()
        original.prefix_mapping.remove_prefix("")
        original.save_to_file(f.name)

        actual = pyhornedowl.open_ontology_from_file(f.name, s)

        assert_ontologies_equal(original, actual)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_write_simple_explicit(s):
    with NamedTemporaryFile(suffix=f".{s}.raw") as f:
        original = simple_ontology()
        original.prefix_mapping.remove_prefix("")
        original.save_to_file(f.name, s)

        actual = pyhornedowl.open_ontology_from_file(f.name, s)

        assert_ontologies_equal(original, actual)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_write_simple_to_string(s):
    original = simple_ontology()
    original.prefix_mapping.remove_prefix("")
    o = original.save_to_string(s)

    actual = pyhornedowl.open_ontology_from_string(o, s)

    assert_ontologies_equal(original, actual)


@pytest.mark.parametrize("s", SERIALIZATIONS)
def test_prefix(s):
    if s == 'owl':
        pytest.skip('RDF/XML parser does not return prefixes')

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

    assert expected == actual
