"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [hasPro, setHasPro] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
        setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const [showAdvanced, setShowAdvanced] = useState<boolean>(false);
  const [swarmActivity, setSwarmActivity] = useState<any[]>([]);
  const [todaysSales, setTodaysSales] = useState<number>(0);
  const [activeCustomers, setActiveCustomers] = useState<number>(0);
  const [pendingOrders, setPendingOrders] = useState<number>(0);
  const [bannerDismissed, setBannerDismissed] = useState<boolean>(true);
  const [teamInvitesSent, setTeamInvitesSent] = useState<number>(0);
  const [productCount, setProductCount] = useState<number>(10);
  const [morningBriefingDismissed, setMorningBriefingDismissed] = useState<boolean>(false);
  const businessName = typeof localStorage !== 'undefined' ? localStorage.getItem('business_name') || 'Maya' : 'Maya';

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

    // Connect to Teammate Mesh WebSocket for real-time swarm activity

    const connectSwarmMesh = () => {
        try {
            const ws = new WebSocket(`ws://${window.location.host}/api/v1/mesh/connect?channel=system`);

            ws.onmessage = (event) => {
                try {
                    const binaryString = atob(event.data);
                    const bytes = new Uint8Array(binaryString.length);
                    for (let i = 0; i < binaryString.length; i++) {
                        bytes[i] = binaryString.charCodeAt(i);
                    }
                    let payload: any = {};
                    try {
                       payload = JSON.parse(new TextDecoder().decode(bytes));
                    } catch(e) {
                       // Since we don't have protobufjs in the legacy Next.js app, perform basic string extraction
                       const str = new TextDecoder("utf-8").decode(bytes);
                       // Standard protobuf strings usually have length prefixes, finding plain text action descriptions
                       // Example actions are standard sentences like "Draft email for review"
                       const stringMatches = str.match(/[a-zA-Z0-9\s_\-\.\:\,]{8,}/g);
                       if (stringMatches && stringMatches.length > 0) {
                           // Filter out base64 padding or noise
                           payload = { action: stringMatches.filter(s => s.indexOf('spiffe') === -1 && s.trim().length > 5).join(' ') || "Processing mesh task..." };
                       } else {
                           return; // Unprocessable binary
                       }
                    }
                    setSwarmActivity(prev => [{
                        id: Math.random().toString(),
                        agent: payload.agent_id || "Swarm Agent",
                        action: payload.action || "Working on task...",
                        time: new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit', second:'2-digit'})
                    }, ...prev].slice(0, 5)); // Keep last 5
                } catch(e) {
                   // Ignore parsing errors
                }
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
            const token = localStorage.getItem('token') || 'test-token';
            const tenant = localStorage.getItem('tenant') || 'e2e-tenant';

            const [metricsRes, invitesRes] = await Promise.all([
                fetch('/api/v1/dashboard/metrics', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${token}` },
                    body: JSON.stringify({ tenant_id: tenant })
                }),
                fetch(`/api/v1/growth/team-invites/metrics?team_id=${tenant}`, {
                    method: 'GET',
                    headers: { 'Authorization': `Bearer ${token}` }
                })
            ]);


            if (metricsRes.ok) {
                const metricsData = await metricsRes.json();
                setTodaysSales(metricsData.total_sales);
                setActiveCustomers(metricsData.active_customers);
                setPendingOrders(metricsData.pending_orders);
            }

            if (invitesRes.ok) {
                const invitesData = await invitesRes.json();
                setTeamInvitesSent(invitesData.total_invites);
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
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Dashboard</h1>
         <nav className="flex items-center gap-3">
             <Link href="/calendar" className="px-4 py-2 bg-purple-100 text-purple-800 rounded-md text-sm font-medium hover:bg-purple-200 transition-colors border border-purple-200 shadow-sm">
               Calendar 📅
             </Link>
             <Link href="/inbox" className="px-4 py-2 bg-blue-100 text-blue-800 rounded-md text-sm font-medium hover:bg-blue-200 transition-colors border border-blue-200 shadow-sm">
               Inbox
             </Link>
             <Link href="/review-campaigns" className="px-4 py-2 bg-yellow-100 text-yellow-800 rounded-md text-sm font-medium hover:bg-yellow-200 transition-colors border border-yellow-200 shadow-sm">
               Review Campaigns ⭐️
             </Link>
             <Link href="/share-cards" className="px-4 py-2 bg-pink-100 text-pink-700 rounded-md text-sm font-medium hover:bg-pink-200 transition-colors border border-pink-200 shadow-sm">
               Social Cards 🎴
             </Link>
             <Link href="/seasonal-promo" className="px-4 py-2 bg-indigo-600 text-white rounded-md text-sm font-medium hover:bg-indigo-700 transition-colors">
               Seasonal Promos ✨
             </Link>
             <Link href="/scribe-mission-track" className="px-4 py-2 bg-indigo-50 text-indigo-700 rounded-md text-sm font-medium hover:bg-indigo-100 transition-colors border border-indigo-100 shadow-sm flex items-center gap-1">Scribe Track</Link>
             <Link href="/agents" className="px-4 py-2 bg-indigo-50 text-indigo-700 rounded-md text-sm font-medium hover:bg-indigo-100 transition-colors border border-indigo-100 shadow-sm flex items-center gap-1">
               <span>🤖</span> AI Departments
             </Link>
             <Link href="/kairos" id="kairos-nav-link" className="px-4 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 transition-colors shadow-sm flex items-center gap-1">
               <span>⚡️</span> KAIROS
             </Link>
             <Link href="/plan" className="px-4 py-2 bg-gray-200 text-gray-800 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors shadow-sm">
               My Plan
             </Link>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </nav>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">

         {/* Morning Briefing */}
         {!morningBriefingDismissed && (
           <section className="mb-6 animate-fade-in">
             <div className="p-6 shadow-md rounded-2xl border transition-all" style={{ background: 'rgba(255, 255, 255, 0.75)', backdropFilter: 'blur(30px) saturate(210%)', borderColor: 'rgba(52, 199, 89, 0.3)' }}>
               <div className="flex items-center gap-3 mb-2">
                 <div className="text-2xl">🌅</div>
                 <h2 className="text-xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>Morning Briefing</h2>
               </div>
               <p className="text-gray-600 font-inter text-sm leading-relaxed mb-5">
                 Good morning {businessName}! Your storefront is live and looking great. Your next step to success is to add your first product or service so customers can start buying.
               </p>
               <div className="flex gap-4">
                 <button onClick={() => setMorningBriefingDismissed(true)} className="px-6 py-3 font-semibold text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-xl transition-colors shadow-sm">Dismiss</button>
                 <Link href="/builder" className="px-6 py-3 font-bold text-white rounded-xl shadow-md transition-transform hover:scale-[1.02] active:scale-[0.98] flex items-center justify-center gap-2" style={{ background: 'linear-gradient(135deg, #34C759 0%, #2eb350 100%)' }}>
                   Add your first product
                 </Link>
               </div>
             </div>
           </section>
         )}

         {/* Action Required (Approvals) */}
         {(approvals.length > 0) && (
            <section className="mb-6">
                <div className="flex items-center justify-between mb-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Action Required</h2>
                    <div className="flex items-center gap-2">
                        <span className="text-sm font-medium" style={{ color: '#86868B' }}>Advanced Settings</span>
                        <button
                            onClick={() => setShowAdvanced(!showAdvanced)}
                            className={`w-10 h-6 rounded-full transition-colors duration-300 relative ${showAdvanced ? 'bg-[#34C759]' : 'bg-gray-300'}`}
                        >
                            <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ${showAdvanced ? 'translate-x-4' : 'translate-x-0'}`}></span>
                        </button>
                    </div>
                </div>
                <div className="flex flex-col gap-4">
                    {approvals.map(approval => {
                        // Extract plain english message and payload
                        let plainMessage = approval.description;
                        let payload = "";
                        const payloadIdx = approval.description.indexOf(" | Payload: ");
                        if (payloadIdx !== -1) {
                            plainMessage = approval.description.substring(0, payloadIdx);
                            payload = approval.description.substring(payloadIdx + " | Payload: ".length);
                        }

                        return (
                            <div key={approval.id} className="p-5 shadow-md flex flex-col gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                                <div className="flex items-center justify-between">
                                    <div className="flex items-center gap-3">
                                        <div className="w-10 h-10 rounded-full flex items-center justify-center text-xl" style={{ background: '#eef2ff', color: '#4f46e5' }}>
                                            {approval.department === 'customer_success' || approval.department === 'CustomerSuccess' ? '🤝' : approval.department === 'operations' || approval.department === 'Operations' ? '⚙️' : '🤖'}
                                        </div>
                                        <div>
                                            <h3 className="font-semibold text-lg font-outfit text-gray-900 capitalize">
                                                {approval.department === 'customer_success' || approval.department === 'CustomerSuccess' ? 'CustomerSuccess' : approval.department} Department
                                            </h3>
                                            <p className="text-gray-600 font-inter text-sm">{plainMessage}</p>
                                        </div>
                                    </div>
                                    <div className="flex gap-2">
                                        <button
                                            onClick={() => handleApprove(approval.id, false)}
                                            className="px-4 py-2 font-medium transition-colors hover:opacity-80"
                                            style={{ borderRadius: '8px', color: '#FF3B30', background: 'rgba(255, 59, 48, 0.1)' }}
                                        >
                                            Reject
                                        </button>
                                        <button
                                            onClick={() => handleApprove(approval.id, true)}
                                            className="px-6 py-2 font-medium text-white transition-colors shadow-sm hover:opacity-90"
                                            style={{ borderRadius: '8px', backgroundColor: '#0066FF' }}
                                        >
                                            Approve
                                        </button>
                                    </div>
                                </div>
                                {showAdvanced && payload && (
                                    <div className="mt-2 p-3 bg-gray-900 text-gray-100 rounded-lg text-xs font-mono overflow-x-auto">
                                        <div className="text-gray-400 mb-1">Technical Payload:</div>
                                        <pre>{payload}</pre>
                                    </div>
                                )}
                            </div>
                        );
                    })}
                </div>
            </section>
         )}

         {/* Milestone Viral Share Loop Banner */}
         {activeCustomers > 0 && !bannerDismissed && (
             <section className="mb-6">
                 <div className="p-4 rounded-xl shadow-sm flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4" style={{ background: 'linear-gradient(135deg, #f6d365 0%, #fda085 100%)', color: '#fff' }}>
                     <div className="flex items-center gap-4">
                         <span className="text-3xl">🎉</span>
                         <div>
                             <h3 className="font-bold text-lg font-outfit" style={{ color: '#fff' }}>Milestone Unlocked: Your First Customers!</h3>
                             <p className="text-sm opacity-90 font-inter" style={{ color: '#fff' }}>You've reached {activeCustomers} active customers. Share your store's success to earn a free month of Pro!</p>
                         </div>
                     </div>
                     <button
                         onClick={() => {
                             const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
                             const text = encodeURIComponent(`I just reached ${activeCustomers} customers on my store! Start your own business today with One Human Corp: ohc://join?ref=${tenant}`);
                             window.open(`https://twitter.com/intent/tweet?text=${text}`, '_blank');

                             localStorage.setItem('milestone_banner_dismissed', 'true');
                             setBannerDismissed(true);
                             fetch('/api/v1/growth/referrals/click', {
                                 method: 'POST',
                                 headers: { 'Content-Type': 'application/json' },
                                 body: JSON.stringify({ id: tenant })
                             }).catch(console.error);

                             alert('Thank you for sharing! Your 1 month of Pro will be applied shortly.');
                         }}
                         className="px-5 py-2 bg-white text-orange-500 font-bold rounded-lg shadow-sm hover:bg-orange-50 transition-colors whitespace-nowrap"
                     >
                         Share & Claim Reward
                     </button>
                 </div>
             </section>
         )}
         {/* Business Snapshot */}
         <section>
            <h2 className="text-xl font-semibold mb-4 font-outfit" style={{ color: '#1D1D1F' }}>Business Snapshot</h2>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">

                {/* Metric Card */}
                <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between">
                    <div className="text-sm font-medium mb-1" style={{ color: '#86868B' }}>Today's Sales</div>
                    <div className="text-3xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>${todaysSales.toFixed(2)}</div>
                </div>

                <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between">
                    <div className="text-sm font-medium mb-1" style={{ color: '#86868B' }}>Active Customers</div>
                    <div className="text-3xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>{activeCustomers}</div>
                </div>

                <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between">
                    <div className="text-sm font-medium mb-1" style={{ color: '#86868B' }}>Pending Orders</div>
                    <div className="text-3xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>{pendingOrders}</div>
                </div>

            </div>
         </section>

         {/* Products & Monetization Snapshot */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Products</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-green-50 rounded-full border border-green-100">
                        <span className="text-xs font-medium text-green-600">{productCount} / 10 Products Used</span>
                    </div>
                </div>
                <button
                    onClick={() => {
                        setProductCount(prev => prev + 1);
                    }}
                    className="flex items-center gap-2 px-5 py-2.5 bg-gray-900 text-white font-semibold rounded-xl shadow-md hover:bg-black transition-all font-inter text-sm"
                >
                    <span>+ Add Product</span>
                </button>
            </div>
         </section>

         {/* Swarm Observability / Team Activity Panel */}
         <section>
            <div className="flex items-center justify-between mb-4">
                <WithTooltip id="team-activity-tooltip" defaultText="Monitor the real-time actions and tasks being performed by your AI workforce."><h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Team Activity</h2></WithTooltip>
                <WithTooltip id="swarm-online-tooltip" defaultText="Your AI workforce is active."><div className="flex items-center gap-2 px-3 py-1 bg-green-50 rounded-full border border-green-100">
                    <div className="w-2 h-2 rounded-full animate-pulse" style={{ backgroundColor: '#34C759' }}></div>
                    <span className="text-xs font-medium" style={{ color: '#34C759' }}>Swarm Online</span>
                </div></WithTooltip>
            </div>

            <div id="agent-activity-feed" className="ohc-hybrid-panel shadow-sm overflow-hidden">
                {swarmActivity.length === 0 ? (
                    <div className="p-8 text-center">
                        <div className="inline-block w-8 h-8 rounded-full border-2 border-gray-200 border-t-blue-500 animate-spin mb-3"></div>
                        <p className="text-sm" style={{ color: '#86868B' }}>Waiting for team activity...</p>
                    </div>
                ) : (
                    <div className="flex flex-col">
                        {swarmActivity.map((activity, index) => (
                            <div key={activity.id} className="flex items-center justify-between p-4 border-b last:border-b-0 transition-all duration-500 ease-in-out hover:bg-white/40" style={{ borderBottomColor: 'rgba(0,0,0,0.05)' }}>
                                <div className="flex items-center gap-4">
                                    <div className="w-10 h-10 rounded-full flex items-center justify-center text-xl shadow-sm" style={{ background: '#ffffff', border: '1px solid rgba(0,0,0,0.05)' }}>
                                        🤖
                                    </div>
                                    <div>
                                        <p className="text-sm font-semibold" style={{ color: '#1D1D1F' }}>{activity.agent}</p>
                                        <p className="text-sm" style={{ color: '#86868B' }}>{activity.action}</p>
                                    </div>
                                </div>
                                <div className="flex flex-col items-end gap-1">
                                    <span className="text-xs font-medium" style={{ color: '#86868B' }}>{activity.time}</span>
                                    {activity.status === 'success' && <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: '#34C759' }}></span>}
                                    {activity.status === 'warning' && <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: '#FF9500' }}></span>}
                                    {activity.status === 'info' && <span className="w-1.5 h-1.5 rounded-full" style={{ backgroundColor: '#0066FF' }}></span>}
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>
         </section>

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
