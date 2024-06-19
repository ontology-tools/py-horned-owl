Installation
============

With pip
---------
The simplest and quickest way to install Py-Horned-OWL is via pip.

.. code-block:: console

   (.venv) $ pip install py-horned-owl



From source
-----------
Clone the repository from github

.. code-block:: console

   $ git checkout https://github.com/ontology-tools/py-horned-owl.git
   $ cd py-horned-owl   



It is recommended to use a virtual environment

.. code-block:: console

    $ python3 -m virtualenv .venv
    $ source .venv/bin/activate



Install the build tool maturin and use ``maturin develop`` to build and install the project in the current virtual environment, or ``maturin build`` to create wheels. Consult the matuin documentation for further details.

.. code-block:: console

    (.venv) $ pip install maturin
    (.venv) $ maturin develop
    (.venv) $ maturin build

