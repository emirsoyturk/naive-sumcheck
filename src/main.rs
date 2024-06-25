use lambdaworks_math::polynomial::dense_multilinear_poly::DenseMultilinearPolynomial;
use naive_sumcheck::{random_binary_coefficients, Fr, F};

struct Prover {
    polynomial: DenseMultilinearPolynomial<F>,
    sum: Fr,
    univariate_polys: Vec<DenseMultilinearPolynomial<F>>,
    compute_time: std::time::Duration
}

impl Prover {
    fn new(coeffs: Vec<Fr>) -> Self {
        let polynomial = DenseMultilinearPolynomial::<F>::new(coeffs);
        Self { polynomial, sum: Fr::zero(), univariate_polys: vec![], compute_time: std::time::Duration::new(0, 0) }
    }

    fn evaluate_sum(&self) -> Fr {
        let num_vars = self.polynomial.num_vars();
        let num_combinations = 1 << num_vars;
        let mut sum = Fr::zero();

        for i in 0..num_combinations {
            let mut point = vec![];
            for (j, _) in (0..num_vars).enumerate() {
                point.push(Fr::from((i >> j) & 1));
            }

            if let Ok(result) = self.polynomial.evaluate(point) {
                sum += result;
            }
        }
        sum
    }

    fn evaluate_sum_at_point(&self, index: usize) -> Vec<Fr> {
        let num_vars = self.polynomial.num_vars();
        let num_combinations = 1 << (num_vars - 1);
        let mut univariate_poly = vec![Fr::zero(); 2];

        for i in 0..num_combinations {
            let mut point = vec![Fr::zero(); num_vars];
            let mut bit_idx = 0;
            for (j, _) in (0..num_vars).enumerate() {
                if j == index {
                    continue;
                }
                point[j] = Fr::from((i >> bit_idx) & 1);
                bit_idx += 1;
            }

            point[index] = Fr::zero();
            let eval = self.polynomial.evaluate(point.clone()).unwrap();
            univariate_poly[0] += eval;

            point[index] = Fr::one();
            let eval = self.polynomial.evaluate(point.clone()).unwrap();
            univariate_poly[1] += eval;
        }
        
        univariate_poly
    }

    fn start_round(&mut self) {
        let start_time = std::time::Instant::now();
        let sum = self.evaluate_sum();
        self.compute_time = start_time.elapsed();
        self.sum = sum;
    }

    fn round(&mut self, index: usize) {
        let univariate_poly = self.evaluate_sum_at_point(index);
        self.univariate_polys.push(DenseMultilinearPolynomial::<F>::new(univariate_poly.clone()));
    }

    fn start_protocol(&mut self) {
        self.start_round();
        for i in 0..self.polynomial.num_vars() {
            self.round(i);
            println!("Prover sent univariate polynomial for round {}", i);
        }
        println!("Prover sent sum and all univariate polynomials to verifier");
    }

}

struct Verifier {
    sum: Fr,
    univariate_polys: Vec<DenseMultilinearPolynomial<F>>,
    compute_time: std::time::Duration
}

impl Verifier {
    fn new(sum: Fr, univariate_polys: Vec<DenseMultilinearPolynomial<F>>) -> Self {
        Self { sum, univariate_polys, compute_time: std::time::Duration::new(0, 0) }
    }

    fn start_round(&self) {
        println!("Verifier received sum: {:?}", self.sum);
    }

    fn round(&self, index: usize) {
        let eval1 = self.univariate_polys[index].evaluate(vec![Fr::zero()]).unwrap();
        let eval2 = self.univariate_polys[index].evaluate(vec![Fr::one()]).unwrap();
        let sum = eval1 + eval2;
        assert_eq!(sum, self.sum, "Sumcheck failed at round {}", index);
    }

    fn start_protocol(&mut self) {
        let start_time = std::time::Instant::now();
        self.start_round();
        for i in 0..self.univariate_polys.len() {
            self.round(i);
        }
        self.compute_time = start_time.elapsed();
        println!("Sumcheck protocol completed successfully");
    }
}

fn main() {
    println!("Naive Sumcheck Protocol");
    let num_coeffs = 2u64.pow(11) as usize;
    let coefs = random_binary_coefficients(num_coeffs);

    let mut prover = Prover::new(coefs);
    prover.start_protocol();
    
    let mut verifier = Verifier::new(prover.sum, prover.univariate_polys);
    verifier.start_protocol();

    println!("Prover summation compute time: {:?}", prover.compute_time);
    println!("Verifier summation verification time: {:?}", verifier.compute_time);
    println!("Performance ratio: {:?}", prover.compute_time.as_secs_f64() / verifier.compute_time.as_secs_f64());
}
