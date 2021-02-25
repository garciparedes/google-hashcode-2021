#!/usr/bin/env bash

git commit -a

set -e
git archive --format zip --output solution.zip HEAD

cargo run < res/a.txt > a.out
cargo run < res/b.txt > b.out
cargo run < res/c.txt > c.out
cargo run < res/d.txt > d.out
cargo run < res/e.txt > e.out
cargo run < res/f.txt > f.out
