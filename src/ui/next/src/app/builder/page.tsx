"use client";

import { useState, useEffect } from "react";
import { SmartBlock, SkeletonBlock, ActionSheet, DraggableBlock, QRCode } from "./components";
import { useWalkthrough } from "../../components/help";
import { WithTooltip } from "../../components/TooltipRegistry";
import { useBuilderStore } from "./store";

export default function BuilderPage() {
  const {
    bio, setBio,
    businessName, setBusinessName,
    businessCategory, setBusinessCategory,
    vibe, setVibe,
    wizardStep, setWizardStep,
    blocks, setBlocks,
    drafts, setDrafts,
    status, setStatus,
    businessGoal, setBusinessGoal,
    liveUrl, setLiveUrl
  } = useBuilderStore();

  const [isLoaded, setIsLoaded] = useState(false);
  const [selectedDraftIndex, setSelectedDraftIndex] = useState(0);
  const [selectedBlockIndex, setSelectedBlockIndex] = useState<number | null>(null);
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);
  const [draggedIndex, setDraggedIndex] = useState<number | null>(null);
  const [startY, setStartY] = useState(0);
  const { startWalkthrough } = useWalkthrough();

  const [wizardStep1Error, setWizardStep1Error] = useState("");

  const handleStep1Next = () => {
    if (businessName.trim().length < 3) {
      setWizardStep1Error("Business name must be at least 3 characters.");
      return;
    }
    if (businessCategory.trim().length < 5) {
      setWizardStep1Error("Category must be at least 5 characters.");
      return;
    }
    setWizardStep1Error("");
    setWizardStep(2);
  };

  // GEO UI State
  const [geoScore, setGeoScore] = useState<number | null>(null);
  const [geoRecs, setGeoRecs] = useState<string[]>([]);
  const [seoApplied, setSeoApplied] = useState(false);

  // Growth Loop: Soft Paywall State
  const [isPremium, setIsPremium] = useState(false);
  const [showUpgradeModal, setShowUpgradeModal] = useState(false);
  const [tenantId, setTenantId] = useState("storefront");

  useEffect(() => {
    const savedTenantId = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "storefront";
    setTenantId(savedTenantId);
    setIsLoaded(true);
  }, []);

  const handleGeoAnalysis = async () => {
    try {
      const response = await fetch('/api/v1/builder/geo_score', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content: bio })
      });
      const data = await response.json();
      setGeoScore(data.generative_score);
      setGeoRecs(data.recommendations);
    } catch (error) {
      console.error("Failed to analyze GEO score", error);
    }
  };

  const handleAutoSeo = async () => {
    try {
      await fetch('/api/v1/builder/auto_seo', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content: bio })
      });
      setSeoApplied(true);
    } catch (error) {
      console.error("Failed to apply Auto SEO", error);
    }
  };

  const handleGenerate = async () => {
    setStatus("generating");

    try {
      const response = await fetch('/api/v1/builder/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: bio })
      });

      const data = await response.json();
      const newBlocks = data.pages[0].blocks.map((b: any) => ({
        type: b.block_type === 'HeroBlock' ? 'Hero' :
              b.block_type === 'ProductGridBlock' ? 'Catalog' :
              b.block_type === 'ServiceBookingBlock' ? 'Booking' :
              b.block_type === 'TestimonialBlock' ? 'Testimonials' : b.block_type,
        props: b.content
      }));

      // Inject Viral Loop: Every new store gets a Referral block by default
      newBlocks.push({
        type: 'Referral',
        props: {
          offerTitle: "Refer a Friend & Earn",
          offerDescription: "Get 20% off your next purchase when a friend buys from us!"
        }
      });

      // For V2, we simulate 3 drafts by slightly varying the first one if only one is returned,
      // or we just use what's there. In real V2, backend would return 3.
      const draft2 = JSON.parse(JSON.stringify(newBlocks));
      if (draft2[0] && draft2[0].props) draft2[0].props.headline += " (Variant B)";

      const draft3 = JSON.parse(JSON.stringify(newBlocks));
      if (draft3[0] && draft3[0].props) draft3[0].props.headline += " (Variant C)";

      setDrafts([newBlocks, draft2, draft3]);
      setBlocks(newBlocks);
      setStatus("selection");
    } catch (error) {
      console.error("Failed to generate storefront", error);
      setStatus("idle");
    }
  };

  const handleLaunch = async () => {
    try {
      const draftBlocks = blocks.map((b, i) => ({
        block_type: b.type === 'Hero' ? 'HeroBlock' :
                    b.type === 'Catalog' ? 'ProductGridBlock' :
                    b.type === 'Booking' ? 'ServiceBookingBlock' :
                    b.type === 'Testimonials' ? 'TestimonialBlock' :
                    b.type === 'Referral' ? 'CustomerReferralBlock' : b.type,
        content: b.props,
        sort_order: i
      }));

      // In a more complete implementation, we'd store the SiteDraft returned from generate,
      // but for now we construct a minimal valid draft payload preserving current blocks.
      const payload = {
          domain: null,
          draft: {
              domain: null,
              pages: [{
                  path: '/',
                  title: 'Home',
                  blocks: draftBlocks,
                  seo_metadata: {
                    "@context": "https://schema.org",
                    "@type": "LocalBusiness",
                    "name": bio
                  }
              }]
          }
      };

      const response = await fetch('/api/v1/builder/publish_draft', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
      });
      if (response.ok) {
        const data = await response.json();
        setStatus("live");
        setLiveUrl(`https://${data.domain || 'myshop'}.ohc.store`);
      } else {
        console.error('Failed to publish');
      }
    } catch (error) {
      console.error('Error publishing:', error);
    }
  };

  if (status === "selection") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter overflow-hidden">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:rounded-[16px] glass-container mac-glass-container backdrop-blur-xl bg-white/30 shadow-2xl">
           <div className="px-8 pt-12 pb-6 text-center">
              <h1 className="text-2xl font-extrabold font-outfit text-gray-900 mb-2">Pick your draft</h1>
              <p className="text-sm text-gray-500">The Architect generated 3 options for you.</p>
           </div>

           <div className="flex-1 overflow-y-auto px-6 space-y-6 pb-24">
              {drafts.map((d, idx) => (
                <button
                  key={idx}
                  onClick={() => {
                    setBlocks(d);
                    setSelectedDraftIndex(idx);
                  }}
                  className={`w-full text-left bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[16px] border-2 transition-all overflow-hidden ${selectedDraftIndex === idx ? 'border-[#0066FF] ring-2 ring-[#0066FF]/20 shadow-lg' : 'border-white/50 dark:border-white/10 opacity-70 hover:opacity-100 hover:border-white/80'}`}
                >
                   <div className="h-32 bg-white/50 dark:bg-black/30 flex items-center justify-center relative backdrop-blur-sm border-b border-white/40 dark:border-white/10">
                      <span className="font-outfit font-bold text-gray-400 dark:text-gray-500">Draft {idx + 1}</span>
                      {selectedDraftIndex === idx && (
                        <div className="absolute top-2 right-2 bg-[#0066FF] text-white rounded-full p-1">
                          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                        </div>
                      )}
                   </div>
                   <div className="p-4 bg-white/60 dark:bg-black/40 backdrop-blur-md">
                      <p className="text-xs font-bold text-gray-500 dark:text-[#A1A1A6] uppercase tracking-wider mb-1">Preview</p>
                      <p className="text-sm text-[#1D1D1F] dark:text-[#F5F5F7] line-clamp-1 font-inter">{d[0]?.props?.headline || "Storefront Preview"}</p>
                   </div>
                </button>
              ))}
           </div>

           <div className="absolute bottom-0 w-full p-6 glass-container mac-glass-container border-t border-white/40 dark:border-white/10 z-50">
              <button
                onClick={() => setStatus("draft")}
                className="w-full bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[8px] font-bold font-outfit shadow-md hover:shadow-lg active:scale-[0.98] transition-all"
              >
                Customize Selected Draft
              </button>
           </div>
        </div>
      </div>
    );
  }

  if (status === "onboarding") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter overflow-hidden">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:rounded-[16px] glass-container mac-glass-container backdrop-blur-xl bg-white/30 shadow-2xl">
          {/* Abstract Background Blur */}
          <div className="absolute inset-0 -z-10">
            <div className="absolute top-[-10%] left-[-10%] w-[120%] h-[120%] bg-gradient-to-br from-blue-400 via-purple-400 to-pink-400 blur-[80px] opacity-30 animate-pulse" />
          </div>

          <div className="flex-1 flex flex-col items-center justify-center px-6 text-center">
            <div className="bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[16px] border border-white/50 dark:border-white/10 shadow-sm p-8 w-full animate-fade-in" style={{ animation: 'fadeIn 300ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
              <h1 className="text-3xl font-extrabold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-6 leading-tight">
                What are you building today?
              </h1>

              <div className="space-y-4">
                {[
                  { id: 'products', label: 'Selling Products', icon: '🛍️' },
                  { id: 'services', label: 'Offering Services', icon: '🛠️' },
                  { id: 'work', label: 'Showcasing Work', icon: '✨' },
                ].map((option) => (
                  <button
                    key={option.id}
                    onClick={() => {
                      setBusinessGoal(option.id as any);
                      setTimeout(() => setStatus("idle"), 300);
                    }}
                    className="w-full p-6 bg-white/60 dark:bg-black/30 backdrop-blur-sm rounded-[16px] border border-white/50 dark:border-white/10 flex flex-col items-center gap-2 active:scale-[0.98] transition-all duration-200 group hover:bg-white/80 dark:hover:bg-black/50"
                  >
                    <span className="text-3xl group-hover:scale-110 transition-transform">{option.icon}</span>
                    <span className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7]">{option.label}</span>
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (status === "idle") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:rounded-[16px] glass-container mac-glass-container backdrop-blur-xl bg-white/30 shadow-2xl">

          <div className="px-8 pt-12 pb-4">
             <div className="flex justify-between mb-8">
               {[1, 2, 3].map(step => (
                 <div key={step} className={`h-1.5 flex-1 mx-1 rounded-full ${step <= wizardStep ? 'bg-[#0071E3]' : 'bg-gray-200 dark:bg-gray-700'}`} style={{ transition: 'all 250ms cubic-bezier(0.4, 0, 0.2, 1)' }} />
               ))}
             </div>
          </div>

          <div className="px-8 pb-8 flex flex-col flex-1 justify-start overflow-y-auto">
            {wizardStep === 1 && (
              <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Let's build your store</h1>
                <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                  Start with the basics. What's your business called, and what do you do?
                </p>

                <label className="text-sm font-semibold text-gray-700 dark:text-[#a1a1a6] mb-2 block text-left">Business Name</label>
                <input
                  type="text"
                  className="w-full border border-white/50 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-md p-4 mb-6 focus:ring-2 focus:ring-[#0066FF]/50 focus:border-[#0066FF] outline-none transition-all text-[#1D1D1F] dark:text-[#f5f5f7] shadow-inner"
                  style={{ borderRadius: '8px' }}
                  value={businessName}
                  onChange={(e) => setBusinessName(e.target.value)}
                  placeholder="e.g. Acme Corp"
                />

                <label className="text-sm font-semibold text-gray-700 dark:text-[#a1a1a6] mb-2 block text-left">Category</label>
                <input
                  type="text"
                  className="w-full border border-white/50 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-md p-4 mb-8 focus:ring-2 focus:ring-[#0066FF]/50 focus:border-[#0066FF] outline-none transition-all text-[#1D1D1F] dark:text-[#f5f5f7] shadow-inner"
                  style={{ borderRadius: '8px' }}
                  value={businessCategory}
                  onChange={(e) => setBusinessCategory(e.target.value)}
                  placeholder="e.g. Retail, Consulting, Tech"
                />

                {wizardStep1Error && (
                   <p className="text-[#FF3B30] text-sm mb-4 text-left">{wizardStep1Error}</p>
                )}

                <button
                  className={`w-full p-4 font-bold font-outfit text-lg transition-all ${
                    businessName.trim().length >= 3 && businessCategory.trim().length >= 5
                      ? "text-white shadow-md active:scale-[0.98] bg-gradient-to-r from-[#0066FF] to-[#0052cc]"
                      : "bg-white/40 dark:bg-black/20 text-gray-400 dark:text-gray-500 cursor-not-allowed border border-white/50 dark:border-white/10"
                  }`}
                  style={{ borderRadius: '8px' }}
                  onClick={handleStep1Next}
                >
                  Next: Choose Vibe
                </button>
              </div>
            )}

            {wizardStep === 2 && (
              <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Select Your Vibe</h1>
                <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                  How should your store feel? Our AI agents will match this tone.
                </p>

                <div className="grid gap-4 mb-8">
                  {['Professional', 'Friendly', 'Energetic', 'Minimalist'].map((v) => (
                    <button
                      key={v}
                      onClick={() => setVibe(v)}
                      className={`p-4 border text-left transition-all font-semibold backdrop-blur-md ${
                        vibe === v ? "border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] shadow-sm" : "border-white/50 dark:border-white/10 text-gray-700 dark:text-gray-300 hover:border-white/80 dark:hover:border-white/20 bg-white/40 dark:bg-black/20"
                      }`}
                      style={{ borderRadius: '8px' }}
                    >
                      {v}
                    </button>
                  ))}
                </div>

                <div className="flex gap-4">
                  <button
                    className="flex-1 p-4 bg-white/40 dark:bg-black/20 text-gray-700 dark:text-gray-300 font-bold font-outfit text-lg transition-all hover:bg-white/60 dark:hover:bg-black/40 active:scale-[0.98] border border-white/50 dark:border-white/10 backdrop-blur-md"
                    style={{ borderRadius: '8px' }}
                    onClick={() => setWizardStep(1)}
                  >
                    Back
                  </button>
                  <button
                    className={`flex-1 p-4 font-bold font-outfit text-lg transition-all ${
                      vibe
                        ? "text-white shadow-md active:scale-[0.98] bg-gradient-to-r from-[#0066FF] to-[#0052cc]"
                        : "bg-white/40 dark:bg-black/20 text-gray-400 dark:text-gray-500 cursor-not-allowed border border-white/50 dark:border-white/10 backdrop-blur-md"
                    }`}
                    style={{ borderRadius: '8px' }}
                    onClick={() => {
                       if (!bio.trim()) {
                         setBio(`I run a ${businessCategory} business called ${businessName}. We want a ${vibe.toLowerCase()} vibe.`);
                       }
                       setWizardStep(3);
                    }}
                    disabled={!vibe}
                  >
                    Next: Details
                  </button>
                </div>
              </div>
            )}

            {wizardStep === 3 && (
              <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#f5f5f7] mb-2">Final Details</h1>
                <p className="text-gray-500 dark:text-[#a1a1a6] text-sm mb-8 leading-relaxed">
                  Review and add any extra details to help our AI generate the perfect store.
                </p>

                <label className="text-sm font-semibold text-gray-700 dark:text-[#a1a1a6] mb-2 block text-left">Your Business Details</label>
                <WithTooltip id="bio-input-tooltip" defaultText="Describe what you sell, your target audience, and the vibe of your brand.">
                  <textarea
                    id="bio-input"
                    className="w-full border border-white/50 dark:border-white/10 bg-white/40 dark:bg-black/20 backdrop-blur-md p-4 mb-8 focus:ring-2 focus:ring-[#0066FF]/50 focus:border-[#0066FF] outline-none transition-all resize-none text-[#1D1D1F] dark:text-[#f5f5f7] shadow-inner"
                    style={{ borderRadius: '8px' }}
                    value={bio}
                    onChange={(e) => setBio(e.target.value)}
                    placeholder="e.g. I run a mobile dog grooming service in Portland"
                    rows={6}
                  />
                </WithTooltip>

                <div className="flex gap-4">
                  <button
                    className="flex-1 p-4 bg-white/40 dark:bg-black/20 text-gray-700 dark:text-gray-300 font-bold font-outfit text-lg transition-all hover:bg-white/60 dark:hover:bg-black/40 active:scale-[0.98] border border-white/50 dark:border-white/10 backdrop-blur-md"
                    style={{ borderRadius: '8px' }}
                    onClick={() => setWizardStep(2)}
                  >
                    Back
                  </button>
                  <WithTooltip id="generate-btn-tooltip" defaultText="Our AI agents will analyze your description and build a ready-to-launch store for you.">
                    <button
                      id="generate-btn"
                      className={`flex-[2] p-4 font-bold font-outfit text-lg transition-all ${
                        bio.trim().length > 5
                          ? "text-white shadow-md active:scale-[0.98] bg-gradient-to-r from-[#0066FF] to-[#0052cc]"
                          : "bg-white/40 dark:bg-black/20 text-gray-400 dark:text-gray-500 cursor-not-allowed border border-white/50 dark:border-white/10 backdrop-blur-md"
                      }`}
                      style={{ borderRadius: '8px' }}
                      onClick={handleGenerate}
                      disabled={bio.trim().length <= 5}
                    >
                      Build Store
                    </button>
                  </WithTooltip>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    );
  }

  if (status === "generating") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:rounded-[16px] glass-container mac-glass-container backdrop-blur-xl bg-white/30 shadow-2xl">
           <div className="px-8 pt-20 pb-4 text-center">
              <h1 className="text-2xl font-extrabold font-outfit text-gray-900 mb-2">AI Architect</h1>
              <p className="text-sm text-gray-500 animate-pulse">Designing your custom storefront...</p>
           </div>
           <div className="flex-1 overflow-y-auto px-4">
              <SkeletonBlock />
              <SkeletonBlock />
              <SkeletonBlock />
           </div>
           {/* Abstract pulse overlay */}
           <div className="absolute inset-0 bg-blue-500/5 animate-pulse pointer-events-none" />
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter">
        <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col items-center overflow-x-hidden overflow-y-auto hide-scrollbar sm:rounded-[16px] glass-container mac-glass-container backdrop-blur-xl bg-white/30 shadow-2xl px-6 pt-12 pb-8">
          {/* Success Animation Background */}
          <div className="absolute top-0 left-0 w-full h-full bg-gradient-to-br from-green-50 via-white to-blue-50 -z-10 animate-fade-in" />

          <div className="w-20 h-20 bg-green-500 text-white rounded-full flex items-center justify-center mb-6 shadow-lg animate-bounce mt-8">
            <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
          </div>

          <h1 className="text-3xl font-extrabold font-outfit text-gray-900 mb-2 tracking-tight">You're Live!</h1>
          <p className="text-gray-500 mb-8 text-sm max-w-[240px]">Your business is now open to the world. Scan the code to see it.</p>

          <div className="mb-8 animate-fade-in" style={{ animationDelay: '300ms' }}>
            <QRCode value={liveUrl} />
          </div>

          {/* Growth Loop: Embeddable Storefront Widget */}
          <div className="w-full bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[16px] border border-white/50 dark:border-white/10 shadow-sm p-5 mb-4 text-left">
            <h2 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">Sell Anywhere 💻</h2>
            <p className="text-xs text-gray-500 dark:text-[#A1A1A6] mb-4">Embed your OHC storefront on your existing website, blog, or partner pages.</p>
            <div className="bg-white/60 dark:bg-black/30 backdrop-blur-sm border border-white/50 dark:border-white/10 rounded-[8px] p-3 relative">
                <pre className="text-[10px] text-[#1D1D1F] dark:text-[#F5F5F7] overflow-x-auto font-mono whitespace-pre-wrap leading-tight">
{`<div id="ohc-embed-root"></div>
<script src="https://ohc.store/embed.js" data-store="${tenantId}"></script>
<div style="text-align: center; margin-top: 8px; font-family: sans-serif; font-size: 11px;">
  <a href="https://ohc.store/join?ref=${tenantId}" target="_blank" style="color: #646b78; text-decoration: none;">Powered by <b>OHC</b></a>
</div>`}
                </pre>
                <button
                    onClick={() => {
                        const code = `<div id="ohc-embed-root"></div>\n<script src="https://ohc.store/embed.js" data-store="${tenantId}"></script>\n<div style="text-align: center; margin-top: 8px; font-family: sans-serif; font-size: 11px;">\n  <a href="https://ohc.store/join?ref=${tenantId}" target="_blank" style="color: #646b78; text-decoration: none;">Powered by <b>OHC</b></a>\n</div>`;
                        navigator.clipboard.writeText(code);
                        alert("Copied embed code to clipboard!");
                    }}
                    className="absolute top-2 right-2 bg-white/70 dark:bg-black/50 text-[#1D1D1F] dark:text-[#F5F5F7] border border-white/50 dark:border-white/10 px-2 py-1 rounded-[8px] text-[10px] font-semibold hover:bg-white/90 dark:hover:bg-black/70 transition-colors backdrop-blur-sm"
                >
                    Copy
                </button>
            </div>
          </div>

          <div className="w-full bg-gray-50 p-3 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline shrink-0">Copy</button>
          </div>

          {/* Growth Loop 1: Acquisition (Get your first customer) */}
          <div className="w-full bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[16px] border border-white/50 dark:border-white/10 shadow-sm p-5 mb-4 text-left">
            <h2 className="text-lg font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-1">Get your first customer 🚀</h2>
            <p className="text-xs text-gray-500 dark:text-[#A1A1A6] mb-4">Share your new store with friends and family to get early sales.</p>

            <div className="flex gap-3">
              <a
                href={`https://wa.me/?text=${encodeURIComponent(`Check out my new store: ${liveUrl}`)}`}
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 bg-[#25D366] text-white flex items-center justify-center gap-2 p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
              >
                <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                WhatsApp
              </a>
              <a
                href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`Just launched my new business on OHC! Check it out: ${liveUrl}`)}`}
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 bg-black text-white flex items-center justify-center gap-2 p-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
              >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                Share
              </a>
            </div>
          </div>


          {/* Generative Visibility Score */}
          <div className="w-full bg-blue-50/50 dark:bg-blue-900/20 backdrop-blur-md border border-[#0066FF]/30 dark:border-[#0066FF]/20 shadow-sm p-5 mb-6 text-left rounded-[16px]">
            <h2 className="text-lg font-bold font-outfit text-[#0066FF] dark:text-blue-300 mb-1">Generative Visibility Score (GEO)</h2>
            <p className="text-xs text-blue-700 dark:text-blue-200 mb-4">Improve how LLM crawlers like ChatGPT or Gemini see your business.</p>

            {geoScore === null ? (
              <button
                onClick={handleGeoAnalysis}
                className="w-full bg-blue-600 text-white font-semibold py-2 rounded-xl text-sm shadow-sm hover:bg-blue-700 transition-all"
              >
                Analyze Visibility
              </button>
            ) : (
              <div className="animate-fade-in">
                <div className="flex items-end gap-2 mb-3">
                  <span className="text-3xl font-black text-blue-900">{geoScore}</span>
                  <span className="text-sm font-medium text-blue-600 pb-1">/ 100</span>
                </div>
                {geoRecs.length > 0 && (
                  <ul className="text-xs text-blue-800 space-y-1 mb-4 list-disc pl-4">
                    {geoRecs.map((r, idx) => <li key={idx}>{r}</li>)}
                  </ul>
                )}
                <button
                  onClick={handleAutoSeo}
                  disabled={seoApplied}
                  className={`w-full font-semibold py-2 rounded-xl text-sm shadow-sm transition-all ${
                    seoApplied
                    ? "bg-green-100 text-green-700 cursor-not-allowed border border-green-200"
                    : "bg-blue-600 text-white hover:bg-blue-700"
                  }`}
                >
                  {seoApplied ? "Recommendations Applied ✓" : "Auto-Apply SEO Metadata"}
                </button>
              </div>
            )}
          </div>

          <button
            className="w-full bg-white/40 dark:bg-black/20 text-[#1D1D1F] dark:text-[#F5F5F7] font-bold p-4 rounded-[8px] active:scale-[0.98] transition-all hover:bg-white/60 dark:hover:bg-black/40 border border-white/50 dark:border-white/10 backdrop-blur-md"
            onClick={() => setStatus("idle")}
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 dark:bg-[#000] font-inter">
      <div className="relative w-[375px] h-[812px] sm:h-[812px] min-h-[100dvh] sm:min-h-auto flex flex-col overflow-hidden sm:rounded-[16px] glass-container mac-glass-container backdrop-blur-xl bg-white/30 shadow-2xl">

        {/* Draft Preview Header */}
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Mobile Editor</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto pb-32 pt-8 hide-scrollbar">
          {blocks.map((b, i) => (
            <DraggableBlock
              key={i}
              isSelected={selectedBlockIndex === i}
              onClick={() => {
                setSelectedBlockIndex(i);
                setIsActionSheetOpen(true);
              }}
              onDragStart={(e) => {
                setDraggedIndex(i);
                if ('touches' in e) {
                  setStartY(e.touches[0].clientY);
                } else if ('clientY' in e) {
                  setStartY((e as React.DragEvent).clientY);
                }
                setSelectedBlockIndex(i);
              }}
              onDragOver={(e) => {
                if (draggedIndex === null) return;
                let currentY = 0;
                if ('touches' in e) {
                  currentY = e.touches[0].clientY;
                } else if ('clientY' in e) {
                  currentY = (e as React.DragEvent).clientY;
                } else {
                  return;
                }
                const diff = currentY - startY;
                if (Math.abs(diff) > 50) {
                  const newIndex = diff > 0 ? i + 1 : i - 1;
                  if (newIndex >= 0 && newIndex < blocks.length && newIndex !== draggedIndex) {
                    const newBlocks = [...blocks];
                    const [removed] = newBlocks.splice(draggedIndex, 1);
                    newBlocks.splice(newIndex, 0, removed);
                    setBlocks(newBlocks);
                    setDraggedIndex(newIndex);
                    setStartY(currentY);
                  }
                }
              }}
              onDragEnd={() => {
                setDraggedIndex(null);
              }}
            >
              <SmartBlock {...b} />
            </DraggableBlock>
          ))}
          {!isPremium && <SmartBlock type="PoweredBy" props={{ tenantId }} />}
        </div>

        {/* Action Sheet for Editing Blocks */}
        <ActionSheet
          isOpen={isActionSheetOpen}
          onClose={() => setIsActionSheetOpen(false)}
          title={`Edit ${blocks[selectedBlockIndex || 0]?.type} Block`}
        >
          <div className="space-y-4 font-inter">
            {blocks[selectedBlockIndex || 0]?.type === 'Hero' && (
              <>
                <label className="text-xs font-bold text-gray-400 uppercase">Headline</label>
                <input
                  type="text"
                  className="w-full p-4 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[8px] border border-white/50 dark:border-white/10 focus:ring-2 focus:ring-[#0066FF] outline-none transition-all text-[#1D1D1F] dark:text-[#F5F5F7] shadow-inner"
                  value={blocks[selectedBlockIndex || 0]?.props.headline}
                  onChange={(e) => {
                    const newBlocks = [...blocks];
                    newBlocks[selectedBlockIndex || 0].props.headline = e.target.value;
                    setBlocks(newBlocks);
                  }}
                />
                <div className="grid grid-cols-2 gap-3 mt-4">
                  <button className="p-4 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[8px] border border-white/50 dark:border-white/10 text-sm font-bold flex flex-col items-center gap-2 hover:bg-white/60 dark:hover:bg-black/40">
                    <span>🖼️</span>
                    <span>Upload Photo</span>
                  </button>
                  <button className="p-4 bg-white/40 dark:bg-black/20 backdrop-blur-md rounded-[8px] border border-white/50 dark:border-white/10 text-sm font-bold flex flex-col items-center gap-2 hover:bg-white/60 dark:hover:bg-black/40">
                    <span>✨</span>
                    <span>AI Generate</span>
                  </button>
                </div>
              </>
            )}
            {blocks[selectedBlockIndex || 0]?.type !== 'Hero' && (
              <p className="text-sm text-gray-500 italic">Context-aware editing for {blocks[selectedBlockIndex || 0]?.type} coming soon...</p>
            )}
            <button
              onClick={() => setIsActionSheetOpen(false)}
              className="w-full bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[8px] font-bold mt-4 shadow-md hover:shadow-lg active:scale-[0.98] transition-all"
            >
              Save Changes
            </button>
          </div>
        </ActionSheet>

        {/* Bottom Action Bar */}
        <div className="absolute bottom-0 w-full p-4 glass-container mac-glass-container border-t border-white/40 dark:border-white/10 z-50">
          <div className="flex gap-3 mb-2">
            <button className="flex-1 py-2 text-sm font-medium text-gray-600 bg-white/50 dark:bg-black/20 backdrop-blur-md border border-white/40 dark:border-white/10 rounded-[8px]">Change Vibe</button>
            {!isPremium && (
              <button
                className="flex-1 py-2 text-sm font-medium text-[#0066FF] bg-blue-50/50 dark:bg-blue-900/30 backdrop-blur-md border border-[#0066FF]/30 rounded-[8px]"
                onClick={() => setShowUpgradeModal(true)}
              >
                Remove Branding ✨
              </button>
            )}
          </div>
          <WithTooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
            <button
              id="launch-btn"
              className="w-full bg-gradient-to-r from-[#34C759] to-[#2eb350] text-white p-4 rounded-[8px] font-bold shadow-md hover:shadow-lg active:scale-[0.98] transition-all flex justify-center items-center gap-2"
              onClick={handleLaunch}
            >
              <span>1-Tap Launch</span>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </button>
          </WithTooltip>
        </div>

        {/* Upgrade Modal */}
        {showUpgradeModal && (
          <div className="absolute inset-0 bg-black/40 backdrop-blur-sm z-[60] flex flex-col justify-end">
            <div className="bg-white/90 dark:bg-[#16161a]/90 backdrop-blur-xl w-full rounded-t-[16px] p-6 shadow-2xl animate-slide-up pb-10 border-t border-white/40 dark:border-white/10">
              <div className="flex justify-between items-start mb-4">
                <div className="w-12 h-12 bg-gradient-to-br from-yellow-100 to-yellow-200 rounded-[12px] flex items-center justify-center text-2xl shadow-inner border border-yellow-300">
                  👑
                </div>
                <button
                  onClick={() => setShowUpgradeModal(false)}
                  className="p-2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 rounded-full hover:bg-white/40 dark:hover:bg-black/40 transition-colors"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Upgrade to Premium</h2>
              <p className="text-gray-500 dark:text-[#A1A1A6] mb-6 font-inter text-sm leading-relaxed">
                Unlock white-labeling, custom domains, and advanced analytics to grow your business faster.
              </p>

              <div className="space-y-3 mb-6 font-inter text-sm">
                <div className="flex items-center gap-3">
                  <svg className="w-5 h-5 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                  <span className="text-gray-700 dark:text-gray-300">Remove "Powered by OHC" footer</span>
                </div>
                <div className="flex items-center gap-3">
                  <svg className="w-5 h-5 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                  <span className="text-gray-700 dark:text-gray-300">Connect a custom domain</span>
                </div>
                <div className="flex items-center gap-3">
                  <svg className="w-5 h-5 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                  <span className="text-gray-700 dark:text-gray-300">Priority AI scheduling</span>
                </div>
              </div>

              <button
                onClick={() => {
                  setIsPremium(true);
                  setShowUpgradeModal(false);
                }}
                className="w-full bg-gradient-to-r from-gray-900 to-black dark:from-gray-100 dark:to-white dark:text-black text-white font-bold p-4 rounded-[8px] shadow-lg active:scale-[0.98] transition-all flex justify-between items-center"
              >
                <span>Upgrade Now</span>
                <span className="font-normal opacity-80">$15 / mo</span>
              </button>
            </div>
          </div>
        )}
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes slideUp {
          from { transform: translateY(100%); opacity: 0; }
          to { transform: translateY(0); opacity: 1; }
        }
        .animate-slide-up { animation: slideUp 300ms cubic-bezier(0.4, 0, 0.2, 1); }
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism { background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; }
        @media (prefers-color-scheme: dark) {
          .glassmorphism { background: rgba(22, 22, 26, 0.7); border: 1px solid rgba(255, 255, 255, 0.1); }
        }
      `}} />
    </div>
  );
}
