"use client";

import React, { useState, useEffect } from 'react';
import DepartmentCard from './components/DepartmentCard';
import ApprovalInbox from './components/ApprovalInbox';

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
  const [selectedDepartment, setSelectedDepartment] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [showInviteModal, setShowInviteModal] = useState(false);
  const [inviteLink, setInviteLink] = useState("");
  const [linkCopied, setLinkCopied] = useState(false);

  const handleGenerateInvite = async () => {
    try {
      const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'e2e-tenant' : 'e2e-tenant';
      const userId = 'current-user-id'; // Use current user ID in reality
      const uuid = Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);

      const req = {
        team_id: tenantId,
        inviter_id: userId,
        invitee_id: `guest-${uuid}`,
      };

      await fetch('/api/v1/growth/team-invites', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(req)
      });

      setInviteLink(`https://ohc.app/invite/${uuid}`);
      setShowInviteModal(true);
    } catch (e) {
      console.error('Failed to generate invite', e);
      // Fallback behavior if server is unavailable
      const uuid = Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
      setInviteLink(`https://ohc.app/invite/${uuid}`);
      setShowInviteModal(true);
    }
  };

  const handleCopyLink = () => {
    navigator.clipboard.writeText(inviteLink);
    setLinkCopied(true);
    setTimeout(() => setLinkCopied(false), 2000);
  };

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

  const handleApprove = async (id: string) => {
    try {
      setApprovals(prev => prev.filter(a => a.id !== id));
      const response = await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved: true })
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

  if (selectedDepartment) {
    const deptInfo = DEPARTMENTS.find(d => d.id === selectedDepartment);
    const deptApprovals = approvals.filter(a => a.department === selectedDepartment);

    return (
      <ApprovalInbox
        departmentId={selectedDepartment}
        departmentName={deptInfo?.name || selectedDepartment}
        approvals={deptApprovals}
        onBack={() => setSelectedDepartment(null)}
        onApprove={handleApprove}
        onReject={handleReject}
      />
    );
  }

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
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 hide-scrollbar">

          {/* Cloud Bridge Growth Loop */}
          <div className="mb-8 p-5 bg-white/70 backdrop-blur-xl border border-blue-100 rounded-2xl shadow-sm">
            <h2 className="text-lg font-bold font-outfit text-gray-900 mb-2">Grow Your Team</h2>
            <p className="text-sm text-gray-600 mb-4">
              Bridge your local sovereignty with cloud-native collaboration.
            </p>
            <button
              onClick={handleGenerateInvite}
              className="w-full py-2.5 bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-semibold rounded-xl text-sm shadow-md hover:shadow-lg transition-all"
            >
              Invite to Cloud Team
            </button>
          </div>

          {loading ? (
             <div className="flex justify-center py-10">
               <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
             </div>
          ) : (
            DEPARTMENTS.map(dept => {
              const pendingCount = approvals.filter(a => a.department === dept.id).length;
              return (
                <DepartmentCard
                  key={dept.id}
                  name={dept.name}
                  pendingCount={pendingCount}
                  onClick={() => setSelectedDepartment(dept.id)}
                />
              );
            })
          )}
        </div>
      </div>

      {showInviteModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
          <div className="w-full max-w-sm bg-white rounded-2xl shadow-2xl overflow-hidden font-inter border border-gray-100">
            <div className="p-6">
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Cloud Bridge Invite</h2>
              <p className="text-sm text-gray-600 mb-6">
                Share this link to provision a temporary multi-tenant context for your collaborator.
              </p>

              <div className="mb-4">
                <input
                  id="cloud-bridge-invite-link"
                  type="text"
                  readOnly
                  value={inviteLink}
                  className="w-full px-3 py-2 text-sm bg-gray-50 border border-gray-200 rounded-lg focus:outline-none text-gray-800 font-mono"
                />
              </div>

              <div className="flex flex-col gap-3">
                <button
                  onClick={handleCopyLink}
                  className={`w-full py-2 rounded-xl text-sm font-semibold transition-all ${linkCopied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                >
                  {linkCopied ? 'Copied!' : 'Copy Link'}
                </button>
                <button
                  onClick={() => setShowInviteModal(false)}
                  className="w-full py-2 bg-white text-gray-700 border border-gray-200 rounded-xl text-sm font-semibold hover:bg-gray-50 transition-all"
                >
                  Close Cloud Bridge Invite
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

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
