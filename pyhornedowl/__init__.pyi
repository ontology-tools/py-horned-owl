import typing
from typing import *
from typing_extensions import deprecated

import model
import reasoning

class PyIndexedOntology:
    """
    Represents a loaded ontology.
    """
    def get_id_for_iri(self, iri: model.IRIParam) -> Optional[str]:
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

    def set_label(self, iri: model.IRIParam, label: str) -> None:
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

    def get_subclasses(self, iri: model.IRIParam) -> Set[str]:
        """
        Gets all direct subclasses of an entity.
        """
        ...

    def get_superclasses(self, iri: model.IRIParam) -> Set[str]:
        """
        Gets all direct superclasses of an entity.
        """
        ...

    def get_classes(self) -> Set[str]:
        """
        Returns the IRIs of all declared classes in the ontology.
        """
        ...

    def get_datatypes(self) -> Set[str]:
        """
        Returns the IRIs of all declared datatypes in the ontology.
        """
        ...

    def get_object_properties(self) -> Set[str]:
        """
        Returns the IRIs of all declared object properties in the ontology.
        """
        ...

    def get_annotation_properties(self) -> Set[str]:
        """
        Returns the IRIs of all declared annotation properties in the ontology.
        """
        ...

    def get_data_properties(self) -> Set[str]:
        """
        Returns the IRIs of all declared data properties in the ontology.
        """
        ...

    def get_named_individuals(self) -> Set[str]:
        """
        Returns the IRIs of all declared named individuals in the ontology.
        """
        ...

    def get_annotation(self, class_iri: model.IRIParam, ann_iri: model.IRIParam) -> Optional[str]:
        """
        Gets the first annotated value for an entity and annotation property.
        
        Note: If there are multiple annotation axioms for the queried entity and annotation property,
        the order is neither necessarily the same as in the ontology neither is it stable.
        Get all annotation values with `PyIndexedOntology.get_annotations`.
        """
        ...

    def get_annotations(self, class_iri: model.IRIParam, ann_iri: model.IRIParam) -> List[str]:
        """
        Gets all annotated value for an entity and annotation property.
        
        Note: The order is neither necessarily the same as in the ontology neither is it stable.
        Get all annotation values with `PyIndexedOntology.get_annotations`.
        """
        ...

    def save_to_string(self, serialization: typing.Literal['owl', 'rdf','ofn', 'owx']) -> str:
        """
        Saves the ontology to a UTF8 string.
        """
        ...

    def save_to_file(self, file_name: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None) -> None:
        """
        Saves the ontology to disk. If no serialization is given it is guessed by the file extension.
        Defaults to OWL/XML
        """
        ...

    def get_axioms_for_iri(self, iri: model.IRIParam) -> List[model.AnnotatedComponent]:
        """
        Gets all axioms for an entity.
        """
        ...

    def get_components_for_iri(self, iri: model.IRIParam) -> List[model.AnnotatedComponent]:
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

    def add_component(self, component: model.Component, annotations: Optional[List[model.Annotation]|Set[model.Annotation]]=None) -> None:
        """
        Adds an axiom to the ontology with optional annotations.
        """
        ...

    def add_axiom(self, ax: model.Component, annotations: Optional[List[model.Annotation]|Set[model.Annotation]]=None) -> None:
        """
        Synonym for `add_component`
        """
        ...

    def remove_component(self, component: model.Component) -> bool:
        """
        Removes a component from the ontology.
        """
        ...

    def remove_axiom(self, ax: model.Component) ->  bool:
        """
        Synonym for `remove_component`
        """
        ...

    def iri(self, iri: model.IRIParam) -> model.IRI:
        """
        Creates a new IRI from string.
        
        Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
        """
        ...

    def curie(self, iri: model.IRIParam) -> model.IRI:
        """
        Creates a new IRI from CURIE string.
        
        Use this method instead of  `model.IRI.parse` if possible as it is more optimized using caches.
        """
        ...

    def clazz(self, iri: model.IRIParam) -> model.Class:
        """
        Convenience method to create a Class from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        
        .. deprecated::
            Use `PyIndexedOntology.class_` instead
        """
        ...

    def class_(self, iri: model.IRIParam) -> model.Class:
        """
        Convenience method to create a Class from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_class(self, iri: model.IRIParam) -> bool:
        """
        Convenience method to add a Declare(Class(iri)) axiom.
        """
        ...

    def object_property(self, iri: model.IRIParam) -> model.ObjectProperty:
        """
        Convenience method to create an ObjectProperty from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_object_property(self, iri: model.IRIParam) -> bool:
        """
        Convenience method to add a Declare(ObjectProperty(iri)) axiom.
        """
        ...

    def data_property(self, iri: model.IRIParam) -> model.DataProperty:
        """
        Convenience method to create a DataProperty from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_data_property(self, iri: model.IRIParam) -> bool:
        """
        Convenience method to add a Declare(DataProperty(iri)) axiom.
        """
        ...

    def annotation_property(self, iri: model.IRIParam) -> model.AnnotationProperty:
        """
        Convenience method to create an annotationProperty from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_annotation_property(self, iri: model.IRIParam) -> bool:
        """
        Convenience method to add a Declare(annotationProperty(iri)) axiom.
        """
        ...

    def named_individual(self, iri: model.IRIParam) -> model.NamedIndividual:
        """
        Convenience method to create a NamedIndividual from an IRI.
        
        Uses the `iri` method to cache native IRI instances.
        """
        ...

    def declare_individual(self, iri: model.IRIParam) -> bool:
        """
        Convenience method to add a Declare(NamedIndividual(iri)) axiom.
        """
        ...

    def anonymous_individual(self, iri: model.IRIParam) -> model.AnonymousIndividual:
        """
        Convenience method to create an AnonymousIndividual from a string.
        """
        ...

    def get_descendants(self, parent: str) -> Set[str]:
        """
        Gets all direct and indirect subclasses of a class.
        
        .. deprecated::
            Use a reasoner instead. See :doc:`reasoner` for details.
        """
        ...

    def get_ancestors(self, child: str) -> Set[str]:
        """
        Gets all direct and indirect super classes of a class.
        
        .. deprecated::
            Use a reasoner instead. See :doc:`reasoner` for details.
        """
        ...

    def build_iri_index(self) -> None:
        """
        Builds an index by iri (IRIMappedIndex).
        """
        ...

    def component_index(self) -> None:
        """
        Builds an index by component kind (ComponentMappedIndex).
        """
        ...

    def build_indexes(self) -> None:
        """
        Builds indexes to allow (a quicker) access to axioms and entities.
        """
        ...

    prefix_mapping: PrefixMapping
    """
    The prefix mapping
    """


class IndexCreationStrategy:
    """
    Values to indicate when to build the additional indexes.
    
    """
    OnLoad: typing.Self
    """
    Create the additional indexes when the ontology is loaded
    """
    OnQuery: typing.Self
    """
    Create the additional indexes only when they are needed
    """
    Explicit: typing.Self
    """
    Only create the additional indexes when explicity requested
    """

class PrefixMapping:
    def __iter__(self, /):
        ...

    def __len__(self, /):
        ...

    def __getitem__(self, key, /):
        ...

    def __setitem__(self, key, value, /):
        ...

    def __delitem__(self, key, /):
        ...

    def __contains__(self, key, /):
        ...

    def add_default_prefix_names(self) -> None:
        """
        Adds the prefix for rdf, rdfs, xsd, and owl
        """
        ...

    def add_prefix(self, iriprefix: str, mappedid: str) -> None:
        """
        Adds the prefix `iriprefix`.
        """
        ...

    def remove_prefix(self, iriprefix: str) -> None:
        """
        Remove a prefix from the mapping.
        """
        ...

    def expand_curie(self, curie: str) -> str:
        """
        Expands a curie. Throws a ValueError if the prefix is invalid or unknown
        """
        ...

    def shrink_iri(self, iri: str) -> str:
        """
        Shrinks an absolute IRI to a CURIE. Throws a ValueError on failure
        """
        ...


def open_ontology(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology:
    """
    Opens an ontology from a path or plain text.
    
    If `ontology` is a path, the file is loaded. Otherwise, `ontology` is interpreted as an ontology
    in plain text.
    If no serialization is specified the serialization is guessed by the file extension or all parsers are tried
    until one succeeds.
    """
    ...


def open_ontology_from_file(path: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology:
    """
    Opens an ontology from a file
    
    If the serialization is not specified it is guessed from the file extension. Defaults to OWL/XML.
    """
    ...


def open_ontology_from_string(ontology: str, serialization: Optional[typing.Literal['owl', 'rdf','ofn', 'owx']]=None, index_strategy = IndexCreationStrategy.OnQuery) -> PyIndexedOntology:
    """
    Opens an ontology from plain text.
    
    If no serialization is specified, all parsers are tried until one succeeds
    """
    ...


