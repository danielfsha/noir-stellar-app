# Setting up the project

Create a folder where you want to store the contracts and also circuit and run the following command. which will generate a contracts folder with the following file structure. 

```bash
stellar contract init contracts
```

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
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

## Creating the circuits
To generate the ZK logic circuit change to `wsl` - windows sub system for liux and under the folder you want to work under run the following command: 

```bash
nargo new circuits
```

This will generate a noir project with the following folder strucuture 
```text
.
├── circuits
│   ├─ src
│   │   └──main.nr
│   └── Cargo.toml
```