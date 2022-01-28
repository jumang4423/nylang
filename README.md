<!-- ![](_img/emojis.png) -->

# nylang

[![rust test action](https://github.com/jumang4423/nylang/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/jumang4423/nylang/actions/workflows/test.yml)

a cute language with a bunch emoji

# documentation

[WIKI](https://github.com/jumang4423/nylang/wiki)

# install

## dependancies

- rust ( cargo )

## install & uninstall

> install 

```
cargo install --git https://github.com/jumang4423/nylang.git
```

> uninstall

```
cargo uninstall nylang && rm -rf ~/.nylang
```

## excute uisng docker

- to start the container

```
docker compose up -d
```

- entering shell

```
docker compose exec nylang bash
```