// Copyright 2024-2026 Johanna Sörngård
// SPDX-License-Identifier: MIT OR Apache-2.0

#[rustversion::since(1.83.0)]
use quickcheck::quickcheck;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use compile_time_sort::{
    into_sorted_bool_array, into_sorted_char_array, into_sorted_f32_array, into_sorted_f64_array,
    into_sorted_i128_array, into_sorted_i16_array, into_sorted_i32_array, into_sorted_i64_array,
    into_sorted_i8_array, into_sorted_isize_array, into_sorted_str_array, into_sorted_u128_array,
    into_sorted_u16_array, into_sorted_u32_array, into_sorted_u64_array, into_sorted_u8_array,
    into_sorted_u8_slice_array, into_sorted_usize_array,
};

#[cfg(feature = "nested")]
use compile_time_sort::{
    into_sorted_bool_slice_array, into_sorted_i128_slice_array, into_sorted_i16_slice_array,
    into_sorted_i32_slice_array, into_sorted_i64_slice_array, into_sorted_i8_slice_array,
    into_sorted_isize_slice_array, into_sorted_u128_slice_array, into_sorted_u16_slice_array,
    into_sorted_u32_slice_array, into_sorted_u64_slice_array, into_sorted_usize_slice_array,
    sort_bool_slice_slice,
};

#[rustversion::since(1.83.0)]
use compile_time_sort::{
    sort_bool_slice, sort_char_slice, sort_f32_slice, sort_f64_slice, sort_i128_slice,
    sort_i16_slice, sort_i32_slice, sort_i64_slice, sort_i8_slice, sort_isize_slice,
    sort_str_slice, sort_u128_slice, sort_u16_slice, sort_u32_slice, sort_u64_slice, sort_u8_slice,
    sort_u8_slice_slice, sort_usize_slice,
};

#[cfg(feature = "nested")]
#[rustversion::since(1.83.0)]
use compile_time_sort::{
    sort_i128_slice_slice, sort_i16_slice_slice, sort_i32_slice_slice, sort_i64_slice_slice,
    sort_i8_slice_slice, sort_isize_slice_slice, sort_u128_slice_slice, sort_u16_slice_slice,
    sort_u32_slice_slice, sort_u64_slice_slice, sort_usize_slice_slice,
};

use paste::paste;

#[rustversion::since(1.83.0)]
macro_rules! quickcheck_slice_sort {
    ($($tpe:ty),+) => {
        $(
            paste! {
                quickcheck! {
                        fn [<quickcheck_ $tpe _slice>](vec: Vec<$tpe>) -> bool {
                            let mut vec = vec;
                            [<sort_ $tpe _slice>](&mut vec);
                            vec.is_sorted()
                        }
                }
            }
        )+
    };
}

macro_rules! test_unsigned_integer {
    ($($tpe:ty),+) => {
        $(
            paste! {
                #[test]
                fn [<test_sort_ $tpe>]() {
                    const TINY_ARR: [$tpe; 1] = [1];
                    const SORTED_TINY_ARR: [$tpe; 1] = [<into_sorted_ $tpe _array>](TINY_ARR);
                    assert!(SORTED_TINY_ARR.is_sorted());


                    const REV_ARRAY: [$tpe; 3] = [3, 2, 1];
                    const SORTED_REV_ARRAY: [$tpe; 3] = [<into_sorted_ $tpe _array>](REV_ARRAY);
                    assert!(SORTED_REV_ARRAY.is_sorted());

                    const CONST_ARRAY: [$tpe; 3] = [2, 2, 2];
                    const SORTED_CONST_ARRAY: [$tpe; 3] = [<into_sorted_ $tpe _array>](CONST_ARRAY);
                    assert!(SORTED_CONST_ARRAY.is_sorted());

                    const INCORRECT_LAST: [$tpe; 4] = [1, 1, 1, 0];
                    const SORTED_INCORRECT_LAST: [$tpe; 4] = [<into_sorted_ $tpe _array>](INCORRECT_LAST);
                    assert!(SORTED_INCORRECT_LAST.is_sorted());

                    const JUST_ZEROS_ARRAY: [$tpe; 100] = [0; 100];
                    const SORTED_JUST_ZEROS_ARRAY: [$tpe; 100] = [<into_sorted_ $tpe _array>](JUST_ZEROS_ARRAY);
                    assert!(SORTED_JUST_ZEROS_ARRAY.is_sorted());

                    const EMPTY_ARRAY: [$tpe; 0] = [];
                    const SORTED_EMPTY_ARRAY: [$tpe; 0] = [<into_sorted_ $tpe _array>](EMPTY_ARRAY);
                    assert!(SORTED_EMPTY_ARRAY.is_sorted());

                    const SINGLETON_ARRAY: [$tpe; 1] = [1];
                    const SORTED_SINGLETON_ARRAY: [$tpe; 1] = [<into_sorted_ $tpe _array>](SINGLETON_ARRAY);
                    assert!(SORTED_SINGLETON_ARRAY.is_sorted());

                    // Test alternating pattern
                    const ALTERNATING: [$tpe; 4] = [2, 1, 2, 1];
                    const SORTED_ALTERNATING: [$tpe; 4] = [<into_sorted_ $tpe _array>](ALTERNATING);
                    assert!(SORTED_ALTERNATING.is_sorted());

                    // Test already sorted array
                    const SORTED: [$tpe; 4] = [1, 2, 3, 4];
                    const STILL_SORTED: [$tpe; 4] = [<into_sorted_ $tpe _array>](SORTED);
                    assert!(STILL_SORTED.is_sorted());

                    // Test reverse sorted array with duplicates
                    const REV_WITH_DUPS: [$tpe; 6] = [5, 4, 4, 3, 2, 1];
                    const SORTED_REV_DUPS: [$tpe; 6] = [<into_sorted_ $tpe _array>](REV_WITH_DUPS);
                    assert!(SORTED_REV_DUPS.is_sorted());

                    // Test array with all same values except one
                    const ALL_SAME_ONE_DIFF: [$tpe; 5] = [2, 2, 1, 2, 2];
                    const SORTED_SAME_DIFF: [$tpe; 5] = [<into_sorted_ $tpe _array>](ALL_SAME_ONE_DIFF);
                    assert!(SORTED_SAME_DIFF.is_sorted());

                    let mut rng = SmallRng::from_seed([0b01010101; 32]);

                    let random_array: [$tpe; 500] = core::array::from_fn(|_| rng.gen());

                    let sorted_array = [<into_sorted_ $tpe _array>](random_array);
                    assert!(sorted_array.is_sorted());

                    assert!([<into_sorted_ $tpe _array>](sorted_array).is_sorted());
                    assert!([<into_sorted_ $tpe _array>]([1; 500]).is_sorted());
                }

                #[rustversion::since(1.83.0)]
                #[test]
                fn [<test_slice_sort_ $tpe _cases>]() {
                    const BIG_IDENTICAL: [$tpe; 500] = {
                        let mut arr = [42; 500];
                        [<sort_ $tpe _slice>](&mut arr);
                        arr
                    };

                    const REVERSE_SORTED: [$tpe; 255] = {
                        let mut arr = [0; 255];
                        let mut i = arr.len();
                        while i > 0 {
                            arr[i-1] = i as $tpe;
                            i -= 1
                        }
                        [<sort_ $tpe _slice>](&mut arr);
                        arr
                    };

                    assert!(BIG_IDENTICAL.is_sorted());
                    assert!(REVERSE_SORTED.is_sorted());
                }
            }
        )+
    };
}

macro_rules! test_signed_integer {
    ($($tpe:ty),+) => {
        $(
            paste! {
                #[test]
                fn [<test_small_negative_sort_ $tpe>]() {
                    const ARRAY_WITH_NEGATIVES: [$tpe; 3] = [0, -1, 2];
                    const SORTED_ARRAY_WITH_NEGATIVES: [$tpe; 3] = [<into_sorted_ $tpe _array>](ARRAY_WITH_NEGATIVES);

                    assert!(SORTED_ARRAY_WITH_NEGATIVES.is_sorted());

                    const FOUND_BY_QUICKCHECK: [$tpe; 27] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
                    const SORTED_QUICKCHECK_CASE: [$tpe; 27] = [<into_sorted_ $tpe _array>](FOUND_BY_QUICKCHECK);

                    assert!(SORTED_QUICKCHECK_CASE.is_sorted());

                    const FOUND_BY_QUICKCHECK_2: [$tpe; 56] = [-4, 0, -1, 0, -3, 0, 3, 0, 0, 1, 0, 3, 0, -2, 0, 0, -4, 0, 3, 0, -3, 0, 0, 0, 0, 2, 3, 1, -2, -3, -3, 2, 2, -3, -2, -3, -3, 1, -4, -3, 2, -2, -2, -3, -4, -3, -2, -3, 0, -1, 2, -3, -3, -3, -2, 3];
                    const SORTED_QUICKCHECK_CASE_2: [$tpe; 56] = [<into_sorted_ $tpe _array>](FOUND_BY_QUICKCHECK_2);

                    assert!(SORTED_QUICKCHECK_CASE_2.is_sorted());
                }

                #[test]
                #[rustversion::since(1.83.0)]
                fn [<test_sort_ $tpe _slice_quickcheck_cases>]() {
                    const SORTED_QUICKCHECK_CASE_1: [$tpe; 27] = {
                        let mut quickcheck_case_1 = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

                        [<sort_ $tpe _slice>](&mut quickcheck_case_1);

                        quickcheck_case_1
                    };

                    const SORTED_QUICKCHECK_CASE_2: [$tpe; 56] = {
                        let mut quickcheck_case_2 = [-4, 0, -1, 0, -3, 0, 3, 0, 0, 1, 0, 3, 0, -2, 0, 0, -4, 0, 3, 0, -3, 0, 0, 0, 0, 2, 3, 1, -2, -3, -3, 2, 2, -3, -2, -3, -3, 1, -4, -3, 2, -2, -2, -3, -4, -3, -2, -3, 0, -1, 2, -3, -3, -3, -2, 3];

                        [<sort_ $tpe _slice>](&mut quickcheck_case_2);

                        quickcheck_case_2
                    };

                    const SORTED_QUICKCHECK_CASE_3: [$tpe; 27] = {
                        let mut quickcheck_case_3 = [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

                        [<sort_ $tpe _slice>](&mut quickcheck_case_3);

                        quickcheck_case_3
                    };

                    assert!(SORTED_QUICKCHECK_CASE_1.is_sorted());
                    assert!(SORTED_QUICKCHECK_CASE_2.is_sorted());
                    assert!(SORTED_QUICKCHECK_CASE_3.is_sorted());
                }

                // Also run all the tests for unsigned integers on the signed integers
                test_unsigned_integer! {$tpe}
            }
        )+
    };
}

test_unsigned_integer! { u8, u16, u32, u64, u128, usize }

test_signed_integer! { i8, i16, i32, i64, i128, isize }

#[rustversion::since(1.83.0)]
quickcheck_slice_sort! {
    u8, i8,
    u16, i16,
    u32, i32,
    u64, i64,
    u128, i128,
    usize, isize,
    char,
    bool
}

#[cfg(feature = "nested")]
#[test]
fn test_sort_bool_slice_arrays() {
    const ARR: [&[bool]; 3] = [&[false, false], &[true], &[true, true]];
    const SORTED_ARR: [&[bool]; 3] = into_sorted_bool_slice_array(ARR);

    assert!(SORTED_ARR.is_sorted());
}

#[cfg(feature = "nested")]
#[rustversion::since(1.83.0)]
#[test]
fn test_sort_bool_slice_slices() {
    const SORTED_ARR: [&[bool]; 3] = {
        let mut arr: [&[bool]; 3] = [&[false], &[true], &[true, true]];
        sort_bool_slice_slice(&mut arr);
        arr
    };

    assert!(SORTED_ARR.is_sorted());
}

macro_rules! test_unsigned_slices {
    ($($tpe:ty),+) => {
        $(
            paste! {
                #[test]
                fn [<test_sort_ $tpe _slice_array>]() {
                    const ARR: [&[$tpe]; 4] = [&[0, 1], &[0, 0], &[1, 0], &[1, 1]];
                    const SORTED_ARR: [&[$tpe]; 4] = [<into_sorted_ $tpe _slice_array>](ARR);

                    assert!(SORTED_ARR.is_sorted());
                }

                #[test]
                fn [<test_sort_long_identical_ $tpe _slice_array>]() {
                    const ARR: [&[$tpe]; 500] = [&[0]; 500];
                    const SORTED_ARR: [&[$tpe]; 500] = [<into_sorted_ $tpe _slice_array>](ARR);

                    assert!(SORTED_ARR.is_sorted());
                }

                #[rustversion::since(1.83.0)]
                #[test]
                fn [<test_sort_ $tpe _slice_slice>]() {
                     const SORTED_ARR: [&[$tpe]; 4] = {
                        let mut arr: [&[$tpe]; 4] = [&[0, 1], &[0, 0], &[1, 0], &[1, 1]];
                        [<sort_ $tpe _slice_slice>](&mut arr);
                        arr
                    };

                    assert!(SORTED_ARR.is_sorted());
                }

                 #[rustversion::since(1.83.0)]
                #[test]
                fn [<test_sort_long_identical_ $tpe _slice_slice>]() {
                    const SORTED_ARR: [&[$tpe]; 500] = {
                        let mut arr: [&[$tpe]; 500] = [&[]; 500];
                        [<sort_ $tpe _slice_slice>](&mut arr);
                        arr
                    };

                    assert!(SORTED_ARR.is_sorted());
                }
            }
        )+
    };
}

#[cfg(feature = "nested")]
macro_rules! test_signed_slices {
    ($($tpe:ty),+) => {
        $(
            paste! {
                #[test]
                fn [<test_sort_ $tpe _slice_array_with_negatives>]() {
                    const ARR: [&[$tpe]; 8] = [&[0, 1], &[0, 0], &[1, 0], &[1, 1], &[0, -1], &[0, 0], &[-1, 0], &[-1, -1]];
                    const SORTED_ARR: [&[$tpe]; 8] = [<into_sorted_ $tpe _slice_array>](ARR);

                    assert!(SORTED_ARR.is_sorted());
                }

                #[rustversion::since(1.83.0)]
                #[test]
                fn [<test_sort_ $tpe _slice_slice_with_negatives>]() {
                     const SORTED_ARR: [&[$tpe]; 8] = {
                        let mut arr: [&[$tpe]; 8] = [&[0, 1], &[0, 0], &[1, 0], &[1, 1], &[0, -1], &[0, 0], &[-1, 0], &[-1, -1]];
                        [<sort_ $tpe _slice_slice>](&mut arr);
                        arr
                    };

                    assert!(SORTED_ARR.is_sorted());
                }
            }
        )+

        test_unsigned_slices! { $($tpe),+ }
    };
}

test_unsigned_slices! { u8 }

#[cfg(feature = "nested")]
test_unsigned_slices! {
    u16, u32, u64, u128, usize
}

#[cfg(feature = "nested")]
test_signed_slices! {
    i8, i16, i32, i64, i128, isize
}

#[test]
fn test_sort_str_array() {
    const ARR: [&str; 4] = ["abc", "abd", "aaaaa", "l"];
    const SORTED_ARR: [&str; 4] = into_sorted_str_array(ARR);

    assert!(SORTED_ARR.is_sorted());
}

#[rustversion::since(1.83.0)]
#[test]
fn test_sort_str_slice() {
    const SORTED_ARR: [&str; 4] = {
        let mut arr: [&str; 4] = ["abc", "abd", "aaaaa", "l"];
        sort_str_slice(&mut arr);
        arr
    };

    assert!(SORTED_ARR.is_sorted());
}

#[test]
fn test_sort_bool() {
    const ARR: [bool; 4] = [false, true, false, true];
    const SORTED_ARR: [bool; 4] = into_sorted_bool_array(ARR);

    assert_eq!(SORTED_ARR, [false, false, true, true]);
}

#[test]
fn test_char_sort() {
    const SORTED_ARR: [char; 4] = into_sorted_char_array(['a', '#', '\n', 'A']);

    assert_eq!(SORTED_ARR, ['\n', '#', 'A', 'a'])
}

#[test]
fn test_f32_into_sorted() {
    const ARR: [f32; 5] = [3.0, 1.0, -0.0, 0.0, f32::MIN];
    const SORTED_ARR: [f32; 5] = into_sorted_f32_array(ARR);

    const CONSTANT_ARR: [f32; 3] = [1.0, 1.0, 1.0];
    const SORTED_CONSTANT_ARR: [f32; 3] = into_sorted_f32_array(CONSTANT_ARR);

    const NAN_ARR: [f32; 3] = [f32::NAN, 1.0, -2.0];
    const SORTED_NAN_ARR: [f32; 3] = into_sorted_f32_array(NAN_ARR);

    const INF_ARR: [f32; 4] = [f32::INFINITY, 1.0, f32::NEG_INFINITY, 0.0];
    const SORTED_INF_ARR: [f32; 4] = into_sorted_f32_array(INF_ARR);

    assert!(SORTED_ARR.is_sorted());
    assert!(SORTED_CONSTANT_ARR.is_sorted());
    assert!(
        SORTED_NAN_ARR[0].is_nan() && SORTED_NAN_ARR[1..].is_sorted()
            || SORTED_NAN_ARR[2].is_nan() && SORTED_NAN_ARR[..2].is_sorted()
    );
    assert!(SORTED_INF_ARR.is_sorted());

    let mut rng = SmallRng::from_seed([0b01010101; 32]);

    let random_array: [f32; 500] = core::array::from_fn(|_| rng.gen());

    let sorted_array = into_sorted_f32_array(random_array);
    assert!(sorted_array.is_sorted());
}

#[rustversion::since(1.83.0)]
#[test]
fn test_f32_sort_slice() {
    const BIG_IDENTICAL: [f32; 500] = {
        let mut arr = [42.0; 500];
        sort_f32_slice(&mut arr);
        arr
    };

    const SORTED_ARR: [f32; 500] = {
        let mut arr = [0.0; 500];
        let mut i = 0;
        while i < arr.len() {
            arr[i] = i as f32;
            i += 1;
        }

        sort_f32_slice(&mut arr);

        arr
    };

    assert!(BIG_IDENTICAL.is_sorted());
    assert!(SORTED_ARR.is_sorted());

    let mut rng = SmallRng::from_seed([0b01010101; 32]);
    let mut random_array: [f32; 500] = core::array::from_fn(|_| rng.gen());
    sort_f32_slice(&mut random_array);
    assert!(random_array.is_sorted());

    let mut all_same = vec![42.0; 500];
    sort_f32_slice(&mut all_same);
    assert!(all_same.is_sorted());
}

#[test]
fn test_f64_into_sorted() {
    const ARR: [f64; 5] = [3.0, 1.0, -0.0, 0.0, f64::MIN];
    const SORTED_ARR: [f64; 5] = into_sorted_f64_array(ARR);

    const CONSTANT_ARR: [f64; 3] = [1.0, 1.0, 1.0];
    const SORTED_CONSTANT_ARR: [f64; 3] = into_sorted_f64_array(CONSTANT_ARR);

    const NAN_ARR: [f64; 3] = [f64::NAN, 1.0, -2.0];
    const SORTED_NAN_ARR: [f64; 3] = into_sorted_f64_array(NAN_ARR);

    const INF_ARR: [f64; 4] = [f64::INFINITY, 1.0, f64::NEG_INFINITY, 0.0];
    const SORTED_INF_ARR: [f64; 4] = into_sorted_f64_array(INF_ARR);

    assert!(SORTED_ARR.is_sorted());
    assert!(SORTED_CONSTANT_ARR.is_sorted());
    assert!(
        SORTED_NAN_ARR[0].is_nan() && SORTED_NAN_ARR[1..].is_sorted()
            || SORTED_NAN_ARR[2].is_nan() && SORTED_NAN_ARR[..2].is_sorted()
    );
    assert!(SORTED_INF_ARR.is_sorted());

    let mut rng = SmallRng::from_seed([0b01010101; 32]);

    let random_array: [f64; 500] = core::array::from_fn(|_| rng.gen());

    let sorted_array = into_sorted_f64_array(random_array);
    assert!(sorted_array.is_sorted());
}

#[rustversion::since(1.83.0)]
#[test]
fn test_f64_sort_slice() {
    const BIG_IDENTICAL: [f64; 500] = {
        let mut arr = [42.0; 500];
        sort_f64_slice(&mut arr);
        arr
    };

    const SORTED_ARR: [f64; 500] = {
        let mut arr = [0.0; 500];
        let mut i = 0;
        while i < arr.len() {
            arr[i] = i as f64;
            i += 1;
        }

        sort_f64_slice(&mut arr);

        arr
    };

    assert!(BIG_IDENTICAL.is_sorted());
    assert!(SORTED_ARR.is_sorted());

    let mut rng = SmallRng::from_seed([0b01010101; 32]);
    let mut random_array: [f64; 500] = core::array::from_fn(|_| rng.gen());
    sort_f64_slice(&mut random_array);
    assert!(random_array.is_sorted());

    let mut all_same = vec![42.0; 500];
    sort_f64_slice(&mut all_same);
    assert!(all_same.is_sorted());
}

#[rustversion::since(1.83.0)]
quickcheck! {
    fn quickcheck_f32_slice(vec: Vec<f32>) -> bool {
        let mut vec = vec;
        sort_f32_slice(&mut vec);
        vec.is_sorted_by(|a, b| matches!(a.total_cmp(&b), std::cmp::Ordering::Less | std::cmp::Ordering::Equal))
    }

    fn quickcheck_f64_slice(vec: Vec<f64>) -> bool {
        let mut vec = vec;
        sort_f64_slice(&mut vec);
        vec.is_sorted_by(|a, b| matches!(a.total_cmp(&b), std::cmp::Ordering::Less | std::cmp::Ordering::Equal))
    }
}
