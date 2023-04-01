#!/usr/bin/env python3

with open("all-is-cubes/src/mesh/chunked_mesh.rs") as fold:
    old = fold.read()

start = old.find("/////// MOVE START")
end = old.find("/////// MOVE END")
early = old.find("/// If true, enables reporting")

very_pre = old[:early]
center = old[early:start]
post = old[end:]

result = f"{very_pre}\
mod blocks {{}}\n\
mod chunk;\n\
pub use chunk::*;\n\
{center}\
{post}"

with open("out.rs", "w") as fout:
    fout.write(result)