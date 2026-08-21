# py-horned-owl
An experimental bridge from horned-owl to python using PyO3. 


## Installation

### Published version 
To install the published library: 

`pip install py_horned_owl`

### From sources
To build locally from sources, you will need [Rust](https://www.rust-lang.org/tools/install), [PyO3](https://github.com/PyO3/pyo3) and [Maturin](https://github.com/PyO3/maturin). 

Check out this repository: 
`git clone https://github.com/ontology-tools/py-horned-owl/`

In the directory py-horned-owl, create and activate a virtual Python environment: 

`virtualenv py-horned-owl`

`source bin/activate`

Then you can get maturin to build the library and install it into the virtual Python environment with: 

`maturin develop`


## Usage

The library parses every OWL serialization that horned-owl supports. The serialization is inferred from the file extension or the content, or you can pass it explicitly (e.g. `open_ontology(text, "omn")`):

| Serialization | `serialization` value | Typical extension |
| --- | --- | --- |
| RDF/XML | `"rdf"` (or `"owl"`) | `.owl`, `.rdf` |
| OWL/XML | `"owx"` | `.owx` |
| OWL Functional Syntax | `"ofn"` | `.ofn` |
| OWL 2 Manchester Syntax | `"omn"` (or `"manchester"`) | `.omn` |
| OBO flat file | `"obo"` | `.obo` |

Other RDF serializations recognised by [oxrdfio](https://docs.rs/oxrdfio/) (Turtle, N-Triples, and so on) are accepted by their extension. The same values work for `save_to_string` and `save_to_file`. For anything not listed here, [ROBOT](http://robot.obolibrary.org/)'s `robot convert` can transform an ontology into one of the above.

To render a *single* axiom or class expression rather than a whole ontology, use `write_snippet`:

```python
import pyhornedowl

onto = pyhornedowl.open_ontology(text)
for ac in onto.get_axioms():
    print(pyhornedowl.write_snippet(ac))              # Manchester (default)
    print(pyhornedowl.write_snippet(ac, "ofn"))       # functional syntax
```

`write_snippet` accepts `"omn"` (alias `"manchester"`) and `"ofn"` (alias `"functional"`), the serializations for which horned-owl provides a per-element writer; the OWL/XML and RDF writers operate on whole ontologies only. It is the per-element counterpart to `save_to_string`, and for Manchester it is the only option: `save_to_string("omn")` groups axioms into entity frames, which cannot be sliced back into individual axioms. 

Example of simple usage:

```
import pyhornedowl

ontoname = "family.owx"

onto = pyhornedowl.open_ontology(ontoname)

print (f"Loaded ontology has {len(onto.get_classes())} classes.")
print (f"Loaded ontology has {len(onto.get_axioms())} axioms.")

for c in onto.get_classes():
    print(onto.get_axioms_for_iri(c))


```

For more information please visit the [documentation](https://ontology-tools.github.io/py-horned-owl/). 


## Citing 
Phillip Lord, Björn Gehrke, Martin Larralde, Janna Hastings, Filippo De Bortoli, James A. Overton, James P. Balhoff, and Jennifer Warrender. Horned-OWL: Flying Further and Faster with Ontologies. In Special Issue on Resources for Graph Data and Knowledge. Transactions on Graph Data and Knowledge (TGDK), Volume 2, Issue 2, pp. 9:1-9:14, Schloss Dagstuhl – Leibniz-Zentrum für Informatik (2024) https://doi.org/10.4230/TGDK.2.2.9


