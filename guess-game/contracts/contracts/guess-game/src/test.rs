#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address, Bytes, BytesN, testutils::Address as _};

#[test]
fn test_initialization() {
    let env = Env::default();
    
    // 1. Deploy Game Contract
    let game_id = env.register(GuessGame, ());
    let game_client = GuessGameClient::new(&env, &game_id);

    // 2. Initialize
    let dummy_vk = Bytes::from_slice(&env, &[0xaa; 32]); 
    // This should succeed without panic
    game_client.initialize(&dummy_vk);
}

#[test]
#[should_panic(expected = "Failed to parse VK")]
fn test_claim_fails_invalid_proof() {
    let env = Env::default();
    
    // 1. Deploy & Init
    let game_id = env.register(GuessGame, ());
    let game_client = GuessGameClient::new(&env, &game_id);
    let dummy_vk = Bytes::from_slice(&env, &[0xaa; 32]); 
    game_client.initialize(&dummy_vk);

    // 2. Player Setup & Commit
    let player = Address::generate(&env);
    let commitment_bytes = [1u8; 32]; 
    let commitment = BytesN::from_array(&env, &commitment_bytes);
    
    env.mock_all_auths();
    game_client.commit_guess(&player, &commitment);

    // 3. Claim with invalid proof
    let proof = Bytes::from_slice(&env, &[0xbb; 32]); 
    
    // This expects to panic because:
    // 1. UltraHonkVerifier::new tries to parse the VK (which is random junk here).
    // 2. Or verifier.verify fails.
    // Based on my code reading, "Failed to parse VK" is the panic message in `claim_win` if `new` fails.
    game_client.claim_win(&player, &proof);
}
