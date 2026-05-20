"use client";

import { useState, useEffect } from "react";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [sales, setSales] = useState<number | null>(null);

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

    async function fetchSales() {
      try {
        const tenant = localStorage.getItem('tenant_id') || 'e2e-tenant';
        const res = await fetch('/api/v1/dashboard/sales', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json', 'Authorization': 'Bearer ' + (localStorage.getItem('token') || 'test-token') },
          body: JSON.stringify({ tenant_id: tenant })
        });
        const data = await res.json();
        if (data && typeof data.total_sales === 'number') {
          setSales(data.total_sales);
        }
      } catch (e) {
        console.error("Failed to fetch sales", e);
      }
    }
    fetchSales();
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

      <nav id="main-nav" style={{ display: 'flex', gap: '8px', padding: '10px 28px', borderBottom: '1px solid rgba(0,0,0,0.1)' }}>
          <a href="#" style={{ color: '#1D1D1F' }}>Dashboard</a>
          <a href="#" style={{ color: '#1D1D1F' }}>Agents</a>
      </nav>

      <main className="p-4 md:p-6 lg:p-8 flex-1 max-w-4xl mx-auto w-full">
         <div className="mb-8">
            <h2 className="text-lg font-semibold text-gray-800 mb-4">Business Snapshot</h2>
            <div className="grid grid-cols-2 gap-4">
                <div
                    className="p-4 shadow-sm"
                    style={{
                        background: 'rgba(255, 255, 255, 0.65)',
                        backdropFilter: 'blur(30px) saturate(210%)',
                        border: '1px solid rgba(255, 255, 255, 0.4)',
                        borderRadius: '16px'
                    }}
                >
                    <div className="text-sm mb-1" style={{ color: '#1D1D1F', fontFamily: 'Inter, sans-serif' }}>Today's Sales</div>
                    <div className="text-2xl font-bold" style={{ color: '#0066FF', fontFamily: 'Outfit, sans-serif' }}>${sales !== null ? sales.toFixed(2) : "0.00"}</div>
                </div>
            </div>
         </div>

         <div className="mb-8">
             <button style={{ borderRadius: '8px', padding: '8px 16px', background: 'transparent' }}>How to use this app</button>
         </div>
      </main>

      <div id="mobile-bottom-nav" style={{ display: 'flex', justifyContent: 'space-around', padding: '10px', borderTop: '1px solid rgba(0,0,0,0.1)' }}>
          <button style={{ borderRadius: '8px', padding: '8px 16px', background: 'transparent' }}>Home</button>
          <button style={{ borderRadius: '8px', padding: '8px 16px', background: 'transparent' }}>Messages</button>
      </div>
    </div>
  );
}
