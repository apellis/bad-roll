use rand::Rng;
use crate::integer::is_prime;

use super::integer::{gcd_with_coefficients, euler_totient, prime_factorize};

/// Represents a residue modulo n
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Residue {
    value: u128,    // value in 0, 1, ..., n-1
    modulus: u128,  // the modulus
}

impl Residue {
    pub fn from_unsigned_integer(n: u128, modulus: u128) -> Residue {
        Residue {
            value: n.rem_euclid(modulus),
            modulus
        }
    }

    pub fn from_signed_integer(n: i128, modulus: u128) -> Residue {
        Residue {
            value: n.rem_euclid(modulus as i128) as u128,
            modulus
        }
    }

    fn assert_valid(&self) {
        assert!(
            self.value < self.modulus,
            "Invalid residue: {} (mod {})",
            self.value, self.modulus);
    }

    pub fn plus(&self, other: &Residue) -> Residue {
        self.assert_valid();
        other.assert_valid();
        assert_eq!(self.modulus, other.modulus);

        Residue::from_unsigned_integer(self.value + other.value, self.modulus)
    }

    pub fn times(&self, other: &Residue) -> Residue {
        self.assert_valid();
        other.assert_valid();
        assert_eq!(self.modulus, other.modulus);

        Residue::from_unsigned_integer(self.value * other.value, self.modulus)
    }

    pub fn scalar_times(&self, scalar: i128) -> Residue {
        self.assert_valid();

        Residue::from_signed_integer(scalar * self.value as i128, self.modulus)
    }

    pub fn neg(&self) -> Residue {
        self.assert_valid();

        self.scalar_times(-1)
    }

    pub fn inv(&self) -> Residue {
        self.assert_valid();

        let (g, u, _) = gcd_with_coefficients(
            self.value, self.modulus);

        if g == 1 {
            return Residue::from_signed_integer(u, self.modulus);
        } else {
            panic!(
                "Tried to invert non-unit {} (mod {})",
                self.value, self.modulus);
        }
    }

    /// Returns self raised to an integer power.
    ///
    /// This method uses a version of the "square then halve the exponent"
    /// method for fast squaring. Requires O(sqrt(e)) time, O(1) space.
    pub fn pow(&self, mut e: i128) -> Residue {
        self.assert_valid();

        if e < 0 {
            return self.inv().pow(-e);
        }

        let mut a = self.clone();
        let mut b = Residue::from_unsigned_integer(1, self.modulus);

        while e > 0 {
            if e % 2 == 1 {
                b = b.times(&a);
            }
            a = a.times(&a);
            e /= 2;
        }

        b
    }

    /// Returns a primitive root for the given modulus.
    ///
    /// Uses the method of guessing random integers between 1 and n-1 and
    /// testing each as follows:
    ///     x is primitive if and only if x^{phi(n)/p} != 1 (mod n)
    ///     for all prime factors p of phi(n), where phi(n) is Euler's
    ///     totient function.
    ///
    /// Since n / phi(n-1) is O(log(log(n))), it should not take too many
    /// guesses in order to find a primitive root.
    ///
    /// Warning: currently only works for n prime. In general, primitive roots
    /// exist if and only if
    ///     n = 1, 2, 4, p^k, or 2p^k,
    /// where p is an odd prime and k is a positive integer.
    /// TODO: handle other cases
    pub fn primitive_root(modulus: u128) -> Residue {
        if !is_prime(modulus) {
            panic!("Residue::primitive_root() only supports integer moduli.")
        }

        let mut rng = rand::thread_rng();
        let phi = euler_totient(modulus);
        let primes: Vec<u128> = prime_factorize(phi)
            .iter()
            .map(|&(p, _)| p)
            .collect();
        let one = Residue::from_unsigned_integer(1, modulus);

        'outer: loop {
            let n = Residue::from_unsigned_integer(
                rng.gen_range(1..modulus), modulus);
            for &p in primes.iter() {
                if n.pow((phi / p) as i128) == one { continue 'outer; }
            }
            return n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_unsigned_integer() {
        assert_eq!(
            Residue { value: 0, modulus: 5 },
            Residue::from_unsigned_integer(0, 5));
        assert_eq!(
            Residue { value: 2, modulus: 5 },
            Residue::from_unsigned_integer(2, 5));
        assert_eq!(
            Residue { value: 2, modulus: 5 },
            Residue::from_unsigned_integer(7, 5));
        assert_eq!(
            Residue { value: 3, modulus: 5 },
            Residue::from_unsigned_integer(10000000003, 5));
    }

    #[test]
    fn test_from_signed_integer() {
        assert_eq!(
            Residue { value: 0, modulus: 5 },
            Residue::from_signed_integer(0, 5));
        assert_eq!(
            Residue { value: 2, modulus: 5 },
            Residue::from_signed_integer(2, 5));
        assert_eq!(
            Residue { value: 2, modulus: 5 },
            Residue::from_signed_integer(7, 5));
        assert_eq!(
            Residue { value: 3, modulus: 5 },
            Residue::from_signed_integer(-2, 5));
        assert_eq!(
            Residue { value: 0, modulus: 5 },
            Residue::from_signed_integer(-5, 5));
        assert_eq!(
            Residue { value: 4, modulus: 5 },
            Residue::from_signed_integer(99999999999, 5));
    }

    #[test]
    fn test_plus_basic() {
        // format: (modulus, x_val, y_val, z_val) where x + y == z
        let test_cases: Vec<(u128, u128, u128, u128)> = vec![
            (7, 0, 0, 0),
            (7, 2, 3, 5),
            (7, 3, 2, 5),
            (7, 4, 5, 2),
            (7, 5, 4, 2),
            (6, 2, 4, 0),
            (6, 3, 2, 5),
        ];

        for &(modulus, x_val, y_val, z_val) in test_cases.iter() {
            assert_eq!(
                Residue::from_unsigned_integer(z_val, modulus),
                Residue::from_unsigned_integer(x_val, modulus)
                    .plus(&Residue::from_unsigned_integer(y_val, modulus)));
        }
    }

    #[test]
    #[should_panic]
    fn test_plus_panics_unequal_moduli () {
        Residue::from_unsigned_integer(2, 3).plus(
            &Residue::from_unsigned_integer(2, 5));
    }

    #[test]
    fn test_times() {
        // format: (modulus, x_val, y_val, z_val) where x * y == z
        let test_cases: Vec<(u128, u128, u128, u128)> = vec![
            (7, 0, 0, 0),
            (7, 2, 3, 6),
            (7, 3, 2, 6),
            (7, 4, 5, 6),
            (7, 5, 4, 6),
            (6, 2, 4, 2),
            (6, 2, 3, 0),
            (6, 3, 2, 0),
            (6, 1, 2, 2),
        ];

        for &(modulus, x_val, y_val, z_val) in test_cases.iter() {
            assert_eq!(
                Residue::from_unsigned_integer(z_val, modulus),
                Residue::from_unsigned_integer(x_val, modulus)
                    .times(&Residue::from_unsigned_integer(y_val, modulus)));
        }
    }

    #[test]
    #[should_panic]
    fn test_times_panics_unequal_moduli () {
        Residue::from_unsigned_integer(2, 3).times(
            &Residue::from_unsigned_integer(2, 5));
    }

    #[test]
    fn test_scalar_times() {
        // format: (modulus, c, x_val, z_val) where c * x == z
        let test_cases: Vec<(u128, i128, u128, u128)> = vec![
            (7, 0, 0, 0),
            (7, 2, 0, 0),
            (7, -2, 0, 0),
            (7, 0, 5, 0),
            (7, 2, 4, 1),
            (7, 2, 3, 6),
            (7, -2, 3, 1),
            (6, 3, 2, 0),
            (6, -3, 2, 0),
            (6, 2, 2, 4),
        ];

        for &(modulus, c, x_val, z_val) in test_cases.iter() {
            assert_eq!(
                Residue::from_unsigned_integer(z_val, modulus),
                Residue::from_unsigned_integer(x_val, modulus)
                    .scalar_times(c));
        }
    }

    #[test]
    fn test_neg() {
        // format: (modulus, x_val, z_val) where -x == z
        let test_cases: Vec<(u128, u128, u128)> = vec![
            (7, 0, 0),
            (7, 1, 6),
            (7, 2, 5),
            (6, 0, 0),
            (6, 1, 5),
            (6, 4, 2),
        ];

        for &(modulus, x_val, z_val) in test_cases.iter() {
            assert_eq!(
                Residue::from_unsigned_integer(z_val, modulus),
                Residue::from_unsigned_integer(x_val, modulus).neg());
        }
    }

    #[test]
    fn test_inv() {
        // format: (modulus, x_val, z_val) where x^{-1} == z
        let test_cases: Vec<(u128, u128, u128)> = vec![
            (7, 1, 1),
            (7, 2, 4),
            (7, 4, 2),
            (7, 3, 5),
            (6, 1, 1),
            (6, 5, 5),
        ];

        for &(modulus, x_val, z_val) in test_cases.iter() {
            assert_eq!(
                Residue::from_unsigned_integer(z_val, modulus),
                Residue::from_unsigned_integer(x_val, modulus).inv());
        }
    }

    #[test]
    fn test_pow() {
        // format: (modulus, x_val, e, z_val) where x^e == z
        let test_cases: Vec<(u128, u128, i128, u128)> = vec![
            (7, 1, 1, 1),
            (7, 2, 1, 2),
            (7, 2, 0, 1),
            (7, 2, -1, 4),
            (7, 5, 0, 1),
            (7, 5, 1, 5),
            (7, 5, 2, 4),
            (7, 5, 3, 6),
            (7, 5, 4, 2),
            (7, 5, 5, 3),
            (7, 5, 6, 1),
            (7, 5, 200, 4),
            (10, 3, 1, 3),
            (10, 3, 2, 9),
            (10, 3, 3, 7),
            (10, 3, 4, 1),
            (10, 3, 5, 3),
            (10, 3, 255, 7),
        ];

        for &(modulus, x_val, e, z_val) in test_cases.iter() {
            assert_eq!(
                Residue::from_unsigned_integer(z_val, modulus),
                Residue::from_unsigned_integer(x_val, modulus).pow(e));
        }
    }

    #[test]
    fn test_primitive_root() {
        for &n in [2, 3, 5, 97].iter() {
            let one = Residue::from_unsigned_integer(1, n);
            let root = Residue::primitive_root(n);

            // root is primitive if and only if none of
            //     root, root^2, ..., root^{n-2}
            // equals 1.
            //
            // TODO: This test doesn't quite work if the modulus is not prime.
            // Once Residue::primitive_root() handles non-prime moduli, this
            // test should be re-written to check the following three properties
            // hold of {1, root, ..., root^{phi(modulus)-1}}:
            //     1. all are distinct
            //     2. all are units mod modulus;
            // and that root^{phi(modulus)} = 1 mod modulus.
            let mut root_power = one.clone();
            for e in 1..(n-1) {
                root_power = root_power.times(&root);
                assert_ne!(root_power, one);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_primitive_root_panics_non_prime_modulus () {
        Residue::primitive_root(10);
    }
}
