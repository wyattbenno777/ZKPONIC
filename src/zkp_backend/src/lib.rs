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

// a + b = c
// a, b are private
// c is public
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

    println!("Creating proofs...");

    let assigment = Circuit {
        a: Some(1.into()),
        b: Some(2.into()),
        c: Some(3.into()),
    };

    let public_input = &[assigment.c.unwrap()];

    let proof = create_random_proof(assigment, &pk, rng).unwrap();

    let mut proof_vec = Vec::new();
    proof.serialize(&mut proof_vec).unwrap();
    println!("proof_vec: {:?}", proof_vec);

    let vk = prepare_verifying_key(&pk.vk);

    let result = verify_proof(&vk, &proof, public_input).unwrap();
    format!("Veirfy proof: {:?}!", result)
}
