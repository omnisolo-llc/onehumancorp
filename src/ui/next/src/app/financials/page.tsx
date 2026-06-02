"use client";

import { useEffect, useState } from "react";

interface Transaction {
  tx_id: string;
  amount: number;
  currency: string;
  timestamp: string;
  direction: string;
}

export default function FinancialsDashboard() {
  const [balance, setBalance] = useState<number>(0);
  const [currency, setCurrency] = useState<string>("USD");
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [loading, setLoading] = useState<boolean>(true);
  const [pendingDeposits, setPendingDeposits] = useState<number>(0);

  useEffect(() => {
    // Simulated fetch - replace with real API call
    setTimeout(() => {
      setBalance(425000); // $4,250.00
      setCurrency("USD");
      setPendingDeposits(15000); // $150.00
      setTransactions([
        {
          tx_id: "tx-1",
          amount: 25000, // $250.00
          currency: "USD",
          timestamp: new Date().toISOString(),
          direction: "CREDIT",
        },
        {
          tx_id: "tx-2",
          amount: 5000, // $50.00
          currency: "USD",
          timestamp: new Date(Date.now() - 86400000).toISOString(),
          direction: "DEBIT",
        },
      ]);
      setLoading(false);
    }, 1000);
  }, []);

  const formatCurrency = (amount: number, curr: string) => {
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: curr,
    }).format(amount / 100);
  };

  if (loading) {
    return (
      <div className="flex h-screen items-center justify-center bg-gray-50/50">
        <div className="h-12 w-12 animate-spin rounded-full border-4 border-[#0066FF] border-t-transparent"></div>
      </div>
    );
  }

  return (
    <div className="min-h-screen p-4 sm:p-8" style={{ backgroundColor: "#f5f5f7" }}>
      <div className="mx-auto max-w-4xl space-y-6">
        <h1 className="text-3xl font-semibold tracking-tight text-[#1D1D1F] font-outfit">Financials</h1>

        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
          {/* Total Balance Card */}
          <div
            className="rounded-2xl p-6 shadow-sm flex flex-col justify-between"
            style={{
              background: "rgba(255, 255, 255, 0.65)",
              backdropFilter: "blur(30px) saturate(210%)",
              border: "1px solid rgba(255, 255, 255, 0.4)",
            }}
          >
            <h2 className="text-sm font-medium text-gray-500 font-inter">Total Balance</h2>
            <div className="mt-2 text-4xl font-semibold tracking-tight text-[#1D1D1F] font-outfit">
              {formatCurrency(balance, currency)}
            </div>
            <div className="mt-4 flex items-center text-sm font-medium text-[#34C759]">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 10l7-7m0 0l7 7m-7-7v18" />
              </svg>
              Updated just now
            </div>
          </div>

          {/* Pending Deposits Card */}
          <div
            className="rounded-2xl p-6 shadow-sm flex flex-col justify-between"
            style={{
              background: "rgba(255, 255, 255, 0.65)",
              backdropFilter: "blur(30px) saturate(210%)",
              border: "1px solid rgba(255, 255, 255, 0.4)",
            }}
          >
            <h2 className="text-sm font-medium text-gray-500 font-inter">Pending Deposits</h2>
            <div className="mt-2 text-4xl font-semibold tracking-tight text-[#1D1D1F] font-outfit">
              {formatCurrency(pendingDeposits, currency)}
            </div>
            <div className="mt-4 flex items-center text-sm font-medium text-[#FF9500]">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 mr-1" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              Processing
            </div>
          </div>
        </div>

        {/* Recent Activity */}
        <div
          className="rounded-2xl overflow-hidden shadow-sm"
          style={{
            background: "rgba(255, 255, 255, 0.65)",
            backdropFilter: "blur(30px) saturate(210%)",
            border: "1px solid rgba(255, 255, 255, 0.4)",
          }}
        >
          <div className="px-6 py-5 border-b border-gray-100">
            <h2 className="text-lg font-medium text-[#1D1D1F] font-outfit">Recent Activity</h2>
          </div>
          <ul className="divide-y divide-gray-100">
            {transactions.map((tx) => (
              <li key={tx.tx_id} className="px-6 py-4 hover:bg-white/50 transition-colors duration-200 cursor-pointer">
                <div className="flex items-center justify-between">
                  <div className="flex items-center space-x-4">
                    <div className={`flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full ${tx.direction === 'CREDIT' ? 'bg-green-50 text-[#34C759]' : 'bg-red-50 text-[#FF3B30]'}`}>
                       {tx.direction === 'CREDIT' ? (
                        <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                          <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-8.707l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.414V13a1 1 0 102 0V9.414l1.293 1.293a1 1 0 001.414-1.414z" clipRule="evenodd" />
                        </svg>
                       ) : (
                         <svg xmlns="http://www.w3.org/2000/svg" className="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                           <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v3.586L7.707 9.293a1 1 0 00-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L11 10.586V7z" clipRule="evenodd" />
                         </svg>
                       )}
                    </div>
                    <div>
                      <p className="text-sm font-medium text-[#1D1D1F] font-inter">
                        {tx.direction === 'CREDIT' ? 'Payment Received' : 'Withdrawal'}
                      </p>
                      <p className="text-sm text-gray-500 font-inter">
                        {new Date(tx.timestamp).toLocaleDateString(undefined, {
                          month: 'short',
                          day: 'numeric',
                          hour: '2-digit',
                          minute: '2-digit'
                        })}
                      </p>
                    </div>
                  </div>
                  <div className={`text-sm font-semibold font-outfit ${tx.direction === 'CREDIT' ? 'text-[#34C759]' : 'text-[#FF3B30]'}`}>
                    {tx.direction === 'CREDIT' ? '+' : '-'}{formatCurrency(tx.amount, tx.currency)}
                  </div>
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}
