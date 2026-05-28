"use client";

import React, { useState, useEffect } from 'react';
import DepartmentCard from './components/DepartmentCard';
import ActionFeed from './components/ActionFeed';

export type ApprovalRequest = {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
  payload?: any;
};

const DEPARTMENTS = [
  { id: 'operations', name: 'The Manager' },
  { id: 'marketing', name: 'The Promoter' },
  { id: 'sales', name: 'The Salesperson' },
  { id: 'customer_success', name: 'The Ambassador' },
  { id: 'finance', name: 'The Accountant' },
  { id: 'legal', name: 'The Protector' },
  { id: 'business_advisory', name: 'The Advisor' },
  { id: 'discovery', name: 'The Scout' },
];

export default function TeamPage() {
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [activities, setActivities] = useState<ApprovalRequest[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchData = async () => {
    try {
      const [appRes, actRes] = await Promise.all([
        fetch('/api/agents/approvals'),
        fetch('/api/agents/approvals/activity')
      ]);

      if (appRes.ok) {
        const appData = await appRes.json();
        setApprovals(appData.pending_approvals || []);
      }

      if (actRes.ok) {
        const actData = await actRes.json();
        setActivities(actData.pending_approvals || []);
      }
    } catch (error) {
      console.error("Failed to fetch data", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchData();
  }, []);

  const handleApprove = async (id: string) => {
    try {
      setApprovals(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved: true })
      });
      if (!response.ok) fetchData();
    } catch (error) {
      console.error("Failed to approve", error);
      fetchData();
    }
  };

  const handleReject = async (id: string) => {
     try {
      setApprovals(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved: false })
      });
      if (!response.ok) fetchData();
    } catch (error) {
      console.error("Failed to reject", error);
      fetchData();
    }
  };

  const feedItems = [...approvals, ...activities];

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/65 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Your Team</h1>
            <p className="text-gray-500 text-sm mt-1">Invisible specialized AI teams</p>
          </div>
          <button
            onClick={() => window.location.href = '/team/chat'}
            className="w-10 h-10 bg-blue-600 hover:bg-blue-700 text-white rounded-full flex items-center justify-center shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all"
            aria-label="Team Chat"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" /></svg>
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto pb-24 hide-scrollbar">
          {/* Horizontal scroll for departments */}
          <div className="px-6 py-6 border-b border-gray-200/50">
            <div className="flex overflow-x-auto pb-2 -mx-6 px-6 hide-scrollbar">
              {DEPARTMENTS.map(dept => {
                const pendingCount = approvals.filter(a => {
                   let formattedDept = dept.id.replace('_', '');
                   let aDept = a.department.replace('_', '').toLowerCase();
                   return aDept === formattedDept || a.department === dept.id;
                }).length;
                return (
                  <DepartmentCard
                    key={dept.id}
                    name={dept.name}
                    pendingCount={pendingCount}
                    onClick={() => {}}
                  />
                );
              })}
            </div>
          </div>

          {/* Vertical Action Feed */}
          <div className="px-4 py-6 bg-gray-50/50 min-h-[500px]">
            <h2 className="text-xs font-bold text-gray-500 mb-4 font-outfit uppercase tracking-wider px-2">Action Feed</h2>
            {loading ? (
               <div className="flex justify-center py-10">
                 <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
               </div>
            ) : (
              <ActionFeed feedItems={feedItems} onApprove={handleApprove} onReject={handleReject} />
            )}
          </div>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
