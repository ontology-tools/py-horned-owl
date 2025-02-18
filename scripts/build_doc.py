# Extracts the documentation from horned owl to attach it to the python types

import json
import os
import subprocess
from os import environ

REPO_ROOT=os.path.join(os.path.dirname(__file__), "..")
# path to local horned owl copy
HORNED_OWL_PATH = environ.get("HORNED_OWL_REPO_PATH", os.path.join(REPO_ROOT, "..", "horned-owl"))

subprocess.call("cargo +nightly rustdoc -Z unstable-options --output-format json".split(" "), cwd=HORNED_OWL_PATH)

with open(os.path.join(HORNED_OWL_PATH, "target", "doc", "horned_owl.json")) as f:
    data = json.load(f)

docs = dict([(x['name'], x['docs']) for x in data["index"].values() if
             x['span'] is not None and x['span']['filename'] == "src/model.rs" and
             x['name'] is not None and
             x['docs'] is not None and
             ('struct' in x['inner'] or 'enum' in x['inner'] or 'variant' in x['inner'])])

lines = [f'    ({n}) => {{ {json.dumps(d)} }};' for n, d in docs.items()]
lines.sort()
lines.append("    ($t:ident) => {\"\"}")

out_path = os.path.join(REPO_ROOT, "src", "doc.rs")
with open(out_path, "w") as f:
    f.write('macro_rules! doc (\n' + "\n".join(lines) + '\n);\n')
