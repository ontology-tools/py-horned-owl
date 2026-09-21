Usage
=====


Open an existing ontology
-------------------------

To open an ontology use the :func:`~pyhornedowl.open_ontology` function. It guesses the serialization of the ontology by the file extension or tries all parsers. Alternatively, specify the serialization format explicitly with the ``serialization`` option. See :doc:`Serializations <serializations>` for a list of supported formats.

.. code-block:: python
   
   import pyhornedowl
   rdf_ontology = pyhornedowl.open_ontology("example.owl")
   omn_ontology = pyhornedowl.open_ontology("example.omn", serialization='omn')


Save an ontology
----------------

Use the :func:`PyIndexedOntology.save_to_file <pyhornedowl.PyIndexedOntology.save_to_file>` function to write the ontology to a file. Again, the serialization is guessed by the file extension and defaults to OWL/XML. Alternatively, specify the serialization format explicitly with the ``serialization`` option. See :doc:`Serializations <serializations>` for a list of supported formats.

.. code-block:: python

   import pyhornedowl
   ontology = pyhornedowl.open_ontology("example.owl")

   ontology.save_to_file("example.owl")
   ontology.save_to_file("example.ofn", serialization='ofn')

Serializations
--------------
The following serializations are supported:

- ``rdf`` (or ``owl``) for RDF/XML
- ``owx`` for OWL/XML
- ``ofn`` for OWL Functional Syntax
- ``omn`` for OWL 2 Manchester Syntax
- ``obo`` for OBO flat files
- Other RDF serializations recognised by oxrdfio, such as Turtle and N-Triples, are accepted by their extension. See `the supported oxrdfio formats <https://docs.rs/oxrdfio/0.2.6/oxrdfio/enum.RdfFormat.html>`__ for details.

   

IRIs and CURIEs
---------------
The preferred way to create IRIs is through an ontology instance as it enables Horned-OWLs caching mechanism. Alternatively, they can be created by hand using :func:`IRI.parse <pyhornedowl.model.IRI.parse>`.

.. code-block:: python

    import pyhornedowl
    from pyhornedowl.model import IRI

    ontology = pyhornedowl.open_ontology("example.owl")

    i1 = ontology.iri("https://example.com/test")
    i2 = IRI.parse("https://example.com/test")

    assert i1 != i2

The :func:`PyIndexedOntology.iri <pyhornedowl.PyIndexedOntology.iri>` function guesses if you passed it an absolute IRI or a CURIE based on the existence of ``://`` in the value. This is also true for all other convenience functions accepting IRIs as an argument. You can explicitly specify if the value is an absolute IRI or a CURIE by using the optional parameter ``absolute``.

An exception to this is the the function :func:`PyIndexedOntology.curie <pyhornedowl.PyIndexedOntology.curie>` which only accepts CURIEs.

.. note::
    To create a curie the prefix must be defined.



Prefixes
--------

By default, no prefixes are defined. The standard prefixes for ``rdf``, ``rdfs``, ``xsd``, and ``owl`` can be added via the :func:`PrefixMapping.add_default_prefix_names <pyhornedowl.PrefixMapping.add_default_prefix_names>`. Other prefixes can be added using the :func:`PrefixMapping.add_prefix <pyhornedowl.PrefixMapping.add_prefix>` method. 

.. code-block:: python

    import pyhornedowl

    ontology = pyhornedowl.open_ontology("example.owl")

    ontology.prefix_mapping.add_default_prefix_names()
    ontology.prefix_mapping.add_prefix("ex", "https://example.com/")


Create entities
---------------
Classes, Individuals, Data- and Objectproperties can be created using convenience methods on an ontology.

.. code-block:: python

    import pyhornedowl
    o = pyhornedowl.open_ontology("example.owl")
    o.add_prefix_mapping("", "https://example.com/")

    c = o.class_(":A")
    op = o.object_property(":op")
    dp = o.data_property(":dp")
    ap = o.annotation_property(":ap")
    i = o.named_individual(":I")
    n = o.anonymous_individual(":n")


Pattern match
---------------
All (``pyhornedowl.model``) classes can be pattern matched using the ``match`` statement.

.. code-block:: python

    import pyhornedowl
    from pyhornedowl.model import *

    o = pyhornedowl.open_ontology("example.owl")

    for component in o.get_components():
        match component:
            case AnnotatedComponent(OntologyID(id, version), _):
                print(f"Ontology ID: {id}, Version: {version}")
            case AnnotatedComponent(SubClassOf(Class(iri1),Class(iri2)), _):
                print(f"{iri1} is a subclass of {iri2}")
            case AnnotatedComponent(SubClassOf(Class(iri1), ObjectUnionOf([Class(iri2), Class(iri3)])), _):
                print(f"{iri1} is a subclass of {iri2} or {iri3}")
            case _:
                pass


Write class expressions
-----------------------

Instead of writing class expressions as nested constructor calls, some expressions can be expressed using operators.

.. code-block:: python

    import pyhornedowl
    from pyhornedowl.model import *

    o = pyhornedowl.PyIndexedOntology()
    o.add_prefix_mapping("", "https://example.com/")

    A = o.class_(":A")
    B = o.class_(":B")
    C = o.class_(":C")
    r = o.object_property(":r")

    assert A & B == ObjectIntersectionOf([A, B])
    assert A | B == ObjectUnionOf([A, B])
    assert ~A == ObjectComplementOf(A)
    assert ~r == InverseObjectProperty(r)
    assert r.some(A) == ObjectSomeValuesFrom(r, A)
    assert r.only(A) == ObjectAllValuesFrom(r, A)
    assert r.some(A & B | (~r).only(C)) == ObjectSomeValuesFrom(r, ObjectUnionOf([ObjectIntersectionOf([A, B]), ObjectAllValuesFrom(InverseObjectProperty(r), C)]))


Render a single axiom
---------------------

``serialize`` renders a single axiom, component, or class expression as a string. Every model class has it. It is the per-element counterpart to ``save_to_string``, and for Manchester it is the only option: ``save_to_string("omn")`` groups axioms into entity frames, which cannot be sliced back into individual axioms.

.. code-block:: python

    import pyhornedowl

    ontology = pyhornedowl.open_ontology("example.owl")

    for axiom in ontology.get_axioms():
        print(axiom.serialize())               # functional syntax (default)
        print(axiom.serialize("omn"))          # Manchester

Accepted values are ``ofn`` (the default) and ``omn``: the serializations for which horned-owl provides a per-element writer. Its OWL/XML, RDF and OBO writers operate on whole ontologies only.

Serializing a single construct to ``obo`` is not possible as OBO is stanza-oriented: A component does not render to a string of its own but to a clause line under some *other* entity's ``[Term]`` stanza, and which stanza that is depends on ``oboInOwl:id`` annotations gathered from the whole ontology. There is nothing for a per-element writer to return.

Pass a :class:`~pyhornedowl.PrefixMapping` to abbreviate IRIs:

.. code-block:: python

    print(axiom.serialize("omn", ontology.prefix_mapping))

Note, that ``ofn`` renders an :class:`~pyhornedowl.model.AnnotatedComponent` including its axiom annotations, while ``omn`` renders only the component.

Currently, the following classes do not support Manchester syntax rendering on their own:
- :class:`~pyhornedowl.model.Annotation`
- :class:`~pyhornedowl.model.AnnotationProperty`
- :class:`~pyhornedowl.model.FacetRestriction`
- :class:`~pyhornedowl.model.Facet`

However, they are rendered if they occur as part of a larger construct (e.g. a class expression).
