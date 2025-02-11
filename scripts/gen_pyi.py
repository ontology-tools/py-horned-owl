import glob
import inspect
import json
import os
import re
import typing

from build_model import as_py_type, as_py_name, build_from_templates


REPO_ROOT=os.path.join(os.path.dirname(__file__), "..")
_CWD = os.getcwd()
os.chdir(REPO_ROOT)


old_files = glob.glob("pyhornedowl/**/__init__.*", recursive=True)

for f in old_files:
    os.unlink(f)

import pyhornedowl.pyhornedowl as pho

os.makedirs("pyhornedowl/model", exist_ok=True)

with open("pyhornedowl/__init__.py", "w") as f:
    pyhornedowl_members = [k for k, v in pho.__dict__.items() if isinstance(v, type) or callable(v)]

    f.write("from __future__ import annotations\n")
    f.write("from .pyhornedowl import ")
    f.write(", ".join(pyhornedowl_members))

    f.write("\n\n")
    f.write("__all__ = [")
    f.write(", ".join([f'"{n}"' for n in pyhornedowl_members]))
    f.write("]\n")

implemented_magic = [f"__{x}__" for x in
                     ["invert", "and", "or", "invert", "iter", "getitem", "setitem", "contains", "len", "delitem"]]

with open("pyhornedowl/__init__.pyi", "w") as f:
    f.write("import typing\n")
    f.write("from typing import *\n")
    f.write("from typing_extensions import deprecated\n\n")
    f.write("import model\n")
    f.write("\n")

    for name, entry in pho.__dict__.items():
        if isinstance(entry, type):
            f.write(f"class {name}:\n")
            # There appears to be a bug with pyo3. Documentation on enum 
            # variants is not attached to their mapped python types. Hence we 
            # use a workarround of adding their documentation to the enum in
            # the style: "<MemberName>: <doc string>".
            member_docs = {}
            if hasattr(entry, "__doc__"):
                entry_doc = entry.__doc__
                if entry_doc is not None:
                    f.write("    \"\"\"\n")
                    for line in entry_doc.splitlines():
                        member_doc_m = re.match(r"^(\w+): (.*)$", line)
                        if member_doc_m:
                            member_docs[member_doc_m.group(1)]=member_doc_m.group(2)
                        else:
                            f.write(f"    {line}\n")

                    f.write("    \"\"\"\n")

            for member_name, member in entry.__dict__.items():
                if member_name.startswith("_") and member_name not in implemented_magic:
                    continue
                
                # E.g. for enums
                if isinstance(member, entry):
                    f.write(f"    {member_name}: typing.Self\n")
                    if member_name in member_docs or hasattr(member, "__doc__") and member.__doc__ is not None:
                        doc = member_docs.get(member_name, getattr(member, "__doc__"))
                        f.write("    \"\"\"\n")
                        for line in doc.splitlines():
                            f.write(f"    {line}\n")
                        f.write("    \"\"\"\n")

                    continue

                if hasattr(member, "__doc__"):
                    doc = member.__doc__
                    if doc is not None:
                        lines = doc.splitlines()
                        if len(lines) > 2:
                            annotations_end = lines.index(next(x for x in lines if not x.startswith("@")), 0)
                            annotations = lines[:annotations_end]
                            sign = lines[annotations_end] 

                            for ann in annotations:
                                f.write(f"    {ann}\n")

                            if callable(member):
                                f.write(f"    def {sign}:\n")
                                doc = "\n".join([f"        {l}" for l in lines[annotations_end+2:]])
                                f.write(f'        """\n{doc}\n        """\n        ...\n\n')
                            else:
                                f.write(f"    {sign}\n")
                                doc = "\n".join([f"    {l}" for l in lines[annotations_end+2:]])
                                f.write(f'    """\n{doc}\n    """\n\n')

                            continue

                if callable(member):
                    f.write(f"    def {member_name}{inspect.signature(member)}:\n        ...\n\n")

                    continue
                                
                    
            f.write("\n")
        elif callable(entry):
            if hasattr(entry, "__doc__"):
                doc = entry.__doc__
                if doc is not None:
                    lines = doc.splitlines()
                    if len(lines) > 2:
                        annotations_end = lines.index(next(x for x in lines if not x.startswith("@")), 0)
                        annotations = lines[:annotations_end]
                        sign = lines[annotations_end] 

                        for ann in annotations:
                            f.write(f"{ann}\n")
                        f.write(f"def {sign}:\n")
                        doc = "\n".join([f"    {l}" for l in lines[annotations_end+2:]])
                        f.write(f'    """\n{doc}\n    """\n    ...\n\n')

            f.write("\n")

with open("pyhornedowl/py.typed", "w"):
    pass


def handle_module(module: str):
    with open(f"pyhornedowl/{module}/__init__.py", "w") as f:
        f.write(f"from ..pyhornedowl import {module}\n")
        f.write("import typing\n\n")

        al = []

        for name, entry in getattr(pho, module).__dict__.items():
            if not isinstance(entry, type) and not type(entry) == typing._UnionGenericAlias:
                continue

            f.write(f"{name} = {module}.{name}\n")
            al.append(name)

        f.write("\n")

        f.write(f"__all__ = {al}")

    with open(f"pyhornedowl/{module}/__init__.pyi", "w") as f:
        f.write(build_from_templates("pyi"))
        # f.write("import typing\n")
        # f.write("from typing import *\n")
        #
        #
        #
        # for name, entry in getattr(pho, module).__dict__.items():
        #     if isinstance(entry, type):
        #         f.write(f"class {name}:\n")
        #         for attr_name, attr in entry.__dict__.items():
        #             if attr_name.startswith("_") and attr_name not in implemented_magic:
        #                 continue
        #
        #             doc = attr.__doc__ if hasattr(attr, "__doc__") else ""
        #             doc = None if doc == "" else doc
        #             if callable(attr):
        #                 if attr_name.startswith("__"):
        #                     f.write(f"    def {attr_name}{str(inspect.signature(attr))}:\n")
        #             else:
        #                 f.write(f"    {doc if doc else f"{attr_name}: Any"}\n")
        #
        #         f.write("    ...\n")
        #     elif type(entry) == typing._UnionGenericAlias:
        #         f.write(f"{name} = {str(entry).replace(f'pyhornedowl.{module}.', '')}")
        #     else:
        #         continue
        #
        #     f.write("\n")
        #
        # f.write("\n")


handle_module("model")

os.chdir(_CWD)
