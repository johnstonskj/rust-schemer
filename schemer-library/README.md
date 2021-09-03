# Crate schemer-library

A pedantic implementation of the core R7RS Scheme language library in Rust.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
[![crates.io](https://img.shields.io/crates/v/schemer_library.svg)](https://crates.io/crates/schemer_library)
[![docs.rs](https://docs.rs/schemer_library/badge.svg)](https://docs.rs/schemer_library)

-----

This crate provides the core library and extended forms not provided in the language crate. The aim is to achieve full 
R7RS compatibility in the library, and some SRFI.

* `(scheme ...)` -- the reserved set of libraries specified by R7RS.
* `(srfi ...)` -- the reserved namespace for SRFI implementations.
* `(schemer ...)`. -- a reserved namespace for locally defined extensions to the R7RS library.

# Example

TBD

-----

## Changes

**Version 0.1.0**

* Initial commit.
