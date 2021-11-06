#!/bin/bash
cargo install --path .
# make directory
mkdir -p ~/.nylang
chmod +x ~/.nylang
# move the binary
cp -r lib/ ~/.nylang/lib