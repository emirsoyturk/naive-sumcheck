# Naive Sumcheck Protocol
This project implements a naive sumcheck protocol in Rust, a cryptographic method used to efficiently verify sums over multilinear polynomials. The implementation focuses on the interaction between a Prover and a Verifier through a structured communication protocol, demonstrating how the sumcheck process can be simulated in a simple cryptographic setting.

## Overview
The protocol is designed around two primary structs:

- Prover: Responsible for generating the polynomial, evaluating its sum, and providing univariate polynomials for the verification process.
- Verifier: Receives results from the Prover, checks the correctness of the proofs, and ensures that the sum of the polynomial is consistent across various rounds.
## Key Features
Generation of random binary coefficients for polynomial construction.
Evaluation of polynomial sums over all possible combinations of binary inputs.
Efficient round-based communication protocol to verify the sums.
Use of rand and lambdaworks_math crates for random number generation and polynomial operations respectively.
Setup and Installation
Prerequisites
Ensure you have Rust and Cargo installed on your machine. Visit Rust's official site for installation instructions if you haven't installed Rust yet.

Clone the Repository
Clone this repository to your local machine using Git:

bash
Copy code
git clone https://github.com/your-github/naive-sumcheck.git
cd naive-sumcheck
Compilation
Compile the code with Cargo, Rust's build system and package manager:

bash
Copy code
cargo build --release
Running the Program
Run the compiled program:

bash
Copy code
cargo run --release
Implementation Details
The Prover starts by generating a multilinear polynomial with randomly chosen coefficients. It then computes the sum of this polynomial over all possible binary inputs.
The Verifier initializes with an expectation of the polynomial's sum, which it later verifies through a series of rounds.
Each round, the Prover generates a univariate polynomial based on a subset of the polynomial's variables, and the Verifier checks if the polynomial evaluations are consistent with previous results.
Random values are used by the Verifier to challenge the Prover in subsequent rounds, enhancing the protocol's security.
Conclusion
This naive implementation serves as a basic demonstration of the sumcheck protocol in cryptographic proof systems, particularly useful for educational purposes and initial testing of cryptographic concepts in a controlled environment.