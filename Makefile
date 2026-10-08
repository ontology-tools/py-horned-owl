.PHONY: all dev mdev model docs tests

all: lib

dev: mdev

mdev: model docs
	uv run --with 'maturin>=1.15' maturin develop --generate-stubs

docs: src/doc.rs

src/doc.rs: .venv
	.venv/bin/python scripts/build_doc.py

model: .venv
	.venv/bin/python scripts/build_model.py

lib: model docs
	uvx --from 'maturin>=1.15' maturin build --release --generate-stubs

.venv:
	uv venv
	uv sync --all-groups --no-install-project

tests: dev
	uv sync --no-install-project
	PYO3_PYTHON="/usr/bin/python3" cargo test
	uv run pytest --doctest-modules test/
