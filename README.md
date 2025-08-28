# solana-demo

This project is a basic Solana smart contract (program) built with Anchor using a Rust template and deployed in a WSL2 Linux environment. It serves as a starting point for learning and experimenting with Solana development.

## Project overview

- Single `initialize()` instruction (no accoiutns/state, just logs `Program log: Greetings from: <PROGRAM_ID>`)
- Local development on localnet (single-node validator; ledger stored in Linux)
- Anchor workspace with generated IDL and deploy artifacts under `target/`
- README documents build/deploy/test

## Steps taken

- Installed Solana CLI, Rust, and Anchor inside WSL2 (Ubuntu).
- Created the project using Anchor's Rust test template.
- Ran the local validator, and stored the ledger under `~/.solana-ledgers/...` for WSL2 compatibility.
- Built the contract using `anchor build`.
- Deployed the contract to a local validator using `anchor deploy`.
- Ran integration tests using `anchor test`.

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

    > **Step explanation**: 
    > - `solana-test-validator` spins up a single-node Solana cluster locally (RPC + ledger) on your machine. It behaves like a tiny, private Solana network so you can deploy and test programs fast, with free airdrops.
    > - `-r` wipes the local ledger directory (usually `./test-ledger` in your current folder) before starting. This gives you a clean slate—no leftover accounts, PDAs, or SOL balances from previous runs.
    > - What starts up: An RPC on `http://127.0.0.1:8899` (what the CLI and your tests talk to, known as your localnet), a WebSocket on `:8900` (used by subscriptions/logs), a built-in faucet so `solana airdrop <amount>` works locally, and a fresh on-disk ledger in your current directory (deleted next time you use `-r`).

2. Point the CLI at it (once per shell):

    ```bash
    solana config set -u localhost
    solana config get   # confirmation
    ```

    > **Step explanation**: Updates your Solana CLI default RPC endpoint to your local validator. 2nd command is a sanity check that prints your current config (RPC URL, keypair path, commitment).

3. Fund your local wallet for deploys/transactions (program deploy needs ~1.33 SOL + fee):

    1. Make sure you're pointed at your local validator:

        ```bash
        solana config set -u localhost
        solana balance -u localhost
        ```

    2. Airdrop some localnet SOL (do a big one so you’re set):

        ```bash
        solana airdrop 10 -u localhost
        solana balance -u localhost
        ```

4. Build, deploy, and run tests (Rust template):

    1. `cd` into `solana-test-app`:

        ```bash
        cd solana-test-app
        ```

        > **Step explanation**: All Anchor commands must be run from the workspace root (the folder that has `Anchor.toml`). 

    2. Run:

        ```bash
        anchor build
        anchor deploy                      # <-- required for Rust tests
        anchor test --skip-local-validator # skipping as we have one running from step 1. Don't skip if none is running.
        ```

        > **Step explanation**: 
        > - `anchor build` compiles your program(s) to Solana BPF and writes artifacts to `target/deploy/` (the `.so` and program keypair) and `target/idl/` (IDL). It *does not* deploy.
        > - `anchor deploy` uploads the compiled `.so` to the cluster your CLI points at (e.g., localnet), creating the upgradeable program + program-data accounts. Rust test template note: `anchor test` doesn't deploy for Rust-only tests. Run `anchor deploy` first (and re-deploy after every `-r` reset).
        > - `anchor test`:
        > For JS/TS tests: Anchor spins up a throwaway local validator and deploys automatically. (This repo doesn't have any)
        > Rust test template (this repo): Runs cargo test; deploy first (and re-deploy after every `-r` reset). If your validator is already running, use `--skip-local-validator`.

5. Verify success (localnet) from the Anchor workspace (folder with Anchor.toml):

    1. Get your Program ID from the program keypair Anchor generated:

        ```bash
        PROG=$(solana address -k target/deploy/solana_test_app-keypair.json)
        echo "$PROG"
        ```

        > **Step explanation**: Program ID is your program’s onchain address (derived from its program keypair); clients/tests use it to call your instructions, and it must match `declare_id!` in `lib.rs` and the [programs.localnet] entry in `Anchor.toml`.

    2. Confirm the program is deployed to your running local validator:
    
        ```bash
        solana program show "$PROG" -u localhost
        ```

        Expect to see details under the BPF Upgradeable Loader (program + program-data accounts).

    3. Check artifacts exist locally:

        ```bash
        ls -1 target/deploy/solana_test_app.so
        ls -1 target/idl/solana_test_app.json
        ```

        > **Step explanation**: This step confirms your local build artifacts exist—which means the anchor build succeeded.
        > - `target/deploy/solana_test_app.so`: The compiled BPF program binary that anchor deploy uploads onchain.
        > - `target/idl/solana_test_app.json`: The IDL your clients/tools use to call your program.

6. (Optional) See runtime logs (only shows something if tests/clients invoked your instruction):

    ```bash
    solana logs -u localhost | grep -A2 -B2 "$PROG"
    ```

    > **Step explanation**: Streams live transaction logs from your local validator and filters them to only your program's entries (`$PROG`), showing 2 lines before and after each match for context (e.g., `invoke`, your `msg!()`, `success`).

## (Optional) Manual deploy without tests

1. Make sure your localnet is running (validator) and your wallet is funded.

    ```bash
    solana config set -u localhost
    solana airdrop 10 -u localhost
    ```

2. (For first deploy only) Generate and set the Program ID in `Anchor.toml` and `lib.rs`:

    ```bash
    cd solana-test-app
    anchor build   # creates target/deploy/*-keypair.json
    PROG=$(solana address -k target/deploy/solana_test_app-keypair.json)
    
    # Paste $PROG into:
    # - Anchor.toml -> [programs.localnet] solana_test_app = "$PROG"
    # - programs/solana_test_app/src/lib.rs -> declare_id!("$PROG");
    anchor build   # rebuild after editing files
    ```

    >**Note**: If you already built earlier, you can skip the first `anchor build` and just read `$PROG`.

3. Deploy (from the Anchor workspace) with `anchor deploy`.

4. Verify it's onchain:

    ```bash
    PROG=$(solana address -k target/deploy/solana_test_app-keypair.json)
    solana program show "$PROG" -u localhost
    ```

## Optional: devnet deploy (smoke test on a public cluster)

Devnet is a public Solana test cluster. Keep this if you want a shareable onchain address. 


1. Point the CLI to devnet and fund your wallet (rate-limited):

    ```bash
    solana config set -u https://api.devnet.solana.com
    solana airdrop 2
    ```

2. Build and deploy (from the Anchor workspace):

    ```bash
    cd solana-test-app
    anchor build
    anchor deploy --provider.cluster devnet
    ```

3. (Optional) Publish the IDL so clients/explorers can fetch it:

    ```bash
    PROG=$(solana address -k target/deploy/solana_test_app-keypair.json)

    # First time:
    anchor idl init -f target/idl/solana_test_app.json "$PROG" --provider.cluster devnet

    # Later updates:
    # anchor idl upgrade -f target/idl/solana_test_app.json "$PROG" --provider.cluster devnet
    ```

4. Verify:

    ```bash
    solana program show "$PROG" -u devnet
    ```

    When you’re done, switch back to localnet:

    ```bash
    solana config set -u localhost
    ```

## Common fixes

- **Program ID mismatch** → make the ID the same in `Anchor.toml`, `lib.rs` (`declare_id!`), and any tests; then `anchor build` again.
- **Not in workspace** → run commands from the folder with `Anchor.toml`.
- **Wrong cluster** → `solana config get`; if needed: `solana config set -u localhost`.
- **Insufficient funds** → `solana airdrop 10 -u localhost`.
- **Stale ledger / WSL path issues** → restart with `solana-test-validator -r --ledger ~/.solana-ledgers/demo`.
- **If you see OpenSSL/pkg-config build errors on Ubuntu** → `sudo apt install -y build-essential pkg-config libssl-dev`.

## Reference

- [Installation guide](https://solana.com/docs/intro/installation) on Solana official docs

## Next steps

- Add account state (e.g., counter)
- Implement additional instructions
- Build a frontend or TypeScript client
