import pyhornedowl

from urllib.request import urlopen

ontourl = "https://raw.githubusercontent.com/addicto-org/addiction-ontology/master/addicto.owx"
data = urlopen(ontourl).read() #bytes
ontostring = data.decode('utf-8')

onto = pyhornedowl.open_ontology(ontostring)

print (f"Loaded ontology has {len(onto.get_classes())} classes.")
print (f"Loaded ontology has {len(onto.get_axioms())} axioms.")

clsid = onto.get_iri_for_label("tobacco smoker")

print(clsid)
