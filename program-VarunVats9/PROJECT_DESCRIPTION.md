# Project Description

**Deployed Frontend URL:** https://program-steel.vercel.app/

**Solana Program ID:** `CeA7jNGCbhQvhAcWPceXNQtf13wm3oNiFtfD6tdU92PV`

## Project Overview

### Description
A decentralized tipping application built on Solana. Users can initialize their account statistics and send SOL tips to other users. The application tracks the total amount of tips sent and received by each user using a Program Derived Address (PDA), ensuring persistent and secure data storage.

### Key Features
- **Initialize Stats**: Create a personal statistics account to track your tipping history.
- **Send Tips**: Seamlessly transfer SOL to any other Solana wallet.
- **Track History**: Automatically update and view your total tips sent and received.
- **Real-time Updates**: View your stats updating instantly after each transaction.

### How to Use the dApp
1. **Connect Wallet**: Connect your Phantom or Solflare wallet.
2. **Initialize**: Click "Initialize Stats" to create your on-chain data account (one-time action).
3. **Enter Recipient**: Paste the public key of the user you want to tip.
4. **Enter Amount**: Specify the amount of SOL to send.
5. **Send Tip**: Click "Send Tip" to transfer funds and update your stats.

## Program Architecture
The Tipping dApp uses a straightforward architecture with a single PDA account type to store user data. It leverages Cross-Program Invocations (CPI) to the System Program for transferring SOL.

### PDA Usage
The program uses Program Derived Addresses to create unique statistic accounts for each user.

**PDAs Used:**
- **UserStats PDA**: Derived from seeds `["user-stats", user_wallet_pubkey]`. This ensures each user has exactly one deterministic stats account that they control.

### Program Instructions
**Instructions Implemented:**
- **initialize_user**: Initializes the `UserStats` account for the signer, setting sent/received counters to 0.
- **tip**: Transfers SOL from the signer to the recipient using a CPI to the System Program, and increments the signer's `total_tips_sent`.

### Account Structure
```rust
#[account]
pub struct UserStats {
    pub total_tips_sent: u64,     // Total lamports sent by this user
    pub total_tips_received: u64, // Total lamports received by this user (future feature)
    pub bump: u8,                 // Bump seed for the PDA
}
```

## Testing

### Test Coverage
The project includes a TypeScript test suite using Mocha and Chai to verify all program functionalities.

**Happy Path Tests:**
- **Is initialized!**: Verifies that a user can successfully initialize their `UserStats` account and that default values are 0.
- **Tips a user successfully**: Checks that SOL is correctly transferred to the recipient and that the sender's `total_tips_sent` is updated correctly.

**Unhappy Path Tests:**
- **Fails to tip with insufficient funds**: Ensures the transaction fails if the user tries to send more SOL than they possess (simulated with a huge amount).

### Running Tests
```bash
cd anchor_project
anchor test
```

### Additional Notes for Evaluators
This project demonstrates the use of PDAs for user-specific data storage and CPIs for native SOL transfers. The frontend integrates with the Solana Wallet Adapter to provide a seamless user experience.