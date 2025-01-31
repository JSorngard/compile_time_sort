use compile_time_sort::{
    into_sorted_bool_array, into_sorted_char_array, into_sorted_i128_array, into_sorted_i32_array,
    into_sorted_i8_array, into_sorted_u32_array, into_sorted_u64_array, into_sorted_u8_array,
};

#[cfg(feature = "sort_slices")]
use compile_time_sort::{
    sort_bool_slice, sort_i32_slice, sort_i8_slice, sort_u128_slice, sort_u32_slice,
};

#[cfg(feature = "sort_slices")]
use rand::{rngs::SmallRng, Rng, SeedableRng};

#[test]
fn test_sort_u64() {
    const REV_ARRAY: [u64; 3] = [3, 2, 1];
    const SORTED_REV_ARRAY: [u64; 3] = into_sorted_u64_array(REV_ARRAY);
    const CONST_ARRAY: [u64; 3] = [2, 2, 2];
    const SORTED_CONST_ARRAY: [u64; 3] = into_sorted_u64_array(CONST_ARRAY);

    assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
    assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
}

#[test]
fn test_sort_i128() {
    const REV_ARRAY: [i128; 3] = [3, 2, 1];
    const SORTED_REV_ARRAY: [i128; 3] = into_sorted_i128_array(REV_ARRAY);
    const CONST_ARRAY: [i128; 3] = [2, 2, 2];
    const SORTED_CONST_ARRAY: [i128; 3] = into_sorted_i128_array(CONST_ARRAY);
    const ARRAY_WITH_NEGATIVES: [i128; 3] = [0, -1, 2];
    const SORTED_ARRAY_WITH_NEGATIVES: [i128; 3] = into_sorted_i128_array(ARRAY_WITH_NEGATIVES);

    assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
    assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
    assert_eq!(SORTED_ARRAY_WITH_NEGATIVES, [-1, 0, 2]);
}

#[test]
fn test_sort_i32() {
    const REV_ARRAY: [i32; 3] = [3, 2, 1];
    const SORTED_REV_ARRAY: [i32; 3] = into_sorted_i32_array(REV_ARRAY);
    const CONST_ARRAY: [i32; 3] = [2, 2, 2];
    const SORTED_CONST_ARRAY: [i32; 3] = into_sorted_i32_array(CONST_ARRAY);
    const ARRAY_WITH_NEGATIVES: [i32; 3] = [0, -1, 2];
    const SORTED_ARRAY_WITH_NEGATIVES: [i32; 3] = into_sorted_i32_array(ARRAY_WITH_NEGATIVES);

    #[cfg(feature = "sort_slices")]
    const SORTED_SLICE: [i32; 3] = {
        let mut arr = REV_ARRAY;
        sort_i32_slice(&mut arr);
        arr
    };

    assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
    assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
    assert_eq!(SORTED_ARRAY_WITH_NEGATIVES, [-1, 0, 2]);
    #[cfg(feature = "sort_slices")]
    assert_eq!(SORTED_SLICE, SORTED_REV_ARRAY);
}

#[test]
fn test_edge_cases() {
    const EMPTY_ARRAY: [i32; 0] = [];
    const SORTED_EMPTY_ARRAY: [i32; 0] = into_sorted_i32_array(EMPTY_ARRAY);
    const SINGLETON_ARRAY: [i32; 1] = [1];
    const SORTED_SINGLETON_ARRAY: [i32; 1] = into_sorted_i32_array(SINGLETON_ARRAY);
    const ARRAY_WITH_NEGATIVES: [i32; 3] = [0, -1, 2];
    const SORTED_ARRAY_WITH_NEGATIVES: [i32; 3] = into_sorted_i32_array(ARRAY_WITH_NEGATIVES);

    assert_eq!(SORTED_EMPTY_ARRAY, []);
    assert_eq!(SORTED_SINGLETON_ARRAY, [1]);
    assert_eq!(SORTED_ARRAY_WITH_NEGATIVES, [-1, 0, 2]);
}

#[test]
fn test_sort_u32() {
    const REV_ARRAY: [u32; 3] = [3, 2, 1];
    const SORTED_REV_ARRAY: [u32; 3] = into_sorted_u32_array(REV_ARRAY);
    const CONST_ARRAY: [u32; 3] = [2, 2, 2];
    const SORTED_CONST_ARRAY: [u32; 3] = into_sorted_u32_array(CONST_ARRAY);

    #[cfg(feature = "sort_slices")]
    const SORTED_SLICE: [u32; 3] = {
        let mut arr = REV_ARRAY;
        sort_u32_slice(&mut arr);
        arr
    };

    assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
    assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
    #[cfg(feature = "sort_slices")]
    assert_eq!(SORTED_SLICE, SORTED_REV_ARRAY)
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
fn test_u8_sort() {
    const ARR: [u8; 5] = [8, 1, u8::MAX, 5, 0];
    const SORTED_ARR: [u8; 5] = into_sorted_u8_array(ARR);

    assert_eq!(SORTED_ARR, [0, 1, 5, 8, u8::MAX]);
}

#[test]
fn test_i8_sort() {
    const ARR: [i8; 5] = [-2, 50, 0, 5, -50];
    const SORTED_ARR: [i8; 5] = into_sorted_i8_array(ARR);

    #[cfg(feature = "sort_slices")]
    const SORTED_SLICE: [i8; 100] = {
        let mut arr = [i8::MIN; 100];
        sort_i8_slice(&mut arr);
        arr
    };

    assert_eq!(SORTED_ARR, [-50, -2, 0, 5, 50]);
    #[cfg(feature = "sort_slices")]
    assert_eq!(SORTED_SLICE, [i8::MIN; 100]);
}

#[test]
fn test_char_sort() {
    const ARR: [char; 4] = ['a', '#', '\n', 'A'];
    const SORTED_ARR: [char; 4] = into_sorted_char_array(ARR);

    assert_eq!(SORTED_ARR, ['\n', '#', 'A', 'a'])
}

#[cfg(feature = "sort_slices")]
#[test]
fn test_big_sort() {
    const N: usize = 1_000_000;

    let mut rng = SmallRng::from_seed([0b01010101; 32]);
    let vals: Vec<u128> = (0..N).map(|_| rng.random()).collect();

    let mut sorted_vals = vals.clone();
    sorted_vals.sort_unstable();

    let mut sorted_slice = vals.clone();
    sort_u128_slice(&mut sorted_slice);

    assert_eq!(sorted_vals, sorted_slice);
}
