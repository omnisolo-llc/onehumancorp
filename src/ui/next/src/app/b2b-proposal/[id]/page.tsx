"use client";

import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';

export default function B2BProposalReviewPage({ params }: { params: { id: string } }) {
  const router = useRouter();
  const [proposal, setProposal] = useState<any>(null);
  const [amount, setAmount] = useState<number>(0);
  const [scope, setScope] = useState<string>('');

  useEffect(() => {
    fetch(`/api/v1/b2b/proposals/${params.id}`)
      .then((res) => res.json())
      .then((data) => {
        setProposal(data);
        setAmount(data.total_amount_cents / 100);
        setScope(data.project_scope || 'Project Scope');
      })
      .catch((err) => console.error(err));
  }, [params.id]);

  const handleApproveAndSend = async () => {
    // In a real flow, this sends the proposal via email/SMS to the client
    await fetch(`/api/v1/b2b/proposals/${params.id}/approve`, {
      method: 'POST',
    });
    router.push('/dashboard');
  };

  if (!proposal) return <div className="p-8 text-center animate-pulse">Loading proposal draft...</div>;

  return (
    <div className="flex flex-col min-h-screen bg-[#F5F5F7] font-inter">
      <header className="px-4 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-xl font-bold font-outfit text-[#1D1D1F]">Review Proposal</h1>
         <button onClick={() => router.back()} className="text-sm font-medium text-gray-500 hover:text-gray-800">Close</button>
      </header>

      <main className="p-4 md:p-6 flex-1 max-w-2xl mx-auto w-full">
        <div className="glassmorphism glass-card rounded-[16px] p-6 shadow-sm border border-white/50 mb-6 bg-white/60 backdrop-blur-xl">
          <div className="mb-6">
            <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-2">Prepared For</h2>
            <p className="text-2xl font-bold text-gray-900">{proposal.client_name}</p>
          </div>

          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Project Scope</label>
              <textarea
                value={scope}
                onChange={(e) => setScope(e.target.value)}
                rows={4}
                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all bg-white/80"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">Total Amount ($)</label>
              <input
                type="number"
                value={amount}
                onChange={(e) => setAmount(Number(e.target.value))}
                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all bg-white/80"
              />
            </div>

            <div className="pt-4 flex gap-3">
              <button
                onClick={handleApproveAndSend}
                data-testid="approve-send-proposal"
                className="flex-1 py-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold rounded-xl shadow-md transition-all flex justify-center items-center gap-2"
              >
                Approve & Send
              </button>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
