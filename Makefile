all: lib pyi

dev: mdev pyi

mdev: model docs
	venv/bin/maturin develop

pyi: venv
	venv/bin/python3 scripts/gen_pyi.py

docs: src/doc.rs

#src/doc.rs: venv
#	venv/bin/python3 scripts/build_doc.py

model: venv
	venv/bin/python3 scripts/build_model.py

lib: model docs
	venv/bin/maturin build --release

venv:
	python3 -m venv venv
	venv/bin/pip3 install -r requirements.txt
