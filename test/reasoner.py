import os.path

import pyhornedowl
from pyhornedowl.model import *
from pyhornedowl.reasoner import Whelk

base = os.path.abspath(os.path.dirname(__file__))

o = pyhornedowl.open_ontology(os.path.join(base, "skeletons-asserted.owx"))

reasoner = Whelk(o)

reasoner.classify()

inferred_axioms = reasoner.inferred_axioms()
print(f"{len(inferred_axioms)} axioms could be inferred.")

print(f"The ontology is {'' if reasoner.consistent() else 'not '}consistent.")

sups = reasoner.get_superclasses("http://www.example.org/skeleton#finger_skeleton")
print(f"#finger_skeleton has {len(sups)} direct or indirect superclasses.")

reasoner.get_subclasses("http://www.example.org/skeleton#finger_skeleton")
reasoner.get_subclasses("http://www.w3.org/2002/07/owl#Thing")
