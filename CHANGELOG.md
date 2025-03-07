# Changelog

This file contains all changes to the crate since version v0.1.0.

## 1.0.6 (unreleased)

- Internal improvements to macro clarity.

## 1.0.5

- Add tests of the functions that sort `usize` and `isize` arrays.
- Use the `paste` crate in the internal macros that define the tests
 such that invoking them is simpler.

## 1.0.4

- Corrected the links to licenses in the readme.

## 1.0.3

- Added doc examples to all functions.

## 1.0.2

- Depend on the `paste` crate to simplify the internal macro definitions and invocations.

## 1.0.1

- Removed unused code that was used to document the now non-existing feature on docs.rs.

## 1.0.0

- Stabilized the crate's API.
- Re-added the "no-std" category.

### Breaking changes

- Removed the `sort_slices` feature.
 Those functions are now activated automatically on Rust versions 1.83.0 and later by using
 the [`rustversion`](https://crates.io/crates/rustversion) crate.

## 0.2.9

- Fixed a bug that could lead to improper sorting of slices in some cases.
- Added CI job to test the crate on the Rust beta branch.
- Added CI job to verify sever compatibility.
- Added CI job to test the crate on no_std targets.
- Removed the "no_std" category from the crate since it is already in
 "no-std::no-alloc", which is a subset of "no-std".

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
