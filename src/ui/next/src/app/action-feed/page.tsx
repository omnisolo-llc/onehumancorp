'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

interface ApprovalRequest {
  id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
}

export default function ActionFeed() {
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchApprovals() {
      try {
        const res = await fetch('/api/agents/approvals');
        if (res.ok) {
          const data = await res.json();
          setApprovals(data.pending_approvals || []);
        }
      } catch (error) {
        console.error('Failed to fetch approvals:', error);
      } finally {
        setLoading(false);
      }
    }
    fetchApprovals();
  }, []);

  const handleDecision = async (id: string, approved: boolean) => {
    // Optimistic update
    setApprovals(prev => prev.filter(a => a.id !== id));

    try {
      await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved })
      });
    } catch (error) {
      console.error('Failed to submit decision', error);
      // In a real app we might revert the optimistic update here if the request fails
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: "rgba(255, 255, 255, 0.65)", backdropFilter: "blur(30px) saturate(210%)", borderBottom: "1px solid rgba(255, 255, 255, 0.4)", position: "sticky", top: 0, zIndex: 50 }}>
        <div className="flex items-center gap-4">
          <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
          <h1 className="text-2xl font-bold font-outfit" style={{ color: "#1D1D1F", letterSpacing: "-0.02em" }}>Action Feed</h1>
        </div>
        <div className="px-3 py-1 rounded-full text-xs font-medium" style={{ background: "rgba(0, 102, 255, 0.1)", color: "#0066FF", border: "1px solid rgba(0, 102, 255, 0.2)" }}>
          {approvals.length} Active Alerts
        </div>
      </header>

      {/* Main Content */}
      <main className="flex-1 max-w-lg w-full mx-auto p-4 flex flex-col gap-4 mt-4">
        {loading ? (
          <div className="animate-pulse flex flex-col gap-4">
             {[1, 2, 3].map(i => (
                <div key={i} className="h-32 bg-white/50 rounded-2xl"></div>
             ))}
          </div>
        ) : approvals.length === 0 ? (
          <div className="text-center py-20 text-gray-500">
             <div className="text-4xl mb-4">✨</div>
             <p className="font-medium">You're all caught up!</p>
             <p className="text-sm">Your AI teammates have no pending drafts for you to review.</p>
          </div>
        ) : (
          approvals.map((approval) => {
            const isHighRisk = approval.action_risk === 'DraftForReview' || approval.action_risk === 'HIGH';

            return (
              <div key={approval.id} className="mac-glass-container rounded-2xl p-5 shadow-sm hover:shadow-md transition-shadow flex flex-col gap-4 animate-fade-in relative overflow-hidden">
                {/* Visual indicator of risk */}
                {isHighRisk && (
                   <div className="absolute top-0 right-0 left-0 h-1 bg-gradient-to-r from-orange-400 to-red-500" />
                )}

                <div className="flex items-start gap-3">
                  <div className="w-10 h-10 rounded-full flex items-center justify-center text-xl shrink-0" style={{ background: '#eef2ff', color: '#4f46e5' }}>
                    {approval.department.toLowerCase().includes('success') ? '🤝' :
                     approval.department.toLowerCase().includes('operations') ? '⚙️' :
                     approval.department.toLowerCase().includes('marketing') ? '📢' :
                     approval.department.toLowerCase().includes('sales') ? '💰' : '🤖'}
                  </div>
                  <div>
                    <h3 className="font-semibold text-lg font-outfit text-gray-900 capitalize">
                      {approval.department}
                    </h3>
                    <p className="text-gray-600 font-inter text-sm leading-relaxed mt-1">
                      {approval.description}
                    </p>
                  </div>
                </div>

                <div className="flex gap-3 pt-2">
                  <button
                    onClick={() => handleDecision(approval.id, true)}
                    className="flex-1 py-2.5 font-bold transition-all shadow-sm hover:shadow active:scale-[0.98] rounded-xl flex items-center justify-center gap-2"
                    style={{ color: 'white', background: 'linear-gradient(135deg, #0066FF 0%, #0052cc 100%)' }}
                  >
                    Approve & Send
                  </button>
                  <button
                    onClick={() => handleDecision(approval.id, false)}
                    className="flex-1 py-2.5 font-bold transition-all shadow-sm hover:bg-red-50 active:scale-[0.98] rounded-xl"
                    style={{ color: '#FF3B30', border: '1px solid rgba(255, 59, 48, 0.3)', background: 'white' }}
                  >
                    Reject
                  </button>
                </div>
              </div>
            );
          })
        )}
      </main>
    </div>
  );
}
