Quickstart
==========

Installation
------------

To use py-horned-owl, first install it using pip:

.. code-block:: console

   (.venv) $ pip install py-horned-owl


Work with ontologies
--------------------

.. code-block:: python
   
   import pyhornedowl
   ontology = pyhornedowl.open_ontology("<path/to/ontology>")

   # Get all axioms
   axioms = ontology.get_axioms()

   # Add a prefix
   ontology.add_prefix_mapping(":", "https://example.com/test#")

   # Construct an axiom
   from pyhornedowl.model import *
   axiom = SubClassOf(
    o.clazz(':Child'),
    ObjectSomeValuesFrom(
        o.object_property(':has_parent'),
        o.clazz(':Human')
    )
   )

   # Add the axiom
   ontology.add_axiom(axiom)

   # Save the ontology
   o.save_to_file("output.owx")