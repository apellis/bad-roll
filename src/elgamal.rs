use rand::Rng;
use std::cmp::min;

use super::modular::Residue;

pub fn choose_base(modulus: u128) -> Residue {
    Residue::primitive_root(modulus)
}

pub fn generate_key_pair(base: &Residue) -> (i128, Residue) {
    let mut rng = rand::thread_rng();
    let private_key = rng.gen_range(
        1..min(base.modulus as i128, std::i128::MAX));
    let public_key = base.pow(private_key);
    (private_key, public_key)
}

pub fn encrypt(
        base: &Residue, message: &Vec<u128>,
        public_key: &Residue) -> Vec<(Residue, Residue)> {
    assert_eq!(
        base.modulus, public_key.modulus,
        "Base and public_key have different moduli.");
    for &piece in message.iter() {
        assert!(
            piece < base.modulus,
            "Message pieces cannot exceed modulus.");
    }

    let mut rng = rand::thread_rng();
    let random_element = rng.gen_range(
        1..min(base.modulus as i128, std::i128::MAX));

    let mut ret = vec![];

    for &piece in message.iter() {
        let c1 = base.pow(random_element);
        let c2 = Residue::from_unsigned_integer(piece, base.modulus)
            .times(&public_key.pow(random_element));
        ret.push((c1, c2));
    }

    ret
}

pub fn decrypt(
        ciphertext: &Vec<(Residue, Residue)>, private_key: i128) -> Vec<u128> {
    let mut ret = vec![];

    for piece in ciphertext.iter() {
        ret.push(
            piece.0
            .pow(private_key)
            .inv()
            .times(&piece.1)
            .value);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elgamal() {
        for p in [997, 2677, 454711, 952252135981] {
            let mut rng = rand::thread_rng();

            let base = choose_base(p);
            let (private_key, public_key) = generate_key_pair(&base);

            let message_length: usize = rng.gen_range(1..100);
            let mut message: Vec<u128> = vec![];
            for _ in 0..message_length {
                message.push(rng.gen_range(1..base.modulus));
            }
            let ciphertext = encrypt(&base, &message, &public_key);
            let decrypted_message = decrypt(&ciphertext, private_key);

            assert_eq!(message, decrypted_message);
        }
    }
}
