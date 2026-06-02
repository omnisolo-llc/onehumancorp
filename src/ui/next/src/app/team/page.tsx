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
  const [linkCopied, setLinkCopied] = useState(false);

  const [staffList, setStaffList] = useState<any[]>([
    { id: 'temp-maya', name: 'Maya', role: 'Manager', status: 'Offline' }
  ]);
  const [newStaffPhone, setNewStaffPhone] = useState('');
  const [newStaffRole, setNewStaffRole] = useState('cashier');

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

  const fetchStaff = async () => {
     try {
       const res = await fetch('/api/staff');
       if (res.ok) {
         const data = await res.json();
         setStaffList(data.staff);
       }
     } catch(e) {
       console.error(e);
     }
  };

  useEffect(() => {
    fetchApprovals();
    fetchStaff();
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

  const handleInviteStaff = async () => {
    try {
      const res = await fetch('/api/staff', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ action: 'create', role: newStaffRole, pin: '1234' })
      });
      if (res.ok) {
        setLinkCopied(true);
        setTimeout(() => {
          setLinkCopied(false);
          setShowInviteModal(false);
          fetchStaff();
        }, 1500);
      }
    } catch(e) {
      console.error(e);
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

          <div className="mb-6">
            <h2 className="text-xl font-semibold font-outfit text-gray-900 px-1 mb-3">Current Staff</h2>
            {staffList.map((staff, i) => (
            <div key={staff.id || i} className="bg-white/65 backdrop-blur-[30px] border border-white/40 rounded-2xl p-4 shadow-sm flex items-center justify-between mb-2">
              <div className="flex items-center gap-3">
                <div className={`w-10 h-10 rounded-full flex items-center justify-center font-bold font-outfit ${staff.role === 'Manager' ? 'bg-gray-100 text-gray-600' : 'bg-green-100 text-green-700'}`}>
                   {staff.name ? staff.name.charAt(0).toUpperCase() : 'S'}
                </div>
                <div>
                  <p className="font-semibold text-gray-900">{staff.name}</p>
                  <p className="text-xs text-gray-500 capitalize">{staff.role} • {staff.status || 'Offline'}</p>
                </div>
              </div>
              <button className="text-gray-400 hover:text-gray-600">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" /></svg>
              </button>
            </div>
            ))}
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

        {/* Floating Action Button */}
        <button
          onClick={() => setShowInviteModal(true)}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gray-900 text-white rounded-full flex items-center justify-center shadow-xl hover:bg-black transition-transform hover:scale-105 active:scale-95 z-20"
          aria-label="Add Staff"
        >
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
        </button>

        {/* Hire Staff Modal */}
        {showInviteModal && (
          <div className="absolute inset-0 bg-black/40 z-50 flex flex-col justify-end">
            <div
              className="bg-white rounded-t-3xl p-6 shadow-2xl transition-transform duration-300"
              style={{ animation: 'slideUp 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}
            >
              <div className="flex justify-between items-start mb-6">
                <h2 className="text-2xl font-bold font-outfit text-gray-900">Who are you hiring?</h2>
                <button
                  onClick={() => setShowInviteModal(false)}
                  className="w-8 h-8 flex items-center justify-center rounded-full bg-gray-100 text-gray-500 hover:bg-gray-200 transition-colors"
                  aria-label="Close Hire Modal"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>

              <div className="space-y-4 mb-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Phone Number</label>
                  <input
                    type="tel"
                    placeholder="(555) 000-0000"
                    value={newStaffPhone}
                    onChange={(e) => setNewStaffPhone(e.target.value)}
                    className="w-full border border-gray-300 rounded-xl px-4 py-3 text-gray-900 outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent transition-all"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Role</label>
                  <select
                    value={newStaffRole}
                    onChange={(e) => setNewStaffRole(e.target.value)}
                    className="w-full border border-gray-300 rounded-xl px-4 py-3 text-gray-900 outline-none focus:ring-2 focus:ring-gray-900 focus:border-transparent transition-all appearance-none bg-white">
                    <option value="cashier">Cashier</option>
                    <option value="manager">Manager</option>
                    <option value="driver">Delivery Driver</option>
                  </select>
                </div>
              </div>

              <button
                onClick={handleInviteStaff}
                className={`w-full py-3.5 rounded-xl text-base font-bold shadow-md transition-all active:scale-[0.98] ${linkCopied ? 'bg-green-600 text-white hover:bg-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
              >
                {linkCopied ? 'Invite Sent!' : 'Send SMS Invite'}
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
