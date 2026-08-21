import tempfile, shutil, os
from sybil import Sybil
from sybil.parsers.rest import PythonCodeBlockParser, DocTestParser
import pyhornedowl
from pyhornedowl.model import ObjectSomeValuesFrom, SubClassOf


def setup(namespace):
    workdir = tempfile.mkdtemp()
    namespace["workdir"] = workdir
    os.chdir(workdir)
    o = pyhornedowl.PyIndexedOntology()

    o.add_prefix_mapping("", "https://example.com/test#")
    axiom = SubClassOf(
        o.class_(":Child"),
        ObjectSomeValuesFrom(o.object_property(":has_parent"), o.class_(":Human")),
    )

    o.add_axiom(axiom)

    o.save_to_file("example.owx")
    o.save_to_file("example.owl")
    o.save_to_file("example.ofn", serialization="ofn")


def teardown(namespace):
    shutil.rmtree(namespace["workdir"], ignore_errors=True)


pytest_collect_file = Sybil(
    parsers=[PythonCodeBlockParser(), DocTestParser()],
    patterns=["*.rst"],
    setup=setup,
    teardown=teardown,
).pytest()
