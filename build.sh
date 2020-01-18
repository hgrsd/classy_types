#!/usr/bin/env sh
cargo build --release
mv target/release/classy_types ./
rm -rf target
