import typing
from typing import *
import pyhornedowl

class Whelk:
    def __new__(self, onto: pyhornedowl.PyIndexedOntology):
        """
        Creates a reasoner for a loaded ontology
        """
        ...

    def classify(self) -> None:
        """
        Calculates the class and role hierarchy. This method is expected to be called before calling
        any other method.
        """
        ...

    def get_subclasses(self, iri: str) -> List[str]:
        """
        Return all asserted and inferred subclasses (direct and indirect) of a class.
        """
        ...

    def get_superclasses(self, iri: str) -> List[str]:
        """
        Return all asserted and inferred superclasses (direct and indirect) of a class.
        """
        ...

    def entails(self, axiom: pyhornedowl.model.Component) -> bool:
        """
        Check if the ontology entails a given axiom.
        """
        ...

    def consistent(self) -> bool:
        """
        Check whether the ontology is consistent
        """
        ...

    def inferred_axioms(self) -> List[pyhornedowl.model.Component]:
        """
        Get all axioms the reasoner could infer from the given ontology
        """
        ...

    ...


