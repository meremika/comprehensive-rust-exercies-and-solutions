use std::cmp::Ordering;

pub fn min<T: Ord>(a: T, b: T) -> T {
    match a.cmp(&b) {
        Ordering::Less | Ordering::Equal => a,
        Ordering::Greater => b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integers() {
        assert_eq!(min(0, 10), 0);
        assert_eq!(min(500, 123), 123);
    }

    #[test]
    fn chars() {
        assert_eq!(min('a', 'z'), 'a');
        assert_eq!(min('7', '1'), '1');
    }

    #[test]
    fn strings() {
        assert_eq!(min("hello", "goodbye"), "goodbye");
        assert_eq!(min("bat", "armadillo"), "armadillo");
    }
}
