use lambdaworks_math::field::{element::FieldElement, fields::u64_prime_field::U64PrimeField};
use rand::{rngs::ThreadRng, RngCore};
use rayon::prelude::*;

pub type F = U64PrimeField<9347661577>;
pub type Fr = FieldElement<F>;

pub fn random_binary_coefficients(num_coeffs: usize) -> Vec<Fr> {
    (0..num_coeffs)
        .into_par_iter()
        .map(|_| Fr::from(ThreadRng::default().next_u64() % 2))
        .collect()
}