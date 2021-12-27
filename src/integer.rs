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
}