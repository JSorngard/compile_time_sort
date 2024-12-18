//! # Description
//!
//! This crate provides functions for sorting arrays and slices of primitives in `const` contexts.
//!
//! Arrays and slices of `bool`s, `u8`s, and `i8`s are sorted with counting sort while arrays of other types
//! are sorted with quicksort.
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
//! `sort_slices`: enables the `sort_*_slice` functions and raises the MSRV of the crate to 1.83.0.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(test)]
mod tests;

#[cfg(feature = "sort_slices")]
/// Defines a `const` function with the given name that takes in a mutable reference to a slice of the given type
/// and sorts it using the quicksort algorithm.
macro_rules! const_slice_quicksort {
    ($name:ident, $tpe:ty) => {
        const fn $name(slice: &mut [$tpe], left: usize, right: usize) {
            let pivot_candidate_1 = left;
            let pivot_candidate_2 = left + (right - left) / 2;
            let pivot_candidate_3 = right;
            let mut pivot_index = if slice[pivot_candidate_1] < slice[pivot_candidate_2] {
                if slice[pivot_candidate_3] < slice[pivot_candidate_2] {
                    if slice[pivot_candidate_1] < slice[pivot_candidate_3] {
                        pivot_candidate_3
                    } else {
                        pivot_candidate_1
                    }
                } else {
                    pivot_candidate_2
                }
            } else {
                if slice[pivot_candidate_3] < slice[pivot_candidate_1] {
                    if slice[pivot_candidate_2] < slice[pivot_candidate_3] {
                        pivot_candidate_3
                    } else {
                        pivot_candidate_2
                    }
                } else {
                    pivot_candidate_1
                }
            };

            let mut l = left;
            let mut r = right;

            while l < r {
                while (slice[pivot_index] < slice[r]) && (l < r) {
                    r -= 1;
                }
                if l != r {
                    (slice[pivot_index], slice[r]) = (slice[r], slice[pivot_index]);
                    pivot_index = r;
                }
                while (slice[l] < slice[pivot_index]) && (l < r) {
                    l += 1;
                }
                if l != r {
                    (slice[pivot_index], slice[l]) = (slice[l], slice[pivot_index]);
                    pivot_index = l;
                }
                if l != r && slice[l] == slice[r] {
                    // Break out of infinite loops
                    // if the elements at l and r are the same.
                    break;
                }
            }
            if left < l {
                $name(slice, left, l - 1);
            }
            if right > l {
                $name(slice, l + 1, right);
            }
        }
    };
}

/// Defines a `const` function with the given name that sorts an array of the given type with the quicksort algorithm.
macro_rules! const_array_quicksort {
    ($name:ident, $tpe:ty) => {
        const fn $name<const N: usize>(
            mut array: [$tpe; N],
            left: usize,
            right: usize,
        ) -> [$tpe; N] {
            let pivot_candidate_1 = left;
            let pivot_candidate_2 = left + (right - left) / 2;
            let pivot_candidate_3 = right;
            let mut pivot_index = if array[pivot_candidate_1] < array[pivot_candidate_2] {
                if array[pivot_candidate_3] < array[pivot_candidate_2] {
                    if array[pivot_candidate_1] < array[pivot_candidate_3] {
                        pivot_candidate_3
                    } else {
                        pivot_candidate_1
                    }
                } else {
                    pivot_candidate_2
                }
            } else {
                if array[pivot_candidate_3] < array[pivot_candidate_1] {
                    if array[pivot_candidate_2] < array[pivot_candidate_3] {
                        pivot_candidate_3
                    } else {
                        pivot_candidate_2
                    }
                } else {
                    pivot_candidate_1
                }
            };

            let mut l = left;
            let mut r = right;

            while l < r {
                while (array[pivot_index] < array[r]) && (l < r) {
                    r -= 1;
                }
                if l != r {
                    (array[pivot_index], array[r]) = (array[r], array[pivot_index]);
                    pivot_index = r;
                }
                while (array[l] < array[pivot_index]) && (l < r) {
                    l += 1;
                }
                if l != r {
                    (array[pivot_index], array[l]) = (array[l], array[pivot_index]);
                    pivot_index = l;
                }
                if l != r && array[l] == array[r] {
                    break;
                }
            }
            if left < l {
                array = $name(array, left, l - 1);
            }
            if right > l {
                array = $name(array, l + 1, right);
            }
            array
        }
    };
}

macro_rules! impl_const_quicksort {
    ($pub_name_array:ident, $pub_name_slice:ident, $qsort_slice_name:ident, $qsort_array_name:ident, $tpe:ty, $tpe_name: literal) => {
        #[cfg(feature = "sort_slices")]
        const_slice_quicksort!{$qsort_slice_name, $tpe}

        const_array_quicksort!{$qsort_array_name, $tpe}

        #[doc = concat!("Sorts the given array of `", $tpe_name, "`s using the quicksort algorithm")]
        pub const fn $pub_name_array<const N: usize>(array: [$tpe; N]) -> [$tpe; N] {
            if N == 0 || N == 1 {
                return array;
            }
            $qsort_array_name(array, 0, N - 1)
        }

        #[cfg(feature = "sort_slices")]
        #[doc = concat!("Sorts the given slice of `", $tpe_name, "`s using the quicksort algorithm")]
        pub const fn $pub_name_slice(slice: &mut [$tpe]) {
            if slice.is_empty() || slice.len() == 1 {
                return;
            }
            let last = slice.len() - 1;
            $qsort_slice_name(slice, 0, last);
        }
    };
}

impl_const_quicksort!(
    into_sorted_char_array,
    sort_char_slice,
    qsort_char_slice,
    qsort_char_array,
    char,
    "char"
);
impl_const_quicksort!(
    into_sorted_u16_array,
    sort_u16_slice,
    qsort_u16_slice,
    qsort_u16_array,
    u16,
    "u16"
);
impl_const_quicksort!(
    into_sorted_i16_array,
    sort_i16_slice,
    qsort_i16_slice,
    qsort_i16_array,
    i16,
    "i16"
);
impl_const_quicksort!(
    into_sorted_u32_array,
    sort_u32_slice,
    qsort_u32_slice,
    qsort_u32_array,
    u32,
    "u32"
);
impl_const_quicksort!(
    into_sorted_i32_array,
    sort_i32_slice,
    qsort_i32_slice,
    qsort_i32_array,
    i32,
    "i32"
);
impl_const_quicksort!(
    into_sorted_u64_array,
    sort_u64_slice,
    qsort_u64_slice,
    qsort_u64_array,
    u64,
    "u64"
);
impl_const_quicksort!(
    into_sorted_i64_array,
    sort_i64_slice,
    qsort_i64_slice,
    qsort_i64_array,
    i64,
    "i64"
);
impl_const_quicksort!(
    into_sorted_u128_array,
    sort_u128_slice,
    qsort_u128_slice,
    qsort_u128_array,
    u128,
    "u128"
);
impl_const_quicksort!(
    into_sorted_i128_array,
    sort_i128_slice,
    qsort_i128_slice,
    qsort_i128_array,
    i128,
    "i128"
);
impl_const_quicksort!(
    into_sorted_usize_array,
    sort_usize_slice,
    qsort_usize_slice,
    qsort_usize_array,
    usize,
    "usize"
);
impl_const_quicksort!(
    into_sorted_isize_array,
    sort_isize_slice,
    qsort_isize_slice,
    qsort_isize_array,
    isize,
    "isize"
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
