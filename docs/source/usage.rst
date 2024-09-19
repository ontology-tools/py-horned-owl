Usage
=====


Open an existing ontology
-------------------------

To open an ontology use the :func:`~pyhornedowl.open_ontology` function. It guesses the serialization of the ontology by the file extension or tries all parsers. Alternatively, specify the serialization format explicitely with the ``serialization`` option.

.. code-block:: python
   
   import pyhornedowl
   rdf_ontology = pyhornedowl.open_ontology("path/to/ontology.owl")
   owx_ontology = pyhornedowl.open_ontology("path/to/ontology.owx")
   ofn_ontology = pyhornedowl.open_ontology("path/to/ontology", serialization='ofn')
   



Save an ontology
----------------

Use the :func:`PyIndexedOntology.save_to_file <pyhornedowl.PyIndexedOntology.save_to_file>` function to write the ontology to a file. Again, the serialization is guessed by the file extension and defaults to OWL/XML. Alternatively, specify the serialization format explicitely with the ``serialization`` option.

.. code-block:: python

   import pyhornedowl
   ontology = pyhornedowl.open_ontology("path/to/ontology.owl")

   ontology.save_to_file("path/to/ontology.owl")
   ontology.save_to_file("path/to/ontology.owx")
   ontology.save_to_file("path/to/ontology", serialization='ofn')
   

IRIs, CURIEs, and OBO IDs
--------------------------
The preferred way to create IRIs is through an ontology instance as it enables Horned-OWLs caching mechanism. Alternatively, they can be created by hand using :func:`IRI.parse <pyhornedowl.model.IRI.parse>`.

.. code-block:: python

    import pyhornedowl
    from pyhornedowl.model import IRI

    ontology = pyhornedowl.open_ontology("path/to/ontology.owl")

    i1 = ontology.iri("https://example.com/test")
    i2 = IRI.parse("https://example.com/test")

    assert i1 == i2

The :func:`PyIndexedOntology.iri <pyhornedowl.PyIndexedOntology.iri>` function guesses if you passed it an absolute IRI or a CURIE based on the existence of ``://`` in the value. This is also true for all other convenience functions accepting IRIs as an argument. You can explicitely specify if the value is an absolute IRI or a CURIE by using the optional parameter ``absolute``.

An exception to this is the the function :func:`PyIndexedOntology.curie <pyhornedowl.PyIndexedOntology.curie>` which only accepts CURIEs.

.. note::
    To work with CURIEs their prefix must be defined.

Expanding and shrinking IRIs and CURIEs
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
The methods :func:`PrefixMapping.expand_curie <pyhornedowl.PrefixMapping.expand_curie>` and :func:`PrefixMapping.shrink_iri <pyhornedowl.PrefixMapping.shrink_iri>` can be used to expand CURIEs or shrink IRIs.

OBO IDs
^^^^^^^
OBO IDs are usually just CURIEs. When the prefix is defined, an OBO ID can be used anywhere where a CURIE can be supplied. Similarly, an OBO ID can be expanded to their full IRI using :func:`PrefixMapping.expand_curie <pyhornedowl.PrefixMapping.expand_curie>` and :func:`PrefixMapping.shrink_iri <pyhornedowl.PrefixMapping.shrink_iri>`.

Prefixes
--------

The registered prefixes are accessible via :func:`PyIndexedOntology.prefix_mapping <pyhornedowl.PyIndexedOntology.prefix_mapping>`. By default, no prefixes are defined. The standard prefixes for ``rdf``, ``rdfs``, ``xsd``, and ``owl`` can be added via the :func:`PrefixMapping.add_default_prefix_names <pyhornedowl.PrefixMapping.add_default_prefix_names>`. Other prefixes can be added using the :func:`PrefixMapping.add_prefix <pyhornedowl.PrefixMapping.add_prefix>` method. Alternatively, prefixes can be added, changed, or deleted using

.. code-block:: python

    import pyhornedowl

    ontology = pyhornedowl.open_ontology("path/to/ontology.owl")

    ontology.prefix_mapping.add_default_prefix_names()
    ontology.prefix_mapping.add_prefix("ex", "https://example.com/")
    ontology.prefix_mapping['foo'] = "https://example.com/foo#"
    print(ontology.prefix_mapping['foo']) # "https://example.com/foo#"
    print('foo' in ontology.prefix_mapping) # True
    del ontology.prefix_mapping['foo']
    print('foo' in ontology.prefix_mapping) # False

    for prefix, iri in ontology.prefix_mapping:
        print(f"{prefix}: <{iri}>")


Querying the ontology
---------------------

Get axioms
^^^^^^^^^^
Use :func:`PyIndexedOntology.get_axioms <pyhornedowl.PyIndexedOntology.get_axioms>` to get all axioms of an ontology.

Use :func:`PyIndexedOntology.get_components <pyhornedowl.PyIndexedOntology.get_components>` to get all components of an ontology. Components include additional constructs like rules.

Use :func:`PyIndexedOntology.get_axioms_for_iri <pyhornedowl.PyIndexedOntology.get_axioms_for_iri>` to get all axioms that a occurs in. This includes axioms where the IRI occurs, for example, in nested class expressions.

If you want to query axioms for an entity by their OBO ID, ensure, that a prefix is added. Then, you can use :func:`PyIndexedOntology.get_axioms_for_iri <pyhornedowl.PyIndexedOntology.get_axioms_for_iri>` as IDs are just CURIEs.

The following example loaded an ontology ``ontology`` with the following content:

.. code-block::

    Prefix: : <https://example.com/ontology#>
    Prefix: EX: <https://example.com/ontology#>
    Prefix: BFO: <http://purl.obolibrary.org/obo/BFO_>
    Ontology:
        Class: B
            Annotation: rdfs:label "The class of all B"
            Annotation: rdfs:label "The class of all B"@en
            Annotation: rdfs:label "Die Klasse aller B"@de
        Class: C
            SubClassOf: r some B

        ObjectProperty: r



.. code-block:: python

    axioms = ontology.get_axioms()

    # Get all subclasses of a class by their IRI/ID/CURIE
    subclasses = ontology.get_subclasses("BFO:0000001")

    # Get all superclasses of a class by their IRI/ID/CURIE
    superclasses = ontology.get_superclasses("EX:B")

    # Get all annotations of an annotation property for an entity by their IRI/ID/CURIE
    labels = ontology.get_annotations("ex:B", "rdfs:label")


Create entities
---------------
Classes, Individuals, Data- and Objectproperties can be created using convenience methods on an ontology.

.. code-block:: python

    import pyhornedowl
    o = pyhornedowl.open_ontology("path/to/ontology.owl")
    o.add_prefix_mapping("", "https://example.com/")

    c = o.clazz(":A")
    op = o.object_property(":op")
    dp = o.data_property(":dp")
    ap = o.annotation_property(":ap")
    i = o.named_individual(":I")
    n = o.anonymous_individual(":n")


Write class expressions
-----------------------

Instead of writing class expressions as nested constructor calls, some expressions can be expressed using operators.

.. code-block:: python

    import pyhornedowl
    from pyhornedowl.model import *

    o = pyhornedowl.PyIndexedOntology()
    o.add_prefix_mapping("", "https://example.com/")

    A = o.clazz(":A")
    B = o.clazz(":B")
    C = o.clazz(":C")
    r = o.object_property("r")

    assert A & B == ObjectIntersectionOf([A, B])
    assert A | B == ObjectUnionOf([A, B])
    assert ~A == ObjectComplementOf(A)
    assert ~r == InverseObjectProperty(r)
    assert r.some(A) == ObjectSomeValuesFrom(r, A)
    assert r.only(A) == ObjectAllValuesFrom(r, A)
    assert r.some((A & B) | (~r).only(C)) == ObjectSomeValuesFrom(r, ObjectUnionOf([ObjectIntersectionOf([A, B]), ObjectAllValuesFrom(InverseObjectProperty(r), C)]))
