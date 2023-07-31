use num_bigint::BigUint;
use num_traits::{One, Zero};
use num_traits::Num;
use std::str::FromStr;

fn is_on_secp256k1_curve(n: (BigUint, BigUint)) -> bool {
    // secp256k1 parameters

    let p: BigUint = BigUint::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap();
    let a: BigUint = BigUint::zero();
    let b: BigUint = BigUint::from(7u64);

    // Calculate y^2 (mod p)
    let y_squared: BigUint = (&n.1 * &n.1) % &p;

    // Calculate x^3 + 7 (mod p)
    let x_cubed_plus_7: BigUint = (&n.0 * &n.0 * &n.0 + &b) % &p;

    
    println!(
        "{:?} y_squared",
        y_squared
    );

    println!(
        "{:?} x_cubed_plus_7",
        x_cubed_plus_7
    );

    // Check if y^2 ≡ x^3 + 7 (mod p)
    y_squared == x_cubed_plus_7
}

fn main() {
    let valid_point1: (BigUint, BigUint) = (
        BigUint::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap(),
        BigUint::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap(),
    );


    let invalid_point1: (BigUint, BigUint) = (
        BigUint::from_str_radix("0000000000000000000000000000000000000000000000000000000000000000", 16).unwrap(),
        BigUint::from_str_radix("0000000000000000000000000000000000000000000000000000000000000000", 16).unwrap(),
    );

    let invalid_point2: (BigUint, BigUint) = (
        BigUint::from_str_radix("2a7b3eeb7b01c3ce1455c88a0043ae814d66b50bb7239c3deada84eb3cb5986a", 16).unwrap(),
        BigUint::from_str_radix("0000000000000000000000000000000000000000000000000000000000000000", 16).unwrap(),
    );

    println!("Valid Point 1: {}", is_on_secp256k1_curve(valid_point1)); // true
    println!("Invalid Point 1: {}", is_on_secp256k1_curve(invalid_point1)); // false
    println!("Invalid Point 2: {}", is_on_secp256k1_curve(invalid_point2)); // false
}
