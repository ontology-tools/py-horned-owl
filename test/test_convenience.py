"""Convenience API contracts, including edits with active indexes."""

import pytest

from pyhornedowl import IndexCreationStrategy, Ontology, open_ontology_from_string
from pyhornedowl.model import (
    AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationProperty,
    AnonymousIndividual, Class, ClassAssertion, Component, DataProperty,
    DataPropertyAssertion, DataSomeValuesFrom, Datatype, DatatypeLiteral,
    DeclareClass, DeclareDatatype, IRI, Import, LanguageLiteral, NamedIndividual,
    ObjectProperty, OntologyAnnotation, OntologyID, Rule, SimpleLiteral, SubClassOf,
)
from pyhornedowl.vocab import RDFS, XSD


@pytest.fixture(params=["on_query", "on_load", "explicit", "indexed", "component_index", "iri_index"])
def o(request):
    strategy = {
        "on_query": IndexCreationStrategy.OnQuery,
        "on_load": IndexCreationStrategy.OnLoad,
        "explicit": IndexCreationStrategy.Explicit,
        "indexed": IndexCreationStrategy.Explicit,
        "component_index": IndexCreationStrategy.Explicit,
        "iri_index": IndexCreationStrategy.Explicit,
    }[request.param]
    ontology = Ontology(index_strategy=strategy)
    ontology.add_prefix_mapping("ex", "https://example.org/")
    if request.param in {"on_load", "indexed"}:
        ontology.build_indexes()
    elif request.param == "component_index":
        ontology.build_component_index()
    elif request.param == "iri_index":
        ontology.build_iri_index()
    return ontology


@pytest.fixture
def entities(o):
    return o.class_("ex:A"), o.class_("ex:B"), o.object_property("ex:p")


def test_copy_and_remove_annotated_variants(o, entities):
    a, b, _ = entities
    body = SubClassOf(a, b)
    source = o.annotation_property("ex:source")
    one = AnnotatedComponent(body, {Annotation(source, "one")})
    two = AnnotatedComponent(body, {Annotation(source, "two")})
    assert o.add_axiom(one)
    assert not o.add_axiom(one)
    assert o.add_axioms([one, two, body, body]) == 2
    assert set(o.query_components(SubClassOf, including=a)) == {one, two, AnnotatedComponent(body)}
    target = Ontology()
    assert target.add_axioms(o.get_axioms()) == 3
    assert set(target.get_axioms()) == set(o.get_axioms())
    assert o.contains_axiom(one)
    assert o.contains_axiom(body)
    assert o.remove_axiom(one)
    assert not o.remove_axiom(one)
    assert o.contains_axiom(body, ignore_annotations=True)
    assert o.remove_components([body, body]) == 1
    assert not o.remove_axiom(body)
    assert o.contains_axiom(body, ignore_annotations=True)
    assert o.remove_components([body, body], ignore_annotations=True) == 1
    assert not o.contains_axiom(body, ignore_annotations=True)
    assert not o.query_components(including=a)
    assert not o.signature()


def test_exact_membership_and_annotation_conflict(o, entities):
    a, b, _ = entities
    body = SubClassOf(a, b)
    annotated = AnnotatedComponent(body, {Annotation(o.annotation_property("ex:source"), "one")})
    assert o.add_component(annotated)
    assert not o.contains_axiom(body)
    assert o.contains_axiom(body, ignore_annotations=True)
    assert not o.contains_axiom(AnnotatedComponent(body))
    with pytest.raises(TypeError):
        o.add_axiom(annotated, annotations=[])
    assert o.get_axioms() == [annotated]
    assert o.remove_component(annotated)


def test_ignore_annotations_for_add_remove_and_contains(o, entities):
    a, b, _ = entities
    body = SubClassOf(a, b)
    ap = o.annotation_property("ex:source")
    one = AnnotatedComponent(body, {Annotation(ap, "one")})
    two = AnnotatedComponent(body, {Annotation(ap, "two")})
    assert o.add_axiom(one, ignore_annotations=True)
    assert not o.add_axiom(two, ignore_annotations=True)
    assert not o.add_component(body, ignore_annotations=True)
    assert o.get_axioms() == [one]  # Insertion preserves supplied annotations.
    assert not o.contains_axiom(body)
    assert not o.contains_axiom(two)
    assert o.contains_axiom(two, ignore_annotations=True)
    assert not o.remove_axiom(body)
    assert not o.remove_component(two)
    assert o.add_component(two)
    assert o.remove_component(body, ignore_annotations=True)
    assert not o.get_axioms()
    assert o.add_axioms([one, two, body], ignore_annotations=True) == 1
    assert o.get_axioms() == [one]
    assert o.remove_components([body]) == 0
    assert o.remove_axiom(two, ignore_annotations=True)
    assert not o.remove_axiom(body, ignore_annotations=True)


def test_bulk_validation_before_mutation(o, entities):
    a, b, _ = entities
    body = SubClassOf(a, b)
    for add in [o.add_axioms, o.add_components]:
        with pytest.raises(TypeError):
            add([body, object()])
        assert not o.get_components()
    o.add_axiom(body)
    with pytest.raises(TypeError):
        o.remove_components([body, object()])
    assert o.contains_axiom(body)
    # Bulk operations accept sequences; generators are rejected before editing.
    for operation in [o.add_axioms, o.add_components, o.remove_components]:
        with pytest.raises(TypeError):
            operation(item for item in [body])
        assert o.contains_axiom(body)
    with pytest.raises(TypeError):
        o.add_axioms("not a collection of axioms")


def test_component_queries_include_nested_references_and_annotations(o, entities):
    a, b, p = entities
    body = SubClassOf(a, p.some(b))
    note = o.annotation_property("ex:note")
    provenance = o.iri("ex:provenance")
    annotated = AnnotatedComponent(body, {Annotation(note, provenance)})
    other = SubClassOf(b, a)
    o.add_axioms([annotated, other, DeclareClass(a)])
    metadata = [OntologyID(provenance), Import(provenance), Rule([], [])]
    assert o.add_components(metadata) == 3
    assert set(o.query_components(SubClassOf)) == {annotated, AnnotatedComponent(other)}
    assert o.query_components(including=p) == [annotated]
    assert o.query_components(SubClassOf, including="ex:provenance") == [annotated]
    assert o.query_components(kind=SubClassOf, including=provenance) == [annotated]
    assert o.query_components(SubClassOf, including=note) == [annotated]
    assert o.query_components(DeclareClass, including=b) == []
    assert set(o.query_components(Component)) == set(o.get_components())
    assert set(o.query_components()) == set(o.get_components())
    assert o.query_components(Import) == [AnnotatedComponent(Import(provenance))]
    assert o.query_components(Rule) == [AnnotatedComponent(Rule([], []))]
    assert o.query_components(OntologyID) == [AnnotatedComponent(OntologyID(provenance))]
    assert set(o.query_components(including=provenance)) == {
        annotated, AnnotatedComponent(Import(provenance)),
        AnnotatedComponent(OntologyID(provenance)),
    }
    assert AnnotatedComponent(Rule([], [])) not in o.get_axioms()
    assert AnnotatedComponent(OntologyID(provenance)) not in o.get_axioms()
    for ty in Component.__subclasses__():
        o.query_components(ty)
    for invalid in [Class, str, SubClassOf(a, b), "SubClassOf"]:
        with pytest.raises(TypeError):
            o.query_components(invalid)
    with pytest.raises(ValueError, match="CURIE"):
        o.query_components(including="unknown:A")
    with pytest.raises(TypeError):
        o.query_components(mentioning=a)


def test_signature_retains_types_and_nested_entities(o, entities):
    a, b, p = entities
    individual = o.named_individual("ex:A")  # OWL punning.
    dp = o.data_property("ex:age")
    ap = o.annotation_property("ex:source")
    nested = o.annotation_property("ex:nested")
    target = o.iri("ex:onlyAnnotationValue")
    o.add_axioms([
        SubClassOf(a, p.some(b) & DataSomeValuesFrom(dp, XSD.integer)),
        ClassAssertion(a, individual),
        DataPropertyAssertion(dp, individual, 5),
        AnnotatedComponent(DeclareClass(b), {Annotation(ap, target, {Annotation(nested, True)})}),
    ])
    o.add_component(OntologyAnnotation(Annotation(ap, target)))
    expected = {a, b, p, individual, dp, ap, nested, Datatype(XSD.integer), Datatype(XSD.boolean)}
    assert set(o.signature()) == expected
    assert o.signature().classes() == {a, b}
    assert o.signature().named_individuals() == {individual}
    assert o.get_classes() == {b}  # Only declared entities are listed.
    assert o.contains_entity(a) and o.contains_entity(individual)
    assert not o.contains_entity(Class(target))
    assert not o.is_declared(a) and o.is_declared(b)
    assert not o.is_declared(individual)
    for invalid in [Class, target, str(a.iri), AnonymousIndividual("_:x")]:
        with pytest.raises(TypeError):
            o.contains_entity(invalid)
        with pytest.raises(TypeError):
            o.is_declared(invalid)
    with pytest.raises(TypeError):
        o.signature(IRI)


@pytest.mark.parametrize("ty", [Class, ObjectProperty, DataProperty, AnnotationProperty, Datatype, NamedIndividual])
def test_declare_typed_entities(o, ty):
    entity = ty(o.iri("ex:E"))
    assert not o.is_declared(entity)
    assert o.declare(entity)
    assert not o.declare(entity)
    assert o.is_declared(entity) and o.contains_entity(entity)
    assert set(o.signature()) == {entity}


def test_datatype_and_individual_factories(o):
    datatype = o.datatype("ex:Age")
    assert datatype == Datatype("https://example.org/Age")
    assert not o.is_declared(datatype)
    assert o.declare_datatype("ex:Age")
    assert o.contains_axiom(DeclareDatatype(datatype))
    assert o.declare_individual("ex:i")
    assert not o.declare_individual("ex:i")


def test_annotations_preserve_values_and_axiom_annotations(o, entities):
    a, _, _ = entities
    ap = o.annotation_property("ex:note")
    values = [SimpleLiteral("hello"), LanguageLiteral("hello", "en"),
              DatatypeLiteral("3", XSD.integer), o.iri("ex:target"), AnonymousIndividual("_:value")]
    provenance = {Annotation(o.annotation_property("ex:source"), "curated")}
    for value in values:
        assert o.add_annotation(a, "ex:note", value, annotations=provenance)
        assert not o.add_annotation(a, ap, value, annotations=provenance)
    assert set(o.get_annotations(a, ap)) == set(values)
    assert set(o.get_annotations(str(a.iri), ap)) == set(values)
    assert o.get_annotations("ex:A", ap) == []  # Subjects are already-resolved IRIs.
    assertions = o.query_components(AnnotationAssertion, including=a)
    assert len(assertions) == 5
    assert all(ax.annotations == provenance for ax in assertions)
    assert set(o.get_annotations(a)) == set(values)
    assert o.get_annotations(a, "ex:missing") == []
    # A reference to a in an annotation value is not an annotation *on* a.
    o.add_annotation(o.iri("ex:other"), ap, a)
    assert len(o.get_annotations(a, ap)) == 5
    assert o.remove_annotation(a, ap, values[0]) == 1
    assert o.remove_annotation(a, ap, values[0]) == 0
    assert set(o.get_annotations(a, ap)) == set(values[1:])
    extra = o.annotation_property("ex:extra")
    o.add_annotation(a, extra, True)
    assert set(o.get_annotations(a)) == {*values[1:], DatatypeLiteral("true", XSD.boolean)}
    assert set(o.get_annotations(a, ap)) == set(values[1:])


def test_remove_annotation_all_variants(o, entities):
    a, _, _ = entities
    ap = o.annotation_property("ex:note")
    nested = {Annotation(ap, "nested")}
    assertion = AnnotationAssertion(a, Annotation(ap, "value", nested))
    o.add_axioms([assertion, AnnotatedComponent(assertion, nested)])
    o.add_annotation(a, ap, "value")
    assert o.get_annotations(a, ap) == [SimpleLiteral("value")] * 3
    assert o.remove_annotation(a, ap, "value") == 3
    assert not o.get_annotations(a)
    assert not o.query_components(AnnotationAssertion, including=a)


def test_anonymous_annotation_subjects(o):
    subject = AnonymousIndividual("_:subject")
    assert o.add_annotation(subject, "ex:note", "value")
    assert o.get_annotations("_:subject", "ex:note") == [SimpleLiteral("value")]
    assert len(o.query_components(AnnotationAssertion)) == 1
    assert o.remove_annotation("_:subject", "ex:note", "value") == 1
    assert not o.get_annotations(subject)


def test_language_aware_labels(o, entities):
    a, b, _ = entities
    labels = [LanguageLiteral("Zulu", "en"), LanguageLiteral("Alpha", "EN"),
              LanguageLiteral("Deutsch", "de"), LanguageLiteral("British", "en-GB"),
              SimpleLiteral("Untagged"), DatatypeLiteral("Typed", XSD.string)]
    for value in labels:
        o.add_annotation(a, RDFS.label, value)
    iri_value = o.iri("ex:notALabel")
    o.add_annotation(a, RDFS.label, iri_value)
    o.set_label(b, "Alpha", language="en")
    # Label selection follows language preferences; order within a language is unspecified.
    assert o.get_label(a) in {"Typed", "Untagged"}
    assert o.get_label(a, ["en"]) in {"Alpha", "Zulu"}
    assert o.get_label(a, ["EN"]) in {"Alpha", "Zulu"}
    assert o.get_label(a, ["de", "en"]) == "Deutsch"
    assert o.get_label(a, languages=["en-GB"]) == "British"
    assert o.get_label(a, ["fr"], default="missing") == "missing"
    assert o.get_label(a, ["en-AU"]) is None
    assert o.get_label(a, []) is None
    assert o.get_label(o.class_("ex:missing")) is None
    assert set(o.get_annotations(a, RDFS.label)) == {*labels, iri_value}
    assert o.get_iri_for_label("Alpha") in {str(a.iri), str(b.iri)}
    o.set_label(a, "New English", language="en")
    assert o.get_label(a, ["en"]) == "New English"
    assert o.get_label(a, ["de"]) == "Deutsch"
    assert o.get_label(a, ["en-GB"]) == "British"
    o.set_label(a, "New Untagged")
    assert o.get_label(a) == "New Untagged"
    assert set(o.get_annotations(a, RDFS.label)) == {
        SimpleLiteral("New Untagged"), LanguageLiteral("New English", "en"),
        LanguageLiteral("Deutsch", "de"), LanguageLiteral("British", "en-GB"), iri_value,
    }


def test_edited_annotations_roundtrip(o, entities):
    a, _, _ = entities
    o.declare(a)
    o.set_label(a, "Name", language="en")
    o.add_annotation(a, "ex:seeAlso", o.iri("ex:target"))
    loaded = open_ontology_from_string(o.save_to_string("ofn"), "ofn")
    assert set(loaded.get_axioms()) == set(o.get_axioms())
    assert loaded.get_label(a, ["en"]) == "Name"
    assert loaded.get_annotations(a, o.iri("ex:seeAlso")) == [o.iri("ex:target")]
