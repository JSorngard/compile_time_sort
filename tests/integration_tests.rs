#[rustversion::since(1.83.0)]
use quickcheck::quickcheck;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use compile_time_sort::{
    into_sorted_bool_array, into_sorted_char_array, into_sorted_i128_array, into_sorted_i16_array,
    into_sorted_i32_array, into_sorted_i64_array, into_sorted_i8_array, into_sorted_isize_array,
    into_sorted_u128_array, into_sorted_u16_array, into_sorted_u32_array, into_sorted_u64_array,
    into_sorted_u8_array, into_sorted_usize_array,
};

#[rustversion::since(1.83.0)]
use compile_time_sort::{
    sort_bool_slice, sort_char_slice, sort_i128_slice, sort_i16_slice, sort_i32_slice,
    sort_i64_slice, sort_i8_slice, sort_isize_slice, sort_u128_slice, sort_u16_slice,
    sort_u32_slice, sort_u64_slice, sort_u8_slice, sort_usize_slice,
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
