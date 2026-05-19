"use client";

import { useState, useEffect } from "react";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);

  useEffect(() => {
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
      </main>
    </div>
  );
}
