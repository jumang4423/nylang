# nylang

![](_img/src.png)

a cute programming language inspired by monkey language in rust

## install & uninstall

```
chmod +x scripts/install.sh && ./scripts/install.sh
```

```
chmod +x scripts/uninstall.sh && ./scripts/uninstall.sh
```

## excution

- show help

```
nylang
```

- run program

```
nylang run <filename>.nyl
```

## nylang documentation

### reserved word

- dog to conditional check: If Expression
```
🐶 ( 👍 == 👍 ) { } 😱 { } ; 
```

- fart to return: Return Expression
```
💨 "cat"
```

- hotel to make a new closure expression: Closure Expression
```
🏨 ( A , B ) { } ;
```

- 🍙 to make a new object in environment memory: Void
```
🍙 cat = "cat" ;
```

- 👍 true, 👎 false

```
bark ( 👍 ) ;
```

- bark to print out

```
bark ( "hoge hoge" ) ;
```

### types

- number

```
123456789
```

- boolean

```
👍 👎
```

- string
```
"hoge"
```
