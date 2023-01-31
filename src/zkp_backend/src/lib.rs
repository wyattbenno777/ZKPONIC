/*
____  __ _  ____   __   __ _  __  ___
(__  )(  / )(  _ \ /  \ (  ( \(  )/ __)
 / _/  )  (  ) __/(  O )/    / )(( (__
(____)(__\_)(__)   \__/ \_)__)(__)\___)
ZKP demos on the IC.
2023-01-30 by Wyatt
*/

//Arkworks general tests.

use ark_bn254::{Bn254, Fr};
use ark_ff::Field;
use ark_groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};
use ark_serialize::CanonicalSerialize;
use ark_std::rand::{rngs::StdRng, SeedableRng};

use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_marlin::Marlin;
use ark_poly::univariate::DensePolynomial;
use ark_poly_commit::marlin_pc::MarlinKZG10;
use blake2::Blake2s;

/*
 Define test circuit
 a and b are private inputs that need to equal the public input c.
*/
pub struct Circuit<F: Field> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub c: Option<F>,
}

impl<F: Field> ConstraintSynthesizer<F> for Circuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;

        let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;

        let c = cs.new_input_variable(|| self.c.ok_or(SynthesisError::AssignmentMissing))?;

        cs.enforce_constraint(lc!() + a + b, lc!() + Variable::One, lc!() + c)?;

        Ok(())
    }
}

/*
 Groth16 - fastest and smallest SNARK for R1CS.
 Non-universal; setup is tied to one circuit.
 https://eprint.iacr.org/2016/260
*/
#[ic_cdk_macros::query]
fn test_groth16() -> String {
    let rng = &mut StdRng::seed_from_u64(0u64);

    let pk = {
        let c = Circuit::<Fr> {
            a: None,
            b: None,
            c: None,
        };
        generate_random_parameters::<Bn254, _, _>(c, rng).unwrap()
    };

    let assigment = Circuit {
        a: Some(1.into()),
        b: Some(2.into()),
        c: Some(3.into()),
    };

    let public_input = &[assigment.c.unwrap()];

    let proof = create_random_proof(assigment, &pk, rng).unwrap();

    let mut proof_vec = Vec::new();
    proof.serialize(&mut proof_vec).unwrap();
    ic_cdk::api::print(format!("proof_vec: {:?}", proof_vec));

    let vk = prepare_verifying_key(&pk.vk);

    let result = verify_proof(&vk, &proof, public_input).unwrap();
    format!("Verify proof: {:?}!", result)
}

/*
 Marlin - a universal setup SNARK for R1CS
 Updatable common reference string.
 Faster than SONIC which it is based on.
 https://eprint.iacr.org/2019/1047
*/
#[ic_cdk_macros::query]
fn test_marlin() -> String {
    type MultiPC = MarlinKZG10<Bls12_381, DensePolynomial<BlsFr>>;
    type MarlinInst = Marlin<BlsFr, MultiPC, Blake2s>;

    let num_constraints: usize = 3;
    let num_variables: usize = 3;
    let rng = &mut ark_std::test_rng();

    let universal_srs =
    MarlinInst::universal_setup(num_constraints, num_variables, num_variables, rng)
        .unwrap();

    let circuit = Circuit {
        a: None,
        b: None,
        c: None,
    };

    let (index_pk, index_vk) = MarlinInst::index(&universal_srs, circuit).unwrap();

    let circuit_instance = Circuit {
        a: Some(1.into()),
        b: Some(2.into()),
        c: Some(3.into()),
    };

    let public_input = &[circuit_instance.c.unwrap()];

    let proof = MarlinInst::prove(&index_pk, circuit_instance, rng).unwrap();

    let mut proof_vec = Vec::new();
    proof.serialize(&mut proof_vec).unwrap();
    println!("proof_vec: {:?}", proof_vec);
    ic_cdk::api::print(format!("proof_vec: {:?}", proof_vec));

    let result = MarlinInst::verify(&index_vk, public_input, &proof, rng).unwrap();
    format!("Verify proof: {:?}!", result)
}
