# Changelog

This file contains all changes to the crate since version v0.1.0.

## [1.5.1] - 2026-04-17

- Update `rand` to version 0.8.6.

## [1.5.0] - 2026-04-13

- Update dev-dependencies.
- Raise the MSRV to 1.56.0.
- Use Rust edition 2021.

## [1.4.1] - 2026-04-11

- Documentation improvements.

## [1.4.0] - 2026-04-11

- Add sorting of slices and arrays of slices of booleans behind the `nested` feature.

## [1.3.0] - 2026-04-11

- Add functions that sort slices and arrays of byte slices and strings.
- Add functions that sort slices and arrays of slices of primitives behind the `nested` feature.

## [1.2.4] - 2026-04-06

- Update crate docs.

## [1.2.3] - 2026-04-06

- Use introsort instead of quicksort.
- Set explicit permissions for the CI jobs.

## [1.2.2] - 2026-01-30

- Inline small crate-internal functions.
- Correct copyright year information.
- Allow the `clippy::inconsistent-msrv` lint as it can not function correctly due to the way this crate uses macros and the `rustversion` crate. MSRV issues are still caught in CI due to the use of the `cargo-msrv` add-on.
- Add dates to the release entries in this log.

## [1.2.1] - 2025-08-28

- Corrected the link to the RFC for `const` traits.

## [1.2.0] - 2025-08-28

- Enable the sorting of floating point values in accordance with the IEEE 754 totalOrder predicate on Rust versions 1.83.0 and newer.
- Change the insertion sort threshold to 8,
 the closest power of two to the previous value of 10.
- Use caching to speed up CI.

## [1.1.4] - 2025-04-30

- Add a crate level `#![forbid(unsafe_code)]` attribute.
- Don't publish unneeded files to crates.io.

## [1.1.3] - 2025-04-13

- Correct MSRV information in the crate docs.

## [1.1.2] - 2025-04-06

- Show the import of the crate in the doc examples.

## [1.1.1] - 2025-04-06

- Correct MSRV information in the readme.

## [1.1.0] - 2025-04-06

- Correct the code examples in the readme.
- Run the test CI job on multiple operating systems.
- Other minor CI improvements.
- Lower the MSRV of the crate to 1.54.0 by using rust edition 2018.

## [1.0.9] - 2025-03-23

- Internal improvements to sort impls.
- Document switch to insertion sort.

## [1.0.8] - 2025-03-23

- Use insertion sort when the collections are small.
 Except for `bool`s, in that case we always use counting sort.

## [1.0.7] - 2025-03-10

- Clearer definition of the docstring in the `sort_*_slice` functions.

## [1.0.6] - 2025-03-07

- Internal improvements to macro clarity.

## [1.0.5] - 2025-03-06

- Add tests of the functions that sort `usize` and `isize` arrays.
- Use the `paste` crate in the internal macros that define the tests
 such that invoking them is simpler.

## [1.0.4] - 2025-03-06

- Corrected the links to licenses in the readme.

## [1.0.3] - 2025-03-06

- Added doc examples to all functions.

## [1.0.2] - 2025-03-06

- Depend on the `paste` crate to simplify the internal macro definitions and invocations.

## [1.0.1] - 2025-03-05

- Removed unused code that was used to document the now non-existing feature on docs.rs.

## [1.0.0] - 2025-03-05

- 🎉 Stabilized the crate's API.
- Re-added the "no-std" category.

### Breaking changes

- Removed the `sort_slices` feature.
 Those functions are now activated automatically on Rust versions 1.83.0 and later by using
 the [`rustversion`](https://crates.io/crates/rustversion) crate.

## [0.2.9] - 2025-03-01

- Fixed a bug that could lead to improper sorting of slices in some cases.
- Added CI job to test the crate on the Rust beta branch.
- Added CI job to verify sever compatibility.
- Added CI job to test the crate on no_std targets.
- Removed the "no_std" category from the crate since it is already in
 "no-std::no-alloc", which is a subset of "no-std".

## [0.2.8] - 2025-01-11

- Noted the original MSRV in feature descriptions.
- Improvements to CI jobs.
- Improvements to the documentation.

## [0.2.7] - 2025-01-03

- Added the "sorting" keyword.

## [0.2.6] - 2025-01-03

- Made the crate `no_std` compatible.
- Added the "no-std" and "no-alloc" categories to the crate.

## [0.2.5] - 2024-12-22

- Added some information about the structure of the crate to the README and docs.

## [0.2.4] - 2024-12-18

- Fixed the panic bug also in the sorting functions of `bool`, `i8`, and `u8` arrays and slices.

## [0.2.3] - 2024-12-18

- Fixed a bug where the sorting functions would panic if they were given an empty array or slice.

## [0.2.2] - 2024-12-18

- Added sorting functions for `u128` and `i128`.

## [0.2.1] - 2024-12-18

- Added a gihub actions workflow status badge to the README and corrected the link to the docs in Cargo.toml.

## [0.2.0] - 2024-12-18

- Added the `sort_slices` feature and gated the functions that utilize mutable references behind it.
 This lowers the default MSRV of the crate to 1.59.0.

## [0.1.2] - 2024-12-16

- Added badges to `README.md`.

## [0.1.1] - 2024-12-16

- Corrected name of by-value function in `README.md`.

## [0.1.0] - 2024-12-16

First release 🎉

This is a small crate I used in another personal project that I have expanded so that it might find some use to others!
The ideal would of course be if the standard library's sorting functions were `const`, but until then, here are a collection of functions to sort arrays and slices in `const` contexts.

It uses counting sort for types with few valid states and quicksort for all other types.
