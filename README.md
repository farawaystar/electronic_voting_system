# Electronic Voting System on Solana

This project implements a simple electronic voting system on the Solana blockchain. It allows for initializing a voting session, casting votes, and tracking vote counts.

## Features

- Initialize a voting session with start and end times
- Cast votes within the specified voting period
- Prevent double voting
- Track total vote count

## Project Structure

- `lib.rs`: Contains the Solana program (smart contract) logic
- `client.rs`: Implements the client-side interface to interact with the Solana program

## Prerequisites

- Rust and Cargo
- Solana CLI tools
- Node.js and npm (for Solana web3.js)

## Setup

1. Clone the repository
2. Install dependencies:
cargo build
3. Set up Solana CLI config for devnet:
solana config set -ud

# Usage
1. Generate necessary keypairs:
solana-keygen new -o payer_keypair.json
solana-keygen new -o vote_state_keypair.json
solana-keygen new -o voter_keypair.json


2. Build and deploy the Solana program:
cargo build-bpf
solana program deploy target/deploy/electronic_voting_system.so


3. Run the client to interact with the program:
cargo run --bin client


## How it Works

1. The program initializes a voting session with specified start and end times.
2. Voters can cast their votes during the voting period.
3. The program prevents double voting by marking accounts that have already voted.
4. The total vote count is tracked and can be retrieved.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Apache License 2.0

Copyright [2024] [farawaystar]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.

