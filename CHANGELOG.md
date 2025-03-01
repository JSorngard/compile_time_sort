# Changelog

This file contains all changes to the crate since version v0.1.0.

## 0.2.9

- Fixed a bug that could lead to improper sorting of slices in some cases.
- Added CI job to test the crate on the Rust beta branch.
- Added CI job to verify sever compatibility.
- Added CI job to test the crate on no_std targets.

## 0.2.8

- Noted the original MSRV in feature descriptions.
- Improvements to CI jobs.
- Improvements to the documentation.

## 0.2.7

- Added the "sorting" keyword.

## 0.2.6

- Made the crate `no_std` compatible.
- Added the "no-std" and "no-alloc" categories to the crate.

## 0.2.5

- Added some information about the structure of the crate to the README and docs.

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
