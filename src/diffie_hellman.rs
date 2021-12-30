use rand::Rng;

use super::modular::Residue;

pub fn choose_base(modulus: u128) -> Residue {
    Residue::primitive_root(modulus)
}

pub fn generate_secret_and_shared_value(base: &Residue) -> (i128, Residue) {
    let mut rng = rand::thread_rng();
    let private_secret: i128 = rng.gen();
    let shared_value = base.pow(private_secret);
    (private_secret, shared_value)
}

pub fn compute_shared_secret(
        private_secret: i128, other_shared_value: &Residue) -> Residue {
    other_shared_value.pow(private_secret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffie_hellman() {
        for p in [997, 2677, 454711, 952252135981] {
            let base = choose_base(p);

            let (alice_secret, alice_shared) =
                generate_secret_and_shared_value(&base);
            let (bob_secret, bob_shared) =
                generate_secret_and_shared_value(&base);

            assert_eq!(
                compute_shared_secret(alice_secret, &bob_shared),
                compute_shared_secret(bob_secret, &alice_shared));
        }
    }
}