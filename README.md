# solana-demo

This project is a basic Solana smart contract (program) built using Anchor with Rust-based tests. It serves as a starting point for learning and experimenting with Solana development in a Linux environment using WSL2.

## Project overview

- Anchor-generated Solana smart contract scaffold
- Rust-based test framework (via `anchor init --test-template rust`)
- Simple `initialize` instruction as a placeholder
- Deployment and testing setup for a local Solana validator

## Steps taken

- Installed Solana CLI, Rust, and Anchor inside WSL2 (Ubuntu)
- Created the project using Anchor’s Rust test template
- Moved the project to WSL-native Linux filesystem for compatibility
- Built the project using `anchor build`
- Deployed the contract to a local validator using `anchor deploy`
- Ran integration tests using `anchor test`

## Next steps

- Add account state (e.g., counter)
- Implement additional instructions
- Build a frontend or TypeScript client
