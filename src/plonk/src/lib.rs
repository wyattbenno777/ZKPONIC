/*
____  __ _  ____   __   __ _  __  ___
(__  )(  / )(  _ \ /  \ (  ( \(  )/ __)
 / _/  )  (  ) __/(  O )/    / )(( (__
(____)(__\_)(__)   \__/ \_)__)(__)\___)
ZKP demos on the IC.
2023 by Wyatt
*/

use dusk_plonk::prelude::*;
use dusk_plonk::prelude::Circuit as PlonkCircuit;
use rand_core::OsRng;
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

/*
PLONK using dusk-plonk.
*/


/*
1) When trying to use Plonk we depend on rand_core.
This does not work ou-of-the-box without JS.
We add our own random generator.

https://docs.rs/getrandom/latest/getrandom/#webassembly-support
https://forum.dfinity.org/t/issue-about-generate-random-string-panicked-at-could-not-initialize-thread-rng-getrandom-this-target-is-not-supported/15198/3
*/
use getrandom::register_custom_getrandom;
fn custom_getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    let mut rng = StdRng::seed_from_u64(123);
    rng.fill_bytes(buf);
    return Ok(());
}

register_custom_getrandom!(custom_getrandom);

// empty test circuit
#[derive(Debug, Default)]
struct EmptyCircuit;

impl Circuit for EmptyCircuit {
    fn circuit<C>(&self, _composer: &mut C) -> Result<(), Error>
    where
        C: Composer,
    {
        Ok(())
    }
}

/*
We exceeded the instruction limit for single message execution if we try all at once.
https://forum.dfinity.org/t/unusual-increase-in-errors-exceeded-the-instruction-limit-for-single-message-execution/13544/3
*/

// Implement a circuit that checks:
// 1) a + b = c where C is a PI
// 2) a <= 2^6
// 3) b <= 2^5
// 4) a * b = d where D is a PI
// 5) JubJub::GENERATOR * e(JubJubScalar) = f where F is a Public Input

#[derive(Debug, Default)]
pub struct TestCircuit {
    a: BlsScalar,
    b: BlsScalar,
    c: BlsScalar,
    d: BlsScalar,
    e: JubJubScalar,
    f: JubJubAffine,
}

impl PlonkCircuit for TestCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer,
    {
        let a = composer.append_witness(self.a);
        let b = composer.append_witness(self.b);

        // Make first constraint a + b = c
        let constraint =
            Constraint::new().left(1).right(1).public(-self.c).a(a).b(b);

        composer.append_gate(constraint);

        // Check that a and b are in range
        //composer.component_range(a, 1 << 6);
        //composer.component_range(b, 1 << 5);

        // Make second constraint a * b = d
        let constraint =
            Constraint::new().mult(1).public(-self.d).a(a).b(b);

        composer.append_gate(constraint);

        /*let e = composer.append_witness(self.e);
        let scalar_mul_result = composer
            .component_mul_generator(e, dusk_jubjub::GENERATOR_EXTENDED)?;

        // Apply the constraint
        composer.assert_equal_public_point(scalar_mul_result, self.f);*/

        Ok(())
    }
}

#[ic_cdk_macros::update]
pub fn test_plonk() -> String {
    let label = b"transcript-arguments";

    let pp = PublicParameters::setup(1 << 5, &mut OsRng)
        .expect("failed to setup");

    let (prover, verifier) = Compiler::compile::<EmptyCircuit>(&pp, label)
        .expect("failed to setup");

    let (proof, public_inputs) = prover
        .prove(&mut OsRng, &EmptyCircuit)
        .expect("failed to prove");

    format!("Result of the verifier: {:?}", verifier.verify(&proof, &public_inputs).expect("failed to verify proof"))

}

#[ic_cdk_macros::update]
pub fn test_plonk_constraints() -> String {
    let label = b"transcript-arguments";

    let pp = PublicParameters::setup(1 << 5, &mut OsRng)
        .expect("failed to setup");

    let (prover, verifier) = Compiler::compile::<TestCircuit>(&pp, label)
        .expect("failed to compile circuit");

    // Generate the proof and its public inputs
    let (proof, public_inputs) = prover
        .prove(&mut OsRng, &TestCircuit::default())
        .expect("failed to prove");


    format!("Result of the verifier: {:?}", verifier.verify(&proof, &public_inputs).expect("failed to verify proof"))

}
