'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function AgentsPage() {
  const [activeTab, setActiveTab] = useState<'departments' | 'approvals'>('departments');
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [approvals, setApprovals] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [actionLoading, setActionLoading] = useState<string | null>(null);
  const [activeAgents, setActiveAgents] = useState<string[]>(['operations']);
  const [showPaywallModal, setShowPaywallModal] = useState(false);

  const fetchApprovals = async () => {
    setLoading(true);
    try {
      const res = await fetch('/api/agents/approvals');
      if (res.ok) {
        const data = await res.json();
        setApprovals(data.pending_approvals || []);
      }
    } catch (e) {
      console.error('Failed to fetch approvals:', e);
    }
    setLoading(false);
  };

  useEffect(() => {
    fetchApprovals();
  }, []);

  const handleDecision = async (id: string, approved: boolean) => {
    setActionLoading(id);
    try {
      const res = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ approved }),
      });
      if (res.ok) {
        // Remove the processed approval from the list
        setApprovals(prev => prev.filter(req => req.id !== id));
      } else {
        console.error('Failed to process approval');
      }
    } catch (e) {
      console.error('Error making decision:', e);
    }
    setActionLoading(null);
  };

  const departments = [
    { id: 'operations', name: 'The Manager', role: 'Operations', icon: '⚙️', description: 'Handles inventory, orders, and fulfillment.' },
    { id: 'customer_success', name: 'The Ambassador', role: 'Customer Success', icon: '🤝', description: 'Responds to customer inquiries and builds loyalty.' },
    { id: 'marketing', name: 'The Promoter', role: 'Marketing', icon: '📣', description: 'Creates social posts and promotional campaigns.' },
    { id: 'sales', name: 'The Closer', role: 'Sales', icon: '💼', description: 'Generates quotes and follows up on leads.' },
    { id: 'finance', name: 'The Accountant', role: 'Finance', icon: '💰', description: 'Tracks expenses and generates invoices.' },
    { id: 'legal', name: 'The Counsel', role: 'Legal', icon: '⚖️', description: 'Drafts contracts and handles compliance.' },
    { id: 'business_advisory', name: 'The Strategist', role: 'Advisory', icon: '📈', description: 'Provides insights and growth strategies.' },
  ];

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter">
      {/* 375px Mobile Container constraint as per design doc */}
      <div className="w-full max-w-[375px] bg-white min-h-screen shadow-xl relative overflow-x-hidden flex flex-col">
        {/* Header */}
        <header className="px-5 pt-8 pb-4 bg-white/80 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-20 border-b border-gray-100">
          <div className="flex justify-between items-center mb-4">
            <Link href="/dashboard" className="text-gray-400 hover:text-gray-600">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
            </Link>
            <div className="flex items-center gap-2">
              <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Advanced</span>
              <button
                onClick={() => setShowAdvanced(!showAdvanced)}
                className={`w-10 h-6 rounded-full transition-colors relative ${showAdvanced ? 'bg-indigo-600' : 'bg-gray-200'}`}
              >
                <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform ${showAdvanced ? 'translate-x-4' : 'translate-x-0'}`}></span>
              </button>
            </div>
          </div>
          <h1 className="text-3xl font-extrabold font-outfit text-gray-900">AI Departments</h1>
          <p className="text-sm text-gray-500 mt-1">Your autonomous business team.</p>
        </header>

        {/* Tabs */}
        <div className="flex border-b border-gray-100">
          <button
            onClick={() => setActiveTab('departments')}
            className={`flex-1 py-3 text-sm font-semibold transition-colors ${activeTab === 'departments' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-700'}`}
          >
            My Team
          </button>
          <button
            onClick={() => setActiveTab('approvals')}
            className={`flex-1 py-3 text-sm font-semibold transition-colors flex items-center justify-center gap-2 ${activeTab === 'approvals' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-700'}`}
          >
            Needs Approval
            {approvals.length > 0 && (
              <span className="bg-red-500 text-white text-[10px] font-bold px-2 py-0.5 rounded-full">
                {approvals.length}
              </span>
            )}
          </button>
        </div>

        {/* Main Content */}
        <main className="flex-1 p-5 overflow-y-auto pb-24 bg-gray-50">
          {activeTab === 'departments' ? (
            <div className="space-y-4">
              {departments.map((dept) => {
                const isActive = activeAgents.includes(dept.id);
                return (
                <div
                  key={dept.id}
                  onClick={() => {
                    if (!isActive) {
                      if (activeAgents.length >= 1) {
                        setShowPaywallModal(true);
                      } else {
                        setActiveAgents([...activeAgents, dept.id]);
                      }
                    }
                  }}
                  className={`backdrop-blur-[30px] saturate-[210%] shadow-sm p-4 rounded-[16px] flex items-start gap-4 cursor-pointer hover:shadow-md transition-all ${isActive ? 'bg-white/90 border border-indigo-200 ring-2 ring-indigo-500/20' : 'bg-white/50 border border-white/50 opacity-80 hover:opacity-100'}`}
                >
                  <div className={`w-12 h-12 rounded-xl flex items-center justify-center text-2xl shrink-0 ${isActive ? 'bg-indigo-100' : 'bg-gray-100 grayscale'}`}>
                    {dept.icon}
                  </div>
                  <div className="flex-1">
                    <div className="flex justify-between items-start">
                      <h3 className={`font-bold font-outfit text-lg ${isActive ? 'text-gray-900' : 'text-gray-700'}`}>{dept.name}</h3>
                      {isActive && (
                        <span className="flex items-center gap-1 text-[10px] font-bold text-green-600 bg-green-50 px-2 py-0.5 rounded-full uppercase tracking-wider">
                          <span className="w-1.5 h-1.5 rounded-full bg-green-500 animate-pulse"></span> Active
                        </span>
                      )}
                    </div>
                    <p className={`text-xs font-semibold uppercase tracking-wide mb-1 ${isActive ? 'text-indigo-600' : 'text-gray-500'}`}>{dept.role}</p>
                    <p className="text-sm text-gray-600 leading-relaxed">{dept.description}</p>

                    {showAdvanced && (
                      <div className="mt-3 pt-3 border-t border-gray-100 flex gap-4 text-xs text-gray-500">
                        <span>Auto-approve: $0</span>
                      </div>
                    )}
                  </div>
                </div>
              )})}
            </div>
          ) : (
            <div className="h-full flex flex-col">
              {loading ? (
                <div className="flex flex-col items-center justify-center flex-1 text-center py-12">
                  <div className="w-8 h-8 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin mb-4"></div>
                  <p className="text-gray-500 text-sm font-medium">Fetching approvals...</p>
                </div>
              ) : approvals.length === 0 ? (
                <div className="flex flex-col items-center justify-center flex-1 text-center py-12">
                  <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mb-4">
                    <span className="text-3xl">🎉</span>
                  </div>
                  <h3 className="font-bold text-gray-900 font-outfit text-xl mb-2">All Caught Up!</h3>
                  <p className="text-gray-500 text-sm">Your AI team has no pending tasks.</p>
                </div>
              ) : (
                <div className="space-y-4 pb-8">
                  {approvals.map((req) => (
                    <div key={req.id} className="bg-white rounded-[16px] shadow-sm border border-gray-200 p-5 font-inter">
                      <div className="flex justify-between items-start mb-3">
                        <span className="bg-orange-100 text-orange-800 text-xs font-bold px-2.5 py-1 rounded-md uppercase tracking-wide">
                          {req.department}
                        </span>
                        <span className="text-xs text-gray-400 font-medium">Draft For Review</span>
                      </div>
                      <p className="text-gray-800 text-sm font-medium mb-5 leading-relaxed">
                        {req.description}
                      </p>

                      <div className="flex gap-3 mt-auto">
                        <button
                          onClick={() => handleDecision(req.id, false)}
                          disabled={actionLoading === req.id}
                          className={`flex-1 py-3 rounded-xl font-semibold text-sm transition-colors ${actionLoading === req.id ? 'bg-gray-200 text-gray-400 cursor-not-allowed' : 'bg-gray-100 hover:bg-gray-200 text-gray-700'}`}
                        >
                          {actionLoading === req.id ? '...' : 'Reject'}
                        </button>
                        <button
                          onClick={() => handleDecision(req.id, true)}
                          disabled={actionLoading === req.id}
                          className={`flex-1 py-3 rounded-xl font-semibold text-sm transition-colors shadow-sm ${actionLoading === req.id ? 'bg-indigo-400 text-white cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 text-white'}`}
                        >
                          {actionLoading === req.id ? '...' : 'Approve'}
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </main>
      </div>

      {/* Paywall Modal */}
      {showPaywallModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-sm rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-indigo-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-indigo-600">
                🤖
              </div>
              <button
                onClick={() => setShowPaywallModal(false)}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Hire more AI Agents</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Your current plan includes <strong className="text-gray-900">1 Active Agent</strong>. Upgrade to the Starter plan to hire more AI assistants and scale your business faster.
            </p>

            <div className="space-y-3">
              <Link
                href="/pricing"
                className="block w-full py-3 bg-indigo-600 text-white font-semibold rounded-xl text-center shadow-md hover:shadow-lg hover:-translate-y-0.5 transition-all"
              >
                Upgrade to Starter
              </Link>
              <button
                onClick={() => setShowPaywallModal(false)}
                className="w-full py-2 rounded-xl text-sm font-semibold text-gray-500 hover:text-gray-700 transition-colors"
              >
                Maybe later
              </button>
            </div>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
