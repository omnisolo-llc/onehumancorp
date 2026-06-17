"use client";

import React, { useEffect, useState } from 'react';
import { useParams } from 'next/navigation';

export default function ProposalViewPage() {
  const params = useParams();
  const id = params?.id as string;
  const [proposal, setProposal] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!id) return;
    fetch(`/api/v1/proposals/${id}`)
      .then(res => res.json())
      .then(data => {
        setProposal(data);
        setLoading(false);
      })
      .catch(err => {
        console.error(err);
        setLoading(false);
      });
  }, [id]);

  const handleAccept = async () => {
    try {
      const res = await fetch(`/api/v1/proposals/${id}/accept`, { method: 'POST' });
      if (res.ok) {
        alert('Proposal accepted!');
        // optionally navigate or update state
        setProposal({ ...proposal, proposal: { ...proposal.proposal, status: 'accepted' } });
      }
    } catch (e) {
      console.error(e);
    }
  };

  if (loading) return <div className="p-4">Loading proposal...</div>;
  if (!proposal) return <div className="p-4">Proposal not found.</div>;

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 max-w-[375px] mx-auto overflow-hidden relative shadow-lg">
      <div className="flex-1 p-6 backdrop-blur-xl bg-white/60">
        <h1 className="text-2xl font-semibold mb-4 text-gray-800">Project Proposal</h1>

        <div className="bg-white/80 p-4 rounded-xl shadow-sm border border-white/20 mb-6">
          <p className="text-sm text-gray-500 uppercase tracking-wide mb-1">Status</p>
          <p className="text-lg font-medium text-gray-900 capitalize">{proposal.proposal.status}</p>
        </div>

        <h2 className="text-lg font-medium text-gray-700 mb-3">Line Items</h2>
        <div className="space-y-3 mb-8">
          {proposal.line_items.map((item: any, idx: number) => (
            <div key={idx} className="bg-white/80 p-4 rounded-xl shadow-sm border border-white/20 flex justify-between items-center">
              <div>
                <p className="font-medium text-gray-800">{item.description}</p>
                <p className="text-sm text-gray-500">Qty: {item.quantity}</p>
              </div>
              <p className="font-medium text-gray-900">${(item.unit_price_cents / 100).toFixed(2)}</p>
            </div>
          ))}
        </div>

        <div className="border-t border-gray-200 pt-4 mb-24">
          <div className="flex justify-between items-center">
            <p className="text-gray-600 font-medium">Total Amount</p>
            <p className="text-2xl font-bold text-gray-900">${(proposal.proposal.total_amount_cents / 100).toFixed(2)}</p>
          </div>
        </div>
      </div>

      <div className="fixed bottom-0 left-0 right-0 max-w-[375px] mx-auto p-4 bg-white/80 backdrop-blur-md border-t border-gray-200">
        <button
          onClick={handleAccept}
          disabled={proposal.proposal.status === 'accepted'}
          className={`w-full py-4 rounded-xl text-white font-medium text-lg transition-all ${
            proposal.proposal.status === 'accepted'
              ? 'bg-gray-400 cursor-not-allowed'
              : 'bg-black hover:bg-gray-800 active:scale-[0.98]'
          }`}
        >
          {proposal.proposal.status === 'accepted' ? 'Accepted & Paid' : 'Accept & Pay Deposit'}
        </button>
      </div>
    </div>
  );
}
