/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
#[allow(dead_code)]
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
    let plain_iter = values.iter();
    let offset_iter = values.iter().cycle().skip(offset);
    plain_iter.zip(offset_iter).map(|(a, b)| b - a).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_one() {
        assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
        assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
        assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
    }

    #[test]
    fn test_larger_offsets() {
        assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
        assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
        assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
        assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
    }

    #[test]
    fn test_degenerate_cases() {
        assert_eq!(offset_differences(1, vec![0]), vec![0]);
        assert_eq!(offset_differences(1, vec![1]), vec![0]);
        let empty: Vec<i32> = vec![];
        assert_eq!(offset_differences(1, empty), vec![]);
    }
}
