# Naive Sumcheck Protocol
This project implements a naive sumcheck protocol in Rust, a cryptographic method used to efficiently verify sums over multilinear polynomials. The implementation focuses on the interaction between a Prover and a Verifier through a structured communication protocol, demonstrating how the sumcheck process can be simulated in a simple cryptographic setting.

## Overview
The protocol is designed around two primary structs:

- Prover: Responsible for generating the polynomial, evaluating its sum, and providing univariate polynomials for the verification process.
- Verifier: Receives results from the Prover, checks the correctness of the proofs, and ensures that the sum of the polynomial is consistent across various rounds.

## Setup and Installation
### Prerequisites
Ensure you have Rust and Cargo installed on your machine. Visit Rust's official site for installation instructions if you haven't installed Rust yet.

### Clone the Repository
```bash
git clone https://github.com/emirsoyturk/naive-sumcheck.git
cd naive-sumcheck
```

### Compilation
```bash
cargo build --release
```

### Run the compiled program:
```bash
cargo run --release
```

## Implementation Details
- The Prover starts by generating a multilinear polynomial with randomly chosen coefficients. It then computes the sum of this polynomial over all possible binary inputs.
- The Verifier initializes with an expectation of the polynomial's sum, which it later verifies through a series of rounds.
- Each round, the Prover generates a univariate polynomial based on a subset of the polynomial's variables, and the Verifier checks if the polynomial evaluations are consistent with previous results.
- Random values are used by the Verifier to challenge the Prover in subsequent rounds, enhancing the protocol's security.

## TODO
- [ ] Access to an oracle in last round
- [ ] Parallelize the protocol for faster execution
