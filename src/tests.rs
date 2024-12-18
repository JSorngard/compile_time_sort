use super::*;

#[test]
fn test_sort_i32() {
    const REV_ARRAY: [i32; 3] = [3, 2, 1];
    const SORTED_REV_ARRAY: [i32; 3] = into_sorted_i32_array(REV_ARRAY);
    const CONST_ARRAY: [i32; 3] = [2, 2, 2];
    const SORTED_CONST_ARRAY: [i32; 3] = into_sorted_i32_array(CONST_ARRAY);
    const ARRAY_WITH_NEGATIVES: [i32; 3] = [0, -1, 2];
    const SORTED_ARRAY_WITH_NEGATIVES: [i32; 3] = into_sorted_i32_array(ARRAY_WITH_NEGATIVES);

    #[cfg(feature = "mut_refs")]
    const SORTED_SLICE: [i32; 3] = {
        let mut arr = REV_ARRAY;
        sort_i32_slice(&mut arr);
        arr
    };

    assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
    assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
    assert_eq!(SORTED_ARRAY_WITH_NEGATIVES, [-1, 0, 2]);
    #[cfg(feature = "mut_refs")]
    assert_eq!(SORTED_SLICE, SORTED_REV_ARRAY);
}

#[test]
fn test_sort_u32() {
    const REV_ARRAY: [u32; 3] = [3, 2, 1];
    const SORTED_REV_ARRAY: [u32; 3] = into_sorted_u32_array(REV_ARRAY);
    const CONST_ARRAY: [u32; 3] = [2, 2, 2];
    const SORTED_CONST_ARRAY: [u32; 3] = into_sorted_u32_array(CONST_ARRAY);

    #[cfg(feature = "mut_refs")]
    const SORTED_SLICE: [u32; 3] = {
        let mut arr = REV_ARRAY;
        sort_u32_slice(&mut arr);
        arr
    };

    assert_eq!(SORTED_REV_ARRAY, [1, 2, 3]);
    assert_eq!(SORTED_CONST_ARRAY, [2, 2, 2]);
    #[cfg(feature = "mut_refs")]
    assert_eq!(SORTED_SLICE, SORTED_REV_ARRAY)
}

#[test]
fn test_sort_bool() {
    const ARR: [bool; 4] = [false, true, false, true];
    const SORTED_ARR: [bool; 4] = into_sorted_bool_array(ARR);

    #[cfg(feature = "mut_refs")]
    const SORTED_SLICE: [bool; 4] = {
        let mut arr = [true, false, true, false];
        sort_bool_slice(&mut arr);
        arr
    };

    assert_eq!(SORTED_ARR, [false, false, true, true]);
    #[cfg(feature = "mut_refs")]
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

    #[cfg(feature = "mut_refs")]
    const SORTED_SLICE: [i8; 100] = {
        let mut arr = [i8::MIN; 100];
        sort_i8_slice(&mut arr);
        arr
    };

    assert_eq!(SORTED_ARR, [-50, -2, 0, 5, 50]);
    #[cfg(feature = "mut_refs")]
    assert_eq!(SORTED_SLICE, [i8::MIN; 100]);
}

#[test]
fn test_char_sort() {
    const ARR: [char; 4] = ['a', '#', '\n', 'A'];
    const SORTED_ARR: [char; 4] = into_sorted_char_array(ARR);

    assert_eq!(SORTED_ARR, ['\n', '#', 'A', 'a'])
}
