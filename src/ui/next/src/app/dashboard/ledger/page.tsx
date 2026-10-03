"use client";


import React, { useEffect, useState } from "react";
import { PageHeader } from "@/components/layout/PageHeader";
import { ErrorState } from "@/components/layout/ErrorState";

interface LedgerEntry {
  id: string;
  transaction_id: string;
  account_id: string;
  amount: number;
  currency: string;
  direction: string;
  entry_type: string;
  created_at: string;
}

function readEntries(value: unknown): LedgerEntry[] {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid ledger response');
  const response = value as Record<string, unknown>;
  if (response.error != null || response.success === false || !Array.isArray(response.entries)) throw new Error('Invalid ledger response');
  return response.entries.map((value: unknown) => {
    if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error('Invalid ledger entry');
    const entry = value as Record<string, unknown>;
    for (const name of ['id', 'transaction_id', 'account_id', 'entry_type', 'created_at']) {
      if (typeof entry[name] !== 'string' || entry[name].length === 0) throw new Error('Invalid ledger entry');
    }
    if (typeof entry.amount !== 'number' || !Number.isFinite(entry.amount)
        || Math.abs(entry.amount) > Number.MAX_SAFE_INTEGER
        || typeof entry.currency !== 'string' || !/^[A-Za-z0-9]{3,12}$/.test(entry.currency)
        || !['credit', 'debit'].includes(String(entry.direction))
        || !Number.isFinite(Date.parse(entry.created_at as string))) throw new Error('Invalid ledger entry');
    return entry as unknown as LedgerEntry;
  });
}

export default function LedgerPage() {
  const [entries, setEntries] = useState<LedgerEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const controller = new AbortController();
    let active = true;
    async function fetchLedger() {
      try {
        const response = await fetch('/api/v1/ledger/entries', { signal: controller.signal, cache: 'no-store' });
        if (!response.ok) {
          throw new Error('Failed to fetch ledger entries');
        }
        const data = readEntries(await response.json());
        if (active) setEntries(data);
      } catch {
        if (active) setError('Recorded ledger entries could not be verified. Reload to try again.');
      } finally {
        if (active) setLoading(false);
      }
    }
    void fetchLedger();
    return () => { active = false; controller.abort(); };
  }, []);

  return (
    <div className="flex flex-col flex-1 w-full max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 h-full bg-slate-50/50">
      <PageHeader
        title="Ledger Statement"
        description="Recorded entries by currency. These entries do not establish an available balance or payment settlement."
      />

      <div className="bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm border border-white/40-sm border border-slate-200 mt-6 p-6">
        {loading && <p className="text-slate-500">Loading ledger entries...</p>}
        {error && <ErrorState title="Ledger unavailable" message={error} />}
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
                      {entry.entry_type.replaceAll('_', ' ')}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm text-slate-500">
                      <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                        entry.direction === 'credit' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'
                      }`}>
                        {entry.direction}
                      </span>
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-slate-900">
                      {entry.currency.toUpperCase()} {new Intl.NumberFormat('en-US', { maximumFractionDigits: 20 }).format(entry.amount)}
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
