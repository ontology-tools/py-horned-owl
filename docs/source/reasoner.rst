Reasoner
==========

Py-Horned-OWL provides an interface for reasoners that are compatible with the Horned OWL library.

Using a reasoner
----------------
A compatible reasoner should provide a python package that exports a `create_reasoner` function. This functions takes an ontology as an argument and returns a :class:`~pyhornedowl.PyReasoner` instance which can be used to perform reasoning tasks. The ontology instance is linked to the reasoner, which means that any changes to the ontology will be reflected in the reasoner. But, a manual call to :func:`PyReasoner.flush <pyhornedowl.PyReasoner.flush>` is required to update the reasoner with any changes made to the ontology. Here is an example with the EL Reasoner `whelk-rs <https://github.com/INCATools/whelk-rs>`__ via `PyWhelk <https://github.com/ontology-tools/py-whelk/>`__.

.. code-block:: console

    (.venv) $ pip install pyhornedowl pywhelk


.. code-block:: python

    import pyhornedowl
    import pywhelk

    # Open an ontology
    o = pyhornedowl.open_ontology("<path/to/ontology>")

    # Create a reasoner instance
    reasoner = pywhelk.create_reasoner(o)

    # Perform reasoning tasks
    print(f"Ontology is consistent: {reasoner.is_consistent()}")

    # Modify the ontology and make it inconsistent
    from pyhornedowl.model import SubClassOf
    o.add_axiom(SubClassOf(o.clazz('owl:Thing'), o.clazz('owl:Nothing')))

    # The ontology is now inconsistent, but the reasoner is not yet aware of it
    print(f"Ontology is consistent: {reasoner.is_consistent()}")
    # Flush the reasoner to update it
    reasoner.flush()
    # Now the reasoner is aware of the inconsistency
    print(f"Ontology is consistent: {reasoner.is_consistent()}")



Developing a reasoner
---------------------

To make a reasoner written in Rust and with Horned-OWL compatible with Py-Horned-OWL, the library must export a type which implements the `Reasoner` trait and a `create_reasoner` function with the following signature:

.. code-block:: rust

    #[unsafe(no_mangle)]
    pub fn create_reasoner(
        ontology: SetOntology<ArcStr>,
    ) -> Box<dyn Reasoner<ArcStr, ArcAnnotatedComponent>>

The library must be compiled as a shared C library (crate type "cdylib"). The reasoner can then be used in Python using :func:`~pyhornedowl.create_reasoner`, which takes the path to the shared library as an argument and returns a :func:`~pyhornedowl.PyReasoner` instance. There is the helper trait `PyReasoner` and macro `export_py_reasoner!` to get type checking for the `create_reasoner` function. A typical reasoner implementation would look like this:

.. code-block:: rust

    pub struct MyReasoner {
        pending_insert: Vec<ArcAnnotatedComponent>,
        pending_remove: Vec<ArcAnnotatedComponent>,
        // ... any other fields needed for storing the internal state of the reasoner
    }

    export_py_reasoner!(MyReasoner);

    impl PyReasoner for MyReasoner {
        fn create_reasoner(ontology: SetOntology<ArcStr>) -> Self {
            todo!()
        }
    }

    impl OntologyIndex<ArcStr, ArcAnnotatedComponent> for MyReasoner {
        fn index_insert(&mut self, cmp: ArcAnnotatedComponent) -> bool {
            self.pending_insert.push(cmp);
            true
        }

        fn index_remove(&mut self, _cmp: &AnnotatedComponent<ArcStr>) -> bool {
            self.pending_remove.push(cmp);
            true
        }
    }

    impl Reasoner<ArcStr, ArcAnnotatedComponent> for MyReasoner {
        fn get_name(&self) -> String {
            todo!()
        }

        fn flush(&mut self) -> Result<(), ReasonerError> {
            // Handle the pending inserts and removes and update the internal state of the reasoner
            todo!()
        }

        fn inferred_axioms(&self) -> Box<dyn Iterator<Item = Component<ArcStr>>> {
            // Return an iterator over the inferred axioms based on the internal state of the reasoner
            todo!()
        }

        // implement any other methods from the `Reasoner` trait supported by your reasoner. By default, all methods are implemented to return `Err(ReasonerError::NotImplemented)`.
    }



It might be useful to package the reasoner as a Python package that exports the `create_reasoner` function, so it can be installed via pip and used directly in Python. The `py-whelk project <https://github.com/ontology-tools/py-whelk/>`__ is an example that can be used as a reference.

How it works
------------
A specific reasoner can be developed using Horned-OWL without targeting Py-Horned-OWL directly. To add support for a specific reasoner anybody can create a Rust library that implements the `Reasoner` trait and exports a `create_reasoner` function without dealing with the specifics of PyO3 or Py-Horned-OWLs inner workings. Py-Horned-OWL provides the necessary wrappers and facades to make the Rust functions available in Python through the :class:`~pyhornedowl.PyReasoner` class.  To create a reasoner it will load the shared library dynamically, look for the `create_reasoner` function and call it.

This seperation allows for reasoners to be developed and installed independently of Py-Horned-OWL and without the need to understand the Python bindings or the PyO3 library. 