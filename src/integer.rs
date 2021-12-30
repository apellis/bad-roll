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

/// Integer square root
///
/// Adapted from:
///     https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Binary_numeral_system_(base_2)
pub fn isqrt(n: u128) -> u128 {
    let mut x = n;
    let mut c = 0u128;
    let mut d = 1 << 126;  // second-to-top bit set

    while d > n { d >>= 2; }

    while d > 0 {
        if x >= c + d {
            x -= c + d;
            c = (c >> 1) + d;
        } else {
            c >>= 1;
        }
        d >>= 2;
    }

    c
}

/// Factors the given positive integer into prime powers.
///
/// Returns a vector of pairs (p, e) where p^e is a maximal prime power of n.
///
/// TODO replace with a faster algorithm (currently using a slow, naïve one)
pub fn prime_factorize(mut n: u128) -> Vec<(u128, u32)> {
    assert!(n > 0, "Cannot factor 0.");

    let mut ret = vec![];

    for i in 2..(isqrt(n) + 1) {
        let mut e = 0;  // will be maximal e such that i^e divides current n
        while n % i == 0 {
            // i must be prime, since all powers of all smaller primes were
            // divided out of n in previous iterations
            e += 1;
            n /= i;
        }
        if e > 0 {
            ret.push((i, e));
        }
    }

    if n > 1 {
        // n is prime
        ret.push((n, 1));
    }

    ret
}

/// Euler's totient function
///
/// This algorithm depends on Euler's product formula:
///     phi(n) = n \prod_{p|n} (1 - 1/p)
/// To avoid floating point arithmetic,
pub fn euler_totient(mut n: u128) -> u128 {
    let prime_factors = prime_factorize(n);
    let mut phi = n;

    for &(p, e) in prime_factors.iter() {
        n /= p.pow(e);
        phi -= phi / p;
    }

    phi
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

    #[test]
    fn test_isqrt() {
        assert_eq!(0, isqrt(0));
        assert_eq!(1, isqrt(1));
        assert_eq!(1, isqrt(2));
        assert_eq!(3, isqrt(9));
        assert_eq!(485725, isqrt(235928775625));
    }

    #[test]
    fn test_prime_factorize() {
        assert_eq!(Vec::<(u128, u32)>::new(), prime_factorize(1));
        assert_eq!(vec![(2, 1)], prime_factorize(2));
        assert_eq!(vec![(2, 2)], prime_factorize(4));
        assert_eq!(vec![(2, 7)], prime_factorize(128));
        assert_eq!(vec![(2, 2), (3, 1)], prime_factorize(12));
        assert_eq!(vec![(7, 1), (13, 1)], prime_factorize(91));
        assert_eq!(vec![(97, 1)], prime_factorize(97));
    }

    #[test]
    fn test_euler_totient() {
        // if p is prime, then phi(p) = p - 1
        assert_eq!(1, euler_totient(2));
        assert_eq!(2, euler_totient(3));
        assert_eq!(52, euler_totient(53));
        assert_eq!(96, euler_totient(97));

        // if p is prime, then phi(p^k) = p^k - p^{k-1}
        assert_eq!(6, euler_totient(9));
        assert_eq!(18, euler_totient(27));
        assert_eq!(54, euler_totient(81));

        // if m and n are relatively prime, then phi(mn) = phi(m)phi(n)
        assert_eq!(4, euler_totient(12));
        assert_eq!(40, euler_totient(100));
    }
}