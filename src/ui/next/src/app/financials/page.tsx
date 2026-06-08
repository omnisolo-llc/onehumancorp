'use client';

import React, { useState, useEffect } from 'react';

export default function FinancialsPage() {
  const [loading, setLoading] = useState(true);

  const [data, setData] = useState({
    totalRevenue: 0,
    estimatedTaxesSaved: 0,
    availableCash: 0,
    recentTransactions: []
  });

  useEffect(() => {
    fetch('/api/financials')
      .then(res => res.json())
      .then(resData => {
        setData(resData);
        setLoading(false);
      })
      .catch(err => {
        console.error(err);
        setLoading(false);
      });
  }, []);

  return (
    <main className="app-main-layout">
      <div className="max-w-3xl mx-auto px-4 py-8">

        {/* Header */}
        <div className="mb-8">
          <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Financials</h1>
          <p className="text-[#1D1D1F]/70 dark:text-[#F5F5F7]/70">Your unified ledger, tax savings, and recent activity.</p>
        </div>

        {/* Advisory Card */}
        <div className="mb-8 glassmorphism p-6 rounded-[16px] border border-blue-500/30 bg-blue-50/50 dark:bg-blue-900/20">
          <div className="flex items-start gap-4">
            <div className="w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-800 flex items-center justify-center text-xl shrink-0">
              🤖
            </div>
            <div>
              <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">From The Accountant</h3>
              <p className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 mb-3">
                You have collected $0 in sales this week. Based on your location, I've automatically set aside 10% for estimated taxes.
              </p>
              <button className="bg-[#0066FF] hover:bg-[#005ce6] text-white px-4 py-2 rounded-lg font-medium text-sm transition-colors w-full sm:w-auto h-[44px]">
                View Tax Summary
              </button>
            </div>
          </div>
        </div>

        {/* Balances Grid */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
          {/* Total Revenue */}
          <div className="glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10">
            <p className="text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 mb-2">Total Revenue (YTD)</p>
            {loading ? (
              <div className="h-8 w-24 bg-black/5 dark:bg-white/5 animate-pulse rounded"></div>
            ) : (
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">
                ${data.totalRevenue.toLocaleString(undefined, { minimumFractionDigits: 2 })}
              </h2>
            )}
          </div>

          {/* Taxes Auto-Saved */}
          <div className="glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10">
            <p className="text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 mb-2">Estimated Taxes Saved</p>
            {loading ? (
              <div className="h-8 w-24 bg-black/5 dark:bg-white/5 animate-pulse rounded"></div>
            ) : (
              <h2 className="text-2xl font-bold font-outfit text-orange-600 dark:text-orange-400">
                ${data.estimatedTaxesSaved.toLocaleString(undefined, { minimumFractionDigits: 2 })}
              </h2>
            )}
          </div>

          {/* Available Cash */}
          <div className="glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10">
            <p className="text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 mb-2">Available Cash</p>
            {loading ? (
              <div className="h-8 w-24 bg-black/5 dark:bg-white/5 animate-pulse rounded"></div>
            ) : (
              <h2 className="text-2xl font-bold font-outfit text-green-600 dark:text-green-400">
                ${data.availableCash.toLocaleString(undefined, { minimumFractionDigits: 2 })}
              </h2>
            )}
          </div>
        </div>

        {/* Ledger Activity */}
        <div className="glassmorphism rounded-[16px] border border-white/40 dark:border-white/10 overflow-hidden">
          <div className="px-6 py-4 border-b border-black/5 dark:border-white/5">
            <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Recent Ledger Activity</h3>
          </div>

          <div className="divide-y divide-black/5 dark:divide-white/5">
            {loading ? (
              [1, 2, 3].map((i) => (
                <div key={i} className="p-6 flex items-center justify-between">
                  <div className="space-y-2">
                    <div className="h-4 w-32 bg-black/5 dark:bg-white/5 animate-pulse rounded"></div>
                    <div className="h-3 w-20 bg-black/5 dark:bg-white/5 animate-pulse rounded"></div>
                  </div>
                  <div className="h-4 w-16 bg-black/5 dark:bg-white/5 animate-pulse rounded"></div>
                </div>
              ))
            ) : data.recentTransactions && data.recentTransactions.length > 0 ? (
              data.recentTransactions.map((tx: any) => (
                <div key={tx.id} className="p-6 flex items-center justify-between hover:bg-black/[0.02] dark:hover:bg-white/[0.02] transition-colors">
                  <div>
                    <p className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">{tx.description}</p>
                    <p className="text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60">{tx.date}</p>
                  </div>
                  <div className={`font-medium ${tx.type === 'credit' ? 'text-green-600 dark:text-green-400' : 'text-[#1D1D1F] dark:text-[#F5F5F7]'}`}>
                    {tx.type === 'credit' ? '+' : ''}${Math.abs(tx.amount).toFixed(2)}
                  </div>
                </div>
              ))
            ) : (
                <div className="p-6 text-center text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60">
                    No recent ledger activity.
                </div>
            )}
          </div>

          <div className="p-4 bg-black/[0.02] dark:bg-white/[0.02] text-center border-t border-black/5 dark:border-white/5">
            <button className="text-sm font-medium text-blue-600 dark:text-blue-400 hover:underline min-h-[44px] min-w-[44px] px-4">
              View Full Statement
            </button>
          </div>
        </div>

      </div>
    </main>
  );
}
