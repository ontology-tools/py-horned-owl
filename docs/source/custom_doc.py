import ast
import functools
import importlib
import re
import types
import typing
from pathlib import Path

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
    # Enum variants are documented in their class' docstring (:cvar:) as well.
    is_variant = getattr(type(obj), name, None) is obj
    return isinstance(obj, types.GetSetDescriptorType) or is_variant or skip


class _AliasNames(ast.NodeTransformer):
    """Writes a union of classes as the model alias with the same members."""

    def __init__(self, aliases: dict[frozenset[str], str]):
        self.aliases = aliases

    def visit_BinOp(self, node):
        members = []

        def collect(n):
            if isinstance(n, ast.BinOp) and isinstance(n.op, ast.BitOr):
                collect(n.left)
                collect(n.right)
            else:
                members.append(n)

        collect(node)
        alias = self.aliases.get(frozenset(ast.unparse(m) for m in members))
        if alias is None:
            return self.generic_visit(node)
        return ast.Name(alias)


@functools.cache
def _stub_signatures() -> dict[str, tuple[str, str | None]]:
    """{public dotted name: (signature, return annotation)} from the generated stubs.

    PyO3's own ``__text_signature__`` carries no types, and the docstrings do
    not repeat the signature, so the typed signatures come from the ``.pyi``
    files ``maturin --generate-stubs`` writes next to the native module.
    """
    native = importlib.import_module("pyhornedowl.pyhornedowl")
    stub_dir = Path(native.__file__).parent / "pyhornedowl"
    if not stub_dir.is_dir():
        raise RuntimeError(
            f"No type stubs in {stub_dir}; build with `maturin develop --generate-stubs`."
        )

    aliases = {
        frozenset(a.__name__ for a in typing.get_args(v)): name
        for name, v in _unions("pyhornedowl.model").items()
        if all(isinstance(a, type) for a in typing.get_args(v))
    }
    shorten = _AliasNames(aliases)

    def render(fn: ast.FunctionDef, method: bool) -> tuple[str, str | None]:
        args = shorten.visit(fn.args)
        if method:  # drop self / cls
            if args.posonlyargs:
                args.posonlyargs = args.posonlyargs[1:]
            else:
                args.args = args.args[1:]
        returns = ast.unparse(shorten.visit(fn.returns)) if fn.returns else None
        return f"({ast.unparse(args)})", returns

    signatures = {}
    for stub in stub_dir.glob("*.pyi"):
        module = "pyhornedowl" if stub.stem == "__init__" else f"pyhornedowl.{stub.stem}"
        for node in ast.parse(stub.read_text()).body:
            if isinstance(node, ast.FunctionDef):
                signatures.setdefault(f"{module}.{node.name}", render(node, method=False))
            elif isinstance(node, ast.ClassDef):
                for item in node.body:
                    if not isinstance(item, ast.FunctionDef):
                        continue
                    if item.name == "__new__":
                        signatures[f"{module}.{node.name}"] = (render(item, method=True)[0], None)
                    # setdefault: a property's setter follows its getter
                    signatures.setdefault(f"{module}.{node.name}.{item.name}", render(item, method=True))
    return signatures


def process_signature(app, what, name, obj, options, signature: str, return_annotation):
    return _stub_signatures().get(name, (signature, return_annotation))


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
    pass
    app.connect("autodoc-skip-member", skip_member)
    app.connect("autodoc-process-signature", process_signature)
    app.connect("autodoc-process-docstring", process_docstring)
