import json
import os
import subprocess
from bisect import insort

from jinja2 import Environment, PackageLoader, select_autoescape, FileSystemLoader, pass_context

with open("./model.json") as f:
    data = json.load(f)



env = Environment(
    loader=FileSystemLoader("./templates"),
    autoescape=select_autoescape()
)

def as_py_type(val):
    if isinstance(val, dict) and "type" in val:
        typ = val["type"]
        if "inner" in val:
            inner_ = val['inner']
            if typ == "Vec":
                return f"List[{as_py_type(inner_)}]"
            if typ == "Box":
                return typ
            if typ == "BTreeSet":
                return f"Set[{as_py_type(inner_)}]"
        else:
            return typ
    elif isinstance(val, list):
        return f"Tuple[{','.join(as_py_type(x) for x in val)}]"
    else:
        return {
            "u32": "int",
            "String": "str",
        }.get(val, val)

def as_rust_type(val):
    if isinstance(val, dict) and "type" in val:
        typ = val["type"]
        if "inner" in val:
            inner_ = val['inner']
            return f"{typ}<{as_rust_type(inner_)}>"
        else:
            return typ
    elif isinstance(val, list):
        return f"({','.join(as_rust_type(x) for x in val)})"
    else:
        return val

def as_py_name(val):
    return {
        0: "first",
        1: "second",
        2: "third",
        3: "fourth",
        4: "fifth",
        5: "sixth",
        6: "seventh",
        7: "eighth",
        8: "ninth",
        9: "tenth",
    }.get(val, val)

@pass_context
def f_py(ctx, val):
    if "variant" in ctx:
        fields = ctx["variant"]["fields"]
    else:
        fields = ctx["model"]["fields"]

    if isinstance(val, tuple) and isinstance(fields, dict):
        return val[1].get("py_name", val[0])
    if isinstance(val, str)  and isinstance(fields, dict):
        return fields[val].get("py_name", val)
    if isinstance(val, list) and "loop" in ctx:
        return as_py_name(ctx["loop"]["index"])


@pass_context
def f_rust(ctx, val):
    if "variant" in ctx:
        fields = ctx["variant"]["fields"]
    else:
        fields = ctx["model"]["fields"]

    if isinstance(val, tuple) and isinstance(fields, dict):
        return val[0]
    if isinstance(val, str)  and isinstance(fields, dict):
        return val
    if isinstance(val, list) and "loop" in ctx:
        return ctx["loop"]["index0"]

    return val

def fields(val):
    if isinstance(val, dict):
        return val.items()
    if isinstance(val, list):
        return list(enumerate(val))

    return val


env.filters['as_py_type'] = as_py_type
env.filters['as_rust_type'] = as_rust_type
env.filters["as_py_name"] = as_py_name
env.filters["f_py"] = f_py
env.filters["fields"] = fields
env.filters["f_rust"] = f_rust
env.tests['list'] = lambda value: isinstance(value, list)

out = []

header_template = env.get_template("header.jinja2")
out.append(header_template.render(models=data) + "\n\n")

for model in data:
    type = model["type"]
    template = env.get_template(type + ".jinja2")
    res = template.render(model=model)

    out.append(res + "\n")


with open("model.rs", "w") as f:
    f.writelines(out)
