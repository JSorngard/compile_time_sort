#[cfg(feature = "sort_slices")]
use rand::{rngs::SmallRng, Rng, SeedableRng};

use compile_time_sort::{
    into_sorted_bool_array, into_sorted_char_array, into_sorted_i128_array, into_sorted_i16_array,
    into_sorted_i32_array, into_sorted_i64_array, into_sorted_i8_array, into_sorted_u128_array,
    into_sorted_u16_array, into_sorted_u32_array, into_sorted_u64_array, into_sorted_u8_array,
};

#[cfg(feature = "sort_slices")]
use compile_time_sort::{
    sort_bool_slice, sort_i128_slice, sort_i16_slice, sort_i32_slice, sort_i64_slice,
    sort_i8_slice, sort_u128_slice, sort_u16_slice, sort_u32_slice, sort_u64_slice, sort_u8_slice,
};

macro_rules! test_unsigned_integer {
    ($tpe:ty, $fn_name:ident, $array_sort_name:ident, $slice_sort_name:ident) => {
        #[test]
        fn $fn_name() {
            const REV_ARRAY: [$tpe; 3] = [3, 2, 1];
            const SORTED_REV_ARRAY: [$tpe; 3] = $array_sort_name(REV_ARRAY);

            const CONST_ARRAY: [$tpe; 3] = [2, 2, 2];
            const SORTED_CONST_ARRAY: [$tpe; 3] = $array_sort_name(CONST_ARRAY);

            const JUST_ZEROS_ARRAY: [$tpe; 100] = [0; 100];
            const SORTED_JUST_ZEROS_ARRAY: [$tpe; 100] = $array_sort_name(JUST_ZEROS_ARRAY);

            const EMPTY_ARRAY: [$tpe; 0] = [];
            const SORTED_EMPTY_ARRAY: [$tpe; 0] = $array_sort_name(EMPTY_ARRAY);

            const SINGLETON_ARRAY: [$tpe; 1] = [1];
            const SORTED_SINGLETON_ARRAY: [$tpe; 1] = $array_sort_name(SINGLETON_ARRAY);

            assert_eq!(SORTED_EMPTY_ARRAY, []);

            assert_eq!(SORTED_SINGLETON_ARRAY, [1]);

            assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);

            assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);

            assert_eq!(SORTED_JUST_ZEROS_ARRAY, [0; 100]);

            #[cfg(feature = "sort_slices")]
            {
                const SORTED_SLICE: [$tpe; 3] = {
                    let mut arr = REV_ARRAY;
                    $slice_sort_name(&mut arr);
                    arr
                };

                assert_eq!(SORTED_SLICE, SORTED_REV_ARRAY);

                let mut rng = SmallRng::from_seed([0b01010101; 32]);
                let random_vec: Vec<$tpe> = (0..10_000).map(|_| rng.random()).collect();
                let std_sorted_vec = {
                    let mut vec = random_vec.clone();
                    vec.sort_unstable();
                    vec
                };
                let sorted_vec = {
                    let mut vec = random_vec;
                    $slice_sort_name(&mut vec);
                    vec
                };

                println!(
                    "{:?}",
                    sorted_vec
                        .iter()
                        .position(|&x| x == 26)
                        .map(|i| &sorted_vec[i - 1..=i + 2])
                );

                for (i, (&std, &custom)) in std_sorted_vec
                    .iter()
                    .zip(sorted_vec.iter())
                    .enumerate()
                    .skip(1)
                    .take(std_sorted_vec.len() - 2)
                {
                    println!("std:    {:?}", &std_sorted_vec[i - 1..=i + 1]);
                    println!("custom: {:?}", &sorted_vec[i - 1..=i + 1]);
                    println!();
                    assert_eq!(std, custom);
                }
            }
        }
    };
}

macro_rules! test_signed_integer {
    ($tpe:ty, $fn_name:ident, $array_sort_name:ident, $slice_sort_name:ident) => {
        #[test]
        fn $fn_name() {
            const REV_ARRAY: [$tpe; 3] = [3, 2, 1];
            const SORTED_REV_ARRAY: [$tpe; 3] = $array_sort_name(REV_ARRAY);
            const CONST_ARRAY: [$tpe; 3] = [2, 2, 2];
            const SORTED_CONST_ARRAY: [$tpe; 3] = $array_sort_name(CONST_ARRAY);
            const ARRAY_WITH_NEGATIVES: [$tpe; 3] = [0, -1, 2];
            const SORTED_ARRAY_WITH_NEGATIVES: [$tpe; 3] = $array_sort_name(ARRAY_WITH_NEGATIVES);

            const EMPTY_ARRAY: [$tpe; 0] = [];
            const SORTED_EMPTY_ARRAY: [$tpe; 0] = $array_sort_name(EMPTY_ARRAY);
            const SINGLETON_ARRAY: [$tpe; 1] = [1];
            const SORTED_SINGLETON_ARRAY: [$tpe; 1] = $array_sort_name(SINGLETON_ARRAY);

            assert_eq!(SORTED_EMPTY_ARRAY, []);
            assert_eq!(SORTED_SINGLETON_ARRAY, [1]);

            assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
            assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
            assert_eq!(SORTED_ARRAY_WITH_NEGATIVES, [-1, 0, 2]);

            #[cfg(feature = "sort_slices")]
            {
                const SORTED_SLICE: [$tpe; 3] = {
                    let mut arr = ARRAY_WITH_NEGATIVES;
                    $slice_sort_name(&mut arr);
                    arr
                };

                assert_eq!(SORTED_SLICE, SORTED_ARRAY_WITH_NEGATIVES);
            }
        }
    };
}

test_unsigned_integer! {u8, test_sort_u8, into_sorted_u8_array, sort_u8_slice}
test_unsigned_integer! {u16, test_sort_u16, into_sorted_u16_array, sort_u16_slice}
test_unsigned_integer! {u32, test_sort_u32, into_sorted_u32_array, sort_u32_slice}
test_unsigned_integer! {u64, test_sort_u64, into_sorted_u64_array, sort_u64_slice}
test_unsigned_integer! {
    u128,
    test_sort_u128,
    into_sorted_u128_array,
    sort_u128_slice
}

test_signed_integer! {i8, test_sort_i8, into_sorted_i8_array, sort_i8_slice}
test_signed_integer! {i16, test_sort_i16, into_sorted_i16_array, sort_i16_slice}
test_signed_integer! {i32, test_sort_i32, into_sorted_i32_array, sort_i32_slice}
test_signed_integer! {i64, test_sort_i64, into_sorted_i64_array, sort_i64_slice}
test_signed_integer! {
    i128,
    test_sort_i128,
    into_sorted_i128_array,
    sort_i128_slice
}

#[test]
fn test_sort_bool() {
    const ARR: [bool; 4] = [false, true, false, true];
    const SORTED_ARR: [bool; 4] = into_sorted_bool_array(ARR);

    #[cfg(feature = "sort_slices")]
    const SORTED_SLICE: [bool; 4] = {
        let mut arr = [true, false, true, false];
        sort_bool_slice(&mut arr);
        arr
    };

    assert_eq!(SORTED_ARR, [false, false, true, true]);
    #[cfg(feature = "sort_slices")]
    assert_eq!(SORTED_SLICE, [false, false, true, true]);
}

#[test]
fn test_char_sort() {
    const ARR: [char; 4] = ['a', '#', '\n', 'A'];
    const SORTED_ARR: [char; 4] = into_sorted_char_array(ARR);

    assert_eq!(SORTED_ARR, ['\n', '#', 'A', 'a'])
}
