'use client';
import React, { useState, useEffect } from 'react';
import AppShell from '@/components/AppShell';

export default function LedgerPage() {
    const [balance, setBalance] = useState(0);
    const [taxSaved, setTaxSaved] = useState(0);

    useEffect(() => {
        const fetchLedgerData = async () => {
            try {
                // Fetch actual data
                const resTax = await fetch(`/api/ledger/tax-savings/mock-tenant`);
                if (resTax.ok) {
                    const data = await resTax.json();
                    setTaxSaved(data.total_saved || 0);
                }

                // For simplicity, using a static balance based on instructions
                setBalance(1500.00);
            } catch (error) {
                console.error("Failed to fetch ledger data", error);
            }
        };

        fetchLedgerData();
    }, []);

    return (
        <AppShell>
            <main className="p-4 md:p-8 max-w-4xl mx-auto space-y-6 pt-24">
                <header className="mb-8">
                    <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Financials</h1>
                    <p className="text-gray-600 dark:text-gray-400 mt-2">Unified view of your financial health and automated tax savings.</p>
                </header>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                    {/* Total Balance Card */}
                    <div className="glassmorphism p-6 rounded-2xl border border-white/40 dark:border-white/10 relative overflow-hidden">
                        <div className="absolute top-0 right-0 p-4 opacity-10">
                            <span className="text-6xl">💰</span>
                        </div>
                        <h2 className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-2">Total Balance</h2>
                        <div className="text-4xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">
                            ${balance.toFixed(2)} <span className="text-lg text-gray-400 font-normal">USD</span>
                        </div>
                        <div className="mt-4 flex items-center text-sm text-emerald-600 dark:text-emerald-400 font-medium">
                            <span>+ 12.5% from last month</span>
                        </div>
                    </div>

                    {/* Tax Savings Envelope Card */}
                    <div className="glassmorphism p-6 rounded-2xl border border-white/40 dark:border-white/10 relative overflow-hidden">
                         <div className="absolute top-0 right-0 p-4 opacity-10">
                            <span className="text-6xl">🏦</span>
                        </div>
                        <div className="flex justify-between items-start mb-2">
                            <h2 className="text-sm font-semibold text-gray-500 uppercase tracking-wider">Estimated Taxes Saved</h2>
                            <span className="bg-blue-100 text-blue-800 text-xs px-2 py-1 rounded-full font-medium dark:bg-blue-900/30 dark:text-blue-300">Auto-Envelope</span>
                        </div>
                        <div className="text-4xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">
                            ${taxSaved.toFixed(2)}
                        </div>
                        <p className="text-sm text-gray-600 dark:text-gray-400 mt-2">
                            Automatically setting aside 30% of revenue for taxes.
                        </p>
                    </div>
                </div>

                 <section className="mt-8">
                    <div className="flex justify-between items-center mb-4">
                        <h2 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">Recent Activity</h2>
                        <button className="text-sm text-blue-600 dark:text-blue-400 font-medium hover:underline">Ledger Statement</button>
                    </div>

                    <div className="glassmorphism rounded-2xl border border-white/40 dark:border-white/10 overflow-hidden">
                        <div className="p-4 border-b border-gray-100 dark:border-gray-800 flex justify-between items-center bg-white/50 dark:bg-black/20">
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 rounded-full bg-emerald-100 dark:bg-emerald-900/30 flex items-center justify-center text-emerald-600">
                                    💳
                                </div>
                                <div>
                                    <p className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">POS Sale - Terminal</p>
                                    <p className="text-xs text-gray-500">Today at 2:30 PM</p>
                                </div>
                            </div>
                            <div className="text-right">
                                <p className="font-bold text-[#1D1D1F] dark:text-[#F5F5F7] text-lg">+$120.00</p>
                                <p className="text-xs text-blue-500 font-medium">+$36.00 to Tax Envelope</p>
                            </div>
                        </div>

                         <div className="p-4 flex justify-between items-center">
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 rounded-full bg-emerald-100 dark:bg-emerald-900/30 flex items-center justify-center text-emerald-600">
                                    🌐
                                </div>
                                <div>
                                    <p className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">Online Order #1042</p>
                                    <p className="text-xs text-gray-500">Yesterday at 10:15 AM</p>
                                </div>
                            </div>
                            <div className="text-right">
                                <p className="font-bold text-[#1D1D1F] dark:text-[#F5F5F7] text-lg">+$85.00</p>
                                <p className="text-xs text-blue-500 font-medium">+$25.50 to Tax Envelope</p>
                            </div>
                        </div>
                    </div>
                </section>
            </main>
        </AppShell>
    );
}
