#!/bin/bash

export RUSTFLAGS=-Zincremental-verify-ich

cp src/lib.rs old.rs
cargo clean
cargo check

git apply crash.patch
# Use python patch for further minimization.
# ./patch.py

OUT=$(cargo check 2>&1)

cp old.rs src/lib.rs

if echo $OUT | grep "internal compiler error";
then
    echo "The ICE reproduces"
    exit 0
else
    echo "No reproduction"
    exit 1
fi
