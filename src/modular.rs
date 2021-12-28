use super::integer::gcd_with_coefficients;

/// Represents a residue modulo n
#[derive(Debug, PartialEq, Eq)]
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
}