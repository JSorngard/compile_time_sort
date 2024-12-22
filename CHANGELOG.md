# Changelog

This file contains all changes to the crate since version v0.1.0.

## 0.2.5

- Add some information about the structure of the crate to the README and docs.

## 0.2.4

- Fixed the panic bug also in the sorting functions of `bool`, `i8`, and `u8` arrays and slices.

## v0.2.3

- Fixed a bug where the sorting functions would panic if they were given an empty array or slice.

## v0.2.2

- Added sorting functions for `u128` and `i128`.

## v0.2.1

- Added a gihub actions workflow status badge to the README and corrected the link to the docs in Cargo.toml.

## v0.2.0

- Added the `sort_slices` feature and gated the functions that utilize mutable references behind it.
 This lowers the default MSRV of the crate to 1.59.0.

## v0.1.2

- Added badges to `README.md`.

## v0.1.1

- Corrected name of by-value function in `README.md`.
