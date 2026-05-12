.PHONY: all dev mdev model docs tests .venv

all: lib pyi

dev: mdev pyi

mdev: model docs .venv
	.venv/bin/maturin develop

pyi: .venv
	.venv/bin/python scripts/gen_pyi.py

docs: src/doc.rs

src/doc.rs: .venv
	.venv/bin/python scripts/build_doc.py

model: .venv
	.venv/bin/python scripts/build_model.py

lib: model docs
	.venv/bin/maturin build --release

.venv:
	uv venv
	uv sync --all-groups

tests: dev .venv
	uv sync
	PYO3_PYTHON="/usr/bin/python3" cargo test
	.venv/bin/pytest --doctest-modules test/