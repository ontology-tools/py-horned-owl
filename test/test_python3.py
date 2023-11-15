import pyhornedowl
from pyhornedowl.model import *

#ontoname = "https://raw.githubusercontent.com/addicto-org/addiction-ontology/master/addicto.owx"

ontoname = "test/addicto-merged.owx"

HAS_PART = "http://purl.obolibrary.org/obo/BFO_0000051"
PARTICIPATES_IN = "http://purl.obolibrary.org/obo/RO_0000056"

onto = pyhornedowl.open_ontology(ontoname)

clsid:str = onto.get_iri_for_label("tobacco smoker")

print(clsid)

clsid2:str = onto.get_iri_for_label("cigarette smoking")
print(clsid2)

print(f"Ontology has {len(onto.get_axioms())} axioms.")
# onto.add_axiom(['EquivalentClasses', clsid, ['ObjectSomeValuesFrom',PARTICIPATES_IN,clsid2]])
onto.add_axiom(EquivalentClasses([Class(onto.iri(clsid)), ObjectSomeValuesFrom(ObjectProperty(onto.iri(PARTICIPATES_IN)), Class(onto.iri(clsid2)))]))
print(f"Ontology has {len(onto.get_axioms())} axioms.")

clsid3:str = onto.get_iri_for_label("abstainer")

# onto.add_axiom(['DisjointClasses', clsid, clsid3])
onto.add_axiom(DisjointClasses([Class(onto.iri(clsid)), Class(onto.iri(clsid3))]))
print(f"Ontology has {len(onto.get_axioms())} axioms.")
