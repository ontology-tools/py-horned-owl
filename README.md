# py-horned-owl
An experimental bridge from horned-owl to python using PyO3. 


## Installation

### Published version 
To install the published library: 

```
pip install py_horned_owl
```

### From sources
To build locally from sources, you will need [Rust](https://www.rust-lang.org/tools/install), [PyO3](https://github.com/PyO3/pyo3) and [Maturin](https://github.com/PyO3/maturin). 

Check out this repository and create and activate a virtual Python environment in the directory of your local copy: 

```
virtualenv py-horned-owl
source bin/activate
```

Then you can get maturin to build the library and install it into the virtual Python environment with: 

`maturin develop`

To build the python stubs compile with the `pyi` cfg flag and run `gen_pyi.py`

```bash
maturin develop -- --cfg pyi --check-cfg "cfg(pyi)"
python3 gen_pyi.py
```


## Usage

The library supports loading ontologies from `.owl` (RDF-XML) and `.owx` (OWL-XML) files via horned-owl's parsing functionality. [ROBOT](http://robot.obolibrary.org/) can transform ontologies that are in other OWL flavours into one of these formats using `robot convert`. 

Example of simple usage:

```py
import pyhornedowl

ontoname = "family.owx"

onto = pyhornedowl.open_ontology(ontoname)

print (f"Loaded ontology has {len(onto.get_classes())} classes.")
print (f"Loaded ontology has {len(onto.get_axioms())} axioms.")

for c in onto.get_classes():
    print(onto.get_axioms_for_iri(c))


```

For more information please visit the [documentation](https://ontology-tools.github.io/py-horned-owl/). 






