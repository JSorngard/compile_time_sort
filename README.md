# compile_time_sort

This crate provides functions for sorting arrays and slices of primitives in `const` contexts.

Arrays and slices of `bool`s, `u8`s, and `i8`s are sorted with counting sort while arrays of other types
are sorted with quicksort.

Functions with the naming convention `into_sorted_*_array` take an array by value,
and functions with the naming convention `sort_*_slice` take a mutable reference to a slice.

## Examples

Sort an array by value:

```rust
use const_sort::sort_i32_array;

const ARRAY: [i32; 5] = [-3, 3, 2, i32::MAX, 0];
const SORTED_ARRAY: [i32; 5] = sort_i32_array(ARRAY);

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
