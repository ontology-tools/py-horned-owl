"""Grouped signature snapshots and opt-in typed entity wrappers."""

import operator

import pytest

from pyhornedowl import Ontology, Signature
from pyhornedowl.model import (
    Class, NamedIndividual, ObjectProperty, DataProperty, AnnotationProperty,
    Datatype, EntityKind, IRI, NamedEntity, SubClassOf,
    SimpleLiteral, LanguageLiteral,
)
from pyhornedowl.vocab import XSD, RDF


KINDS = [
    (Class, EntityKind.Class, "classes"),
    (NamedIndividual, EntityKind.NamedIndividual, "named_individuals"),
    (ObjectProperty, EntityKind.ObjectProperty, "object_properties"),
    (DataProperty, EntityKind.DataProperty, "data_properties"),
    (AnnotationProperty, EntityKind.AnnotationProperty, "annotation_properties"),
    (Datatype, EntityKind.Datatype, "datatypes"),
]


def test_all_signature_groups_preserve_punning_and_types():
    o = Ontology()
    entities = [ty("https://example.org/shared") for ty, _, _ in KINDS]
    for entity in entities:
        o.declare(entity)
    signature = o.signature()
    assert isinstance(signature, Signature)
    assert len(signature) == 6
    assert list(signature) == entities  # One entity per kind, in group order.
    assert set(signature) == set(entities)
    assert "https://example.org/shared" not in signature
    for (ty, kind, field), entity in zip(KINDS, entities):
        assert isinstance(entity, NamedEntity)
        assert entity.entity_kind() == kind
        group = getattr(signature, field)()
        assert isinstance(group, set)
        assert len(group) == 1
        assert group == {entity}
        assert isinstance(next(iter(group)), ty)
        assert entity in group and entity in signature
        assert None not in group
        assert Class("https://example.org/absent") not in group
        for other in entities:
            assert (other in group) == (type(other) is ty)


def test_snapshot_groups_and_iterators_survive_edits_and_deletion():
    o = Ontology()
    a, b = Class("https://example.org/A"), Class("https://example.org/B")
    axiom = SubClassOf(a, b)
    o.add_axiom(axiom)
    signature = o.signature()
    group = signature.classes()
    iterator = iter(group)
    all_iterator = iter(signature)
    assert iter(iterator) is iterator
    first = next(iterator)
    assert first in {a, b}
    o.remove_axiom(axiom)
    assert not o.signature()
    assert not o.contains_entity(a)
    assert a in signature
    assert len(group) == 2
    del o, signature, group
    assert set(iterator) == {a, b} - {first}
    assert list(iterator) == []
    assert set(all_iterator) == {a, b}
    assert list(all_iterator) == []


def test_signature_snapshot_is_immutable_and_empty_groups_work():
    signature = Ontology().signature()
    assert not signature
    assert list(signature) == []
    for _, _, field in KINDS:
        group = getattr(signature, field)()
        assert group == set()
        with pytest.raises(AttributeError):
            setattr(signature, field, set())
        group.add(Class("https://example.org/A"))
        assert getattr(signature, field)() == set()
    assert not signature


@pytest.mark.parametrize("ty, kind, field", KINDS)
def test_signature_group_sets_are_independent_copies(ty, kind, field):
    o = Ontology()
    entity = ty("https://example.org/A")
    o.declare(entity)
    signature = o.signature()
    group = getattr(signature, field)()
    group.clear()
    group.add(ty("https://example.org/other"))
    assert getattr(signature, field)() == {entity}
    assert entity in signature
    assert set(signature) == {entity}


@pytest.mark.parametrize("ty, kind, field", KINDS)
def test_external_typed_entity_protocol(ty, kind, field):
    class Reference:
        def __owl_entity_iri__(self):
            return IRI("https://example.org/A")

        def entity_kind(self):
            return kind

    ref = Reference()
    assert isinstance(ref, NamedEntity)
    o = Ontology()
    assert not o.contains_entity(ref)
    assert not o.is_declared(ref)
    assert o.declare(ref)
    assert not o.declare(ref)
    assert o.contains_entity(ref) and o.is_declared(ref)
    signature = o.signature()
    assert ref in signature
    assert ty("https://example.org/A") in getattr(signature, field)()
    assert set(signature) == {ty("https://example.org/A")}
    assert o.iri(ref) == ref.__owl_entity_iri__()


def test_iri_only_wrappers_still_work_in_iri_arguments():
    class IRIReference:
        def __owl_entity_iri__(self):
            return IRI("https://example.org/A")

    ref = IRIReference()
    assert not isinstance(ref, NamedEntity)
    assert Ontology().iri(ref) == IRI("https://example.org/A")
    with pytest.raises(TypeError, match="NamedEntity"):
        Ontology().contains_entity(ref)


def test_named_entity_hooks_are_evaluated_once_per_operation():
    class Reference:
        def __init__(self):
            self.calls = []

        def entity_kind(self):
            self.calls.append("kind")
            return EntityKind.Class

        def __owl_entity_iri__(self):
            self.calls.append("iri")
            return IRI("https://example.org/A")

    ref = Reference()
    o = Ontology()
    assert o.declare(ref)
    signature = o.signature()
    for operation in [o.contains_entity, o.is_declared, o.declare,
                      signature.__contains__]:
        ref.calls.clear()
        operation(ref)
        assert ref.calls == ["kind", "iri"]


@pytest.mark.parametrize("kind", ["Class", None, 0])
def test_typed_protocol_rejects_invalid_kinds(kind):
    class BadReference:
        def entity_kind(self):
            return kind

        def __owl_entity_iri__(self):
            return IRI("https://example.org/A")

    for method in ["contains_entity", "is_declared", "declare"]:
        with pytest.raises(TypeError, match="must return an EntityKind"):
            getattr(Ontology(), method)(BadReference())


@pytest.mark.parametrize("iri", ["https://example.org/A", Class("https://example.org/A"), None])
def test_typed_protocol_requires_an_actual_iri(iri):
    class BadReference:
        def entity_kind(self):
            return EntityKind.Class

        def __owl_entity_iri__(self):
            return iri

    with pytest.raises(TypeError, match="must return an IRI"):
        Ontology().declare(BadReference())


def test_typed_protocol_propagates_hook_exceptions():
    class BadReference:
        def entity_kind(self):
            raise RuntimeError("broken kind")

    for method in ["contains_entity", "is_declared", "declare"]:
        with pytest.raises(RuntimeError, match="broken kind"):
            getattr(Ontology(), method)(BadReference())
    with pytest.raises(RuntimeError, match="broken kind"):
        BadReference() in Ontology().signature()

    class MissingIRI:
        def entity_kind(self):
            return EntityKind.Class

    with pytest.raises(TypeError, match="__owl_entity_iri__"):
        Ontology().declare(MissingIRI())


@pytest.mark.parametrize("kind", ["Class", Class, None, IRI("https://example.org/A")])
def test_typed_entity_kind_must_be_an_entity_kind(kind):
    class BadReference:
        def __owl_entity_iri__(self):
            return IRI("https://example.org/A")

        def entity_kind(self):
            return kind

    for operation in [Ontology().contains_entity, Ontology().is_declared, Ontology().declare]:
        with pytest.raises(TypeError, match="must return an EntityKind"):
            operation(BadReference())


@pytest.mark.parametrize("hook", ["__owl_entity_iri__", "entity_kind"])
def test_protocol_hook_errors_propagate(hook):
    class Reference:
        def __owl_entity_iri__(self):
            return IRI("https://example.org/A")

        def entity_kind(self):
            return EntityKind.Class

    def fail(self):
        raise RuntimeError("wrapper failed")

    setattr(Reference, hook, fail)
    ref = Reference()
    with pytest.raises(RuntimeError, match="wrapper failed"):
        Ontology().contains_entity(ref)
    with pytest.raises(RuntimeError, match="wrapper failed"):
        operator.contains(Ontology().signature(), ref)


@pytest.mark.parametrize("value", ["https://example.org/A", None, Class("https://example.org/A")])
def test_typed_entity_iri_must_be_an_iri(value):
    class Reference:
        def __owl_entity_iri__(self):
            return value

        def entity_kind(self):
            return EntityKind.Class

    with pytest.raises(TypeError, match="must return an IRI"):
        Ontology().declare(Reference())


def test_membership_recognizes_implicit_literal_datatypes_with_iri_index():
    o = Ontology()
    o.build_indexes()
    a = Class("https://example.org/A")
    o.add_annotation(a, "https://example.org/note", SimpleLiteral("hello"))
    o.add_annotation(a, "https://example.org/note", LanguageLiteral("bonjour", "fr"))
    assert o.contains_entity(Datatype(XSD.string))
    assert o.contains_entity(Datatype(RDF.langString))
    assert not o.contains_entity(Datatype(XSD.integer))
    assert o.signature().datatypes() == {Datatype(XSD.string), Datatype(RDF.langString)}
