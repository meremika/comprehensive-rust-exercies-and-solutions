pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut count = 0;
    let mut double = false;

    for c in cc_number.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            count += 1;
            if double {
                let double_digit = digit * 2;
                sum += if double_digit > 9 {
                    double_digit - 9
                } else {
                    double_digit
                };
            } else {
                sum += digit;
            }
        } else if c.is_whitespace() {
            continue;
        } else {
            return false;
        }
        double = !double;
    }

    if count < 2 { false } else { sum % 10 == 0 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_empty_cc_number() {
        assert!(!luhn(""));
        assert!(!luhn("     "));
    }

    #[test]
    fn test_short_cc_number() {
        assert!(!luhn("1"));
        assert!(!luhn("  5  "));
        assert!(!luhn("  0  "));
    }

    #[test]
    fn test_reject_invalid_characters() {
        assert!(!luhn("4263 abcd 9826 4026 9299"));
    }
}
