# 🚀 Hello World Solana dApp with Anchor

A minimal "Hello World" smart contract on Solana blockchain built with the Anchor framework. This project demonstrates the basic structure of a Solana program, how to deploy it to a local blockchain, and how to interact with it using tests.

## 📋 Table of Contents
- [Overview](#-overview)
- [Prerequisites](#-prerequisites)
- [Installation](#-installation)
  - [Linux (Fedora/Ubuntu)](#linux-fedoraubuntu)
  - [Windows](#windows)
- [Environment Setup](#-environment-setup)
- [Project Structure](#-project-structure)
- [Usage](#-usage)
- [Testing](#-testing)
- [Troubleshooting](#-troubleshooting)
- [Next Steps](#-next-steps)

## 🔍 Overview

This project contains a simple Solana program that logs "Hello, World!" to the blockchain when invoked. It demonstrates:
- Basic Anchor program structure
- Local Solana validator setup
- Program deployment workflow
- Testing with TypeScript
- Transaction signing and submission

## 📦 Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| **Rust** | 1.80+ | Smart contract programming language |
| **Solana CLI** | 2.1.21 | Blockchain interaction and local validator |
| **Anchor CLI** | 0.32.1 | Framework for Solana development |
| **Node.js** | 18+ | JavaScript runtime for tests |
| **npm/yarn** | Latest | Package management |
| **Git** | Latest | Version control |

## ⚙️ Installation

### Linux (Fedora/Ubuntu)

#### 1. Install System Dependencies

**Fedora:**
```bash
sudo dnf update -y
sudo dnf install -y \
    openssl-devel \
    systemd-devel \
    pkg-config \
    zlib-devel \
    llvm \
    clang \
    cmake \
    make \
    protobuf-devel \
    protobuf-compiler \
    perl-core \
    gcc \
    gcc-c++ \
    git \
    wget \
    curl \
    jq
```

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt upgrade -y
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libudev-dev \
    llvm \
    clang \
    cmake \
    make \
    protobuf-compiler \
    perl \
    git \
    wget \
    curl \
    jq
```

#### 2. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc  # or ~/.zshrc if using zsh
rustc --version  # Verify installation
```

#### 3. Install Solana CLI

```bash
# Install Solana 2.1.21 (compatible with Anchor 0.32.1)
curl -sSfL https://release.anza.xyz/v2.1.21/install | sh

# Add Solana to PATH
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify installation
solana --version  # Should show solana-cli 2.1.21
```

#### 4. Install Anchor CLI

```bash
# Install Anchor 0.32.1
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Verify installation
anchor --version  # Should show anchor-cli 0.32.1
```

#### 5. Install Node.js

**Fedora:**
```bash
sudo dnf install -y nodejs npm
```

**Ubuntu:**
```bash
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs
```

**Verify:**
```bash
node --version  # Should be v18+
npm --version
```

### Windows

#### 1. Install Windows Prerequisites

**Download and install:**
- [Git for Windows](https://git-scm.com/download/win)
- [Node.js 18+](https://nodejs.org/) (LTS version recommended)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
  - During installation, select "Desktop development with C++"
- [CMake](https://cmake.org/download/)
- [LLVM](https://github.com/llvm/llvm-project/releases) (Download the Windows pre-built binary)

#### 2. Install Rust on Windows

1. Download and run [rustup-init.exe](https://win.rustup.rs/)
2. Select "Default installation" when prompted
3. Open a **new PowerShell** window as Administrator

```powershell
# Verify Rust installation
rustc --version
cargo --version
```

#### 3. Install Solana CLI on Windows

Open **PowerShell as Administrator**:

```powershell
# Install Solana 2.1.21
curl -sSfL https://release.anza.xyz/v2.1.21/install | sh

# Add Solana to PATH (the installer will show the command)
# Usually it's adding this to your PATH:
# %USERPROFILE%\.local\share\solana\install\active_release\bin

# Verify installation
solana --version
```

#### 4. Install Anchor CLI on Windows

```powershell
# Install Anchor 0.32.1
cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked

# Verify installation
anchor --version
```

#### 5. Install Additional Windows Tools

```powershell
# Install yarn (optional but recommended)
npm install -g yarn

# Install Windows Build Tools for Node
npm install --global windows-build-tools
```

## 🔧 Environment Setup

### Linux Setup

```bash
# 1. Configure Solana for local development
solana config set --url localhost

# 2. Create a development wallet
solana-keygen new

# 3. View your wallet address
solana address

# 4. View configuration
solana config get

# 5. Start local validator (in a separate terminal)
solana-test-validator

# 6. Get test SOL (in original terminal)
solana airdrop 100
solana balance
```

### Windows Setup

Open **PowerShell**:

```powershell
# 1. Configure Solana for local development
solana config set --url localhost

# 2. Create a development wallet
solana-keygen new

# 3. View your wallet address
solana address

# 4. View configuration
solana config get

# 5. Start local validator (in a separate PowerShell window)
solana-test-validator

# 6. Get test SOL (in original PowerShell)
solana airdrop 100
solana balance
```

## 📁 Project Structure

```
hello-world/
├── programs/
│   └── hello-world/
│       └── src/
│           └── lib.rs          # Smart contract code
├── tests/
│   └── hello-world.ts          # Test file
├── migrations/
│   └── deploy.ts               # Deployment script
├── Anchor.toml                  # Anchor configuration
├── Cargo.toml                   # Rust dependencies
├── package.json                 # Node dependencies
└── .gitignore                   # Git ignore file
```

## 🚀 Usage

### 1. Create a New Project

**Linux:**
```bash
anchor init hello-world
cd hello-world
npm install
```

**Windows (PowerShell):**
```powershell
anchor init hello-world
cd hello-world
npm install
```

### 2. Write the Smart Contract

Edit `programs/hello-world/src/lib.rs`:

```rust
use anchor_lang::prelude::*;

// Program ID - will be updated by anchor keys sync
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod hello_world {
    use super::*;

    pub fn say_hello(ctx: Context<SayHello>) -> Result<()> {
        msg!("Hello, World! from Solana!");
        msg!("My first dApp is working! 🚀");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}
```

### 3. Write Tests

Edit `tests/hello-world.ts`:

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HelloWorld } from "../target/types/hello_world";
import { expect } from "chai";

describe("hello-world", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.HelloWorld as Program<HelloWorld>;

  it("Says hello! 🎉", async () => {
    // Call the sayHello instruction
    const tx = await program.methods.sayHello().rpc();
    
    console.log("Transaction signature:", tx);
    
    console.log("Transaction confirmed!");
  });
});
```

### 4. Build and Deploy

**Linux:**
```bash
# Build the program
anchor build

# Sync program ID (updates declare_id! with real ID)
anchor keys sync

# Deploy to local validator
anchor deploy
```

**Windows (PowerShell):**
```powershell
# Build the program
anchor build

# Sync program ID
anchor keys sync

# Deploy to local validator
anchor deploy
```

Expected output:
```
Deploying workspace: https://api.mainnet-beta.solana.com
Upgrade authority: ~/.config/solana/id.json
Deploying program "hello-world"...
Program Id: <your-program-id>
Deployment successful.
```

### 5. Run Tests

**Linux:**
```bash
# Make sure solana-test-validator is running in another terminal
anchor test --skip-local-validator
```

**Windows (PowerShell):**
```powershell
# Make sure solana-test-validator is running in another PowerShell window
anchor test --skip-local-validator
```

Expected output:
```
  hello-world
    ✓ Says hello! 🎉 (201ms)

  1 passing (201ms)

✨  Done in 2.34s.
```

### 6. View Program Logs

**Linux (open a third terminal):**
```bash
solana logs
```

**Windows (open a third PowerShell window):**
```powershell
solana logs
```

Then run your test again. You'll see:
```
Streaming transaction logs...
Transaction executed in slot 123:
  Program log: Hello, World! from Solana!
  Program log: My first dApp is working! 🚀
```

## 🧪 Testing

### Run All Tests

**Linux/Windows:**
```bash
# With local validator
anchor test

# Or if validator is already running
anchor test --skip-local-validator
```

## 🔍 Troubleshooting

### Linux Issues

| Issue | Solution |
|-------|----------|
| **"Account not funded"** | Run `solana airdrop 100` |
| **"Connection refused"** | Ensure `solana-test-validator` is running |
| **Program ID mismatch** | Run `anchor keys sync` |
| **Build fails** | Run `anchor clean` then `anchor build` |
| **Platform tools error** | Delete `~/.cache/solana` and rebuild |
| **Version mismatch** | Use Solana 2.1.21 with Anchor 0.32.1 |
| **"Error: unable to send transaction"** | Check wallet balance: `solana balance` |
| **"Blockhash not found"** | Validator might be syncing, wait a moment |

### Windows Issues

| Issue | Solution |
|-------|----------|
| **"command not found"** | Ensure PATH is set correctly, restart PowerShell |
| **Permission errors** | Run PowerShell as Administrator |
| **Build tools missing** | Install Visual Studio Build Tools with C++ features |
| **"link.exe" not found** | Add VC++ toolchain to PATH or open "Developer Command Prompt" |
| **Long path errors** | Enable long paths in Windows: `Set-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name "LongPathsEnabled" -Value 1` |
| **Firewall blocking** | Allow node/solana through Windows Firewall |
| **AVM install fails** | Install OpenSSL for Windows manually |

### Verify Your Setup

**Linux:**
```bash
# Quick verification script
echo "=== Development Environment ==="
echo "OS: $(cat /etc/os-release | grep PRETTY_NAME | cut -d'"' -f2)"
echo "Solana version: $(solana --version)"
echo "Anchor version: $(anchor --version)"
echo "Node version: $(node --version)"
echo "Rust version: $(rustc --version)"
echo "Wallet address: $(solana address)"
echo "Wallet balance: $(solana balance)"
echo "Current cluster: $(solana config get | grep "RPC URL")"
echo "Validator status: $(solana ping -c 1 2>&1 | head -1)"
```

**Windows (PowerShell):**
```powershell
# Quick verification script
Write-Host "=== Development Environment ===" -ForegroundColor Green
Write-Host "OS: $(Get-ComputerInfo | Select WindowsProductName -ExpandProperty WindowsProductName)"
Write-Host "Solana version: $(solana --version)"
Write-Host "Anchor version: $(anchor --version)"
Write-Host "Node version: $(node --version)"
Write-Host "Rust version: $(rustc --version)"
Write-Host "Wallet address: $(solana address)"
Write-Host "Wallet balance: $(solana balance)"
Write-Host "Current cluster: $(solana config get | Select-String 'RPC URL')"
```

## 📝 Additional Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Solana Stack Exchange](https://solana.stackexchange.com/)
- [Anchor GitHub Repository](https://github.com/coral-xyz/anchor)
- [Solana Cookbook](https://solanacookbook.com/)

## 🎯 Quick Reference Commands

### Linux Commands
```bash
# Start local validator
solana-test-validator

# Airdrop SOL
solana airdrop 100

# Build project
anchor build

# Sync program ID
anchor keys sync

# Deploy
anchor deploy

# Run tests
anchor test --skip-local-validator

# View logs
solana logs

# Check balance
solana balance

# Reset everything
rm -rf test-ledger
rm -rf ~/.cache/solana
```

### Windows Commands (PowerShell)
```powershell
# Start local validator
solana-test-validator

# Airdrop SOL
solana airdrop 100

# Build project
anchor build

# Sync program ID
anchor keys sync

# Deploy
anchor deploy

# Run tests
anchor test --skip-local-validator

# View logs
solana logs

# Check balance
solana balance

# Reset everything
Remove-Item -Recurse -Force test-ledger -ErrorAction SilentlyContinue
Remove-Item -Recurse -Force ~/.cache/solana -ErrorAction SilentlyContinue
```
