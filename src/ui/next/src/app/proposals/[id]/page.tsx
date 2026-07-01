'use client';

import React, { useEffect, useState } from 'react';
import { useParams, useRouter } from 'next/navigation';
import { AppShell } from '../../components/AppShell';

interface LineItem {
  id: string;
  description: string;
  unit_price_cents: number;
  quantity: number;
  is_optional: boolean;
}

interface Proposal {
  id: string;
  customer_id: string;
  status: string;
  total_amount_cents: number;
  required_deposit_cents: number;
  checkout_url?: string;
}

interface ProposalResponse {
  proposal: Proposal;
  line_items: LineItem[];
}

export default function ProposalReviewPage() {
  const params = useParams();
  const router = useRouter();
  const id = params.id as string;
  const [data, setData] = useState<ProposalResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [approving, setApproving] = useState(false);

  useEffect(() => {
    async function fetchProposal() {
      try {
        const res = await fetch(`/api/proposals/${id}`);
        if (!res.ok) throw new Error('Failed to fetch proposal');
        const json = await res.json();
        setData(json);
      } catch (err: any) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    }
    fetchProposal();
  }, [id]);

  const handleApprove = async () => {
    try {
      setApproving(true);
      const res = await fetch(`/api/proposals/${id}/approve`, { method: 'POST' });
      if (!res.ok) throw new Error('Failed to approve proposal');
      const updated = await res.json();
      setData(updated);
      if (updated.proposal && updated.proposal.checkout_url) {
        alert('Proposal Approved! Stripe Payment Link generated and invoice created.');
      }
    } catch (err: any) {
      alert(err.message);
    } finally {
      setApproving(false);
    }
  };

  if (loading) return <AppShell title="Loading Proposal..."><div className="p-4 text-center">Loading...</div></AppShell>;
  if (error) return <AppShell title="Error"><div className="p-4 text-center text-[#FF3B30]">{error}</div></AppShell>;
  if (!data || !data.proposal) return <AppShell title="Not Found"><div className="p-4 text-center">Proposal not found</div></AppShell>;

  const { proposal, line_items } = data;

  return (
    <AppShell title="Review Proposal" subtitle={`Proposal #${id.slice(0, 8)}`}>
      <div className="w-full max-w-md mx-auto p-4 space-y-6">
        <div className="glassmorphism p-6 space-y-4">
          <div className="flex justify-between items-center">
            <span className="text-sm font-medium text-gray-500">Status</span>
            <span className={`text-xs font-bold px-2 py-1 rounded-full ${proposal.status === 'ACCEPTED' ? 'bg-green-100 text-green-700' : 'bg-blue-100 text-blue-700'}`}>
              {proposal.status}
            </span>
          </div>

          <div className="space-y-3">
            <h3 className="text-[11px] font-bold uppercase tracking-wider text-gray-400">Line Items</h3>
            {line_items?.map((item) => (
              <div key={item.id} className="flex flex-col gap-1 py-2 border-b border-gray-50 dark:border-gray-800 last:border-0">
                <div className="flex justify-between text-sm">
                  <span>{item.description} (x{item.quantity})</span>
                  <span className="font-medium">${((item.unit_price_cents * item.quantity) / 100).toFixed(2)}</span>
                </div>
              </div>
            ))}
          </div>

          <div className="pt-4 border-t border-gray-100 dark:border-gray-800 space-y-2">
            <div className="flex justify-between items-center font-bold">
              <span>Total Amount</span>
              <span>${(proposal.total_amount_cents / 100).toFixed(2)}</span>
            </div>
            <div className="flex justify-between items-center text-sm text-gray-500">
              <span>Required Deposit</span>
              <span>${(proposal.required_deposit_cents / 100).toFixed(2)}</span>
            </div>
          </div>

          {proposal.checkout_url && (
            <div className="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg">
              <p className="text-[11px] font-bold text-[#0071E3] dark:text-blue-400 mb-1 uppercase">Stripe Payment Link</p>
              <a href={proposal.checkout_url} target="_blank" rel="noreferrer" className="text-sm text-[#0066FF] underline break-all">
                {proposal.checkout_url}
              </a>
            </div>
          )}
        </div>

        {proposal.status === 'DRAFT' && (
          <button
            onClick={handleApprove}
            disabled={approving}
            className="w-full min-h-[44px] bg-[#0066FF] text-white font-bold shadow-lg hover:bg-[#0052CC] transition-all disabled:opacity-50 rounded-lg"
          >
            {approving ? 'Approving...' : 'Approve & Send'}
          </button>
        )}

        <button
          onClick={() => router.back()}
          className="w-full min-h-[44px] border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white font-medium hover:bg-gray-50 dark:hover:bg-gray-800 transition-all rounded-lg"
        >
          Back
        </button>
      </div>
    </AppShell>
  );
}
