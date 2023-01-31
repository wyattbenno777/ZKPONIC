# ZKPonIC
Rust examples of Zero Knowledge Proofs (ZKP) running on the Internet Computer blockchain.
This directory is for testing the use and deployment of ZKP onto the Internet Computer Blockchain.

There are no basic examples of [Arkworks](https://github.com/arkworks-rs) libraries in use, this repo has those examples.
Anyone can use these examples to understand schemes and get started with our comprehensible templates.

## Schemes
**Groth16** - fastest and smallest SNARK for R1CS.
Non-universal; setup is tied to one circuit.
https://eprint.iacr.org/2016/260

**Marlin** - a universal setup SNARK for R1CS
Updatable common reference string.
Faster than SONIC which it is based on.
https://eprint.iacr.org/2019/1047

**Plonk** - a universal preprocessing general-purpose zk-SNARK.
Updatable preprocessing for new circuits.
Slower than groth16.
https://eprint.iacr.org/2019/953

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy

# Run commands defined in ../lib.rs // should return true.
dfx canister call zkp_backend test_groth16 '("")'

dfx canister call plonk test_plonk '("")'
```
