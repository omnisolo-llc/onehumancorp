"use client";

import React, { useState, useEffect } from 'react';

type DepartmentType = 'Operations' | 'Marketing' | 'Sales' | 'CustomerSuccess' | 'Finance' | 'Legal' | 'BusinessAdvisory';

interface ApprovalRequest {
  id: string;
  department: DepartmentType;
  description: string;
  status: string;
  action_risk?: string;
}

const DEPARTMENT_INFO: Record<DepartmentType, { name: string, icon: string, role: string, color: string }> = {
  Operations: { name: 'The Manager', icon: '💼', role: 'Operations', color: 'from-blue-500 to-indigo-600' },
  Marketing: { name: 'The Promoter', icon: '📣', role: 'Marketing', color: 'from-pink-500 to-rose-600' },
  Sales: { name: 'The Salesperson', icon: '🤝', role: 'Sales', color: 'from-green-500 to-emerald-600' },
  CustomerSuccess: { name: 'The Ambassador', icon: '🌟', role: 'Customer Success', color: 'from-yellow-400 to-orange-500' },
  Finance: { name: 'The Accountant', icon: '📊', role: 'Finance', color: 'from-emerald-500 to-teal-600' },
  Legal: { name: 'The Protector', icon: '🛡️', role: 'Legal', color: 'from-slate-500 to-gray-600' },
  BusinessAdvisory: { name: 'The Advisor', icon: '💡', role: 'Advisory', color: 'from-purple-500 to-fuchsia-600' },
};

export default function TeamPage() {
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchApprovals();
  }, []);

  const fetchApprovals = async () => {
    try {
      setLoading(true);
      const res = await fetch('/api/agents/approvals');
      if (!res.ok) throw new Error('Failed to fetch approvals');
      const data = await res.json();
      setApprovals(data.pending_approvals || []);
      setError(null);
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  const handleApproval = async (id: string, approved: boolean) => {
    try {
      // Optimistically remove
      setApprovals(prev => prev.filter(a => a.id !== id));

      const res = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ data: { approved } })
      });

      if (!res.ok) {
        throw new Error('Failed to submit approval');
      }
    } catch (err) {
      // Revert on error
      fetchApprovals();
      console.error(err);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-8">
      <div className="w-[375px] min-h-[812px] bg-white shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden">

        {/* Header */}
        <div className="bg-gray-900 text-white p-6 rounded-b-3xl shadow-md z-10">
          <h1 className="text-3xl font-bold font-outfit mb-1">Your Team</h1>
          <p className="text-gray-300 text-sm opacity-90">The Invisible Workforce</p>
        </div>

        <div className="flex-1 overflow-y-auto hide-scrollbar pb-24 bg-gray-50/50 relative">

          {/* Team Roster section */}
          <div className="p-6">
            <div className="flex justify-between items-end mb-4">
              <h2 className="text-xl font-bold font-outfit text-gray-800">Active Agents</h2>
              <button className="text-xs text-blue-600 font-medium">Settings</button>
            </div>

            <div className="grid grid-cols-4 gap-3 mb-2">
              {['Operations', 'Marketing', 'CustomerSuccess', 'Finance'].map(dept => {
                const info = DEPARTMENT_INFO[dept as DepartmentType];
                if (!info) return null;
                return (
                  <div key={dept} className="flex flex-col items-center gap-1 group cursor-pointer">
                    <div className={`w-14 h-14 rounded-full bg-gradient-to-br ${info.color} flex items-center justify-center text-2xl shadow-sm border-2 border-white ring-2 ring-transparent group-hover:ring-gray-200 transition-all`}>
                      {info.icon}
                    </div>
                    <span className="text-[10px] font-semibold text-center text-gray-700 leading-tight">
                      {info.name}
                    </span>
                  </div>
                );
              })}
            </div>
          </div>

          {/* Daily Brief section */}
          <div className="px-4 pb-6">
            <h2 className="text-xl font-bold font-outfit text-gray-800 mb-4 px-2">The Daily Brief</h2>

            {loading ? (
              <div className="flex justify-center p-8">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
              </div>
            ) : error ? (
              <div className="p-4 bg-red-50 text-red-600 rounded-xl text-sm border border-red-100 mx-2">
                {error}
              </div>
            ) : approvals.length === 0 ? (
              <div className="glassmorphism p-8 rounded-2xl mx-2 text-center border border-gray-200/50 bg-white/60 shadow-sm">
                <div className="text-4xl mb-3">☕</div>
                <h3 className="font-bold text-gray-800 font-outfit mb-1">All caught up!</h3>
                <p className="text-xs text-gray-500">Your team has no pending tasks.</p>
              </div>
            ) : (
              <div className="space-y-4 mx-2">
                {approvals.map(approval => {
                  const info = DEPARTMENT_INFO[approval.department] || DEPARTMENT_INFO.Operations;
                  return (
                    <div key={approval.id} className="bg-white/80 backdrop-blur-md border border-gray-100 rounded-2xl p-4 shadow-sm relative overflow-hidden group">
                      {/* Gradient Accent */}
                      <div className={`absolute top-0 left-0 w-1 h-full bg-gradient-to-b ${info.color}`}></div>

                      <div className="flex items-start gap-3 mb-3 pl-2">
                        <div className="text-2xl mt-1">{info.icon}</div>
                        <div className="flex-1">
                          <div className="flex items-center gap-2 mb-1">
                            <span className="font-bold text-sm text-gray-900 font-outfit">{info.name}</span>
                            <span className="text-[10px] uppercase tracking-wider font-semibold text-gray-400 bg-gray-100 px-1.5 py-0.5 rounded">Draft</span>
                          </div>
                          <p className="text-sm text-gray-600 leading-snug">{approval.description}</p>
                        </div>
                      </div>

                      <div className="flex gap-2 pl-2 mt-4">
                        <button
                          onClick={() => handleApproval(approval.id, true)}
                          className="flex-1 bg-gray-900 text-white text-sm font-semibold py-2.5 rounded-xl hover:bg-black active:scale-[0.98] transition-all shadow-sm"
                        >
                          Approve
                        </button>
                        <button
                          onClick={() => handleApproval(approval.id, false)}
                          className="px-4 text-sm font-semibold text-gray-500 bg-gray-100 rounded-xl hover:bg-gray-200 active:scale-[0.98] transition-all"
                        >
                          Discard
                        </button>
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        </div>

        {/* Bottom Nav Simulation */}
        <div className="absolute bottom-0 w-full bg-white border-t border-gray-200 pb-safe z-50">
          <div className="flex justify-around items-center h-16 px-6">
            <button className="flex flex-col items-center gap-1 opacity-40">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>
              <span className="text-[10px] font-medium">Home</span>
            </button>
            <button className="flex flex-col items-center gap-1 opacity-100 text-blue-600">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
              <span className="text-[10px] font-medium">Team</span>
            </button>
            <button className="flex flex-col items-center gap-1 opacity-40">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
              <span className="text-[10px] font-medium">Settings</span>
            </button>
          </div>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism { background: rgba(255, 255, 255, 0.4); backdrop-filter: blur(12px); -webkit-backdrop-filter: blur(12px); border: 1px solid rgba(255, 255, 255, 0.5); }
      `}} />
    </div>
  );
}
