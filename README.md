```
export RUSTFLAGS=-Zincremental-verify-ich
```

```
cargo clean -p all-is-cubes 
cargo build -p all-is-cubes
git apply crash.patch
cargo build -p all-is-cubes
# ICE!
```

```
cp premesh.rs all-is-cubes/src/mesh/chunked_mesh.rs
# Restored!
```

```
 cargo minimize --script-path=./minimize.sh --script-path-lints=./minimize-lints.sh --ignore-file=all-is-cubes/src/mesh/chunked_mesh --passes=FILL_OUT_HERE ./all-is-cubes/src
```
