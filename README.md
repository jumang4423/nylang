![](_img/emojis.png)

# nylang

a cute language with a bunch emoji

# quick demo

- 1. ```git clone <this github repo>```

- 2. install cargo to excute rust compiler

- 3. run ```cargo run run demo/demo.nyl```

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

- 💨 to return: Return Expression
```
💨 "cat"
```

- 🏨 to make a new closure expression: Closure Expression
```
🏨 ( A , B ) { } ;
```

- 🍙 to make a variable and assign in environment: Void
```
🍙 cat = "cat" ;
```

- 🐶 to conditional check: If Expression

```
🐶 ( 👍 == 👍 ) { } 😱 { } ; 
```

this also returns something

```
🍙 a = 🐶 ( 👍 == 👍 ) { 💨 "ass" } ; 
```

- 🌸 to loop: Void

usage 1: with a loop number

```
// ident ( closure, number )

🌸 ( 
    🏨 ( ) { // function
        // statement
    } , 
    100 // loop number
)
```

usage 2: without loop number but the function returns boolean

```
// ident ( closure )

🍙 cnt = 0 ;

🌸 ( 
    🏨 ( ) { // function
        🍙 cnt = cnt + 1 ;
        💨 cnt != 100 // loop till the function returns false
    }
)
```

- ```🐽🐽🐽 ( "<filename>.nyl" ) ;``` to import namespaced functions to the file

```
🐽🐽🐽 ( ".nylang/demo/__test__.nyl" ) ;

__test__say_hello ( ) ; // imported!
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

- 🎤(🎶) to print out: Void

```
🎤🎶 ( "hoge" ) ; // prints hoge\n
```

- 👀 ( type T ) to get input: T

T can be ```"string", "number" and "boolean"```

```
🍙 _input = 👀 ( "number" ) ;

🎤🎶 ( _input * _input ) ;

```

- 😪 to sleep: Void

```
😪 ( 500 ) ; // wait 0.5s 
```

- 🌹 to return random emojis: String(Emojis)

```
🌹 ( 5 ) ; // random 5 emojis
```

there is 10 emojis available, which means that somehow we can generates random numbers using this function.