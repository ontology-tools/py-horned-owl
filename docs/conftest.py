import os
import shutil
import tempfile

from sybil import Sybil
from sybil.parsers.rest import DocTestParser, PythonCodeBlockParser

import pyhornedowl
from pyhornedowl.model import ObjectSomeValuesFrom, SubClassOf


def setup(namespace):
    workdir = tempfile.mkdtemp()
    namespace["workdir"] = workdir
    os.chdir(workdir)
    o = pyhornedowl.PyIndexedOntology()

    o.prefix_mapping.add_prefix("", "https://example.com/test#")
    axiom = SubClassOf(
        o.class_(":Child"),
        ObjectSomeValuesFrom(o.object_property(":has_parent"), o.class_(":Human")),
    )

    o.add_axiom(axiom)

    o.save_to_file("example.owx", serialization="owx")
    o.save_to_file("example.owl", serialization="rdf")
    o.save_to_file("example.ofn", serialization="ofn")
    o.save_to_file("example.omn", serialization="omn")
    o.save_to_file("example.obo", serialization="obo")


def teardown(namespace):
    shutil.rmtree(namespace["workdir"], ignore_errors=True)


pytest_collect_file = Sybil(
    parsers=[PythonCodeBlockParser(), DocTestParser()],
    patterns=["*.rst"],
    setup=setup,
    teardown=teardown,
).pytest()
