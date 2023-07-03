#[macro_use]
extern crate bulletproofs;
extern crate relations;

use relations::curve_tree::*;

use rand::thread_rng;

use ark_ff::PrimeField;
use ark_std::{UniformRand, Zero};
use ark_ec::{
    short_weierstrass::{Affine, SWCurveConfig},
    CurveGroup
};
use ark_secp256k1::{Config as SecpConfig, Fq as SecpFieldBase, Fr as SecpScalarBase};
use ark_secq256k1::{Config as SecqConfig};

// A simple set of values to add to the accumulator
const VALUES: [&str; 5] = ["foo", "bar", "baz", "qux", "quux"];

fn main() {
    let mut rng = rand::thread_rng();
    let depth = 3;
    let leaf_width = 5;
    let generators_length_log_2 = 11;
    let generators_length = 1 << generators_length_log_2;

    let sr_params = SelRerandParameters::<SecpConfig, SecqConfig>::new(generators_length, generators_length, &mut rng);

    const NELEMENTS: usize = 5;

    // So far this example generates random curve points instead of using VALUES.
    let leaf_elements: Vec<_> = (0..leaf_width)
        .map(|_| SecpScalarBase::rand(&mut rng))
        .collect();
    println!("Leaf elements:");
    for i in 0..NELEMENTS {
        println!("\t{}", leaf_elements[i]);
    }

    let leaf_commitments: Vec<_> = (0..leaf_width)
        .map(|x| {sr_params
            .even_parameters
            .commit(&[leaf_elements[x]], SecpScalarBase::zero(), 0)
        }).collect();
    println!("Leaf Commitments:");
    for i in 0..NELEMENTS {
        println!("\t{}", leaf_commitments[i]);
    }

    let (permissible_points, permissible_randomnesses): (Vec<_>, Vec<_>) = (0..leaf_width)
        .map(|x| { sr_params.even_parameters.uh.permissible_commitment(
            &leaf_commitments[x],
            &sr_params.even_parameters.pc_gens.B_blinding)
        }).unzip();

    println!("Committed set size: {}", permissible_points.len());
    println!("Set elements:");
    for i in 0..permissible_points.len() {
        println!("\t{}", permissible_points[i]);
    }

    let curve_tree = CurveTree::<{NELEMENTS}, SecpConfig, SecqConfig>::from_set(&permissible_points, &sr_params, Some(depth));

    // FIXME now output witnesses and make proofs...
}
