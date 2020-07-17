pub enum SortMethod {
    Asc,
    Desc,
}

pub fn bubble_sort(slice: &mut [u32], method: SortMethod) {
    if slice.is_empty() {
        return;
    }

    let mut count = 0;
    while count < slice.len() - 1 {
        let mut index = 0;
        while index < slice.len() - 1 - count {
            let m: bool = match method {
                SortMethod::Desc => slice[index] < slice[index + 1],
                SortMethod::Asc => slice[index] > slice[index + 1],
            };

            if m {
                slice[index] = slice[index] ^ slice[index + 1];
                slice[index + 1] = slice[index] ^ slice[index + 1];
                slice[index] = slice[index] ^ slice[index + 1];
            }
            index += 1;
        }

        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use core::{bubble_sort, SortMethod};

    #[test]
    fn test_empty() {
        let mut input: [u32; 0] = [];
        bubble_sort(&mut input, SortMethod::Asc);
        assert_eq!(0, input.len());
    }

    #[test]
    fn test_sort_asc() {
        let mut input: [u32; 6] = [5, 4, 3, 2, 1, 0];
        bubble_sort(&mut input, SortMethod::Asc);
        for i in 0..=5 {
            assert_eq!(0, input[0]);
        }
    }

    #[test]
    fn test_sort_desc() {
        let mut input: [u32; 6] = [0, 1, 2, 3, 4, 5];
        bubble_sort(&mut input, SortMethod::Asc);
        for i in 5..=0 {
            assert_eq!(5, input[input.len() - i]);
        }
    }

    #[test]
    fn test_sort_asc_with_duplicate_items() {
        let mut input: [u32; 8] = [5, 4, 3, 4, 2, 1, 0, 0];
        bubble_sort(&mut input, SortMethod::Asc);

        assert_eq!(input[0], 0);
        assert_eq!(input[1], 0);
        assert_eq!(input[2], 1);
        assert_eq!(input[3], 2);
        assert_eq!(input[4], 3);
        assert_eq!(input[5], 4);
        assert_eq!(input[6], 4);
        assert_eq!(input[7], 5);
    }

    #[test]
    fn test_sort_desc_with_duplicate_items() {
        let mut input: [u32; 8] = [0, 1, 5, 2, 3, 4, 5, 4];
        bubble_sort(&mut input, SortMethod::Desc);

        assert_eq!(input[0], 5);
        assert_eq!(input[1], 5);
        assert_eq!(input[2], 4);
        assert_eq!(input[3], 4);
        assert_eq!(input[4], 3);
        assert_eq!(input[5], 2);
        assert_eq!(input[6], 1);
        assert_eq!(input[7], 0);
    }
}
