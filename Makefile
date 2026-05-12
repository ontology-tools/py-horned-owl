.PHONY: all dev mdev model docs tests .venv

all: lib pyi

dev: mdev pyi

mdev: model docs
	uv run --with maturin maturin develop

pyi: .venv
	.venv/bin/python scripts/gen_pyi.py

docs: src/doc.rs

src/doc.rs: .venv
	.venv/bin/python scripts/build_doc.py

model: .venv
	.venv/bin/python scripts/build_model.py

lib: model docs
	uvx maturin build --release

.venv:
	uv venv
	uv sync --all-groups

tests: dev
	uv sync
	PYO3_PYTHON="/usr/bin/python3" cargo test
	uv run pytest --doctest-modules test/
