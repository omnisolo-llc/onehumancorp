'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function AgentsPage() {
  const [activeTab, setActiveTab] = useState<'departments' | 'feed' | 'approvals'>('departments');
  const [feed, setFeed] = useState<any[]>([]);
  const [feedLoading, setFeedLoading] = useState(false);
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [approvals, setApprovals] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [actionLoading, setActionLoading] = useState<string | null>(null);
  const [showWizard, setShowWizard] = useState(false);
  const [agentName, setAgentName] = useState('');
  const [agentRole, setAgentRole] = useState('Operations');
  const [providerType, setProviderType] = useState('openai');
  const [agentModel, setAgentModel] = useState('');
  const [isHiring, setIsHiring] = useState(false);
  const [hiredAgents, setHiredAgents] = useState<{name: string, role: string, id: string}[]>([]);
  const [hireSuccess, setHireSuccess] = useState(false);

  const fetchFeed = async () => {
    setFeedLoading(true);
    try {
      const res = await fetch('/api/agents/feed');
      if (res.ok) {
        const data = await res.json();
        setFeed(data.feed || []);
      }
    } catch (e) {
      console.error('Failed to fetch feed:', e);
    }
    setFeedLoading(false);
  };

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
    fetchFeed();
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

  const handleHireAgent = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsHiring(true);
    try {
      const payload: any = {
        name: agentName,
        role: agentRole,
        providerType: providerType,
      };
      if (agentModel) {
        payload.model = agentModel;
      }
      const res = await fetch('/api/agents/hire', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (res.ok) {
        setHireSuccess(true);
        setHiredAgents([...hiredAgents, {name: agentName, role: agentRole, id: Date.now().toString()}]);
        setTimeout(() => { setShowWizard(false); setHireSuccess(false); setAgentName(''); }, 2000);
      }
    } catch (err) {
      console.error(err);
    }
    setIsHiring(false);
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
          <div className="flex justify-between items-end">
            <div>
              <h1 className="text-3xl font-extrabold font-outfit text-gray-900">AI Departments</h1>
              <p className="text-sm text-gray-500 mt-1">Your autonomous business team.</p>
            </div>
            <button
              onClick={() => setShowWizard(true)}
              className="bg-indigo-600 hover:bg-indigo-700 text-white px-3 py-1.5 rounded-[8px] text-sm font-semibold transition-colors shadow-sm flex items-center gap-1">
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg> Hire Agent
            </button>
          </div>
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
            onClick={() => setActiveTab('feed')}
            className={`flex-1 py-3 text-sm font-semibold transition-colors ${activeTab === 'feed' ? 'text-indigo-600 border-b-2 border-indigo-600' : 'text-gray-500 hover:text-gray-700'}`}
          >
            Activity Feed
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
              {hiredAgents.map((agent) => (
                <div
                  key={agent.id}
                  className="bg-white/70 backdrop-blur-[30px] saturate-[210%] border border-indigo-200 shadow-sm p-4 rounded-[16px] flex items-start gap-4 cursor-pointer hover:shadow-md transition-shadow relative overflow-hidden"
                >
                  <div className="absolute top-0 right-0 bg-indigo-500 text-white text-[10px] font-bold px-2 py-0.5 rounded-bl-lg">NEW</div>
                  <div className="w-12 h-12 rounded-xl bg-indigo-50 flex items-center justify-center text-2xl shrink-0">
                    🤖
                  </div>
                  <div>
                    <h3 className="font-bold text-gray-900 font-outfit text-lg">{agent.name}</h3>
                    <p className="text-xs font-semibold text-indigo-600 uppercase tracking-wide mb-1">{agent.role}</p>
                    <p className="text-sm text-gray-600 leading-relaxed">Active and ready to work.</p>
                    {showAdvanced && (
                      <div className="mt-3 pt-3 border-t border-gray-100 flex gap-4 text-xs text-gray-500">
                        <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-green-500"></span> Active</span>
                      </div>
                    )}
                  </div>
                </div>
              ))}
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
                      <div className="mt-3 pt-3 border-t border-gray-100 flex gap-4 text-xs text-gray-500">
                        <span className="flex items-center gap-1">
                          <span className="w-2 h-2 rounded-full bg-green-500"></span> Active
                        </span>
                        <span>Auto-approve: $0</span>
                      </div>
                    )}
                  </div>
                </div>
              ))}
            </div>

          ) : activeTab === 'feed' ? (
            <div className="h-full flex flex-col">
              {feedLoading ? (
                <div className="flex flex-col items-center justify-center flex-1 text-center py-12">
                  <div className="w-8 h-8 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin mb-4"></div>
                  <p className="text-gray-500 text-sm font-medium">Fetching feed...</p>
                </div>
              ) : feed.length === 0 ? (
                 <div className="flex flex-col items-center justify-center flex-1 text-center py-12">
                  <p className="text-gray-500 text-sm">No activity yet.</p>
                </div>
              ) : (
                <div className="space-y-4 pb-8">
                  {feed.map((item) => (
                    <div key={item.id} className="bg-white rounded-[16px] shadow-sm border border-gray-200 p-5 font-inter">
                      <div className="flex justify-between items-start mb-3">
                        <span className="bg-blue-100 text-blue-800 text-xs font-bold px-2.5 py-1 rounded-md uppercase tracking-wide">
                          {item.department}
                        </span>
                        <span className="text-xs text-gray-400 font-medium">{item.timestamp}</span>
                      </div>
                      <p className="text-gray-800 text-sm font-medium leading-relaxed">
                        {item.description}
                      </p>
                    </div>
                  ))}
                </div>
              )}
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

        {/* Wizard Modal */}
        {showWizard && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-gray-900/40 backdrop-blur-sm transition-opacity">
            <div className="bg-white/90 backdrop-blur-[30px] saturate-[210%] w-full max-w-sm rounded-[16px] shadow-2xl overflow-hidden border border-white/50 relative flex flex-col">
              <div className="p-5 border-b border-gray-100 flex justify-between items-center">
                <h2 className="text-xl font-bold font-outfit text-gray-900">Hire Agent</h2>
                <button onClick={() => setShowWizard(false)} className="text-gray-400 hover:text-gray-600">
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>

              {hireSuccess ? (
                <div className="p-8 text-center flex flex-col items-center">
                  <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-4">
                    <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                  </div>
                  <h3 className="text-xl font-bold text-gray-900 mb-1">Agent Hired!</h3>
                  <p className="text-sm text-gray-500">Your new AI teammate is ready to work.</p>
                </div>
              ) : (
                <form onSubmit={handleHireAgent} className="p-5 flex flex-col gap-4">
                  <div>
                    <label className="block text-sm font-semibold text-gray-700 mb-1">Agent Name</label>
                    <input
                      type="text"
                      required
                      placeholder="e.g. Nova, Jules"
                      className="w-full px-3 py-2 border border-gray-200 rounded-[8px] focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm"
                      value={agentName}
                      onChange={(e) => setAgentName(e.target.value)}
                    />
                  </div>

                  <div>
                    <label className="block text-sm font-semibold text-gray-700 mb-1">Role</label>
                    <select
                      className="w-full px-3 py-2 border border-gray-200 rounded-[8px] focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm bg-white"
                      value={agentRole}
                      onChange={(e) => setAgentRole(e.target.value)}
                    >
                      <option value="Operations">Operations (The Manager)</option>
                      <option value="Customer Success">Customer Success (The Ambassador)</option>
                      <option value="Marketing">Marketing (The Promoter)</option>
                      <option value="Sales">Sales (The Closer)</option>
                      <option value="Finance">Finance (The Accountant)</option>
                      <option value="Legal">Legal (The Counsel)</option>
                      <option value="Advisory">Advisory (The Strategist)</option>
                    </select>
                  </div>

                  {showAdvanced && (
                    <div className="pt-3 mt-1 border-t border-gray-100 flex flex-col gap-4">
                      <div>
                        <label className="block text-sm font-semibold text-gray-700 mb-1">Provider Type</label>
                        <select
                          className="w-full px-3 py-2 border border-gray-200 rounded-[8px] focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm bg-white"
                          value={providerType}
                          onChange={(e) => setProviderType(e.target.value)}
                        >
                          <option value="openai">OpenAI</option>
                          <option value="minimax">MiniMax</option>
                          <option value="anthropic">Anthropic</option>
                          <option value="ollama">Ollama</option>
                          <option value="openai-compatible">OpenAI-Compatible</option>
                        </select>
                      </div>
                      <div>
                        <label className="block text-sm font-semibold text-gray-700 mb-1">Model (Optional)</label>
                        <input
                          type="text"
                          placeholder="Leave blank for default"
                          className="w-full px-3 py-2 border border-gray-200 rounded-[8px] focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent text-sm"
                          value={agentModel}
                          onChange={(e) => setAgentModel(e.target.value)}
                        />
                      </div>
                    </div>
                  )}

                  <button
                    type="submit"
                    disabled={isHiring || !agentName}
                    className="mt-2 w-full bg-indigo-600 hover:bg-indigo-700 text-white py-3 rounded-[8px] text-sm font-semibold transition-colors shadow-sm disabled:bg-indigo-400 disabled:cursor-not-allowed"
                  >
                    {isHiring ? 'Hiring...' : 'Confirm Hire'}
                  </button>
                </form>
              )}
            </div>
          </div>
        )}
      </div>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
