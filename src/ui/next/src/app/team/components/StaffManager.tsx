"use client";

import React, { useState, useEffect } from 'react';

type StaffMember = {
  id: string;
  name: string;
  phone_number: string;
  role: string;
  pin_hash: string;
};

export default function StaffManager() {
  const [staff, setStaff] = useState<StaffMember[]>([]);
  const [summaries, setSummaries] = useState<any[]>([]);
  const [showAddForm, setShowAddForm] = useState(false);
  const [loading, setLoading] = useState(true);

  // Form State
  const [name, setName] = useState('');
  const [phone, setPhone] = useState('');
  const [role, setRole] = useState('Cashier');
  const [isSubmitting, setIsSubmitting] = useState(false);

  useEffect(() => {
    fetchStaff();
  }, []);

const fetchStaff = async () => {
    try {
      const response = await fetch('/api/staff');
      if (response.ok) {
        const data = await response.json();
        setStaff(data.staff || []);
      }

      const summariesRes = await fetch('/api/staff/summaries');
      if (summariesRes.ok) {
        const data = await summariesRes.json();
        setSummaries(data.summaries || []);
      }
    } catch (e) {
      console.error("Failed to fetch data", e);
    } finally {
      setLoading(false);
    }
  };

  const handleAddStaff = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!name || !phone || !role) return;

    setIsSubmitting(true);
    try {
      const response = await fetch('/api/staff', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, phone_number: phone, role })
      });

      if (response.ok) {
        await fetchStaff();
        setShowAddForm(false);
        setName('');
        setPhone('');
        setRole('Cashier');
      }
    } catch (e) {
      console.error("Failed to add staff", e);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="mb-6">
       <div className="flex justify-between items-center mb-4">
         <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider px-1">Your Human Staff</h2>
       </div>

       {loading ? (
          <div className="flex justify-center py-4">
             <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-gray-900"></div>
          </div>
       ) : (
         <div className="space-y-3">
           {staff.map(member => (
             <div key={member.id} className="app-card rounded-2xl p-4 shadow-sm flex items-center justify-between">
               <div>
                 <p className="font-semibold text-gray-900 font-outfit">{member.name}</p>
                 <p className="text-xs text-gray-500 mt-0.5">{member.role} • {member.phone_number}</p>
               </div>
               <div className="w-8 h-8 rounded-full bg-green-100 flex items-center justify-center text-green-600">
                  <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
               </div>
             </div>
           ))}
         </div>
       )}


       <div className="flex justify-between items-center mb-4 mt-8">
         <h2 className="text-sm font-bold text-gray-400 uppercase tracking-wider px-1">Owner-Ready Shift Summaries</h2>
       </div>

       {loading ? (
          <div className="flex justify-center py-4">
             <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-gray-900"></div>
          </div>
       ) : summaries.length === 0 ? (
          <div className="text-center py-4 text-gray-500 text-sm">No recent shift summaries.</div>
       ) : (
         <div className="space-y-3">
           {summaries.map((summary: any) => (
             <div key={summary.id} className="bg-white/65 backdrop-blur-[30px] p-4 rounded-xl shadow-sm border border-white/40">
               <p className="font-semibold text-gray-900 font-outfit text-sm">{new Date(summary.created_at).toLocaleDateString()}</p>
               <p className="text-sm text-gray-700 mt-2">{summary.summary_text}</p>
               {summary.metrics && summary.metrics.completed_tasks && (
                  <div className="mt-2 text-xs text-gray-500">
                    Tasks Completed: {summary.metrics.completed_tasks}
                  </div>
               )}
             </div>
           ))}
         </div>
       )}

       {showAddForm ? (
         <form onSubmit={handleAddStaff} className="mt-4 bg-white rounded-2xl p-4 border border-gray-200 shadow-sm">
            <h3 className="font-semibold text-gray-900 mb-3">Add New Staff</h3>

            <div className="space-y-3">
              <div>
                <label className="text-xs text-gray-500 block mb-1">Name</label>
                <input
                  type="text"
                  value={name}
                  onChange={e => setName(e.target.value)}
                  className="w-full bg-gray-50 border border-gray-200 rounded-lg p-2 text-sm outline-none focus:border-[#0066FF]"
                  required
                />
              </div>

              <div>
                <label className="text-xs text-gray-500 block mb-1">Phone Number</label>
                <input
                  type="tel"
                  value={phone}
                  onChange={e => setPhone(e.target.value)}
                  className="w-full bg-gray-50 border border-gray-200 rounded-lg p-2 text-sm outline-none focus:border-[#0066FF]"
                  required
                />
              </div>

              <div>
                <label className="text-xs text-gray-500 block mb-1">Role</label>
                <select
                  value={role}
                  onChange={e => setRole(e.target.value)}
                  className="w-full bg-gray-50 border border-gray-200 rounded-lg p-2 text-sm outline-none focus:border-[#0066FF]"
                >
                  <option value="Cashier">Cashier</option>
                  <option value="Manager">Manager</option>
                  <option value="Driver">Driver</option>
                  <option value="Assistant">Assistant</option>
                </select>
              </div>
            </div>

            <div className="flex gap-2 mt-4">
               <button
                 type="button"
                 onClick={() => setShowAddForm(false)}
                 className="flex-1 py-2 bg-gray-100 text-gray-600 rounded-lg text-sm font-semibold"
               >
                 Cancel
               </button>
               <button
                 type="submit"
                 disabled={isSubmitting}
                 className="flex-1 py-2 bg-[#0071E3] text-white rounded-lg text-sm font-semibold disabled:opacity-50"
               >
                 {isSubmitting ? 'Adding...' : 'Add Staff'}
               </button>
            </div>
         </form>
       ) : (
         <button
           onClick={() => setShowAddForm(true)}
           className="mt-4 w-full py-3 border-2 border-dashed border-gray-300 rounded-xl text-gray-500 text-sm font-semibold hover:border-[#0066FF] hover:text-[#0071E3] transition-colors flex items-center justify-center gap-2"
         >
           <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6v6m0 0v6m0-6h6m-6 0H6" /></svg>
           Add Staff Member
         </button>
       )}
    </div>
  );
}
