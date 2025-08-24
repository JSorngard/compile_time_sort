//! This module contains functions to sort slices of [`f32`] and [`f64`] using a quicksort algorithm.
//! These functions use a `const` implementation of [`f32::total_cmp`] and [`f64::total_cmp`] to implement
//! a total order over floating-point numbers.

#[rustversion::since(1.83.0)]
use crate::INSERTION_SIZE;

#[rustversion::since(1.83.0)]
/// Sorts the given slice of `f32`s using the quicksort algorithm.
///
/// This function uses a `const` implementation of [`f32::total_cmp`] to implement
/// a total order over `f32`s.
///
/// Switches to insertion sort when the slice is small.
///
/// This function is only available on Rust versions 1.83 and above.
///
/// # Examples
///
/// ```rust
/// use compile_time_sort::sort_f32_slice;
/// let mut arr = [3.0, 1.0, 4.0, 1.5];
/// sort_f32_slice(&mut arr);
/// assert_eq!(arr, [1.0, 1.5, 3.0, 4.0]);
/// ```
pub const fn sort_f32_slice(slice: &mut [f32]) {
    // SAFETY: This is safe because `FloatOrdF32` is `repr(transparent)` over `f32`.
    let slice_ord: &mut [FloatOrdF32] = unsafe { core::mem::transmute(slice) };
    quicksort_f32_ord(slice_ord);
}

#[rustversion::since(1.83.0)]
pub const fn sort_f64_slice(slice: &mut [f64]) {
    // SAFETY: This is safe because `FloatOrdF64` is `repr(transparent)` over `f64`.
    let slice_ord: &mut [FloatOrdF64] = unsafe { core::mem::transmute(slice) };
    quicksort_f64_ord(slice_ord);
}

#[rustversion::since(1.83.0)]
const fn quicksort_f32_ord(slice: &mut [FloatOrdF32]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0].greater_than(&slice[1]) {
                (slice[0], slice[1]) = (slice[1], slice[0]);
            }
            return;
        }
        3..=INSERTION_SIZE => {
            insertion_sort_f32_ord(slice);
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
        if rest[left].less_or_equal(pivot) {
            left += 1;
        } else if rest[right].greater_than(pivot) {
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
    quicksort_f32_ord(left);
    if let Some((_pivot, right)) = right.split_first_mut() {
        quicksort_f32_ord(right);
    }
}

#[rustversion::since(1.83.0)]
const fn insertion_sort_f32_ord(slice: &mut [FloatOrdF32]) {
    let n = slice.len();
    if n <= 1 {
        return;
    }

    // SAFETY: This is safe because `FloatOrdF32` is `repr(transparent)` over `f32`.
    let slice_ord: &mut [FloatOrdF32] = unsafe { core::mem::transmute(slice) };

    let mut i = 1;
    while i < n {
        let mut j = i;
        while j > 0 && slice_ord[j - 1].greater_than(&slice_ord[j]) {
            (slice_ord[j - 1], slice_ord[j]) = (slice_ord[j], slice_ord[j - 1]);
            j -= 1;
        }
        i += 1;
    }
}

#[rustversion::since(1.83.0)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct FloatOrdF32(f32);

#[rustversion::since(1.83.0)]
use core::cmp::Ordering;

#[rustversion::since(1.83.0)]
impl FloatOrdF32 {
    #[inline]
    const fn greater_than(&self, other: &FloatOrdF32) -> bool {
        matches!(self.total_cmp(other), Ordering::Greater)
    }

    #[inline]
    const fn less_or_equal(&self, other: &FloatOrdF32) -> bool {
        !self.greater_than(other)
    }

    #[inline]
    /// `const` implementation of [`f32::total_cmp`].
    const fn total_cmp(&self, other: &FloatOrdF32) -> Ordering {
        let mut left = self.0.to_bits() as i32;

        let mut right = other.0.to_bits() as i32;

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
}

#[rustversion::since(1.83.0)]
const fn quicksort_f64_ord(slice: &mut [FloatOrdF64]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0].greater_than(&slice[1]) {
                (slice[0], slice[1]) = (slice[1], slice[0]);
            }
            return;
        }
        3..=INSERTION_SIZE => {
            insertion_sort_f64_ord(slice);
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
        if rest[left].less_or_equal(pivot) {
            left += 1;
        } else if rest[right].greater_than(pivot) {
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
    quicksort_f64_ord(left);
    if let Some((_pivot, right)) = right.split_first_mut() {
        quicksort_f64_ord(right);
    }
}

#[rustversion::since(1.83.0)]
const fn insertion_sort_f64_ord(slice: &mut [FloatOrdF64]) {
    let n = slice.len();
    if n <= 1 {
        return;
    }

    // SAFETY: This is safe because `FloatOrdF64` is `repr(transparent)` over `f64`.
    let slice_ord: &mut [FloatOrdF64] = unsafe { core::mem::transmute(slice) };

    let mut i = 1;
    while i < n {
        let mut j = i;
        while j > 0 && slice_ord[j - 1].greater_than(&slice_ord[j]) {
            (slice_ord[j - 1], slice_ord[j]) = (slice_ord[j], slice_ord[j - 1]);
            j -= 1;
        }
        i += 1;
    }
}

#[rustversion::since(1.83.0)]
#[repr(transparent)]
#[derive(Copy, Clone)]
struct FloatOrdF64(f64);

#[rustversion::since(1.83.0)]
impl FloatOrdF64 {
    #[inline]
    const fn greater_than(&self, other: &FloatOrdF64) -> bool {
        matches!(self.total_cmp(other), Ordering::Greater)
    }

    #[inline]
    const fn less_or_equal(&self, other: &FloatOrdF64) -> bool {
        !self.greater_than(other)
    }

    #[inline]
    /// `const` implementation of [`f32::total_cmp`].
    const fn total_cmp(&self, other: &FloatOrdF64) -> Ordering {
        let mut left = self.0.to_bits() as i64;
        let mut right = other.0.to_bits() as i64;

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
}
