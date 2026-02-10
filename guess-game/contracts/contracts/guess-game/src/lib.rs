#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, crypto::BnScalar, vec, Address, Bytes, BytesN, Env, Symbol,
    U256, xdr::ToXdr,
};
use soroban_poseidon::poseidon2_hash;

#[contract]
pub struct GuessGame;

#[contracttype]
pub enum DataKey {
    Commitment(Address),
    Target(Address), // Store the target for the player
    Verifier,
}

#[contractimpl]
impl GuessGame {
    pub fn initialize(env: Env, verifier: Address) {
        env.storage().instance().set(&DataKey::Verifier, &verifier);
    }

    /// Players "Commit" to a guess. 
    /// The contract generates a random TARGET for them and stores it.
    pub fn commit_guess(env: Env, player: Address, hash: BytesN<32>) {
        player.require_auth();
        
        // 1. Store the player's commitment
        env.storage().persistent().set(&DataKey::Commitment(player.clone()), &hash);

        // 2. Generate a random target for this session
        // Use Poseidon Hash(player_address, nonce) to generate a seed
        // Then normalize to a range (e.g., 0-100)
        
        // Convert Address to U256 (safe for BN254 field)
        let mut addr_bytes = [0u8; 32];
        let addr_vec = player.clone().to_xdr(&env);
        let addr_hash = env.crypto().sha256(&addr_vec); 
        // Use only 31 bytes to ensure it fits in BN254 scalar field
        addr_bytes[1..32].copy_from_slice(&addr_hash.to_array()[0..31]);
        let addr_field = U256::from_be_bytes(&env, &Bytes::from_array(&env, &addr_bytes));

        // Generate Nonce (random)
        let nonce: u64 = env.prng().gen();
        let nonce_field = U256::from_u32(&env, nonce as u32); // Use low bits for simplicity or u128

        // Poseidon Hash(Addr, Nonce)
        let inputs = vec![&env, addr_field, nonce_field];
        let seed: U256 = poseidon2_hash::<4, BnScalar>(&env, &inputs);

        // Derive Target: random number between 0 and 100
        // Simplification: use the last byte of the seed % 100
        let seed_bytes = seed.to_be_bytes();
        let last_byte = seed_bytes.get(31).unwrap();
        let target: u32 = (last_byte as u32) % 100;

        // Store the target securely
        env.storage().persistent().set(&DataKey::Target(player), &target);
    }

    /// Verify the ZK proof submitted by the player against the STORED target.
    pub fn claim_win(env: Env, player: Address, proof: Bytes) {
        player.require_auth();
        
        // 1. Retrieve the original commitment
        let commitment: BytesN<32> = env.storage().persistent().get(&DataKey::Commitment(player.clone())).unwrap();
        
        // 2. Retrieve the ASSIGNED target
        let target: u32 = env.storage().persistent().get(&DataKey::Target(player.clone())).unwrap();

        // 3. Prepare Public Inputs: [commitment, target]
        let mut public_inputs = Bytes::new(&env);
        public_inputs.append(&Bytes::from_slice(&env, commitment.to_array().as_slice()));

        // Convert u32 target to 32-byte Field element (Big Endian)
        let mut target_bytes = [0u8; 32];
        target_bytes[28] = (target >> 24) as u8;
        target_bytes[29] = (target >> 16) as u8;
        target_bytes[30] = (target >> 8) as u8;
        target_bytes[31] = target as u8;
        public_inputs.append(&Bytes::from_slice(&env, &target_bytes));

        // 4. Call External Verifier Contract
        let verifier_address: Address = env.storage().instance().get(&DataKey::Verifier).unwrap();
        
        let _: () = env.invoke_contract(
            &verifier_address,
            &Symbol::new(&env, "verify_proof"),
            vec![&env, public_inputs.to_val(), proof.to_val()],
        );
        
        // If successful, maybe reward the player?
        // env.events().publish(...)
    }
}

mod test;

