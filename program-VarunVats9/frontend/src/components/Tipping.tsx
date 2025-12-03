"use client";

import { FC, useEffect, useState } from "react";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, web3, BN } from "@coral-xyz/anchor";

import { IDL, Tipping } from "../utils/idl";

const PROGRAM_ID = new web3.PublicKey("CeA7jNGCbhQvhAcWPceXNQtf13wm3oNiFtfD6tdU92PV");

export const TippingUI: FC = () => {
    const { connection } = useConnection();
    const wallet = useWallet();

    const [amount, setAmount] = useState("");
    const [recipient, setRecipient] = useState("");
    const [stats, setStats] = useState<{ sent: number; received: number } | null>(null);
    const [loading, setLoading] = useState(false);

    const getProgram = () => {
        if (!wallet.publicKey) return null;
        const provider = new AnchorProvider(connection, wallet as any, {
            preflightCommitment: "processed",
        });
        return new Program<Tipping>(IDL, PROGRAM_ID, provider);
    };

    const fetchStats = async () => {
        const program = getProgram();
        if (!program || !wallet.publicKey) return;

        try {
            const [userStatsPda] = web3.PublicKey.findProgramAddressSync(
                [Buffer.from("user-stats"), wallet.publicKey.toBuffer()],
                program.programId
            );
            const account = await program.account.userStats.fetch(userStatsPda);
            setStats({
                sent: account.totalTipsSent.toNumber(),
                received: account.totalTipsReceived.toNumber(),
            });
        } catch (e) {
            console.log("Stats not initialized or error fetching:", e);
            setStats(null);
        }
    };

    useEffect(() => {
        if (wallet.connected) {
            fetchStats();
        } else {
            setStats(null);
        }
    }, [wallet.connected]);

    const initialize = async () => {
        const program = getProgram();
        if (!program || !wallet.publicKey) return;
        setLoading(true);
        try {
            const [userStatsPda] = web3.PublicKey.findProgramAddressSync(
                [Buffer.from("user-stats"), wallet.publicKey.toBuffer()],
                program.programId
            );

            await program.methods
                .initializeUser()
                .accounts({
                    signer: wallet.publicKey,
                    userStats: userStatsPda,
                    systemProgram: web3.SystemProgram.programId,
                })
                .rpc();

            await fetchStats();
            alert("Initialized successfully!");
        } catch (e) {
            console.error(e);
            alert("Error initializing: " + e);
        } finally {
            setLoading(false);
        }
    };

    const sendTip = async () => {
        const program = getProgram();
        if (!program || !wallet.publicKey) return;
        setLoading(true);
        try {
            const [userStatsPda] = web3.PublicKey.findProgramAddressSync(
                [Buffer.from("user-stats"), wallet.publicKey.toBuffer()],
                program.programId
            );

            const lamports = new BN(parseFloat(amount) * web3.LAMPORTS_PER_SOL);
            const recipientPubkey = new web3.PublicKey(recipient);

            await program.methods
                .tip(lamports)
                .accounts({
                    signer: wallet.publicKey,
                    recipient: recipientPubkey,
                    userStats: userStatsPda,
                    systemProgram: web3.SystemProgram.programId,
                })
                .rpc();

            await fetchStats();
            alert("Tip sent successfully!");
            setAmount("");
            setRecipient("");
        } catch (e) {
            console.error(e);
            alert("Error sending tip: " + e);
        } finally {
            setLoading(false);
        }
    };

    if (!wallet.connected) {
        return (
            <div className="flex justify-center items-center h-64">
                <p className="text-xl">Please connect your wallet to continue.</p>
            </div>
        );
    }

    return (
        <div className="max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-md text-black">
            <h2 className="text-2xl font-bold mb-6 text-center">DeFi Tipping</h2>

            {stats ? (
                <div className="mb-6 p-4 bg-gray-100 rounded">
                    <h3 className="font-semibold mb-2">My Stats</h3>
                    <p>Total Sent: {(stats.sent / web3.LAMPORTS_PER_SOL).toFixed(4)} SOL</p>
                    <p>Total Received: {(stats.received / web3.LAMPORTS_PER_SOL).toFixed(4)} SOL</p>
                </div>
            ) : (
                <div className="mb-6 text-center">
                    <p className="mb-4">Account not initialized.</p>
                    <button
                        onClick={initialize}
                        disabled={loading}
                        className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 disabled:opacity-50"
                    >
                        {loading ? "Initializing..." : "Initialize Stats"}
                    </button>
                </div>
            )}

            <div className="space-y-4">
                <div>
                    <label className="block text-sm font-medium text-gray-700">Recipient Address</label>
                    <input
                        type="text"
                        value={recipient}
                        onChange={(e) => setRecipient(e.target.value)}
                        className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm p-2 border"
                        placeholder="Public Key"
                    />
                </div>
                <div>
                    <label className="block text-sm font-medium text-gray-700">Amount (SOL)</label>
                    <input
                        type="number"
                        value={amount}
                        onChange={(e) => setAmount(e.target.value)}
                        className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm p-2 border"
                        placeholder="0.1"
                    />
                </div>
                <button
                    onClick={sendTip}
                    disabled={loading || !stats}
                    className="w-full bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600 disabled:opacity-50"
                >
                    {loading ? "Processing..." : "Send Tip"}
                </button>
            </div>
        </div>
    );
};
