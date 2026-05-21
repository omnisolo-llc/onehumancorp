"use client";

import { useState, useEffect } from "react";
import Link from "next/link";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [showAdvanced, setShowAdvanced] = useState<boolean>(false);
  const [swarmActivity, setSwarmActivity] = useState<any[]>([]);
  const [todaysSales, setTodaysSales] = useState<number>(0);
  const [activeCustomers, setActiveCustomers] = useState<number>(0);
  const [pendingOrders, setPendingOrders] = useState<number>(0);
  const [orderDrafts, setOrderDrafts] = useState<any[]>([]);
  const [weeklyInsights, setWeeklyInsights] = useState<any[]>([]);
  const [bannerDismissed, setBannerDismissed] = useState<boolean>(true);

  // Growth Loop: Referral Modal State
  const [showReferralModal, setShowReferralModal] = useState<boolean>(false);
  const [copied, setCopied] = useState<boolean>(false);

  useEffect(() => {
    setBannerDismissed(localStorage.getItem('milestone_banner_dismissed') === 'true');
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

    const connectSwarmMesh = () => {
        try {
            const ws = new WebSocket(`ws://${window.location.host}/api/v1/mesh/ws?topic=system`);
            ws.onmessage = (event) => {
                try {
                    const payload = JSON.parse(event.data);
                    setSwarmActivity(prev => [{
                        id: Math.random().toString(),
                        agent: payload.agent_id || "Swarm Agent",
                        action: payload.action || "Working on task...",
                        time: new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit', second:'2-digit'})
                    }, ...prev].slice(0, 5));
                } catch(e) {}
            };
            return ws;
        } catch(e) {
            console.error("Mesh websocket failed", e);
            return null;
        }
    };

    const ws = connectSwarmMesh();

    const fetchMetrics = async () => {
        try {
            const tenant = localStorage.getItem('tenant') || 'system';
            const snapshotRes = await fetch(`/api/v1/dashboard/snapshot?organization_id=${tenant}`);
            if (snapshotRes.ok) {
                const data = await snapshotRes.json();
                setOrderDrafts(data.order_drafts || []);
                setWeeklyInsights(data.weekly_insights || []);
                if (data.orders) setPendingOrders(data.orders.length);
            }

            const token = localStorage.getItem('token') || 'test-token';
            const salesRes = await fetch('/api/v1/dashboard/sales', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${token}` },
                body: JSON.stringify({ tenant_id: tenant })
            });
            if (salesRes.ok) {
                const salesData = await salesRes.json();
                setTodaysSales(salesData.total_sales);
            }
        } catch (e) {
            console.error("Failed to fetch dashboard metrics", e);
        }
    };

    fetchMetrics();

    return () => {
        if (ws) ws.close();
    };
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
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>

      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.03)', backdropFilter: 'blur(20px) saturate(200%)', borderBottom: '1px solid rgba(255, 255, 255, 0.08)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Dashboard</h1>
         <div className="flex items-center gap-3">
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">AC</div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">

         {/* Weekly Insights */}
         {weeklyInsights.length > 0 && (
             <section className="animate-fade-in">
                 <div className="flex items-center gap-2 mb-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Weekly Insights</h2>
                    <span className="bg-blue-100 text-blue-600 text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-widest">AI Driven</span>
                 </div>
                 <div className="grid grid-cols-1 gap-4">
                     {weeklyInsights.map(insight => (
                         <div key={insight.id} className="ohc-hybrid-panel p-6 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-6" style={{ background: 'rgba(255, 255, 255, 0.7)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.5)', color: '#1D1D1F' }}>
                             <div className="flex items-start gap-4">
                                 <div className="w-12 h-12 bg-blue-50 rounded-2xl flex items-center justify-center text-2xl shrink-0">💡</div>
                                 <div>
                                     <h3 className="font-bold text-lg font-outfit mb-1">{insight.title}</h3>
                                     <p className="text-sm text-gray-500 leading-relaxed max-w-xl">{insight.description}</p>
                                 </div>
                             </div>
                             <button
                                onClick={() => setWeeklyInsights(weeklyInsights.filter(i => i.id !== insight.id))}
                                className="px-6 py-3 bg-blue-600 text-white font-bold rounded-xl shadow-lg shadow-blue-200 hover:bg-blue-700 hover:-translate-y-0.5 transition-all active:translate-y-0 whitespace-nowrap"
                             >
                                 {insight.action_label}
                             </button>
                         </div>
                     ))}
                 </div>
             </section>
         )}

         {/* Action Required */}
         <section>
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Action Required</h2>
                <div className="flex items-center gap-2">
                    <span className="text-sm font-medium" style={{ color: '#86868B' }}>Advanced Settings</span>
                    <button onClick={() => setShowAdvanced(!showAdvanced)} className={`w-10 h-6 rounded-full transition-colors duration-300 relative ${showAdvanced ? 'bg-blue-500' : 'bg-gray-300'}`}>
                        <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ${showAdvanced ? 'translate-x-4' : 'translate-x-0'}`}></span>
                    </button>
                </div>
            </div>
            <div className="flex flex-col gap-4">
                {orderDrafts.map(draft => (
                    <div key={draft.id} className="p-5 flex flex-col gap-4 ohc-hybrid-panel animate-fade-in" style={{ background: 'rgba(255, 255, 255, 0.6)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                        <div className="flex items-center justify-between">
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 rounded-full flex items-center justify-center text-xl" style={{ background: draft.source_channel === 'WhatsApp' ? '#dcfce7' : '#fef2f2', color: draft.source_channel === 'WhatsApp' ? '#166534' : '#991b1b' }}>
                                    {draft.source_channel === 'WhatsApp' ? '💬' : '📸'}
                                </div>
                                <div>
                                    <div className="flex items-center gap-2">
                                        <h3 className="font-bold text-gray-900 font-outfit">New {draft.source_channel} Order Draft</h3>
                                        <span className="text-[10px] font-black bg-gray-900 text-white px-1.5 py-0.5 rounded uppercase">AI Draft</span>
                                    </div>
                                    <p className="text-gray-600 text-sm italic">"{draft.raw_message}"</p>
                                </div>
                            </div>
                            <div className="flex gap-2">
                                <button onClick={() => setOrderDrafts(orderDrafts.filter(d => d.id !== draft.id))} className="px-4 py-2 font-bold text-gray-400 hover:text-gray-600">Reject</button>
                                <button onClick={() => setOrderDrafts(orderDrafts.filter(d => d.id !== draft.id))} className="px-6 py-2 font-bold text-white bg-gray-900 hover:bg-black transition-all rounded-xl shadow-sm">Approve ${(draft.suggested_amount_cents / 100).toFixed(2)}</button>
                            </div>
                        </div>
                    </div>
                ))}

                {approvals.map(approval => (
                    <div key={approval.id} className="p-5 shadow-md flex flex-col gap-4 ohc-hybrid-panel" style={{ background: 'rgba(255, 255, 255, 0.03)', border: '1px solid rgba(255, 255, 255, 0.08)', borderRadius: '16px' }}>
                        <div className="flex items-center justify-between">
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 rounded-full flex items-center justify-center text-xl" style={{ background: '#eef2ff', color: '#4f46e5' }}>
                                    {approval.department === 'CustomerSuccess' ? '🤝' : approval.department === 'Operations' ? '⚙️' : '🤖'}
                                </div>
                                <div>
                                    <h3 className="font-semibold text-lg font-outfit text-gray-900">{approval.department} Department</h3>
                                    <p className="text-gray-600 font-inter text-sm">{approval.description}</p>
                                </div>
                            </div>
                            <div className="flex gap-2">
                                <button onClick={() => handleApprove(approval.id, false)} className="px-4 py-2 font-medium text-red-600 bg-red-50 hover:bg-red-100 rounded-lg">Reject</button>
                                <button onClick={() => handleApprove(approval.id, true)} className="px-6 py-2 font-medium text-white bg-blue-600 hover:bg-blue-700 rounded-lg shadow-sm">Approve</button>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
         </section>

         {/* Business Snapshot */}
         <section>
            <div className="flex items-center justify-between mb-4">
                <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Business Snapshot</h2>
                <Link href="/catalog" className="text-sm font-bold text-blue-600 hover:underline flex items-center gap-1">
                    Manage Catalog
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" /></svg>
                </Link>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="ohc-hybrid-panel p-6 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.75)', color: '#1D1D1F', border: '1px solid rgba(255, 255, 255, 0.5)' }}>
                    <div className="text-xs font-bold uppercase tracking-widest mb-1 text-gray-400">Today's Sales</div>
                    <div className="text-4xl font-bold font-outfit">${todaysSales.toFixed(2)}</div>
                </div>
                <div className="ohc-hybrid-panel p-6 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.75)', color: '#1D1D1F', border: '1px solid rgba(255, 255, 255, 0.5)' }}>
                    <div className="text-xs font-bold uppercase tracking-widest mb-1 text-gray-400">Active Customers</div>
                    <div className="text-4xl font-bold font-outfit">{activeCustomers}</div>
                </div>
                <div className="ohc-hybrid-panel p-6 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.75)', color: '#1D1D1F', border: '1px solid rgba(255, 255, 255, 0.5)' }}>
                    <div className="text-xs font-bold uppercase tracking-widest mb-1 text-gray-400">Pending Orders</div>
                    <div className="text-4xl font-bold font-outfit">{pendingOrders}</div>
                </div>
            </div>
         </section>

         {/* Growth Loop: Referral Program */}
         <section>
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Referral Program</h2>
                    <div className="px-3 py-1 bg-indigo-50 rounded-full border border-indigo-100 text-xs font-medium text-indigo-600 uppercase tracking-widest">Active</div>
                </div>
                <button onClick={() => setShowReferralModal(true)} className="flex items-center gap-2 px-5 py-2.5 bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:-translate-y-0.5 transition-all text-sm"><span>🎁 Invite a Business & Earn $50</span></button>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="ohc-hybrid-panel p-5 flex flex-col justify-between glass-light">
                    <div className="text-xs font-bold uppercase tracking-widest mb-1 text-indigo-400">Active Referrals</div>
                    <div className="text-3xl font-bold font-outfit text-indigo-900">4</div>
                </div>
                <div className="ohc-hybrid-panel p-5 flex flex-col justify-between glass-light">
                    <div className="text-xs font-bold uppercase tracking-widest mb-1 text-indigo-400">Revenue</div>
                    <div className="text-3xl font-bold font-outfit text-indigo-900">$120.00</div>
                </div>
                <div className="ohc-hybrid-panel p-5 flex flex-col justify-between glass-light">
                    <div className="text-xs font-bold uppercase tracking-widest mb-1 text-indigo-400">Pending Rewards</div>
                    <div className="text-3xl font-bold font-outfit text-indigo-900">$24.00</div>
                </div>
            </div>
         </section>

      </main>

      {/* Referral Modal */}
      {showReferralModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-sm">
          <div className="bg-white w-full max-w-md rounded-3xl p-8 shadow-2xl relative overflow-hidden animate-fade-in">
            <div className="flex justify-between items-start mb-4">
              <div className="w-14 h-14 bg-indigo-100 rounded-2xl flex items-center justify-center text-3xl shadow-inner text-indigo-600">🚀</div>
              <button onClick={() => { setShowReferralModal(false); setCopied(false); }} className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"><svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg></button>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Help a Business Grow!</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">When your friends launch their storefront on OHC, they get priority AI setup, and you earn <strong className="text-gray-900">$50 credit</strong> toward your premium tools.</p>
            <div className="space-y-4">
              <div>
                <label className="block text-xs font-bold text-gray-400 uppercase tracking-widest mb-2">Your Unique Link</label>
                <div className="flex gap-2">
                  <input type="text" readOnly value="https://ohc.store/join?ref=acme-corp" className="flex-1 bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-600 outline-none focus:ring-2 focus:ring-indigo-500 transition-all"/>
                  <button onClick={() => { navigator.clipboard.writeText("https://ohc.store/join?ref=acme-corp"); setCopied(true); setTimeout(() => setCopied(false), 2000); }} className={`px-6 py-3 rounded-xl text-sm font-bold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}>{copied ? 'Copied!' : 'Copy'}</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap'); .font-inter { font-family: 'Inter', sans-serif; } .font-outfit { font-family: 'Outfit', sans-serif; } .animate-fade-in { animation: fadeIn 0.4s ease-out forwards; } @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }`}} />
    </div>
  );
}
