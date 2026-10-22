# Native Windows build and Devnet deployment

This guide builds and deploys the Anchor program with the Anchor CLI on Devnet.

> This guide uses Devnet.

## 1. Install prerequisites

Install:

- Rust and Cargo using `rustup`
- The Solana CLI for Windows (see Anza's [Windows install guide](https://docs.anza.xyz/cli/install))
- Visual Studio Build Tools or Visual Studio with **Desktop development with C++** and a Windows SDK

Open **Developer PowerShell for Visual Studio** so `link.exe`, the Windows SDK libraries, and compiler environment variables are available. In Visual Studio 2026 Community, you can load the shell with:

```powershell
& "C:\Program Files\Microsoft Visual Studio\18\Community\Common7\Tools\Launch-VsDevShell.ps1" -Arch amd64 -HostArch amd64
```

Check the tools:

```powershell
rustc --version
cargo --version
solana --version
where.exe link.exe
```

If Rust cannot find `link.exe` or the linker reports that it cannot find `kernel32.lib`, install the C++ workload and Windows SDK, then use Developer PowerShell.

## 2. Install the SBF builder

Install the builder version used by this project:

```powershell
cargo install cargo-build-sbf --version 4.3.0 --force
```

Put Cargo's executable folder before the Solana CLI folder in this PowerShell session so the installed builder is selected:

```powershell
$env:PATH = "$HOME\.cargo\bin;$env:PATH"
```

The project uses `anchor build`, which invokes the SBF builder automatically. To keep the installed builder selected in new sessions, put `%USERPROFILE%\.cargo\bin` before Solana's `active_release\bin` folder in your Windows user PATH.

## 3. Select the Devnet wallet

The wallet configured for this project is set in `Anchor.toml`:

```powershell
$wallet = "C:/Users/Joyesh/.config/solana/Greetings.json"
solana address --keypair $wallet
solana balance --url devnet --keypair $wallet
```

Do not share the keypair file. Fund the wallet before a new deployment:

```powershell
solana airdrop 2 --url devnet --keypair $wallet
```

If the faucet is rate-limited, use the [Solana Devnet faucet](https://faucet.solana.com/).

## 4. Build the program

From the project root:

```powershell
cd E:\Programs\solana-contract
anchor build
```

The compiled program will be `target\deploy\greeting.so`, and the IDL will be
`target\idl\greeting.json`.

## 5. Deploy a new program

Generate a program keypair once. Keep it secure because it determines the
program ID:

```powershell
New-Item -ItemType Directory -Force target\deploy | Out-Null
solana-keygen new `
  --outfile target\deploy\greeting-keypair.json `
  --no-bip39-passphrase
```

Get the program ID and update both `Anchor.toml` and
`programs\greeting\src\lib.rs` with the same value:

```powershell
$programId = solana address --keypair target\deploy\greeting-keypair.json
$programId
anchor keys sync
```

Build and deploy the new program using the project wallet:

```powershell
anchor clean
anchor build
anchor program deploy `
  --provider.cluster devnet `
  --program-name greeting `
  --provider.wallet $wallet `
  --upgrade-authority $wallet
```

For later upgrades, rebuild and run the same deploy command. If Anchor
reports an error after the upgrade transaction has been submitted, use
`--no-idl` to skip the post-deployment IDL operation:

```powershell
anchor program deploy `
  --provider.cluster devnet `
  --program-name greeting `
  --provider.wallet $wallet `
  --upgrade-authority $wallet `
  --no-idl
```

Never run `anchor keys sync` after deployment unless you intentionally want to
change the program ID. Keep `target\deploy\greeting-keypair.json` backed up.

Verify the deployment:

```powershell
solana program show $programId --url devnet
```

## Troubleshooting

- **`cargo-build-sbf` panic from version 4.1.0:** ensure `%USERPROFILE%\.cargo\bin` appears before Solana's `active_release\bin` in the Windows user PATH, then open a new PowerShell window. Run `anchor build`; the builder is invoked automatically.
- **`program not found`:** confirm that `target\deploy\greeting-keypair.json`, `Anchor.toml`, and `declare_id!` all refer to the same program ID. For a new deployment, also confirm that the wallet has enough Devnet SOL.
- **`Error: program not found` after a deployment signature:** confirm the transaction with `solana confirm <SIGNATURE> --url devnet` and inspect the program with `solana program show`. If finalized, retry with `anchor program deploy ... --no-idl`.
- **Insufficient funds:** fund the configured wallet with Devnet SOL and check its balance before deploying.
- **`link.exe` not found or `kernel32.lib` missing:** use Developer PowerShell and install the Visual Studio C++ workload plus a Windows SDK.

## Program instructions

- `initialize(greeting)`: save a non-empty greeting up to 200 UTF-8 bytes.
- `set_greeting(greeting)`: update it; only the wallet that initialized it can do so.
