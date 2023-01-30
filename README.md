# ZKPonIC
Rust examples of Zero Knowledge Proofs (ZKP) running on the Internet Computer blockchain.
This directory is for testing the use and deployment of ZKP onto the Internet Computer Blockchain.

There are no basic examples of [Arkworks](https://github.com/arkworks-rs) libraries. Anyone can use these examples to understand schemes and get started with our comprehensible templates.

## Schemes
**Groth16** - fastest and smallest SNARK for R1CS.
Non-universal; setup is tied to one circuit.
https://eprint.iacr.org/2016/260

**Marlin** - a universal setup SNARK for R1CS
Updatable common reference string.
Faster than SONIC which it is based on.
https://eprint.iacr.org/2019/1047

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy

# Run commands defined in /zkp_backend/main.rs // should return true.
dfx canister call zkp_backend test_groth16 '("")'
```
