import os

import pyhornedowl
from pyhornedowl.model import *

RDFS_LABEL = "http://www.w3.org/2000/01/rdf-schema#label"


def r(*args: str) -> str:
    return os.path.abspath(os.path.join(os.path.dirname(__file__), "resources", *args))


def res(resource: str) -> str:
    with open(r(resource)) as f:
        return f.read()


def simple_ontology_comps() -> list[Component]:
    return [
        DeclareClass(Class(IRI.parse("https://example.com/A"))),
        DeclareClass(Class(IRI.parse("https://example.com/B"))),
        DeclareClass(Class(IRI.parse("https://example.com/C"))),
        SubClassOf(
            Class(IRI.parse("https://example.com/A")),
            Class(IRI.parse("https://example.com/B"))),
        AnnotationAssertion(
            IRI.parse("https://example.com/A"),
            Annotation(AnnotationProperty(IRI.parse(RDFS_LABEL)),
                       SimpleLiteral("ClassA"))),
        AnnotationAssertion(
            IRI.parse("https://example.com/B"),
            Annotation(AnnotationProperty(IRI.parse(RDFS_LABEL)),
                       SimpleLiteral("ClassB")))
    ]


def simple_ontology_comps_annotated() -> list[AnnotatedComponent]:
    return [AnnotatedComponent(x, set()) for x in simple_ontology_comps()]


def simple_ontology() -> pyhornedowl.PyIndexedOntology:
    onto = pyhornedowl.PyIndexedOntology()
    onto.add_default_prefix_names()
    onto.add_prefix_mapping("", "https://example.com/")

    for c in simple_ontology_comps():
        onto.add_component(c)

    return onto
