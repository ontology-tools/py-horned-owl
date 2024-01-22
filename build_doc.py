import json

with open("hornedowldoc/model.json") as f:
    data = json.load(f)

docs = dict([(x['name'], x['docs']) for x in data["index"].values() if 
             x['name'] is not None and 
             x['docs'] is not None and 
             ('struct' in x['inner'] or 'enum' in x['inner'] or 'variant' in x['inner'])])

lines = [ f'    ({n}) => {{ {json.dumps(d)} }};' for n, d in docs.items()]
lines.append("    ($t:ident) => {\"\"}")
         
with open("src/doc.rs", "w") as f:
    f.write('macro_rules! doc (\n'+ "\n".join(lines) +'\n);\n')