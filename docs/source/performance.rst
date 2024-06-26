Performance
===========

The underlying Rust library Horned-OWL offers an `index system <https://docs.rs/horned-owl/latest/horned_owl/ontology/index.html>`_ to allow fast access to different kinds of parts of an ontology. 

By default, Py-Horned-OWL loads an ontology in a simple hash set (using a `set index <https://docs.rs/horned-owl/latest/horned_owl/ontology/set/index.html>`_). While this allows for quick loading and iterating of all components in an ontology, queries like "give me all classes" or "give me all axioms for this IRI" are slower or even not supported at all.

Especially queries by IRI are not supported without having an `iri mapped index <https://docs.rs/horned-owl/latest/horned_owl/ontology/iri_mapped/index.html>`_. By default, it is implicitly created when a function requiring it is called. 

The `component mapped index <https://docs.rs/horned-owl/latest/horned_owl/ontology/component_mapped/index.html>`_ improves the performance of entity or specific axioms lookups. For example, the functions :func:`~pyhornedowl.PyIndexedOntology.get_classes`, :func:`~pyhornedowl.PyIndexedOntology.get_object_properties`, etc., :func:`~pyhornedowl.PyIndexedOntology.get_iri`, or :func:`~pyhornedowl.PyIndexedOntology.get_iri_for_label` benefit from a component index.

The indexes can be build manually using the :func:`~pyhornedowl.PyIndexedOntology.build_iri_index`, :func:`~pyhornedowl.PyIndexedOntology.build_component_index`, or to build both indexes together :func:`~pyhornedowl.PyIndexedOntology.build_indexes` methods.

You change the behaviour for index creating using the `index_creation_strategy` parameters in the :func:`~pyhornedowl.open_ontology` functions or the constructor of :func:`~pyhornedowl.PyIndexedOntology`.

