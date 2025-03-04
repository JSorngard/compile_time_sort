# compile_time_sort

[![Crates.io Version](https://img.shields.io/crates/v/compile_time_sort?logo=rust)](https://crates.io/crates/compile_time_sort)
[![Docs.rs Documentation](https://img.shields.io/badge/docs.rs-compile__time__sort-66c2a5?logo=docs.rs)](https://docs.rs/compile_time_sort/latest/compile_time_sort/)
[![Github Repository Link](https://img.shields.io/badge/github-JSorngard%2Fcompile__time__sort-8da0cb?logo=github)](https://github.com/JSorngard/compile_time_sort)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/compile_time_sort/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/compile_time_sort/actions/workflows/rust.yml)
[![Code Coverage](https://codecov.io/gh/JSorngard/compile_time_sort/graph/badge.svg?token=F61FO63ZKW)](https://codecov.io/gh/JSorngard/compile_time_sort)

This small crate provides functions for sorting arrays and slices of primitives in `const` contexts.

Arrays and slices of `bool`s, `u8`s, and `i8`s are sorted with counting sort while arrays of other types
are sorted with quicksort.

This implementation is usable on Rust version 1.59.0,
before the [`const_trait_impl`](https://github.com/rust-lang/rust/issues/67792) feature is stabilized.
This means that it unfortunately can not be generic,
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

Sort by reference:

```rust
use const_sort::sort_i32_slice;

const SORTED_ARRAY: [i32; 5] = {
    let mut arr = [5, i32::MIN, 0, -2, 0];
    sort_i32_slice(&mut arr);
    arr
};

assert_eq!(SORTED_ARRAY, [i32::MIN, -2, 0, 0, 5]);
```

The functions that sort slices by reference are only available on Rust version 1.83 and above.

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
