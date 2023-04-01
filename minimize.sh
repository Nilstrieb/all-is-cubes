#!/bin/bash

#export RUSTFLAGS=-Zincremental-verify-ich

cp all-is-cubes/src/mesh/chunked_mesh.rs premesh.rs
cargo clean -p all-is-cubes
cargo check -p all-is-cubes

./patch.py
mv out.rs all-is-cubes/src/mesh/chunked_mesh.rs

OUT=$(cargo check -p all-is-cubes 2>&1)

cp premesh.rs all-is-cubes/src/mesh/chunked_mesh.rs

if echo $OUT | grep "internal compiler error";
then
    echo "The ICE reproduces"
    exit 0
else
    echo "No reproduction"
    exit 1
fi
