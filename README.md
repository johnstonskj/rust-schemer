# (project schemer)

A pedantic implementation of the R7RS Scheme programming language in Rust.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.53-green.svg)
![Build](https://github.com/johnstonskj/rust-schemer/workflows/Rust/badge.svg)
![Audit](https://github.com/johnstonskj/rust-schemer/workflows/Security%20audit/badge.svg)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-schemer.svg)](https://github.com/johnstonskj/rust-schemer/stargazers)

-----

TBD.

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


# Example

TBD

-----

## TODO

TBD