# compile_time_sort

[![Crates.io Version](https://img.shields.io/crates/v/compile_time_sort?logo=rust)](https://crates.io/crates/compile_time_sort)
[![Docs.rs Documentation](https://img.shields.io/docsrs/compile_time_sort?logo=docs.rs)](https://docs.rs/compile_time_sort/latest/compile_time_sort/)
[![Github Repository Link](https://img.shields.io/badge/github-JSorngard%2Fcompile__time__sort-8da0cb?logo=github)](https://github.com/JSorngard/compile_time_sort)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/compile_time_sort/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/compile_time_sort/actions/workflows/rust.yml)

This crate provides functions for sorting arrays and slices of primitives in `const` contexts.

Arrays and slices of `bool`s, `u8`s, and `i8`s are sorted with counting sort while arrays of other types
are sorted with quicksort.

This implementation is usable on stable before the [`const_fn_in_trait`](https://github.com/rust-lang/rust/issues/67792) feature is stabilized,
but that means that it unfortunately can not be generic,
and so there are separate functions for every primitive type.

Functions with the naming convention `into_sorted_*_array` take an array by value,
and functions with the naming convention `sort_*_slice` take a mutable reference to a slice.

## Examples

Sort an array by value:

```rust
use const_sort::into_sorted_i32_array;

const ARRAY: [i32; 5] = [-3, 3, 2, i32::MAX, 0];
const SORTED_ARRAY: [i32; 5] = into_sorted_i32_array(ARRAY);

assert_eq!(SORTED_ARRAY, [-3, 0, 2, 3, i32::MAX]);
```

Sort an array by reference:

```rust
use const_sort::sort_i32_slice;

const SORTED_ARRAY: [i32; 5] = {
    let mut arr = [5, i32::MIN, 0, -2, 0];
    sort_i32_slice(&mut arr);
    arr
};

assert_eq!(SORTED_ARRAY, [i32::MIN, -2, 0, 0, 5]);
```

## Features

`sort_slices`: enables the `sort_*_slice` functions and raises the MSRV of the crate from 1.59.0 to 1.83.0.

<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
