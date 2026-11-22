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

   from pyhornedowl import Ontology
   from pyhornedowl.model import SubClassOf

   ontology = Ontology()  # Or open_ontology("example.owl").
   ontology.add_prefix_mapping("ex", "https://example.org/")
   child, human = ontology.class_("ex:Child"), ontology.class_("ex:Human")
   parent = ontology.object_property("ex:has_parent")
   ontology.declare(child)
   ontology.add_axiom(SubClassOf(child, parent.some(human)))
   ontology.set_label(child, "Child")

   assert ontology.signature().classes() == {child, human}
   for axiom in ontology.query_components(SubClassOf, including=child):
       print(axiom.serialize("omn", ontology.prefix_mapping))

   ontology.save_to_file("output.ofn")
