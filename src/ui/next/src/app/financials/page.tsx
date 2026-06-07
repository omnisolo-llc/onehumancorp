"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { AppShell } from "@/components/AppShell";

interface ApiLedgerEntry {
  entry_id: string;
  tx_id: string;
  account_id: string;
  direction: string;
  amount_cents: number;
}

interface ApiTransaction {
  tx_id: string;
  amount_cents: number;
  currency: string;
  timestamp: number;
  entries: ApiLedgerEntry[];
}

export default function FinancialsPage() {
  const [balance, setBalance] = useState<number | null>(null);
  const [currency, setCurrency] = useState<string>("USD");
  const [transactions, setTransactions] = useState<ApiTransaction[]>([]);
  const [loading, setLoading] = useState(true);

  const tenantId = typeof window !== "undefined" ? localStorage.getItem("tenant_id") || "DEFAULT" : "DEFAULT";
  const accountId = "main";

  useEffect(() => {
    async function fetchData() {
      try {
        const [balanceRes, statementRes] = await Promise.all([
          fetch(`/api/v1/ledger/balance?tenant_id=${tenantId}&account_id=${accountId}`),
          fetch(`/api/v1/ledger/statement?tenant_id=${tenantId}&account_id=${accountId}`)
        ]);

        if (balanceRes.ok) {
          const balanceData = await balanceRes.json();
          setBalance(balanceData.balance_cents);
          setCurrency(balanceData.currency);
        }

        if (statementRes.ok) {
          const statementData = await statementRes.json();
          setTransactions(statementData);
        }
      } catch (e) {
        console.error("Failed to fetch financials data", e);
      } finally {
        setLoading(false);
      }
    }

    fetchData();
  }, [tenantId]);

  const formatMoney = (cents: number | null, curr: string) => {
    if (cents === null) return "$0.00";
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: curr,
    }).format(cents / 100);
  };

  const formatDate = (timestamp: number) => {
    return new Date(timestamp * 1000).toLocaleDateString("en-US", {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });
  };

  return (
    <AppShell title="Financials & Ledger">
      <main className="p-4 md:p-8 max-w-5xl mx-auto space-y-6">
        <header className="mb-8">
          <div className="flex items-center gap-4 mb-2">
            <Link href="/dashboard" className="text-gray-500 hover:text-gray-800 dark:hover:text-white transition-colors">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
            </Link>
            <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">Unified Ledger</h1>
          </div>
          <p className="text-gray-600 dark:text-gray-400">Track multi-currency transactions and double-entry settlements automatically.</p>
        </header>

        <section className="glassmorphism rounded-[24px] p-8 border border-white/40 dark:border-white/10 shadow-lg">
          <div className="text-center">
            <h2 className="text-sm font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2">Total Available Balance</h2>
            <div className="text-5xl font-bold text-gray-900 dark:text-white font-outfit" id="ledger-balance-display">
              {loading ? (
                <div className="animate-pulse bg-gray-200 dark:bg-gray-700 h-12 w-48 rounded mx-auto"></div>
              ) : (
                formatMoney(balance, currency)
              )}
            </div>
            {!loading && <p className="text-sm text-green-600 dark:text-green-400 mt-2 flex items-center justify-center gap-1">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
              </svg>
              Settled & available
            </p>}
          </div>
        </section>

        <section className="mt-8">
          <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-6">Recent Transactions</h2>

          {loading ? (
            <div className="space-y-4">
              {[1, 2, 3].map((i) => (
                <div key={i} className="glassmorphism h-24 rounded-[16px] animate-pulse border border-white/20"></div>
              ))}
            </div>
          ) : transactions.length === 0 ? (
            <div className="text-center py-12 glassmorphism rounded-[16px] border border-white/20">
              <div className="text-gray-400 mb-2">No transactions recorded yet.</div>
              <p className="text-sm text-gray-500">Your double-entry ledger will automatically track new sales and settlements.</p>
            </div>
          ) : (
            <div className="space-y-4">
              {transactions.sort((a, b) => b.timestamp - a.timestamp).map((tx) => (
                <div key={tx.tx_id} className="glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10 hover:bg-white/50 dark:hover:bg-white/5 transition-all group">
                  <div className="flex justify-between items-start mb-4">
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="font-semibold text-gray-900 dark:text-white text-lg">{formatMoney(tx.amount_cents, tx.currency)}</span>
                        <span className="text-xs bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-300 px-2 py-0.5 rounded-full font-mono">{tx.tx_id.split("-")[0]}</span>
                      </div>
                      <div className="text-sm text-gray-500">{formatDate(tx.timestamp)}</div>
                    </div>
                  </div>

                  <div className="bg-gray-50/50 dark:bg-gray-900/30 rounded-xl p-4 mt-2">
                    <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-3">Double-Entry Breakdown</h4>
                    <div className="space-y-2">
                      {tx.entries.map((entry) => (
                        <div key={entry.entry_id} className="flex justify-between items-center text-sm">
                          <div className="flex items-center gap-2">
                            <span className={`w-2 h-2 rounded-full ${entry.direction === "CREDIT" ? "bg-purple-400" : "bg-teal-400"}`}></span>
                            <span className="text-gray-700 dark:text-gray-300 font-mono">{entry.account_id}</span>
                          </div>
                          <div className="flex items-center gap-4">
                            <span className={`text-xs px-2 py-0.5 rounded border ${entry.direction === "CREDIT" ? "border-purple-200 text-purple-700 dark:border-purple-800 dark:text-purple-400" : "border-teal-200 text-teal-700 dark:border-teal-800 dark:text-teal-400"}`}>
                              {entry.direction}
                            </span>
                            <span className="font-mono text-gray-900 dark:text-white w-20 text-right">{formatMoney(entry.amount_cents, tx.currency)}</span>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>
      </main>
    </AppShell>
  );
}
