"use client";

import React, { useEffect, useState } from 'react';
import { PoweredByOHC } from '../../components/PoweredByOHC';

export default function ClientProposalViewPage({ params }: { params: { id: string } }) {
  const [proposal, setProposal] = useState<any>(null);

  useEffect(() => {
    fetch(`/api/v1/b2b/proposals/${params.id}`)
      .then((res) => res.json())
      .then((data) => {
        setProposal(data);
      })
      .catch((err) => console.error(err));
  }, [params.id]);

  if (!proposal) {
    return (
      <div className="min-h-screen flex items-center justify-center font-inter bg-[#F5F5F7]">
        <div className="animate-pulse text-gray-500 font-medium">Loading Proposal...</div>
      </div>
    );
  }

  const handleAcceptAndPay = () => {
    if (proposal.checkout_url) {
        window.location.href = proposal.checkout_url;
    } else {
        alert("Redirecting to payment gateway...");
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-10 px-4 sm:px-6 lg:px-8 font-inter">
      <main className="max-w-3xl mx-auto">
        <section className="bg-white/70 backdrop-blur-2xl rounded-[16px] shadow-sm border border-white/50 p-8 md:p-12 mb-8 relative overflow-hidden">
          <div className="flex justify-between items-start mb-12 relative z-10">
            <div>
              <h1 className="text-4xl font-extrabold text-[#1D1D1F] font-outfit mb-2">PROJECT PROPOSAL</h1>
              <p className="text-gray-500 font-medium">Prepared for: {proposal.client_name}</p>
            </div>
          </div>

          <div className="mb-10">
             <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-3">Project Scope & Details</h2>
             <div className="text-gray-800 leading-relaxed whitespace-pre-wrap">
               {proposal.project_scope}
             </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10">
              <div className="p-6 bg-[#0066FF]/5 rounded-xl border border-[#0066FF]/10">
                  <h2 className="text-sm font-bold text-[#0066FF]/70 uppercase tracking-wider mb-2">Estimated Timeline</h2>
                  <p className="text-xl font-bold text-[#0066FF]">{proposal.timeline}</p>
              </div>
          </div>

          <div className="flex justify-between items-center border-t border-gray-200 pt-8 mt-8">
            <span className="text-xl font-bold text-gray-900 font-outfit uppercase tracking-wider">Total Investment</span>
            <span className="text-4xl font-extrabold text-[#0066FF] font-outfit">${(proposal.total_amount_cents / 100).toFixed(2)}</span>
          </div>

          <div className="flex justify-between items-center mt-2 mb-8">
            <span className="text-sm font-bold text-gray-500 font-outfit tracking-wider">Required Deposit</span>
            <span className="text-xl font-bold text-gray-600 font-outfit">${(proposal.required_deposit_cents / 100).toFixed(2)}</span>
          </div>

          <button
            onClick={handleAcceptAndPay}
            data-testid="client-accept-pay"
            className="w-full py-4 bg-[#0066FF] hover:bg-[#0052CC] text-white font-bold rounded-xl shadow-md transition-all text-lg flex items-center justify-center gap-2 transform hover:-translate-y-1"
          >
            Accept & Pay Deposit
          </button>
        </section>

        <div className="text-center pb-8 animate-fade-in flex flex-col items-center">
          <PoweredByOHC tenantId={proposal.tenant_id} />
        </div>
      </main>
    </div>
  );
}
