'use client';

import { errorMessage } from '@/lib/errors';
import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

interface Proposal {
  id: string;
  customer_id: string;
  status: string;
  total_amount_cents: number;
  required_deposit_cents: number;
  checkout_url?: string;
  project_scope?: string;
  created_at: string;
}

interface ProposalListResponse {
  proposals: Proposal[];
}

export default function ProposalsPage() {
  const router = useRouter();
  const [data, setData] = useState<ProposalListResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function fetchProposals() {
      try {
        const res = await fetch(`/api/v1/proposals/list`);
        if (!res.ok) throw new Error('Failed to fetch proposals');
        const json = await res.json();
        setData(json);
      } catch (err: unknown) {
        setError(errorMessage(err));
      } finally {
        setLoading(false);
      }
    }
    fetchProposals();
  }, []);

  if (loading) return <AppShell title="Proposals"><div className="p-4 text-center">Loading...</div></AppShell>;
  if (error) return <AppShell title="Error"><div className="p-4 text-center text-[#FF3B30]">{error}</div></AppShell>;
  if (!data || !data.proposals) return <AppShell title="Proposals"><div className="p-4 text-center">No proposals found</div></AppShell>;

  const { proposals } = data;

  return (
    <AppShell title="Proposals" subtitle="Review and approve draft proposals">
      <div className="w-full max-w-md mx-auto p-4 space-y-4">
        {proposals.length === 0 ? (
          <div className="text-center py-8 text-gray-500">No proposals available.</div>
        ) : (
          proposals.map(proposal => (
            <div
              key={proposal.id}
              onClick={() => router.push(`/proposals/${proposal.id}`)}
              className="glassmorphism p-4 rounded-xl cursor-pointer hover:bg-gray-50/50 dark:hover:bg-gray-800/50 transition-colors border border-gray-100 dark:border-gray-800"
            >
              <div className="flex justify-between items-start mb-2">
                <div className="text-sm font-medium text-gray-900 dark:text-white truncate">
                  {proposal.project_scope || 'Custom Project Scope'}
                </div>
                <span className={`text-[10px] font-bold px-2 py-0.5 rounded-full ${proposal.status === 'ACCEPTED' ? 'bg-green-100 text-green-700' : 'bg-blue-100 text-blue-700'}`}>
                  {proposal.status}
                </span>
              </div>
              <div className="flex justify-between items-center text-xs text-gray-500">
                <span>Amount: ${(proposal.total_amount_cents / 100).toFixed(2)}</span>
                <span>{new Date(proposal.created_at).toLocaleDateString()}</span>
              </div>
            </div>
          ))
        )}
      </div>
    </AppShell>
  );
}
