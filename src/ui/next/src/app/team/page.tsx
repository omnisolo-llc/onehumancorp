"use client";

import React, { useState, useEffect } from 'react';
import DepartmentCard from './components/DepartmentCard';
import ApprovalCard from './components/ApprovalCard';

export type ApprovalRequest = {
  id: string;
  tenant_id: string;
  department: string;
  description: string;
  status: string;
  action_risk: string;
};

const DEPARTMENTS = [
  { id: 'Operations', name: 'The Manager' },
  { id: 'Marketing', name: 'The Promoter' },
  { id: 'Sales', name: 'The Salesperson' },
  { id: 'CustomerSuccess', name: 'The Ambassador' },
  { id: 'Finance', name: 'The Accountant' },
  { id: 'Legal', name: 'The Protector' },
  { id: 'BusinessAdvisory', name: 'The Advisor' },
];

export default function TeamPage() {
  const [approvals, setApprovals] = useState<ApprovalRequest[]>([]);
  const [selectedDepartment, setSelectedDepartment] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const fetchApprovals = async () => {
    try {
      const response = await fetch('/api/agents/approvals');
      if (response.ok) {
        const data = await response.json();
        setApprovals(data.pending_approvals || []);
      }
    } catch (error) {
      console.error("Failed to fetch approvals", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchApprovals();
  }, []);

  const handleApprove = async (id: string, newDescription?: string) => {
    try {
      setApprovals(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved: true, description: newDescription })
      });
      if (!response.ok) fetchApprovals();
    } catch (error) {
      console.error("Failed to approve", error);
      fetchApprovals();
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
      if (!response.ok) fetchApprovals();
    } catch (error) {
      console.error("Failed to reject", error);
      fetchApprovals();
    }
  };

  const displayedApprovals = selectedDepartment
    ? approvals.filter(a => a.department === selectedDepartment)
    : approvals;

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/60 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Your Team</h1>
              <p className="text-gray-500 text-sm mt-1">Invisible specialized AI teams</p>
            </div>
            {selectedDepartment && (
              <button
                onClick={() => setSelectedDepartment(null)}
                className="text-sm font-semibold text-blue-600 bg-blue-50 px-3 py-1.5 rounded-full"
              >
                Show All
              </button>
            )}
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto pb-24 hide-scrollbar">
          {/* Departments Horizontal Scroll */}
          <div className="px-4 py-6">
            <h2 className="text-sm font-bold text-gray-900 font-outfit mb-4 px-2">DEPARTMENTS</h2>
            <div className="flex overflow-x-auto hide-scrollbar pb-2 px-2 -mx-2">
              {DEPARTMENTS.map(dept => {
                const pendingCount = approvals.filter(a => a.department === dept.id).length;
                return (
                  <div key={dept.id} className={selectedDepartment === dept.id ? 'ring-2 ring-blue-500 rounded-2xl scale-105 transition-transform' : 'transition-transform'}>
                    <DepartmentCard
                      name={dept.name}
                      pendingCount={pendingCount}
                      onClick={() => setSelectedDepartment(selectedDepartment === dept.id ? null : dept.id)}
                    />
                  </div>
                );
              })}
            </div>
          </div>

          {/* Action Feed */}
          <div className="px-4 pb-6">
            <h2 className="text-sm font-bold text-gray-900 font-outfit mb-4 px-2">ACTION FEED</h2>

            {loading ? (
              <div className="flex justify-center py-10">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
              </div>
            ) : displayedApprovals.length === 0 ? (
              <div className="flex flex-col items-center justify-center h-48 text-center px-8 bg-white/40 rounded-2xl border border-white/60">
                 <div className="w-12 h-12 bg-green-50 text-green-500 rounded-full flex items-center justify-center mb-3">
                   <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                     <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                   </svg>
                 </div>
                 <h3 className="font-outfit font-bold text-gray-900 text-md mb-1">All Caught Up!</h3>
                 <p className="text-xs text-gray-500">No actions require your review.</p>
              </div>
            ) : (
              <div className="space-y-4">
                {displayedApprovals.map(req => (
                  <ApprovalCard
                    key={req.id}
                    request={req}
                    onApprove={handleApprove}
                    onReject={handleReject}
                  />
                ))}
              </div>
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
