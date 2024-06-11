#!/usr/bin/env bash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )


robot convert -i "$SCRIPT_DIR/simple.omn" -o "$SCRIPT_DIR/simple.ofn"
robot convert -i "$SCRIPT_DIR/simple.omn" -o "$SCRIPT_DIR/simple.owx"
robot convert -i "$SCRIPT_DIR/simple.omn" -o "$SCRIPT_DIR/simple.owl"
ln -f "$SCRIPT_DIR/simple.omn" "$SCRIPT_DIR/simple.omn.raw"
ln -f "$SCRIPT_DIR/simple.ofn" "$SCRIPT_DIR/simple.ofn.raw"
ln -f "$SCRIPT_DIR/simple.owx" "$SCRIPT_DIR/simple.owx.raw"
ln -f "$SCRIPT_DIR/simple.owl" "$SCRIPT_DIR/simple.owl.raw"
