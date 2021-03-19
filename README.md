# py-horned-owl
An experimental bridge from horned-owl to python using PyO3. 


## Installation

### Published version 
To install the published library, which is currently only available for python 3.8.x on MacOS: 

`pip install py_horned_owl`

### From sources
To build locally from sources, you will need [Rust](https://www.rust-lang.org/tools/install), [PyO3](https://github.com/PyO3/pyo3) and [Maturin](https://github.com/PyO3/maturin). 

First check out the associated Python-compatible version of horned-owl: 
`git clone https://github.com/jannahastings/horned-owl`

Then check out this repository: 
`git clone https://github.com/jannahastings/py-horned-owl/`

In the directory py-horned-owl, create and activate a virtual Python environment: 

`virtualenv py-horned-owl`

`source bin/activate`

Then you can get maturin to build the library and install it into the virtual Python environment with: 

`maturin develop`


## Usage

Currently the library only supports loading ontologies from `.owx` (OWL-XML) files. Some test files in this format are available in the [owl-xml](https://github.com/jannahastings/horned-owl/tree/main/src/ont/owl-xml) horned-owl folder. Alternatively, [ROBOT](http://robot.obolibrary.org/) can transform ontologies into this format using `robot convert`. 

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







