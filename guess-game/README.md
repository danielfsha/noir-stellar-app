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

## Contract Integration

The Game Contract is designed to work with a Zero-Knowledge Verifier contract.

### 1. Build & Test Contracts

The `guess-game` contract has been updated to include the logic.
It validates proofs by calling an external Verifier contract.

To run the integration tests (which use a Mock Verifier):

```bash
cargo test --manifest-path contracts/Cargo.toml
```

### 2. Deployment (Production)

To deploy the full system:

1.  **Deploy the Verifier**: Deploy the logic from `3-soroban-verifier` (or a similar UltraHonk verifier).
    *   Initialize it with the **Verification Key** from the `nargo compile` step.
2.  **Deploy the Game**: Deploy the `guess-game` contract.
3.  **Link**: Call `initialize(verifier_address)` on the Game contract to point it to the Verifier.

### 3. Playing the Game

1.  **Commit**: Player generates a random `salt` and their `guess`. They compute `Poseidon(guess, salt)` and submit this commitment to the `commit_guess` function.
2.  **Generate Proof**: Player uses `nargo prove` to generate a proof that they know the preimage of the commitment and that the guess matches the secret target.
3.  **Claim Win**: Player calls `claim_win` with the generated proof. The contract verifies it and grants the win.
