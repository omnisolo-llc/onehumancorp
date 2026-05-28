"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function Dashboard() {
  const [approvals, setApprovals] = useState<any[]>([]);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [isSendingCampaign, setIsSendingCampaign] = useState(false);
  const [campaignSuccess, setCampaignSuccess] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
        setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const [showAdvanced, setShowAdvanced] = useState<boolean>(false);
  const [showMilestoneBanner, setShowMilestoneBanner] = useState<boolean>(true);
  const [swarmActivity, setSwarmActivity] = useState<any[]>([]);
  const [todaysSales, setTodaysSales] = useState<number>(0);
  const [activeCustomers, setActiveCustomers] = useState<number>(0);
  const [pendingOrders, setPendingOrders] = useState<number>(0);
  const [bannerDismissed, setBannerDismissed] = useState<boolean>(true);
  const [teamInvitesSent, setTeamInvitesSent] = useState<number>(0);
  const [productCount, setProductCount] = useState<number>(10);
  const [morningBriefingDismissed, setMorningBriefingDismissed] = useState<boolean>(false);
  const businessName = typeof localStorage !== 'undefined' ? localStorage.getItem('business_name') || 'Maya' : 'Maya';

  // Growth Loop: Trial Extension State
  const [trialDaysLeft, setTrialDaysLeft] = useState<number>(14);
  const [twitterConnected, setTwitterConnected] = useState<boolean>(false);
  const [reviewLeft, setReviewLeft] = useState<boolean>(false);
  const [productAdded, setProductAdded] = useState<boolean>(false);

  // Growth Loop: Referral Modal State
  const [showReferralModal, setShowReferralModal] = useState<boolean>(false);
  const [showPaywallModal, setShowPaywallModal] = useState<boolean>(false);
  const [showEmbedModal, setShowEmbedModal] = useState<boolean>(false);
  const [embedCopied, setEmbedCopied] = useState<boolean>(false);
  const [showPromoModal, setShowPromoModal] = useState<boolean>(false);
  const [copied, setCopied] = useState<boolean>(false);
  const [referralLink, setReferralLink] = useState<string>("");

  const [isGeneratingReferral, setIsGeneratingReferral] = useState<boolean>(false);

  const [isGeneratingPromo, setIsGeneratingPromo] = useState<boolean>(false);
  const [promoMessage, setPromoMessage] = useState<string>("Hey there! 🎉 We're running an exclusive flash sale this weekend. Grab your favorite items before they're gone! Shop now and get 10% off: https://ohc.store/shop/acme-corp");

  // Growth Loop: Automated Review Request State
  const [showReviewModal, setShowReviewModal] = useState<boolean>(false);
  const [isGeneratingReview, setIsGeneratingReview] = useState<boolean>(false);
  const [reviewMessage, setReviewMessage] = useState<string>("");
  const [reviewSent, setReviewSent] = useState<boolean>(false);

  // Growth Loop: Abandoned Cart Recovery State
  const [showCartModal, setShowCartModal] = useState<boolean>(false);
  const [isGeneratingCartCampaign, setIsGeneratingCartCampaign] = useState<boolean>(false);
  const [cartCampaignMessage, setCartCampaignMessage] = useState<string>("");
  const [cartCampaignSent, setCartCampaignSent] = useState<boolean>(false);

  // Growth Loop: VIP Customer Referral Campaign State
  const [showCustomerReferralModal, setShowCustomerReferralModal] = useState<boolean>(false);
  const [isGeneratingCustomerReferral, setIsGeneratingCustomerReferral] = useState<boolean>(false);
  const [customerReferralMessage, setCustomerReferralMessage] = useState<string>("");
  const [customerReferralSent, setCustomerReferralSent] = useState<boolean>(false);

  useEffect(() => {
    setReferralLink(`https://ohc.store/join?ref=${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}`);
  }, []);

  const openReferralModal = async () => {
    setIsGeneratingReferral(true);
    try {
      const response = await fetch("/api/v1/growth/referrals/generate", {
        method: "POST"
      });
      if (response.ok) {
        const data = await response.json();
        if (data.referral_link) {
          setReferralLink(data.referral_link);
        }
      } else {
        // Fallback to local storage tenant if API fails or no auth
        const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
        setReferralLink(`https://ohc.store/join?ref=${tenant}`);
      }
    } catch (e) {
      console.error("Failed to generate dynamic referral link", e);
      const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
      setReferralLink(`https://ohc.store/join?ref=${tenant}`);
    } finally {
      setIsGeneratingReferral(false);
      setShowReferralModal(true);
    }
  };

  // Growth Loop: Upgrade Modal State
  const [showUpgradeModal, setShowUpgradeModal] = useState<boolean>(false);

  // Growth Loop: Milestone Modal State
  const [showMilestoneModal, setShowMilestoneModal] = useState<boolean>(false);
  const [currentMilestone, setCurrentMilestone] = useState<any>(null);

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

  const handleSendCampaign = () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }

    setIsSendingCampaign(false);
    setCampaignSuccess(true);
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ohc://join?ref=' + tenant)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    alert('Thank you for sharing! Your 7-day Pro trial has been activated.');
    handleSendCampaign();
  };

  const handleApprove = async (id: string, approved: boolean) => {
    // Check if this is the automated review request approval
    const approval = approvals.find(a => a.id === id);

    // Safety & Maintainability: We use payload structured data instead of string description matching
    let isReviewRequest = false;
    let payloadObj: any = null;

    if (approval && approval.payload) {
        if (typeof approval.payload === 'string') {
            try {
                payloadObj = JSON.parse(approval.payload);
            } catch (e) {
                // Ignore parsing errors for simple strings
            }
        } else {
            payloadObj = approval.payload;
        }

        if (payloadObj && payloadObj.feature_type === 'automated_review_request') {
            isReviewRequest = true;
        }
    }

    if (approved && isReviewRequest) {
        setApprovals(approvals.filter(a => a.id !== id));

        // Ensure it's removed from backend as well
        try {
          await fetch(`/api/agents/approvals/${id}`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ approved: true })
          });
        } catch (e) {
            console.error("Failed to submit decision", e);
        }

        // Use dynamic payload data to generate the review if present, otherwise fallback
        const orderId = payloadObj?.target_order_id || '8922';
        const customerName = payloadObj?.target_customer_name || 'Sarah';
        const productName = payloadObj?.target_product_name || 'Signature Coffee Blend';

        // Open the review modal as per the new growth loop flow
        setShowReviewModal(true);
        setIsGeneratingReview(true);
        try {
            const response = await fetch('/api/v1/growth/campaign/generate-review', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    order_id: orderId,
                    customer_name: customerName,
                    product_name: productName
                })
            });
            if (response.ok) {
                const data = await response.json();
                setReviewMessage(data.message);
            } else {
                setReviewMessage(`Hi ${customerName},\n\nWe noticed you recently received your ${productName} and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/${orderId}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`);
            }
        } catch (e) {
            console.error("Failed to generate review", e);
            setReviewMessage(`Hi ${customerName},\n\nWe noticed you recently received your ${productName} and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/${orderId}\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`);
        } finally {
            setIsGeneratingReview(false);
            setReviewSent(false);
        }

        return;
    }

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

         {/* Growth Loop: Frictionless Soft Paywall Upgrade CTA */}
         {!hasPro && (
           <section className="mb-6 animate-fade-in">
             <div className="p-6 shadow-md rounded-2xl border transition-all flex flex-col sm:flex-row items-center justify-between gap-4" style={{ background: 'linear-gradient(135deg, #fdfbfb 0%, #ebedee 100%)', borderColor: 'rgba(0,0,0,0.05)' }}>
               <div>
                   <div className="flex items-center gap-3 mb-2">
                     <div className="text-2xl">🚀</div>
                     <h2 className="text-xl font-bold font-outfit" style={{ color: '#1D1D1F' }}>Ready to scale?</h2>
                   </div>
                   <p className="text-gray-600 font-inter text-sm leading-relaxed max-w-lg">
                     Upgrade to Pro for unlimited agents, advanced analytics, and custom domains. Grow your business faster and without limits.
                   </p>
               </div>
               <Link href="/pricing" className="px-6 py-3 font-bold text-white bg-indigo-600 hover:bg-indigo-700 rounded-xl shadow-md transition-transform hover:scale-[1.02] active:scale-[0.98] whitespace-nowrap">
                 Upgrade to Pro
               </Link>
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
                 <WithTooltip id="stripe-setup-tooltip" defaultText="Connect your bank account securely with Stripe to start getting paid.">
                     <button id="stripe-setup-btn" className="px-5 py-2 bg-red-600 text-white font-bold rounded-lg shadow-sm hover:bg-red-700 transition-colors whitespace-nowrap">
                         Complete Stripe Setup
                     </button>
                 </WithTooltip>
             </div>
         </section>

         {/* Plain-Language Weekly Financial Brief */}
         <section className="mb-8">
            <h2 className="text-xl font-semibold mb-4 font-outfit" style={{ color: '#1D1D1F' }}>Weekly Insights</h2>
            <div className="p-6 shadow-sm border rounded-2xl bg-white border-blue-100 relative overflow-hidden">
                <div className="absolute top-0 right-0 w-24 h-24 bg-blue-50 rounded-bl-full -z-10"></div>
                <div className="flex items-start gap-4">
                   <div className="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center shrink-0">
                      <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                   </div>
                   <div>
                       <h3 className="text-sm font-bold text-gray-900 mb-1">AI Business Advisory</h3>
                       <p className="text-gray-800 text-sm leading-relaxed">
                           Great job! You sold 20 more lunches than last week. Chicken was your top seller. Consider adjusting your pricing by 5% to maximize profits.
                       </p>
                   </div>
                </div>
            </div>
         </section>

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

         {/* Automated AI Review Requests Growth Loop */}
         <section className="mb-6">
            <div className="p-6 shadow-md rounded-2xl border transition-all" style={{ background: 'rgba(255, 255, 255, 0.75)', backdropFilter: 'blur(30px) saturate(210%)', borderColor: 'rgba(16, 185, 129, 0.3)' }}>
                <div className="flex flex-col sm:flex-row justify-between sm:items-center mb-4 gap-2">
                    <h3 className="font-semibold text-lg font-outfit text-gray-900 m-0 flex items-center flex-wrap gap-2">
                        Automated AI Review Requests
                        <span className="text-xs px-3 py-1 rounded-full font-medium" style={{ background: 'rgba(16, 185, 129, 0.1)', color: '#10b981' }}>
                            New Growth Loop
                        </span>
                    </h3>
                </div>
                <p className="text-gray-600 font-inter text-sm mb-5 leading-relaxed">
                    You have 12 recent orders without reviews. Let AI generate and send personalized follow-up emails to collect more 5-star reviews and increase your conversion rate.
                </p>

                {campaignSuccess ? (
                    <div className="p-4 rounded-xl mb-4 font-bold text-sm" style={{ background: 'rgba(16, 185, 129, 0.1)', color: '#10b981' }}>
                        ✓ Campaign sent to <span id="review-emails-sent">12</span> customers!
                    </div>
                ) : (
                    <button
                        onClick={handleSendCampaign}
                        disabled={isSendingCampaign}
                        className="w-full sm:w-auto px-6 py-3 rounded-xl font-bold text-white transition-all hover:opacity-90 disabled:opacity-70 disabled:cursor-not-allowed shadow-md"
                        style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
                    >
                        {isSendingCampaign ? 'Generating drafts...' : '✨ Send AI Review Requests'}
                    </button>
                )}
            </div>
         </section>


         {/* SaaS Conversion: AI Business Insights (Soft Paywall) */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>AI Business Insights</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                        <span className="text-xs font-medium text-yellow-600">Pro Feature</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-2xl flex flex-col md:flex-row gap-6 items-center mb-8" style={{ background: 'linear-gradient(to right, #ffffff, #fcfbf8)', border: '1px solid rgba(0,0,0,0.05)' }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Unlock Advanced Store Analytics</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">Discover hidden trends in your sales data. Our AI analyzes customer behavior to recommend exactly what to sell next and how to price it for maximum profit.</p>
                    <button
                        onClick={() => setShowUpgradeModal(true)}
                        className="px-6 py-3 bg-gradient-to-r from-yellow-500 to-orange-500 hover:from-yellow-600 hover:to-orange-600 text-white font-semibold rounded-xl shadow-sm transition-all flex items-center gap-2"
                    >
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" /></svg>
                        View AI Insights
                    </button>
                </div>
                <div className="hidden md:flex w-32 h-32 items-center justify-center relative">
                   {/* Decorative visual */}
                   <div className="absolute inset-0 bg-gradient-to-br from-yellow-400 to-orange-500 rounded-full opacity-20 blur-xl animate-pulse"></div>
                   <div className="relative w-20 h-20 bg-gradient-to-tr from-yellow-400 to-orange-500 rounded-2xl rotate-3 shadow-lg flex items-center justify-center text-white">
                        <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                   </div>
                </div>
            </div>
         </section>

         {/* Growth Loop: VIP Customer Referral Campaign */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>VIP Customer Referrals</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-purple-50 rounded-full border border-purple-100">
                        <span className="text-xs font-medium text-purple-600">Customer Acquisition</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-2xl flex flex-col md:flex-row gap-6 items-center" style={{ background: '#ffffff', border: '1px solid rgba(0,0,0,0.05)' }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Turn Customers into Promoters</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">You have <strong>12 top customers</strong> who haven't joined your VIP referral program. Ask them to refer their friends using an AI-generated email campaign.</p>
                    <div className="flex flex-col gap-3">
                        <div className="flex items-center justify-between bg-gray-50 p-4 rounded-xl border border-gray-100">
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 rounded-full bg-purple-100 flex items-center justify-center text-purple-600 text-lg">
                                    🎁
                                </div>
                                <div>
                                    <h4 className="text-sm font-semibold text-gray-900">VIP Referral Invite</h4>
                                    <p className="text-xs text-gray-500">12 top customers</p>
                                </div>
                            </div>
                            <button
                                onClick={async () => {
                                    setIsGeneratingCustomerReferral(true);
                                    setShowCustomerReferralModal(true);
                                    setCustomerReferralSent(false);
                                    try {
                                        const response = await fetch('/api/v1/growth/campaign/generate-customer-referral', {
                                            method: 'POST',
                                            headers: { 'Content-Type': 'application/json' },
                                            body: JSON.stringify({ store_name: businessName })
                                        });
                                        if (response.ok) {
                                            const data = await response.json();
                                            if (data.message) {
                                                setCustomerReferralMessage(data.message);
                                            }
                                        }
                                    } catch (e) {
                                        console.error("Failed to generate VIP referral campaign", e);
                                        setCustomerReferralMessage("Hi there! We love having you as a top customer. As a special thank you, give your friends 15% off their first order. When they buy, you get $10! Share your link today.");
                                    } finally {
                                        setIsGeneratingCustomerReferral(false);
                                    }
                                }}
                                className="px-4 py-2 bg-purple-600 text-white rounded-lg text-sm font-semibold hover:bg-purple-700 transition-colors shadow-sm whitespace-nowrap"
                            >
                                Generate Campaign
                            </button>
                        </div>
                    </div>
                </div>
                <div className="w-full md:w-1/3 bg-gray-50 rounded-xl p-4 flex flex-col items-center justify-center border border-gray-100 min-h-[160px]">
                    <div className="text-4xl mb-3">🤝</div>
                    <span className="text-sm font-medium text-gray-600 text-center">+25% more new customers</span>
                </div>
            </div>
         </section>

         {/* Growth Loop: Abandoned Cart Recovery */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Abandoned Cart Recovery</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-red-50 rounded-full border border-red-100">
                        <span className="text-xs font-medium text-red-600">Revenue Recovery</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-2xl flex flex-col md:flex-row gap-6 items-center" style={{ background: '#ffffff', border: '1px solid rgba(0,0,0,0.05)' }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Win Back Lost Sales</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">You have <strong>5 abandoned carts</strong> totaling <strong className="text-green-600">$240.00</strong>. Recover these sales with an AI-generated discount campaign.</p>
                    <div className="flex flex-col gap-3">
                        <div className="flex items-center justify-between bg-gray-50 p-4 rounded-xl border border-gray-100">
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 rounded-full bg-red-100 flex items-center justify-center text-red-600 text-lg">
                                    🛒
                                </div>
                                <div>
                                    <h4 className="text-sm font-semibold text-gray-900">Cart #4410 - Abandoned</h4>
                                    <p className="text-xs text-gray-500">Alex M. left $85.00 in their cart</p>
                                </div>
                            </div>
                            <button
                                onClick={async () => {
                                    setShowCartModal(true);
                                    setIsGeneratingCartCampaign(true);
                                    try {
                                        const response = await fetch('/api/v1/growth/campaign/generate-cart', {
                                            method: 'POST',
                                            headers: { 'Content-Type': 'application/json' },
                                            body: JSON.stringify({
                                                customer_name: 'Alex',
                                                cart_value: '$85.00'
                                            })
                                        });
                                        if (response.ok) {
                                            const data = await response.json();
                                            setCartCampaignMessage(data.message);
                                        } else {
                                            setCartCampaignMessage("Hi Alex,\n\nWe noticed you left some items in your cart totaling $85.00. Did you have any questions or need help checking out?\n\nAs a special thank you for shopping with us, here is a 10% discount code to complete your purchase: COMEBACK10\n\nClick here to securely finish your checkout: https://ohc.store/checkout/recover\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC");
                                        }
                                    } catch (e) {
                                        console.error("Failed to generate cart recovery", e);
                                        setCartCampaignMessage("Hi Alex,\n\nWe noticed you left some items in your cart totaling $85.00. Did you have any questions or need help checking out?\n\nAs a special thank you for shopping with us, here is a 10% discount code to complete your purchase: COMEBACK10\n\nClick here to securely finish your checkout: https://ohc.store/checkout/recover\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC");
                                    } finally {
                                        setIsGeneratingCartCampaign(false);
                                        setCartCampaignSent(false);
                                    }
                                }}
                                className="px-4 py-2 bg-red-600 text-white rounded-lg text-sm font-semibold hover:bg-red-700 transition-colors shadow-sm whitespace-nowrap"
                            >
                                Recover Cart
                            </button>
                        </div>
                    </div>
                </div>
                <div className="w-full md:w-1/3 bg-gray-50 rounded-xl p-4 flex flex-col items-center justify-center border border-gray-100 min-h-[160px]">
                    <div className="text-4xl mb-3">💸</div>
                    <span className="text-sm font-medium text-gray-600 text-center">+15% average recovery rate</span>
                </div>
            </div>
         </section>

         {/* Growth Loop: Automated AI-Driven Review Requests */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Automated Review Requests</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-blue-50 rounded-full border border-blue-100">
                        <span className="text-xs font-medium text-blue-600">Merchant Delight</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-2xl flex flex-col md:flex-row gap-6 items-center" style={{ background: '#ffffff', border: '1px solid rgba(0,0,0,0.05)' }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Turn Customers into Advocates</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">You have <strong>3 recent orders</strong> delivered that haven't left a review. Ask for a review with one tap and build your store's credibility automatically.</p>
                    <div className="flex flex-col gap-3">
                        <div className="flex items-center justify-between bg-gray-50 p-4 rounded-xl border border-gray-100">
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center text-blue-600 text-lg">
                                    ⭐
                                </div>
                                <div>
                                    <h4 className="text-sm font-semibold text-gray-900">Order #8922 - Delivered</h4>
                                    <p className="text-xs text-gray-500">Sarah J. bought Signature Coffee Blend</p>
                                </div>
                            </div>
                            <button
                                onClick={async () => {
                                    setShowReviewModal(true);
                                    setIsGeneratingReview(true);
                                    try {
                                        const response = await fetch('/api/v1/growth/campaign/generate-review', {
                                            method: 'POST',
                                            headers: { 'Content-Type': 'application/json' },
                                            body: JSON.stringify({
                                                order_id: '8922',
                                                customer_name: 'Sarah',
                                                product_name: 'Signature Coffee Blend'
                                            })
                                        });
                                        if (response.ok) {
                                            const data = await response.json();
                                            setReviewMessage(data.message);
                                        } else {
                                            setReviewMessage("Hi Sarah,\n\nWe noticed you recently received your Signature Coffee Blend and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/8922\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC");
                                        }
                                    } catch (e) {
                                        console.error("Failed to generate review", e);
                                        setReviewMessage("Hi Sarah,\n\nWe noticed you recently received your Signature Coffee Blend and we hope you are absolutely loving it!\n\nAs a small business, we rely on feedback from amazing customers like you to grow and improve. If you have a minute, we would be incredibly grateful if you could share your thoughts by leaving a quick review here: https://ohc.store/review/8922\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC");
                                    } finally {
                                        setIsGeneratingReview(false);
                                        setReviewSent(false);
                                    }
                                }}
                                className="px-4 py-2 bg-blue-600 text-white rounded-lg text-sm font-semibold hover:bg-blue-700 transition-colors shadow-sm whitespace-nowrap"
                            >
                                Request Review
                            </button>
                        </div>
                    </div>
                </div>
                <div className="hidden md:flex w-full md:w-1/3 flex-col items-center justify-center p-4">
                     <div className="w-24 h-24 bg-gradient-to-tr from-blue-100 to-indigo-50 rounded-full flex items-center justify-center mb-3">
                         <div className="text-4xl animate-bounce">💌</div>
                     </div>
                     <p className="text-xs font-medium text-gray-500 text-center">Stores with reviews sell 3x more</p>
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
                        onClick={async () => {
                            setShowPromoModal(true);
                            setIsGeneratingPromo(true);
                            try {
                                const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
                                const response = await fetch("/api/v1/growth/promotions/generate", {
                                    method: "POST",
                                    headers: { "Content-Type": "application/json" },
                                    body: JSON.stringify({ tenant })
                                });
                                if (response.ok) {
                                    const data = await response.json();
                                    if (data.message) {
                                        setPromoMessage(data.message);
                                    }
                                }
                            } catch (e) {
                                console.error("Failed to generate promotion", e);
                            } finally {
                                setIsGeneratingPromo(false);
                            }
                        }}
                        disabled={isGeneratingPromo}
                        className={`px-6 py-3 bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-700 hover:to-indigo-700 text-white font-semibold rounded-xl shadow-sm transition-all flex items-center gap-2 ${isGeneratingPromo ? "opacity-75 cursor-not-allowed" : ""}`}
                    >
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                        {isGeneratingPromo ? "Generating..." : "Generate Promotion"}
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

         {/* Growth Loop: Interactive Analytics Soft Paywall */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Advanced Analytics</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                        <span className="text-xs font-medium text-yellow-700">Premium Growth Loop</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-2xl relative overflow-hidden" style={{ background: 'rgba(255, 255, 255, 0.03)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.08)', borderColor: 'rgba(0,0,0,0.05)', backgroundColor: '#ffffff' }}>
                <div className="filter blur-sm opacity-60 select-none flex flex-col sm:flex-row gap-6 items-center">
                    <div className="flex-1 w-full">
                        <div className="bg-gray-50 border border-gray-100 p-4 rounded-xl flex items-center justify-between mb-3">
                            <span className="font-semibold text-gray-700">Conversion Rate</span>
                            <span className="text-xl font-bold text-green-600">4.2%</span>
                        </div>
                        <div className="bg-gray-50 border border-gray-100 p-4 rounded-xl flex items-center justify-between">
                            <span className="font-semibold text-gray-700">Customer Lifetime Value</span>
                            <span className="text-xl font-bold text-blue-600">$184.50</span>
                        </div>
                    </div>
                    <div className="w-full md:w-1/3 bg-gray-50 rounded-xl p-4 flex flex-col items-center justify-center border border-gray-100 min-h-[160px]">
                        <div className="text-4xl mb-3">📈</div>
                        <span className="text-sm font-medium text-gray-600 text-center">Top Traffic: Organic Search</span>
                    </div>
                </div>

                <div className="absolute inset-0 flex flex-col items-center justify-center bg-white/40 backdrop-blur-[2px]">
                    <div className="bg-white p-6 rounded-2xl shadow-xl border border-yellow-100 text-center max-w-sm flex flex-col items-center animate-fade-in" style={{ transform: 'translateY(10px)' }}>
                        <div className="w-12 h-12 bg-yellow-100 text-yellow-600 rounded-full flex items-center justify-center text-xl mb-3">
                            🔒
                        </div>
                        <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Unlock Growth Insights</h3>
                        <p className="text-sm text-gray-600 mb-4">See exactly where your best customers come from and optimize your store to double your conversion rate.</p>
                        <button
                            onClick={() => setShowUpgradeModal(true)}
                            className="w-full py-2.5 bg-gradient-to-r from-yellow-500 to-orange-500 text-white rounded-xl text-sm font-semibold shadow-md hover:shadow-lg transition-all hover:scale-[1.02]"
                        >
                            Upgrade to Premium
                        </button>
                    </div>
                </div>
            </div>
         </section>

         {/* Growth Loop: Milestone Celebration */}
         {showMilestoneBanner && (
           <section className="mb-8 animate-fade-in">
              <div className="p-6 shadow-sm border rounded-[16px] flex flex-col md:flex-row gap-6 items-center" style={{ background: 'linear-gradient(135deg, #f6d365 0%, #fda085 100%)', borderColor: 'rgba(0,0,0,0.05)' }}>
                  <div className="flex-1 text-white">
                      <div className="flex items-center gap-3 mb-2">
                          <span className="text-3xl">🎉</span>
                          <h3 className="text-xl font-bold font-outfit text-white">Milestone Unlocked: Your First Customers!</h3>
                      </div>
                      <p className="text-sm text-white/90 mb-4 leading-relaxed font-medium">You've reached <strong className="text-white">100 active customers</strong>. Share your store's success to earn a free month of Pro!</p>
                      <button
                          onClick={() => {
                              const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
                              const url = `ohc://join?ref=${tenant}`;
                              const text = `I just reached 100 customers on my store! Start your own business today with One Human Corp: ${url}`;
                              window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}`, '_blank');
                              setShowMilestoneBanner(false);
                          }}
                          className="px-5 py-2.5 bg-white text-orange-500 font-bold rounded-xl shadow-md hover:bg-orange-50 transition-all font-inter text-sm"
                      >
                          Share & Claim Reward
                      </button>
                  </div>
                  <div className="hidden md:flex flex-col items-center justify-center p-4">
                      <div className="w-24 h-24 rounded-full bg-white/20 flex items-center justify-center backdrop-blur-sm border border-white/30">
                          <span className="text-4xl font-bold text-white">100</span>
                      </div>
                  </div>
              </div>
           </section>
         )}

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
            <div className="p-6 shadow-sm border rounded-2xl flex flex-col md:flex-row gap-6 items-center" style={{ background: 'rgba(255, 255, 255, 0.03)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.08)', borderColor: 'rgba(0,0,0,0.05)', backgroundColor: '#ffffff' }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Sell Anywhere</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">Embed your OHC storefront on your existing website, blog, or partner pages. This powerful widget allows customers to buy directly from you anywhere on the web.</p>
                    <div className="bg-gray-50 border border-gray-200 rounded-lg p-3 relative">
                        <div className="flex gap-2 items-center">
                            <input type="text" readOnly value={`<iframe src="https://ohc.app/api/v1/growth/storefront/embed" ...></iframe>`} className="flex-1 bg-transparent text-sm text-gray-500 outline-none p-1 font-mono border rounded" />
                            <button
                                onClick={() => setShowEmbedModal(true)}
                                className="px-3 py-1.5 bg-gray-900 text-white rounded-md text-xs font-semibold hover:bg-black transition-colors shadow-sm whitespace-nowrap"
                            >
                                Get Widget
                            </button>
                        </div>
                    </div>
                </div>
                <div className="w-full md:w-1/3 bg-gray-50 rounded-xl p-4 flex flex-col items-center justify-center border border-gray-100 min-h-[160px]">
                    <div className="text-4xl mb-3">💻</div>
                    <span className="text-sm font-medium text-gray-600 text-center">Preview: Connect your brand everywhere</span>
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
                        if (productCount >= 10) {
                            setShowPaywallModal(true);
                        } else {
                            setProductCount(prev => prev + 1);
                            if (!productAdded) {
                                setProductAdded(true);
                                setTrialDaysLeft(prev => prev + 7);
                            }
                        }
                    }}
                    className="flex items-center gap-2 px-5 py-2.5 bg-gray-900 text-white font-semibold rounded-xl shadow-md hover:bg-black transition-all font-inter text-sm"
                >
                    <span>+ Add Product</span>
                </button>
            </div>
         </section>

         {/* Growth Loop: Interactive Trial Extension */}
         <section className="mb-8">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-4 gap-4">
                <div className="flex items-center gap-4">
                    <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Extend Your Trial</h2>
                    <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                        <span className="text-xs font-medium text-yellow-600">Grow Faster</span>
                    </div>
                </div>
            </div>
            <div className="p-6 shadow-sm border rounded-[16px] flex flex-col md:flex-row gap-6 items-center" style={{ background: 'linear-gradient(135deg, #fdfbfb 0%, #ebedee 100%)', borderColor: 'rgba(0,0,0,0.05)' }}>
                <div className="flex-1">
                    <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Unlock More Time</h3>
                    <p className="text-sm text-gray-600 mb-4 leading-relaxed">You have <strong className="text-gray-900">{trialDaysLeft} days left</strong> in your free trial. Complete these quick tasks to earn more time and get the most out of OHC.</p>

                    <div className="flex flex-col gap-3">
                        <div className="flex items-center justify-between bg-white p-3 rounded-[8px] shadow-sm border border-gray-100">
                            <div className="flex items-center gap-3">
                                <div className="w-8 h-8 rounded-full bg-blue-50 flex items-center justify-center text-blue-500">
                                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                                </div>
                                <div>
                                    <h4 className="text-sm font-semibold text-gray-900">Connect Twitter</h4>
                                    <p className="text-xs text-gray-500">+7 Days</p>
                                </div>
                            </div>
                            <button
                                onClick={() => {
                                    if (!twitterConnected) {
                                        setTwitterConnected(true);
                                        setTrialDaysLeft(prev => prev + 7);
                                    }
                                }}
                                disabled={twitterConnected}
                                className={`px-4 py-1.5 text-xs font-semibold rounded-[6px] transition-colors ${twitterConnected ? 'bg-green-100 text-green-700 cursor-not-allowed' : 'bg-blue-600 text-white hover:bg-blue-700'}`}
                            >
                                {twitterConnected ? 'Connected' : 'Connect'}
                            </button>
                        </div>

                        <div className="flex items-center justify-between bg-white p-3 rounded-[8px] shadow-sm border border-gray-100">
                            <div className="flex items-center gap-3">
                                <div className="w-8 h-8 rounded-full bg-purple-50 flex items-center justify-center text-purple-500">
                                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" /></svg>
                                </div>
                                <div>
                                    <h4 className="text-sm font-semibold text-gray-900">Leave a Review</h4>
                                    <p className="text-xs text-gray-500">+7 Days</p>
                                </div>
                            </div>
                            <button
                                onClick={() => {
                                    if (!reviewLeft) {
                                        setReviewLeft(true);
                                        setTrialDaysLeft(prev => prev + 7);
                                    }
                                }}
                                disabled={reviewLeft}
                                className={`px-4 py-1.5 text-xs font-semibold rounded-[6px] transition-colors ${reviewLeft ? 'bg-green-100 text-green-700 cursor-not-allowed' : 'bg-gray-900 text-white hover:bg-gray-800'}`}
                            >
                                {reviewLeft ? 'Done' : 'Review'}
                            </button>
                        </div>

                        <div className="flex items-center justify-between bg-white p-3 rounded-[8px] shadow-sm border border-gray-100">
                            <div className="flex items-center gap-3">
                                <div className="w-8 h-8 rounded-full bg-green-50 flex items-center justify-center text-green-500">
                                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
                                </div>
                                <div>
                                    <h4 className="text-sm font-semibold text-gray-900">Add First Product</h4>
                                    <p className="text-xs text-gray-500">+7 Days</p>
                                </div>
                            </div>
                            <button
                                disabled={true}
                                className={`px-4 py-1.5 text-xs font-semibold rounded-[6px] transition-colors ${productAdded ? 'bg-green-100 text-green-700 cursor-not-allowed' : 'bg-gray-100 text-gray-500 cursor-not-allowed'}`}
                            >
                                {productAdded ? 'Done' : 'Pending'}
                            </button>
                        </div>
                    </div>
                </div>
                <div className="w-full md:w-1/3 flex justify-center mt-4 md:mt-0">
                     <div className="text-center">
                        <div className="text-5xl font-outfit font-bold text-gray-900 mb-2">{trialDaysLeft}</div>
                        <div className="text-sm font-medium text-gray-500 uppercase tracking-widest">Days Left</div>
                    </div>
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
                    name="Referrals"
                    onClick={openReferralModal}
                    disabled={isGeneratingReferral}
                    className={`flex items-center gap-2 px-5 py-2.5 bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0 transition-all font-inter text-sm ${isGeneratingReferral ? "opacity-75 cursor-not-allowed" : ""}`}
                >
                    <span>{isGeneratingReferral ? "Generating..." : "🎁 Invite a Business & Earn $50"}</span>
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
                  href={`https://wa.me/?text=${encodeURIComponent(`Just hit an amazing milestone: ${currentMilestone.title} on my new store! 🚀 Built entirely with AI on @OneHumanCorp. Launch yours and get $50 credit: ${referralLink}`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                  Share to WhatsApp
                </a>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`Just hit an amazing milestone: ${currentMilestone.title} on my new store! 🚀 Built entirely with AI on @OneHumanCorp. Launch yours and get $50 credit: ${referralLink}`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-black text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                  Share to X
                </a>
              </div>
              <div className="flex justify-center pt-2">
                 <button
                    onClick={() => setShowMilestoneModal(false)}
                    className="text-xs font-semibold text-gray-500 hover:text-gray-700 uppercase tracking-widest transition-colors"
                 >
                    Dismiss
                 </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Paywall Modal */}
      {showPaywallModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter">
            {/* Background embellishment */}
            <div className="absolute top-0 right-0 w-32 h-32 bg-orange-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-orange-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-orange-600">
                ⭐
              </div>
              <button
                onClick={() => setShowPaywallModal(false)}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">You've hit your limit!</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              You have reached the <strong className="text-gray-900">10 Products Limit</strong> on the Free plan. Upgrade to the Starter plan to add more products and unlock unlimited potential.
            </p>

            <div className="space-y-3">
              <Link
                href="/pricing"
                className="block w-full py-3 bg-gradient-to-r from-orange-500 to-red-500 text-white font-semibold rounded-xl text-center shadow-md hover:shadow-lg hover:-translate-y-0.5 transition-all"
              >
                Upgrade to Starter
              </Link>
              <button
                onClick={() => setShowPaywallModal(false)}
                className="w-full py-2 rounded-xl text-sm font-semibold text-gray-500 hover:text-gray-700 transition-colors"
              >
                Maybe later
              </button>
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
              {isGeneratingPromo ? (
                 <div className="flex flex-col items-center justify-center py-8">
                     <div className="inline-block w-8 h-8 rounded-full border-2 border-purple-200 border-t-purple-600 animate-spin mb-3"></div>
                     <span className="text-sm text-gray-500">Generating the perfect message...</span>
                 </div>
              ) : (
                <>
                  <div>
                    <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">Generated Message</label>
                    <textarea
                      readOnly
                      rows={4}
                      value={promoMessage}
                      className="w-full bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none resize-none"
                    />
                  </div>

                  <button
                    onClick={() => {
                      navigator.clipboard.writeText(promoMessage);
                      setCopied(true);
                      setTimeout(() => setCopied(false), 2000);
                    }}
                    className={`w-full py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                  >
                    {copied ? 'Copied!' : 'Copy to Clipboard'}
                  </button>
                </>
              )}

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

      {/* Abandoned Cart Modal */}
      {showCartModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-red-100">
            <div className="absolute top-0 right-0 w-32 h-32 bg-red-50 rounded-bl-full -z-10"></div>
            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-red-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-red-600">
                🛒
              </div>
              <button
                onClick={() => {
                  setShowCartModal(false);
                  setCartCampaignSent(false);
                }}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">AI Cart Recovery</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              We've drafted a personalized recovery message for Alex. Review and send it to win back this sale.
            </p>
            <div className="space-y-4">
              {isGeneratingCartCampaign ? (
                 <div className="w-full bg-gray-50 border border-gray-200 rounded-lg p-6 flex flex-col items-center justify-center min-h-[160px]">
                    <svg className="animate-spin h-8 w-8 text-red-500 mb-3" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle><path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                    <span className="text-sm font-medium text-gray-500">Drafting personalized campaign...</span>
                 </div>
              ) : (
                <>
                    <textarea
                        value={cartCampaignMessage}
                        onChange={(e) => setCartCampaignMessage(e.target.value)}
                        className="w-full bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-red-500"
                        rows={8}
                    />
                    {cartCampaignSent ? (
                        <div className="w-full py-3 bg-green-50 text-green-700 rounded-xl text-center text-sm font-semibold border border-green-200 flex items-center justify-center gap-2">
                             <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                             Campaign Sent Successfully!
                        </div>
                    ) : (
                        <button
                            onClick={async () => {
                                try {
                                    const response = await fetch('/api/v1/growth/campaign/send', {
                                        method: 'POST',
                                        headers: { 'Content-Type': 'application/json' },
                                        body: JSON.stringify({
                                            name: 'Abandoned Cart Recovery',
                                            subject: 'Forgot something? Here is 10% off',
                                            message: cartCampaignMessage
                                        })
                                    });
                                    if (response.ok) {
                                        setCartCampaignSent(true);
                                    } else {
                                        console.error('Failed to send abandoned cart campaign');
                                    }
                                } catch (e) {
                                    console.error('Failed to send abandoned cart campaign', e);
                                }
                            }}
                            className="w-full py-3 bg-red-600 text-white rounded-xl text-sm font-semibold shadow-md hover:bg-red-700 transition-colors"
                        >
                            Send Campaign
                        </button>
                    )}
                </>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Review Request Modal */}
      {showReviewModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-blue-100">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>
            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-blue-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-blue-600">
                ⭐
              </div>
              <button
                onClick={() => {
                  setShowReviewModal(false);
                  setReviewSent(false);
                }}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">AI Review Request</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              We've drafted a personalized review request for Sarah. Review and send it with one click.
            </p>
            <div className="space-y-4">
              {isGeneratingReview ? (
                 <div className="w-full bg-gray-50 border border-gray-200 rounded-lg p-6 flex flex-col items-center justify-center min-h-[160px]">
                    <svg className="animate-spin h-8 w-8 text-blue-500 mb-3" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle><path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                    <span className="text-sm font-medium text-gray-500">Drafting personalized email...</span>
                 </div>
              ) : (
                <>
                    <textarea
                        value={reviewMessage}
                        onChange={(e) => setReviewMessage(e.target.value)}
                        className="w-full bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-700 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        rows={8}
                    />
                    {reviewSent ? (
                        <div className="w-full py-3 bg-green-50 text-green-700 rounded-xl text-center text-sm font-semibold border border-green-200 flex items-center justify-center gap-2">
                             <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                             Review Request Sent!
                        </div>
                    ) : (
                        <button
                            onClick={async () => {
                                try {
                                    const response = await fetch('/api/v1/growth/campaign/send', {
                                        method: 'POST',
                                        headers: { 'Content-Type': 'application/json' },
                                        body: JSON.stringify({
                                            name: 'Automated Review Request',
                                            subject: 'How did we do? Leave a review!',
                                            body: reviewMessage,
                                            target_segment: 'recent_buyers_no_review'
                                        })
                                    });
                                    if (response.ok) {
                                        setReviewSent(true);
                                        setShowReviewModal(false);
                                    } else {
                                        setReviewMessage('Failed to send campaign. Please try again later.');
                                    }
                                } catch (e) {
                                    console.error('Failed to send review campaign', e);
                                    setReviewMessage('Failed to send campaign. Please try again later.');
                                }
                            }}
                            className="w-full py-3 rounded-xl text-sm font-semibold transition-all bg-blue-600 text-white hover:bg-blue-700 shadow-md flex items-center justify-center gap-2"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                            Send Email
                        </button>
                    )}
                </>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Customer Referral Modal */}
      {showCustomerReferralModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-purple-100">
            {/* Background embellishment */}
            <div className="absolute top-0 right-0 w-32 h-32 bg-purple-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-purple-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-purple-600">
                🎁
              </div>
              <button
                onClick={() => setShowCustomerReferralModal(false)}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">AI Referral Invite</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              {customerReferralSent ? "Campaign successfully sent to your top 12 customers!" : "Review the AI-generated referral invite. This will be emailed to your top 12 customers."}
            </p>

            <div className="space-y-4">
              {!customerReferralSent && (
                <>
                    {isGeneratingCustomerReferral ? (
                        <div className="w-full bg-gray-50 border border-gray-200 rounded-lg p-6 flex flex-col items-center justify-center gap-3">
                             <div className="w-8 h-8 border-4 border-purple-200 border-t-purple-600 rounded-full animate-spin"></div>
                             <span className="text-sm font-medium text-gray-500">Drafting personalized invites...</span>
                        </div>
                    ) : (
                        <textarea
                            value={customerReferralMessage}
                            onChange={(e) => setCustomerReferralMessage(e.target.value)}
                            rows={8}
                            className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-purple-500 text-sm text-gray-700 resize-none font-inter"
                        />
                    )}

                    {!isGeneratingCustomerReferral && (
                        <button
                            onClick={async () => {
                                // Simulate sending email
                                setCustomerReferralSent(true);
                                setTimeout(() => {
                                    setShowCustomerReferralModal(false);
                                }, 3000);
                            }}
                            className="w-full py-3 bg-gradient-to-r from-purple-600 to-indigo-600 text-white font-semibold rounded-xl text-center shadow-md hover:shadow-lg hover:-translate-y-0.5 transition-all"
                        >
                            Send Campaign to 12 Customers
                        </button>
                    )}
                </>
              )}
            </div>
          </div>
        </div>
      )}

      {/* SaaS Conversion: Upgrade Modal (Soft Paywall) */}
      {showUpgradeModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-yellow-100">
            {/* Background embellishment */}
            <div className="absolute top-0 right-0 w-32 h-32 bg-yellow-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-yellow-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-yellow-600">
                📈
              </div>
              <button
                onClick={() => setShowUpgradeModal(false)}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Unlock AI Business Insights and start seeing actionable trends in your sales data. Pro members sell <strong className="text-gray-900">3x more</strong> on average within their first month!
            </p>

            <div className="space-y-4">
               <ul className="text-sm text-gray-700 space-y-3 mb-6">
                 <li className="flex items-center gap-2">
                    <svg className="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                    Predictive sales analytics
                 </li>
                 <li className="flex items-center gap-2">
                    <svg className="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                    AI-driven pricing recommendations
                 </li>
                 <li className="flex items-center gap-2">
                    <svg className="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                    Automated competitor tracking
                 </li>
               </ul>

              <button
                onClick={() => {
                  window.location.href = '/checkout';
                }}
                className="w-full py-3 rounded-xl text-sm font-semibold transition-all bg-gradient-to-r from-yellow-500 to-orange-500 text-white hover:from-yellow-600 hover:to-orange-600 shadow-md hover:shadow-lg"
              >
                Upgrade Now - $29/mo
              </button>

              <button
                onClick={() => setShowUpgradeModal(false)}
                className="w-full py-2 rounded-xl text-sm font-semibold text-gray-500 hover:text-gray-700 transition-colors"
              >
                Maybe later
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="text-5xl mb-4">✨</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Unlock AI Power</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Automated AI Review Requests are a Pro feature. Upgrade to our Pro plan to boost your sales on autopilot.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
            >
              Upgrade to Pro
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm hover:bg-gray-50 flex items-center justify-center gap-2"
              style={{ color: '#1DA1F2', border: '2px solid #1DA1F2', background: 'white' }}
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X to get 7 Days Free
            </button>
          </div>
        </div>
      )}

      {/* Embed Modal */}
      {showEmbedModal && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-6 shadow-2xl relative overflow-hidden font-inter border border-green-100">
            <div className="absolute top-0 right-0 w-32 h-32 bg-green-50 rounded-bl-full -z-10"></div>
            <div className="flex justify-between items-start mb-4">
              <div className="w-12 h-12 bg-green-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-green-600">
                🌐
              </div>
              <button
                onClick={() => {
                  setShowEmbedModal(false);
                  setEmbedCopied(false);
                }}
                className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors"
              >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
              </button>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Embed Storefront</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Copy this widget code and paste it on your blog, personal website, or partner pages to let customers buy directly from you.
            </p>
            <div className="space-y-4">
              <div>
                <label className="block text-xs font-semibold text-gray-700 uppercase tracking-wide mb-2">Widget HTML snippet</label>
                <div className="flex flex-col gap-2">
                  <textarea
                    readOnly
                    value={`<iframe src="https://ohc.app/api/v1/growth/storefront/embed?tenant=${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}&theme=light" width="320" height="400" frameborder="0" scrolling="no"></iframe>`}
                    className="w-full bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none font-mono text-xs"
                    rows={4}
                  />
                  <button
                    onClick={() => {
                      navigator.clipboard.writeText(`<iframe src="https://ohc.app/api/v1/growth/storefront/embed?tenant=${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}&theme=light" width="320" height="400" frameborder="0" scrolling="no"></iframe>`);
                      setEmbedCopied(true);
                      setTimeout(() => setEmbedCopied(false), 2000);
                    }}
                    className={`w-full py-2 rounded-lg text-sm font-semibold transition-all ${embedCopied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                  >
                    {embedCopied ? 'Copied!' : 'Copy Code'}
                  </button>
                </div>
              </div>
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
                    value={referralLink}
                    className="flex-1 bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none"
                  />
                  <button
                    onClick={() => {
                      navigator.clipboard.writeText(referralLink);
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
                  href={`https://wa.me/?text=${encodeURIComponent(`Launch your business online instantly with OHC! Use my invite link: ${referralLink}`)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                  WhatsApp
                </a>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`Launch your business online instantly with OHC! Use my invite link: ${referralLink}`)}`}
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
