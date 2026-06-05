import React, { useState, useEffect } from 'react';
import { ProposalCard, AgentProposal } from './ProposalCard';

export const AgentFeed: React.FC = () => {
  const [proposals, setProposals] = useState<AgentProposal[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchProposals();
  }, []);

  const fetchProposals = async () => {
    try {
      setLoading(true);
      const res = await fetch('/api/agents/approvals?tenant_id=default');
      if (!res.ok) throw new Error('Failed to fetch');
      const data = await res.json();
      setProposals(data);
      setError(null);
    } catch (err) {
      setError('Failed to load agent proposals');
    } finally {
      setLoading(false);
    }
  };

  const handleApprove = async (id: string) => {
    // Optimistic UI
    setProposals(prev =>
      prev.map(p => p.id === id ? { ...p, status: 'approved' } : p)
    );

    try {
      await fetch(`/api/agents/approvals/${id}/approve`, { method: 'POST' });
    } catch (err) {
      // Revert on failure
      fetchProposals();
    }
  };

  const handleDecline = async (id: string) => {
    // Optimistic UI
    setProposals(prev =>
      prev.map(p => p.id === id ? { ...p, status: 'declined' } : p)
    );

    try {
      await fetch(`/api/agents/approvals/${id}/decline`, { method: 'POST' });
    } catch (err) {
      // Revert on failure
      fetchProposals();
    }
  };

  if (loading && proposals.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-20 space-y-4">
        <div className="w-8 h-8 border-4 border-blue-500/20 border-t-blue-500 rounded-full animate-spin" />
        <p className="text-white/40 text-sm animate-pulse">Consulting agent council...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="p-8 rounded-2xl border border-red-500/20 bg-red-500/5 text-center">
        <p className="text-red-400 font-medium mb-4">{error}</p>
        <button
          onClick={fetchProposals}
          className="text-white/60 text-xs underline hover:text-white"
        >
          Try again
        </button>
      </div>
    );
  }

  if (proposals.length === 0) {
    return (
      <div className="text-center py-20">
        <div className="w-16 h-16 rounded-full bg-white/5 border border-white/10 flex items-center justify-center mx-auto mb-4">
          <span className="text-2xl">✨</span>
        </div>
        <h3 className="text-white font-medium">All caught up</h3>
        <p className="text-white/40 text-sm max-w-[240px] mx-auto mt-1">
          Your agents are working silently in the background. No actions needed right now.
        </p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {proposals.map(proposal => (
        <ProposalCard
          key={proposal.id}
          proposal={proposal}
          onApprove={handleApprove}
          onDecline={handleDecline}
        />
      ))}
    </div>
  );
};
