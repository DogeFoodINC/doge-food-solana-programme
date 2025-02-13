# Solana Dog Food Payment & Splitter Program

This Solana-based program facilitates payments for dog food on a website and automatically splits incoming payments between two wallets. 80% of the funds are allocated to the team wallet, while 20% is directed to a donation wallet, ensuring a smooth and transparent process for both business and charity. (Percentage split is configurable)

# Features

> - Payment Acceptance: Accepts payments for dog food via Solana transactions.
> - Automatic Payment Split: 80% of the incoming payment goes to the team wallet, and 20% is sent to a donation wallet.
> - Seamless Integration: Can be integrated into a website to accept payments directly from users.
> - Transparent Donation: Helps track donations made towards a specific cause (e.g., charity or animal rescue).

# Architecture

The program is built on Solana's blockchain and uses smart contracts to:

> - Checks and processes incoming orders.
> - Split the payments according to predefined percentages (80% to the team, 20% to donations).
> - Ensure the funds are transferred securely and automatically to the designated wallets.

# Installation

## Prerequisites

> - Solana CLI: Ensure you have the Solana CLI installed to interact with the Solana blockchain.
> - Rust: This project is written in Rust, so you’ll need the Rust toolchain set up.
> - Anchor: Install it and simplify smart contract deployment.
> - Install Solana CLI: Follow the official Solana installation guide: https://docs.solana.com/cli/install-solana-cli-tools
> - Install Rust: Install Rust from the official website: https://www.rust-lang.org/tools/install
> - Install Anchor:
`$ npm install -g @project-serum/anchor-cli`

## Clone the Repository
- `git clone https://github.com/yourusername/solana-dogfood-payment-splitter.git`
- `cd solana-dogfood-payment-splitter`

## Deploy the Program
To deploy the Solana smart contract, follow these steps:

> - Build the program: `anchor build`
> - Deploy the program to the Solana devnet/testnet/mainnet: `anchor deploy`
Make sure you have a Solana wallet and some SOL in your wallet for deployment costs.
