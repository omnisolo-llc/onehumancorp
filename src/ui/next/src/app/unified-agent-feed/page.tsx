'use client';

import React, { useState } from 'react';

// Mock agent proposals
const mockProposals = [
  {
    id: '1',
    agent: 'Marketing Agent',
    title: 'New Social Media Campaign',
    description: 'Proposing a new Instagram campaign for the upcoming summer sale.',
    status: 'pending',
  },
  {
    id: '2',
    agent: 'Sales Agent',
    title: 'Follow-up with Leads',
    description: 'Suggesting a follow-up email sequence for recent leads who haven\'t purchased.',
    status: 'pending',
  },
  {
    id: '3',
    agent: 'Support Agent',
    title: 'Update FAQ',
    description: 'Recommending updates to the FAQ based on recent customer inquiries.',
    status: 'pending',
  },
];

export default function UnifiedAgentFeed() {
  const [proposals, setProposals] = useState(mockProposals);

  const handleApprove = (id: string) => {
    setProposals((prev) =>
      prev.map((prop) => (prop.id === id ? { ...prop, status: 'approved' } : prop))
    );
  };

  const handleReject = (id: string) => {
    setProposals((prev) =>
      prev.map((prop) => (prop.id === id ? { ...prop, status: 'rejected' } : prop))
    );
  };

  return (
    <div className="min-h-screen bg-gray-50/50 backdrop-blur-[20px] p-4 md:p-8 flex flex-col items-center">
      <div className="w-full max-w-[375px] md:max-w-md lg:max-w-lg space-y-6">
        <header className="text-center mb-8">
          <h1 className="text-2xl font-bold text-gray-900 font-outfit">Unified Agent Feed</h1>
          <p className="text-gray-500 font-inter text-sm">Review and approve agent proposals.</p>
        </header>

        <div className="space-y-4">
          {proposals.map((proposal) => (
            <div
              key={proposal.id}
              className="bg-white/70 backdrop-blur-md rounded-2xl shadow-sm border border-white/20 p-5 transition-all hover:shadow-md"
            >
              <div className="flex justify-between items-start mb-2">
                <span className="text-xs font-semibold uppercase tracking-wider text-blue-600 bg-blue-50 px-2 py-1 rounded-full">
                  {proposal.agent}
                </span>
                {proposal.status !== 'pending' && (
                  <span
                    className={`text-xs font-medium px-2 py-1 rounded-full ${
                      proposal.status === 'approved'
                        ? 'bg-green-100 text-green-700'
                        : 'bg-red-100 text-red-700'
                    }`}
                  >
                    {proposal.status.charAt(0).toUpperCase() + proposal.status.slice(1)}
                  </span>
                )}
              </div>
              <h2 className="text-lg font-semibold text-gray-800 font-outfit mb-1">
                {proposal.title}
              </h2>
              <p className="text-gray-600 font-inter text-sm mb-4 leading-relaxed">
                {proposal.description}
              </p>

              {proposal.status === 'pending' && (
                <div className="flex space-x-3 mt-4">
                  <button
                    onClick={() => handleApprove(proposal.id)}
                    className="flex-1 bg-black text-white font-medium py-2 px-4 rounded-xl shadow-sm hover:bg-gray-800 transition-colors font-inter text-sm focus:outline-none focus:ring-2 focus:ring-black/20"
                  >
                    Approve
                  </button>
                  <button
                    onClick={() => handleReject(proposal.id)}
                    className="flex-1 bg-white text-gray-700 font-medium py-2 px-4 rounded-xl border border-gray-200 shadow-sm hover:bg-gray-50 transition-colors font-inter text-sm focus:outline-none focus:ring-2 focus:ring-gray-200"
                  >
                    Reject
                  </button>
                </div>
              )}
            </div>
          ))}
          {proposals.filter(p => p.status === 'pending').length === 0 && (
            <div className="text-center p-8 bg-white/50 backdrop-blur-sm rounded-2xl border border-white/20">
              <p className="text-gray-500 font-inter">No pending proposals. You're all caught up!</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
