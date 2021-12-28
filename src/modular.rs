/// Represents a residue modulo n
#[derive(Debug, PartialEq, Eq)]
pub struct Residue {
    value: u128,    // value in 0, 1, ..., n-1
    modulus: u128,  // the modulus
}

impl Residue {
    pub fn from_integer(n: u128, modulus: u128) -> Residue {
        Residue {
            value: n,
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

        Residue {
            value: self.reduce_value(self.value + other.value),
            modulus: self.modulus
        }
    }

    pub fn times(&self, other: &Residue) -> Residue {
        self.assert_valid();
        other.assert_valid();
        assert_eq!(self.modulus, other.modulus);

        Residue {
            value: self.reduce_value(self.value * other.value),
            modulus: self.modulus
        }
    }

    pub fn scalar_times(&self, scalar: i128) -> Residue {
        self.assert_valid();

        Residue {
            value: self.reduce_value(
                self.value * (scalar as u128).rem_euclid(self.modulus)),
            modulus: self.modulus
        }
    }

    pub fn negated(&self) -> Residue {
        self.assert_valid();

        self.scalar_times(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_integer() {
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
    fn test_negated() {
        // TODO
    }
}