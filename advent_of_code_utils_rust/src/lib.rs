use num::Integer;
use std::cmp;

pub fn greatest_common_divisor<T: Integer + Copy>(x: T, y: T) -> T {
    let mut a = cmp::max(x, y);
    let mut b = cmp::min(x, y);
    loop {
        let rem = a.rem(b);
        if rem == T::zero() {
            break;
        }
        a = b;
        b = rem;
    }
    return b;
}

pub fn extended_euclidean_algorithm<T: Integer + Copy>(x: T, y: T) -> (T, T) {
    let mut switch = false;
    let (mut a, mut b) = if x > y {
        (x, y)
    } else {
        switch = true;
        (y, x)
    };
    let mut s_current = T::zero();
    let mut s_prev = T::one();
    let mut t_current = T::one();
    let mut t_prev = T::zero();

    loop {
        let (quotient, rem) = a.div_mod_floor(&b);
        if rem == T::zero() {
            return match switch {
                false => (s_current, t_current),
                _ => (t_current, s_current),
            };
        }
        a = b;
        b = rem;

        let s_next = s_prev - quotient * s_current;
        s_prev = s_current;
        s_current = s_next;
        let t_next = t_prev - quotient * t_current;
        t_prev = t_current;
        t_current = t_next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(100, 90, 10)]
    #[case(210, 45, 15)]
    #[case(2, 6, 2)]
    fn test_greatest_common_divisor(#[case] a: i128, #[case] b: i128, #[case] expected: i128) {
        assert_eq!(greatest_common_divisor(a, b), expected);
        assert_eq!(greatest_common_divisor(b, a), expected);
    }

    #[rstest]
    #[case(45, 210, 5, -1)]
    #[case(7, 3, 1, -2)]
    fn test_extended_euclidean_algorithm(
        #[case] a: i128,
        #[case] b: i128,
        #[case] expected_coeff_a: i128,
        #[case] expected_coeff_b: i128,
    ) {
        let (coeff_a, coeff_b) = extended_euclidean_algorithm(a, b);
        assert_eq!(a * coeff_a + b * coeff_b, greatest_common_divisor(a, b));
        assert_eq!(coeff_a, expected_coeff_a);
        assert_eq!(coeff_b, expected_coeff_b);
    }
}
