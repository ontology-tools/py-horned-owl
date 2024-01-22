import typing
import pyhornedowl.pyhornedowl as pho
import os
import re

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

with open("pyhornedowl/__init__.pyi", "w") as f:
    f.write("import typing\nfrom typing import *\n\n")
    f.write("import model\n")
    f.write("\n")

    for name, entry in pho.__dict__.items():
        if isinstance(entry, type):
            f.write(f"class {name}:\n")

            for member_name, member in entry.__dict__.items():
                if member_name.startswith("_"):
                    continue

                if hasattr(member, "__doc__"):
                    doc = member.__doc__
                    if doc is not None:
                        lines = doc.splitlines()
                        if len(lines) > 2:
                            sign = lines[0]

                            f.write(f"    def {sign}:\n")
                            doc = "\n".join([f"        {l}" for l in lines[2:]])
                            f.write(f'        """\n{doc}\n        """\n        ...\n\n')

        if callable(entry):
            if hasattr(entry, "__doc__"):
                doc = entry.__doc__
                if doc is not None:
                    lines = doc.splitlines()
                    if len(lines) > 2:
                        sign = lines[0]

                        f.write(f"def {sign}:\n")
                        doc = "\n".join([f"    {l}" for l in lines[2:]])
                        f.write(f'    """\n{doc}\n    """\n     ..\n\n')

            f.write("\n")


with open("pyhornedowl/py.typed", "w"):
    pass


with open("pyhornedowl/model/__init__.py", "w") as f:
    f.write("from ..pyhornedowl import model\n")
    f.write("import typing\n\n")

    al = []

    for name, entry in pho.model.__dict__.items():
        if not isinstance(entry, type) and not type(entry) == typing._UnionGenericAlias:
            continue

        f.write(f"{name} = model.{name}\n")
        al.append(name)

    # module_pyi = pyhornedowl.model.__pyi__()

    # al += [l.split("=")[0].strip() for l in module_pyi.split("\n") if len(l.strip()) > 0]    

    # f.write(module_pyi)
    f.write("\n")

    f.write(f"__all__ = {al}")

with open("pyhornedowl/model/__init__.pyi", "w") as f:
    f.write("import typing\n")
    f.write("from typing import *\n\n")

    for name, entry in pho.model.__dict__.items():
        if isinstance(entry, type):
            if "__pyi__" in entry.__dict__:
                pyi: str = entry.__pyi__()
                pyi = re.sub(r"    from: .*$|from: .*?[,)]", "", pyi)
                f.write(pyi)
            else:
                f.write(f"class {name}:\n")

                for attr_name, attr in entry.__dict__.items():
                    if attr_name.startswith("_") or attr_name == "from":
                        continue

                    f.write(f"    {attr_name}: Any\n")

                f.write("    ...\n")
        elif type(entry) == typing._UnionGenericAlias:
            f.write(f"{name} = {str(entry).replace('pyhornedowl.model.', '')}")
        else:
            continue

        f.write("\n")
    
    # f.write(pyhornedowl.model.__pyi__())
    f.write("\n")
