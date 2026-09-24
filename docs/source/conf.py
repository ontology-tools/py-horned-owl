# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'py-horned-owl'
copyright = '2026, Janna Hastings, Björn Gehrke'
author = 'Janna Hastings, Björn Gehrke'
release = 'unknown'


import datetime
import tomllib
from pathlib import Path

with open(Path(__file__).parent.parent.parent / "Cargo.toml", "rb") as f:
    cargo = tomllib.load(f)

pkg = cargo["package"]

project = pkg["name"]
version = pkg["version"]
release = pkg["version"]
copyright = f"{datetime.datetime.now().year}, " + ", ".join(
    a.split("<")[0].strip() for a in pkg.get("authors", [])
)


# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration
# import pathlib
# import sys
# sys.path.insert(0, pathlib.Path(__file__).parents[2].resolve().as_posix())

import sys
import os
sys.path.append(os.path.dirname(__file__))

import custom_doc

autosummary_imported_members = True
autodoc_default_options = {
    'members': True,
    'special-members': '__init__',
}
autodoc_typehints = "both"

autosummary_generate = True

# Union aliases (ClassExpression, Component, ...) are documented as ``py:type``
# by the autosummary module template so that annotations referring to them link.
type_aliases = custom_doc.type_aliases(
    'pyhornedowl', 'pyhornedowl.model', 'pyhornedowl.reasoning')
autosummary_context = {'type_aliases': type_aliases}


extensions = [
    'sphinx.ext.autodoc',
   'sphinx.ext.autosummary',
   "custom_doc"
]

templates_path = ['_templates']
exclude_patterns = []



# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'furo'
html_static_path = ['_static']
