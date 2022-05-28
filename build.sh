#!/bin/bash
set -e

RUSTFLAG='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p out
cp target/wasm32-unknown-unknown/release/*.wasm out/stacking-constract.wasm
