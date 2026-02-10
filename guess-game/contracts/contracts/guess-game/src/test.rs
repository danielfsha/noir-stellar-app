#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, Address, Bytes, testutils::Address as _};

// Mock Verifier Contract to simulate successful verification
// In production, this would be the actual ultra-honk-verifier contract
#[contract]
pub struct MockVerifier;

#[contractimpl]
impl MockVerifier {
    pub fn verify_proof(_env: Env, _public_inputs: Bytes, _proof_bytes: Bytes) {
        // In a real verifier, this would check the proof against a VK
        // Here we just succeed (return ()) if called.
        // To verify failure cases, you could check specific input patterns.
    }
}

#[test]
fn test_game_flow() {
    let env = Env::default();
    
    // 1. Deploy Game Contract
    let game_id = env.register(GuessGame, ());
    let game_client = GuessGameClient::new(&env, &game_id);

    // 2. Deploy Mock Verifier Contract
    let verifier_id = env.register(MockVerifier, ());
    // (In reality you'd initialize the verifier with a VK here)

    // 3. Initialize Game with Verifier Address
    game_client.initialize(&verifier_id);

    // 4. Player Setup
    let player = Address::generate(&env);
    
    // 5. Commit Phase
    // Player hashes their guess (e.g., 42) + salt. 
    // real_commitment = Poseidon(guess, salt)
    // For this test we just use dummy bytes.
    let commitment_bytes = [1u8; 32]; 
    let commitment = BytesN::from_array(&env, &commitment_bytes);
    
    // Mock authentication
    env.mock_all_auths();
    
    game_client.commit_guess(&player, &commitment);

    // 6. Claim Phase
    // Player reveals target (42) and provides proof
    let target: u32 = 42;
    let proof = Bytes::from_slice(&env, &[0xaa; 32]); // Dummy proof

    // This should succeed because MockVerifier always returns success
    game_client.claim_win(&player, &proof, &target);
}
