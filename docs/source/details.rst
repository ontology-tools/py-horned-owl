Details
=======

Py-Horned-OWL provides Python bindings via PyO3. This bridge presents some challenges.

Mapping Rust ADT enums to Python classes
----------------------------------------
While Horned-OWL C like structs can be directly translated to Python classes bridging Rust ADTs to Python is not fully supported by PyO3 yet (`https://github.com/PyO3/pyo3/issues/417 <https://github.com/PyO3/pyo3/issues/417>`_). Hence Py-Horned-OWL uses a custom design.


For each tuple structs, a C like struct with fields ``first`` and ``second`` (depending on their airity) is generated. They are then directly translated to Python classes.


For each enum type ``enum E ( V1(...), V2{...}, ... )`` struct types ``E(E_Inner), V1(...), V2{...}, ...`` and an internal enum type ``enum E_Inner(V1(V1), V2(V2), ...)`` are generated. The structs ``V1, V2, ...`` are translated to Python classes.
The struct ``E`` manually implements the ``FromPyObject`` and ``IntoPy`` traits to hide the inner enum. Most notably, ``E`` is **not** exposed to Python. Instead, Py-Horned-OWL exposes ``E`` a ``typing.Union`` consisting of all variants ``E = typing.Union[V1, V2, ...]``. Ideally ``E`` would be a Python class as well and ``V1, V2, ...`` would be subclasses of ``E``. Unfortunately, class hierarchies are not supported by PyO3 to a level where this would be easily possible. The current approach, however, still allows for type hints and even runtime instance checking (as ``isinstance(V1(...), E) == True``).


Wrapper types
-------------

Exposing Rust datatypes to Python via PyO3 requires implementing certrain traits ``PyClass`` or ``FromPyObjectBound`` (or to use their macros like ``#[pyclass]``). Due to Rusts `orphan rules <https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules>`_ the traits cannot be directly implemented for the datatypes in Horned-OWL. Therefore, each Horned-OWL type is wrapped (new type idiom). For each type ``T`` the procedure would be the same:

#. Define the wrapper type ``T_W`` depending on the Rust data type
#. Conversion from ``hornedowl::model::T`` to ``T_W`` and vice versa
#. Add python methods e.g. for creating, string conversion, equality, and hash.

As the tasks are very repetitive, macros are defined. The main macro is ``wrapped`` which takes Rust struct and enum definitions as defined in Horned-OWL and produces the wrapper types and implementations. It accepts custom arguments to control the wrapped datatypes:

``transparent pub enum ...``
    Only valid on enums of the form ``pub enum E{ V1(V1), V2(V2), ... }``. It prevents the creation of intermediate types for ``V1, V2, ...``.

``#[transparent] V``
    Only valid on variants of the form ``V1(V1)``. It also prevents the creation of an intermediate type.

``#[suffixed] pub enum ...``
    For an enum ``pub enum E{ V1(...), V2{...}}`` the ``suffixed`` argument creates structs (and python classes) by concatenating enum and variant name (e.g. ``SimpleLiteral``). For some datatypes this makes their intention clearer and avoids name conflicts (e.g. ``Language`` vs. ``LanguageLiteral``).


To ensure the same interface in the macros, the ``FromCompatible`` trait is introduced as a wrapper around the ``From`` trait. This allows to redefine the conversion from data types from the Rust standard library e.g. Box or Vec for the wrapper types in Py-Horned-OWL.

Similarly, newtypes are defined for ``String``, ``Vec``, ``Box``, and ``BTreeSet``.


Python documentation
--------------------
Unfortunately, the documentation of rust datatypes vanishes at compile time. Therefore, we cannot simply copy the documentation of Horned-OWL data types to the wrapped data types. But a helper script extracts the doc strings from Horned-OWL and provides them in a form of a macro. Additionally, Py-Horned-OWL datatypes and functions follow the convention to include their signature as the first line of their documentation.

Python stubs / type hints
-------------------------
Currently, PyO3 does not output python type hints. So, all of the asserted type information in Rust is lost during the bridging to Python. To counter it, Py-Horned-OWL datatypes implement the trait ``ToPyi`` which contains a ``pyi(module: Option<String>) -> String`` function which returns a python stub. A helper script ``gen_pyi.py`` then iterates over all members and generates python stub files. If no such method is defined, the script searches the first line of the ``__doc__`` field for a signature and uses it instead.