import pytest
from pyhornedowl import PyIndexedOntology
from pyhornedowl.model import *
from test_base import simple_ontology, RDFS_LABEL


def test_pattern_match_annotated_component():
    a = AnnotatedComponent(DeclareClass(Class(IRI.parse("http://example.com/A"))), set())
    match a:
        case AnnotatedComponent(_, _):
            pass
        case _:
            pytest.fail("Pattern match failed for AnnotatedComponent")


def test_pattern_match_component():
    iri = IRI.parse("http://example.com/example")
    
    components: list[Component] = [
        OntologyID(None, None),
        DocIRI(iri),
        OntologyAnnotation(Annotation(AnnotationProperty(iri), SimpleLiteral("Example"))),
        Import(iri),
        DeclareClass(Class(iri)),
        SubClassOf(Class(iri), Class(iri)),
        FunctionalObjectProperty(ObjectProperty(iri)),
    ]
    
    for component in components:
        match component:
            case OntologyID(_, _):
                pass
            case DocIRI(_):
                pass
            case OntologyAnnotation(_):
                pass
            case Import(_):
                pass
            case DeclareClass(_):
                pass
            case SubClassOf(_, _):
                pass
            case FunctionalObjectProperty(_):
                pass
            case _:
                pytest.fail(f"Pattern match failed for {component.__class__.__name__}")


def test_pattern_match_nested():
    a = AnnotatedComponent(
        SubClassOf(
            Class(IRI.parse("http://example.com/A")),
            Class(IRI.parse("http://example.com/B"))
        ),
        {
            Annotation(
                AnnotationProperty(IRI.parse("http://example.com/annotation")),
                SimpleLiteral("Example Annotation")
            )
        }
    )
    
    match a:
        case AnnotatedComponent(SubClassOf(Class(iri1),Class(iri2)), anns):
            assert str(iri1) == "http://example.com/A"
            assert str(iri2) == "http://example.com/B"
            
            match list(anns):
                case [Annotation(AnnotationProperty(property), SimpleLiteral(value))]:
                    assert str(property) == "http://example.com/annotation"
                    assert value == "Example Annotation"
                case _:
                    pytest.fail("Pattern match failed for annotations in nested AnnotatedComponent")
            
        case _:
            pytest.fail("Pattern match failed for nested AnnotatedComponent") 
