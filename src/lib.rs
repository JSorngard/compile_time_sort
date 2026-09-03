// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

// These Markdown ideas are taken from https://linebender.org/blog/doc-include.
//
// This style is used in the readme itself to hide specific parts of it when rendered on docs.rs.
//! <style>
//! .rustdoc-hidden { display: none; }
//! </style>
#![doc = include_str!("../README.md")]
//!
//! # Features
//!
//! `nested`: enables the functions that sort slices of slices and arrays of slices.

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// This is added because of https://github.com/rust-lang/rust-clippy/issues/16450#issuecomment-3794847429
#![allow(clippy::incompatible_msrv)]

use core::num::NonZeroUsize;

mod primitives;
pub use primitives::*;

#[doc(hidden)]
/// If the array/slice is smaller than this size insertion sort will be used.
pub const INSERTION_SIZE: usize = 16;

#[rustversion::since(1.85.0)]
/// Sorts the given array with the given closure.
///
/// Only available on Rust version 1.85.0 and later.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// use compile_time_sort::const_sort_array_by;
///
/// // The `derive` on this type is only utilized in the assertion at the bottom
/// // of this example to check that the sorting succeeded.
/// // It is not needed for the macro to function.
/// #[derive(PartialOrd, PartialEq)]
/// struct Foo(u8);
///
/// const UNSORTED: [Foo; 3] = [Foo(3), Foo(0), Foo(1)];
/// const SORTED: [Foo; 3] = const_sort_array_by!(UNSORTED, |a: Foo, b| { a.0 <= b.0 } );
///
/// assert!(SORTED.is_sorted());
/// ```
#[macro_export]
macro_rules! const_sort_array_by {
    ($to_be_sorted:expr, |$a:ident: $element_type:ty, $b:ident| $are_in_sorted_order:block) => {{
        const fn are_in_sorted_order($a: &$element_type, $b: &$element_type) -> bool {
            $are_in_sorted_order
        }

        const fn intro_sort<const N: usize>(
            array: [$element_type; N],
            recursion_depth: u32,
            left: usize,
            right: usize,
        ) -> [$element_type; N] {
            let len = right - left;
            if len <= 1 {
                array
            } else if len <= $crate::INSERTION_SIZE {
                insertion_sort(array)
            } else if recursion_depth == 0 {
                heap_sort(array)
            } else {
                let (pivot_index, mut array) = partition(array, left, right);
                array = intro_sort(array, recursion_depth - 1, left, pivot_index);
                array = intro_sort(array, recursion_depth - 1, pivot_index + 1, right);
                array
            }
        }

        const fn partition<const N: usize>(
            mut arr: [$element_type; N],
            left: usize,
            right: usize,
        ) -> (usize, [$element_type; N]) {
            let len = right - left;
            let pivot_index = left + len / 2;
            let last_index = right - 1;

            arr.swap(pivot_index, last_index);

            let mut store_index = left;
            let mut i = left;
            while i < last_index {
                if are_in_sorted_order(&arr[i], &arr[last_index]) {
                    arr.swap(i, store_index);
                    store_index += 1;
                }
                i += 1;
            }
            arr.swap(store_index, last_index);

            (store_index, arr)
        }

        const fn heapify<const N: usize>(
            mut array: [$element_type; N],
            n: usize,
            i: usize,
        ) -> [$element_type; N] {
            let mut largest = i;

            let l = 2 * i + 1;
            let r = l + 1;

            if l < n && !are_in_sorted_order(&array[l], &array[largest]) {
                largest = l;
            }

            if r < n && !are_in_sorted_order(&array[r], &array[largest]) {
                largest = r;
            }

            if largest != i {
                array.swap(i, largest);
                array = heapify(array, n, largest);
            }

            array
        }

        const fn heap_sort<const N: usize>(mut array: [$element_type; N]) -> [$element_type; N] {
            if N <= 1 {
                return array;
            }

            let mut i = N / 2 - 1;
            while i > 0 {
                array = heapify(array, N, i);
                i -= 1;
            }
            // This call is ok since we know `i` is never negative.
            // We know this because we return early when `N` < 2, which means `i` >= 0.
            array = heapify(array, N, i);

            let mut i = N - 1;
            while i > 0 {
                array.swap(0, i);
                array = heapify(array, i, 0);
                i -= 1;
            }

            array
        }

        const fn insertion_sort<const N: usize>(
            mut array: [$element_type; N],
        ) -> [$element_type; N] {
            if N <= 1 {
                return array;
            }

            let mut i = 1;
            while i < N {
                let mut j = i;
                while j > 0 && !are_in_sorted_order(&array[j - 1], &array[j]) {
                    array.swap(j, j - 1);
                    j -= 1;
                }
                i += 1;
            }

            array
        }

        match ::core::num::NonZeroUsize::new($to_be_sorted.len()) {
            Some(nz) => {
                if nz.get() == 1 {
                    $to_be_sorted;
                }
                let max_depth = 2 * $crate::ilog2(nz);
                intro_sort($to_be_sorted, max_depth, 0, $to_be_sorted.len())
            }
            None => $to_be_sorted,
        }
    }};
}

#[rustversion::since(1.85.0)]
/// Sorts the given slice with the given closure.
///
/// Only available on Rust version 1.85.0 and later.
///
/// # Example
///
/// Basic usage:
///
/// ```
/// use compile_time_sort::const_sort_slice_by;
///
/// // The `derive` on this type is only utilized in the assertion at the bottom
/// // of this example to check that the sorting succeeded.
/// // It is not needed for the macro to function.
/// #[derive(PartialOrd, PartialEq)]
/// struct Test(u8);
///
/// const SORTED: [Test; 3] = {
///     let mut arr = [Test(1), Test(2), Test(0)];
///     const_sort_slice_by!(&mut arr, |a: Test, b| { a.0 <= b.0 });
///     arr
/// };
///
/// assert!(SORTED.is_sorted());
/// ```
#[macro_export]
macro_rules! const_sort_slice_by {
    ($to_be_sorted:expr, |$a:ident: $element_type:ty, $b:ident| $are_in_sorted_order:block) => {{
        const fn are_in_sorted_order($a: &$element_type, $b: &$element_type) -> bool {
            $are_in_sorted_order
        }

        const fn intro_sort(slice: &mut [$element_type], recursion_depth: u32) {
            if slice.len() <= 1 {
            } else if slice.len() <= $crate::INSERTION_SIZE {
                insertion_sort(slice);
            } else if recursion_depth == 0 {
                heap_sort(slice);
            } else {
                let (pivot, rest) = slice
                    .split_first_mut()
                    .expect("slice is not empty, as verified above");

                let mut left = 0;
                let mut right = rest.len() - 1;
                while left <= right {
                    if are_in_sorted_order(&rest[left], pivot) {
                        left += 1;
                    } else if !are_in_sorted_order(&rest[right], pivot) {
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    } else {
                        rest.swap(left, right);
                        left += 1;
                        if right == 0 {
                            break;
                        }
                        right -= 1;
                    }
                }

                slice.swap(0, left);

                let (left, right) = slice.split_at_mut(left);
                intro_sort(left, recursion_depth - 1);
                if let Some((_pivot, right)) = right.split_first_mut() {
                    intro_sort(right, recursion_depth - 1);
                }
            }
        }

        const fn insertion_sort(slice: &mut [$element_type]) {
            let n = slice.len();
            if n <= 1 {
                return;
            }

            let mut i = 1;
            while i < n {
                let mut j = i;
                while j > 0 && !are_in_sorted_order(&slice[j - 1], &slice[j]) {
                    slice.swap(j - 1, j);
                    j -= 1;
                }
                i += 1;
            }
        }

        const fn heapify(slice: &mut [$element_type], n: usize, i: usize) {
            let mut largest = i;

            let l = 2 * i + 1;
            let r = l + 1;

            if l < n && !are_in_sorted_order(&slice[l], &slice[largest]) {
                largest = l;
            }

            if r < n && !are_in_sorted_order(&slice[r], &slice[largest]) {
                largest = r;
            }

            if largest != i {
                slice.swap(i, largest);

                heapify(slice, n, largest);
            }
        }

        const fn heap_sort(slice: &mut [$element_type]) {
            let n = slice.len();

            if n <= 1 {
                return;
            }

            let mut i = n / 2 - 1;
            while i > 0 {
                heapify(slice, n, i);
                i -= 1;
            }
            // This call is ok since we know `i` is never negative.
            // We know this because we return early when `n` < 2, which means `i` >= 0.
            heapify(slice, n, i);

            let mut i = n - 1;
            while i > 0 {
                slice.swap(0, i);

                heapify(slice, i, 0);
                i -= 1;
            }
        }

        if let Some(nz) = ::core::num::NonZeroUsize::new($to_be_sorted.len()) {
            if nz.get() > 1 {
                let max_depth = 2 * $crate::ilog2(nz);
                intro_sort($to_be_sorted, max_depth);
            }
        }
    }};
}

#[doc(hidden)]
/// Implementation of the `ilog2` function that becomes available in Rust 1.67.0.
pub const fn ilog2(n: NonZeroUsize) -> u32 {
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
