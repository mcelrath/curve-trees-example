extern crate bulletproofs;
extern crate relations;

use relations::curve_tree::*;

use rand::thread_rng;

use ark_std::UniformRand;
use ark_ec::{
    short_weierstrass::{Affine, SWCurveConfig},
    CurveGroup
};
use ark_secp256k1::{Config as SecpConfig, Fq as SecpBase};
use ark_secq256k1::{Config as SecqConfig};

// A simple set of values to add to the accumulator
const VALUES: [&str; 5] = ["foo", "bar", "baz", "qux", "quux"];

fn main() {
    let mut rng = rand::thread_rng();
    let depth = 3;
    let generators_length_log_2 = 11;
    let generators_length = 1<< generators_length_log_2;
    let sr_params = SelRerandParameters::<SecpConfig, SecqConfig>::new(generators_length, generators_length, &mut rng);

    let nelements = 5;
    let mut set = vec![];

    for i in 0..nelements {
        let some_point = Affine::<SecpConfig>::rand(&mut rng);
        // FIXME we need to Pedersen hash the data in VALUES into curve points (Affine::<SecpCconfig>)
        // So far this example generates random curve points.
        let (permissible_point, _) = sr_params
            .even_parameters
            .uh
            .permissible_commitment(&some_point, &sr_params.even_parameters.pc_gens.B_blinding);
        set.push(permissible_point);
    }

    // VALUES needs to be &[Affine<Config>] meaning curve points on SecpBase field

    let curve_tree = CurveTree::<256, SecpConfig, SecqConfig>::from_set(&set, &sr_params, Some(depth));

    println!("main.rs");
    println!("set size: {}", set.len());
    println!("set elements:");
    for i in 0..nelements {
        println!("\t{}", set[i]);
    }
}
