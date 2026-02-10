# Setting up the project

Create a folder where you want to store the contracts and also circuit and run the following command. which will generate a contracts folder with the following file structure. 

```bash
stellar contract init contracts
```

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       ├── Cargo.toml
│       └── Makefile
├── .gitignore
├── Cargo.toml
└── README.md

```

## Creating the ZK Circuits

To generate the ZK logic circuit, ensure you are using `wsl` (Windows Subsystem for Linux) if on Windows. Inside your project folder, run:

```bash
nargo new circuits
```

This will generate a Noir project with the following structure:

```text
.
├── circuits
│   ├─ src
│   │   └──main.nr
│   └── Nargo.toml
```

### 1. Configure Dependencies

Open `circuits/Nargo.toml` and add the Poseidon hasher dependency under `[dependencies]`. This is required for the hashing functions used in the circuit.

```toml
[dependencies]
poseidon = { tag = "v0.2.0", git = "https://github.com/noir-lang/poseidon" }
```

### 2. Implement Circuit Logic

Open `circuits/src/main.nr` and replace its content with the following code. Note that we use `Poseidon2::hash` from the dependency we added.

```rust
use poseidon::poseidon2::Poseidon2; // Efficient hashing for ZK

fn main(
    commitment: pub Field, // Public: The hash of your guess + salt
    guess: Field,          // Private: Your actual secret guess
    salt: Field,           // Private: Random salt to prevent brute force
    target: pub Field      // Public: The number to match
) {
    // 1. Verify the commitment matches the private guess + salt
    let hash = Poseidon2::hash([guess, salt], 2);
    assert(hash == commitment);

    // 2. Verify the guess matches the winning target
    assert(guess == target);
}
```

### 3. Compile the Circuit

Run the following commands to compile the circuit and verify everything is correct:

```bash
cd circuits
nargo compile
```

This will generate a `target/circuits.json` file. You will need the **Verification Key** from this step for the smart contract.

## Deployment & Integration

This project requires two smart contracts to be deployed: 
1. The **Verifier Contract** (Standard UltraHonk Verifier)
2. The **Game Contract** (Your custom logic)

### 1. Build and Deploy the Verifier

We use the standard `ultrahonk_soroban_contract` for verifying Noir proofs.

```bash
# 1. Build the Verifier
cd ../3-soroban-verifier
stellar contract build

# 2. Deploy the Verifier
# Save the returned verify_contract_id
stellar contract deploy \
  --wasm target/wasm32v1-none/release/ultrahonk_soroban_contract.wasm \
  --source sban \
  --network testnet
```

### 2. Build and Deploy the Game Contract

```bash
# 1. Build the Game
cd ../guess-game/contracts
stellar contract build

# 2. Deploy the Game
# Save the returned game_contract_id
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/guess_game.wasm \
  --source sban \
  --network testnet
```

### 3. Initialize the Game

Connect the Game contract to the Verifier contract.

```bash
# Initialize with the Verifier Address from Step 1
stellar contract invoke \
  --id <game_contract_id> \
  --source sban \
  --network testnet \
  -- \
  initialize \
  --verifier <verify_contract_id>
```

### 4. Running Tests

The `guess-game` contract now integrates **Posiedon Hashing** (via `soroban-poseidon`) to generate fair key-targets on-chain.

To run the integration tests:

```bash
cargo test --manifest-path contracts/Cargo.toml
```

### 2. Deployment (Production)

To deploy the full system, you will need to setup your Stellar CLI and fund an account on Testnet.

#### Configure Source Account

Generate a keypair for your deploying account (e.g., `alice`) and fund it using Friendbot:

```bash
stellar keys generate alice --network testnet --fund
```

#### Deploy the Contracts

1.  **Build the Contracts**:
    ```bash
    stellar contract build
    ```

2.  **Deploy the Verifier (from `3-soroban-verifier`)**:
    Build the verifier from the `3-soroban-verifier` directory:
    ```bash
    rustup target add wasm32v1-none
    cd ../3-soroban-verifier
    stellar contract build --optimize
    cd ../guess-game
    ```

    Then deploy it:
    ```bash
    stellar contract deploy \
      --wasm ../3-soroban-verifier/target/wasm32v1-none/release/ultrahonk_soroban_contract.wasm \
      --source-account alice \
      --network testnet \
      --alias verifier \
      -- \
      --vk_bytes <YOUR_VK_BYTES_HEX>
    ```
    *Note: The new verifier requires the Verification Key to be passed as a constructor argument during deployment (`-- --vk_bytes ...`).*

3.  **Deploy the Game Contract**:
    ```bash
    stellar contract deploy \
      --wasm target/wasm32v1-none/release/guess_game.wasm \
      --source-account alice \
      --network testnet \
      --alias guess_game
    ```

5.  **Link the Contracts**:
    Tell the Game Contract where to find the Verifier.
    ```bash
    stellar contract invoke \
      --id guess_game \
      --source-account alice \
      --network testnet \
      -- \
      initialize \
      --verifier <VERIFIER_CONTRACT_ID>
    ```

### 3. Playing the Game via CLI

1.  **Commit Phase**:
    Player sends their commitment hash.
    ```bash
    stellar contract invoke \
      --id guess_game \
      --source-account alice \
      --network testnet \
      -- \
      commit_guess \
      --player alice \
      --hash <YOUR_COMMITMENT_HASH_HEX>
    ```

2.  **Generate Proof**:
    Use `nargo prove` (in WSL) to generate your proof string.

3.  **Claim Phase**:
    Submit the proof to win.
    ```bash
    stellar contract invoke \
      --id guess_game \
      --source-account alice \
      --network testnet \
      -- \
      claim_win \
      --player alice \
      --proof <YOUR_PROOF_BYTES_HEX>
    ```

