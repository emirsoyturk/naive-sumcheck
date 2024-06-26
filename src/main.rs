use lambdaworks_math::polynomial::dense_multilinear_poly::DenseMultilinearPolynomial;
use naive_sumcheck::{random_binary_coefficients, Fr, F};
use rand::{rngs::ThreadRng, RngCore};

struct Prover {
    polynomial: DenseMultilinearPolynomial<F>,
    sum: Fr,
    univariate_polys: Vec<DenseMultilinearPolynomial<F>>,
    compute_time: std::time::Duration,
    random_values: Vec<Fr>
}

impl Prover {
    fn new(num_coefs: usize) -> Self {
        let coefs = random_binary_coefficients(num_coefs);
        let polynomial = DenseMultilinearPolynomial::<F>::new(coefs);
        Self { polynomial, sum: Fr::zero(), univariate_polys: vec![], compute_time: std::time::Duration::new(0, 0), random_values: vec![] }
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

    fn evaluate_sum_at_point(&self, index: usize) -> DenseMultilinearPolynomial<F> {
        let num_vars = self.polynomial.num_vars();
        let num_combinations = 1 << (num_vars - 1);
        let mut univariate_poly = vec![Fr::zero(); 2];

        for i in 0..num_combinations {
            let mut point = vec![Fr::zero(); num_vars];
            let mut bit_idx = 0;
            for (j, _) in (0..num_vars).enumerate() {
                match j.cmp(&index) {
                    std::cmp::Ordering::Less => {
                        point[j] = Fr::from(self.random_values[j].value());
                    }
                    std::cmp::Ordering::Greater => {
                        point[j] = Fr::from((i >> bit_idx) & 1);
                        bit_idx += 1;
                    }
                    _ => {}
                }
            }

            point[index] = Fr::zero();
            let eval = self.polynomial.evaluate(point.clone()).unwrap();
            univariate_poly[0] += eval;

            point[index] = Fr::one();
            let eval = self.polynomial.evaluate(point.clone()).unwrap();
            univariate_poly[1] += eval;
        }
        
        DenseMultilinearPolynomial::<F>::new(univariate_poly)
    }

    fn start_round(&mut self) {
        let start_time = std::time::Instant::now();
        let sum = self.evaluate_sum();
        self.compute_time = start_time.elapsed();
        self.sum = sum;
    }

    fn round(&mut self, index: usize) {
        let univariate_poly = self.evaluate_sum_at_point(index);
        self.univariate_polys.push(univariate_poly);
    }
}

struct Verifier {
    sum: Fr,
    univariate_polys: Vec<DenseMultilinearPolynomial<F>>,
    compute_time: std::time::Duration,
    random_values: Vec<Fr>
}

impl Verifier {
    fn new() -> Self {
        Self { sum: Fr::zero(), univariate_polys: vec![], compute_time: std::time::Duration::new(0, 0), random_values: vec![] }
    }

    fn start_round(&self) {
        println!("Verifier received sum: {:?}", self.sum);
    }

    fn round(&self, index: usize) {
        let expected = match index {
            0 => self.sum,
            _ => self.univariate_polys[index - 1].evaluate(vec![self.random_values[index - 1]]).unwrap().double(),
        };

        let eval1 = self.univariate_polys[index].evaluate(vec![Fr::zero()]).unwrap();
        let eval2 = self.univariate_polys[index].evaluate(vec![Fr::one()]).unwrap();
        let sum = eval1 + eval2;
        println!("{:?} + {:?} = {:?} ?== {:?}", eval1.value(), eval2.value(), sum.value(), expected.value());
        assert_eq!(sum, expected, "Sumcheck failed at round {}", index);
    } 

    fn pick_random(&mut self) -> Fr {
        let random = Fr::from(ThreadRng::default().next_u64());
        self.random_values.push(random);

        random
    }   
}

fn start_protocol(prover: &mut Prover, verifier: &mut Verifier) {
    prover.start_round();
    verifier.sum = prover.sum;
    verifier.start_round();

    for i in 0..prover.polynomial.num_vars() {
        prover.round(i);
        verifier.univariate_polys.push(prover.univariate_polys[i].clone());
        verifier.round(i);
        let random = verifier.pick_random();
        prover.random_values.push(random);
    }
}

fn main() {
    println!("Naive Sumcheck Protocol");
    let num_coeffs = 2u64.pow(11) as usize;

    let mut prover = Prover::new(num_coeffs);
    let mut verifier = Verifier::new();

    start_protocol(&mut prover, &mut verifier);

    println!("Prover summation compute time: {:?}", prover.compute_time);
    println!("Verifier summation verification time: {:?}", verifier.compute_time);
    println!("Performance ratio: {:?}", prover.compute_time.as_secs_f64() / verifier.compute_time.as_secs_f64());
}
