// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Description
//!
//! This small crate provides functions for sorting arrays and slices of primitives in `const` contexts.
//!
//! Arrays and slices of `bool`s, `u8`s, and `i8`s are sorted with counting sort while other types
//! are sorted with introsort.
//!
//! This implementation is usable on Rust version 1.54.0, before the [`const_trait_impl`](https://github.com/rust-lang/rust/issues/143874) feature is stabilized.
//! This means that it unfortunately can not be generic,
//! and so there are separate functions for every primitive type.
//!
//! Functions with the naming convention `into_sorted_*_array` take an array by value,
//! and functions with the naming convention `sort_*_slice` take a mutable reference to a slice.
//!
//! The functions that sort slices by reference are only available on Rust versions 1.83 and above, as are the functions that sort floats as they need [`{float}::to_bits`](f32::to_bits)
//! to be `const` in order to generate a total ordering in accordance with [`{float}::total_cmp`](f32::total_cmp).
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
//! Sort by reference:
//!
//! ```
//! use compile_time_sort::sort_i32_slice;
//!
//! const SORTED_ARRAY: [i32; 5] = {
//!     let mut arr = [5, i32::MIN, 0, -2, 0];
//!     sort_i32_slice(&mut arr);
//!     arr
//! };
//!
//! assert_eq!(SORTED_ARRAY, [i32::MIN, -2, 0, 0, 5]);
//! ```
//!
//! # Features
//!
//! `nested`: enables the functions that sort slices of slices and arrays of slices.

// This crate is implemented mainly through macros. This is used to copy-paste the implementation
// of the sorting algorithms many times, once for each type, as we can not use const generics due to MSRV.
// `impl_const_introsort!` is tha macro that expands to the sorting implementation, but it needs a prerequisites
// in order for the generated code to compile. There must be const functions that implement various const comparisons
// available in the callers scope. The macro `impl_default_const_compare!` creates that needed functions for any type
// that already has const comparison operators, other types must be implemented manually.
//
// This almost works. Unfortunately there are some types that need special handling, and those are the zero-sized types
// (string slices and other slices) as well as types that don't have an Ord impl (floats).
//
// A new macro is made to generate const comparison functions for zero-sized types (`impl_default_const_slice_compare!`), and string slices are then
// compared using the functions for byte slices. Floats have a `total_cmp` function which unfortunately isn't const as of time of writing.
// That function has been manually implemented to be const in this library, and then floats have const comparison function implemented in terms of those.

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// This is added because of https://github.com/rust-lang/rust-clippy/issues/16450#issuecomment-3794847429
#![allow(clippy::incompatible_msrv)]

use core::cmp::Ordering;
use core::num::NonZeroUsize;

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadMeDocTests;

/// If the array/slice is smaller than this size insertion sort will be used.
const INSERTION_SIZE: usize = 16;

// region: comparison wrappers

/// This macro generates wrappers around the default comparison operators for the given types.
/// This is needed because floats can not be compared without generating these wrappers
/// around the [`f32::total_cmp`] and [`f64::total_cmp`] functions.
/// This means that if we just wrap the comparison operators for all types
/// we can use the same macros to generate the sorting functions for all types.
macro_rules! impl_default_const_compare {
    ($($tpe:ty),+) => {
        $(
            paste::paste! {
                #[allow(unused)]
                #[inline]
                const fn [<greater_than_ $tpe>](a: $tpe, b: $tpe) -> bool {
                    a > b
                }

                #[allow(unused)]
                #[inline]
                const fn [<less_or_equal_ $tpe>](a: $tpe, b: $tpe) -> bool {
                    a <= b
                }

                #[allow(unused)]
                #[inline]
                const fn [<less_than_ $tpe>](a: $tpe, b: $tpe) -> bool {
                    a < b
                }
            }
        )+
    };
}

impl_default_const_compare! {
    bool,
    char,
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize
}

// Below are the wrappers for floats. They are taken from the standard library
// implementation of `{float}::total_cmp` and adapted to be `const`.

#[rustversion::since(1.83.0)]
#[inline]
const fn total_cmp_f32(a: f32, b: f32) -> Ordering {
    let mut left = a.to_bits() as i32;

    let mut right = b.to_bits() as i32;

    left ^= (((left >> 31) as u32) >> 1) as i32;

    right ^= (((right >> 31) as u32) >> 1) as i32;

    if left < right {
        Ordering::Less
    } else if left > right {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[rustversion::since(1.83.0)]
#[inline]
const fn total_cmp_f64(a: f64, b: f64) -> Ordering {
    let mut left = a.to_bits() as i64;

    let mut right = b.to_bits() as i64;

    left ^= (((left >> 63) as u64) >> 1) as i64;

    right ^= (((right >> 63) as u64) >> 1) as i64;

    if left < right {
        Ordering::Less
    } else if left > right {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[rustversion::since(1.83.0)]
#[inline]
const fn greater_than_f32(a: f32, b: f32) -> bool {
    matches!(total_cmp_f32(a, b), Ordering::Greater)
}

#[rustversion::since(1.83.0)]
#[inline]
const fn less_or_equal_f32(a: f32, b: f32) -> bool {
    matches!(total_cmp_f32(a, b), Ordering::Less | Ordering::Equal)
}

#[rustversion::since(1.83.0)]
#[inline]
const fn less_than_f32(a: f32, b: f32) -> bool {
    matches!(total_cmp_f32(a, b), Ordering::Less)
}

#[rustversion::since(1.83.0)]
#[inline]
const fn greater_than_f64(a: f64, b: f64) -> bool {
    matches!(total_cmp_f64(a, b), Ordering::Greater)
}

#[rustversion::since(1.83.0)]
#[inline]
const fn less_or_equal_f64(a: f64, b: f64) -> bool {
    matches!(total_cmp_f64(a, b), Ordering::Less | Ordering::Equal)
}

#[rustversion::since(1.83.0)]
#[inline]
const fn less_than_f64(a: f64, b: f64) -> bool {
    matches!(total_cmp_f64(a, b), Ordering::Less)
}

/// This macro implements lexicographic ordering of slices of the given types.
macro_rules! impl_default_const_slice_compare {
    ($($tpe:ty),+) => {
        $(
            paste::paste! {
                const fn [<compare_ $tpe _slices>](a: &[$tpe], b: &[$tpe]) -> Ordering {
                    let mut i = 0;
                    while i < a.len() && i < b.len() {
                        if [<less_than_ $tpe>](a[i], b[i]) {
                            return Ordering::Less;
                        } else if [<greater_than_ $tpe>](a[i], b[i]) {
                            return Ordering::Greater;
                        }
                        i += 1;
                    }
                    if a.len() < b.len() {
                        Ordering::Less
                    } else if a.len() == b.len() {
                        Ordering::Equal
                    } else {
                        Ordering::Greater
                    }
                }

                #[allow(unused)]
                #[inline]
                const fn [<greater_than_ $tpe _slice>](a: &[$tpe], b: &[$tpe]) -> bool {
                    matches!([<compare_ $tpe _slices>](a, b), Ordering::Greater)
                }

                #[allow(unused)]
                #[inline]
                const fn [<less_or_equal_ $tpe _slice>](a: &[$tpe], b: &[$tpe]) -> bool {
                    matches!([<compare_ $tpe _slices>](a, b), Ordering::Less | Ordering::Equal)
                }

                #[allow(unused)]
                #[inline]
                const fn [<less_than_ $tpe _slice>](a: &[$tpe], b: &[$tpe]) -> bool {
                    matches!([<compare_ $tpe _slices>](a, b), Ordering::Less)
                }
            }
        )+
    };
}

impl_default_const_slice_compare! {
    u8
}

#[cfg(feature = "nested")]
impl_default_const_slice_compare! {
    char,
    bool,
    i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize
}

#[cfg(feature = "nested")]
#[rustversion::since(1.83.0)]
impl_default_const_slice_compare! {
    f32, f64
}

const fn compare_str_slices(a: &str, b: &str) -> Ordering {
    compare_u8_slices(a.as_bytes(), b.as_bytes())
}

const fn greater_than_str(a: &str, b: &str) -> bool {
    matches!(compare_str_slices(a, b), Ordering::Greater)
}

const fn less_or_equal_str(a: &str, b: &str) -> bool {
    matches!(compare_str_slices(a, b), Ordering::Less | Ordering::Equal)
}

const fn less_than_str(a: &str, b: &str) -> bool {
    matches!(compare_str_slices(a, b), Ordering::Less)
}

// endregion: comparison wrappers

// region: introsort implementations

#[rustversion::since(1.83.0)]
/// Defines a `const` function with the given name that takes in a mutable reference to a slice of the given type
/// and sorts it using the introsort algorithm while switching to the insertion sort algorithm when the array is small.
macro_rules! const_slice_introsort {
    ($tpe:ty, $intro_name:ident, $insertion_name:ident, $heap_name:ident, $max_heapify_name: ident, $less_or_equal:ident, $greater_than:ident) => {
        const_slice_insertion_sort!($tpe, $insertion_name, $greater_than);

        const_slice_heapsort!($tpe, $heap_name, $max_heapify_name, $greater_than);

        const fn $intro_name(slice: &mut [$tpe], recursion_depth: u32) {
            if slice.len() <= 1 {
            } else if slice.len() <= INSERTION_SIZE {
                $insertion_name(slice);
            } else if recursion_depth == 0 {
                $heap_name(slice);
            } else {
                let (pivot, rest) = slice
                    .split_first_mut()
                    .expect("slice is not empty, as verified above");

                let mut left = 0;
                let mut right = rest.len() - 1;
                while left <= right {
                    if $less_or_equal(rest[left], *pivot) {
                        left += 1;
                    } else if $greater_than(rest[right], *pivot) {
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    } else {
                        (rest[left], rest[right]) = (rest[right], rest[left]);
                        left += 1;
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    }
                }

                (slice[0], slice[left]) = (slice[left], slice[0]);

                let (left, right) = slice.split_at_mut(left);
                $intro_name(left, recursion_depth - 1);
                if let Some((_pivot, right)) = right.split_first_mut() {
                    $intro_name(right, recursion_depth - 1);
                }
            }
        }
    };
}

/// Defines a `const` function with the given name that sorts an array of the given type with the introsort algorithm
/// for large arrays and switches to the insertion sort algorithm when the array is small.
macro_rules! const_array_introsort {
    ($tpe:ty, $intro_name:ident, $partition_name:ident, $insertion_name:ident, $heap_name:ident, $max_heapify_name: ident, $greater_than:ident, $less_than:ident) => {
        const_array_insertion_sort! {$tpe, $insertion_name, $greater_than}

        const_array_heapsort! {$tpe, $heap_name, $max_heapify_name, $greater_than}

        const fn $intro_name<const N: usize>(
            array: [$tpe; N],
            recursion_depth: u32,
            left: usize,
            right: usize,
        ) -> [$tpe; N] {
            let len = right - left;
            if len <= 1 {
                array
            } else if len <= INSERTION_SIZE {
                $insertion_name(array)
            } else if recursion_depth == 0 {
                $heap_name(array)
            } else {
                let (pivot_index, mut array) = $partition_name(array, left, right);
                array = $intro_name(array, recursion_depth - 1, left, pivot_index);
                array = $intro_name(array, recursion_depth - 1, pivot_index + 1, right);
                array
            }
        }

        const fn $partition_name<const N: usize>(
            mut arr: [$tpe; N],
            left: usize,
            right: usize,
        ) -> (usize, [$tpe; N]) {
            let len = right - left;
            let pivot_index = left + len / 2;
            let last_index = right - 1;

            let temp = arr[pivot_index];
            arr[pivot_index] = arr[last_index];
            arr[last_index] = temp;

            let mut store_index = left;
            let mut i = left;
            while i < last_index {
                if $less_than(arr[i], arr[last_index]) {
                    let temp = arr[i];
                    arr[i] = arr[store_index];
                    arr[store_index] = temp;
                    store_index += 1;
                }
                i += 1;
            }
            let temp = arr[store_index];
            arr[store_index] = arr[last_index];
            arr[last_index] = temp;

            (store_index, arr)
        }
    };
}

/// Defines a `const` function with the given name that sorts an array of the given type with the insertion sort algorithm.
macro_rules! const_array_insertion_sort {
    ($tpe:ty, $name:ident, $greater_than:ident) => {
        const fn $name<const N: usize>(mut array: [$tpe; N]) -> [$tpe; N] {
            if N <= 1 {
                return array;
            }

            let mut i = 1;
            while i < N {
                let mut j = i;
                while j > 0 && $greater_than(array[j - 1], array[j]) {
                    let temp = array[j - 1];
                    array[j - 1] = array[j];
                    array[j] = temp;
                    j -= 1;
                }
                i += 1;
            }

            array
        }
    };
}

#[rustversion::since(1.83.0)]
/// Defines a `const` function with the given name that sorts a slice of the given type with the insertion sort algorithm.
macro_rules! const_slice_insertion_sort {
    ($tpe:ty, $name:ident, $greater_than:ident) => {
        const fn $name(slice: &mut [$tpe]) {
            let n = slice.len();
            if n <= 1 {
                return;
            }

            let mut i = 1;
            while i < n {
                let mut j = i;
                while j > 0 && $greater_than(slice[j - 1], slice[j]) {
                    (slice[j - 1], slice[j]) = (slice[j], slice[j - 1]);
                    j -= 1;
                }
                i += 1;
            }
        }
    };
}

/// Defines a `const` function with the given name that sorts the given array with heapsort.
macro_rules! const_array_heapsort {
    ($tpe:ty, $name:ident, $heapify_name:ident, $greater_than:ident) => {
        const fn $heapify_name<const N: usize>(
            mut array: [$tpe; N],
            n: usize,
            i: usize,
        ) -> [$tpe; N] {
            let mut largest = i;

            let l = 2 * i + 1;
            let r = l + 1;

            if l < n && $greater_than(array[l], array[largest]) {
                largest = l;
            }

            if r < n && $greater_than(array[r], array[largest]) {
                largest = r;
            }

            if largest != i {
                let temp = array[i];
                array[i] = array[largest];
                array[largest] = temp;

                array = $heapify_name(array, n, largest);
            }

            array
        }

        const fn $name<const N: usize>(mut array: [$tpe; N]) -> [$tpe; N] {
            if N <= 1 {
                return array;
            }

            let mut i = N / 2 - 1;
            while i > 0 {
                array = $heapify_name(array, N, i);
                i -= 1;
            }
            // This call is ok since we know `i` is never negative.
            // We know this because we return early when `N` < 2, which means `i` >= 0.
            array = $heapify_name(array, N, i);

            let mut i = N - 1;
            while i > 0 {
                let temp = array[0];
                array[0] = array[i];
                array[i] = temp;

                array = $heapify_name(array, i, 0);
                i -= 1;
            }

            array
        }
    };
}

#[rustversion::since(1.83.0)]
/// Defines a `const` function with the given name that sorts the given slice with heapsort.
macro_rules! const_slice_heapsort {
    ($tpe:ty, $name:ident, $heapify_name:ident, $greater_than:ident) => {
        const fn $heapify_name(slice: &mut [$tpe], n: usize, i: usize) {
            let mut largest = i;

            let l = 2 * i + 1;
            let r = l + 1;

            if l < n && $greater_than(slice[l], slice[largest]) {
                largest = l;
            }

            if r < n && $greater_than(slice[r], slice[largest]) {
                largest = r;
            }

            if largest != i {
                (slice[i], slice[largest]) = (slice[largest], slice[i]);

                $heapify_name(slice, n, largest);
            }
        }

        const fn $name(slice: &mut [$tpe]) {
            let n = slice.len();

            if n <= 1 {
                return;
            }

            let mut i = n / 2 - 1;
            while i > 0 {
                $heapify_name(slice, n, i);
                i -= 1;
            }
            // This call is ok since we know `i` is never negative.
            // We know this because we return early when `n` < 2, which means `i` >= 0.
            $heapify_name(slice, n, i);

            let mut i = n - 1;
            while i > 0 {
                (slice[0], slice[i]) = (slice[i], slice[0]);

                $heapify_name(slice, i, 0);
                i -= 1;
            }
        }
    };
}

/// Defines the public const introsort implementations for the given list of types.
/// One function that sorts slices and one function that sorts arrays for each type.
///
/// The macro has two arms, one for defining functions for types, and one for defining functions for slices of types.
macro_rules! impl_const_introsort {
    ($([$tpe:ident]),+) => {
        $(
            paste::paste! {
                #[rustversion::since(1.83.0)]
                const_slice_introsort!{&[$tpe], [<introsort_ $tpe _slice_slice>], [<insertion_sort_ $tpe _slice_slice>], [<heapsort_ $tpe _slice_slice>], [<max_heapify_ $tpe _slice_slice>], [<less_or_equal_ $tpe _slice>], [<greater_than_ $tpe _slice>]}

                const_array_introsort!{&[$tpe], [<introsort_ $tpe _slice_array>], [<partition_ $tpe _slice_array>], [<insertion_sort_ $tpe _slice_array>], [<heapsort_ $tpe _slice_array>], [<max_heapify_ $tpe _slice_array>], [<greater_than_ $tpe _slice>], [<less_than_ $tpe _slice>]}

                #[doc = "Sorts the given array of `&[" $tpe "]`s using the introsort algorithm and returns it."]
                pub const fn [<into_sorted_ $tpe _slice_array>]<const N: usize>(array: [&[$tpe]; N]) -> [&[$tpe]; N] {
                    match NonZeroUsize::new(N) {
                        Some(nz) => {
                            if nz.get() == 1 {
                                return array;
                            }
                            let max_depth = 2*ilog2(nz);
                            [<introsort_ $tpe _slice_array>](array, max_depth, 0, N)
                        }
                        None => array
                    }
                }

                #[rustversion::since(1.83.0)]
                #[doc = "Sorts the given slice of `&[" $tpe "]`s using the introsort algorithm."]
                pub const fn [<sort_ $tpe _slice_slice>](slice: &mut [&[$tpe]]) {
                    if let Some(nz) = NonZeroUsize::new(slice.len()) {
                        if nz.get() <= 1 {
                            return;
                        }

                        let max_depth = 2*ilog2(nz);
                        [<introsort_ $tpe _slice_slice>](slice, max_depth);
                    }
                }
            }
        )+
    };
    ($($tpe:ty),+) => {
        $(
            paste::paste! {
                #[rustversion::since(1.83.0)]
                const_slice_introsort!{$tpe, [<introsort_ $tpe _slice>], [<insertion_sort_ $tpe _slice>], [<heapsort_ $tpe _slice>], [<max_heapify_ $tpe _slice>], [<less_or_equal_ $tpe>], [<greater_than_ $tpe>]}

                const_array_introsort!{$tpe, [<introsort_ $tpe _array>], [<partition_ $tpe _array>], [<insertion_sort_ $tpe _array>], [<heapsort_ $tpe _array>], [<max_heapify_ $tpe _array>], [<greater_than_ $tpe>], [<less_than_ $tpe>]}

                #[doc = "Sorts the given array of `" $tpe "`s using the introsort algorithm and returns it."]
                #[doc = ""]
                #[doc = "# Example"]
                #[doc = ""]
                #[doc = "```"]
                #[doc = "use compile_time_sort::" [<into_sorted_ $tpe _array>] ";"]
                #[doc = ""]
                #[doc = "const SORTED_ARRAY: [" $tpe "; 3] = " [<into_sorted_ $tpe _array>] "([0 as " $tpe ", " $tpe "::MAX, " $tpe "::MIN]);"]
                #[doc = ""]
                #[doc = "assert!(SORTED_ARRAY.is_sorted());"]
                #[doc = "```"]
                pub const fn [<into_sorted_ $tpe _array>]<const N: usize>(array: [$tpe; N]) -> [$tpe; N] {
                    match NonZeroUsize::new(N) {
                        Some(nz) => {
                            if nz.get() == 1 {
                                return array;
                            }
                            let max_depth = 2*ilog2(nz);
                            [<introsort_ $tpe _array>](array, max_depth, 0, N)
                        }
                        None => array
                    }
                }

                #[rustversion::since(1.83.0)]
                #[doc = "Sorts the given slice of `" $tpe "`s using the introsort algorithm."]
                #[doc = ""]
                #[doc = "# Example"]
                #[doc = ""]
                #[doc = "```"]
                #[doc = "use compile_time_sort::" [<sort_ $tpe _slice>] ";"]
                #[doc = ""]
                #[doc = "const SORTED_ARRAY: [" $tpe "; 3]= {"]
                #[doc = "    let mut arr = [0 as " $tpe ", " $tpe "::MAX, " $tpe "::MIN];"]
                #[doc = "    " [<sort_ $tpe _slice>] "(&mut arr);"]
                #[doc = "    arr"]
                #[doc = "};"]
                #[doc = ""]
                #[doc = "assert!(SORTED_ARRAY.is_sorted());"]
                #[doc = "```"]
                pub const fn [<sort_ $tpe _slice>](slice: &mut [$tpe]) {
                    if let Some(nz) = NonZeroUsize::new(slice.len()) {
                        if nz.get() <= 1 {
                            return;
                        }

                        let max_depth = 2*ilog2(nz);
                        [<introsort_ $tpe _slice>](slice, max_depth);
                    }
                }
            }
        )+
    };
}

/// Implementation of the `ilog2` function that becomes available in Rust 1.67.0.
const fn ilog2(n: NonZeroUsize) -> u32 {
    let mut n = n.get();

    let mut exp = usize::BITS / 2;
    let mut i = 0;
    while exp > 0 {
        if n >= (1 << exp) {
            i += exp;
            n >>= exp;
        }
        exp /= 2;
    }
    i
}

// We don't call this macro on `bool`, `u8`, or `i8` because they can be efficiently sorted with counting sort
// and that requires a custom implementation for each type.
impl_const_introsort! {
    char,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize
}

impl_const_introsort! {
    [u8]
}

#[cfg(feature = "nested")]
impl_const_introsort! {
    [char],
    [bool],
    [i8],
    [u16], [i16],
    [u32], [i32],
    [u64], [i64],
    [u128], [i128],
    [usize], [isize]
}

#[cfg(feature = "nested")]
#[rustversion::since(1.83.0)]
impl_const_introsort! {
    [f32], [f64]
}

#[rustversion::since(1.83.0)]
const_slice_introsort! {&str, introsort_str_slice, insertion_sort_str_slice, heapsort_str_slice, max_heapify_str_slice, less_or_equal_str, greater_than_str}

const_array_introsort! {&str, introsort_str_array, partition_str_array, insertion_sort_str_array, heapsort_str_array, max_heapify_str_array, greater_than_str, less_than_str}

/// Sorts the given array of `str`s using the introsort algorithm.
pub const fn into_sorted_str_array<const N: usize>(array: [&str; N]) -> [&str; N] {
    match NonZeroUsize::new(N) {
        Some(nz) => {
            if nz.get() == 1 {
                return array;
            }
            let max_depth = 2 * ilog2(nz);
            introsort_str_array(array, max_depth, 0, N)
        }
        None => array,
    }
}

#[rustversion::since(1.83.0)]
/// Sorts the given slice of `str`s using the introsort algorithm.
pub const fn sort_str_slice(slice: &mut [&str]) {
    if let Some(nz) = NonZeroUsize::new(slice.len()) {
        if nz.get() <= 1 {
            return;
        }

        let max_depth = 2 * ilog2(nz);
        introsort_str_slice(slice, max_depth);
    }
}

#[rustversion::since(1.83.0)]
impl_const_introsort! {f32, f64}

// endregion: introsort implementations

// region: counting sort implementations

#[rustversion::since(1.83.0)]
/// Sorts the given slice of `i8`s using the counting sort algorithm.
///
/// Switches to insertion sort when the slice is small.
///
/// This function is only available on Rust versions 1.83 and above.
///
/// # Example
///
/// ```
/// use compile_time_sort::sort_i8_slice;
///
/// const ARRAY: [i8; 3] = [0, i8::MAX, i8::MIN];
/// const SORTED_ARRAY: [i8; 3] = {
///     let mut arr = ARRAY;
///     sort_i8_slice(&mut arr);
///     arr
/// };
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
pub const fn sort_i8_slice(slice: &mut [i8]) {
    if slice.len() <= 1 {
        return;
    } else if slice.len() <= INSERTION_SIZE {
        insertion_sort_i8_slice(slice);
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

#[rustversion::since(1.83.0)]
const_slice_insertion_sort!(i8, insertion_sort_i8_slice, greater_than_i8);

/// Sorts the given array of `i8`s using the counting sort algorithm and returns it.
///
/// Switches to insertion sort when the array is small.
///
/// # Example
///
/// ```
/// use compile_time_sort::into_sorted_i8_array;
///
/// const SORTED_ARRAY: [i8; 3] = into_sorted_i8_array([0, i8::MAX, i8::MIN]);
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
pub const fn into_sorted_i8_array<const N: usize>(mut array: [i8; N]) -> [i8; N] {
    if N <= 1 {
        return array;
    } else if N <= INSERTION_SIZE {
        return insertion_sort_i8_array(array);
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

const_array_insertion_sort!(i8, insertion_sort_i8_array, greater_than_i8);

#[rustversion::since(1.83.0)]
/// Sorts the given slice of `u8`s using the counting sort algorithm.
///
/// Switches to insertion sort when the slice is small.
///
/// This function is only available on Rust versions 1.83 and above.
///
/// # Example
///
/// ```
/// use compile_time_sort::sort_u8_slice;
///
/// const ARRAY: [u8; 3] = [0, u8::MAX, u8::MIN];
/// const SORTED_ARRAY: [u8; 3] = {
///     let mut arr = ARRAY;
///     sort_u8_slice(&mut arr);
///     arr
/// };
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
pub const fn sort_u8_slice(slice: &mut [u8]) {
    if slice.len() <= 1 {
        return;
    } else if slice.len() <= INSERTION_SIZE {
        insertion_sort_u8_slice(slice);
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

#[rustversion::since(1.83.0)]
const_slice_insertion_sort!(u8, insertion_sort_u8_slice, greater_than_u8);

/// Sorts the given array of `u8`s using the counting sort algorithm and returns it.
///
/// Switches to insertion sort when the array is small.
///
/// # Example
///
/// ```
/// use compile_time_sort::into_sorted_u8_array;
///
/// const SORTED_ARRAY: [u8; 3] = into_sorted_u8_array([0, u8::MAX, u8::MIN]);
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
pub const fn into_sorted_u8_array<const N: usize>(mut array: [u8; N]) -> [u8; N] {
    if N <= 1 {
        return array;
    } else if N <= INSERTION_SIZE {
        return insertion_sort_u8_array(array);
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

const_array_insertion_sort!(u8, insertion_sort_u8_array, greater_than_u8);

#[rustversion::since(1.83.0)]
/// Sorts the given slice of `bool`s using the counting sort algorithm.
///
/// This function is only available on Rust versions 1.83 and above.
///
/// # Example
///
/// ```
/// use compile_time_sort::sort_bool_slice;
///
/// const ARRAY: [bool; 2] = [true, false];
/// const SORTED_ARRAY: [bool; 2] = {
///     let mut arr = ARRAY;
///     sort_bool_slice(&mut arr);
///     arr
/// };
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
pub const fn sort_bool_slice(slice: &mut [bool]) {
    if slice.len() <= 1 {
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

/// Sorts the given array of `bool`s using the counting sort algorithm and returns it.
///
/// # Example
///
/// ```
/// use compile_time_sort::into_sorted_bool_array;
///
/// const SORTED_ARRAY: [bool; 2] = into_sorted_bool_array([true, false]);
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
pub const fn into_sorted_bool_array<const N: usize>(mut array: [bool; N]) -> [bool; N] {
    if N <= 1 {
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

// endregion: counting sort implementations

#[cfg(test)]
mod test {
    use crate::ilog2;
    use core::num::NonZeroUsize;

    #[test]
    fn test_ilog2() {
        for i in 1..10000 {
            assert_eq!(ilog2(NonZeroUsize::new(i).unwrap()), i.ilog2());
        }
    }
}
