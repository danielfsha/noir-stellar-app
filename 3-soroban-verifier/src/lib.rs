#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Bytes, Env, Symbol};
use ultrahonk_rust_verifier::{UltraHonkVerifier, PROOF_BYTES};

/// Contract
#[contract]
pub struct UltraHonkVerifierContract;

#[contracterror]
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    VkParseError = 1,
    ProofParseError = 2,
    VerificationFailed = 3,
    VkNotSet = 4,
    AlreadyInitialized = 5,
}

#[contractimpl]
impl UltraHonkVerifierContract {
    fn key_vk() -> Symbol {
        symbol_short!("vk")
    }

    /// Initialize the on-chain VK. Check if already initialized.
    pub fn initialize(env: Env, vk_bytes: Bytes) -> Result<(), Error> {
        if env.storage().instance().has(&Self::key_vk()) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&Self::key_vk(), &vk_bytes);
        Ok(())
    }

    /// Verify an UltraHonk proof using the stored VK.
    pub fn verify_proof(env: Env, public_inputs: Bytes, proof_bytes: Bytes) -> Result<(), Error> {
        let vk_bytes: Bytes = env
            .storage()
            .instance()
            .get(&Self::key_vk())
            .ok_or(Error::VkNotSet)?;

        if proof_bytes.len() as usize != PROOF_BYTES {
            return Err(Error::ProofParseError);
        }

        // Deserialize verification key bytes
        let verifier = UltraHonkVerifier::new(&env, &vk_bytes).map_err(|_| Error::VkParseError)?;

        // Verify
        verifier
            .verify(&proof_bytes, &public_inputs)
            .map_err(|_| Error::VerificationFailed)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_fails_without_vk() {
        let env = Env::default();
        let contract_id = env.register_contract(None, UltraHonkVerifierContract);
        let client = UltraHonkVerifierContractClient::new(&env, &contract_id);

        let public_inputs = Bytes::from_slice(&env, &[0; 32]);
        let proof_bytes = Bytes::from_slice(&env, &[0; 32]);

        // Should fail because VK is not set
        let result = client.try_verify_proof(&public_inputs, &proof_bytes);
        assert_eq!(result, Err(Ok(Error::VkNotSet)));
    }
}
