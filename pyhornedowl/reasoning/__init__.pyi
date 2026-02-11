import typing
from typing import *
from .. import PyIndexedOntology
from ..model import *

def create_reasoner(name: str, ontology: PyIndexedOntology) -> PyReasoner:
    """
    Loads a reasoner from a shared library.
    
    :param str name: name of the reasoner to load or path to the shared library to load
    """
    ...


def create_structural_reasoner(ontology: PyIndexedOntology) -> PyReasoner:
    """
    Creates a structural reasoner for the given ontology. The structural reasoner only uses the asserted named subclass and sub-property hierarchies to answer queries.
    """
    ...


class PyReasoner:
    def get_name(self) -> str:
        """
        Returns the name of the reasoner.
        """
        ...

    def get_version(self) -> str:
        """
        Returns the version of the reasoner.
        """
        ...

    def flush(self) -> None:
        """
        Flushes pending changes to the reasoner. This invalidates any cached results and updates the reasoner with the current state of the ontology.
        """
        ...

    def inferred_axioms(self) -> Set[Component]:
        """
        Returns a set of inferred axioms from the reasoner.
        """
        ...

    def is_consistent(self) -> bool:
        """
        Checks if the ontology is consistent.
        """
        ...

    def is_entailed(self, cmp: Component) -> bool:
        """
        Checks if the ontology entails the given component.
        """
        ...

    def is_satifisable(self, cmp: ClassExpression) -> bool:
        """
        Checks if the given class expression is satisfiable.
        """
        ...

    def get_unsatisfiable_classes(self) -> Set[Class]:
        """
        Returns the set of unsatisfiable classes.
        """
        ...

    def get_subclasses(self, cmp: ClassExpression) -> Set[Class]:
        """
        Returns the set of asserted and inferred subclasses for the given class expression.
        """
        ...

    def get_superclasses(self, cmp: ClassExpression) -> Set[Class]:
        """
        Returns the set of asserted and inferred superclasses for the given class expression.
        """
        ...

    def get_equivalent_classes(self, cmp: ClassExpression) -> Set[Class]:
        """
        Returns the set of classes asserted or inferred to be equivalent to given the class expression.
        """
        ...

    def get_disjoint_classes(self, cmp: ClassExpression) -> Set[Class]:
        """
        Returns the set of classes asserted or inferred to be disjoint with the given class expression.
        """
        ...


