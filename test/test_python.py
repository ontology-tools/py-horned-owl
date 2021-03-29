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


# Get all classes with asserted charge but no smiles
# THIS IS SLOW :-(
#if FULL:
#    clsses_with_charge_no_smiles = []
#    for c in [c for c in onto.get_classes() if len(onto.get_subclasses(c))>1]: # Only if you have multiple subclasses
#        chg = onto.get_annotation(c,CHARGE)
#        smls = onto.get_annotation(c,SMILES)
#        if chg and not smls:
#            clsses_with_charge_no_smiles.append(c)

#    print(f"The ontology has {len(clsses_with_charge_no_smiles)} classes with multiple subclasses, charge and no smiles.")



#print(f"Object property {HAS_ROLE} has label {onto.get_annotation(HAS_ROLE,RDFSLABEL)}")
#print(f"Object property 'has part' has iri {onto.get_iri_for_label('has part')}" )

# Get all the classes with asserted parts:
#clsses_with_parts = []
#for c in onto.get_classes():
#    axs = onto.get_axioms_for_iri(c)
#    for ax in axs:
#        if ax.index("SubClassOf")==0



# Works, but is slow!
#onto.save_to_file("chebi_updated.owx")


#print("Sleeping")
#time.sleep(5)

print("Axioms for IRI ",clssid,": ",onto.get_axioms_for_iri(clssid))


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

#if FULL:
#    onto.save_to_file("test/chebi-updated.owx")

print("Quitting")
