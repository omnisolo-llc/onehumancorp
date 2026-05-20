"use client";

import { useState } from "react";
import { SmartBlock } from "./components";
import { Tooltip, useWalkthrough } from "../../components/help";

export default function BuilderPage() {
  const [bio, setBio] = useState("");
  const [businessName, setBusinessName] = useState("");
  const [businessCategory, setBusinessCategory] = useState("");
  const [vibe, setVibe] = useState("");
  const [wizardStep, setWizardStep] = useState(1);
  const [blocks, setBlocks] = useState<any[]>([]);
  const [status, setStatus] = useState<"idle" | "generating" | "draft" | "live">("idle");
  const [liveUrl, setLiveUrl] = useState("");
  const { startWalkthrough } = useWalkthrough();

  // GEO UI State
  const [geoScore, setGeoScore] = useState<number | null>(null);
  const [geoRecs, setGeoRecs] = useState<string[]>([]);
  const [seoApplied, setSeoApplied] = useState(false);

  // Growth Loop: Soft Paywall State
  const [isPremium, setIsPremium] = useState(false);
  const [showUpgradeModal, setShowUpgradeModal] = useState(false);

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

      setBlocks(newBlocks);
      setStatus("draft");
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
                    b.type === 'Referral' ? 'ReferralBlock' : b.type,
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

  if (status === "idle") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200"
             style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>

          <div className="px-8 pt-12 pb-4">
             <div className="flex justify-between mb-8">
               {[1, 2, 3].map(step => (
                 <div key={step} className={`h-1.5 flex-1 mx-1 rounded-full ${step <= wizardStep ? 'bg-[#0071E3]' : 'bg-gray-200'}`} style={{ transition: 'all 250ms cubic-bezier(0.4, 0, 0.2, 1)' }} />
               ))}
             </div>
          </div>

          <div className="px-8 pb-8 flex flex-col flex-1 justify-start overflow-y-auto">
            {wizardStep === 1 && (
              <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Let's build your store</h1>
                <p className="text-gray-500 text-sm mb-8 leading-relaxed">
                  Start with the basics. What's your business called, and what do you do?
                </p>

                <label className="text-sm font-semibold text-gray-700 mb-2 block">Business Name</label>
                <input
                  type="text"
                  className="w-full border border-gray-300 p-4 mb-6 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800"
                  style={{ borderRadius: '8px' }}
                  value={businessName}
                  onChange={(e) => setBusinessName(e.target.value)}
                  placeholder="e.g. Acme Corp"
                />

                <label className="text-sm font-semibold text-gray-700 mb-2 block">Category</label>
                <input
                  type="text"
                  className="w-full border border-gray-300 p-4 mb-8 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all text-gray-800"
                  style={{ borderRadius: '8px' }}
                  value={businessCategory}
                  onChange={(e) => setBusinessCategory(e.target.value)}
                  placeholder="e.g. Retail, Consulting, Tech"
                />

                <button
                  className={`w-full p-4 font-bold font-outfit text-lg transition-all ${
                    businessName.trim() && businessCategory.trim()
                      ? "text-white shadow-md active:scale-[0.98]"
                      : "bg-gray-100 text-gray-400 cursor-not-allowed"
                  }`}
                  style={{ borderRadius: '8px', background: (businessName.trim() && businessCategory.trim()) ? '#0071E3' : '' }}
                  onClick={() => setWizardStep(2)}
                  disabled={!businessName.trim() || !businessCategory.trim()}
                >
                  Next: Choose Vibe
                </button>
              </div>
            )}

            {wizardStep === 2 && (
              <div className="animate-fade-in" style={{ animation: 'fadeIn 250ms cubic-bezier(0.4, 0, 0.2, 1)' }}>
                <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Select Your Vibe</h1>
                <p className="text-gray-500 text-sm mb-8 leading-relaxed">
                  How should your store feel? Our AI agents will match this tone.
                </p>

                <div className="grid gap-4 mb-8">
                  {['Professional', 'Friendly', 'Energetic', 'Minimalist'].map((v) => (
                    <button
                      key={v}
                      onClick={() => setVibe(v)}
                      className={`p-4 border text-left transition-all font-semibold ${
                        vibe === v ? "border-[#0071E3] bg-blue-50 text-[#0071E3]" : "border-gray-200 text-gray-700 hover:border-gray-300"
                      }`}
                      style={{ borderRadius: '8px' }}
                    >
                      {v}
                    </button>
                  ))}
                </div>

                <div className="flex gap-4">
                  <button
                    className="flex-1 p-4 bg-gray-100 text-gray-700 font-bold font-outfit text-lg transition-all hover:bg-gray-200 active:scale-[0.98]"
                    style={{ borderRadius: '8px' }}
                    onClick={() => setWizardStep(1)}
                  >
                    Back
                  </button>
                  <button
                    className={`flex-1 p-4 font-bold font-outfit text-lg transition-all ${
                      vibe
                        ? "text-white shadow-md active:scale-[0.98]"
                        : "bg-gray-100 text-gray-400 cursor-not-allowed"
                    }`}
                    style={{ borderRadius: '8px', background: vibe ? '#0071E3' : '' }}
                    onClick={() => {
                       // Pre-fill bio based on earlier steps if empty
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
                <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Final Details</h1>
                <p className="text-gray-500 text-sm mb-8 leading-relaxed">
                  Review and add any extra details to help our AI generate the perfect store.
                </p>

                <label className="text-sm font-semibold text-gray-700 mb-2 block">Your Business Details</label>
                <Tooltip id="bio-input-tooltip" defaultText="Describe what you sell, your target audience, and the vibe of your brand.">
                  <textarea
                    id="bio-input"
                    className="w-full border border-gray-300 p-4 mb-8 focus:ring-2 focus:ring-[#0071E3] focus:border-[#0071E3] outline-none transition-all resize-none text-gray-800"
                    style={{ borderRadius: '8px' }}
                    value={bio}
                    onChange={(e) => setBio(e.target.value)}
                    placeholder="e.g. I run a mobile dog grooming service in Portland"
                    rows={6}
                  />
                </Tooltip>

                <div className="flex gap-4">
                  <button
                    className="flex-1 p-4 bg-gray-100 text-gray-700 font-bold font-outfit text-lg transition-all hover:bg-gray-200 active:scale-[0.98]"
                    style={{ borderRadius: '8px' }}
                    onClick={() => setWizardStep(2)}
                  >
                    Back
                  </button>
                  <Tooltip id="generate-btn-tooltip" defaultText="Our AI agents will analyze your description and build a ready-to-launch store for you.">
                    <button
                      id="generate-btn"
                      className={`flex-[2] p-4 font-bold font-outfit text-lg transition-all ${
                        bio.trim().length > 5
                          ? "text-white shadow-md active:scale-[0.98]"
                          : "bg-gray-100 text-gray-400 cursor-not-allowed"
                      }`}
                      style={{ borderRadius: '8px', background: (bio.trim().length > 5) ? '#0071E3' : '' }}
                      onClick={handleGenerate}
                      disabled={bio.trim().length <= 5}
                    >
                      Build Store
                    </button>
                  </Tooltip>
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
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col items-center justify-center border-x border-gray-200">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-gray-900 mb-6"></div>
          <p className="text-gray-600 font-medium animate-pulse text-center px-8">
            The Promoter is picking colors and building your menu...
          </p>
        </div>
      </div>
    );
  }

  if (status === "live") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col items-center border-x border-gray-200 text-center px-6 relative overflow-y-auto hide-scrollbar pt-12 pb-8">
          <div className="absolute top-0 left-0 w-full h-64 bg-gradient-to-br from-green-50 to-white -z-10" />

          <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-4 shadow-sm">
            <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">You're Live!</h1>
          <p className="text-gray-500 mb-6 text-sm">Your automated storefront is successfully published.</p>

          <div className="w-full bg-gray-50 p-3 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline shrink-0">Copy</button>
          </div>

          {/* Growth Loop 1: Acquisition (Get your first customer) */}
          <div className="w-full bg-white rounded-2xl border border-gray-200 shadow-sm p-5 mb-4 text-left">
            <h2 className="text-lg font-bold font-outfit text-gray-900 mb-1">Get your first customer 🚀</h2>
            <p className="text-xs text-gray-500 mb-4">Share your new store with friends and family to get early sales.</p>

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
          <div className="w-full bg-blue-50 border border-blue-200 shadow-sm p-5 mb-6 text-left rounded-2xl">
            <h2 className="text-lg font-bold font-outfit text-blue-900 mb-1">Generative Visibility Score (GEO)</h2>
            <p className="text-xs text-blue-700 mb-4">Improve how LLM crawlers like ChatGPT or Gemini see your business.</p>

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
            className="w-full bg-gray-100 text-gray-800 font-bold p-4 rounded-xl active:scale-[0.98] transition-all hover:bg-gray-200"
            onClick={() => setStatus("idle")}
          >
            Go to Dashboard
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
      <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col relative border-x border-gray-200 overflow-hidden">

        {/* Draft Preview Header */}
        <div className="absolute top-0 left-0 w-full bg-black/80 backdrop-blur-md text-white text-xs py-2 text-center font-medium z-50 flex justify-between px-4 items-center">
          <span>Preview Mode</span>
          <span className="bg-white/20 px-2 py-0.5 rounded">375px</span>
        </div>

        {/* Content Area */}
        <div className="flex-1 overflow-y-auto pb-24 pt-8 hide-scrollbar">
          {blocks.map((b, i) => (
            <SmartBlock key={i} {...b} />
          ))}
          {!isPremium && <SmartBlock type="PoweredBy" props={{}} />}
        </div>

        {/* Bottom Action Bar */}
        <div className="absolute bottom-0 w-full p-4 bg-white/90 backdrop-blur-md border-t border-gray-200 z-50">
          <div className="flex gap-3 mb-2">
            <button className="flex-1 py-2 text-sm font-medium text-gray-600 bg-gray-100 rounded-lg">Change Vibe</button>
            {!isPremium && (
              <button
                className="flex-1 py-2 text-sm font-medium text-blue-600 bg-blue-50 rounded-lg border border-blue-100"
                onClick={() => setShowUpgradeModal(true)}
              >
                Remove Branding ✨
              </button>
            )}
          </div>
          <Tooltip id="launch-btn-tooltip" defaultText="Launch your storefront immediately to a live URL.">
            <button
              id="launch-btn"
              className="w-full bg-blue-600 text-white p-4 rounded-xl font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
              onClick={handleLaunch}
            >
              <span>1-Tap Launch</span>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </button>
          </Tooltip>
        </div>

        {/* Upgrade Modal */}
        {showUpgradeModal && (
          <div className="absolute inset-0 bg-black/60 z-[60] flex flex-col justify-end">
            <div className="bg-white w-full rounded-t-3xl p-6 shadow-2xl animate-slide-up pb-10">
              <div className="flex justify-between items-start mb-4">
                <div className="w-12 h-12 bg-gradient-to-br from-yellow-100 to-yellow-200 rounded-xl flex items-center justify-center text-2xl shadow-inner">
                  👑
                </div>
                <button
                  onClick={() => setShowUpgradeModal(false)}
                  className="p-2 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100"
                >
                  <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                </button>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Upgrade to Premium</h2>
              <p className="text-gray-500 mb-6 font-inter text-sm leading-relaxed">
                Unlock white-labeling, custom domains, and advanced analytics to grow your business faster.
              </p>

              <div className="space-y-3 mb-6 font-inter text-sm">
                <div className="flex items-center gap-3">
                  <svg className="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                  <span className="text-gray-700">Remove "Powered by OHC" footer</span>
                </div>
                <div className="flex items-center gap-3">
                  <svg className="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                  <span className="text-gray-700">Connect a custom domain</span>
                </div>
                <div className="flex items-center gap-3">
                  <svg className="w-5 h-5 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                  <span className="text-gray-700">Priority AI scheduling</span>
                </div>
              </div>

              <button
                onClick={() => {
                  setIsPremium(true);
                  setShowUpgradeModal(false);
                }}
                className="w-full bg-gradient-to-r from-gray-900 to-black text-white font-bold p-4 rounded-xl shadow-lg active:scale-[0.98] transition-all flex justify-between items-center"
              >
                <span>Upgrade Now</span>
                <span className="font-normal opacity-80">$15 / mo</span>
              </button>
            </div>
          </div>
        )}
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        .hide-scrollbar::-webkit-scrollbar { display: none; }
        .hide-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism { background: rgba(255, 255, 255, 0.65); backdrop-filter: blur(30px) saturate(210%); -webkit-backdrop-filter: blur(30px) saturate(210%); border: 1px solid rgba(255, 255, 255, 0.4); border-radius: 16px; }
      `}} />
    </div>
  );
}
