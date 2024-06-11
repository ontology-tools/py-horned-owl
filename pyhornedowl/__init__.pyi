import typing
from typing import *

import model


class PyIndexedOntology:
    def add_default_prefix_names(self) -> None:
        """
        Adds the prefix for rdf, rdfs, xsd, and owl
        """
        ...

    def get_id_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Optional[str]:
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

    def set_label(self, iri: str, label: str, *, absolute: Optional[bool] = None) -> None:
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

    def get_subclasses(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Set[str]:
        """
        Gets all subclasses of an entity.
        """
        ...

    def get_superclasses(self, iri: str, iri_is_absolute: Optional[bool] = None) -> Set[str]:
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

    def get_annotation(self, class_iri: str, ann_iri: str, *, class_iri_is_absolute: Optional[bool] = None,
                       ann_iri_is_absolute: Optional[bool] = None) -> Optional[str]:
        """
        Gets the first annotated value for an entity and annotation property.
        
        Note: If there are multiple annotation axioms for the queried entity and annotation property,
        the order is neither necessarily the same as in the ontology neither is it stable.
        Get all annotation values with `PyIndexedOntology.get_annotations`.
        """
        ...

    def get_annotations(self, class_iri: str, ann_iri: str, *, class_iri_is_absolute: Optional[bool] = None,
                        ann_iri_is_absolute: Optional[bool] = None) -> List[str]:
        """
        Gets all annotated value for an entity and annotation property.
        
        Note: The order is neither necessarily the same as in the ontology neither is it stable.
        Get all annotation values with `PyIndexedOntology.get_annotations`.
        """
        ...

    def save_to_file(self, file_name: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> None:
        """
        Saves the ontology to disk. If no serialization is given it is guessed by the file extension.
        Defaults to OWL/XML
        """
        ...

    def get_axioms_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> List[model.AnnotatedComponent]:
        """
        Gets all axioms for an entity.
        """
        ...

    def get_components_for_iri(self, iri: str, iri_is_absolute: Optional[bool] = None) -> List[
        model.AnnotatedComponent]:
        """
        Gets all components (axiom, swrl, and meta component) for an entity.
        """
        ...

    def get_axioms(self) -> List[model.AnnotatedComponent]:
        """
        Returns all axioms of the ontology.
        """
        ...

    def get_components(self) -> List[model.AnnotatedComponent]:
        """
        Returns all axioms of the ontology.
        """
        ...

    def add_component(self, component: model.Component, annotations: Optional[List[model.Annotation]] = None) -> None:
        """
        Adds an axiom to the ontology with optional annotations.
        """
        ...

    def add_axiom(self, ax: model.Component, annotations: Optional[List[model.Annotation]]=None) -> None:
        """
        Synonym for `add_component`
        """
        ...

    def remove_component(self, component: model.Component) -> None:
        """
        Removes a component from the ontology.
        """
        ...

    def remove_axiom(self, ax: model.Component) -> None:
        """
        Synonym for `remove_component`
        """
        ...

    def iri(self, iri: str, *, absolute: Optional[bool] = True) -> model.IRI:
        """
        Creates a new IRI from string.
        
        Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
        If `absolute` is None it is guessed by the occurrence of `"://"` in the IRI whether the iri
        is absolute or not.
        """
        ...

    def curie(self, iri: str) -> model.IRI:
        """
        Creates a new IRI from CURIE string.
        
        Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
        """
        ...

    def clazz(self, iri: str, *, absolute: Optional[bool] = None) -> model.Class:
        """
        Convenience method to create a Class from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_class(self, iri: str, *, absolute: Optional[bool] = None) -> bool:
        """
        Convenience method to add a Declare(Class(iri)) axiom.
        """
        ...

    def object_property(self, iri: str, *, absolute: Optional[bool] = None) -> model.ObjectProperty:
        """
        Convenience method to create an ObjectProperty from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_object_property(self, iri: str, *, absolute: Optional[bool] = None) -> bool:
        """
        Convenience method to add a Declare(ObjectProperty(iri)) axiom.
        """
        ...

    def data_property(self, iri: str, *, absolute: Optional[bool] = None) -> model.DataProperty:
        """
        Convenience method to create a DataProperty from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_data_property(self, iri: str, *, absolute: Optional[bool] = None) -> bool:
        """
        Convenience method to add a Declare(DataProperty(iri)) axiom.
        """
        ...

    def annotation_property(self, iri: str, *, absolute: Optional[bool] = None) -> model.annotationProperty:
        """
        Convenience method to create an annotationProperty from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_annotation_property(self, iri: str, *, absolute: Optional[bool] = None) -> bool:
        """
        Convenience method to add a Declare(annotationProperty(iri)) axiom.
        """
        ...

    def named_individual(self, iri: str, *, absolute: Optional[bool] = None) -> model.NamedIndividual:
        """
        Convenience method to create a NamedIndividual from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_individual(self, iri: str, *, absolute: Optional[bool] = None) -> bool:
        """
        Convenience method to add a Declare(NamedIndividual(iri)) axiom.
        """
        ...

    def anonymous_individual(self, iri: str) -> model.AnonymousIndividual:
        """
        Convenience method to create an AnonymousIndividual from a string.
        """
        ...

    def get_descendants(self, parent: str, iri_is_absolute: Optional[bool] = None) -> Set[str]:
        """
        Gets all direct and indirect subclasses of a class.
        """
        ...

    def get_ancestors(onto: PyIndexedOntology, child: str, iri_is_absolute: Optional[bool] = None) -> Set[str]:
        """
        Gets all direct and indirect super classes of a class.
        """
        ...


def open_ontology(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> PyIndexedOntology:
    """
    Opens an ontology from a path or plain text.
    
    If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interpreted as an ontology
    in plain text.
    If no serialization is specified the serialization is guessed by the file extension or all parsers are tried
    until one succeeds.
    """
    ...


def open_ontology_from_file(path: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> PyIndexedOntology:
    """
    Opens an ontology from a file
    
    If the serialization is not specified it is guessed from the file extension. Defaults to OWL/XML.
    """
    ...


def open_ontology_from_string(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> PyIndexedOntology:
    """
    Opens an ontology from plain text.
    
    If no serialization is specified, all parsers are tried until one succeeds
    """
    ...


def get_descendants(onto: PyIndexedOntology, parent: str) -> Set[str]:
    """
    DEPRECATED: please use `PyIndexedOntology::get_descendants` instead
    Gets all direct and indirect subclasses of a class.
    """
    ...


def get_ancestors(onto: PyIndexedOntology, child: str) -> Set[str]:
    """
    DEPRECATED: please use `PyIndexedOntology::get_ancestors` instead
    Gets all direct and indirect super classes of a class.
    """
    ...
