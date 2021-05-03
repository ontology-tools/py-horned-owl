import pyhornedowl

#ontoname = "https://raw.githubusercontent.com/addicto-org/addiction-ontology/master/addicto.owx"

ontoname = "test/addicto-merged.owx"

HAS_PART = "http://purl.obolibrary.org/obo/BFO_0000051"
PARTICIPATES_IN = "http://purl.obolibrary.org/obo/RO_0000056"

onto = pyhornedowl.open_ontology(ontoname)

clsid = onto.get_iri_for_label("tobacco smoker")

print(clsid)

clsid2 = onto.get_iri_for_label("cigarette smoking")
print(clsid2)

print(f"Ontology has {len(onto.get_axioms())} axioms.")
onto.add_axiom(['EquivalentClasses', clsid, ['ObjectSomeValuesFrom',PARTICIPATES_IN,clsid2]])
print(f"Ontology has {len(onto.get_axioms())} axioms.")

clsid3 = onto.get_iri_for_label("abstainer")

onto.add_axiom(['DisjointClasses', clsid, clsid3])
print(f"Ontology has {len(onto.get_axioms())} axioms.")
