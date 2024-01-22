# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'py-horned-owl'
copyright = '2024, Janna Hastings, Björn Gehrke'
author = 'Janna Hastings, Björn Gehrke'
release = '0.3.1'

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

autodoc_type_aliases = {
    'ClassExpression ': 'pyhornedowl.module.ClassExpression',
}

extensions = [
    'sphinx.ext.autodoc',
   'sphinx.ext.autosummary',
   "custom_doc"
]

templates_path = ['_templates']
exclude_patterns = []



# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']
