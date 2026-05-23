"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [showAdvanced, setShowAdvanced] = useState<boolean>(false);
  const [swarmActivity, setSwarmActivity] = useState<any[]>([]);
  const [todaysSales, setTodaysSales] = useState<number>(0);
  const [activeCustomers, setActiveCustomers] = useState<number>(0);
  const [pendingOrders, setPendingOrders] = useState<number>(0);
  const [bannerDismissed, setBannerDismissed] = useState<boolean>(true);
  const [teamInvitesSent, setTeamInvitesSent] = useState<number>(0);
  const [showMilestoneAlert, setShowMilestoneAlert] = useState<boolean>(true);

  // Growth Loop: Referral Modal State
  const [showReferralModal, setShowReferralModal] = useState<boolean>(false);
  const [showPromoModal, setShowPromoModal] = useState<boolean>(false);
  const [copied, setCopied] = useState<boolean>(false);

  // Growth Loop: Milestone Modal State
  const [showMilestoneModal, setShowMilestoneModal] = useState<boolean>(false);
  const [currentMilestone, setCurrentMilestone] = useState<any>(null);
  // Growth Loop: Automated Review Request State
  const [isReviewGenerating, setIsReviewGenerating] = useState<boolean>(false);
  const [reviewCampaignSent, setReviewCampaignSent] = useState<boolean>(false);
  const [reviewEmailsSent, setReviewEmailsSent] = useState<number>(0);
  const [showReviewRequestCard, setShowReviewRequestCard] = useState<boolean>(true);

  const handleApproveReviewRequest = () => {
    setIsReviewGenerating(true);
    setTimeout(() => {
      setIsReviewGenerating(false);
      setReviewCampaignSent(true);
      setReviewEmailsSent(3);
      setTimeout(() => {
        setShowReviewRequestCard(false);
      }, 3000);
    }, 2000);
  };


  useEffect(() => {
    async function checkMilestones() {
      if (localStorage.getItem('10th_order_milestone_shown') === 'true') return;
      try {
        const res = await fetch('/api/v1/growth/milestones/check');
        const data = await res.json();
        if (data && data.milestones) {
          const orderMilestone = data.milestones.find((m: any) => m.id === "3" && m.reached);
          if (orderMilestone) {
            setCurrentMilestone(orderMilestone);
            setShowMilestoneModal(true);
            localStorage.setItem('10th_order_milestone_shown', 'true');
          }
        }
      } catch (e) {
        console.error("Failed to check milestones", e);
      }
    }
    checkMilestones();

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

            const [salesRes, metricsRes, invitesRes] = await Promise.all([
                fetch('/api/v1/dashboard/sales', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${token}` },
                    body: JSON.stringify({ tenant_id: tenant })
                }),
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

            if (salesRes.ok) {
                const salesData = await salesRes.json();
                setTodaysSales(salesData.total_sales);
            }

            if (metricsRes.ok) {
                const metricsData = await metricsRes.json();
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

  const handleGenerateReviewCampaign = async () => {
    setIsReviewGenerating(true);
    try {
      const response = await fetch("/api/v1/growth/campaign/send", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          name: "Automated Review Request",
          subject: "How did we do? Leave a review!",
          body: "We hope you loved your recent purchase. Please leave a review.",
          target_segment: "recent_buyers_no_review"
        })
      });
      const data = await response.json();
      setReviewEmailsSent(data.emails_sent);
      setReviewCampaignSent(true);
    } catch (e) {
      console.error("Failed to generate review campaign", e);
    } finally {
      setIsReviewGenerating(false);
    }
  };


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
             <button onClick={() => setShowReferralModal(true)} className="ml-4 px-4 py-2 bg-indigo-600 text-white rounded-lg text-sm font-semibold">Referrals</button>
         <div className="flex items-center gap-3">
             <Link href="/seasonal-promo" className="px-4 py-2 bg-indigo-600 text-white rounded-md text-sm font-medium hover:bg-indigo-700 transition-colors">
               Seasonal Promos ✨
             </Link>
             <Link href="/agents" className="px-4 py-2 bg-indigo-50 text-indigo-700 rounded-md text-sm font-medium hover:bg-indigo-100 transition-colors border border-indigo-100 shadow-sm flex items-center gap-1">
               <span>🤖</span> AI Departments
             </Link>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">

         {/* Success Milestone Alert */}
         {showMilestoneAlert && (
           <div className="bg-gradient-to-r from-indigo-500 to-purple-600 rounded-2xl p-6 shadow-lg text-white flex flex-col md:flex-row items-center justify-between gap-6 relative overflow-hidden">
             <div className="absolute top-0 right-0 w-32 h-32 bg-white/10 rounded-bl-full -z-0"></div>
             <div className="relative z-10 flex-1">
               <div className="flex items-center gap-3 mb-2">
                 <span className="text-2xl">🎉</span>
                 <h2 className="text-xl font-bold font-outfit">Milestone Unlocked: 10th Order!</h2>
               </div>
               <p className="text-indigo-100 text-sm mb-4 leading-relaxed">
                 You just hit a major milestone. Success breeds success! Share your achievement to inspire others and grow your audience. Plus, when a friend opens a store, you get $50 in premium credit!
               </p>
               <div className="flex flex-wrap gap-3">
                 <a
                   href={`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just hit my 10th order on my @OneHumanCorp store! 🚀 Start your own business journey and get priority setup: ' + referralLink)}`}
                   target="_blank"
                   rel="noopener noreferrer"
                   className="flex items-center gap-2 bg-black/20 hover:bg-black/30 backdrop-blur-sm text-white px-4 py-2 rounded-xl font-semibold text-sm transition-all border border-white/20"
                 >
                   <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                   Share on X
                 </a>
                 <a
                   href={`https://wa.me/?text=${encodeURIComponent('I just hit my 10th order on my OneHumanCorp store! 🚀 Start your own business journey and get priority setup: ' + referralLink)}`}
                   target="_blank"
                   rel="noopener noreferrer"
                   className="flex items-center gap-2 bg-[#25D366]/90 hover:bg-[#25D366] text-white px-4 py-2 rounded-xl font-semibold text-sm transition-all border border-white/20 shadow-sm"
                 >
                   <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                   WhatsApp
                 </a>
               </div>
             </div>
             <button
               onClick={() => setShowMilestoneAlert(false)}
               className="relative z-10 p-2 text-white/70 hover:text-white rounded-full hover:bg-white/10 transition-colors shrink-0 self-start"
               aria-label="Dismiss"
             >
               <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
             </button>
           </div>
         )}

         {/* Action Required (Approvals) */}
         {(approvals.length > 0 || showReviewRequestCard) && (
            <section className="mb-6">
                <div className="flex items-center justify-between mb-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Action Required</h2>
                    <div className="flex items-center gap-2">
                        <span className="text-sm font-medium" style={{ color: '#86868B' }}>Advanced Settings</span>
                        <button
                            onClick={() => setShowAdvanced(!showAdvanced)}
                            className={`w-10 h-6 rounded-full transition-colors duration-300 relative ${showAdvanced ? 'bg-blue-500' : 'bg-gray-300'}`}
                        >
                            <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 ${showAdvanced ? 'translate-x-4' : 'translate-x-0'}`}></span>
                        </button>
                    </div>
                </div>
                <div className="flex flex-col gap-4">
                    {/* Hardcoded Automated Review Request Card based on mockup */}
                    {showReviewRequestCard && (
                        <div className="p-5 shadow-md flex flex-col gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                            <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                                <div className="flex items-center gap-3">
                                    <div className="w-10 h-10 rounded-full flex items-center justify-center text-xl shrink-0" style={{ background: '#eef2ff', color: '#4f46e5' }}>
                                        🤝
                                    </div>
                                    <div>
                                        <h3 className="font-semibold text-lg font-outfit text-gray-900">
                                            CustomerSuccess Department
                                        </h3>
                                        <p className="text-gray-600 font-inter text-sm">3 customers haven't reviewed their orders. Request reviews?</p>
                                    </div>
                                </div>

                                {isReviewGenerating ? (
                                    <div className="flex items-center gap-2 text-sm text-blue-600 font-medium whitespace-nowrap px-4 py-2 bg-blue-50 rounded-lg">
                                        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600"></div>
                                        AI generating personalized review requests...
                                    </div>
                                ) : reviewCampaignSent ? (
                                    <div className="flex items-center gap-2 text-sm text-green-600 font-medium whitespace-nowrap px-4 py-2 bg-green-50 rounded-lg">
                                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                                        Sent to 3 customers!
                                    </div>
                                ) : (
                                    <div className="flex gap-2 shrink-0">
                                        <button
                                            onClick={() => setShowReviewRequestCard(false)}
                                            className="px-4 py-2 font-medium text-red-600 bg-red-50 hover:bg-red-100 transition-colors"
                                            style={{ borderRadius: '8px' }}
                                        >
                                            Reject
                                        </button>
                                        <button
                                            onClick={handleApproveReviewRequest}
                                            className="px-6 py-2 font-medium text-white bg-blue-600 hover:bg-blue-700 transition-colors shadow-sm"
                                            style={{ borderRadius: '8px' }}
                                        >
                                            Approve
                                        </button>
                                    </div>
                                )}
                            </div>
                        </div>
                    )}

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
                                            {approval.department === 'CustomerSuccess' ? '🤝' : approval.department === 'Operations' ? '⚙️' : '🤖'}
                                        </div>
                                        <div>
                                            <h3 className="font-semibold text-lg font-outfit text-gray-900">
                                                {approval.department} Department
                                            </h3>
                                            <p className="text-gray-600 font-inter text-sm">{plainMessage}</p>
                                        </div>
                                    </div>
                                    <div className="flex gap-2">
                                        <button
                                            onClick={() => handleApprove(approval.id, false)}
                                            className="px-4 py-2 font-medium text-red-600 bg-red-50 hover:bg-red-100 transition-colors"
                                            style={{ borderRadius: '8px' }}
                                        >
                                            Reject
                                        </button>
                                        <button
                                            onClick={() => handleApprove(approval.id, true)}
                                            className="px-6 py-2 font-medium text-white bg-blue-600 hover:bg-blue-700 transition-colors shadow-sm"
                                            style={{ borderRadius: '8px' }}
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
                             const tenant = localStorage.getItem('tenant') || 'DEFAULT';
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

         {/* Top Action Banner (Stripe Setup) */}
         <section className="mb-6">
             <div className="p-4 rounded-xl shadow-sm flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 bg-red-50 text-red-900 border border-red-100">
                 <div className="flex items-center gap-4">
                     <div>
                         <h3 className="font-bold text-sm sm:text-lg font-outfit text-red-800">1 Action Required: Connect Stripe to accept payments.</h3>
                     </div>
                 </div>
                 <button className="px-5 py-2 bg-red-600 text-white font-bold rounded-lg shadow-sm hover:bg-red-700 transition-colors whitespace-nowrap">
                     Complete Stripe Setup
                 </button>
             </div>
         </section>

         <>
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

         {/* Growth & Promotions Generator Card */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Growth & Promotions</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-purple-50 rounded-full border border-purple-100">
                        <span className="text-xs font-medium text-purple-600">AI Powered</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-2xl flex flex-col md:flex-row gap-6 items-center mb-8" style={{ background: 'linear-gradient(to right, #ffffff, #fdfbfb)', border: '1px solid rgba(0,0,0,0.05)' }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Boost Sales with AI Campaigns</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">Let our AI generate high-converting promotional messages for your next holiday or flash sale. Ready to send via SMS or WhatsApp.</p>
                    <button
                        onClick={() => setShowPromoModal(true)}
                        className="px-6 py-3 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-700 hover:to-indigo-700 text-white font-semibold rounded-xl shadow-sm transition-all flex items-center gap-2"
                    >
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                        Generate Promotion
                    </button>
                </div>
                <div className="hidden md:flex w-32 h-32 items-center justify-center relative">
                   {/* Decorative AI visual */}
                   <div className="absolute inset-0 bg-gradient-to-br from-purple-400 to-indigo-500 rounded-full opacity-20 blur-xl animate-pulse"></div>
                   <div className="relative w-20 h-20 bg-gradient-to-tr from-purple-500 to-indigo-500 rounded-2xl rotate-3 shadow-lg flex items-center justify-center text-white">
                        <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" /></svg>
                   </div>
                </div>
            </div>
         </section>

         {/* Growth Loop: Embeddable Storefront Widget */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Embed Your Store</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-green-50 rounded-full border border-green-100">
                        <span className="text-xs font-medium text-green-600">New Growth Loop</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-2xl flex flex-col md:flex-row gap-6 items-center" style={{ background: 'rgba(255, 255, 255, 0.03)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.08)', borderColor: 'rgba(0,0,0,0.05)', backgroundColor: '#ffffff' }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Sell Anywhere</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">Embed your OHC storefront on your existing website, blog, or partner pages. This powerful widget allows customers to buy directly from you anywhere on the web.</p>
                    <div className="bg-gray-50 border border-gray-200 rounded-lg p-3 relative">
                        <pre className="text-xs text-gray-600 overflow-x-auto font-mono whitespace-pre-wrap">
{`<div id="ohc-embed-root"></div>
<script src="https://ohc.store/embed.js" data-store="${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}"></script>
<div style="text-align: center; margin-top: 8px; font-family: sans-serif; font-size: 11px;">
  <a href="https://ohc.store/join?ref=${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}" target="_blank" style="color: #646b78; text-decoration: none;">Powered by <b>OHC</b></a>
</div>`}
                        </pre>
                        <button
                            onClick={() => {
                                const code = `<div id="ohc-embed-root"></div>\n<script src="https://ohc.store/embed.js" data-store="${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}"></script>\n<div style="text-align: center; margin-top: 8px; font-family: sans-serif; font-size: 11px;">\n  <a href="https://ohc.store/join?ref=${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}" target="_blank" style="color: #646b78; text-decoration: none;">Powered by <b>OHC</b></a>\n</div>`;
                                navigator.clipboard.writeText(code);
                                alert('Copied embed code to clipboard!');
                            }}
                            className="absolute top-2 right-2 bg-white text-gray-700 border border-gray-200 px-3 py-1 rounded-md text-xs font-semibold hover:bg-gray-50 transition-colors"
                        >
                            Copy Code
                        </button>
                    </div>
                </div>
                <div className="w-full md:w-1/3 bg-gray-50 rounded-xl p-4 flex flex-col items-center justify-center border border-gray-100 min-h-[160px]">
                    <div className="text-4xl mb-3">💻</div>
                    <span className="text-sm font-medium text-gray-600 text-center">Preview: Connect your brand everywhere</span>
                </div>
            </div>
         </section>


         {/* Growth Loop: Automated AI Review Requests */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: "#1D1D1F" }}>Automated Review Requests</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-green-50 rounded-full border border-green-100">
                        <span className="text-xs font-medium text-green-600">New Growth Loop</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-2xl flex flex-col md:flex-row gap-6 items-center" style={{ background: "rgba(255, 255, 255, 0.03)", backdropFilter: "blur(20px) saturate(200%)", border: "1px solid rgba(255, 255, 255, 0.08)", borderColor: "rgba(0,0,0,0.05)", backgroundColor: "#ffffff" }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Boost Your Trust Score</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">
                        You have 12 recent orders without reviews. Let AI generate and send personalized follow-up emails to collect more 5-star reviews and increase your conversion rate.
                    </p>
                    {reviewCampaignSent ? (
                        <div className="inline-flex items-center gap-2 text-green-600 font-semibold bg-green-50 px-4 py-2 rounded-lg border border-green-100">
                            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                            Campaign sent to {reviewEmailsSent} customers!
                        </div>
                    ) : (
                        <button onClick={handleGenerateReviewCampaign} disabled={isReviewGenerating} className={`px-5 py-2.5 rounded-xl font-semibold text-sm shadow-sm transition-all ${isReviewGenerating ? "bg-gray-200 text-gray-500 cursor-not-allowed" : "bg-gradient-to-r from-blue-600 to-indigo-600 text-white hover:shadow-md hover:-translate-y-0.5 active:translate-y-0"}`}>
                            {isReviewGenerating ? "Generating..." : "✨ Send AI Review Requests"}
                        </button>
                    )}
                </div>
            </div>
         </section>


         {/* Growth Loop: Referral Program Snapshot */}
         <section>
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Referral Program</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-indigo-50 rounded-full border border-indigo-100">
                        <span className="text-xs font-medium text-indigo-600">Active</span>
                    </div>
                </div>
                <button
                    onClick={() => setShowReferralModal(true)}
                    className="flex items-center gap-2 px-5 py-2.5 bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0 transition-all font-inter text-sm"
                >
                    <span>🎁 Invite a Business & Earn $50</span>
                </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
                <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between">
                    <div className="text-sm font-medium mb-1 text-indigo-800">Team Invites Sent</div>
                    <div className="text-3xl font-bold font-outfit text-indigo-900">{teamInvitesSent}</div>
                </div>

                <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between">
                    <div className="text-sm font-medium mb-1 text-indigo-800">Active Referrals</div>
                    <div className="text-3xl font-bold font-outfit text-indigo-900">4</div>
                </div>

                <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between">
                    <div className="text-sm font-medium mb-1 text-indigo-800">Revenue from Referrals</div>
                    <div className="text-3xl font-bold font-outfit text-indigo-900">$120.00</div>
                </div>

                <div className="ohc-hybrid-panel p-5 shadow-sm flex flex-col justify-between">
                    <div className="text-sm font-medium mb-1 text-indigo-800">Pending Rewards</div>
                    <div className="text-3xl font-bold font-outfit text-indigo-900">$24.00</div>
                </div>
            </div>
         </section>

         </>
{/* Swarm Observability / Team Activity Panel */}
         <section>
            <div className="flex items-center justify-between mb-4">
                <WithTooltip id="team-activity-tooltip" defaultText="Monitor the real-time actions and tasks being performed by your AI workforce."><h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Team Activity</h2></WithTooltip>
                <WithTooltip id="swarm-online-tooltip" defaultText="Your AI workforce is active."><div className="flex items-center gap-2 px-3 py-1 bg-green-50 rounded-full border border-green-100">
                    <div className="w-2 h-2 rounded-full animate-pulse" style={{ backgroundColor: '#34C759' }}></div>
                    <span className="text-xs font-medium" style={{ color: '#34C759' }}>Swarm Online</span>
                </div></WithTooltip>
            </div>

            <div className="ohc-hybrid-panel shadow-sm overflow-hidden">
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

      {/* Milestone Modal */}
      {showMilestoneModal && currentMilestone && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter">
            {/* Background embellishment */}
            <div className="absolute top-0 right-0 w-32 h-32 bg-yellow-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-yellow-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-yellow-600">
                🎉
              </div>
              <button
                onClick={() => setShowMilestoneModal(false)}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">{currentMilestone.title}</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              {currentMilestone.description}
            </p>

            <div className="space-y-4">
              <div className="relative py-3">
                <div className="absolute inset-0 flex items-center"><div className="w-full border-t border-gray-200"></div></div>
                <div className="relative flex justify-center"><span className="bg-white px-2 text-xs text-gray-500 uppercase font-semibold tracking-wide">Share Your Success</span></div>
              </div>

              {/* Social Share Buttons */}
              <div className="grid grid-cols-2 gap-3">
                <a
                  href={`https://wa.me/?text=${encodeURIComponent('Just hit my 10th order on my new store! Built entirely with AI on @OneHumanCorp. Launch yours and get $50 credit: https://ohc.store/join?ref=acme-corp')}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                  WhatsApp
                </a>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent('Just hit my 10th order on my new store! Built entirely with AI on @OneHumanCorp. Launch yours and get $50 credit: https://ohc.store/join?ref=acme-corp')}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-black text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                  X (Twitter)
                </a>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Promo Modal */}
      {showPromoModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter">
            <div className="absolute top-0 right-0 w-32 h-32 bg-purple-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-purple-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-purple-600">
                ✨
              </div>
              <button
                onClick={() => {
                  setShowPromoModal(false);
                  setCopied(false);
                }}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">AI Promotion Generator</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              We generated a custom message tailored for your store. Send it to your customers to drive sales!
            </p>

            <div className="space-y-4">
              <div>
                <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">Generated Message</label>
                <textarea
                  readOnly
                  rows={4}
                  value="Hey there! 🎉 We're running an exclusive flash sale this weekend. Grab your favorite items before they're gone! Shop now and get 10% off: https://ohc.store/shop/acme-corp"
                  className="w-full bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none resize-none"
                />
              </div>

              <button
                onClick={() => {
                  navigator.clipboard.writeText("Hey there! 🎉 We're running an exclusive flash sale this weekend. Grab your favorite items before they're gone! Shop now and get 10% off: https://ohc.store/shop/acme-corp");
                  setCopied(true);
                  setTimeout(() => setCopied(false), 2000);
                }}
                className={`w-full py-3 rounded-xl text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
              >
                {copied ? 'Message Copied!' : 'Copy Message'}
              </button>

              <div className="relative py-3">
                <div className="absolute inset-0 flex items-center"><div className="w-full border-t border-gray-200"></div></div>
                <div className="relative flex justify-center"><span className="bg-white px-2 text-xs text-gray-500 uppercase font-semibold tracking-wide">Or Share Directly</span></div>
              </div>

              <a
                href={`https://wa.me/?text=${encodeURIComponent("Hey there! 🎉 We're running an exclusive flash sale this weekend. Grab your favorite items before they're gone! Shop now and get 10% off: https://ohc.store/shop/acme-corp")}`}
                target="_blank"
                rel="noopener noreferrer"
                className="w-full flex items-center justify-center gap-2 bg-[#25D366] text-white py-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                Send via WhatsApp
              </a>
            </div>
          </div>
        </div>
      )}

      {/* Referral Modal */}
      {showReferralModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter">
            {/* Background embellishment */}
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-indigo-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-indigo-600">
                🚀
              </div>
              <button
                onClick={() => {
                  setShowReferralModal(false);
                  setCopied(false);
                }}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Help a Business Grow!</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              When your friends launch their storefront on OHC, they get priority AI setup, and you earn <WithTooltip id="credit-tooltip" defaultText="Earn credits to use on premium tools when you refer a friend."><strong className="text-gray-900">$50 credit</strong></WithTooltip> toward your premium tools.
            </p>

            <div className="space-y-4">
              {/* Copy Link Section */}
              <div>
                <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">Your Unique Link</label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    readOnly
                    value="https://ohc.store/join?ref=acme-corp"
                    className="flex-1 bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none"
                  />
                  <button
                    onClick={() => {
                      navigator.clipboard.writeText("https://ohc.store/join?ref=acme-corp");
                      setCopied(true);
                      setTimeout(() => setCopied(false), 2000);
                    }}
                    className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                  >
                    {copied ? 'Copied!' : 'Copy'}
                  </button>
                </div>
              </div>

              <div className="relative py-3">
                <div className="absolute inset-0 flex items-center"><div className="w-full border-t border-gray-200"></div></div>
                <div className="relative flex justify-center"><span className="bg-white px-2 text-xs text-gray-500 uppercase font-semibold tracking-wide">Or Share Via</span></div>
              </div>

              {/* Social Share Buttons */}
              <div className="grid grid-cols-2 gap-3">
                <a
                  href={`https://wa.me/?text=${encodeURIComponent('Launch your business online instantly with OHC! Use my invite link: https://ohc.store/join?ref=acme-corp')}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                  WhatsApp
                </a>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent('Launch your business online instantly with OHC! Use my invite link: https://ohc.store/join?ref=acme-corp')}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-black text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                  X (Twitter)
                </a>
              </div>
            </div>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
