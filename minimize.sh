#!/bin/bash

cp premesh.rs all-is-cubes/src/mesh/chunked_mesh.rs
cargo clean -p all-is-cubes
cargo check -p all-is-cubes
git apply crash.patch

OUT=$(cargo check -p all-is-cubes 2>&1)

if echo $OUT | grep "internal compiler error";
then
    echo "The ICE reproduces"
    exit 0
else
    echo "No reproduction"
    exit 1
fi
