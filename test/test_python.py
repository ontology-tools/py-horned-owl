import pyhornedowl
import time

print("Loading ontology")

ontoname = "test/chebi_core.owx"

FULL = True
RDFSLABEL = "http://www.w3.org/2000/01/rdf-schema#label"
SMILES = "http://purl.obolibrary.org/obo/chebi/smiles"
DEFN = "http://purl.obolibrary.org/obo/IAO_0000115"
SYN = "http://purl.obolibrary.org/obo/IAO_0000115"
HAS_ROLE = "http://purl.obolibrary.org/obo/RO_0000087"
HAS_PART = "http://purl.obolibrary.org/obo/BFO_0000051"
CHARGE = "http://purl.obolibrary.org/obo/chebi/charge"

onto = pyhornedowl.open_ontology(ontoname)

print (f"Loaded ontology has {len(onto.get_classes())} classes.")
print (f"Loaded ontology has {len(onto.get_axioms())} axioms.")

#print("Sleeping")
#time.sleep(5)

clssname = "caffeine"
clssid = onto.get_iri_for_label(clssname)
print(f"Class {clssid} ({clssname}) has axioms: {onto.get_axioms_for_iri(clssid)}")

onto.add_axiom(['SubClassOf',clssid,'http://testing.testing'])
onto.add_axiom(['AnnotationAssertion', 'http://purl.obolibrary.org/obo/CHEBI_27732', 'http://test_annotation', 'Testing Added Annotation Value'])
print (f"Ontology now has {len(onto.get_axioms())} axioms.")

print ("Removing one axiom")
onto.remove_axiom(['SubClassOf', 'http://purl.obolibrary.org/obo/CHEBI_27732', 'http://purl.obolibrary.org/obo/CHEBI_27134'])
print (f"Ontology now has {len(onto.get_axioms())} axioms.")

print ("Adding a complex axiom")

onto.add_axiom(['SubClassOf', 'http://purl.obolibrary.org/obo/CHEBI_27732', ['ObjectSomeValuesFrom', 'http://test-op', 'http://test-ob-target']])

onto.add_axiom(['SubClassOf', 'http://purl.obolibrary.org/obo/CHEBI_27732', ['ObjectIntersectionOf', 'http://purl.obolibrary.org/obo/CHEBI_27134', 'http://test-ob-target']])


print (f"Ontology now has {len(onto.get_axioms())} axioms.")

print (f"Getting all classes with charge and no smiles")
# Get all classes with asserted charge but no smiles
if FULL:
    classes_with_charge = set()
    classes_with_smiles = set()
    for a in onto.get_axioms():
        #Example: ['AnnotationAssertion', 'http://purl.obolibrary.org/obo/CHEBI_27732', 'http://www.geneontology.org/formats/oboInOwl#hasAlternativeId', 'CHEBI:41472']
        if len(a)==4 and a[0]=='AnnotationAssertion':
            if a[2]==CHARGE:
                chg = a[3]
                classes_with_charge.add(a[1])
            if a[2]==SMILES:
                smls = a[3]
                classes_with_smiles.add(a[1])
    classes_with_charge_no_smiles = [c for c in classes_with_charge if c not in classes_with_smiles]
    print(f"The ontology has {len(classes_with_charge_no_smiles)} classes with charge and no smiles.")

# Get all the asserted parts:
if FULL:
    print("Getting all asserted part relations")
    asserted_parts = set() # A list of tuples
    for a in onto.get_axioms():
        # Example: ['SubClassOf', 'http://purl.obolibrary.org/obo/CHEBI_27732', ['ObjectSomeValuesFrom', 'http://purl.obolibrary.org/obo/RO_0000087', 'http://purl.obolibrary.org/obo/CHEBI_85234']]
        if len(a)==3 and a[0]=='SubClassOf' and \
          isinstance(a[2], list) and len(a[2])==3 and \
          a[2][0]=='ObjectSomeValuesFrom' and a[2][1]==HAS_PART:
            asserted_parts.add((a[1],a[2][2]))

    print(f"There are {len(asserted_parts)} parthood relations asserted in ChEBI.")

#print("Sleeping")
#time.sleep(5)

#print("Axioms for IRI ",clssid,": ",onto.get_axioms_for_iri(clssid))

if FULL:
    for input_label in ["carboxy group","codeine","caffeine"]:
        clssid = onto.get_iri_for_label(input_label)
        output_label = onto.get_annotation(clssid,RDFSLABEL)
        print(f"Got label '{output_label}' for iri {clssid}")
        output_def = onto.get_annotation(clssid,DEFN)
        print(f"Got definition '{output_def}' for iri {clssid}")
        output_sm = onto.get_annotation(clssid,SMILES)
        print(f"Got SMILES '{output_sm}' for iri {clssid}")
        output_ax = onto.get_axioms_for_iri(clssid)
        print(f"Got axioms for iri {clssid}: {output_ax}")


#print("Sleeping")
#time.sleep(5)

if FULL:
    clssid = onto.get_iri_for_label("caffeine")
    ancestors = pyhornedowl.get_ancestors(onto,clssid)
    print(f"Got ancestors {ancestors} for class {clssid}")
    print("Getting a whole lot of labels for IRIs")
    descs = pyhornedowl.get_descendants(onto,onto.get_iri_for_label('carboxylic acid'))
    desc_names = [onto.get_annotation(d,RDFSLABEL) for d in descs]
    print(len(desc_names))

if FULL:
    newlabel = "caffeine2"
    onto.set_label(clssid,newlabel)
    print(f"The new label of the class {clssid} is '{onto.get_annotation(clssid,RDFSLABEL)}'")

#print("Sleeping")
#time.sleep(5)

# Currently this is a bit slow. But it does work!
#if FULL:
#    onto.save_to_file("test/chebi-updated.owx")

print("Quitting")
