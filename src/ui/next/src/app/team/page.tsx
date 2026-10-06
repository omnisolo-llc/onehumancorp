"use client";

import React, { useEffect, useState } from 'react';
import { useTeamApprovals } from './useTeamApprovals';
import type { Approval } from './approvalContract';
import DepartmentCard from './components/DepartmentCard';
import ApprovalInbox from './components/ApprovalInbox';
import GrowthReferralWidget from '../components/GrowthReferralWidget';

export type ApprovalRequest = Omit<Approval, 'status' | 'department' | 'action_risk'> & { status: string; department: string; action_risk: string };

const DEPARTMENTS = [
  { id: 'operations', name: 'The Manager' },
  { id: 'marketing', name: 'The Promoter' },
  { id: 'sales', name: 'The Salesperson' },
  { id: 'customer_success', name: 'The Ambassador' },
  { id: 'finance', name: 'The Accountant' },
  { id: 'legal', name: 'The Protector' },
  { id: 'business_advisory', name: 'The Advisor' },
];

export default function TeamPage() {
  const team = useTeamApprovals();
  const { items: approvals, loading } = team;
  const [selectedDepartment, setSelectedDepartment] = useState<string | null>(null);
  useEffect(() => { setSelectedDepartment(null); }, [team.revision]);
  const status = <div className="p-4 space-y-2">
    {team.error && <p role="alert">{team.error}</p>}
    {[...team.notices].map(([id, notice]) => <p key={id} role="status">{notice.message}</p>)}
    <button type="button" disabled={team.busy} onClick={() => void team.refresh()} className="min-h-[44px] text-blue-700 underline">Refresh recorded decisions</button>
  </div>;
  const handleApprove = (id: string, editedPayload?: import('@/lib/agent-feed-types').ActionPayload) => team.decide(id, 'APPROVED', editedPayload);
  const handleReject = (id: string) => team.decide(id, 'DISMISSED');

  if (selectedDepartment) {
    const deptInfo = DEPARTMENTS.find(d => d.id === selectedDepartment);
    const deptApprovals = approvals.filter(a => a.department === selectedDepartment);

    return (
      <div hidden={!team.ready}><ApprovalInbox
        departmentId={selectedDepartment}
        departmentName={deptInfo?.name || selectedDepartment}
        approvals={deptApprovals}
        onBack={() => setSelectedDepartment(null)}
        onApprove={handleApprove}
        onReject={handleReject}
        blocked={team.blocked}
        status={status}
        readState={team.loading ? 'loading' : team.error || !team.ready ? 'unavailable' : 'ready'}
      /></div>
    );
  }

  return (
    <div className="flex w-full flex-col items-center bg-gray-50 font-inter lg:py-4">
      <div className="w-full max-w-[375px] lg:max-w-6xl min-h-[812px] lg:min-h-0 mx-auto bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200 lg:rounded-2xl lg:border">

        {/* Header */}
        <div className="py-6 px-6 lg:px-8 bg-white/65 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex justify-between items-center">
          <div>
            <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Your Team</h1>
            <p className="text-gray-500 text-sm mt-1">Invisible specialized AI teams</p>
          </div>
          <button
            onClick={() => window.location.href = '/team/chat'}
            className="w-10 h-10 bg-[#0071E3] hover:bg-blue-700 text-white rounded-full flex items-center justify-center shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all"
            aria-label="Team Chat"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" /></svg>
          </button>
        </div>

        {status}
        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 lg:px-8 lg:pb-8 hide-scrollbar">

          <div className="mb-6">
            <GrowthReferralWidget />
          </div>

          <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-1">AI Departments</h2>

          {loading ? (
             <div className="flex justify-center py-10">
               <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
             </div>
          ) : team.error || !team.ready ? null : (
            <div className="grid grid-cols-1 gap-x-4 lg:grid-cols-2">
              {DEPARTMENTS.map(dept => {
                const pendingCount = approvals.filter(a => a.department === dept.id).length;
                return (
                  <DepartmentCard
                    key={dept.id}
                    name={dept.name}
                    pendingCount={pendingCount}
                    onClick={() => setSelectedDepartment(dept.id)}
                  />
                );
              })}
            </div>
          )}
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes slideUp {
          from { transform: translateY(100%); }
          to { transform: translateY(0); }
        }
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
