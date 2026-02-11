"""Tests for the structural reasoner.

The structural reasoner traverses SubClassOf and SubObjectPropertyOf axiom chains
without performing any logical inference.
"""
import pytest
from test_base import simple_ontology

import pyhornedowl
from pyhornedowl.model import (
    Class,
    IRI,
    DeclareClass,
    DeclareObjectProperty,
    SubClassOf,
    SubObjectPropertyOf,
    ObjectProperty,
)
from pyhornedowl.reasoning import create_structural_reasoner

def property_hierarchy_ontology() -> pyhornedowl.PyIndexedOntology:
    """
    Creates an ontology with an object property hierarchy:
    
        P           R
        |
        Q
        |
        S
    """
    onto = pyhornedowl.PyIndexedOntology()
    onto.prefix_mapping.add_default_prefix_names()
    onto.prefix_mapping.add_prefix("", "https://example.com/")
    
    onto.add_component(DeclareObjectProperty(ObjectProperty(IRI.parse("https://example.com/P"))))
    onto.add_component(DeclareObjectProperty(ObjectProperty(IRI.parse("https://example.com/Q"))))
    onto.add_component(DeclareObjectProperty(ObjectProperty(IRI.parse("https://example.com/R"))))
    onto.add_component(DeclareObjectProperty(ObjectProperty(IRI.parse("https://example.com/S"))))

    onto.add_component(SubObjectPropertyOf(
        onto.object_property("https://example.com/Q"),
        onto.object_property("https://example.com/P"),
    ))
    onto.add_component(SubObjectPropertyOf(
        onto.object_property("https://example.com/S"),
        onto.object_property("https://example.com/Q"),
    ))
    
    return onto


class TestReasonerMetadata:
    """Tests for basic reasoner metadata methods."""

    def test_get_name(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        assert reasoner.get_name() == "StructuralReasoner"

    def test_get_version(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        # Version should be a non-empty string
        version = reasoner.get_version()
        assert isinstance(version, str)
        assert len(version) > 0

    def test_flush(self):
        """flush() should not raise an error for structural reasoner."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        # Should complete without error
        reasoner.flush()


class TestInferredAxioms:
    """Tests for inferred_axioms method."""

    def test_no_inferred_axioms(self):
        """Structural reasoner does not produce inferred axioms."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        inferred = reasoner.inferred_axioms()
        assert inferred == set()


class TestConsistency:
    """Tests for consistency checking."""

    def test_is_consistent_not_implemented(self):
        """Structural reasoner cannot determine consistency."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        with pytest.raises(ValueError):
            reasoner.is_consistent()


class TestSubclasses:
    """Tests for get_subclasses method."""

    def test_direct_subclass(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        class_a = Class(IRI.parse("https://example.com/A"))
        subclasses = reasoner.get_subclasses(class_a)
        
        # A has subclass B, which has subclass D (transitive)
        expected_iris = {"https://example.com/B", "https://example.com/D"}
        actual_iris = {str(c.first) for c in subclasses}
        
        assert actual_iris == expected_iris

    def test_no_subclasses(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        # D has no subclasses
        class_d = Class(IRI.parse("https://example.com/D"))
        subclasses = reasoner.get_subclasses(class_d)
        
        assert subclasses == set()

    def test_independent_class_no_subclasses(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        # C is independent, has no subclasses
        class_c = Class(IRI.parse("https://example.com/C"))
        subclasses = reasoner.get_subclasses(class_c)
        
        assert subclasses == set()

    def test_subclasses_intermediate_node(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        # B has subclass D
        class_b = Class(IRI.parse("https://example.com/B"))
        subclasses = reasoner.get_subclasses(class_b)
        
        expected_iris = {"https://example.com/D"}
        actual_iris = {str(c.first) for c in subclasses}
        
        assert actual_iris == expected_iris


class TestSuperclasses:
    """Tests for get_superclasses method."""

    def test_direct_superclass(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        class_d = Class(IRI.parse("https://example.com/D"))
        superclasses = reasoner.get_superclasses(class_d)
        
        # D has superclass B, which has superclass A (transitive)
        expected_iris = {"https://example.com/A", "https://example.com/B"}
        actual_iris = {str(c.first) for c in superclasses}
        
        assert actual_iris == expected_iris

    def test_no_superclasses(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        # A has no superclasses
        class_a = Class(IRI.parse("https://example.com/A"))
        superclasses = reasoner.get_superclasses(class_a)
        
        assert superclasses == set()

    def test_independent_class_no_superclasses(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        # C is independent, has no superclasses
        class_c = Class(IRI.parse("https://example.com/C"))
        superclasses = reasoner.get_superclasses(class_c)
        
        assert superclasses == set()

    def test_superclasses_intermediate_node(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        # B has superclass A
        class_b = Class(IRI.parse("https://example.com/B"))
        superclasses = reasoner.get_superclasses(class_b)
        
        expected_iris = {"https://example.com/A"}
        actual_iris = {str(c.first) for c in superclasses}
        
        assert actual_iris == expected_iris


class TestOwlThingAndNothing:
    """Tests for owl:Thing and owl:Nothing handling."""

    def test_subclasses_of_owl_thing(self):
        """owl:Thing should have all classes as subclasses."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        owl_thing = Class(IRI.parse("http://www.w3.org/2002/07/owl#Thing"))
        subclasses = reasoner.get_subclasses(owl_thing)
        
        # All classes should be subclasses of owl:Thing
        actual_iris = {str(c.first) for c in subclasses}
        
        # At minimum, root classes (A and C) should be included
        assert "https://example.com/A" in actual_iris
        assert "https://example.com/C" in actual_iris

    def test_superclasses_of_owl_thing(self):
        """owl:Thing has no superclasses."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        owl_thing = Class(IRI.parse("http://www.w3.org/2002/07/owl#Thing"))
        superclasses = reasoner.get_superclasses(owl_thing)
        
        assert superclasses == set()

    def test_subclasses_of_owl_nothing(self):
        """owl:Nothing has no subclasses."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        owl_nothing = Class(IRI.parse("http://www.w3.org/2002/07/owl#Nothing"))
        subclasses = reasoner.get_subclasses(owl_nothing)
        
        assert subclasses == set()


class TestEquivalentClasses:
    """Tests for get_equivalent_classes method."""

    def test_equivalent_classes_not_implemented(self):
        """Structural reasoner does not compute equivalent classes."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        class_a = Class(IRI.parse("https://example.com/A"))
        
        with pytest.raises(ValueError):
            reasoner.get_equivalent_classes(class_a)


class TestDisjointClasses:
    """Tests for get_disjoint_classes method."""

    def test_disjoint_classes_not_implemented(self):
        """Structural reasoner does not compute disjoint classes."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        class_a = Class(IRI.parse("https://example.com/A"))
        
        with pytest.raises(ValueError):
            reasoner.get_disjoint_classes(class_a)


class TestUnsatisfiableClasses:
    """Tests for get_unsatisfiable_classes method."""

    def test_unsatisfiable_classes_not_implemented(self):
        """Structural reasoner cannot compute unsatisfiable classes."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        with pytest.raises(ValueError):
            reasoner.get_unsatisfiable_classes()


class TestIsEntailed:
    """Tests for is_entailed method."""

    def test_entailment_not_implemented(self):
        """Structural reasoner does not support entailment checking."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        axiom = SubClassOf(
            onto.clazz("https://example.com/B"),
            onto.clazz("https://example.com/A")
        )
        
        with pytest.raises(ValueError):
            reasoner.is_entailed(axiom)


class TestIsSatisfiable:
    """Tests for is_satisfiable method."""

    def test_satisfiability_not_implemented(self):
        """Structural reasoner does not support satisfiability checking."""
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        class_a = Class(IRI.parse("https://example.com/A"))
        
        with pytest.raises(ValueError):
            reasoner.is_satifisable(class_a)


class TestWithSimpleOntology:
    """Tests using the simple_ontology from test_base."""

    def test_subclasses_with_simple_ontology(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        class_a = Class(IRI.parse("https://example.com/A"))
        subclasses = reasoner.get_subclasses(class_a)
        
        expected_iris = {"https://example.com/B", "https://example.com/D"}
        actual_iris = {str(c.first) for c in subclasses}
        
        assert actual_iris == expected_iris

    def test_superclasses_with_simple_ontology(self):
        onto = simple_ontology()
        reasoner = create_structural_reasoner(onto)
        
        class_d = Class(IRI.parse("https://example.com/D"))
        superclasses = reasoner.get_superclasses(class_d)
        
        expected_iris = {"https://example.com/A", "https://example.com/B"}
        actual_iris = {str(c.first) for c in superclasses}
        
        assert actual_iris == expected_iris


class TestMultipleInheritance:
    """Tests for ontologies with multiple inheritance."""

    def test_diamond_hierarchy(self):
        """Test diamond-shaped hierarchy: A -> B, A -> C, B -> D, C -> D."""
        onto = pyhornedowl.PyIndexedOntology()
        onto.prefix_mapping.add_default_prefix_names()
        onto.prefix_mapping.add_prefix("", "https://example.com/")
        
        onto.add_component(DeclareClass(Class(IRI.parse("https://example.com/A"))))
        onto.add_component(DeclareClass(Class(IRI.parse("https://example.com/B"))))
        onto.add_component(DeclareClass(Class(IRI.parse("https://example.com/C"))))
        onto.add_component(DeclareClass(Class(IRI.parse("https://example.com/D"))))

        # B and C are both subclasses of A
        onto.add_component(SubClassOf(
            onto.clazz("https://example.com/B"),
            onto.clazz("https://example.com/A"),
        ))
        onto.add_component(SubClassOf(
            onto.clazz("https://example.com/C"),
            onto.clazz("https://example.com/A"),
        ))
        # D is subclass of both B and C
        onto.add_component(SubClassOf(
            onto.clazz("https://example.com/D"),
            onto.clazz("https://example.com/B"),
        ))
        onto.add_component(SubClassOf(
            onto.clazz("https://example.com/D"),
            onto.clazz("https://example.com/C"),
        ))
        
        reasoner = create_structural_reasoner(onto)
        
        # Test subclasses of A
        class_a = Class(IRI.parse("https://example.com/A"))
        subclasses = reasoner.get_subclasses(class_a)
        expected_iris = {"https://example.com/B", "https://example.com/C", "https://example.com/D"}
        actual_iris = {str(c.first) for c in subclasses}
        assert actual_iris == expected_iris
        
        # Test superclasses of D
        class_d = Class(IRI.parse("https://example.com/D"))
        superclasses = reasoner.get_superclasses(class_d)
        expected_iris = {"https://example.com/A", "https://example.com/B", "https://example.com/C"}
        actual_iris = {str(c.first) for c in superclasses}
        assert actual_iris == expected_iris
