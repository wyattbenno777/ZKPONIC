/*
____  __ _  ____   __   __ _  __  ___
(__  )(  / )(  _ \ /  \ (  ( \(  )/ __)
 / _/  )  (  ) __/(  O )/    / )(( (__
(____)(__\_)(__)   \__/ \_)__)(__)\___)
ZKP demos on the IC.
2023 by Wyatt
*/

use rand_core::OsRng;
use rand::{RngCore, SeedableRng};
use rand::rngs::StdRng;

use plonky2::field::types::Field as PlonkyField;
use plonky2::iop::witness::{PartialWitness, WitnessWrite};
use plonky2::plonk::circuit_builder::CircuitBuilder;
use plonky2::plonk::circuit_data::CircuitConfig;
use plonky2::plonk::config::{GenericConfig, PoseidonGoldilocksConfig};

/*
When trying to use Plonk we depend on rand_core.
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

/*
 Plonky2 - a recursive SNARK with fast proofs and no trusted setup.
 https://github.com/mir-protocol/plonky2/blob/main/plonky2/plonky2.pdf
 https://drops.dagstuhl.de/opus/volltexte/2018/9018/pdf/LIPIcs-ICALP-2018-14.pdf
*/

/// An example of using Plonky2 to prove a statement of the form
/// "I know n * (n + 1) * ... * (n + 99)".
/// When n == 1, this is proving knowledge of 100!.
#[ic_cdk_macros::update]
fn test_plonky2() -> String {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;

    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    // The arithmetic circuit.
    let initial = builder.add_virtual_target();
    let mut cur_target = initial;
    for i in 2..101 {
        let i_target = builder.constant(F::from_canonical_u32(i));
        cur_target = builder.mul(cur_target, i_target);
    }

    // Public inputs are the initial value (provided below) and the result (which is generated).
    builder.register_public_input(initial);
    builder.register_public_input(cur_target);

    let mut pw = PartialWitness::new();
    pw.set_target(initial, F::ONE);

    let data = builder.build::<C>();
    match data.prove(pw) {
        Ok(proof) => format!(
            "Factorial starting at {} is {}",
            proof.public_inputs[0], proof.public_inputs[1]
        ),
        Err(e) => format!("Error: {}", e),
    }

}
