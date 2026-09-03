# compile_time_sort

[![Crates.io Version](https://img.shields.io/crates/v/compile_time_sort?logo=rust)](https://crates.io/crates/compile_time_sort)
[![Docs.rs Documentation](https://img.shields.io/badge/docs.rs-compile__time__sort-66c2a5?logo=docs.rs)](https://docs.rs/compile_time_sort/latest/compile_time_sort/)
[![Github Repository Link](https://img.shields.io/badge/github-JSorngard%2Fcompile__time__sort-8da0cb?logo=github)](https://github.com/JSorngard/compile_time_sort)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/JSorngard/compile_time_sort/rust.yml?logo=github&label=CI)](https://github.com/JSorngard/compile_time_sort/actions/workflows/rust.yml)
[![Code Coverage](https://codecov.io/gh/JSorngard/compile_time_sort/graph/badge.svg?token=F61FO63ZKW)](https://codecov.io/gh/JSorngard/compile_time_sort)

This small crate provides macros for sorting arrays and slices of any type in `const` contexts with [introsort](https://en.wikipedia.org/wiki/Introsort).

This implementation is usable on Rust version 1.85.0, before the [`const_trait_impl`](https://github.com/rust-lang/rust/issues/143874) feature is stabilized.

The crate also provides `const` functions for sorting arrays and slices of primitives which are available on earlier Rust versions.
The functions that sort arrays are usable already on Rust version 1.56.0,
except the ones that sort arrays of floats, which need 1.83.0.
The functions that sort slices also need 1.83.0.

These functions do exactly the same thing as the macros, but they have been added as their own separate thing to let the crate sort primitives on even earlier Rust versions,
and they can also sometimes use more optimal sorting algorithms (like how `bool`s, `u8`, and `i8`s are sorted with [counting sort](https://en.wikipedia.org/wiki/Counting_sort).

## Examples

Sort an array by value:

```rust
use compile_time_sort::into_sorted_array_by;

// The `derive` on this type is only utilized in the assertion at the bottom
// of this example to check that the sorting succeeded.
// It is not needed for the macro to function.
#[derive(PartialOrd, PartialEq)]
struct ExampleStruct(u8);

const UNSORTED: [ExampleStruct; 3] = [ExampleStruct(3), ExampleStruct(1), ExampleStruct(2)];

const SORTED: [ExampleStruct; 3] = into_sorted_array_by!(
    UNSORTED,
    |a: ExampleStruct, b| { a.0 <= b.0 }
);

assert!(SORTED.is_sorted());
```

Sort by reference:

```rust
use compile_time_sort::sort_slice_by;

#[derive(PartialOrd, PartialEq)]
struct ExampleStruct(u8);

const SORTED: [ExampleStruct; 3] = {
    let mut arr =  [ExampleStruct(3), ExampleStruct(1), ExampleStruct(2)];
    sort_slice_by!(&mut arr, |a: ExampleStruct, b| { a.0 <= b.0 });
    arr
};

assert!(SORTED.is_sorted());
```

<div class = "rustdoc-hidden">

<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE.txt">Apache License, Version
2.0</a> or <a href="LICENSE-MIT.txt">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>

</div>