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
| OWL 2 Manchester Syntax | `"omn"` | `.omn` |
| OBO flat file | `"obo"` | `.obo` |

Other RDF serializations recognised by [oxrdfio](https://docs.rs/oxrdfio/) (Turtle, N-Triples, and so on) are accepted by their extension. The same values work for `save_to_string` and `save_to_file`. For anything not listed here, [ROBOT](http://robot.obolibrary.org/)'s `robot convert` can transform an ontology into one of the above.

```python
from pyhornedowl import Ontology
from pyhornedowl.model import SubClassOf

onto = Ontology()  # Or open_ontology("family.owl").
onto.add_prefix_mapping("ex", "https://example.org/")
child, person = onto.class_("ex:Child"), onto.class_("ex:Person")
onto.add_axiom(SubClassOf(child, person))
onto.declare(child)
onto.set_label(child, "Child")

print(onto.signature().classes())  # All used classes, including undeclared ones.
print(onto.get_classes())         # Declared Class objects.
for axiom in onto.query_components(SubClassOf, including=child):
    print(axiom.serialize())      # Functional syntax; also accepts "omn".

onto.save_to_file("family.ofn")
```

`get_annotations(entity, property=None)` returns model values, retaining
language tags and datatypes. `get_label(entity, ["en", ""])` chooses a label
by language preference; its default is untagged labels only.

For more information please visit the [documentation](https://ontology-tools.github.io/py-horned-owl/). 


## Citing 
Phillip Lord, Björn Gehrke, Martin Larralde, Janna Hastings, Filippo De Bortoli, James A. Overton, James P. Balhoff, and Jennifer Warrender. Horned-OWL: Flying Further and Faster with Ontologies. In Special Issue on Resources for Graph Data and Knowledge. Transactions on Graph Data and Knowledge (TGDK), Volume 2, Issue 2, pp. 9:1-9:14, Schloss Dagstuhl – Leibniz-Zentrum für Informatik (2024) https://doi.org/10.4230/TGDK.2.2.9


