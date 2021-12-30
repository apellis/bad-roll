use rand::Rng;
use std::cmp::min;

use super::integer::gcd;
use super::modular::Residue;

/// Given two (secret) primes, generates the correspondingpublic key (N, e).
///
/// The generated exponent (.1 of the return value) is guaranteed not to exceed
/// std::i128::MAX.
///
/// TODO: Consider rejecting (p, q) large enough that they may cause overflow
/// issues during encryption (or, instead, set a lower cap on e).
pub fn generate_public_key(p: u128, q: u128) -> (u128, u128) {
    let mut rng = rand::thread_rng();

    // Choose exponent e relatively prime to (p-1)(q-1) by trail and error
    let modulus = (p - 1) * (q - 1);
    loop {
        let max_exponent = min(modulus as i128, std::i128::MAX);
        let e = rng.gen_range(1..max_exponent) as u128;
        if gcd(e, modulus) == 1 {
            return (p * q, e);
        }
    }
}

pub fn encrypt(message: &Vec<u128>, public_key: (u128, u128)) -> Vec<Residue> {
    let mut ret  = vec![];

    for &piece in message.iter() {
        ret.push(
            Residue::from_unsigned_integer(piece, public_key.0)
                .pow(public_key.1 as i128));
    }

    ret
}

pub fn decrypt(p: u128, q: u128, ciphertext: &Vec<Residue>, e: u128) -> Vec<u128> {
    let d = Residue::from_unsigned_integer(e, (p - 1) * (q - 1))
        .inv()
        .value;
    let modulus = p * q;

    let mut ret = vec![];

    for piece in ciphertext.iter() {
        ret.push(
            piece
                .pow(d as i128)
                .value);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa() {
        let prime_pairs = [
            (997, 757),
            (2677, 3217),
            (454711, 255361),
            (8552137547, 75522871)
        ];
        for (p, q) in prime_pairs {
            let mut rng = rand::thread_rng();

            let (n, e) = generate_public_key(p, q);

            let message_length: usize = rng.gen_range(1..100);
            let mut message: Vec<u128> = vec![];
            for _ in 0..message_length {
                message.push(rng.gen_range(1..n));
            }
            let ciphertext = encrypt(&message, (n, e));
            let decrypted_message = decrypt(p, q, &ciphertext, e);

            assert_eq!(message, decrypted_message);
        }
    }
}
