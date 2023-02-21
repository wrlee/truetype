# TrueType Metadata

This program reads the metadata from a font file (Truetype, OpenType, etc.) and prints information about the font's name, version, etc.

> This program was written as an exercise in learning Rust and uses the `Font` struct from the `fonttools` crate to navigate through the file data.

## Build and Run

- `cargo build` [`--release`] or
- `cargo run`  [`--release`] {_filename_}

## TODOs
- Attempt to "Fix" "Deserialization error Font must begin with a valid version" for font files with b'true' rather than 0x00000001
- Add option switches to control what to output
  - `-r` raw output -- all data, no parsing
  - `-t` comma separated list of names of specific tables to output (e.g., name, hhea, cmap, etc.)
  - `-v` _verbose_ to include all data
  - `--color` to colorize output
- Support for TrueType Collection (.ttc) files

## History
First drop.
- Parses `head` and `name` tables