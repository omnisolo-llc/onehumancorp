'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function AgentsPage() {
  const [activeTab, setActiveTab] = useState<'departments' | 'approvals'>('departments');
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [approvals, setApprovals] = useState<any[]>([]);
  const [prompts, setPrompts] = useState<Record<string, string>>({});
  const [toggles, setToggles] = useState<Record<string, Record<string, boolean>>>({});
  const [loading, setLoading] = useState(false);
  const [actionLoading, setActionLoading] = useState<string | null>(null);

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
              <span className="text-xs font-semibold text-gray-500 uppercase tracking-wider">Pro Mode</span>
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
              {departments.map((dept) => (
                <div
                  key={dept.id}
                  className="bg-white/70 backdrop-blur-[30px] saturate-[210%] border border-white/50 shadow-sm p-4 rounded-[16px] flex items-start gap-4 cursor-pointer hover:shadow-md transition-shadow"
                >
                  <div className="w-12 h-12 rounded-xl bg-indigo-50 flex items-center justify-center text-2xl shrink-0">
                    {dept.icon}
                  </div>
                  <div>
                    <h3 className="font-bold text-gray-900 font-outfit text-lg">{dept.name}</h3>
                    <p className="text-xs font-semibold text-indigo-600 uppercase tracking-wide mb-1">{dept.role}</p>
                    <p className="text-sm text-gray-600 leading-relaxed">{dept.description}</p>

                    {showAdvanced && (
                      <div className="mt-4 pt-4 border-t border-gray-100/50 space-y-4">
                        <div>
                          <label className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2 block">Agent Persona & Tone</label>
                          <textarea
                            className="w-full p-3 rounded-[8px] border border-white/40 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none bg-white/65 backdrop-blur-[30px] saturate-[210%] text-gray-900 shadow-inner transition-all text-sm resize-none h-20"
                            placeholder="e.g. Friendly, professional, and helpful..."
                            value={prompts[dept.id] || ''}
                            onChange={(e) => setPrompts({...prompts, [dept.id]: e.target.value})}
                          />
                        </div>
                        <div>
                          <label className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2 block">Capabilities</label>
                          <div className="space-y-2">
                            <label className="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                              <input
                                type="checkbox"
                                className="w-4 h-4 rounded-[4px] border-gray-300 text-[#0066FF] focus:ring-[#0066FF]"
                                checked={toggles[dept.id]?.autoDraft ?? true}
                                onChange={(e) => setToggles({...toggles, [dept.id]: {...(toggles[dept.id] || {}), autoDraft: e.target.checked}})}
                              />
                              Auto-draft responses
                            </label>
                            <label className="flex items-center gap-2 text-sm text-gray-700 cursor-pointer">
                              <input
                                type="checkbox"
                                className="w-4 h-4 rounded-[4px] border-gray-300 text-[#0066FF] focus:ring-[#0066FF]"
                                checked={toggles[dept.id]?.autoPublish ?? false}
                                onChange={(e) => setToggles({...toggles, [dept.id]: {...(toggles[dept.id] || {}), autoPublish: e.target.checked}})}
                              />
                              Publish without approval
                            </label>
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              ))}
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
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
