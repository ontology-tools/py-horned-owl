"""Loading and saving the OMN (Manchester) and OBO serializations."""

import pytest

import pyhornedowl
from test_base import res, simple_ontology

# `simple.omn` in test/resources relies on the prefix names the OWL 2 Manchester
# Syntax spec predefines (rdf, rdfs, xsd, owl). horned-owl's OMN reader does not
# predefine them yet, so this fixture spells them out; see
# test_predefined_prefixes_are_not_yet_supported below.
SIMPLE_OMN = """Prefix: : <https://example.com/>
Prefix: rdfs: <http://www.w3.org/2000/01/rdf-schema#>
Prefix: owl: <http://www.w3.org/2002/07/owl#>

Ontology:
    Class: :A
        Annotations: rdfs:label "ClassA"
    Class: :B
        Annotations: rdfs:label "ClassB"
        SubClassOf: :A
    Class: :C
    Class: :D
        SubClassOf: :B
        SubClassOf: owl:Thing
"""

SIMPLE_OBO = """format-version: 1.2
ontology: test

[Term]
id: EX:1
name: Alpha

[Term]
id: EX:2
name: Beta
is_a: EX:1
"""


def assert_ontologies_equal(actual, expected):
    assert set(expected.get_axioms()) == set(actual.get_axioms()), "Axioms do not match!"
    assert expected.get_iri() == actual.get_iri(), "Ontology IRIs do not match!"


@pytest.mark.parametrize("serialization", ["omn", "manchester"])
def test_load_omn_explicit(serialization):
    actual = pyhornedowl.open_ontology(SIMPLE_OMN, serialization)

    assert_ontologies_equal(actual, simple_ontology())


def test_load_omn_guessed_from_content():
    actual = pyhornedowl.open_ontology(SIMPLE_OMN)

    assert_ontologies_equal(actual, simple_ontology())


def test_save_omn_round_trips():
    expected = simple_ontology()

    actual = pyhornedowl.open_ontology(expected.save_to_string("omn"), "omn")

    assert_ontologies_equal(actual, expected)


def test_load_obo_explicit():
    onto = pyhornedowl.open_ontology(SIMPLE_OBO, "obo")

    assert any(
        "EX_1" in pyhornedowl.to_manchester(ac) for ac in onto.get_axioms()
    ), "expected the OBO term EX:1 to map to an IRI"


def test_load_obo_guessed_from_content():
    explicit = pyhornedowl.open_ontology(SIMPLE_OBO, "obo")
    guessed = pyhornedowl.open_ontology(SIMPLE_OBO)

    # Compares axioms and the ontology IRI as a string: pyhornedowl.model.IRI has
    # no __eq__, so IRI comparison falls back to object identity.
    assert set(guessed.get_axioms()) == set(explicit.get_axioms())
    assert str(guessed.get_iri()) == str(explicit.get_iri())


def test_unknown_serialization_still_rejected():
    with pytest.raises(ValueError):
        pyhornedowl.open_ontology(res("simple.owl"), "definitely-not-a-serialization")


@pytest.mark.xfail(
    reason="horned-owl's OMN reader does not predefine the rdf/rdfs/xsd/owl prefix "
           "names that the OWL 2 Manchester Syntax spec declares predefined. Fixing "
           "that belongs in horned-owl's io::omn::reader, not in this binding. Once "
           "fixed, test/resources/simple.omn loads as-is and this passes.",
    raises=ValueError,
    strict=True,
)
def test_predefined_prefixes_are_not_yet_supported():
    pyhornedowl.open_ontology(res("simple.omn"), "omn")
