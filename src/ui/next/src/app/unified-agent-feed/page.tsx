"use client";

import React, { useEffect, useState } from 'react';
import { AppShell } from '../components/AppShell';

interface Action {
  label: string;
  style: string;
}

interface Proposal {
  id: string;
  agent_type: string;
  type: string;
  status: string;
  title: string;
  description: string;
  actions: Action[];
  icon: string;
  color: string;
}

export default function UnifiedAgentFeedPage() {
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetch('/api/agents/proposals')
      .then((res) => res.json())
      .then((data) => {
        setProposals(data.proposals || []);
        setLoading(false);
      })
      .catch((err) => {
        console.error('Failed to load proposals', err);
        setLoading(false);
      });
  }, []);

  return (
    <AppShell>
      <main className="app-page">
        <header className="mb-6">
          <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Unified Agent Feed</h1>
          <p className="text-gray-600 dark:text-gray-400">Review and approve agent proposals for your business.</p>
        </header>

        <section className="grid grid-cols-1 gap-6 max-w-2xl">
          {loading ? (
             <div className="text-gray-500" data-testid="loading-state">Loading proposals from intelligence layer...</div>
          ) : proposals.length === 0 ? (
             <div className="text-gray-500" data-testid="empty-state">No pending proposals at this time.</div>
          ) : (
            proposals.map(proposal => (
              <div key={proposal.id} className="mac-glass-container p-6 rounded-[16px] hover:shadow-lg transition-all group border border-white/40 dark:border-white/10 relative">
                <div className="flex items-start justify-between mb-4">
                  <div className="flex items-center gap-3">
                    <div className={`w-10 h-10 rounded-full bg-${proposal.color}-50 dark:bg-${proposal.color}-900/30 flex items-center justify-center text-xl`}>{proposal.icon}</div>
                    <div>
                      <h3 className="font-semibold text-gray-900 dark:text-white">{proposal.agent_type}</h3>
                      <p className="text-xs text-gray-500">{proposal.type}</p>
                    </div>
                  </div>
                  <span className={`text-xs font-medium bg-${proposal.color}-100 text-${proposal.color}-800 dark:bg-${proposal.color}-900/50 dark:text-${proposal.color}-300 px-2 py-1 rounded-full`}>{proposal.status}</span>
                </div>

                <h4 className="text-lg font-medium text-gray-900 dark:text-white mb-2">{proposal.title}</h4>
                <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">{proposal.description}</p>

                <div className="flex gap-3">
                  {proposal.actions.map((action, i) => (
                    <button
                       key={i}
                       className={`flex-1 py-2 rounded-lg font-medium transition-colors ${action.style === 'primary' ? `bg-${proposal.color}-600 hover:bg-${proposal.color}-700 text-white` : 'bg-white hover:bg-gray-50 text-gray-700 border border-gray-200'}`}
                    >
                      {action.label}
                    </button>
                  ))}
                </div>
              </div>
            ))
          )}
        </section>
      </main>
    </AppShell>
  );
}
