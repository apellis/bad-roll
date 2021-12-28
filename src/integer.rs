/// Determines whether a given positiv integer is prime.
///
/// This function uses a simple sieve algorithm, with the "6k+k1"
/// optimization. Time complexity O(sqrt(n)), space O(1).
pub fn is_prime(n: u128) -> bool {
    if n == 0 || n == 1 || n > 2 && n % 2 == 0 || n > 3 && n % 3 == 0 {
        return false;
    }

    let mut i: u128 = 5;
    while i.pow(2) <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

/// Returns the greatest common divisor of the given numbers.
///
/// Uses the Euclidean algorithm.
pub fn gcd(x: u128, y: u128) -> u128 {
    let (mut a, mut b) = if x >= y { (x, y) } else { (y, x) };

    loop {
        let r = a % b;

        a = b;
        b = r;

        if b == 0 { break a; }
    }
}

/// Given integers (x, y), returns (g, u, v) such that ux + vy = g = gcd(x, y).
///
/// Uses the Extended Euclidean Algorithm.
pub fn gcd_with_coefficients(x: u128, y: u128) -> (u128, i128, i128) {
    let mut u = 1i128;
    let v;
    let mut g = x as i128;
    let mut a = 0i128;
    let mut b = y as i128;

    loop {
        if b == 0 {
            v = (g - x as i128 * u) / y as i128;
            break;
        }
        let (q, t) = (g / b, g % b);  // division with remainder
        let s = u - q * a;
        u = a;
        g = b;
        a = s;
        b = t;
    }

    (g as u128, u, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(true, is_prime(2));
        assert_eq!(true, is_prime(3));
        assert_eq!(true, is_prime(17));

        assert_eq!(false, is_prime(0));
        assert_eq!(false, is_prime(1));
        assert_eq!(false, is_prime(57));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(2, gcd(2, 6));
        assert_eq!(2, gcd(6, 2));
        assert_eq!(1, gcd(3, 2));
        assert_eq!(3, gcd(24, 15));
        assert_eq!(3, gcd(15, 24));
        assert_eq!(7, gcd(7, 7));
        assert_eq!(9, gcd(9, 27));
    }

    #[test]
    fn test_gcd_with_coefficients() {
        assert_eq!((17, -31, 13), gcd_with_coefficients(527, 1258));
        assert_eq!((12, -37, 8), gcd_with_coefficients(228, 1056));
        assert_eq!((7, 4517, -4430), gcd_with_coefficients(163961, 167181));
        assert_eq!((1, 59789, -970295), gcd_with_coefficients(3892394, 239847));
    }
}