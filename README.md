# (project schemer)

A pedagogical implementation of the [R7RS](https://small.r7rs.org/) 
[Scheme](https://en.wikipedia.org/wiki/Scheme_(programming_language)) programming language in 
[Rust](https://www.rust-lang.org/).

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.53-green.svg)
![Build](https://github.com/johnstonskj/rust-schemer/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-schemer/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-schemer.svg)](https://github.com/johnstonskj/rust-schemer/stargazers)

-----

First, it will be important to answer some basic questions.

Q1. Does the world need another [Scheme implementation](https://en.wikipedia.org/wiki/Category:Scheme_(programming_language)_implementations)?

Probably not. Given some really fine implementations including [MIT Scheme](https://www.gnu.org/software/mit-scheme/),
[Gauche](http://practical-scheme.net/gauche/), [chez](https://www.scheme.com), and [Racket](https://racket-lang.org/)
-- and I really enjoy Racket.

Q2. Does the world need another Scheme [implementation in Rust](https://crates.io/search?q=scheme%20language)?

Maybe not, although the attempt to build a full-function repl based on a compliant language seems to be less common.

Q3. Why R7RS?

Because a lot of work went into it, it's a good base, and allows for portability. Although right now it's just the
published R7RS small, will keep watching the work on [_large_](http://www.scheme-reports.org/).

Q4. Does the author really need to build a Scheme implementation?

Certainly not, I have a lot of other things I could be doing, and quite a few I **should** be. But it's fun. This leads
to one final question why _pedagogical_? Because the intent is that the implementation will value readability, 
structure, and extension over performance. I want to learn from building it, and I want to pick it up one day in the 
future and learn from reading it.

# Structure

* Crate [schemer-lang](schemer-lang/README.md) -- the core language model and includes data types, environments, forms
  and expressions. This does not include the ability to import or load external libraries as that would require the 
  parser crate.
* Crate [schemer-parse](schemer-parse/README.md) -- the parser for Scheme, currently written using 
  [pest](https://pest.rs/). This crate depends on the language crate only.
* Crate [schemer-library](schemer-library/README.md) -- the core library and extended forms not provided in the language
  crate. The aim is to achieve full R7RS compatibility in the library, and some SRFI.
  * `(scheme ...)` -- the reserved set of libraries specified by R7RS.
  * `(srfi ...)` -- the reserved namespace for SRFI implementations.
  * `(schemer ...)`. -- a reserved namespace for locally defined extensions to the R7RS library.
* Crate [schemer-repl](schemer-repl/README.md) -- the shell for executing both scripts and interactive sessions.

The crate [schemer-macros](schemer-macros/) is only used internally at this time, it is not published.

# Example - REPL

```bash
$ schemer-repl
Welcome to schemer, v0.1.0.
> (import (schemer repl))
#!unspecified
> (print-current-environment)
┌╴ **repl**
│  ('help . #<builtin-procedure:help:1..1>)
│  ('inspect . #<builtin-procedure:inspect:1..1>)
│  ('print-current-environment . #<builtin-procedure:print-current-environment:0..0>)
│  ('schemer-repl-history-file . "schemer-history.txt")
│  ('schemer-repl-init-file . "/Users/simonjo/repl-init.sr")
│  ┌╴ **scheme-base**
│  │  ('boolean? . #<builtin-procedure:boolean?:1..1>)
│  │  ('bytevector? . #<builtin-procedure:bytevector?:1..1>)
│  │  ('char? . #<builtin-procedure:char?:1..1>)
│  │  ('complex? . #<builtin-procedure:complex?:1..1>)
│  │  ('current_error_port . #<builtin-procedure:current_error_port:0..0>)
│  │  ('current_input_port . #<builtin-procedure:current_input_port:0..0>)
│  │  ('current_output_port . #<builtin-procedure:current_output_port:0..0>)
│  │  ('even? . #<builtin-procedure:even?:1..1>)
│  │  ('exact-integer? . #<builtin-procedure:exact-integer?:1..1>)
│  │  ('exact? . #<builtin-procedure:exact?:1..1>)
│  │  ('features . #<builtin-procedure:features:0..0>)
│  │  ('flush-output-port . #<builtin-procedure:flush-output-port:1..*>)
│  │  ('import . #<standard-form:import:1..1>)
│  │  ('inexact? . #<builtin-procedure:inexact?:1..1>)
│  │  ('integer? . #<builtin-procedure:integer?:1..1>)
│  │  ('list? . #<builtin-procedure:list?:1..1>)
│  │  ('negative? . #<builtin-procedure:negative?:1..1>)
│  │  ('newline . #<builtin-procedure:newline:1..1>)
│  │  ('null? . #<builtin-procedure:null?:1..1>)
│  │  ('number? . #<builtin-procedure:number?:1..1>)
│  │  ('odd? . #<builtin-procedure:odd?:1..1>)
│  │  ('positive? . #<builtin-procedure:positive?:1..1>)
│  │  ('procedure? . #<builtin-procedure:procedure?:1..1>)
│  │  ('rational? . #<builtin-procedure:rational?:1..1>)
│  │  ('real? . #<builtin-procedure:real?:1..1>)
│  │  ('string-length . #<builtin-procedure:string-length:1..1>)
│  │  ('string? . #<builtin-procedure:string?:1..1>)
│  │  ('symbol? . #<builtin-procedure:symbol?:1..1>)
│  │  ('vector? . #<builtin-procedure:vector?:1..1>)
│  │  ('write-bytevector . #<builtin-procedure:write-bytevector:1..*>)
│  │  ('write-char . #<builtin-procedure:write-char:1..*>)
│  │  ('write-string . #<builtin-procedure:write-string:1..*>)
│  │  ('write-u8 . #<builtin-procedure:write-u8:1..*>)
│  │  ('zero? . #<builtin-procedure:zero?:1..1>)
│  │  ┌╴ *top*
│  │  │  ('begin . #<standard-form:begin:0..*>)
│  │  │  ('define . #<standard-form:define:2..*>)
│  │  │  ('if . #<standard-form:if:2..*>)
│  │  │  ('lambda . #<standard-form:lambda:1..*>)
│  │  │  ('quote . #<standard-form:quote:1..1>)
│  │  └╴ ('set! . #<standard-form:set!:2..2>)
│  └╴
└╴
#t
> ^D
Goodbye
```

-----

## TODO

TBD