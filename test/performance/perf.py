import pyhornedowl
import time

from pyhornedowl.model import SubClassOf


def perf(path: str):
    print(f"===={path}====")
    print(f"Opening...")

    open_time= 0
    for i in range(3):
        start = time.perf_counter()
        o = pyhornedowl.open_ontology(path)
        open_time += time.perf_counter() - start

    open_time /= 3
    print(f"Opened in {open_time}s")

    print("Iterating...")
    amount = 0
    iter_time = 0
    for _ in range(3):
        start = time.perf_counter()
        for a in o.get_components():
            if len(a.ann) > 0:
                amount += 1
        iter_time += time.perf_counter() - start

    iter_time /= 3
    print(f"Iterated in {iter_time}s")

    print("Adding 100.000 axioms...")
    o = pyhornedowl.PyIndexedOntology()
    start = time.perf_counter()
    for i in range(100_000):
        o.add_axiom(SubClassOf(o.clazz(f"https://example.com/entity_{i}"), o.clazz(f"https://example.com/entity{i+1}")))
    add_time = time.perf_counter() - start
    print(f"Added 100.000 axioms in {add_time}s")


if __name__ == "__main__":
    perf("go.owl")
    perf("chebi_core.owl")