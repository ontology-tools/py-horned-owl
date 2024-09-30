#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

FILES=("simple" "prefix")

EMPTY_PREFIX="https://example.com/"

for f in "${FILES[@]}"; do
  robot convert -i "$SCRIPT_DIR/$f.omn" -o "$SCRIPT_DIR/$f.ofn"
  sed -i "1iPrefix(:=<$EMPTY_PREFIX>)" "$SCRIPT_DIR/$f.ofn"

  robot convert -i "$SCRIPT_DIR/$f.omn" -o "$SCRIPT_DIR/$f.owx"
  sed -i "10i    <Prefix name=\"\" IRI=\"$EMPTY_PREFIX\"/>" "$SCRIPT_DIR/$f.owx"

  robot convert -i "$SCRIPT_DIR/$f.omn" -o "$SCRIPT_DIR/$f.owl"
  # no empty prefix in rdf/xml?

  ln -f "$SCRIPT_DIR/$f.omn" "$SCRIPT_DIR/$f.omn.raw"
  ln -f "$SCRIPT_DIR/$f.ofn" "$SCRIPT_DIR/$f.ofn.raw"
  ln -f "$SCRIPT_DIR/$f.owx" "$SCRIPT_DIR/$f.owx.raw"
  ln -f "$SCRIPT_DIR/$f.owl" "$SCRIPT_DIR/$f.owl.raw"
done



