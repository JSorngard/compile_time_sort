//! # Description
//!
//! This small crate provides functions for sorting arrays and slices of primitives in `const` contexts.
//!
//! Arrays and slices of `bool`s, `u8`s, and `i8`s are sorted with counting sort while arrays of other types
//! are sorted with quicksort.
//!
//! This implementation is usable on stable before the [`const_trait_impl`](https://github.com/rust-lang/rust/issues/67792) feature is stabilized,
//! but that means that it unfortunately can not be generic,
//! and so there are separate functions for every primitive type.
//!
//! Functions with the naming convention `into_sorted_*_array` take an array by value,
//! and functions with the naming convention `sort_*_slice` take a mutable reference to a slice.
//!
//! # Examples
//!
//! Sort an array by value:
//!
//! ```
//! use compile_time_sort::into_sorted_i32_array;
//!
//! const ARRAY: [i32; 5] = [-3, 3, 2, i32::MAX, 0];
//! const SORTED_ARRAY: [i32; 5] = into_sorted_i32_array(ARRAY);
//!
//! assert_eq!(SORTED_ARRAY, [-3, 0, 2, 3, i32::MAX]);
//! ```
//!
//! Sort an array by reference:
#![cfg_attr(
    feature = "sort_slices",
    doc = r"```
use compile_time_sort::sort_i32_slice;

const SORTED_ARRAY: [i32; 5] = {
    let mut arr = [5, i32::MIN, 0, -2, 0];
    sort_i32_slice(&mut arr);
    arr
};

assert_eq!(SORTED_ARRAY, [i32::MIN, -2, 0, 0, 5]);
```"
)]
//!
//! # Features
//!
//! `sort_slices`: enables the `sort_*_slice` functions and raises the MSRV of the crate from 1.59.0 to 1.83.0.

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "sort_slices")]
/// Defines a `const` function with the given name that takes in a mutable reference to a slice of the given type
/// and sorts it using the quicksort algorithm.
macro_rules! const_slice_quicksort {
    ($name:ident, $partition_name:ident, $tpe:ty) => {
        const fn $name(slice: &mut [$tpe], left: usize, right: usize) {
            if right - left > 1 {
                let pivot_index = $partition_name(slice.split_at_mut(right).0.split_at_mut(left).1);
                $name(slice, left, pivot_index);
                $name(slice, pivot_index + 1, right);
            }
        }

        const fn $partition_name(slice: &mut [$tpe]) -> usize {
            let len = slice.len();
            let pivot_index = len / 2;
            let last_index = len - 1;

            let tmp = slice[pivot_index];
            slice[pivot_index] = slice[last_index];
            slice[last_index] = tmp;

            let mut store_index = 0;
            let mut i = 0;
            while i < last_index {
                if slice[i] < slice[last_index] {
                    let tmp = slice[store_index];
                    slice[store_index] = slice[i];
                    slice[i] = tmp;
                    store_index += 1;
                }
                i += 1;
            }
            let tmp = slice[store_index];
            slice[store_index] = slice[last_index];
            slice[last_index] = tmp;

            store_index
        }
    };
}

/// Defines a `const` function with the given name that sorts an array of the given type with the quicksort algorithm.
macro_rules! const_array_quicksort {
    ($name:ident, $partition_name:ident, $tpe:ty) => {
        const fn $name<const N: usize>(array: [$tpe; N], left: usize, right: usize) -> [$tpe; N] {
            if right - left > 1 {
                let (pivot_index, mut array) = $partition_name(array, left, right);
                array = $name(array, left, pivot_index);
                array = $name(array, pivot_index + 1, right);
                return array;
            }

            array
        }

        const fn $partition_name<const N: usize>(
            mut arr: [$tpe; N],
            left: usize,
            right: usize,
        ) -> (usize, [$tpe; N]) {
            let len = right - left;
            let pivot_index = left + len / 2;
            let last_index = right - 1;

            (arr[pivot_index], arr[last_index]) = (arr[last_index], arr[pivot_index]);

            let mut store_index = left;
            let mut i = left;
            while i < last_index {
                if arr[i] < arr[last_index] {
                    (arr[store_index], arr[i]) = (arr[i], arr[store_index]);
                    store_index += 1;
                }
                i += 1;
            }
            (arr[store_index], arr[last_index]) = (arr[last_index], arr[store_index]);

            (store_index, arr)
        }
    };
}

macro_rules! impl_const_quicksort {
    ($pub_name_array:ident, $pub_name_slice:ident, $qsort_slice_name:ident, $partition_slice_name:ident, $qsort_array_name:ident, $partition_array_name:ident, $tpe:ty) => {
        #[cfg(feature = "sort_slices")]
        const_slice_quicksort!{$qsort_slice_name, $partition_slice_name, $tpe}

        const_array_quicksort!{$qsort_array_name, $partition_array_name, $tpe}

        #[doc = concat!("Sorts the given array of `", stringify!($tpe), "`s using the quicksort algorithm and returns it.")]
        /*#[doc = ""]
        #[doc = "# Example"]
        #[doc = ""]
        #[doc = "```"]
        #[doc = concat!("use compile_time_sort::into_sorted_", stringify!($tpe), "_array;")]
        #[doc = ""]
        #[doc = concat!("const ARRAY: [", stringify!($tpe), "; 5] = [3, 3, 2, ", stringify!($tpe), "::MAX, 0];")]
        #[doc = concat!("const SORTED_ARRAY: [", stringify!($tpe), "; 5] = into_sorted_", stringify!($tpe), "_array(ARRAY);")]
        #[doc = ""]
        #[doc = concat!("assert_eq!(SORTED_ARRAY, [0, 2, 3, 3, ", stringify!($tpe), "::MAX]);")]*/
        pub const fn $pub_name_array<const N: usize>(array: [$tpe; N]) -> [$tpe; N] {
            if N <= 1 {
                return array;
            }
            $qsort_array_name(array, 0, N)
        }

        #[cfg(feature = "sort_slices")]
        #[doc = concat!("Sorts the given slice of `", stringify!($tpe), "`s using the quicksort algorithm.")]
        pub const fn $pub_name_slice(slice: &mut [$tpe]) {
            if slice.len() <= 1 {
                return;
            }
            $qsort_slice_name(slice, 0, slice.len());
        }
    };
}

impl_const_quicksort!(
    into_sorted_char_array,
    sort_char_slice,
    qsort_char_slice,
    partition_char_slice,
    qsort_char_array,
    partition_char_array,
    char
);
impl_const_quicksort!(
    into_sorted_u16_array,
    sort_u16_slice,
    qsort_u16_slice,
    partition_u16_slice,
    qsort_u16_array,
    partition_u16_array,
    u16
);
impl_const_quicksort!(
    into_sorted_i16_array,
    sort_i16_slice,
    qsort_i16_slice,
    partition_i16_slice,
    qsort_i16_array,
    partition_i16_array,
    i16
);
impl_const_quicksort!(
    into_sorted_u32_array,
    sort_u32_slice,
    qsort_u32_slice,
    partition_u32_slice,
    qsort_u32_array,
    partition_u32_array,
    u32
);
impl_const_quicksort!(
    into_sorted_i32_array,
    sort_i32_slice,
    qsort_i32_slice,
    partition_i32_slice,
    qsort_i32_array,
    partition_i32_array,
    i32
);
impl_const_quicksort!(
    into_sorted_u64_array,
    sort_u64_slice,
    qsort_u64_slice,
    partition_u64_slice,
    qsort_u64_array,
    partition_u64_array,
    u64
);
impl_const_quicksort!(
    into_sorted_i64_array,
    sort_i64_slice,
    qsort_i64_slice,
    partition_i64_slice,
    qsort_i64_array,
    partition_i64_array,
    i64
);
impl_const_quicksort!(
    into_sorted_u128_array,
    sort_u128_slice,
    qsort_u128_slice,
    partition_u128_slice,
    qsort_u128_array,
    partition_u128_array,
    u128
);
impl_const_quicksort!(
    into_sorted_i128_array,
    sort_i128_slice,
    qsort_i128_slice,
    partition_i128_slice,
    qsort_i128_array,
    partition_i128_array,
    i128
);
impl_const_quicksort!(
    into_sorted_usize_array,
    sort_usize_slice,
    qsort_usize_slice,
    partition_usize_slice,
    qsort_usize_array,
    partition_usize_array,
    usize
);
impl_const_quicksort!(
    into_sorted_isize_array,
    sort_isize_slice,
    qsort_isize_slice,
    partition_isize_slice,
    qsort_isize_array,
    partition_isize_array,
    isize
);

#[cfg(feature = "sort_slices")]
/// Sorts the given slice of `i8`s using the counting sort algorithm.
pub const fn sort_i8_slice(slice: &mut [i8]) {
    if slice.is_empty() || slice.len() == 1 {
        return;
    }
    let mut counts = [0_usize; u8::MAX as usize + 1];
    let mut i = 0;
    let n = slice.len();
    while i < n {
        counts[(slice[i] as i16 + i8::MIN.unsigned_abs() as i16) as usize] += 1;
        i += 1;
    }
    i = 0;
    let mut j = 0;
    'outer: while i < n {
        while counts[j] == 0 {
            if j + 1 > u8::MAX as usize {
                break 'outer;
            }
            j += 1;
        }
        slice[i] = (j as i16 + i8::MIN.unsigned_abs() as i16) as i8;
        counts[j] -= 1;
        i += 1;
    }
}

/// Sorts the given array of `i8`s using the counting sort algorithm.
pub const fn into_sorted_i8_array<const N: usize>(mut array: [i8; N]) -> [i8; N] {
    if N == 0 || N == 1 {
        return array;
    }
    let mut counts = [0_usize; u8::MAX as usize + 1];
    let mut i = 0;
    while i < N {
        counts[(array[i] as i16 + i8::MIN.unsigned_abs() as i16) as usize] += 1;
        i += 1;
    }

    i = 0;
    let mut j = 0;
    'outer: while i < N {
        while counts[j] == 0 {
            if j + 1 > u8::MAX as usize {
                break 'outer;
            }
            j += 1;
        }
        array[i] = (j as i16 + i8::MIN.unsigned_abs() as i16) as i8;
        counts[j] -= 1;
        i += 1;
    }

    array
}

#[cfg(feature = "sort_slices")]
/// Sorts the given slice of `u8`s using the counting sort algorithm.
pub const fn sort_u8_slice(slice: &mut [u8]) {
    if slice.is_empty() || slice.len() == 1 {
        return;
    }
    let mut counts = [0_usize; u8::MAX as usize + 1];
    let mut i = 0;
    let n = slice.len();
    while i < n {
        counts[slice[i] as usize] += 1;
        i += 1;
    }
    i = 0;
    let mut j = 0;
    'outer: while i < n {
        while counts[j] == 0 {
            if j + 1 > u8::MAX as usize {
                break 'outer;
            }
            j += 1;
        }
        slice[i] = j as u8;
        counts[j] -= 1;
        i += 1;
    }
}

/// Sorts the given array of `u8`s using the counting sort algorithm.
pub const fn into_sorted_u8_array<const N: usize>(mut array: [u8; N]) -> [u8; N] {
    if N == 0 || N == 1 {
        return array;
    }
    let mut counts = [0_usize; u8::MAX as usize + 1];
    let mut i = 0;
    while i < N {
        counts[array[i] as usize] += 1;
        i += 1;
    }
    i = 0;
    let mut j = 0;
    'outer: while i < N {
        while counts[j] == 0 {
            if j + 1 > u8::MAX as usize {
                break 'outer;
            }
            j += 1;
        }
        array[i] = j as u8;
        counts[j] -= 1;
        i += 1;
    }
    array
}

#[cfg(feature = "sort_slices")]
/// Sorts the given slice of `bool`s using the counting sort algorithm.
pub const fn sort_bool_slice(slice: &mut [bool]) {
    if slice.is_empty() || slice.len() == 1 {
        return;
    }
    let mut falses = 0;
    let mut i = 0;
    let n = slice.len();
    while i < n {
        if !slice[i] {
            falses += 1;
        }
        i += 1;
    }

    i = 0;
    while i < n {
        if falses > 0 {
            slice[i] = false;
            falses -= 1;
        } else {
            slice[i] = true;
        }
        i += 1;
    }
}

/// Sorts the given array of `bool`s using the counting sort algorithm.
pub const fn into_sorted_bool_array<const N: usize>(mut array: [bool; N]) -> [bool; N] {
    if N == 0 || N == 1 {
        return array;
    }
    let mut falses = 0;
    let mut i = 0;
    while i < N {
        if !array[i] {
            falses += 1;
        }
        i += 1;
    }

    i = 0;
    while i < N {
        if falses > 0 {
            array[i] = false;
            falses -= 1;
        } else {
            array[i] = true;
        }
        i += 1;
    }

    array
}
