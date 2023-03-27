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