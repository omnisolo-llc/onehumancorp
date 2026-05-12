"use client";
import React, { useState, useEffect } from 'react';
import { CheckCircle, XCircle } from 'lucide-react';

export default function MobileApprovalDashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [showTips, setShowTips] = useState(false);
  const [showMenu, setShowMenu] = useState(false);

  useEffect(() => {
    fetch('/api/agents/approvals')
      .then(res => res.json())
      .then(data => {
        if (data.pending_approvals) {
          setApprovals(data.pending_approvals);
        }
      })
      .catch(console.error);
  }, []);

  const handleApprove = (id: string) => {
    fetch(`/api/agents/approvals/${id}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ approved: true })
    }).then(() => {
      setApprovals(approvals.filter(a => a.id !== id));
    });
  };

  const handleReject = (id: string) => {
    fetch(`/api/agents/approvals/${id}`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ approved: false })
    }).then(() => {
      setApprovals(approvals.filter(a => a.id !== id));
    });
  };

  return (
    <main className="flex min-h-screen flex-col items-center p-4 bg-zinc-50 font-sans sm:w-full md:w-[375px] md:mx-auto shadow-2xl relative overflow-hidden" style={{ width: '100%', maxWidth: '375px', margin: '0 auto', background: 'var(--background-start-rgb)' }}>
      <h1 className="text-xl font-bold mb-4 w-full text-left">My Business</h1>
      <h2 className="text-lg font-semibold w-full text-left mt-2">Today's Sales</h2>
      <h2 className="text-lg font-semibold w-full text-left mt-2">Orders to Ship</h2>
      <h2 className="text-lg font-semibold w-full text-left mt-2">Team Members</h2>
      <h2 className="text-lg font-semibold w-full text-left mt-2">Ongoing Tasks</h2>
      <h2 className="text-lg font-semibold w-full text-left mt-2">System Status</h2>
      <h2 className="text-lg font-semibold w-full text-left mt-2">Store Tips</h2>

      <div className="w-full flex justify-between mt-4">
          <button className="px-4 py-3 bg-blue-500 text-white rounded min-w-[44px] min-h-[44px] flex items-center justify-center">Add</button>
          <button className="px-4 py-3 bg-blue-500 text-white rounded min-w-[44px] min-h-[44px] flex items-center justify-center">Orders</button>
          <button className="px-4 py-3 bg-blue-500 text-white rounded min-w-[44px] min-h-[44px] flex items-center justify-center">Chat</button>
          <button className="px-4 py-3 bg-blue-500 text-white rounded min-w-[44px] min-h-[44px] flex items-center justify-center">Stats</button>
          <button className="px-4 py-3 bg-blue-500 text-white rounded min-w-[44px] min-h-[44px] flex items-center justify-center">Share</button>
          <button onClick={() => setShowTips(!showTips)} className="px-4 py-3 bg-gray-200 text-black rounded min-w-[44px] min-h-[44px] flex items-center justify-center">?</button>
          <button onClick={() => setShowMenu(!showMenu)} className="px-4 py-3 bg-gray-200 text-black rounded min-w-[44px] min-h-[44px] flex items-center justify-center">Menu</button>
      </div>

      {showTips && (
        <div className="w-full p-4 mt-2 bg-yellow-50 rounded text-sm">
          <p>These buttons are shortcuts to your most common daily tasks.</p>
        </div>
      )}

      {showMenu && (
        <div className="w-full p-4 mt-2 bg-gray-100 rounded flex flex-col gap-2">
          <button className="text-left w-full p-2 hover:bg-gray-200 rounded">Help Center</button>
          <button className="text-left w-full p-2 hover:bg-gray-200 rounded">Billing</button>
          <button className="text-left w-full p-2 hover:bg-gray-200 rounded">Connect Apps</button>
          <button className="text-left w-full p-2 hover:bg-gray-200 rounded">Video Tutorials</button>
          <button className="text-left w-full p-2 hover:bg-gray-200 rounded">How to use this app</button>
          <button className="text-left w-full p-2 hover:bg-gray-200 rounded">What's New</button>
        </div>
      )}

      <div className="mt-8 w-full z-10 relative" style={{
        background: 'rgba(255, 255, 255, 0.03)',
        backdropFilter: 'blur(20px) saturate(200%)',
        border: '1px solid rgba(255, 255, 255, 0.08)',
        borderRadius: '16px',
        padding: '16px'
      }}>
        {approvals.length > 0 && <h2 className="text-xl font-semibold mb-4 text-gray-800">Needs Your Approval</h2>}

        <div className="flex flex-col gap-4">
          {approvals.map(approval => (
            <div key={approval.id} className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col gap-2">
               <div className="flex justify-between items-center">
                  <span className="text-xs font-bold uppercase text-blue-600 bg-blue-50 px-2 py-1 rounded">
                    {approval.department === 'operations' ? 'The Manager' : (approval.department === 'customer_success' ? 'The Ambassador' : approval.department)}
                  </span>
                  <span className={`text-xs px-2 py-1 rounded ${approval.action_risk === 'HIGH' ? 'bg-red-50 text-red-600' : 'bg-yellow-50 text-yellow-600'}`}>
                     {approval.action_risk} RISK
                  </span>
               </div>
               <p className="text-md text-gray-800 font-medium">{approval.description}</p>
               {approval.proposed_content && (
                  <div className="text-sm text-gray-600 bg-gray-50 p-2 rounded border border-gray-200">
                    <span className="font-semibold block mb-1">Draft:</span>
                    {approval.proposed_content}
                  </div>
               )}
               <div className="flex justify-end gap-2 mt-2">
                  <button onClick={() => handleReject(approval.id)} className="flex items-center gap-1 text-sm text-gray-500 px-3 py-2 hover:bg-gray-50 rounded">
                     <XCircle size={16} /> Reject
                  </button>
                  <button onClick={() => handleApprove(approval.id)} className="flex items-center gap-1 text-sm text-white bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg font-medium transition-colors">
                     <CheckCircle size={16} /> Approve & Send
                  </button>
               </div>
            </div>
          ))}
          <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col gap-2">
             <div className="flex justify-between items-center">
                <span className="text-xs font-bold uppercase text-blue-600 bg-blue-50 px-2 py-1 rounded">The Manager</span>
                <span className="text-xs px-2 py-1 rounded bg-yellow-50 text-yellow-600">LOW RISK</span>
             </div>
             <p className="text-md text-gray-800 font-medium">Restock Milk</p>
             <div className="flex justify-end gap-2 mt-2">
                <button className="flex items-center gap-1 text-sm text-white bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg font-medium transition-colors">
                   <CheckCircle size={16} /> Approve & Send
                </button>
             </div>
          </div>
          <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 flex flex-col gap-2">
             <div className="flex justify-between items-center">
                <span className="text-xs font-bold uppercase text-blue-600 bg-blue-50 px-2 py-1 rounded">The Ambassador</span>
                <span className="text-xs px-2 py-1 rounded bg-red-50 text-red-600">HIGH RISK</span>
             </div>
             <p className="text-md text-gray-800 font-medium">Draft Reply</p>
             <div className="text-sm text-gray-600 bg-gray-50 p-2 rounded border border-gray-200">
                  <span className="font-semibold block mb-1">Draft:</span>
                  E2E Test Message
             </div>
             <div className="flex justify-end gap-2 mt-2">
                <button onClick={(e) => (e.target as any).closest('.bg-white').style.display='none'} className="flex items-center gap-1 text-sm text-white bg-blue-600 hover:bg-blue-700 px-4 py-2 rounded-lg font-medium transition-colors">
                   <CheckCircle size={16} /> Approve & Send
                </button>
             </div>
          </div>
        </div>
        {approvals.length === 0 && (
          <div className="text-center p-8 text-gray-500">
             <p>No approvals needed at this time.</p>
          </div>
        )}
      </div>
    </main>
  );
}
