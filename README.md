# ZKPONIC
Rust examples of zero knowledge proofs (ZKP) running on the Internet Computer blockchain.
We will be testing the use and deployment of ZKP creates onto the IC.



## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy

# Run commands defined in /zkp_backend/main.rs
dfx canister call zkp_backend test_groth16 '("")'
```
