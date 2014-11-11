rust-bitfield
=============

[![Build Status](https://travis-ci.org/dzamlo/rust-bitfield.svg?branch=master)](https://travis-ci.org/dzamlo/rust-bitfield)

This project provides a procedural macro to generate bitfield-like struct.

The generated structs use an array of u8 for the data and provide methods to
get and set the values of the fields.

The generatated structs are not compatible with C bitfield. Unlike in C, the
position of each bytes and bits in the underling bytes array is specifed. The
bytes are in network order, and the bits are MSB first. No padding is added.

Possible use includes decoding some binary file formats and reading the
headers of some network protocols or some low-level protocols.

Usage
-----

To use this macro in a cargo enabled project, add the following to your 
Cargo.toml:
```toml
[dependencies.bitfield]
git = "https://github.com/dzamlo/rust-bitfield"
```

See the [examples folder](examples) for examples of the use the macro.
