# solana-demo

This project is a basic Solana smart contract (program) built with Anchor using a Rust template and deployed in a WSL2 Linux environment. It serves as a starting point for learning and experimenting with Solana development.

## Project overview

- Anchor-generated Solana smart contract scaffold
- Rust-based test framework (via `anchor init --test-template rust`)
- Simple `initialize` instruction as a placeholder
- Deployment and testing setup for a local Solana validator

## Prerequisites

- **WSL2 + Ubuntu** (or native Linux/macOS)
  - Ubuntu packages: `build-essential`, `pkg-config`, `libssl-dev`
- **Solana CLI** (includes `solana-test-validator`)
- **Rust toolchain** (`rustup`, `cargo`)
- **Anchor CLI** (installed via `avm`)
- **Node.js 18+ (LTS)** (required if running Anchor JS/TS tests or writing a client)
  - To skip tests, remove/comment the `test` entry under `[scripts]` in `Anchor.toml`
- **Git**

### Quick checks

These should all succeed:

```bash
solana --version
anchor --version
rustc --version
cargo --version
node -v                 # if using JS/TS tests
npm -v                  # if using JS/TS tests
solana-keygen pubkey    # If this errors, see 'wallet setup' below
```

### First-time wallet setup 

`solana-keygen pubkey` prints the public key of the wallet (keypair) your Solana CLI is using. Think of it as "What address am I?". If it errors, create a keypair:

```bash
solana-keygen new        # accept defaults; store the seed phrase safely
solana-keygen pubkey     # now prints your address
solana config set --keypair ~/.config/solana/id.json
```

## Steps taken

- Installed Solana CLI, Rust, and Anchor inside WSL2 (Ubuntu)
- Created the project using Anchor’s Rust test template
- Moved the project to WSL-native Linux filesystem for compatibility
- Built the project using `anchor build`
- Deployed the contract to a local validator using `anchor deploy`
- Ran integration tests using `anchor test`

## Local test loop

1. Start a clean local validator in a secondary terminal (long-running process):

    For macOS/Linux or if your repo lives in the Linux filesystem:

    ```bash
    solana-test-validator -r
    ```

    For WSL2 + repo on `/mnt/c` (Windows filesystem), use a Linux-home ledger to avoid socket/FS issues:

    ```bash
    solana-test-validator -r --ledger ~/.solana-ledgers/demo
    ```

    > **Step explanation**: `solana-test-validator` spins up a single-node Solana cluster locally (RPC + ledger) on your machine. It behaves like a tiny, private Solana network so you can deploy and test programs fast, with free airdrops.
    >
    > `-r` wipes the local ledger directory (usually `./test-ledger` in your current folder) before starting. This gives you a clean slate—no leftover accounts, PDAs, or SOL balances from previous runs.
    >
    >What starts up: An RPC on `http://127.0.0.1:8899` (what the CLI and your tests talk to, known as your localnet), a WebSocket on `:8900` (used by subscriptions/logs), a built-in faucet so `solana airdrop <amount>` works locally, and a fresh on-disk ledger in your current directory (deleted next time you use `-r`).

2. Point the CLI at it (once per shell):

    ```bash
    solana config set --url http://127.0.0.1:8899
    solana config get   # confirmation
    ```

    > **Step explanation**: Updates your Solana CLI default RPC endpoint to your local validator. 2nd command is a sanity check that prints your current config (RPC URL, keypair path, commitment).

3. Airdrop localnet SOL to cover running your local validator (~1.33 SOL + fee):

    1. Make sure you’re pointed at your local validator:

    ```bash
    solana config set -u localhost
    solana balance -u localhost
    ```

    2. Airdrop some localnet SOL (do a big one so you’re set):

    ```bash
    solana airdrop 10 -u localhost
    solana balance -u localhost
    ```

4. Build and run tests (auto-deploys to localnet):

    1. `cd` into `solana-test-app`:

        ```bash
        cd solana-test-app
        ```

    2. Run:

        ```bash
        anchor build
        anchor deploy                      # <-- required for Rust tests
        anchor test --skip-local-validator # skipping as we have one running from step 1
        ```

      - View program logs: `solana logs -u localhost`
      - If another validator is running, stop it or use `anchor test --skip-local-validator`.

    > **Step explanation**: All Anchor commands must be run from the workspace root (the folder that has `Anchor.toml`). 
    >
    > `anchor build` compiles your program(s) from Rust to Solana BPF, and emits artifacts: `.so` + program keypair under `target/deploy/`, and the IDL under `target/idl/`. It does *not* deploy. 
    >
    > `anchor test` builds your program(s), spins up a throwaway local validator, deploys the build to it, then runs your test suite (TS or Rust). State resets every run. If you already have a validator running, use `--skip-local-validator`.

5. Verify success (localnet) from the Anchor workspace (folder with Anchor.toml):

    1. Get your Program ID from the program keypair Anchor generated:

    ```bash
    PROG=$(solana address -k target/deploy/solana_test_app-keypair.json)
    echo "$PROG"
    ```

    2. Confirm the program is deployed to your running local validator:
    
    ```bash
    solana program show "$PROG" -u localhost
    ```

    Expect to see details under the BPF Upgradeable Loader (program + program-data accounts).

    3. (Optional) List loaded programs and grep yours:

    ```bash
    solana program list -u localhost | grep "$PROG"
    ```

    4. Check artifacts exist locally:

    ```bash
    ls -1 target/deploy/solana_test_app.so
    ls -1 target/idl/solana_test_app.json
    ```

5. (Optional) See runtime logs (only shows something if tests/clients invoked your instruction):

    ```bash
    solana logs -u localhost | grep -A2 -B2 "$PROG"
    ```

## (Optional) Manual deploy without tests

Make sure your localnet is running (the validator) and your wallet is funded.

1. Run:

    ```bash
    cd solana-test-app
    anchor build
    anchor deploy
    ```

2. For first deploy only: set the Program ID in `Anchor.toml`:

    ```bash
    # paste this under [programs.localnet]
    solana address -k target/deploy/solana_test_app-keypair.json
    ```

3. Verify it’s onchain:

    ```bash
    PROG=$(solana address -k target/deploy/solana_test_app-keypair.json)
    solana program show "$PROG" -u localhost
    ```

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
