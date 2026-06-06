"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';

export default function CartRecoveryPage() {
  const router = useRouter();
  const [isEnabled, setIsEnabled] = useState(false);
  const [delay, setDelay] = useState("4");
  const [includeDiscount, setIncludeDiscount] = useState(true);

  // Real inputs for preview
  const [customerName, setCustomerName] = useState("Sarah");
  const [cartValue, setCartValue] = useState("$45.00");

  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedDraft, setGeneratedDraft] = useState<string | null>(null);
  const [hasPro, setHasPro] = useState(true);
  const [isSaved, setIsSaved] = useState(false);

  useEffect(() => {
    // Check if user is Pro
    const plan = localStorage.getItem('plan') || 'Free';
    setHasPro(plan === 'Pro' || plan === 'Business');

    // Fetch existing settings
    const fetchSettings = async () => {
      try {
        const res = await fetch('/api/v1/growth/cart-recovery');
        if (res.ok) {
          const data = await res.json();
          if (data.isEnabled !== undefined) setIsEnabled(data.isEnabled);
          if (data.delay !== undefined) setDelay(data.delay);
          if (data.includeDiscount !== undefined) setIncludeDiscount(data.includeDiscount);
        }
      } catch (error) {
        console.error("Failed to fetch settings", error);
      }
    };
    fetchSettings();
  }, []);

  const handleGeneratePreview = async () => {
    setIsGenerating(true);
    setGeneratedDraft(null);

    try {
      const response = await fetch('/api/v1/growth/campaign/generate-cart', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          customer_name: customerName,
          cart_value: cartValue,
          include_discount: includeDiscount
        })
      });

      if (response.ok) {
        const data = await response.json();
        setGeneratedDraft(data.message);
      } else {
        setGeneratedDraft("Failed to generate preview. Please try again.");
      }
    } catch (error) {
      console.error("Failed to generate draft", error);
      setGeneratedDraft("An error occurred while generating the preview.");
    } finally {
      setIsGenerating(false);
    }
  };

  const handleSave = async () => {
    setIsSaved(true);
    try {
      await fetch('/api/v1/growth/cart-recovery', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          isEnabled,
          delay,
          includeDiscount
        })
      });
    } catch (error) {
      console.error("Failed to save settings", error);
    }
    setTimeout(() => {
      setIsSaved(false);
    }, 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Automated Cart Recovery 🛒</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8">
        <div className="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-100 rounded-2xl p-6 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Recover Lost Sales Automatically</h2>
           <p className="text-gray-600 text-sm">
             The Cart Recovery Agent monitors abandoned shopping sessions and automatically triggers personalized follow-up sequences to bring customers back. Businesses using this feature see up to a <strong>20% increase</strong> in recovered revenue.
           </p>
        </div>

        <div className="flex flex-col md:flex-row gap-8">
          <section className="w-full md:w-1/2 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Agent Configuration</h3>

            <div className="flex flex-col gap-5">
              <div className="flex items-center justify-between bg-white p-4 rounded-xl border border-gray-200 shadow-sm">
                <div>
                  <h4 className="font-bold text-gray-900">Enable Agent</h4>
                  <p className="text-xs text-gray-500">Let the AI monitor and email customers.</p>
                </div>
                <label className="relative inline-flex items-center cursor-pointer">
                  <input type="checkbox" className="sr-only peer" checked={isEnabled} onChange={(e) => setIsEnabled(e.target.checked)} />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
                </label>
              </div>

              <div>
                <WithTooltip id="delay-tooltip" defaultText="How long should the agent wait after a cart is abandoned before sending the email?">
                    <label className="block text-sm font-medium text-gray-700 mb-1">Follow-up Delay</label>
                </WithTooltip>
                <select
                  value={delay}
                  onChange={(e) => setDelay(e.target.value)}
                  disabled={!isEnabled}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white disabled:opacity-50"
                >
                  <option value="1">1 Hour</option>
                  <option value="4">4 Hours (Recommended)</option>
                  <option value="24">24 Hours</option>
                </select>
              </div>

              <div className="flex flex-col gap-2">
                <WithTooltip id="discount-tooltip" defaultText="The agent will generate a unique, single-use 10% discount code to incentivize the purchase.">
                    <label className="flex items-center gap-2 cursor-pointer">
                    <input
                        type="checkbox"
                        checked={includeDiscount}
                        onChange={(e) => setIncludeDiscount(e.target.checked)}
                        disabled={!isEnabled}
                        className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500 disabled:opacity-50"
                    />
                    <span className={`text-sm font-medium ${isEnabled ? 'text-gray-700' : 'text-gray-400'}`}>Include 10% Incentive Discount</span>
                    </label>
                </WithTooltip>
              </div>

              <button
                id="save-btn"
                onClick={handleSave}
                className={`w-full py-3 mt-2 text-white font-semibold rounded-xl shadow-md transition-all ${isSaved ? 'bg-green-600 hover:bg-green-700' : 'bg-indigo-600 hover:bg-indigo-700 hover:shadow-lg hover:-translate-y-0.5 active:translate-y-0'}`}
              >
                {isSaved ? "Saved!" : "Save Configuration"}
              </button>
            </div>
          </section>

          <section className="w-full md:w-1/2 p-6 shadow-md flex flex-col" style={{ background: '#ffffff', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4 flex items-center gap-2" style={{ color: '#1D1D1F' }}>
              <span className="text-indigo-500">📧</span> AI Email Preview
            </h3>

            <div className="flex gap-4 mb-4">
              <div className="flex-1">
                <label className="block text-xs font-medium text-gray-500 mb-1 uppercase tracking-wider">Test Name</label>
                <input
                  type="text"
                  value={customerName}
                  onChange={(e) => setCustomerName(e.target.value)}
                  className="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:outline-none focus:border-indigo-500"
                  placeholder="e.g. Sarah"
                />
              </div>
              <div className="flex-1">
                <label className="block text-xs font-medium text-gray-500 mb-1 uppercase tracking-wider">Test Cart Value</label>
                <input
                  type="text"
                  value={cartValue}
                  onChange={(e) => setCartValue(e.target.value)}
                  className="w-full px-3 py-2 text-sm border border-gray-200 rounded-lg focus:outline-none focus:border-indigo-500"
                  placeholder="e.g. $45.00"
                />
              </div>
            </div>

            {generatedDraft ? (
              <div className="flex-1 flex flex-col">
                <div className="flex-1 bg-gray-50 border border-gray-100 rounded-xl p-4 mb-4 overflow-y-auto">
                  <pre className="whitespace-pre-wrap text-sm text-gray-700 font-inter font-medium" style={{ fontFamily: 'inherit' }}>
                    {generatedDraft}
                  </pre>
                </div>
                <button
                    onClick={handleGeneratePreview}
                    disabled={isGenerating}
                    className="w-full py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-bold rounded-xl shadow-sm transition-all flex items-center justify-center gap-2"
                  >
                     {isGenerating ? 'Regenerating...' : 'Regenerate Preview'}
                  </button>
              </div>
            ) : (
              <div className="flex-1 flex flex-col items-center justify-center text-gray-400 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center">
                <svg className="w-12 h-12 mb-3 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" /></svg>
                <p className="text-sm font-medium mb-4">See how the agent will personalize emails based on cart contents.</p>
                <button
                    onClick={handleGeneratePreview}
                    disabled={isGenerating}
                    className="w-full py-2 bg-gray-900 hover:bg-black text-white font-bold rounded-lg shadow-sm transition-all flex items-center justify-center gap-2"
                >
                    {isGenerating ? 'Generating...' : 'Generate AI Preview'}
                </button>
              </div>
            )}
          </section>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
