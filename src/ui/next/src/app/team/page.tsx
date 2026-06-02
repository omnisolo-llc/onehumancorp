"use client";

import React, { useState, useEffect } from 'react';
import DepartmentCard from './components/DepartmentCard';
import ApprovalInbox from './components/ApprovalInbox';
import AddStaffModal from './components/AddStaffModal';

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
  const [isAddStaffModalOpen, setIsAddStaffModalOpen] = useState(false);
  const [staffMembers, setStaffMembers] = useState<{id: string, name: string, role: string, status: string, duration: string}[]>([]);
  const [loading, setLoading] = useState(true);
  const [showInviteModal, setShowInviteModal] = useState(false);
  const [linkCopied, setLinkCopied] = useState(false);
  const [inviteLink, setInviteLink] = useState('https://ohc.app/invite/team-default');

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

    if (typeof window !== 'undefined' && window.localStorage) {
      const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'team-default';
      setInviteLink(`https://ohc.app/invite/${encodeURIComponent(tenantId)}`);
    }
  }, []);

  const handleAddStaff = async (name: string, role: string, phoneNumber: string) => {
    try {
      const res = await fetch('/api/v1/staff', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, role, phone_number: phoneNumber })
      });
      if (res.ok) {
        const data = await res.json();
        setStaffMembers([...staffMembers, { id: Date.now().toString(), name, role, status: 'Invited', duration: '' }]);
        alert('Staff member added! PIN setup link: ' + data.pin_setup_link);
      }
    } catch (error) {
      console.error('Failed to add staff', error);
    }
  };

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

          <div className="mb-6 p-5 rounded-[16px] border flex flex-col gap-3 shadow-sm relative overflow-hidden group" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderColor: 'rgba(255, 255, 255, 0.4)' }}>
            <div className="absolute inset-0 bg-gradient-to-r from-blue-50/50 to-indigo-50/50 opacity-0 group-hover:opacity-100 transition-opacity"></div>
            <div className="relative z-10">
              <h2 className="text-xl font-semibold font-outfit text-gray-900">Grow Your Team</h2>
              <p className="text-sm text-gray-600 mt-1 mb-3">Bridge your local sovereignty with cloud-native collaboration. Invite a member to a shared multi-tenant space.</p>
              <button
                onClick={() => setShowInviteModal(true)}
                className="w-full py-2.5 bg-gray-900 text-white rounded-xl text-sm font-semibold shadow-md hover:bg-black transition-all active:scale-[0.98]"
              >
                Invite to Cloud Team
              </button>
            </div>
          </div>

          <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider mb-4 px-1">AI Departments</h2>

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

        <button
          onClick={() => setIsAddStaffModalOpen(true)}
          className="absolute bottom-6 right-6 w-14 h-14 bg-blue-600 hover:bg-blue-700 text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 active:scale-[0.95] transition-all z-20"
        >
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 4v16m8-8H4"></path></svg>
        </button>

        <AddStaffModal isOpen={isAddStaffModalOpen} onClose={() => setIsAddStaffModalOpen(false)} onAddStaff={handleAddStaff} />

        {/* Cloud Bridge Invite Modal */}
        {/* Human Staff Section */}
        <div className="mb-6">
          <h2 className="text-lg font-outfit font-semibold text-gray-800 mb-3 ml-1">Human Staff</h2>
          {staffMembers.length === 0 ? (
            <div className="text-center py-6 bg-white/40 rounded-2xl border border-gray-200 border-dashed">
              <p className="text-gray-500 text-sm">No human staff added yet.</p>
            </div>
          ) : (
            <div className="space-y-3">
              {staffMembers.map(staff => (
                <div key={staff.id} className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-2xl p-4 shadow-sm flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-full bg-green-100 flex items-center justify-center border border-green-200">
                      <span className="text-green-700 font-bold">{staff.name.charAt(0)}</span>
                    </div>
                    <div>
                      <p className="font-semibold text-gray-900">{staff.name}</p>
                      <p className="text-xs text-gray-500">{staff.role}</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className="flex items-center gap-1.5 justify-end">
                      <div className={`w-2 h-2 rounded-full ${staff.status === 'Clocked In' ? 'bg-green-500' : 'bg-yellow-500'}`}></div>
                      <span className="text-sm font-medium text-gray-700">{staff.status}</span>
                    </div>
                    {staff.duration && <p className="text-xs text-gray-500 mt-0.5">{staff.duration}</p>}
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* AI Agents Section */}
        <div className="mb-3">
          <h2 className="text-lg font-outfit font-semibold text-gray-800 ml-1">AI Agents</h2>
        </div>

        {showInviteModal && (
          <div className="absolute inset-0 bg-black/40 z-50 flex flex-col justify-end">
            <div
              className="bg-white rounded-t-3xl p-6 shadow-2xl transition-transform duration-300"
              style={{ animation: 'slideUp 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}
            >
              <div className="flex justify-between items-start mb-2">
                <h2 className="text-xl font-bold font-outfit text-gray-900">Cloud Bridge Invite</h2>
                <button
                  onClick={() => setShowInviteModal(false)}
                  className="w-8 h-8 flex items-center justify-center rounded-full bg-gray-100 text-gray-500 hover:bg-gray-200 transition-colors"
                  aria-label="Close Cloud Bridge Invite"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>

              <p className="text-sm text-gray-600 mb-4">Share this link to provision a temporary multi-tenant context for your collaborator, while you maintain local sovereignty.</p>

              <div className="bg-gray-50 p-3 rounded-xl border border-gray-200 flex items-center gap-2 mb-4">
                <input
                  id="cloud-bridge-invite-link"
                  type="text"
                  readOnly
                  value={inviteLink}
                  className="flex-1 bg-transparent text-sm text-gray-700 outline-none"
                />
              </div>

              <button
                onClick={() => {
                  navigator.clipboard.writeText(inviteLink);
                  setLinkCopied(true);
                  setTimeout(() => setLinkCopied(false), 2000);
                }}
                className={`w-full py-3 rounded-xl text-sm font-bold shadow-md transition-all active:scale-[0.98] ${linkCopied ? 'bg-green-600 text-white hover:bg-green-700' : 'bg-blue-600 text-white hover:bg-blue-700'}`}
              >
                {linkCopied ? 'Copied!' : 'Copy Link'}
              </button>
            </div>
          </div>
        )}
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes slideUp {
          from { transform: translateY(100%); }
          to { transform: translateY(0); }
        }
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
