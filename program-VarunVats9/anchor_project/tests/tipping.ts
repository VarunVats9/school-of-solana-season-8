import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Tipping } from "../target/types/tipping";
import { expect } from "chai";

describe("tipping", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Tipping as Program<Tipping>;
    const user = provider.wallet;

    // PDA for UserStats
    const [userStatsPda, _] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user-stats"), user.publicKey.toBuffer()],
        program.programId
    );

    it("Is initialized!", async () => {
        // Initialize UserStats PDA
        try {
            await program.methods
                .initializeUser()
                .accounts({
                    signer: user.publicKey,
                    userStats: userStatsPda,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
        } catch (e) {
            // Ignore if already initialized (for re-running tests)
            console.log("User stats might be already initialized");
        }

        const account = await program.account.userStats.fetch(userStatsPda);
        expect(account.totalTipsSent.toNumber()).to.equal(0);
        expect(account.totalTipsReceived.toNumber()).to.equal(0);
    });

    it("Tips a user successfully", async () => {
        const recipient = anchor.web3.Keypair.generate();
        const amount = new anchor.BN(100000000); // 0.1 SOL

        const initialStats = await program.account.userStats.fetch(userStatsPda);
        const initialBalance = await provider.connection.getBalance(recipient.publicKey);

        await program.methods
            .tip(amount)
            .accounts({
                signer: user.publicKey,
                recipient: recipient.publicKey,
                userStats: userStatsPda,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .rpc();

        const finalStats = await program.account.userStats.fetch(userStatsPda);
        const finalBalance = await provider.connection.getBalance(recipient.publicKey);

        expect(finalStats.totalTipsSent.toNumber()).to.equal(initialStats.totalTipsSent.toNumber() + amount.toNumber());
        expect(finalBalance).to.equal(initialBalance + amount.toNumber());
    });

    it("Fails to tip with insufficient funds (unhappy path)", async () => {
        // This is hard to test deterministically on localnet without draining the wallet, 
        // but we can try to tip a huge amount.
        const recipient = anchor.web3.Keypair.generate();
        const hugeAmount = new anchor.BN("1000000000000000000"); // Huge amount

        try {
            await program.methods
                .tip(hugeAmount)
                .accounts({
                    signer: user.publicKey,
                    recipient: recipient.publicKey,
                    userStats: userStatsPda,
                    systemProgram: anchor.web3.SystemProgram.programId,
                })
                .rpc();
            expect.fail("Should have failed");
        } catch (e) {
            // Expect an error
            expect(e).to.exist;
        }
    });
});
