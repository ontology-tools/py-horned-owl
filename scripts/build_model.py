import json
import os
from typing import Optional, Literal

from jinja2 import Environment, select_autoescape, FileSystemLoader, pass_context

REPO_ROOT=os.path.join(os.path.dirname(__file__), "..")

def as_py_type(val, module: Optional[str] = None):
    if isinstance(val, dict) and "type" in val:
        typ = val["type"]
        if "inner" in val:
            inner_ = val['inner']
            if typ == "Vec":
                return f"typing.List[{as_py_type(inner_)}]"
            if typ == "Box":
                return as_py_type(inner_)
            if typ == "BTreeSet":
                return f"typing.Set[{as_py_type(inner_)}]"
            if typ == "Option":
                return f"typing.Optional[{as_py_type(inner_)}]"

        return typ if module is None else f"{module}.{typ}"
    elif isinstance(val, list):
        return f"typing.Tuple[{','.join(as_py_type(x) for x in val)}]"
    else:
        return {
            "u32": "int",
            "String": "str",
            "StringWrapper": "str"
        }.get(val, val if module is None else f"{module}.{val}")


def as_rust_type(val):
    wrapped_types = {
        "Vec": "VecWrap",
        "Box": "BoxWrap",
        "BTreeSet": "BTreeSetWrap"
    }
    if isinstance(val, dict) and "type" in val:
        typ = val["type"]

        if "inner" in val:
            inner_ = val['inner']
            outer = wrapped_types.get(typ, typ)
            return f"{outer}<{as_rust_type(inner_)}>"
        else:
            return wrapped_types.get(typ, typ)
    elif isinstance(val, list):
        return f"({','.join(as_rust_type(x) for x in val)})"
    else:
        return wrapped_types.get(val, val)


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


def py_field(field, type):
    if isinstance(type, dict):
        return type.get("py_name", field)

    return field


@pass_context
def f_rust(ctx, val):
    if "variant" in ctx:
        fields = ctx["variant"]["fields"]
    else:
        fields = ctx["model"]["fields"]

    if isinstance(val, tuple) and isinstance(fields, dict):
        return val[0]
    if isinstance(val, str) and isinstance(fields, dict):
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

# horned-owl implements AsManchester for the OWL entities, the expressions and
# `Component` (see its io/omn/writer/as_manchester.rs). Every component type
# reaches the writer through `Component`; these wrappers are neither, so
# Manchester syntax can only render them inside the element that holds them.
NO_MANCHESTER = {"Annotation", "AnnotationProperty", "FacetRestriction"}


def manchester_route(name: str, component_variants: set) -> Optional[str]:
    """How a model class reaches horned-owl's Manchester writer.

    ``"direct"``            the class's own horned-owl type implements AsManchester
    ``"component"``         convert to ``Component`` first
    ``"wrapped-component"`` an AnnotatedComponent: render the component it wraps
    ``None``                no Manchester rendering
    """
    if name in NO_MANCHESTER:
        return None
    if name == "AnnotatedComponent":
        return "wrapped-component"
    if name in component_variants:
        return "component"
    return "direct"


def build_from_templates(lang: Literal["rs", "pyi"]):
    with open(os.path.join(REPO_ROOT, "template", "model.json")) as f:
        data = json.load(f)

    env = Environment(
        loader=FileSystemLoader(os.path.join(REPO_ROOT, "template", "templates")),
        autoescape=select_autoescape()
    )

    env.filters['as_py_type'] = as_py_type
    env.filters['as_rust_type'] = as_rust_type
    env.filters["as_py_name"] = as_py_name
    env.filters["py_field"] = py_field
    env.filters["fields"] = fields
    env.filters["f_rust"] = f_rust
    env.tests['list'] = lambda value: isinstance(value, list)

    out = []

    component_variants = set(
        next(m for m in data if m["name"] == "Component")["variants"].values()
    )

    header_template = env.get_template(f"static.{lang}")
    out.append(header_template.render(models=data) + "\n\n")

    for model in data:
        type = model["type"]
        route = manchester_route(model["name"], component_variants)
        # An enum's variants render through the parent enum's horned-owl type,
        # so the parent is the one that has to reach the Manchester writer.
        if type == "enum" and route != "direct":
            raise ValueError(
                f"{model['name']} is an enum but does not implement AsManchester; "
                "enum.rs.jinja2 renders its variants through it"
            )
        template = env.get_template(f"{type}.{lang}.jinja2")
        res = template.render(model=model, route=route)
        out.append(res + "\n")

    return "".join(out)

def main():

    with open(os.path.join(REPO_ROOT, "src", "model_generated.rs"), "w") as f:
        f.write(build_from_templates("rs"))


if __name__ == "__main__":
    main()
