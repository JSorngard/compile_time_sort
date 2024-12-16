# const_sort

This crate provides functions for sorting arrays and slices of primitives in `const` contexts.

Depending on how you are doing `const` evaluation, sorting an array by value
or by reference might be useful. This crate provides functions for both.

Arrays of `bool`s, `u8`s, and `i8`s are sorted with counting sort while arrays of other types
are sorted with quicksort.

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
