import unittest

from pyhornedowl import PyIndexedOntology
from pyhornedowl.model import *
from test_base import simple_ontology, RDFS_LABEL


class PatternMatchTestCase(unittest.TestCase):
    def test_pattern_match_annotated_component(self):
        a = AnnotatedComponent(DeclareClass(Class(IRI.parse("http://example.com/A"))), set())
        match a:
            case AnnotatedComponent(_, _):
                pass
            case _:
                self.fail("Pattern match failed for AnnotatedComponent")
                
    def test_pattern_match_component(self):
        iri = IRI.parse("http://example.com/example")
        
        components: list[Component] = [
            OntologyID(None, None),
            DocIRI(IRI.parse(iri)),
            OntologyAnnotation(Annotation(iri, SimpleLiteral("Example"))),
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
                    self.fail(f"Pattern match failed for {component.__class__.__name__}")
                    
    def test_pattern_match_nested(self):
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
                self.assertEqual(str(iri1), "http://example.com/A")
                self.assertEqual(str(iri2), "http://example.com/B")
                
                match list(anns):
                    case [Annotation(AnnotationProperty(property), SimpleLiteral(value))]:
                        self.assertEqual(str(property), "http://example.com/annotation")
                        self.assertEqual(value, "Example Annotation")
                    case _:
                        self.fail("Pattern match failed for annotations in nested AnnotatedComponent")
                
            case _:
                self.fail("Pattern match failed for nested AnnotatedComponent") 