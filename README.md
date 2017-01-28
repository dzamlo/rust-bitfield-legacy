rust-bitfield
=============

[![Build Status](https://travis-ci.org/dzamlo/rust-bitfield.svg?branch=master)](https://travis-ci.org/dzamlo/rust-bitfield)

This project provides a procedural macro to generate bitfield-like struct.

The generated structs use an array of u8 for the data and provide methods to
get and set the values of the fields.

The generatated structs are not compatible with C bitfield. Unlike in C, the
position of each bytes and bits in the underling bytes array is specifed. The
bytes are in network order, and the bits are MSB first. No padding is added.

Because the generated struct is just a normal struct, you can add other 
methods to it or implement the trait you want.

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

As the stable compiler (`rustc`) doesn't allow plugin for now, you have to use
[the nighlty toolchain](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust).

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
