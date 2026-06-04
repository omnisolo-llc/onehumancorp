'use client';

import React, { useEffect, useState } from 'react';
import { ProposalCard, AgentProposal } from './ProposalCard';

export const AgentFeed: React.FC = () => {
  const [proposals, setProposals] = useState<AgentProposal[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const tenantId = () => {
    if (typeof window === 'undefined') return 'default';
    return localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'default';
  };

  useEffect(() => {
    let mounted = true;
    async function fetchProposals() {
      try {
        const tenant = tenantId();
        const res = await fetch(`/api/agents/approvals?tenant_id=${tenant}`, {
          headers: {
            'x-tenant-id': tenant,
            'x-user-id': 'default',
          },
        });

        if (!res.ok) {
          throw new Error('Failed to load agent proposals');
        }

        const data = await res.json();
        if (mounted && data.pending_approvals) {
          // Map backend response to AgentProposal interface
          const mappedProposals: AgentProposal[] = data.pending_approvals.map((p: any) => ({
            id: p.id,
            department: p.department,
            description: p.description,
            actionRisk: p.action_risk,
            status: p.status,
            payload: p.payload,
          }));
          setProposals(mappedProposals);
        }
      } catch (err: any) {
        if (mounted) {
          setError(err.message || 'Failed to load proposals');
        }
      } finally {
        if (mounted) {
          setLoading(false);
        }
      }
    }

    fetchProposals();
    return () => {
      mounted = false;
    };
  }, []);

  const handleDecision = async (id: string, approved: boolean) => {
    // Optimistic UI update
    setProposals((prev) => prev.filter((p) => p.id !== id));

    try {
      const tenant = tenantId();
      const res = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': tenant,
          'x-user-id': 'default',
        },
        body: JSON.stringify({ approved }),
      });

      if (!res.ok) {
        throw new Error('Failed to submit decision');
      }
    } catch (err: any) {
      setError(err.message || 'Action failed');
      // Refresh feed on failure
      const tenant = tenantId();
      const refreshRes = await fetch(`/api/agents/approvals?tenant_id=${tenant}`);
      if (refreshRes.ok) {
        const data = await refreshRes.json();
        setProposals(data.pending_approvals.map((p: any) => ({
          id: p.id,
          department: p.department,
          description: p.description,
          actionRisk: p.action_risk,
          status: p.status,
          payload: p.payload,
        })));
      }
    }
  };

  if (loading) {
    return (
      <div
        className="w-full mb-6 p-8 rounded-[16px] text-center text-gray-500 font-inter animate-pulse"
        style={{
          background: 'rgba(255, 255, 255, 0.65)',
          backdropFilter: 'blur(30px) saturate(210%)',
          border: '1px solid rgba(255, 255, 255, 0.4)',
        }}
      >
        Synchronizing with Agents...
      </div>
    );
  }

  if (error) {
    return (
      <div className="w-full mb-6 p-6 rounded-[16px] border border-red-500/20 bg-red-50 text-red-600 text-center font-inter">
        {error}
      </div>
    );
  }

  if (proposals.length === 0) {
    return (
      <div
        className="w-full mb-6 p-8 rounded-[16px] text-center"
        style={{
          background: 'rgba(255, 255, 255, 0.65)',
          backdropFilter: 'blur(30px) saturate(210%)',
          border: '1px solid rgba(255, 255, 255, 0.4)',
        }}
      >
        <div className="text-4xl mb-3">✨</div>
        <h3 className="text-xl font-bold font-outfit text-[#1D1D1F]">All Clear!</h3>
        <p className="text-sm text-gray-600 font-inter mt-1">
          Your AI agents are monitoring your business in the background.
        </p>
      </div>
    );
  }

  return (
    <section className="mb-8 w-full" aria-label="Unified Agent Feed">
      <div className="mb-4 flex items-center justify-between">
        <h2 className="text-xl font-bold font-outfit text-[#1D1D1F]">Daily Feed</h2>
        <span
          className="px-3 py-1 rounded-full text-[10px] font-bold uppercase tracking-wider"
          style={{
            backgroundColor: 'rgba(255, 149, 0, 0.1)',
            color: '#FF9500',
          }}
        >
          {proposals.length} Items
        </span>
      </div>

      <div className="flex flex-col gap-4">
        {proposals.map((proposal) => (
          <ProposalCard
            key={proposal.id}
            proposal={proposal}
            onApprove={(id) => handleDecision(id, true)}
            onDecline={(id) => handleDecision(id, false)}
          />
        ))}
      </div>
    </section>
  );
};
