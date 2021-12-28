mod integer;
mod modular;

fn main() {
    println!("is 91 prime? {}", integer::is_prime(91));

    let modulus = 4;
    let val1 = 3;
    let val2 = 2;
    let res1 = modular::Residue::from_unsigned_integer(val1, modulus);
    let res2 = modular::Residue::from_unsigned_integer(val2, modulus);
    println!("working in Z/{}Z:", modulus);
    println!("{} + {} = {:?}", val1, val2, res1.plus(&res2));
    println!("{} * {} = {:?}", val1, val2, res1.times(&res2));
    println!("1 * {} = {:?}", val1, res1.scalar_times(1));
    println!("2 * {} = {:?}", val1, res1.scalar_times(2));
    println!("-1 * {} = {:?}", val1, res1.scalar_times(-1));
    println!("-{} = {:?}", val1, res1.neg());
    assert_eq!(res1.scalar_times(-1), res1.neg());

    println!("(527, 1258) -> {:?}", integer::gcd_with_coefficients(527, 1258));
    println!("(228, 1056) -> {:?}", integer::gcd_with_coefficients(228, 1056));
    println!(
        "(163961, 167181) -> {:?}",
        integer::gcd_with_coefficients(163961, 167181));
    println!(
        "(3892394, 239847) -> {:?}",
        integer::gcd_with_coefficients(3892394, 239847));

    for i in 1..11 {
        let res = modular::Residue::from_signed_integer(i, 11u128);
        println!("{}^{{-1}} = {:?}", i, res.inv());
    }
}
