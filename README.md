<!-- ![](_img/emojis.png) -->

# nylang

[![rust test action](https://github.com/jumang4423/nylang/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/jumang4423/nylang/actions/workflows/test.yml)

a cute language with a bunch emoji

# documentation

[WIKI](https://github.com/jumang4423/nylang/wiki)

# usage

## dependancies

- rust ( cargo )

## install & uninstall

> install 

```
chmod +x scripts/install.sh && ./scripts/install.sh
```

> uninstall

```
chmod +x scripts/uninstall.sh && ./scripts/uninstall.sh
```

## quick demo

```
cargo run --release run samples/demo.nyl
```
## excution

> show help

```
nylang
```

> make new project

u can edit hoge.nylsh
then excute with ./hoge.nylsh

```
nylang new <project name>
```

> run program

run hoge.nyl file

```
nylang run <filename>.nyl
```

> show tokens program

only run lexer

```
nylang lexer <filename>.nyl
```

> show tree

only show ast tree

```
nylang parser <filename>.nyl
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