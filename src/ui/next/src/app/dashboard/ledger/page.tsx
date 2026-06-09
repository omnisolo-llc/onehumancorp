"use client";

import React, { useEffect, useState } from "react";

interface LedgerEntry {
  id: string;
  type: string;
  amount: number;
  currency: string;
  status: string;
  created_at: string;
}

export default function LedgerPage() {
  const [entries, setEntries] = useState<LedgerEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchLedger = async () => {
    try {
      setLoading(true);
      setError(null);
      const res = await fetch("/api/ui/dashboard/metrics"); // Note: Temporary endpoint mapping for ledger mock
      if (!res.ok) throw new Error("Failed to load ledger data");
      const data = await res.json();
      setEntries(data.recentTransactions || []);
    } catch (e: any) {
      setError(e.message || "An error occurred");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchLedger();
  }, []);

  if (error) {
    return (
        <div className="flex flex-col items-center justify-center p-12 bg-white rounded-2xl shadow-sm border border-gray-100">
          <p className="text-red-500 mb-4">{error}</p>
          <button onClick={fetchLedger} className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors">Retry</button>
        </div>
    );
  }

  return (
    <div className="flex flex-col flex-1 w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 gap-8">
        <div className="flex flex-col md:flex-row justify-between items-start md:items-center bg-white p-6 rounded-2xl shadow-sm border border-gray-100 mb-6">
          <div>
            <h1 className="text-2xl font-bold text-gray-900 font-outfit">Transaction Ledger</h1>
            <p className="text-gray-500 mt-1">View all processed transactions from online and offline sources.</p>
          </div>
        </div>

      <div className="bg-white/80 backdrop-blur-md rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
        {loading ? (
          <div className="p-12 text-center text-gray-500">
            <svg className="animate-spin h-8 w-8 mx-auto text-blue-600 mb-4" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
              <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Loading ledger entries...
          </div>
        ) : entries.length === 0 ? (
          <div className="p-12 text-center text-gray-500">
            No transactions found.
          </div>
        ) : (
          <div className="overflow-x-auto">
            <table className="w-full text-sm text-left">
              <thead className="text-xs text-gray-500 bg-gray-50 uppercase border-b border-gray-200">
                <tr>
                  <th scope="col" className="px-6 py-4 font-medium">Date</th>
                  <th scope="col" className="px-6 py-4 font-medium">Type</th>
                  <th scope="col" className="px-6 py-4 font-medium">Amount</th>
                  <th scope="col" className="px-6 py-4 font-medium">Status</th>
                  <th scope="col" className="px-6 py-4 font-medium text-right">ID</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-100">
                {entries.map((entry) => (
                  <tr key={entry.id} className="hover:bg-gray-50/50 transition-colors">
                    <td className="px-6 py-4 whitespace-nowrap text-gray-900">
                      {new Date(entry.created_at).toLocaleDateString()} {new Date(entry.created_at).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'})}
                    </td>
                    <td className="px-6 py-4">
                      <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-50 text-blue-800">
                        {entry.type.replace('_', ' ')}
                      </span>
                    </td>
                    <td className="px-6 py-4 font-medium text-gray-900">
                      {(entry.amount / 100).toLocaleString('en-US', { style: 'currency', currency: entry.currency || 'USD' })}
                    </td>
                    <td className="px-6 py-4">
                      <span className={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium ${
                        entry.status === 'succeeded' || entry.status === 'completed' ? 'bg-green-50 text-green-700 border border-green-200' :
                        entry.status === 'pending' ? 'bg-yellow-50 text-yellow-700 border border-yellow-200' :
                        'bg-gray-50 text-gray-700 border border-gray-200'
                      }`}>
                        <span className={`w-1.5 h-1.5 rounded-full ${
                           entry.status === 'succeeded' || entry.status === 'completed' ? 'bg-green-500' :
                           entry.status === 'pending' ? 'bg-yellow-500' :
                           'bg-gray-500'
                        }`}></span>
                        {entry.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-right text-gray-400 font-mono text-xs">
                      {entry.id.split('_').pop()?.slice(0,8)}...
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
