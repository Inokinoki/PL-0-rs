# A naive PL/0 interpreter in Rust

This project is to make me more familiar with Rust. It works right now, but maybe doesn't work well in some cases.

Welcome to any contribution, if you can READ (yes, it's not even readable) the project.

## PL/0 Introduction

Wait, what is it, the PL/0? From Wikipedia:

    PL/0 is a programming language, intended as an educational programming language, that is similar to but much simpler than Pascal, a general-purpose programming language.

It's EBNF representation is like:

```ebnf
program = block "." .

block = [ "const" ident "=" number {"," ident "=" number} ";"]
        [ "var" ident {"," ident} ";"]
        { "procedure" ident ";" block ";" } statement .

statement = [ ident ":=" expression | "call" ident 
              | "?" ident | "!" expression 
              | "begin" statement {";" statement } "end" 
              | "if" condition "then" statement 
              | "while" condition "do" statement ].

condition = "odd" expression |
            expression ("="|"#"|"<"|"<="|">"|">=") expression .

expression = [ "+"|"-"] term { ("+"|"-") term}.

term = factor {("*"|"/") factor}.

factor = ident | number | "(" expression ")".
```

## Functionality

There is a simple Virtual Machine(vm) in `vm.rs` to execute the "code" generated.

And a parser in `compile` to parse and generate vm code.

The lexer using in this project is [Logos](https://github.com/maciejhirsz/logos).

## Build

```
cargo build
```

## Run

```
cargo run <pl/0-file-path>
```
The output will be there.

Up to now, ~~only the `sample1.pl0` and `sample5.pl0` can be correctly executed and give the right output (WHAT A PITY).~~ all samples can be run correctly (Congratulation) !!!

## Test

There are some tests(unit test/integration test) in the project.

You can run them with

```
cargo test
```

~~Some of them might be broken by my latest commits. So, please be patient.~~ All of them should be passed :)

## Many TODOs

- Error handle (some)
- ~~Fix While loop parsing (not work yet with `sample/sample0.pl0`)~~ Done
- List code, ~~generate~~ export symbol table
- VM code to native binary (maybe?)
- ...
