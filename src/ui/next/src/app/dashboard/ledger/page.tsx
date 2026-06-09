"use client";

import React, { useEffect, useState } from "react";

interface LedgerEntry {
  id: string;
  transaction_id: string;
  account_id: string;
  amount: number;
  currency: string;
  direction: string;
  type: string;
  created_at: string;
}

export default function LedgerPage() {
  const [entries, setEntries] = useState<LedgerEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchLedger() {
      try {
        const response = await fetch('/api/ledger/entries');
        if (!response.ok) {
          throw new Error('Failed to fetch ledger entries');
        }
        const data = await response.json();
        setEntries(data.entries || []);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    }
    fetchLedger();
  }, []);

  return (
    <div className="flex flex-col flex-1 w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 h-full bg-slate-50/50">
      <div className="mb-8">
          <h1 className="text-2xl font-bold text-gray-900">Ledger Statement</h1>
          <p className="text-sm text-gray-500 mt-1">Recent financial activity</p>
      </div>

      <div className="bg-white rounded-xl shadow-sm border border-slate-200 mt-6 p-6">
        {loading && <p className="text-slate-500">Loading ledger entries...</p>}
        {error && <div className="text-red-500">{error}</div>}
        {!loading && !error && entries.length === 0 && (
          <p className="text-slate-500">No recent activity.</p>
        )}
        {!loading && !error && entries.length > 0 && (
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-slate-200">
              <thead>
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Date</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Type</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Direction</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-slate-500 uppercase tracking-wider">Amount</th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-slate-200">
                {entries.map((entry) => (
                  <tr key={entry.id}>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-900">
                      {new Date(entry.created_at).toLocaleDateString()}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-500 capitalize">
                      {entry.type.replace('_', ' ')}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-500">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                        entry.direction === 'credit' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                      }`}>
                        {entry.direction}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-900">
                      {new Intl.NumberFormat('en-US', { style: 'currency', currency: entry.currency }).format(entry.amount)}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
