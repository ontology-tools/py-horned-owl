import re
from typing import List

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

def process_docstring(app, what, name, obj, options, lines: List[str]):
    md_link = re.compile(r'\[([^][]+)\]\(((?:[^()]+)+)\)')

    for i, line in enumerate(lines):
        lines[i] = re.sub(md_link, r"`\1 <\2>`_", line)
            

def setup(app):
    app.connect('autodoc-process-signature', process_signature)
    app.connect('autodoc-process-docstring', process_docstring)
