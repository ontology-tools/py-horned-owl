import copy
import enum
import pickle
import sys
import warnings

import pytest
from pyhornedowl import PyIndexedOntology
from pyhornedowl.model import *

A_IRI = "https://example.com/A"
B_IRI = "https://example.com/B"
XSD = "http://www.w3.org/2001/XMLSchema#"


def test_iri_from_str():
    assert Class(A_IRI) == Class(IRI(A_IRI))
    assert Class(A_IRI).iri == IRI.parse(A_IRI)


def test_iri_from_entity():
    o = PyIndexedOntology()
    a = Class(A_IRI)
    assert AnnotationAssertion(a, Annotation(o.annotation_property(B_IRI), "x")).subject == IRI(A_IRI)


def test_class_expression_from_str():
    assert SubClassOf(A_IRI, B_IRI) == SubClassOf(Class(A_IRI), Class(B_IRI))


def test_curie_still_expanded_by_ontology():
    o = PyIndexedOntology()
    o.add_prefix_mapping("ex", "https://example.com/")
    assert o.class_("ex:A") == Class(A_IRI)


@pytest.mark.parametrize(
    "value, expected",
    [
        ("hi", SimpleLiteral("hi")),
        (True, DatatypeLiteral("true", XSD + "boolean")),
        (42, DatatypeLiteral("42", XSD + "integer")),
        (1.5, DatatypeLiteral("1.5", XSD + "double")),
        (float("inf"), DatatypeLiteral("INF", XSD + "double")),
    ],
)
def test_literal_from_python(value, expected):
    o = PyIndexedOntology()
    assert Annotation(o.annotation_property(A_IRI), value).value == expected


def test_keyword_arguments():
    assert SubClassOf(super_class=B_IRI, sub_class=A_IRI) == SubClassOf(A_IRI, B_IRI)


def test_missing_argument():
    with pytest.raises(TypeError, match="super_class"):
        SubClassOf(A_IRI)


def test_horned_owl_field_names_are_gone():
    assert not hasattr(SubClassOf(A_IRI, B_IRI), "sup")
    assert not hasattr(Class(A_IRI), "first")


def test_fields_are_read_only():
    with pytest.raises(AttributeError):
        SubClassOf(A_IRI, B_IRI).sub_class = Class(B_IRI)


def test_ordering():
    assert sorted([Class(B_IRI), Class(A_IRI)]) == [Class(A_IRI), Class(B_IRI)]


def test_repr_round_trips():
    ax = SubClassOf(Class(A_IRI), Class(A_IRI) & ~Class(B_IRI))
    assert eval(repr(ax)) == ax


def test_pickle_and_deepcopy():
    ax = SubClassOf(Class(A_IRI), Class(A_IRI) | Class(B_IRI))
    assert pickle.loads(pickle.dumps(ax)) == ax
    assert copy.deepcopy(ax) == ax


@pytest.mark.skipif(sys.version_info < (3, 13), reason="copy.replace is new in 3.13")
def test_copy_replace():
    ax = SubClassOf(A_IRI, B_IRI)
    assert copy.replace(ax, super_class=Class(A_IRI)) == SubClassOf(A_IRI, A_IRI)


def test_str_and_serialize():
    assert str(Class(A_IRI)) == A_IRI
    assert str(SubClassOf(A_IRI, B_IRI)) == f"SubClassOf(<{A_IRI}> <{B_IRI}>)"
    assert "SubClassOf" in SubClassOf(A_IRI, B_IRI).serialize("omn")


def test_class_expression_hierarchy():
    a, b = Class(A_IRI), Class(B_IRI)
    for ce in [a, a & b, a | b, ~a, ObjectOneOf([])]:
        assert isinstance(ce, ClassExpression)
    assert ObjectIntersectionOf.__mro__[1] is ClassExpression
    assert not isinstance(SubClassOf(a, b), ClassExpression)


def test_class_expression_is_abstract():
    with pytest.raises(TypeError):
        ClassExpression()


def test_operators_are_inherited_from_class_expression():
    a, b = Class(A_IRI), Class(B_IRI)
    assert "__and__" not in vars(Class) and "__and__" in vars(ClassExpression)
    assert (a & b) | ~a == ObjectUnionOf([ObjectIntersectionOf([a, b]), ObjectComplementOf(a)])


def test_operators_flatten():
    a, b, c, d = (Class(f"https://example.com/{n}") for n in "abcd")
    assert a & b & c == ObjectIntersectionOf([a, b, c])
    assert (a & b) & (c & d) == ObjectIntersectionOf([a, b, c, d])
    assert a | (b | c) == ObjectUnionOf([a, b, c])
    assert (a | b) & c == ObjectIntersectionOf([a | b, c])


def test_reflected_operators():
    b = Class(B_IRI)
    assert A_IRI & b == ObjectIntersectionOf([Class(A_IRI), b])
    assert A_IRI | b == ObjectUnionOf([Class(A_IRI), b])
    with pytest.raises(TypeError):
        1 & b


def test_literal_hierarchy():
    assert all(issubclass(c, Literal) for c in (SimpleLiteral, LanguageLiteral, DatatypeLiteral))
    with pytest.raises(TypeError):
        Literal()


def test_match_on_base_class():
    match Class(A_IRI) & Class(B_IRI):
        case ObjectUnionOf():
            assert False
        case ClassExpression():
            pass


def test_named_entity_protocol():
    assert isinstance(Class(A_IRI), NamedEntity)
    assert not isinstance(ObjectUnionOf([]), NamedEntity)


def test_iri_from_any_named_entity():
    class Thing:
        def __owl_entity_iri__(self) -> IRI:
            return IRI(A_IRI)

    thing = Thing()
    assert isinstance(thing, NamedEntity)
    assert issubclass(Thing, NamedEntity)
    assert Class(thing) == Class(A_IRI)
    assert PyIndexedOntology().iri(thing) == IRI(A_IRI)
    assert Annotation(AnnotationProperty(B_IRI), thing).value == IRI(A_IRI)


def test_every_entity_from_str():
    assert ObjectSomeValuesFrom("https://ex.org/p", A_IRI) == ObjectSomeValuesFrom(
        ObjectProperty(IRI("https://ex.org/p")), Class(A_IRI)
    )
    assert DataSomeValuesFrom(DataProperty("https://ex.org/d"), XSD + "integer").data_range == Datatype(
        XSD + "integer"
    )
    assert ClassAssertion(A_IRI, "https://ex.org/i").individual == NamedIndividual("https://ex.org/i")


def test_a_str_is_not_a_collection():
    with pytest.raises(TypeError):
        EquivalentClasses(A_IRI)
    assert EquivalentClasses([A_IRI, B_IRI]).class_expressions == [Class(A_IRI), Class(B_IRI)]


@pytest.mark.parametrize(
    "base, members",
    [
        ("Component", [SubClassOf(A_IRI, B_IRI), DeclareClass(A_IRI), Rule([], [])]),
        ("ObjectPropertyExpression", [ObjectProperty(A_IRI), InverseObjectProperty(A_IRI)]),
        ("DataRange", [Datatype(A_IRI), DataUnionOf([])]),
        ("Individual", [NamedIndividual(A_IRI), AnonymousIndividual("_:x")]),
        ("Atom", [SameIndividualAtom(Variable(A_IRI), Variable(B_IRI))]),
    ],
)
def test_hierarchies(base, members):
    base = globals()[base]
    for m in members:
        assert isinstance(m, base)
    with pytest.raises(TypeError):
        base()


def test_object_property_builders():
    p = ObjectProperty(A_IRI)
    assert p.some(B_IRI) == ObjectSomeValuesFrom(p, B_IRI)
    assert (~p).some(B_IRI) == ObjectSomeValuesFrom(InverseObjectProperty(p), B_IRI)
    assert ~~p == p
    assert p.min(2, B_IRI).cardinality == 2


def test_sub_object_property_expression():
    p, q = ObjectProperty(A_IRI), ObjectProperty(B_IRI)
    assert SubObjectPropertyOf(A_IRI, B_IRI).sub_property == p
    assert SubObjectPropertyOf([p, q], q).sub_property == [p, q]


def test_annotation_values():
    o = PyIndexedOntology()
    label = o.annotation_property(A_IRI)
    assert Annotation(label, "text").value == SimpleLiteral("text")
    assert Annotation(label, Class(B_IRI)).value == IRI(B_IRI)
    assert Annotation(label, "text").annotations == set()


def test_entity_str_is_its_iri():
    for entity in (Class, ObjectProperty, DataProperty, AnnotationProperty, Datatype, NamedIndividual):
        assert str(entity(A_IRI)) == A_IRI
        assert isinstance(entity(A_IRI), NamedEntity)


def test_ordering_across_subclasses():
    a = Class(A_IRI)
    # Like horned-owl's `ClassExpression`: `Class` is its first variant.
    assert sorted([ObjectUnionOf([]), a, ObjectIntersectionOf([])]) == [
        a,
        ObjectIntersectionOf([]),
        ObjectUnionOf([]),
    ]
    assert sorted([SubClassOf(A_IRI, B_IRI), DeclareClass(A_IRI)]) == [DeclareClass(A_IRI), SubClassOf(A_IRI, B_IRI)]
    assert Class(A_IRI) < Class(B_IRI) and not Class(B_IRI) <= Class(A_IRI)
    assert Class(A_IRI) != ObjectUnionOf([])
    with pytest.raises(TypeError):
        Class(A_IRI) < DataProperty(A_IRI)


def test_hash_matches_equality_across_subclasses():
    assert len({Class(A_IRI), Class(IRI(A_IRI)), ObjectUnionOf([]), ObjectUnionOf([])}) == 2
    assert hash(DeclareClass(A_IRI)) == hash(DeclareClass(A_IRI))


def test_pickle_facet():
    restriction = DatatypeRestriction(XSD + "integer", [FacetRestriction(Facet.MinInclusive, 1)])
    assert pickle.loads(pickle.dumps(Facet.MaxLength)) is Facet.MaxLength
    assert copy.deepcopy(restriction) == restriction


class _Number(float):
    def __repr__(self):
        return "_Number(...)"


class _Count(int):
    def __str__(self):
        return "many"


class _Size(enum.IntEnum):
    LARGE = 3


@pytest.mark.parametrize(
    "value, expected",
    [(_Number(1.5), "1.5"), (_Count(7), "7"), (_Size.LARGE, "3")],
)
def test_literal_from_number_subclass(value, expected):
    o = PyIndexedOntology()
    assert Annotation(o.annotation_property(A_IRI), value).value.literal == expected


def test_property_expression_from_str_warns():
    with pytest.warns(UserWarning, match="ObjectProperty"):
        key = HasKey(A_IRI, [B_IRI])
    assert key.property_expressions == [ObjectProperty(B_IRI)]
    with warnings.catch_warnings():
        warnings.simplefilter("error")
        assert HasKey(A_IRI, [DataProperty(B_IRI)]).property_expressions == [DataProperty(B_IRI)]


def test_ontology_methods_take_named_entities():
    o = PyIndexedOntology()
    o.add_prefix_mapping("ex", "https://example.com/")
    assert o.class_(Class(A_IRI)) == Class(A_IRI)
    assert o.iri(ObjectProperty(A_IRI)) == IRI(A_IRI)
    assert o.iri("ex:A") == IRI(A_IRI)


def test_ambiguous_property_warning_can_be_an_error():
    with warnings.catch_warnings():
        warnings.simplefilter("error", UserWarning)
        with pytest.raises(UserWarning, match="explicit ObjectProperty"):
            HasKey(A_IRI, [B_IRI])


@pytest.mark.parametrize("entity", [Class, ObjectProperty, DataProperty, AnnotationProperty, Datatype, NamedIndividual])
def test_ontology_iri_accepts_all_entity_kinds(entity):
    o = PyIndexedOntology()
    assert o.iri(entity(A_IRI)) == IRI(A_IRI)


def test_ontology_iri_preserves_explicit_tuple_handling():
    o = PyIndexedOntology()
    o.add_prefix_mapping("ex", "https://example.com/")
    assert o.iri(("ex:A", False)) == IRI(A_IRI)
    assert o.iri(("ex:A", True)) == IRI("ex:A")
    # An entity's IRI is already resolved, even if it resembles a CURIE.
    assert o.iri(Class("ex:A")) == IRI("ex:A")


@pytest.mark.parametrize("facet", [getattr(Facet, name) for name in (
    "Length", "MinLength", "MaxLength", "Pattern", "MinInclusive", "MinExclusive",
    "MaxInclusive", "MaxExclusive", "TotalDigits", "FractionDigits", "LangRange",
)])
def test_all_facets_pickle_by_name(facet):
    assert pickle.loads(pickle.dumps(facet)) is facet
    assert copy.deepcopy(facet) is facet


def test_nested_connectives_flatten_without_changing_order_or_duplicates():
    a, b = Class(A_IRI), Class(B_IRI)
    assert ObjectIntersectionOf([a, ObjectIntersectionOf([b, a])]) & b == ObjectIntersectionOf([a, b, a, b])
    assert a | ObjectUnionOf([b, ObjectUnionOf([a, b])]) == ObjectUnionOf([a, b, a, b])


@pytest.mark.parametrize("left, right", [
    (Class(A_IRI), ObjectUnionOf([])),
    (DeclareClass(A_IRI), SubClassOf(A_IRI, B_IRI)),
    (ObjectProperty(A_IRI), InverseObjectProperty(A_IRI)),
    (Datatype(A_IRI), DataUnionOf([])),
    (SimpleLiteral("a"), LanguageLiteral("a", "en")),
    (NamedIndividual(A_IRI), AnonymousIndividual("_:b")),
])
def test_all_comparison_operators_across_variants(left, right):
    assert (left < right) != (right < left)
    assert (left <= right) == (left < right)
    assert (left > right) == (right < left)
    assert (left >= right) == (left > right)
    assert left != right
    assert not (left == right)


def test_pickle_axiom_containing_datatype_restriction():
    restriction = DatatypeRestriction(XSD + "integer", [FacetRestriction(Facet.MinInclusive, 1)])
    axiom = SubClassOf(A_IRI, DataSomeValuesFrom(DataProperty(B_IRI), restriction))
    assert pickle.loads(pickle.dumps(axiom)) == axiom
    assert copy.deepcopy(axiom) == axiom


@pytest.mark.parametrize("entity_type", [Class, ObjectProperty, DataProperty, AnnotationProperty, Datatype, NamedIndividual])
def test_native_named_entity_hook(entity_type):
    entity = entity_type(A_IRI)
    assert entity.__owl_entity_iri__() == entity.iri == IRI(A_IRI)
    assert isinstance(entity, NamedEntity)
    assert issubclass(entity_type, NamedEntity)
    assert Class(entity) == Class(A_IRI)
    assert PyIndexedOntology().iri(entity) == IRI(A_IRI)


@pytest.mark.parametrize("value", [Variable(A_IRI), Import(A_IRI), OntologyID(A_IRI), DocIRI(A_IRI)])
def test_non_entity_iri_fields_do_not_opt_in(value):
    assert not isinstance(value, NamedEntity)
    with pytest.raises(TypeError, match="named entity"):
        Class(value)
    with pytest.raises(TypeError, match="named entity"):
        PyIndexedOntology().iri(value)
    with pytest.raises(TypeError):
        Annotation(AnnotationProperty(B_IRI), value)


def test_plain_iri_attribute_is_not_a_named_entity():
    class HasIRI:
        iri = IRI(A_IRI)

    value = HasIRI()
    assert not isinstance(value, NamedEntity)
    with pytest.raises(TypeError, match="named entity"):
        Class(value)
    with pytest.raises(TypeError, match="named entity"):
        PyIndexedOntology().iri(value)


@pytest.mark.parametrize("result", [None, A_IRI, Class(A_IRI)])
def test_named_entity_hook_requires_an_actual_iri(result):
    class BadEntity:
        def __owl_entity_iri__(self):
            return result

    with pytest.raises(TypeError, match=r"__owl_entity_iri__\(\) must return an IRI"):
        Class(BadEntity())
    with pytest.raises(TypeError, match=r"__owl_entity_iri__\(\) must return an IRI"):
        PyIndexedOntology().iri(BadEntity())


def test_named_entity_hook_must_be_callable():
    class BadEntity:
        __owl_entity_iri__ = IRI(A_IRI)

    with pytest.raises(TypeError, match="not callable"):
        Class(BadEntity())


def test_named_entity_hook_errors_propagate():
    class BrokenEntity:
        def __owl_entity_iri__(self):
            raise ValueError("broken entity")

    with pytest.raises(ValueError, match="broken entity"):
        Class(BrokenEntity())
    with pytest.raises(ValueError, match="broken entity"):
        PyIndexedOntology().iri(BrokenEntity())


def test_named_entity_hook_lookup_errors_propagate():
    class BrokenEntity:
        @property
        def __owl_entity_iri__(self):
            raise ValueError("broken lookup")

    with pytest.raises(ValueError, match="broken lookup"):
        Class(BrokenEntity())
