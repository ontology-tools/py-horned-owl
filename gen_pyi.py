import pyhornedowl
import os

os.makedirs("pyhornedowl/model", exist_ok=True)

with open("pyhornedowl/__init__.py", "w") as f:
    f.write("from .pyhornedowl import *")


with open("pyhornedowl/py.typed", "w"):
    pass


with open("pyhornedowl/model/__init__.py", "w") as f:
    f.write("from ..pyhornedowl import model\n")
    f.write("import typing\n\n")

    al = []

    for name, entry in pyhornedowl.model.__dict__.items():
        if not isinstance(entry, type):
            continue

        f.write(f"{name} = model.{name}\n")
        al.append(name)

    module_pyi = pyhornedowl.model.__pyi__()

    al += [l.split("=")[0].strip() for l in module_pyi.split("\n") if len(l.strip()) > 0]    

    f.write(module_pyi)
    f.write("\n")

    f.write(f"__all__ = {al}")

with open("pyhornedowl/model/__init__.pyi", "w") as f:
    f.write("import typing\n")
    f.write("from typing import *\n\n")

    for name, entry in pyhornedowl.model.__dict__.items():
        if not isinstance(entry, type):
            continue

        if "__pyi__" in entry.__dict__:
            f.write(entry.__pyi__())
        else:
            f.write(f"class {name}:\n")

            for attr_name, attr in entry.__dict__.items():
                if attr_name.startswith("_"):
                    continue

                f.write(f"    {attr_name}: Any\n")

            f.write(f"    def __init__(self, {', '.join([k for k in entry.__dict__.keys() if not k.startswith('_')])}):\n        ...\n")
            f.write("    ...\n")

        f.write("\n")
    
    f.write(pyhornedowl.model.__pyi__())
    f.write("\n")
