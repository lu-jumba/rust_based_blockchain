use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
use pairing::bls12_381::Bls12;

struct ZKPCircuit {
    secret_value: Option<u64>,
}

impl Circuit<Bls12> for ZKPCircuit {
    fn synthesize<CS: ConstraintSystem<Bls12>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let value_var = cs.alloc(|| "secret_value", || Ok(self.secret_value.unwrap()))?;
        cs.enforce(|| "secret_value must be greater than 10", |lc| lc + value_var, |lc| lc + CS::one(), |lc| lc + (10, CS::one()));
        Ok(())
    }
}

pub fn generate_proof() {
    let rng = &mut rand::thread_rng();
    let params = groth16::generate_random_parameters::<Bls12, _, _>(ZKPCircuit { secret_value: None }, rng).unwrap();
    let circuit = ZKPCircuit { secret_value: Some(15) };
    let proof = groth16::create_random_proof(circuit, &params, rng).unwrap();
    let vk = &params.vk;
    let result = groth16::verify_proof(vk, &proof, &[]).unwrap();
    println!("zk-SNARK proof valid: {}", result);
}
