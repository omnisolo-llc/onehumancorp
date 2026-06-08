"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function GoalTrackerPage() {
  const router = useRouter();

  const [title, setTitle] = useState('');
  const [target, setTarget] = useState('100');
  const [isGenerating, setIsGenerating] = useState(false);
  const [widgetHtml, setWidgetHtml] = useState('');
  const [copied, setCopied] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [trialStatus, setTrialStatus] = useState<string | null>(null);

  useEffect(() => {
    const checkPlan = async () => {
      try {
        const res = await fetch("/api/v1/auth/me");
        if (res.ok) {
          const data = await res.json();
          if (data.tenant?.plan_tier === "pro" || data.tenant?.plan_tier === "enterprise") {
             setHasPro(true);
          }
        } else {
           setHasPro(true);
        }
      } catch (e) {
        setHasPro(true);
      }
    };
    checkPlan();
  }, []);

  const generateWidget = async () => {
    setIsGenerating(true);
    try {
      const res = await fetch('/api/v1/growth/campaign/goal-tracker', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
           goal_name: title || "Business Goal",
           target: target || "100",
        })
      });
      const data = await res.json();
      setWidgetHtml(data.widget_html);
    } catch (e) {
      console.error(e);
      // Fallback
      setWidgetHtml(`<div class='ohc-goal-tracker'><h3>${title || "Business Goal"}</h3><p>Target: ${target || "100"}</p><div class='progress-bar'><div class='progress' style='width: 0%'></div></div></div>`);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleGenerate = () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }
    generateWidget();
  };

  const claimTrialExtension = async () => {
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent("I just launched a viral goal tracker for my business on One Human Corp! Start your own business today: https://ohc.example")}`, "_blank");
    try {
      await fetch("/api/v1/growth/trial-extension/claim", { method: "POST" });
      setHasPro(true);
      setShowSoftPaywall(false);
      setTrialStatus("Your 7-day Pro trial has been activated.");
      generateWidget();
    } catch (e) {
      console.error("Trial extension failed", e);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Viral Goal Tracker 📈</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {trialStatus && <div className="w-full bg-green-50 px-4 py-3 text-sm font-semibold text-green-800 text-center mb-4 rounded-xl" role="status">{trialStatus}</div>}
        {/* Settings */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
          <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <div className="flex items-center gap-4 mb-4">
              <h2 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>Goal Details</h2>
              <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                  <span className="text-xs font-medium text-yellow-600">Pro Feature</span>
              </div>
            </div>

            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="goal-title" className="block text-sm font-medium text-gray-700 mb-1">Goal Name</label>
                <input
                  id="goal-title"
                  type="text"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  placeholder="e.g. Help us reach 100 sales!"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div>
                <label htmlFor="goal-target" className="block text-sm font-medium text-gray-700 mb-1">Target Number</label>
                <input
                  id="goal-target"
                  type="text"
                  value={target}
                  onChange={(e) => setTarget(e.target.value)}
                  placeholder="e.g. 100"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <button
                onClick={handleGenerate}
                disabled={!title || isGenerating}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all ${(!title || isGenerating) ? 'bg-blue-400 cursor-not-allowed' : 'bg-blue-600 hover:bg-blue-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
              >
                {isGenerating ? 'Generating...' : 'Generate Tracker Widget'}
              </button>
            </div>
          </div>

          {widgetHtml && (
            <div className="p-6 shadow-md bg-white border border-green-200 rounded-[16px]">
              <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2 flex items-center gap-2">
                <span className="text-green-500">✅</span> Widget Ready!
              </h3>
              <p className="text-sm text-gray-600 mb-4">Embed this HTML snippet on your storefront or share it to display your progress.</p>

              <div className="flex gap-2">
                <input
                  type="text"
                  readOnly
                  value={widgetHtml}
                  className="flex-1 px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg text-sm text-gray-700 focus:outline-none"
                />
                <button
                  onClick={() => {
                    navigator.clipboard.writeText(widgetHtml);
                    setCopied(true);
                    setTimeout(() => setCopied(false), 2000);
                  }}
                  className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                >
                  {copied ? 'Copied!' : 'Copy'}
                </button>
              </div>
            </div>
          )}
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex justify-center items-start">
             <div className="w-full max-w-sm bg-white rounded-3xl shadow-2xl overflow-hidden relative border border-gray-200 flex flex-col items-center">
                 <div className="w-full h-32 bg-gradient-to-r from-blue-500 to-indigo-500 relative flex items-center justify-center">
                     <span className="text-5xl drop-shadow-md">📈</span>
                 </div>

                 <div className="w-full p-8 flex flex-col items-center text-center">
                     <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">
                         {title || 'Business Goal'}
                     </h2>
                     <p className="text-sm text-gray-600 mb-6 leading-relaxed">
                         Target: {target || '100'}
                     </p>

                     <div className="w-full h-4 bg-gray-200 rounded-full overflow-hidden mb-2">
                        <div className="h-full bg-blue-600 rounded-full" style={{ width: '0%' }}></div>
                     </div>
                     <span className="text-xs text-gray-500">0% achieved</span>

                     <div className="mt-8">
                        <span className="text-xs font-semibold text-gray-400 uppercase tracking-widest">Powered by OHC</span>
                     </div>
                 </div>
             </div>
        </section>
      </main>

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
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Viral Goal Trackers are a Pro feature. Upgrade to our Pro plan to generate embeddable widgets and share your progress publicly.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #3b82f6 0%, #4f46e5 100%)' }}
            >
              Upgrade to Pro
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm bg-black text-white border-2 border-black hover:bg-gray-800 flex items-center justify-center gap-2"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X to get 7 Days Free
            </button>
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
