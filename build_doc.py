# Extracts the documentation from horned owl to attach it to the python types

import json
import os
import subprocess

# path to local horned owl copy
HORNED_OWL_PATH = "../horned-owl"

subprocess.call("cargo rustdoc -Z unstable-options --output-format json".split(" "), cwd=HORNED_OWL_PATH)

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

with open("src/doc.rs", "w") as f:
    f.write('macro_rules! doc (\n' + "\n".join(lines) + '\n);\n')
