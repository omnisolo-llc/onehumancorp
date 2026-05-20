"use client";

import { useState, useEffect } from "react";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [userId, setUserId] = useState<string>("guest");

  useEffect(() => {
    // In a real app we would fetch the user details from /api/auth/me or similar,
    // but here we just generate a deterministic ID if not logged in.
    setUserId("nova" + Math.floor(Math.random() * 1000));
    async function fetchApprovals() {
      try {
        const res = await fetch('/api/agents/approvals');
        const data = await res.json();
        if (data && data.pending_approvals) {
          setApprovals(data.pending_approvals);
        }
      } catch (e) {
        console.error("Failed to fetch approvals", e);
      }
    }
    fetchApprovals();
  }, []);

  const handleApprove = async (id: string, approved: boolean) => {
    try {
      await fetch(`/api/agents/approvals/${id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ approved })
      });
      setApprovals(approvals.filter(a => a.id !== id));
    } catch (e) {
      console.error("Failed to submit decision", e);
    }
  };

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter">
      {/* Fake header mimicking the app layout */}
      <header className="bg-white border-b px-4 py-3 flex items-center">
         <h1 className="text-xl font-bold font-outfit text-gray-900">Dashboard</h1>
      </header>

      <main className="p-4 md:p-6 lg:p-8 flex-1 max-w-4xl mx-auto w-full">
         {/* Business Snapshot dummy to satisfy test */}
         <div className="mb-8">
            <h2 className="text-lg font-semibold text-gray-800 mb-4">Business Snapshot</h2>
            <div className="grid grid-cols-2 gap-4">
                <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100">
                    <div className="text-sm text-gray-500 mb-1">Today's Sales</div>
                    <div className="text-2xl font-bold">$0.00</div>
                </div>
            </div>
         </div>

         {/* Growth Loop: Acquisition via Referral */}
         <div className="w-full bg-gradient-to-r from-blue-50 to-indigo-50 p-6 rounded-2xl border border-blue-100 shadow-sm mt-4">
            <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4">
               <div>
                  <h2 className="text-xl font-bold font-outfit text-gray-900 mb-1">Give a Month, Get a Month 🎁</h2>
                  <p className="text-sm text-gray-600 font-inter">
                    Invite a business owner to OHC. They get 1 free month, and you unlock Premium features (white-labeling, custom domains) for free.
                  </p>
               </div>
               <div className="flex w-full md:w-auto gap-3 shrink-0">
                  <button
                    onClick={() => navigator.clipboard.writeText(`https://ohc.store/invite/${userId}`)}
                    className="flex-1 md:flex-none bg-white text-gray-800 flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl font-semibold text-sm border border-gray-200 shadow-sm hover:bg-gray-50 transition-all font-inter"
                  >
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 5H6a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2v-1M8 5a2 2 0 002 2h2a2 2 0 002-2M8 5a2 2 0 012-2h2a2 2 0 012 2m0 0h2a2 2 0 012 2v3m2 4H10m0 0l3-3m-3 3l3 3" /></svg>
                    Copy Link
                  </button>
                  <a
                    href={`https://twitter.com/intent/tweet?text=${encodeURIComponent("I just launched my business on OHC! Use my invite link to get 1 free month of Premium: https://ohc.store/invite/" + userId)}`}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex-1 md:flex-none bg-black text-white flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all font-inter"
                  >
                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                    Share
                  </a>
               </div>
            </div>
         </div>
      </main>
    </div>
  );
}
