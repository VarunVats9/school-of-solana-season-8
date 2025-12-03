![School of Solana](https://github.com/Ackee-Blockchain/school-of-solana/blob/master/.banner/banner.png?raw=true)

# School of Solana (Season 8) - Portfolio

This repository contains coursework, coding challenges, and final graduation project for the **Ackee Blockchain School of Solana (Season 8)**. The curriculum progressed from Rust fundamentals to complex Anchor program development and security auditing.

## 🚀 Final Graduation Project

**Decentralized Tipping Application**  
*A decentralized tipping dApp built on Solana where users can initialize their account statistics and send SOL tips to other users, with persistent tracking of all tipping activity.*

* **Program:** Rust & Anchor Framework
* **Frontend:** TypeScript & React
* **Solana Program ID:** `CeA7jNGCbhQvhAcWPceXNQtf13wm3oNiFtfD6tdU92PV`
* **Key Features:**
  - Initialize personal statistics account to track tipping history
  - Seamlessly transfer SOL to any Solana wallet
  - Automatically update and view total tips sent and received
  - Real-time stats updates after each transaction
  - PDA-based user statistics with deterministic addressing
  - Cross-Program Invocations (CPI) to System Program for SOL transfers
* **Live Demo:** [https://program-steel.vercel.app/](https://program-steel.vercel.app/)

### Program Architecture
The Tipping dApp uses a straightforward architecture with a single PDA account type to store user data. It leverages Cross-Program Invocations (CPI) to the System Program for transferring SOL.

**PDA Structure:**
- **UserStats PDA**: Derived from seeds `["user-stats", user_wallet_pubkey]`
- Ensures each user has exactly one deterministic stats account

**Program Instructions:**
- **initialize_user**: Initializes the `UserStats` account for the signer
- **tip**: Transfers SOL from signer to recipient using CPI to System Program

---

## 📂 Course Curriculum & Tasks

| Directory | Topic | Description |
| :--- | :--- | :--- |
| **task1-VarunVats9** | **Solana Theory** | Exploration of the Solana ecosystem, CLI tools, and theoretical core concepts including account model, PDAs, and transaction structure. |
| **task2-VarunVats9** | **Rust Fundamentals** | Implementation of geometric shapes with validation and trait implementations, plus a Calculator with arithmetic operations, history tracking, and overflow/underflow handling. |
| **task3-VarunVats9** | **On-Chain Vault** | A secure Vault program demonstrating **PDAs**, **CPIs**, state management, and deposit/withdraw logic with lock/unlock functionality. |
| **task4-VarunVats9** | **Decentralized Twitter** | A complex social dApp allowing users to tweet, like/dislike, and comment. Features unique PDA seeds for data relationships and content hash-based addressing. |
| **task5-VarunVats9** | **Security Audit** | Analysis of Solana security mechanisms, runtime policies, and common vulnerabilities in SPL programs and Anchor applications. |
| **program-VarunVats9** | **Final Project** | Decentralized Tipping Application - the graduation project showcasing learned concepts in production. |

## 🛠 Tech Stack

* **Languages:** Rust, TypeScript
* **Frameworks:** Anchor (v0.31.1), React
* **Tools:** Solana CLI (v2.2.12), Mocha/Chai (Testing), Vercel
* **Blockchain:** Solana (Devnet)

## 📊 Key Concepts Demonstrated

- **Program Derived Addresses (PDAs)** - Deterministic account generation
- **Cross-Program Invocations (CPIs)** - Calling other programs on-chain
- **Account Validation** - Secure account ownership and signer verification
- **State Management** - On-chain data storage and updates
- **Error Handling** - Custom error types and proper error propagation
- **Event Emission** - Logging important state changes
- **Rent Exemption** - Account lifecycle and lamport management
- **Security Best Practices** - Anchor constraints and runtime validations

## 🎓 Learning Outcomes

- Deep understanding of Solana's account model and programming paradigm
- Proficiency in Rust programming for blockchain development
- Hands-on experience with Anchor framework for rapid dApp development
- Knowledge of common security vulnerabilities and mitigation strategies
- Full-stack blockchain development from smart contracts to user interfaces

---

## 📝 License

Educational project completed as part of the Ackee Blockchain School of Solana (Season 8).

## 🔗 Useful Links

- [Ackee Blockchain](https://ackee.xyz/)
- [School of Solana](https://github.com/Ackee-Blockchain/school-of-solana)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Documentation](https://www.anchor-lang.com/)
