//! # const_sort
//!
//! This crate provides functions for sorting arrays and slices of primitives in `const` contexts.
//!
//! Depending on how you are doing `const` evaluation, sorting an array by value
//! or by reference might be useful. This crate provides functions for both.
//!
//! Arrays of `bool`s, `u8`s, and `i8`s are sorted with counting sort while arrays of other types
//! are sorted with quicksort.
//!
//! # Examples
//!
//! Sort an array by value:
//!
//! ```
//! use const_sort::sort_i32_array;
//!
//! const ARRAY: [i32; 5] = [-3, 3, 2, i32::MAX, 0];
//! const SORTED_ARRAY: [i32; 5] = sort_i32_array(ARRAY);
//!
//! assert_eq!(SORTED_ARRAY, [-3, 0, 2, 3, i32::MAX]);
//! ```
//!
//! Sort an array by reference:
//!
//! ```
//! use const_sort::sort_i32_slice;
//!
//! const SORTED_ARRAY: [i32; 5] = {
//!     let mut arr = [5, i32::MIN, 0, -2, 0];
//!     sort_i32_slice(&mut arr);
//!     arr
//! };
//!
//! assert_eq!(SORTED_ARRAY, [i32::MIN, -2, 0, 0, 5]);
//! ```

macro_rules! impl_const_quicksort {
    ($pub_name_array:ident, $pub_name_slice:ident, $qsort_name:ident, $tpe:ty, $tpe_name: literal) => {
        const fn $qsort_name(slice: &mut [$tpe], left: usize, right: usize) {
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
                $qsort_name(slice, left, l - 1);
            }
            if right > l {
                $qsort_name(slice, l + 1, right);
            }
        }

        #[doc = concat!("Sorts the given array of `", $tpe_name, "`s using the quicksort algorithm")]
        pub const fn $pub_name_array<const N: usize>(mut arr: [$tpe; N]) -> [$tpe; N] {
            let last_index = arr.len() - 1;
            $qsort_name(&mut arr, 0, last_index);
            arr
        }

        #[doc = concat!("Sorts the given slice of `", $tpe_name, "`s using the quicksort algorithm")]
        pub const fn $pub_name_slice(slice: &mut [$tpe]) {
            let last_index = slice.len() - 1;
            $qsort_name(slice, 0, last_index);
        }
    };
}

impl_const_quicksort!(sort_u16_array, sort_u16_slice, qsort_u16, u16, "u16");
impl_const_quicksort!(sort_i16_array, sort_i16_slice, qsort_i16, i16, "i16");
impl_const_quicksort!(sort_u32_array, sort_u32_slice, qsort_u32, u32, "u32");
impl_const_quicksort!(sort_i32_array, sort_i32_slice, qsort_i32, i32, "i32");
impl_const_quicksort!(sort_u64_array, sort_u64_slice, qsort_u64, u64, "u64");
impl_const_quicksort!(sort_i64_array, sort_i64_slice, qsort_i64, i64, "i64");
impl_const_quicksort!(
    sort_usize_array,
    sort_usize_slice,
    qsort_usize,
    usize,
    "usize"
);
impl_const_quicksort!(
    sort_isize_array,
    sort_isize_slice,
    qsort_isize,
    isize,
    "isize"
);

/// Sorts the given slice of `i8`s using the counting sort algorithm.
pub const fn sort_i8_slice(slice: &mut [i8]) {
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
pub const fn sort_i8_array<const N: usize>(mut array: [i8; N]) -> [i8; N] {
    sort_i8_slice(&mut array);
    array
}

/// Sorts the given slice of `u8`s using the counting sort algorithm.
pub const fn sort_u8_slice(slice: &mut [u8]) {
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
pub const fn sort_u8_array<const N: usize>(mut array: [u8; N]) -> [u8; N] {
    sort_u8_slice(&mut array);
    array
}

/// Sorts the given slice of `bool`s using the counting sort algorithm.
pub const fn sort_bool_slice(slice: &mut [bool]) {
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
pub const fn sort_bool_array<const N: usize>(mut array: [bool; N]) -> [bool; N] {
    sort_bool_slice(&mut array);
    array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_i32() {
        const REV_ARRAY: [i32; 3] = [3, 2, 1];
        const SORTED_REV_ARRAY: [i32; 3] = sort_i32_array(REV_ARRAY);
        const CONST_ARRAY: [i32; 3] = [2, 2, 2];
        const SORTED_CONST_ARRAY: [i32; 3] = sort_i32_array(CONST_ARRAY);
        const ARRAY_WITH_NEGATIVES: [i32; 3] = [0, -1, 2];
        const SORTED_ARRAY_WITH_NEGATIVES: [i32; 3] = sort_i32_array(ARRAY_WITH_NEGATIVES);

        let arr = const {
            let mut arr = REV_ARRAY;
            sort_i32_slice(&mut arr);
            arr
        };

        assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
        assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
        assert_eq!(SORTED_ARRAY_WITH_NEGATIVES, [-1, 0, 2]);
        assert_eq!(arr, SORTED_REV_ARRAY);
    }

    #[test]
    fn test_sort_u32() {
        const REV_ARRAY: [u32; 3] = [3, 2, 1];
        const SORTED_REV_ARRAY: [u32; 3] = sort_u32_array(REV_ARRAY);
        const CONST_ARRAY: [u32; 3] = [2, 2, 2];
        const SORTED_CONST_ARRAY: [u32; 3] = sort_u32_array(CONST_ARRAY);

        let arr = const {
            let mut arr = REV_ARRAY;
            sort_u32_slice(&mut arr);
            arr
        };

        assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
        assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
        assert_eq!(arr, SORTED_REV_ARRAY)
    }

    #[test]
    fn test_sort_bool() {
        const ARR: [bool; 4] = [false, true, false, true];
        const SORTED_ARR: [bool; 4] = sort_bool_array(ARR);

        let arr = {
            let mut arr = [true, false, true, false];
            sort_bool_slice(&mut arr);
            arr
        };

        assert_eq!(SORTED_ARR, [false, false, true, true]);
        assert_eq!(arr, [false, false, true, true]);
    }

    #[test]
    fn test_u8_sort() {
        const ARR: [u8; 5] = [8, 1, u8::MAX, 5, 0];
        const SORTED_ARR: [u8; 5] = sort_u8_array(ARR);

        assert_eq!(SORTED_ARR, [0, 1, 5, 8, u8::MAX]);
    }

    #[test]
    fn test_i8_sort() {
        const ARR: [i8; 5] = [-2, 50, 0, 5, -50];
        const SORTED_ARR: [i8; 5] = sort_i8_array(ARR);

        let sorted_arr = const {
            let mut arr = [i8::MIN; 100];
            sort_i8_slice(&mut arr);
            arr
        };

        assert_eq!(SORTED_ARR, [-50, -2, 0, 5, 50]);
        assert_eq!(sorted_arr, [i8::MIN; 100]);
    }
}
