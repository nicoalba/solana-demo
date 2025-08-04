# solana-demo

This project is a basic Solana smart contract (program) built with Anchor using a Rust template and deployed in a WSL2 Linux environment. It serves as a starting point for learning and experimenting with Solana development.

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

## Reference

- [Installation guide](https://solana.com/docs/intro/installation) on Solana official docs

## Next steps

- Add account state (e.g., counter)
- Implement additional instructions
- Build a frontend or TypeScript client
