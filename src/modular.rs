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

    fn reduce_value(&self, value: u128) -> u128 {
        value.rem_euclid(self.modulus)
    }

    pub fn plus(&self, other: &Residue) -> Residue {
        self.assert_valid();
        other.assert_valid();
        assert_eq!(self.modulus, other.modulus);

        Residue::from_unsigned_integer(
            self.reduce_value(self.value + other.value), self.modulus)
    }

    pub fn times(&self, other: &Residue) -> Residue {
        self.assert_valid();
        other.assert_valid();
        assert_eq!(self.modulus, other.modulus);

        Residue::from_unsigned_integer(
            self.reduce_value(self.value * other.value), self.modulus)
    }

    pub fn scalar_times(&self, scalar: i128) -> Residue {
        self.assert_valid();

        Residue::from_unsigned_integer(
            self.reduce_value(self.value * self.reduce_value(scalar as u128)),
            self.modulus)
    }

    pub fn neg(&self) -> Residue {
        self.assert_valid();

        self.scalar_times(-1)
    }

    pub fn inv(&self) -> Residue {
        self.assert_valid();

        let (g, u, v) = gcd_with_coefficients(
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
        // TODO
    }

    #[test]
    fn test_from_signed_integer() {
        // TODO
    }

    #[test]
    fn test_plus() {
        // TODO
    }

    #[test]
    fn test_times() {
        // TODO
    }

    #[test]
    fn test_scalar_times() {
        // TODO
    }

    #[test]
    fn test_neg() {
        // TODO
    }

    #[test]
    fn test_inv() {
        // TODO
    }
}