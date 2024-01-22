Usage
=====

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

   # Construct an axiom
   from pyhornedowl.model import *
   axiom = SubClassOf(
    Class(IRI.parse(':Child')),
    ObjectSomeValuesFrom(
        ObjectProperty(IRI.parse(':has_parent')),
        Class(IRI.parse(':Human'))
    )
   )

   # Add the axiom
   ontology.add_axiom(axiom)