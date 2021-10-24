# nylang

![](_img/src.png)

a cute programming language inspired by monkey language in rust

# install & uninstall

```
chmod +x scripts/install.sh && ./scripts/install.sh
```

```
chmod +x scripts/uninstall.sh && ./scripts/uninstall.sh
```

# excution

- show help

```
nylang
```

- run program

```
nylang run <filename>.nyl
```

# nylang documentation

## reserved word

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

- 🌸 to loop
```
🌸 ( 
    🏨 ( ) { // function
        // statement
    } , 
    100 // loop number
)
```

- 👍 true, 👎 false

```
bark ( 👍 ) ;
```

# types

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

## builtin functions

- bark(ln) to print out

```
barkln ( "hoge" ) ; // prints hoge\n
```

- 😪 to sleep

```
😪 ( 500 ) ; // wait 0.5s 
```