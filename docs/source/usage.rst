Usage
=====


Open an existing ontology
-------------------------

To open an ontology use the :func:`~pyhornedowl.open_ontology` function. It guesses the serialization of the ontology by the file extension or tries all parsers. Alternatively, specify the serialization format explicitly with the ``serialization`` option.

.. code-block:: python
   
   import pyhornedowl
   rdf_ontology = pyhornedowl.open_ontology("path/to/ontology.owl")
   owx_ontology = pyhornedowl.open_ontology("path/to/ontology.owx")
   ofn_ontology = pyhornedowl.open_ontology("path/to/ontology", serialization='ofn')
   



Save an ontology
----------------

Use the :func:`PyIndexedOntology.save_to_file <pyhornedowl.PyIndexedOntology.save_to_file>` function to write the ontology to a file. Again, the serialization is guessed by the file extension and defaults to OWL/XML. Alternatively, specify the serialization format explicitly with the ``serialization`` option.

.. code-block:: python

   import pyhornedowl
   ontology = pyhornedowl.open_ontology("path/to/ontology.owl")

   ontology.save_to_file("path/to/ontology.owl")
   ontology.save_to_file("path/to/ontology.owx")
   ontology.save_to_file("path/to/ontology", serialization='ofn')
   

IRIs and CURIEs
--------------------------
The preferred way to create IRIs is through an ontology instance as it enables Horned-OWLs caching mechanism. Alternatively, they can be created by hand using :func:`IRI.parse <pyhornedowl.model.IRI.parse>`.

.. code-block:: python

    import pyhornedowl
    from pyhornedowl.model import IRI

    ontology = pyhornedowl.open_ontology("path/to/ontology.owl")

    i1 = ontology.iri("https://example.com/test")
    i2 = IRI.parse("https://example.com/test")

    assert i1 == i2

The :func:`PyIndexedOntology.iri <pyhornedowl.PyIndexedOntology.iri>` function guesses if you passed it an absolute IRI or a CURIE based on the existence of ``://`` in the value. This is also true for all other convenience functions accepting IRIs as an argument. You can explicitly specify if the value is an absolute IRI or a CURIE by using the optional parameter ``absolute``.

An exception to this is the the function :func:`PyIndexedOntology.curie <pyhornedowl.PyIndexedOntology.curie>` which only accepts CURIEs.

.. note::
    To create a curie the prefix must be defined.



Prefixes
--------

By default, no prefixes are defined. The standard prefixes for ``rdf``, ``rdfs``, ``xsd``, and ``owl`` can be added via the :func:`PyIndexedOntology.add_default_prefix_names <pyhornedowl.PyIndexedOntology.add_default_prefix_names>`. Other prefixes can be added using the :func:`PyIndexedOntology.add_prefix_mapping <pyhornedowl.PyIndexedOntology.add_prefix_mapping>` method. 

.. code-block:: python

    import pyhornedowl

    ontology = pyhornedowl.open_ontology("path/to/ontology.owl")

    ontology.add_default_prefix_names()
    ontology.add_prefix_mapping("ex", "https://example.com/")


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
    r = o.object_property(":r")

    assert A & B == ObjectIntersectionOf([A, B])
    assert A | B == ObjectUnionOf([A, B])
    assert ~A == ObjectComplementOf(A)
    assert ~r == InverseObjectProperty(r)
    assert r.some(A) == ObjectSomeValuesFrom(r, A)
    assert r.only(A) == ObjectAllValuesFrom(r, A)
    assert r.some(A & B | (~r).only(C)) == ObjectSomeValuesFrom(r, ObjectUnionOf([ObjectIntersectionOf([A, B]), ObjectAllValuesFrom(InverseObjectProperty(r), C)]))
