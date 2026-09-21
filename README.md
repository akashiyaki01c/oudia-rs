# oudia-rs

[![Crates.io](https://img.shields.io/crates/v/oudia-rs.svg)](https://crates.io/crates/oudia-rs)
[![Documentation](https://docs.rs/oudia-rs/badge.svg)](https://docs.rs/oudia-rs)
[![License](https://img.shields.io/crates/l/oudia-rs.svg)](https://github.com/akashiyaki01c/oudia-rs#license)
![](https://img.shields.io/badge/status-Alpha_version-red.svg)

oudia-rs is a Rust library for parsing and serializing OuDia files (`.oud`).

## Overview

OuDia is a railway timetable file format used in Japan, and this crate provides a Rust-friendly representation of OuDia data so that applications can read, inspect, and re-emit timetable information in a structured way.

This project currently focuses on the OuDia 1.02 format and exposes a simple API for deserializing and serializing the data model.

## Features

- Parse OuDia files in Rust
- Support the OuDia 1.02 format
- Convert OuDia data into Rust data structures
- Serialize Rust data back to OuDia text format
- Easy integration with Cargo-based Rust projects

## Current support status

At the moment, this library supports the OuDia 1.02 format.

Support for other OuDia versions and for the various OuDiaSecond formats is not yet implemented.

> This crate is currently in a pre-stable stage. Before a stable release, the public API may change in breaking ways, so please use it with that in mind when integrating it into production systems.

## Future plans

We plan to expand compatibility to additional OuDia versions and to OuDiaSecond formats in the future.

The library is designed with version-aware parsing in mind, so we can extend support while keeping the API consistent and maintainable as more formats are added.

## Installation

Add the crate to your project with Cargo:

```bash
cargo add oudia-rs
```

## Quick start

```rust
use oudia_rs::io::{deserialize_oudia, serialize_oudia};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read("test_data/keio.oud")?;
    let data = deserialize_oudia(&input)?;

    let output = serialize_oudia(&data);
    std::fs::write("output.oud", output)?;

    Ok(())
}
```

This reads an OuDia file, parses it into a Rust model, and writes it back to the OuDia text format.

## API

The crate exposes the main entry points for working with OuDia files:

- `deserialize_oudia(value: &[u8]) -> Result<OuDiaFile, Error>`
  - Parses a Shift-JIS encoded OuDia file into a Rust data model
- `serialize_oudia(data: &OuDiaFile) -> Vec<u8>`
  - Serializes the loaded model as Shift-JIS encoded OuDia bytes

The model layer is organized by format version and data type, and the current implementation targets the OuDia 1.02 specification.

## Use cases

This library is useful for:

- Railway timetable file analysis
- Data extraction and conversion
- Custom tooling around OuDia-based schedule data
- Building applications that need to read or emit OuDia files

## License

This project is licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
