use ark_ec::short_weierstrass::Affine;
use ark_ff::{Zero, PrimeField};
use ark_secp256k1::{Config as SecpConfig, Fr as SecpScalarBase};
use ark_secq256k1::Config as SecqConfig;
use ark_std::UniformRand;
use rand::thread_rng;
use relations::curve_tree::*;

const LEAF_WIDTH: usize = 5;
const VALUES: [&[u8]; LEAF_WIDTH] =  [b"foo", b"bar", b"baz", b"qux", b"quux"];
const NUM_ELEMENTS: usize = 5;
const DEPTH: usize = 3;
const GEN_LOG_2: usize = 11;

fn main() {
    let mut rng = thread_rng();

    // See also curve-trees/relations/src/coin.rs:element_from_bytes_stat
    let leaf_elements: Vec<_> = VALUES
        .into_iter()
        .map(|x| SecpScalarBase::from_le_bytes_mod_order(x)).collect();
    // Generate random leaf elements
    //let leaf_elements = generate_random_leaf_elements(&mut rng);
    display_elements("Leaf elements:", &leaf_elements);

    // Create select-rerandomize reltion and generate leaf commitments
    let sr_params = create_sel_rerand_parameters(&mut rng);
    let leaf_commitments = create_leaf_commitments(&leaf_elements, &sr_params);
    display_elements("Leaf Commitments:", &leaf_commitments);

    // Create permissible points and randomness
    let (permissible_points, _permissible_randomnesses) =
        create_permissible_points_and_randomnesses(&leaf_commitments, &sr_params);
    display_elements("Set elements:", &permissible_points);

    let _curve_tree = CurveTree::<{ NUM_ELEMENTS }, SecpConfig, SecqConfig>::from_set(
        &permissible_points,
        &sr_params,
        Some(DEPTH),
    );
}

fn create_sel_rerand_parameters<R: rand::Rng>(
    rng: &mut R,
) -> SelRerandParameters<SecpConfig, SecqConfig> {
    let generators_length = 1 << GEN_LOG_2;
    SelRerandParameters::<SecpConfig, SecqConfig>::new(generators_length, generators_length, rng)
}

fn generate_random_leaf_elements<R: rand::Rng>(rng: &mut R) -> Vec<SecpScalarBase> {
    (0..LEAF_WIDTH).map(|_| SecpScalarBase::rand(rng)).collect()
}

fn create_leaf_commitments(
    leaf_elements: &[SecpScalarBase],
    sr_params: &SelRerandParameters<SecpConfig, SecqConfig>,
) -> Vec<Affine<SecpConfig>> {
    leaf_elements
        .iter()
        .map(|&elem| {
            sr_params
                .even_parameters
                .commit(&[elem], SecpScalarBase::zero(), 0)
        })
        .collect()
}

fn create_permissible_points_and_randomnesses(
    leaf_commitments: &[Affine<SecpConfig>],
    sr_params: &SelRerandParameters<SecpConfig, SecqConfig>,
) -> (Vec<Affine<SecpConfig>>, Vec<SecpScalarBase>) {
    leaf_commitments
        .iter()
        .map(|commitment| {
            sr_params
                .even_parameters
                .uh
                .permissible_commitment(commitment, &sr_params.even_parameters.pc_gens.B_blinding)
        })
        .unzip()
}

fn display_elements<T: std::fmt::Display>(header: &str, elements: &[T]) {
    println!("{}", header);
    elements.iter().for_each(|elem| println!("\t{}", elem));
}
