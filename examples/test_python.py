import time

import pyhornedowl
from pyhornedowl.model import *

print("Loading ontology")

ontoname = "chebi_core.owl"

FULL = True
RDFSLABEL = "http://www.w3.org/2000/01/rdf-schema#label"
SMILES = "http://purl.obolibrary.org/obo/chebi/smiles"
DEFN = "http://purl.obolibrary.org/obo/IAO_0000115"
SYN = "http://purl.obolibrary.org/obo/IAO_0000115"
HAS_ROLE = "http://purl.obolibrary.org/obo/RO_0000087"
HAS_PART = "http://purl.obolibrary.org/obo/BFO_0000051"
CHARGE = "http://purl.obolibrary.org/obo/chebi/charge"

start = time.time_ns()
onto = pyhornedowl.open_ontology(ontoname)
end = time.time_ns()
print(f"Loaded {ontoname} in {end - start}ns")

print (f"Loaded ontology has {len(onto.get_classes())} classes.")
print (f"Loaded ontology has {len(onto.get_axioms())} axioms.")

#print("Sleeping")
#time.sleep(5)

clssname = "caffeine"
clssid: str = onto.get_iri_for_label(clssname)

print(f"{clssname} has iri {clssid}")

print(f"Class {clssid} ({clssname}) has axioms: {onto.get_axioms_for_iri(clssid)}")


onto.add_axiom(SubClassOf(Class(onto.iri("https://test.test/test")), Class(  onto.iri('http://testing.testing'))))
onto.add_axiom(AnnotationAssertion(onto.iri('http://test_annotation'), Annotation(AnnotationProperty( onto.iri('http://purl.obolibrary.org/obo/CHEBI_27732')), SimpleLiteral('Testing Added Annotation Value'))))
print (f"Ontology now has {len(onto.get_axioms())} axioms.")


print ("Removing one axiom")
onto.remove_axiom(SubClassOf(Class(onto.iri("https://test.test/test")), Class(  onto.iri('http://testing.testing'))))
print (f"Ontology now has {len(onto.get_axioms())} axioms.")

print ("Adding a complex axiom")

# onto.add_axiom(['SubClassOf', 'http://purl.obolibrary.org/obo/CHEBI_27732', ['ObjectSomeValuesFrom', 'http://test-op', 'http://test-ob-target']])
onto.add_axiom(SubClassOf(Class(onto.iri("http://purl.obolibrary.org/obo/CHEBI_27732")), ObjectSomeValuesFrom(ObjectProperty(onto.iri('http://test-op')), Class(onto.iri('http://test-ob-target')))))

# onto.add_axiom(['SubClassOf', 'http://purl.obolibrary.org/obo/CHEBI_27732', ['ObjectIntersectionOf', 'http://purl.obolibrary.org/obo/CHEBI_27134', 'http://test-ob-target']])
onto.add_axiom(SubClassOf(Class(onto.iri("http://purl.obolibrary.org/obo/CHEBI_27732")), ObjectIntersectionOf([Class(onto.iri('http://purl.obolibrary.org/obo/CHEBI_27134')), Class(onto.iri('http://test-ob-target'))])))

print (f"Ontology now has {len(onto.get_axioms())} axioms.")

print (f"Getting all classes with charge and no smiles")
# Get all classes with asserted charge but no smiles
if FULL:
    classes_with_charge = set()
    classes_with_smiles = set()
    for a in onto.get_axioms():
        #Example: ['AnnotationAssertion', 'http://purl.obolibrary.org/obo/CHEBI_27732', 'http://www.geneontology.org/formats/oboInOwl#hasAlternativeId', 'CHEBI:41472']
        # if len(a)==4 and a[0]=='AnnotationAssertion':
        if isinstance(a.axiom, AnnotationAssertion):

            # if a[2]==CHARGE:
            if a.axiom.ann.ap == onto.iri(CHARGE):

                # chg = a[3]
                chg = a.axiom.ann.av
                # classes_with_charge.add(a[1])
                classes_with_charge.add(a.axiom.subject)

            # if a[2]==SMILES:
            if a.axiom.ann.ap == onto.iri(SMILES):

                # smls = a[3]
                smls = a.axiom.ann.av

                # classes_with_smiles.add(a[1])
                classes_with_smiles.add(a.axiom.subject)

    classes_with_charge_no_smiles = [c for c in classes_with_charge if c not in classes_with_smiles]
    print(f"The ontology has {len(classes_with_charge_no_smiles)} classes with charge and no smiles.")

# Get all the asserted parts:
if FULL:
    print("Getting all asserted part relations")
    asserted_parts = set() # A list of tuples
    for a in onto.get_axioms():
        # Example: ['SubClassOf', 'http://purl.obolibrary.org/obo/CHEBI_27732', ['ObjectSomeValuesFrom', 'http://purl.obolibrary.org/obo/RO_0000087', 'http://purl.obolibrary.org/obo/CHEBI_85234']]
        # if len(a)==3 and a[0]=='SubClassOf' and \
        #   isinstance(a[2], list) and len(a[2])==3 and \
        #   a[2][0]=='ObjectSomeValuesFrom' and a[2][1]==HAS_PART:
        if isinstance(a.axiom, SubClassOf) and isinstance(a.axiom.sup, ObjectSomeValuesFrom) \
            and a.axiom.sup.ope == onto.iri(HAS_PART):
            # asserted_parts.add((a[1],a[2][2]))
            asserted_parts.add((a.axiom.sub, a.axiom.sup.bce))


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






#Memory Profiler for all your memory needs.

#https://pypi.python.org/pypi/memory_profiler

#Run a pip install:

#pip install memory_profiler

#Import the library:

#import memory_profiler

#Add a decorator to the item you wish to profile:

#@profile
#def my_func():
#    a = [1] * (10 ** 6)
#    b = [2] * (2 * 10 ** 7)
#    del b
#    return a

#if __name__ == '__main__':
#    my_func()

#Execute the code:

#python -m memory_profiler example.py

#Recieve the output:

# Line #    Mem usage  Increment   Line Contents
# ==============================================
# 3                           @profile
# 4      5.97 MB    0.00 MB   def my_func():
# 5     13.61 MB    7.64 MB       a = [1] * (10 ** 6)
# 6    166.20 MB  152.59 MB       b = [2] * (2 * 10 ** 7)
# 7     13.61 MB -152.59 MB       del b
# 8     13.61 MB    0.00 MB       return a

#Examples are from the docs, linked above.
