# Ethereum Price Oracle Circuit

This Noir circuit checks the price of Ethereum by calling an external oracle. It demonstrates how to use `unconstrained` functions to fetch data from an external source (JSON-RPC) and verifies constraints on the returned data.

## 📋 Prerequisites

- **Nargo**: The Noir build tool.
  - [Installation Guide](https://noir-lang.org/docs/getting_started/installation/)
- **WSL (Windows Subsystem for Linux)**: Recommended for Windows users as Nargo runs natively on Linux/macOS.

## 🛠️ Compilation

To check if the circuit compiles correctly:

```bash
cd 1-noir-circuit/eth_price_circuit
nargo check
```

## 🧪 Testing with Oracle

Because this circuit relies on an external oracle (`#[oracle(fetchEthPrice)]`), standard execution (`nargo execute`) will fail unless an Oracle Resolver is provided.

**The recommended way to run this circuit is using `nargo test` with a resolver.**

### Option 1: Live Oracle (Easiest)
You can use the included helper script to run the test against the live oracle:

```bash
./test.sh
```

Or run the command manually:

```bash
nargo test --oracle-resolver https://starlit-bubblegum-c0e20b.netlify.app
```

### Option 2: Local Oracle
If you are running the `2-oracle-server` locally (on port 5555):

```bash
nargo test --oracle-resolver http://localhost:5555
```

## ⚠️ Known Issues / Troubleshooting

### "0 output values were provided as a foreign call result"
If you run `nargo execute` without a resolver, you will see this error:
```text
error: Assertion failed: '0 output values were provided as a foreign call result for 1 destination slots'
```
This is expected behavior because `nargo execute` does not know how to resolve the `fetchEthPrice` oracle call. Use `nargo test` with the `--oracle-resolver` flag instead.

### "Functions marked with #[oracle] must have no body"
If you try to mock the price by adding a body to the oracle function:
```noir
#[oracle(fetchEthPrice)]
unconstrained fn get_price() -> Field { 2850 } // Error!
```
You must remove the `#[oracle]` attribute if you provide a simplified implementation for local testing without a resolver.