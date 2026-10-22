# Solana Greeting Contract

An Anchor-based Solana program that stores and updates a greeting message under a Program Derived Address (PDA) associated with the user's wallet.

---

## Prerequisites

- [Rust & Cargo](https://rustup.rs/)
- [Solana CLI](https://docs.anza.xyz/cli/install) (v1.18+)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) (v1.2.1)
- *Windows users:* See [WINDOWS_SETUP.md](file:///e:/Programs/solana-contract/WINDOWS_SETUP.md) for required C++ build tools and environment setup.

---

## Deploy to Devnet

### 1. Set Cluster to Devnet
```bash
solana config set --url https://api.devnet.solana.com
```

### 2. Fund Deployer Wallet
Check your wallet address and ensure you have at least 2 SOL on Devnet:
```bash
solana address
solana balance
solana airdrop 2
```
*(If the CLI airdrop is rate-limited, use [faucet.solana.com](https://faucet.solana.com/)).*

Make sure `provider.wallet` in [`Anchor.toml`](file:///e:/Programs/solana-contract/Anchor.toml) points to your keypair file.

### 3. Generate Program Keypair & Sync IDs
Generate a deploy keypair (first time only):
```bash
solana-keygen new --outfile target/deploy/greeting-keypair.json --no-bip39-passphrase
```

Sync the generated Program ID across [`Anchor.toml`](file:///e:/Programs/solana-contract/Anchor.toml) and [`programs/greeting/src/lib.rs`](file:///e:/Programs/solana-contract/programs/greeting/src/lib.rs):
```bash
anchor keys sync
```

### 4. Build the Program
```bash
anchor build
```

### 5. Deploy
```bash
anchor deploy
```

Or deploy explicitly with Anchor CLI:
```bash
anchor program deploy \
  --provider.cluster devnet \
  --program-name greeting \
  --program-keypair target/deploy/greeting-keypair.json
```

### 6. Verify Deployment
```bash
solana program show <PROGRAM_ID> --url devnet
```

---

## Upgrades

To deploy code changes to an existing program:

1. Update code in [`programs/greeting/src/lib.rs`](file:///e:/Programs/solana-contract/programs/greeting/src/lib.rs).
2. Rebuild and deploy:
   ```bash
   anchor build
   anchor program deploy --program-name greeting --program-keypair target/deploy/greeting-keypair.json
   ```

---

## Program Instructions

- **`initialize(greeting)`**: Creates a PDA (`seeds = [b"greeting", authority]`) and sets the initial message (1–200 UTF-8 bytes).
- **`set_greeting(greeting)`**: Updates the greeting message. Only the authority wallet that initialized it can call this.
