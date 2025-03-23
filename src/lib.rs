//! # Description
//!
//! This small crate provides functions for sorting arrays and slices of primitives in `const` contexts.
//!
//! Arrays and slices of `bool`s, `u8`s, and `i8`s are sorted with counting sort while arrays of other types
//! are sorted with quicksort.
//! All types except `bool` are sorted with insertion sort if the length is small.
//!
//! This implementation is usable on Rust version 1.59.0, before the [`const_trait_impl`](https://github.com/rust-lang/rust/issues/67792) feature is stabilized.
//! This means that it unfortunately can not be generic,
//! and so there are separate functions for every primitive type.
//!
//! Functions with the naming convention `into_sorted_*_array` take an array by value,
//! and functions with the naming convention `sort_*_slice` take a mutable reference to a slice.
//!
//! The functions that sort slices by reference are only available on Rust versions 1.83 and above.
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

#![no_std]

/// If the array/slice is larger than this size quicksort will be used,
/// otherwise insertion sort will be used.
const INSERTION_SIZE: usize = 10;

use paste::paste;

// region: quicksort implementations

#[rustversion::since(1.83.0)]
/// Defines a `const` function with the given name that takes in a mutable reference to a slice of the given type
/// and sorts it using the quicksort algorithm while switching to the insertion sort algorithm when the array is small.
// This implementation is the one from <https://github.com/jonhoo/orst/blob/master/src/quicksort.rs> but made const.
macro_rules! const_slice_quicksort {
    ($tpe:ty, $name:ident, $insertion_name:ident) => {
        const_slice_insersion_sort!($tpe, $insertion_name);

        const fn $name(slice: &mut [$tpe]) {
            match slice.len() {
                0 | 1 => return,
                2 => {
                    if slice[0] > slice[1] {
                        (slice[0], slice[1]) = (slice[1], slice[0]);
                    }
                    return;
                }
                3..=10 => {
                    $insertion_name(slice);
                    return;
                }
                _ => {}
            }

            let (pivot, rest) = slice
                .split_first_mut()
                .expect("slice is not empty, as verified above");

            let mut left = 0;
            let mut right = rest.len() - 1;
            while left <= right {
                if rest[left] <= *pivot {
                    left += 1;
                } else if rest[right] > *pivot {
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
            $name(left);
            if let Some((_pivot, right)) = right.split_first_mut() {
                $name(right);
            }
        }
    };
}

/// Defines a `const` function with the given name that sorts an array of the given type with the quicksort algorithm
/// for large arrays and switches to the insertion sort algorithm when the array is small.
macro_rules! const_array_quicksort {
    ($tpe:ty, $name:ident, $partition_name:ident, $insertion_name:ident) => {
        const_array_insertion_sort! {$tpe, $insertion_name}

        const fn $name<const N: usize>(array: [$tpe; N], left: usize, right: usize) -> [$tpe; N] {
            let len = right - left;
            if len <= 1 {
            } else if len <= INSERTION_SIZE {
                return $insertion_name(array);
            } else {
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

/// Defines a `const` function with the given name that sorts an array of the given type with the insertion sort algorithm.
macro_rules! const_array_insertion_sort {
    ($tpe:ty, $name:ident) => {
        const fn $name<const N: usize>(mut array: [$tpe; N]) -> [$tpe; N] {
            if N <= 1 {
                return array;
            }

            let mut i = 1;
            while i < N {
                let mut j = i;
                while j > 0 && array[j - 1] > array[j] {
                    (array[j - 1], array[j]) = (array[j], array[j - 1]);
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
macro_rules! const_slice_insersion_sort {
    ($tpe:ty, $name:ident) => {
        const fn $name(slice: &mut [$tpe]) {
            let n = slice.len();
            if n <= 1 {
                return;
            }

            let mut i = 1;
            while i < n {
                let mut j = i;
                while j > 0 && slice[j - 1] > slice[j] {
                    (slice[j - 1], slice[j]) = (slice[j], slice[j - 1]);
                    j -= 1;
                }
                i += 1;
            }
        }
    };
}

/// Defines the public const quicksort implementations for the given list of types.
/// One function that sorts slices and one function that sorts arrays for each type.
macro_rules! impl_const_quicksort {
    ($($tpe:ty),+) => {
        $(
            paste! {
                #[rustversion::since(1.83.0)]
                const_slice_quicksort!{$tpe, [<qsort_ $tpe _slice>], [<insertion_sort_ $tpe _slice>]}

                const_array_quicksort!{$tpe, [<qsort_ $tpe _array>], [<partition_ $tpe _array>], [<insertion_sort_ $tpe _array>]}

                #[doc = "Sorts the given array of `" $tpe "`s using the quicksort algorithm and returns it."]
                #[doc = ""]
                #[doc = "# Example"]
                #[doc = ""]
                #[doc = "```"]
                #[doc = "# use compile_time_sort::" [<into_sorted_ $tpe _array>] ";"]
                #[doc = "const SORTED_ARRAY: [" $tpe "; 3] = " [<into_sorted_ $tpe _array>] "([0 as " $tpe ", " $tpe "::MAX, " $tpe "::MIN]);"]
                #[doc = ""]
                #[doc = "assert!(SORTED_ARRAY.is_sorted());"]
                #[doc = "```"]
                pub const fn [<into_sorted_ $tpe _array>]<const N: usize>(array: [$tpe; N]) -> [$tpe; N] {
                    if N <= 1 {
                        array
                    } else {
                        [<qsort_ $tpe _array>](array, 0, N)
                    }
                }

                #[rustversion::since(1.83.0)]
                #[doc = "Sorts the given slice of `" $tpe "`s using the quicksort algorithm."]
                #[doc = ""]
                #[doc = "This function is only available on Rust versions 1.83 and above."]
                #[doc = ""]
                #[doc = "# Example"]
                #[doc = ""]
                #[doc = "```"]
                #[doc = "# use compile_time_sort::" [<sort_ $tpe _slice>] ";"]
                #[doc = "const ARRAY: [" $tpe "; 3] = [0 as " $tpe ", " $tpe "::MAX, " $tpe "::MIN];"]
                #[doc = "const SORTED_ARRAY: [" $tpe "; 3]= {"]
                #[doc = "    let mut arr = ARRAY;"]
                #[doc = "    " [<sort_ $tpe _slice>] "(&mut arr);"]
                #[doc = "    arr"]
                #[doc = "};"]
                #[doc = ""]
                #[doc = "assert!(SORTED_ARRAY.is_sorted());"]
                #[doc = "```"]
                pub const fn [<sort_ $tpe _slice>](slice: &mut [$tpe]) {
                    if slice.len() <= 1 {
                        return;
                    } else {
                        [<qsort_ $tpe _slice>](slice);
                    }
                }
            }
        )+
    };
}

// We don't call this macro on `bool`, `u8`, or `i8` because they can be efficiently sorted with counting sort.
impl_const_quicksort! {
    char,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize
}

// endregion: quicksort implementations

// region: counting sort implementations

#[rustversion::since(1.83.0)]
/// Sorts the given slice of `i8`s using the counting sort algorithm.
///
/// This function is only available on Rust versions 1.83 and above.
///
/// # Example
///
/// ```
/// # use compile_time_sort::sort_i8_slice;
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
    if slice.is_empty() || slice.len() == 1 {
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
const_slice_insersion_sort!(i8, insertion_sort_i8_slice);

/// Sorts the given array of `i8`s using the counting sort algorithm and returns it.
///
/// # Example
///
/// ```
/// # use compile_time_sort::into_sorted_i8_array;
/// const SORTED_ARRAY: [i8; 3] = into_sorted_i8_array([0, i8::MAX, i8::MIN]);
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
pub const fn into_sorted_i8_array<const N: usize>(mut array: [i8; N]) -> [i8; N] {
    if N == 0 || N == 1 {
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

const_array_insertion_sort!(i8, insertion_sort_i8_array);

#[rustversion::since(1.83.0)]
/// Sorts the given slice of `u8`s using the counting sort algorithm.
///
/// This function is only available on Rust versions 1.83 and above.
///
/// # Example
///
/// ```
/// # use compile_time_sort::sort_u8_slice;
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
    if slice.is_empty() || slice.len() == 1 {
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
const_slice_insersion_sort!(u8, insertion_sort_u8_slice);

/// Sorts the given array of `u8`s using the counting sort algorithm and returns it.
///
/// # Example
///
/// ```
/// # use compile_time_sort::into_sorted_u8_array;
/// const SORTED_ARRAY: [u8; 3] = into_sorted_u8_array([0, u8::MAX, u8::MIN]);
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
pub const fn into_sorted_u8_array<const N: usize>(mut array: [u8; N]) -> [u8; N] {
    if N == 0 || N == 1 {
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

const_array_insertion_sort!(u8, insertion_sort_u8_array);

#[rustversion::since(1.83.0)]
/// Sorts the given slice of `bool`s using the counting sort algorithm.
///
/// This function is only available on Rust versions 1.83 and above.
///
/// # Example
///
/// ```
/// # use compile_time_sort::sort_bool_slice;
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

/// Sorts the given array of `bool`s using the counting sort algorithm and returns it.
///
/// # Example
///
/// ```
/// # use compile_time_sort::into_sorted_bool_array;
/// const SORTED_ARRAY: [bool; 2] = into_sorted_bool_array([true, false]);
///
/// assert!(SORTED_ARRAY.is_sorted());
/// ```
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

// endregion: counting sort implementations
