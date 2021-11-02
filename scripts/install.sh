#!/bin/bash
cargo build --release
# make directory
mkdir -p ~/.nylang
chmod +x ~/.nylang
# move the binary
cp target/release/nylang ~/.nylang
cp -r lib/ ~/.nylang/lib
rm target/release/nylang
chmod +x ~/.nylang/nylang
# bash PATH
echo "export PATH="~/.nylang"":'"$PATH"' >>~/.profile
source ~/.profile
# fish PATH
mkdir -p ~/.config/fish/conf.d
touch ~/.config/fish/conf.d/nylang.fish
echo "set PATH ~/.nylang" : '"$PATH"' >> ~/.config/fish/conf.d/nylang.fish
source ~/.config/fish/conf.d/nylang.fish