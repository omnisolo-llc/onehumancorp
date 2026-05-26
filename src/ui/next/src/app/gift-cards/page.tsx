"use client";

import React, { useState, useEffect } from "react";
import Link from 'next/link';
import { openDB } from "idb";

const DB_NAME = "ohc_offline_ledger";
const STORE_NAME = "tx_queue";

// Helper to interact with IndexedDB for offline support
async function getOfflineDB() {
    return openDB(DB_NAME, 1, {
        upgrade(db) {
            if (!db.objectStoreNames.contains(STORE_NAME)) {
                db.createObjectStore(STORE_NAME, { keyPath: "id", autoIncrement: true });
            }
        },
    });
}

export default function GiftCardsPage() {
    const [giftCards, setGiftCards] = useState([]);
    const [loading, setLoading] = useState(true);
    const [isIssuing, setIsIssuing] = useState(false);
    const [amount, setAmount] = useState("");
    const [phone, setPhone] = useState("");
    const [type, setType] = useState("GIFT_CARD");
    const [redeemCode, setRedeemCode] = useState("");
    const [redeemAmount, setRedeemAmount] = useState("");
    const [isRedeeming, setIsRedeeming] = useState(false);
    const [isOffline, setIsOffline] = useState(false);
    const [offlineQueueLength, setOfflineQueueLength] = useState(0);

    useEffect(() => {
        fetchGiftCards();
        checkOfflineQueue();

        const handleOnline = () => {
            setIsOffline(false);
            syncOfflineQueue();
        };
        const handleOffline = () => setIsOffline(true);

        window.addEventListener("online", handleOnline);
        window.addEventListener("offline", handleOffline);

        if (!navigator.onLine) {
            setIsOffline(true);
        }

        return () => {
            window.removeEventListener("online", handleOnline);
            window.removeEventListener("offline", handleOffline);
        };
    }, []);

    const checkOfflineQueue = async () => {
        try {
            const db = await getOfflineDB();
            const txs = await db.getAll(STORE_NAME);
            setOfflineQueueLength(txs.length);
        } catch (e) {
            console.error("Error checking offline queue", e);
        }
    };

    const syncOfflineQueue = async () => {
        try {
            const db = await getOfflineDB();
            const txs = await db.getAll(STORE_NAME);
            if (txs.length === 0) return;

            for (const tx of txs) {
                try {
                    const res = await fetch(tx.url, {
                        method: tx.method,
                        headers: { "Content-Type": "application/json" },
                        body: JSON.stringify(tx.body)
                    });

                    if (res.ok) {
                        await db.delete(STORE_NAME, tx.id);
                    }
                } catch (err) {
                    console.error("Failed to sync tx", tx, err);
                }
            }

            checkOfflineQueue();
            fetchGiftCards();
        } catch (e) {
            console.error("Error syncing offline queue", e);
        }
    };

    const fetchGiftCards = async () => {
        if (!navigator.onLine) {
            setLoading(false);
            return;
        }
        try {
            const res = await fetch("/api/v1/gift-cards");
            if (res.ok) {
                const data = await res.json();
                setGiftCards(data);
            }
        } catch (error) {
            console.error("Error fetching gift cards", error);
        } finally {
            setLoading(false);
        }
    };

    const addToOfflineQueue = async (url: string, method: string, body: any) => {
        try {
            const db = await getOfflineDB();
            await db.add(STORE_NAME, { url, method, body, timestamp: Date.now() });
            checkOfflineQueue();
        } catch (e) {
            console.error("Failed to add to offline queue", e);
        }
    };

    const handleIssue = async () => {
        if (!amount || isNaN(Number(amount)) || Number(amount) <= 0) {
            alert("Amount must be a positive number.");
            return;
        }

        setIsIssuing(true);
        const payload = {
            amount: Number(amount),
            customer_id: phone || null,
            type_: type
        };

        if (!navigator.onLine) {
            await addToOfflineQueue("/api/v1/gift-cards", "POST", payload);
            alert("Offline mode: Issuance queued for sync.");
            setAmount("");
            setPhone("");
            setIsIssuing(false);
            return;
        }

        try {
            const res = await fetch("/api/v1/gift-cards", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
            });

            if (res.ok) {
                setAmount("");
                setPhone("");
                fetchGiftCards();
                // Placeholder: Here you would normally coordinate with AI Operations Agent for Wallet pass generation
                // await generateAppleWalletPass(data.gift_card.id);
            } else {
                alert("Failed to issue gift card.");
            }
        } catch (error) {
            console.error("Error issuing gift card", error);
        } finally {
            setIsIssuing(false);
        }
    };

    const handleRedeem = async () => {
        if (!redeemCode || !redeemAmount || isNaN(Number(redeemAmount)) || Number(redeemAmount) <= 0) {
            alert("Amount must be a positive number.");
            return;
        }

        setIsRedeeming(true);
        const payload = {
            code: redeemCode,
            amount: Number(redeemAmount),
            transaction_ref: "MANUAL_REDEEM_OFFLINE_CAPABLE"
        };

        if (!navigator.onLine) {
            await addToOfflineQueue("/api/v1/gift-cards/redeem", "POST", payload);
            alert("Offline mode: Redemption queued for sync. Final confirmation depends on actual balance upon reconnect.");
            setRedeemCode("");
            setRedeemAmount("");
            setIsRedeeming(false);
            return;
        }

        try {
            const res = await fetch("/api/v1/gift-cards/redeem", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(payload)
            });

            if (res.ok) {
                setRedeemCode("");
                setRedeemAmount("");
                fetchGiftCards();
                alert("Redeemed successfully");
            } else {
                alert("Failed to redeem. Check code and balance.");
            }
        } catch (error) {
            console.error("Error redeeming gift card", error);
        } finally {
            setIsRedeeming(false);
        }
    };

    const formatDate = (dateString: string) => {
        const date = new Date(dateString);
        return date.toLocaleDateString();
    };

    return (
        <div className="flex min-h-screen bg-gray-900 text-white">
            <div className="w-64 bg-gray-900 border-r border-gray-800 hidden md:block">
               <div className="p-4">
                 <Link href="/dashboard" className="text-xl font-bold text-white mb-8 block">OHC</Link>
                 <nav className="space-y-2">
                    <Link href="/dashboard" className="block px-4 py-2 text-gray-400 hover:text-white rounded hover:bg-gray-800">Dashboard</Link>
                    <Link href="/gift-cards" className="block px-4 py-2 text-white bg-gray-800 rounded">Gift Cards</Link>
                 </nav>
               </div>
            </div>

            <div className="flex-1 p-4 md:p-8 overflow-y-auto w-full relative">
                {isOffline && (
                    <div className="sticky top-0 z-50 bg-yellow-600 text-white text-center py-2 px-4 rounded-b-lg mb-4 shadow-lg font-medium text-sm animate-pulse">
                        Offline Mode: Queuing {offlineQueueLength > 0 ? `${offlineQueueLength} ` : ''}Redemptions and Issuances
                    </div>
                )}

                <div className="max-w-4xl mx-auto space-y-8 pb-12 mt-4">
                    <header>
                        <h1 className="text-3xl font-bold bg-gradient-to-r from-white to-gray-400 bg-clip-text text-transparent">
                            Gift Cards & Store Credit
                        </h1>
                        <p className="text-gray-400 mt-2">
                            Manage omnichannel gift cards and store credit ledgers.
                        </p>
                    </header>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div className="mac-glass-container p-6 rounded-2xl shadow-xl border border-white/10">
                            <h2 className="text-xl font-semibold mb-4 text-white">Issue New</h2>

                            <div className="space-y-4">
                                <div>
                                    <label className="block text-sm text-gray-400 mb-1">Type</label>
                                    <select
                                        className="w-full bg-gray-800/50 border border-gray-700 rounded-lg p-3 text-white focus:outline-none focus:border-blue-500"
                                        value={type}
                                        onChange={(e) => setType(e.target.value)}
                                    >
                                        <option value="GIFT_CARD">Gift Card</option>
                                        <option value="STORE_CREDIT">Store Credit</option>
                                    </select>
                                </div>

                                <div>
                                    <label className="block text-sm text-gray-400 mb-1">Amount ($)</label>
                                    <input
                                        type="number"
                                        placeholder="e.g. 50"
                                        min="0.01"
                                        step="0.01"
                                        className="w-full bg-gray-800/50 border border-gray-700 rounded-lg p-3 text-white focus:outline-none focus:border-blue-500"
                                        value={amount}
                                        onChange={(e) => setAmount(e.target.value)}
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm text-gray-400 mb-1">Customer Phone (Optional)</label>
                                    <input
                                        type="text"
                                        placeholder="e.g. 555-0123"
                                        className="w-full bg-gray-800/50 border border-gray-700 rounded-lg p-3 text-white focus:outline-none focus:border-blue-500"
                                        value={phone}
                                        onChange={(e) => setPhone(e.target.value)}
                                    />
                                </div>

                                <button
                                    onClick={handleIssue}
                                    disabled={isIssuing || !amount}
                                    className="w-full py-3 bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white rounded-lg font-medium transition-all shadow-lg disabled:opacity-50 mt-2 flex items-center justify-center gap-2"
                                >
                                    {isIssuing ? "Issuing..." : "+ Issue to Customer"}
                                    {isOffline && <span className="w-2 h-2 rounded-full bg-yellow-400 inline-block ml-1"></span>}
                                </button>
                            </div>
                        </div>

                        <div className="mac-glass-container p-6 rounded-2xl shadow-xl border border-white/10">
                            <h2 className="text-xl font-semibold mb-4 text-white">Redeem</h2>

                            <div className="space-y-4">
                                <div>
                                    <label className="block text-sm text-gray-400 mb-1">Code</label>
                                    <input
                                        type="text"
                                        placeholder="e.g. GC-A1B2C3D4"
                                        className="w-full bg-gray-800/50 border border-gray-700 rounded-lg p-3 text-white uppercase focus:outline-none focus:border-blue-500"
                                        value={redeemCode}
                                        onChange={(e) => setRedeemCode(e.target.value.toUpperCase())}
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm text-gray-400 mb-1">Amount ($)</label>
                                    <input
                                        type="number"
                                        placeholder="e.g. 10"
                                        min="0.01"
                                        step="0.01"
                                        className="w-full bg-gray-800/50 border border-gray-700 rounded-lg p-3 text-white focus:outline-none focus:border-blue-500"
                                        value={redeemAmount}
                                        onChange={(e) => setRedeemAmount(e.target.value)}
                                    />
                                </div>

                                <button
                                    onClick={handleRedeem}
                                    disabled={isRedeeming || !redeemCode || !redeemAmount}
                                    className="w-full py-3 bg-gray-800 hover:bg-gray-700 text-white rounded-lg font-medium border border-gray-700 transition-all disabled:opacity-50 mt-2 flex items-center justify-center gap-2"
                                >
                                    {isRedeeming ? "Redeeming..." : "Redeem Amount"}
                                    {isOffline && <span className="w-2 h-2 rounded-full bg-yellow-400 inline-block ml-1"></span>}
                                </button>
                            </div>

                            <div className="mt-6 pt-6 border-t border-gray-800">
                                <div className="p-4 bg-gray-800/50 rounded-xl border border-gray-700 flex items-center justify-center cursor-pointer hover:bg-gray-800 transition-colors">
                                    <div className="text-center">
                                        <div className="w-12 h-12 bg-gray-700 rounded-full flex items-center justify-center mx-auto mb-2">
                                            <svg className="w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"></path><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
                                        </div>
                                        <span className="text-sm font-medium text-gray-300">Scan QR Code</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div className="mac-glass-container p-6 rounded-2xl shadow-xl border border-white/10 mt-8">
                        <h2 className="text-xl font-semibold mb-6 text-white">Active Ledger</h2>

                        {loading ? (
                            <div className="text-center py-10 text-gray-400">Loading...</div>
                        ) : giftCards.length === 0 ? (
                            <div className="text-center py-10 text-gray-500">No gift cards or store credit issued yet.</div>
                        ) : (
                            <div className="overflow-x-auto">
                                <table className="w-full text-left border-collapse">
                                    <thead>
                                        <tr className="text-sm text-gray-400 border-b border-gray-800">
                                            <th className="pb-3 font-medium">Code</th>
                                            <th className="pb-3 font-medium">Type</th>
                                            <th className="pb-3 font-medium">Balance</th>
                                            <th className="pb-3 font-medium">Status</th>
                                            <th className="pb-3 font-medium text-right">Created</th>
                                        </tr>
                                    </thead>
                                    <tbody className="text-sm">
                                        {giftCards.map((gc: any) => (
                                            <tr key={gc.id} className="border-b border-gray-800/50 hover:bg-gray-800/30 transition-colors">
                                                <td className="py-4 font-mono text-gray-300">
                                                    <div className="flex items-center gap-2">
                                                        {gc.code}
                                                        <button
                                                            className="text-gray-500 hover:text-white transition-colors"
                                                            title="Generate Apple Wallet Pass Placeholder"
                                                            onClick={() => alert(`Coordination triggered for Operations Agent to generate Wallet Pass for ${gc.code}`)}
                                                        >
                                                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" /></svg>
                                                        </button>
                                                    </div>
                                                </td>
                                                <td className="py-4">
                                                    <span className={`px-2 py-1 rounded-md text-xs ${gc.type_ === 'STORE_CREDIT' ? 'bg-purple-500/20 text-purple-400' : 'bg-blue-500/20 text-blue-400'}`}>
                                                        {gc.type_.replace('_', ' ')}
                                                    </span>
                                                </td>
                                                <td className="py-4 font-semibold">${gc.current_balance.toFixed(2)}</td>
                                                <td className="py-4">
                                                    <span className={`px-2 py-1 rounded-md text-xs ${gc.status === 'ACTIVE' ? 'bg-green-500/20 text-green-400' : 'bg-gray-500/20 text-gray-400'}`}>
                                                        {gc.status}
                                                    </span>
                                                </td>
                                                <td className="py-4 text-right text-gray-500">
                                                    {formatDate(gc.created_at)}
                                                </td>
                                            </tr>
                                        ))}
                                    </tbody>
                                </table>
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
}
