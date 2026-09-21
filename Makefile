.PHONY: all dev mdev model docs tests

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
	uv sync --all-groups --no-install-project

tests: dev
	uv sync --no-install-project
	PYO3_PYTHON="/usr/bin/python3" cargo test
	uv run pytest --doctest-modules test/
