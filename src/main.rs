use num_bigint::BigUint;

use crate::elliptic_curve::{EllipticCurve,Point};

pub mod finite_fields;
pub mod elliptic_curve;

fn main() {

    let p = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
        16,
    )
    .expect("could not convert p");

    let n = BigUint::parse_bytes(
        b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
        16,
    )
    .expect("could not convert n");

    let gx = BigUint::parse_bytes(
        b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
        16,
    )
    .expect("could not convert gx");
    
    let gy = BigUint::parse_bytes(
        b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
        16,
    )
    .expect("could not convert gy");

    let ec = EllipticCurve {
        a: BigUint::from(0u32),
        b: BigUint::from(7u32),
        p,
    };
    
    let g = Point::Coor(gx, gy);


    let is_true =ec.is_on_curve(&g);

    println!("Valid Point: {}", is_true); // true

}
