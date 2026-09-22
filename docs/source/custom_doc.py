import importlib
import re
import types
import typing

from sphinx.util.typing import stringify_annotation


def _unions(module_name: str):
    """The module's union aliases (ClassExpression, Component, ...)."""
    module = importlib.import_module(module_name)
    return {
        name: value
        for name, value in vars(module).items()
        if not name.startswith("_") and typing.get_origin(value) is typing.Union
    }


def type_aliases(*modules: str) -> dict[str, dict[str, str]]:
    """{module: {alias: "A | B | ..."}} for the autosummary module template."""
    return {
        module_name: {
            name: stringify_annotation(value, "smart")
            for name, value in _unions(module_name).items()
        }
        for module_name in modules
    }


def skip_member(app, what, name, obj, skip, options):
    # Fields are documented in their class' docstring (:ivar:/:vartype:), so
    # their getters would only repeat the name without adding anything.
    return isinstance(obj, types.GetSetDescriptorType) or skip


def process_signature(app, what, name, obj, options, signature: str, return_annotation):
    if signature is not None:
        signature = re.sub(r"BoxWrap<(.*)>", r"\1", signature)
        signature = signature.replace("<", "[")
        signature = signature.replace(">", "]")
        signature = signature.replace("VecWrap", "typing.List")
        signature = signature.replace("StringWrap", "str")
        signature = signature.replace("BTreeSetWrap", "typing.Set")
        signature = signature.replace("u32", "int")

    if return_annotation is not None:
        return_annotation = re.sub(r"BoxWrap<(.*)>", r"\1", return_annotation)
        return_annotation = return_annotation.replace("<", "[")
        return_annotation = return_annotation.replace(">", "]")
        return_annotation = return_annotation.replace("VecWrap", "typing.List")
        return_annotation = return_annotation.replace("StringWrap", "str")
        return_annotation = return_annotation.replace("BTreeSetWrap", "typing.Set")
        return_annotation = return_annotation.replace("u32", "int")

    return (signature, return_annotation)


def process_docstring(app, what, name, obj, options, lines: list[str]):
    text = "\n".join(lines)
    md_code = re.compile(r"(`[^`]+`)")
    md_internal_link = re.compile(r"\[([^][]+)\]\(struct\.([^()]+)\.html\)")
    md_link = re.compile(r"\[([^][]+)\]\(([^()]+)\)")

    text = re.sub(md_code, r":code:\1", text)
    text = re.sub(md_internal_link, r":py:class:`~pyhornedowl.model.\2`", text)
    text = re.sub(md_link, r"`\1 <\2>`_", text)

    lines.clear()
    lines += text.splitlines()


def setup(app):
    app.connect("autodoc-skip-member", skip_member)
    app.connect("autodoc-process-signature", process_signature)
    app.connect("autodoc-process-docstring", process_docstring)
