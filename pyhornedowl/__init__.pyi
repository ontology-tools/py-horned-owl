import typing
from typing import *

import model

class PyIndexedOntology:
    def get_id_for_iri(self, iri: str) -> Optional[str]:
        """
        Gets the ID of term by it IRI.
        
        If the term does not have an ID, `None` is returned.
        """
        ...

    def get_iri_for_id(self, id: str) -> Optional[str]:
        """
        Gets the IRI of a term by its ID.
        
        If the term does not have an IRI, `None` is returned.
        """
        ...

    def add_prefix_mapping(self, iriprefix: str, mappedid: str) -> None:
        """
        Adds the prefix `iriprefix`.
        """
        ...

    def set_label(self, iri: str, label: str) -> None:
        """
        Sets the label of a term by iri.
        
        Adds an or updates the `AnnotationAssertion` axiom for `rdfs:label`.
        """
        ...

    def get_iri_for_label(self, label: str) -> Optional[str]:
        """
        Returns the IRI of a term by its label if it exists.
        
        If the term does not have a label, `None` is returned.
        """
        ...

    def get_iri(self) -> Optional[str]:
        """
        Returns the ontology iri, if it exists.
        """
        ...

    def get_version_iri(self) -> Optional[str]:
        """
        Returns the ontologys version iri, if it exists.
        """
        ...

    def get_subclasses(self, iri: str) -> Set[str]:
        """
        Gets all subclasses of an entity.
        """
        ...

    def get_superclasses(self, iri: str) -> Set[str]:
        """
        Gets all superclasses of an entity.
        """
        ...

    def get_classes(self) -> Set[str]:
        """
        Returns the IRIs of all declared classes in the ontology.
        """
        ...

    def get_object_properties(self) -> Set[str]:
        """
        Returns the IRIs of all declared object properties in the ontology.
        """
        ...

    def get_annotation(self, class_iri: str, ann_iri: str) -> Optional[str]:
        """
        Gets the first annotated value for an entity and annotation property.
        
        Note: If there are multiple annotation axioms for the queried entity and annotation property,
        the order is neither necessarily the same as in the ontology neither is it stable.
        Get all annotation values with `PyIndexedOntology.get_annotations`.
        """
        ...

    def get_annotations(self, class_iri: str, ann_iri: str) -> List[str]:
        """
        Gets all annotated value for an entity and annotation property.
        
        Note: The order is neither necessarily the same as in the ontology neither is it stable.
        Get all annotation values with `PyIndexedOntology.get_annotations`.
        """
        ...

    def save_to_file(self, file_name: str) -> None:
        """
        Saves the ontology to disk in owx format.
        """
        ...

    def get_axioms_for_iri(self, iri: str) -> List[model.AnnotatedAxiom]:
        """
        Gets all axioms for an entity.
        """
        ...

    def get_axioms(self) -> List[model.AnnotatedAxioms]:
        """
        Returns all axioms of the ontology.
        """
        ...

    def add_axiom(self, ax: model.Axiom, annotations: Optional[List[model.Annotation]]) -> None:
        """
        Adds an axiom to the ontology with optional annotations.
        """
        ...

    def remove_axiom(self, ax: model.Axiom) -> None:
        """
        Removes an axiom from the ontology.
        """
        ...

    def iri(self, iri: str) -> model.IRI:
        """
        Creates an new IRI from string.
        
        Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
        """
        ...


def open_ontology(ontology: str) -> PyIndexedOntology:
    """
    Opens an ontology from a path or plain text.
    
    If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interepreted as an ontology in either owx or owl format.
    Note: Only .owl and .owx files are currently supported.
    """
     ..


def get_descendants(onto: PyIndexedOntology, parent: str) -> Set[str]:
    """
    Gets all direct and indirect subclasses of an class.
    """
     ..


def get_ancestors(onto: PyIndexedOntology, child: str) -> Set[str]:
    """
    Gets all direct and indirect super classes of a class.
    """
     ..


