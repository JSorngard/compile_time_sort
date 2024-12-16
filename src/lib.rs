macro_rules! const_sort_t {
    ($($name:ident, $tpe:ty);*) => {
        $(
            pub const fn $name<const N: usize>(mut arr: [$tpe; N]) -> [$tpe; N] {
                let mut i = 0;
                while i < N {
                    let mut j = 0;
                    while j < N - 1 - i {
                        if arr[j] > arr[j + 1] {
                            let temp = arr[j];
                            arr[j] = arr[j + 1];
                            arr[j + 1] = temp;
                        }
                        j += 1;
                    }
                    i += 1;
                }
                arr
            }
        )*
    };
}

const_sort_t!(
    const_sort_usize, usize;
    const_sort_isize, isize;
    const_sort_u8, u8;
    const_sort_i8, i8;
    const_sort_u16, u16;
    const_sort_i16, i16;
    const_sort_u32, u32;
    const_sort_i32, i32;
    const_sort_u64, u64;
    const_sort_i64, i64
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_usize() {
        const ARRAY: [usize; 3] = [3, 2, 1];
        const SORTED_ARRAY: [usize; 3] = const_sort_usize(ARRAY);

        assert_eq!(SORTED_ARRAY, [1, 2, 3])
    }
}
