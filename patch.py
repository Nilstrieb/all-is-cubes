#!/usr/bin/env python3

with open("src/lib.rs") as fold:
    old = fold.read()

start = old.find("/////// MOVE START")
end = old.find("/////// MOVE END")
early = 0

very_pre = old[:early]
center = old[early:start]
post = old[end:]

result = f"{very_pre}\
{center}\
{post}"

with open("src/lib.rs", "w") as fout:
    fout.write(result)