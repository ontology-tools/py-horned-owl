import pyhornedowl

#ontoname = "https://raw.githubusercontent.com/addicto-org/addiction-ontology/master/addicto.owx"

ontoname = "addicto.owx"

onto = pyhornedowl.open_ontology(ontoname)

clsid = onto.get_iri_for_label("tobacco smoker")

print(clsid)

#http://addictovocab.org/ADDICTO_ -> ADDICTO:
onto.add_prefix_mapping("ADDICTO","http://addictovocab.org/ADDICTO_")

clsshrtid = onto.get_id_for_iri(clsid)

print(clsshrtid)

clsid2 = onto.get_iri_for_id(clsshrtid)

print(clsid2)


clsid3 = onto.get_iri_for_id("oopsie")
print(clsid3)
