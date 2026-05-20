"use client";

import { useState } from "react";
import { SmartBlock } from "./components";

export default function BuilderPage() {
  const [bio, setBio] = useState("");
  const [blocks, setBlocks] = useState<any[]>([]);
  const [status, setStatus] = useState<"idle" | "generating" | "draft" | "live">("idle");
  const [liveUrl, setLiveUrl] = useState("");

  // Growth Loop: Soft Paywall State
  const [isPremium, setIsPremium] = useState(false);
  const [showUpgradeModal, setShowUpgradeModal] = useState(false);

  const handleGenerate = async () => {
    setStatus("generating");

    try {
      const response = await fetch('/builder/api', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ bio })
      });

      const data = await response.json();
      setBlocks(data.blocks);
      setStatus("draft");
    } catch (error) {
      console.error("Failed to generate storefront", error);
      setStatus("idle");
    }
  };

  const handleLaunch = () => {
    // Simulate background provisioning of subdomain and SSL
    setTimeout(() => {
      setStatus("live");
      const subdomain = bio.toLowerCase().replace(/[^a-z0-9]/g, '').substring(0, 10);
      setLiveUrl(`https://${subdomain || 'myshop'}.ohc.store`);
    }, 1500);
  };

  if (status === "idle") {
    return (
      <div className="flex flex-col items-center justify-center h-screen bg-gray-50 font-inter">
        <div className="w-[375px] h-[812px] bg-white shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
          <div className="p-8 flex flex-col flex-1 justify-center">
            <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Welcome to OHC Smart Builder</h1>
            <p className="text-gray-500 text-sm mb-8 leading-relaxed">
              Tell us about your business in a few words, and we'll magically generate your storefront in seconds.
            </p>

            <label className="text-sm font-semibold text-gray-700 mb-2 block">Your Business</label>
            <textarea
              className="w-full border border-gray-300 p-4 rounded-xl mb-6 focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all resize-none text-gray-800"
              value={bio}
              onChange={(e) => setBio(e.target.value)}
              placeholder="e.g. I run a mobile dog grooming service in Portland"
              rows={4}
            />

            <button
              className={`w-full p-4 rounded-xl font-bold font-outfit text-lg transition-all ${
                bio.trim().length > 5
                  ? "bg-gray-900 text-white hover:bg-gray-800 shadow-md active:scale-[0.98]"
                  : "bg-gray-100 text-gray-400 cursor-not-allowed"
              }`}
              onClick={handleGenerate}
              disabled={bio.trim().length <= 5}
            >
              Build My Storefront
            </button>
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
        <div className="w-[375px] h-[812px] bg-white shadow-2xl flex flex-col items-center justify-center border-x border-gray-200 text-center px-6 relative overflow-hidden">
          <div className="absolute top-0 left-0 w-full h-full bg-gradient-to-br from-green-50 to-white -z-10" />
          <div className="w-20 h-20 bg-green-100 text-green-600 rounded-full flex items-center justify-center mb-6 shadow-sm">
            <svg className="w-10 h-10" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">You're Live!</h1>
          <p className="text-gray-500 mb-8">Your automated storefront is successfully published and ready for customers.</p>

          <div className="w-full bg-gray-50 p-4 rounded-xl border border-gray-100 mb-6 flex items-center justify-between">
            <span className="text-sm text-gray-700 truncate mr-2 font-medium">{liveUrl}</span>
            <button className="text-blue-600 font-semibold text-sm hover:underline">Copy</button>
          </div>

          <div className="w-full text-left mb-6">
            <h3 className="text-sm font-bold font-outfit text-gray-900 mb-2">Get your first customer</h3>
            <div className="flex gap-3">
              <a href={`https://twitter.com/intent/tweet?text=${encodeURIComponent('Check out my new storefront!')}&url=${encodeURIComponent(liveUrl)}`} target="_blank" rel="noopener noreferrer" className="flex-1 bg-blue-50 text-blue-600 border border-blue-100 font-semibold py-2 rounded-lg flex items-center justify-center gap-2 text-sm hover:bg-blue-100 transition-colors">
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/></svg>
                Twitter
              </a>
              <a href={`https://wa.me/?text=${encodeURIComponent('Check out my new storefront! ' + liveUrl)}`} target="_blank" rel="noopener noreferrer" className="flex-1 bg-green-50 text-green-700 border border-green-100 font-semibold py-2 rounded-lg flex items-center justify-center gap-2 text-sm hover:bg-green-100 transition-colors">
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M12.031 0C5.385 0 0 5.385 0 12.031c0 2.651.854 5.114 2.33 7.15L.82 24l5.042-1.46c1.944 1.258 4.26 1.986 6.74 1.986 6.646 0 12.031-5.385 12.031-12.031C24.633 5.385 19.248 0 12.031 0zm0 22.42c-2.316 0-4.484-.73-6.26-1.996l-.45-.316-3.666 1.06.98-3.548-.346-.546C1.177 15.347.41 13.722.41 12.03.41 5.614 5.614.41 12.03.41c6.417 0 11.62 5.203 11.62 11.62 0 6.416-5.203 11.62-11.62 11.62zm6.36-8.73c-.347-.174-2.054-1.014-2.373-1.13-.318-.116-.55-.174-.78.174-.23.348-.9 1.13-1.1 1.362-.2.23-.4.26-.748.087-2.12-.99-3.5-2.066-4.836-4.343-.23-.347.086-.347.434-.694.348-.348.695-.695.81-.926.116-.23.058-.434-.03-.608-.087-.174-.78-1.88-.11-2.576.67-.696 2.08-.174 2.89.58 1.928 1.796 2.448 3.012 3.144 4.546.116.26.058.492-.058.695-.116.203-.434.318-.78.492z"/></svg>
                WhatsApp
              </a>
            </div>
          </div>

          <div className="w-full bg-gradient-to-r from-yellow-50 to-orange-50 p-4 rounded-xl border border-yellow-100 mb-8 text-left">
            <div className="flex items-center gap-2 mb-2">
              <span className="text-xl">🎁</span>
              <h3 className="font-bold font-outfit text-yellow-900">Invite & Get Premium Free</h3>
            </div>
            <p className="text-xs text-yellow-800 mb-3 font-inter leading-relaxed">Know another business owner? Share OHC and both get 3 months of Premium when they launch.</p>
            <button
              className="w-full bg-white text-yellow-900 border border-yellow-200 font-semibold py-2 rounded-lg text-sm hover:bg-yellow-100 transition-colors"
              onClick={() => alert("Copied referral link: https://ohc.store/ref/" + liveUrl.split('//')[1].split('.')[0])}
            >
              Get Referral Link
            </button>
          </div>

          <button
            className="w-full bg-gray-900 text-white font-bold p-4 rounded-xl shadow-md active:scale-[0.98]"
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
          <button
            className="w-full bg-blue-600 text-white p-4 rounded-xl font-bold shadow-lg hover:bg-blue-700 active:scale-[0.98] transition-all flex justify-center items-center gap-2"
            onClick={handleLaunch}
          >
            <span>1-Tap Launch</span>
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
          </button>
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
        .glassmorphism { background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(10px); -webkit-backdrop-filter: blur(10px); border: 1px solid rgba(255, 255, 255, 0.2); }
      `}} />
    </div>
  );
}
