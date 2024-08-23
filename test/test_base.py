import os

import pyhornedowl
from pyhornedowl.model import *

from typing import List

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


def simple_ontology_comps_annotated() -> List[AnnotatedComponent]:
    return [AnnotatedComponent(x, set()) for x in simple_ontology_comps()]


def simple_ontology() -> pyhornedowl.PyIndexedOntology:
    onto = pyhornedowl.PyIndexedOntology()
    onto.prefix_mapping.add_default_prefix_names()
    onto.prefix_mapping.add_prefix("", "https://example.com/")

    for c in simple_ontology_comps():
        onto.add_component(c)

    return onto


# A             B
# |   \    \/   |   \
# A1   A2  AB  B1  B2
# |    |   |    |   |
# A11 A21 AB1  B11 B21
def hierarchical_ontology_comps() -> List[Component]:
    a = Class(IRI.parse("https://example.com/A"))
    b = Class(IRI.parse("https://example.com/B"))
    ab = Class(IRI.parse("https://example.com/AB"))
    a1 = Class(IRI.parse("https://example.com/A1"))
    a2 = Class(IRI.parse("https://example.com/A2"))
    b1 = Class(IRI.parse("https://example.com/B1"))
    b2 = Class(IRI.parse("https://example.com/B2"))
    ab1 = Class(IRI.parse("https://example.com/AB1"))
    a11 = Class(IRI.parse("https://example.com/A11"))
    a21 = Class(IRI.parse("https://example.com/A21"))
    b11 = Class(IRI.parse("https://example.com/B11"))
    b21 = Class(IRI.parse("https://example.com/B21"))
    return [*[DeclareClass(x) for x in [a, b, ab, a1, a2, b1, b2, ab1, a11, a21, b11, b21]],
            SubClassOf(sub=a1, sup=a),
            SubClassOf(sub=a2, sup=a),
            SubClassOf(sub=ab, sup=a),
            SubClassOf(sub=ab, sup=b),
            SubClassOf(sub=b1, sup=b),
            SubClassOf(sub=b2, sup=b),

            SubClassOf(sub=a11, sup=a1),
            SubClassOf(sub=a21, sup=a2),
            SubClassOf(sub=ab1, sup=ab),
            SubClassOf(sub=b11, sup=b1),
            SubClassOf(sub=b21, sup=b2)
            ]


def hierarchical_ontology() -> pyhornedowl.PyIndexedOntology:
    onto = pyhornedowl.PyIndexedOntology()
    onto.prefix_mapping.add_default_prefix_names()
    onto.prefix_mapping.add_prefix("", "https://example.com/")

    for c in hierarchical_ontology_comps():
        onto.add_component(c)

    return onto
