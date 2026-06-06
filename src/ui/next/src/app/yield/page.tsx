'use client';
import { useState, useEffect } from 'react';
import Link from 'next/link';

export default function YieldManagementPage() {
  const [opportunities, setOpportunities] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);

  const fetchOpportunities = () => {
    const tenantId = localStorage.getItem('tenant_id') || 'e2e-tenant';
    fetch(`/api/v1/yield?tenant_id=${tenantId}`)
      .then(res => res.json())
      .then(data => {
        if (data && Array.isArray(data.opportunities)) {
          setOpportunities(data.opportunities);
        } else {
          setOpportunities([]);
        }
        setLoading(false);
      })
      .catch(err => {
        console.error(err);
        setLoading(false);
      });
  };

  useEffect(() => {
    fetchOpportunities();
  }, []);

  const approveOpportunity = async (id: string) => {
    const tenantId = localStorage.getItem('tenant_id') || 'e2e-tenant';
    try {
      const res = await fetch('/api/v1/yield/approve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ tenant_id: tenantId, opportunity_id: id })
      });
      if (res.ok) {
        fetchOpportunities(); // Refresh the list
      } else {
        console.error('Failed to approve opportunity');
      }
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <div className="min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <div className="flex items-center gap-4">
          <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Yield Opportunities</h1>
        </div>
      </header>

      <main className="p-6 max-w-lg mx-auto">
        <section className="bg-white rounded-[16px] shadow-sm p-6" style={{ border: '1px solid rgba(0,0,0,0.05)' }}>
          <h2 className="text-xl font-semibold font-outfit mb-4 text-gray-900">Yield Management</h2>
          <p className="text-sm text-gray-500 mb-6">AI has identified opportunities to fill your empty slots.</p>

          <div className="space-y-4">
            {loading ? (
              <div className="text-sm text-gray-500 text-center py-4">Checking for opportunities...</div>
            ) : opportunities.length === 0 ? (
              <div className="text-sm text-gray-500 text-center py-4 border border-gray-100 rounded-lg">No opportunities at this time.</div>
            ) : opportunities.map(opp => (
              <div key={opp.id} className="p-4 border border-blue-100 bg-blue-50/50 rounded-lg shadow-sm">
                <div className="flex justify-between items-start mb-2">
                  <h3 className="font-semibold text-gray-900">Fill Empty Slots</h3>
                  <span className="px-2 py-1 bg-green-100 text-green-800 text-xs font-bold rounded">Opportunity</span>
                </div>
                <p className="text-sm text-gray-800 mb-4">
                  You have <span className="font-bold">{opp.empty_slots} empty slots</span> on <span className="font-semibold">{opp.target_date}</span>.
                  Send a <span className="font-bold">{opp.recommended_discount_percent}% discount</span> offer to your <span className="italic">{opp.target_audience}</span>?
                </p>
                <button
                  onClick={() => approveOpportunity(opp.id)}
                  className="w-full py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-medium transition-colors"
                >
                  Approve Offer
                </button>
              </div>
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}
