# mini-scheme
Interpreter of Scheme subset written in Rust

## Usage
### Build
`cargo build --release`

### Run
`./target/release/mini-scheme`

with files:

`./target/release/mini-scheme -f foo.scm -f bar.scm`

## Syntax, functions
```
define, load, lambda, quote, set!, let, let*, letrec, if, cond, and, or, begin, do
```
```
number?, +, -, *, /, =, <, <=, >, >=
```
```
boolean?, not
```
```
null?, pair?, list?, car, cdr, cons, list, length, memq, last, append, set-car!, set-cdr!
```
```
string?, string-append, symbol->string, string->symbol, string->number, number->string
```
```
symbol?
```
```
procedure?
```
```
eq?, equal?, neq?
```
```
display
```

## Feature
- circular list


