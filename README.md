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

## Local test loop

1. Start a clean local validator in another terminal:

    ```bash
    solana-test-validator -r
    ```

2. Point the CLI at it (once per shell):

    ```bash
    solana config set --url http://127.0.0.1:8899
    solana config get   # confirm
    ```

3. Build & run tests (auto-deploys to localnet):

    ```bash
    anchor build
    anchor test
    ```

   - View program logs: `solana logs -u localhost`
   - If another validator is running, stop it or use `anchor test --skip-local-validator`.

## Manual local deploy (without tests)

Run:

```bash
anchor build
anchor deploy
```

Find the Program ID Anchor generated (after first build):

```bash
solana address -k target/deploy/<program_name>-keypair.json
anchor keys list
```

Ensure `Anchor.toml` has that Program ID under `[programs.localnet]`.

## Optional: devnet deploy (smoke test on a public cluster)

Run:

```bash
solana config set --url https://api.devnet.solana.com
solana airdrop 2            # fund your keypair on devnet
anchor build
anchor deploy --provider.cluster devnet
```

## Common fixes

- **Mismatched Program ID** → update `Anchor.toml` to match the keypair you deploy with.
- **Stale local ledger** → restart validator with `solana-test-validator -r`.
- **Wrong cluster** → `solana config get` and switch as needed.

## Reference

- [Installation guide](https://solana.com/docs/intro/installation) on Solana official docs

## Next steps

- Add account state (e.g., counter)
- Implement additional instructions
- Build a frontend or TypeScript client
