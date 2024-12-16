macro_rules! const_sort_t {
    ($($pub_name:ident, $qsort_name:ident, $tpe:ty);*) => {
        $(
            const fn $qsort_name(slice: &mut [$tpe], left: usize, right: usize) {
                let mut l = left;
                let mut r = right;

                let p1_i = left;
                let p2_i = (left + right) / 2;
                let p3_i = right;
                let mut pivot_i = if slice[p1_i] < slice[p2_i] {
                    if slice[p3_i] < slice[p2_i] {
                        if slice[p1_i] < slice[p3_i] {
                            p3_i
                        } else {
                            p1_i
                        }
                    } else {
                        p2_i
                    }
                } else { // slice[p2_i] <= slice[p1_i]
                    if slice[p3_i] < slice[p1_i] {
                        if slice[p2_i] < slice[p3_i] {
                            p3_i
                        } else {
                            p2_i
                        }
                    } else {
                        p1_i
                    }
                };

                while l < r {
                    while (slice[pivot_i] < slice[r]) && (l < r) {
                        r = r - 1;
                    }
                    if l != r {
                        (slice[pivot_i], slice[r]) = (slice[r], slice[pivot_i]);
                        pivot_i = r;
                    }
                    while (slice[l] < slice[pivot_i]) && (l < r) {
                        l = l + 1;
                    }
                    if l != r {
                        (slice[pivot_i], slice[l]) = (slice[l], slice[pivot_i]);
                        pivot_i = l;
                    }
                    if l != r && slice[l] == slice[r] {
                        // Break out of infinite loops
                        // if every element is the same.
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

            /// Sorts the given array using the quick-sort algorithm.
            pub const fn $pub_name<const N: usize>(mut arr: [$tpe; N]) -> [$tpe; N] {
                let len = arr.len() - 1;
                $qsort_name(&mut arr, 0, len);
                arr
            }
        )*
    };
}

const_sort_t!(
    const_sort_usize, qsort_usize, usize;
    const_sort_isize, qsort_isize, isize;
    const_sort_u8, qsort_u8, u8;
    const_sort_i8, qsort_i8, i8;
    const_sort_u16, qsort_u16, u16;
    const_sort_i16, qsort_i16, i16;
    const_sort_u32, qsort_u32, u32;
    const_sort_i32, qsort_i32, i32;
    const_sort_u64, qsort_u64, u64;
    const_sort_i64, qsort_i64, i64
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_i32() {
        const REV_ARRAY: [i32; 3] = [3, 2, 1];
        const SORTED_REV_ARRAY: [i32; 3] = const_sort_i32(REV_ARRAY);
        const CONST_ARRAY: [i32; 3] = [2, 2, 2];
        const SORTED_CONST_ARRAY: [i32; 3] = const_sort_i32(CONST_ARRAY);
        const ARRAY_WITH_NEGATIVES: [i32; 3] = [0, -1, 2];
        const SORTED_ARRAY_WITH_NEGATIVES: [i32; 3] = const_sort_i32(ARRAY_WITH_NEGATIVES);

        assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
        assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
        assert_eq!(SORTED_ARRAY_WITH_NEGATIVES, [-1, 0, 2]);
    }
}
