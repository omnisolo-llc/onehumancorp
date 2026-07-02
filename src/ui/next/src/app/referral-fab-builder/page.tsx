"use client";

import React, { useState, useEffect, useRef } from "react";
import Head from "next/head";

export default function ReferralFabBuilder() {
  const [reward, setReward] = useState("$10");
  const [themeColor, setThemeColor] = useState("#2563eb");
  const [tenantId, setTenantId] = useState("my-business");
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isCopied, setIsCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== "undefined") {
      const storedTenant = localStorage.getItem("tenant") || "my-business";
      setTenantId(storedTenant);
      setHasPro(localStorage.getItem("has_pro") === "true");
    }
    document.title = "Referral FAB Builder | OHC";
  }, []);

  const handleBrandingToggle = () => {
    if (!removeBranding) {
      if (!hasPro) {
        setShowPaywall(true);
      } else {
        setRemoveBranding(true);
      }
    } else {
      setRemoveBranding(false);
    }
  };

  const handleUpgrade = () => {
    setRemoveBranding(true);
    setHasPro(true);
    if (typeof localStorage !== "undefined") {
      localStorage.setItem("has_pro", "true");
    }
    setShowPaywall(false);
  };

  const embedUrl = `https://ohc.app/api/v1/growth/referral-fab/embed?tenant=${encodeURIComponent(tenantId)}&reward=${encodeURIComponent(reward)}&themeColor=${encodeURIComponent(themeColor)}&removeBranding=${removeBranding}`;

  const generatedCode = `
<!-- OHC Referral FAB -->
<script src="${embedUrl}"></script>
  `.trim();

  const handleCopyCode = () => {
    navigator.clipboard.writeText(generatedCode);
    setIsCopied(true);
    setTimeout(() => setIsCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      <Head>
        <title>Referral FAB Builder | OHC</title>
      </Head>

      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Referral FAB Builder</h1>
        <button
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors"
          onClick={() => window.history.back()}
        >
          Back to Dashboard
        </button>
      </header>

      <main className="flex-1 p-6 md:p-10 max-w-6xl mx-auto w-full grid grid-cols-1 lg:grid-cols-2 gap-10">

        {/* Editor Section */}
        <div className="space-y-6">
          <div className="bg-white/80 backdrop-blur-xl border border-gray-100 rounded-3xl p-8 shadow-sm">
            <h2 className="text-xl font-semibold mb-6">Customize Your Floating Action Button</h2>

            <div className="space-y-5">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Reward (e.g. "$10", "20% Off")</label>
                <input
                  type="text"
                  value={reward}
                  onChange={(e) => setReward(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 min-h-[44px]"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Theme Color</label>
                <div className="flex gap-2 flex-wrap">
                  {['#2563eb', '#16a34a', '#dc2626', '#9333ea', '#ea580c', '#000000'].map((c) => (
                    <button
                      key={c}
                      onClick={() => setThemeColor(c)}
                      className={`w-10 h-10 rounded-full border-2 ${themeColor === c ? 'border-gray-900' : 'border-transparent'}`}
                      style={{ backgroundColor: c }}
                      aria-label={`Select color ${c}`}
                    />
                  ))}
                  <input
                    type="color"
                    value={themeColor}
                    onChange={(e) => setThemeColor(e.target.value)}
                    className="w-10 h-10 rounded-full cursor-pointer border-0 p-0"
                    aria-label="Custom color picker"
                  />
                </div>
              </div>

              <div className="flex items-center justify-between pt-4 border-t border-gray-100">
                <div>
                  <div className="font-medium text-gray-900">Remove Branding</div>
                  <div className="text-sm text-gray-500">Hide "Powered by OHC"</div>
                </div>
                <button
                  role="switch"
                  aria-checked={removeBranding}
                  onClick={handleBrandingToggle}
                  className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${removeBranding ? 'bg-indigo-600' : 'bg-gray-200'}`}
                >
                  <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${removeBranding ? 'translate-x-6' : 'translate-x-1'}`} />
                </button>
              </div>

            </div>
          </div>

          {/* Code Section */}
          <div className="bg-gray-900 rounded-3xl p-8 shadow-sm text-white">
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-lg font-medium">Embed Code</h2>
              <button
                onClick={handleCopyCode}
                className="px-4 py-2 bg-white/10 hover:bg-white/20 rounded-xl text-sm font-medium transition-colors min-h-[44px]"
              >
                {isCopied ? "Copied!" : "Copy Code"}
              </button>
            </div>
            <pre className="text-sm font-mono text-gray-300 overflow-x-auto p-4 bg-black/50 rounded-xl">
              <code>{generatedCode}</code>
            </pre>
            <p className="mt-4 text-sm text-gray-400">
              Paste this script tag right before the closing &lt;/body&gt; tag of your website.
            </p>
          </div>
        </div>

        {/* Preview Section */}
        <div className="bg-white/50 backdrop-blur-3xl rounded-[40px] border-[8px] border-white/60 shadow-2xl overflow-hidden relative flex flex-col h-[600px] lg:h-auto min-h-[600px]">
           {/* Website Background */}
           <div className="bg-gray-100 h-12 flex items-center px-4 gap-2 border-b border-gray-200">
             <div className="w-3 h-3 rounded-full bg-red-400"></div>
             <div className="w-3 h-3 rounded-full bg-yellow-400"></div>
             <div className="w-3 h-3 rounded-full bg-green-400"></div>
             <div className="ml-4 bg-white rounded flex-1 h-6 max-w-[200px]"></div>
           </div>

           <div className="p-8 flex-1">
             <div className="w-1/2 h-8 bg-gray-200 rounded mb-4"></div>
             <div className="w-full h-4 bg-gray-200 rounded mb-2"></div>
             <div className="w-full h-4 bg-gray-200 rounded mb-2"></div>
             <div className="w-3/4 h-4 bg-gray-200 rounded mb-12"></div>

             <div className="grid grid-cols-3 gap-4">
               <div className="h-32 bg-gray-200 rounded-xl"></div>
               <div className="h-32 bg-gray-200 rounded-xl"></div>
               <div className="h-32 bg-gray-200 rounded-xl"></div>
             </div>
           </div>

           {/* Live Preview of the FAB */}
           <div className="absolute bottom-6 right-6 flex flex-col items-end group">
             {/* The FAB itself */}
             <div
               className="w-14 h-14 rounded-full shadow-lg flex items-center justify-center cursor-pointer transition-transform hover:scale-105 text-white"
               style={{ backgroundColor: themeColor }}
             >
                <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v13m0-13V6a2 2 0 112 2h-2zm0 0V5.5A2.5 2.5 0 109.5 8H12zm-7 4h14M5 12a2 2 0 110-4h14a2 2 0 110 4M5 12v7a2 2 0 002 2h10a2 2 0 002-2v-7" /></svg>
             </div>

             {/* Popover Preview (simulated hover state) */}
             <div className="absolute bottom-16 right-0 mb-2 w-64 bg-white rounded-2xl shadow-xl border border-gray-100 p-5 transform origin-bottom-right transition-all opacity-0 scale-95 group-hover:opacity-100 group-hover:scale-100">
               <h3 className="font-bold text-gray-900 mb-1">Get {reward}</h3>
               <p className="text-sm text-gray-600 mb-4">Give a friend {reward} off their first order, and get {reward} when they buy!</p>
               <input type="email" placeholder="Enter your email" className="w-full text-sm px-3 py-2 border border-gray-200 rounded-lg mb-3 bg-gray-50 pointer-events-none" />
               <button className="w-full py-2 text-white text-sm font-semibold rounded-lg pointer-events-none" style={{ backgroundColor: themeColor }}>
                 Get Share Link
               </button>
               {!removeBranding && (
                 <div className="mt-3 text-center text-[10px] text-gray-400 font-medium">
                   ⚡ Powered by OHC
                 </div>
               )}
             </div>
           </div>
        </div>
      </main>

      {/* Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
          <div className="bg-white rounded-3xl p-8 max-w-sm w-full shadow-2xl relative">
            <button
              onClick={() => setShowPaywall(false)}
              className="absolute top-4 right-4 text-gray-400 hover:text-gray-600"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
            </button>
            <div className="w-12 h-12 rounded-full bg-indigo-100 text-indigo-600 flex items-center justify-center mb-4">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
            </div>
            <h3 className="text-xl font-bold mb-2 text-gray-900">Upgrade to Pro</h3>
            <p className="text-gray-600 mb-6">
              Remove the "Powered by OHC" branding and unlock premium widgets by upgrading to our Pro plan.
            </p>
            <button
              onClick={handleUpgrade}
              className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl font-semibold transition-colors"
            >
              Upgrade Now
            </button>
          </div>
        </div>
      )}
    </div>
  );
}