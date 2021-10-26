FROM --platform=arm64 rustlang/rust:nightly
# What distro is this image based on?
COPY . /nylang
# It is not necessary do that,
# just making a copy is enough
WORKDIR /nylang