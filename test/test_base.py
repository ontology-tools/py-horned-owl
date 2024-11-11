import os

from typing import List

import pyhornedowl
from pyhornedowl.model import *

RDFS_LABEL = "http://www.w3.org/2000/01/rdf-schema#label"


def r(*args: str) -> str:
    return os.path.abspath(os.path.join(os.path.dirname(__file__), "resources", *args))


def res(resource: str) -> str:
    with open(r(resource)) as f:
        return f.read()


def simple_ontology_comps() -> List[Component]:
    return [
        DeclareClass(Class(IRI.parse("https://example.com/A"))),
        DeclareClass(Class(IRI.parse("https://example.com/B"))),
        DeclareClass(Class(IRI.parse("https://example.com/C"))),
        DeclareClass(Class(IRI.parse("https://example.com/D"))),
        SubClassOf(
            sup=Class(IRI.parse("https://example.com/A")),
            sub=Class(IRI.parse("https://example.com/B"))),
        SubClassOf(
            sup=Class(IRI.parse("https://example.com/B")),
            sub=Class(IRI.parse("https://example.com/D"))),
        AnnotationAssertion(
            IRI.parse("https://example.com/A"),
            Annotation(AnnotationProperty(IRI.parse(RDFS_LABEL)),
                       SimpleLiteral("ClassA"))),
        AnnotationAssertion(
            IRI.parse("https://example.com/B"),
            Annotation(AnnotationProperty(IRI.parse(RDFS_LABEL)),
                       SimpleLiteral("ClassB")))
    ]


def simple_ontology_comps_annotated() -> List[AnnotatedComponent]:
    return [AnnotatedComponent(x, set()) for x in simple_ontology_comps()]


def simple_ontology() -> pyhornedowl.PyIndexedOntology:
    onto = pyhornedowl.PyIndexedOntology()
    onto.prefix_mapping.add_default_prefix_names()
    onto.prefix_mapping.add_prefix("", "https://example.com/")

    for c in simple_ontology_comps():
        onto.add_component(c)

    return onto
