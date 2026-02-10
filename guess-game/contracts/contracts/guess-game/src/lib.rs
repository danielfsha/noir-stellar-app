#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, BytesN, Bytes, Symbol, vec};

#[contract]
pub struct GuessGame;

#[contracttype]
pub enum DataKey {
    Commitment(Address),
    Verifier,
}

#[contractimpl]
impl GuessGame {
    pub fn initialize(env: Env, verifier: Address) {
        env.storage().instance().set(&DataKey::Verifier, &verifier);
    }

    // Players "Commit" to a guess by sending a hash
    pub fn commit_guess(env: Env, player: Address, hash: BytesN<32>) {
        player.require_auth();
        // Store commitment in persistent storage so it lasts until reveal
        env.storage().persistent().set(&DataKey::Commitment(player), &hash);
    }

    // Verify the ZK proof submitted by the player
    pub fn claim_win(env: Env, player: Address, proof: Bytes, target: u32) {
        player.require_auth();
        
        // 1. Retrieve the original commitment
        let commitment: BytesN<32> = env.storage().persistent().get(&DataKey::Commitment(player.clone())).unwrap();
        
        // 2. Prepare Public Inputs: [commitment, target]
        // Noir inputs are typically serialized as 32-byte field elements.
        let mut public_inputs = Bytes::new(&env);
        
        // Append commitment (first public input)
        // Note: Check your circuit input order. If main(commitment, ..., target), then commitment is first.
        public_inputs.append(&Bytes::from_slice(&env, commitment.to_array().as_slice()));

        // Append target (second public input)
        // Convert u32 target to 32-byte Field element (Big Endian)
        let mut target_bytes = [0u8; 32];
        target_bytes[28] = (target >> 24) as u8;
        target_bytes[29] = (target >> 16) as u8;
        target_bytes[30] = (target >> 8) as u8;
        target_bytes[31] = target as u8;
        public_inputs.append(&Bytes::from_slice(&env, &target_bytes));

        // 3. Call External Verifier Contract
        let verifier_address: Address = env.storage().instance().get(&DataKey::Verifier).unwrap();
        
        // Verify proof(public_inputs, proof)
        // Assuming the verifier contract has a function `verify_proof` taking (Bytes, Bytes)
        let _: () = env.invoke_contract(
            &verifier_address,
            &Symbol::new(&env, "verify_proof"),
            vec![&env, public_inputs.to_val(), proof.to_val()],
        );
        
        // If we reach here, verification succeeded!
    }
}
