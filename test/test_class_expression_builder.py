import pytest
from pyhornedowl import PyIndexedOntology
from pyhornedowl.model import *


def test_ce_not():
    o = PyIndexedOntology()
    c = o.clazz("https://example.com/A")

    expected = ObjectComplementOf(c)
    actual = ~c

    assert expected == actual


def test_ce_not_complex():
    o = PyIndexedOntology()
    c = ObjectUnionOf([o.clazz("https://example.com/A"), o.clazz("https://example.com/B")])

    expected = ObjectComplementOf(c)
    actual = ~c

    assert expected == actual


def test_ce_and():
    o = PyIndexedOntology()
    c1 = o.clazz("https://example.com/A")
    c2 = o.clazz("https://example.com/B")

    expected = ObjectIntersectionOf([c1, c2])
    actual = c1 & c2

    assert expected == actual


def test_ce_or():
    o = PyIndexedOntology()
    c1 = o.clazz("https://example.com/A")
    c2 = o.clazz("https://example.com/B")

    expected = ObjectUnionOf([c1, c2])
    actual = c1 | c2

    assert expected == actual


def test_op_not():
    o = PyIndexedOntology()
    op = o.object_property("https://example.com/op")

    expected = InverseObjectProperty(op)
    actual = ~op

    assert expected == actual


def test_ce_some():
    o = PyIndexedOntology()
    c = o.clazz("https://example.com/A")
    op = o.object_property("https://example.com/op")

    expected = ObjectSomeValuesFrom(op, c)
    actual = op.some(c)

    assert expected == actual


def test_ce_only():
    o = PyIndexedOntology()
    c = o.clazz("https://example.com/A")
    op = o.object_property("https://example.com/op")

    expected = ObjectAllValuesFrom(op, c)
    actual = op.only(c)

    assert expected == actual


def test_ce_min():
    o = PyIndexedOntology()
    c = o.clazz("https://example.com/A")
    op = o.object_property("https://example.com/op")

    expected = ObjectMinCardinality(2, op, c)
    actual = op.min(2, c)

    assert expected == actual


def test_ce_max():
    o = PyIndexedOntology()
    c = o.clazz("https://example.com/A")
    op = o.object_property("https://example.com/op")

    expected = ObjectMaxCardinality(2, op, c)
    actual = op.max(2, c)

    assert expected == actual


def test_ce_exact():
    o = PyIndexedOntology()
    c = o.clazz("https://example.com/A")
    op = o.object_property("https://example.com/op")

    expected = ObjectExactCardinality(2, op, c)
    actual = op.exact(2, c)

    assert expected == actual


def test_ce_has_value():
    o = PyIndexedOntology()
    op = o.object_property("https://example.com/op")
    i = o.named_individual("https://example.com/I")

    expected = ObjectHasValue(op, i)
    actual = op.has_value(i)

    assert expected == actual


def test_ce_has_self():
    o = PyIndexedOntology()
    op = o.object_property("https://example.com/op")

    expected = ObjectHasSelf(op)
    actual = op.has_self()

    assert expected == actual
