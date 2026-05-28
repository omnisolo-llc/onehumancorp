"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';

export default function AIUpsellPage() {
  const router = useRouter();
  const [productName, setProductName] = useState('');
  const [targetAudience, setTargetAudience] = useState('new');
  const [upsellRecommendations, setUpsellRecommendations] = useState<any>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleGenerate = () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }

    setIsGenerating(true);
    setTimeout(() => {
        setUpsellRecommendations([
            { id: 1, name: `Premium ${productName} Add-on`, price: '$19.99', desc: 'Frequently bought together by this audience.', increase: '+15% AOV' },
            { id: 2, name: `${productName} Care Kit`, price: '$9.99', desc: 'Perfect cross-sell item at checkout.', increase: '+8% AOV' }
        ]);
        setIsGenerating(false);
    }, 1500);
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I am boosting my sales with AI-powered upsell recommendations on One Human Corp! Start your own business today: ohc://join?ref=' + tenant)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    setTimeout(() => {
      alert('Your 7-day Pro trial has been activated.');
      handleGenerate();
    }, 500);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
         <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">AI Upsell Recommendations 📈</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">

        {/* Campaign Settings */}
        <section className="w-full md:w-1/2 p-6 shadow-md bg-white/65 backdrop-blur-md border border-white/40 rounded-2xl flex flex-col gap-6">
            <div>
                <h3 className="text-xl font-semibold font-outfit mb-2 text-[#1D1D1F]">Generate Upsell Strategy</h3>
                <p className="text-sm text-gray-600">Use our AI to analyze your product and suggest high-converting cross-sells and upsells to boost your Average Order Value (AOV).</p>
            </div>

            <div className="flex flex-col gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Base Product</label>
                <input
                  type="text"
                  value={productName}
                  onChange={(e) => setProductName(e.target.value)}
                  placeholder="e.g. Signature Coffee Blend"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Target Audience</label>
                <select
                  value={targetAudience}
                  onChange={(e) => setTargetAudience(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white"
                >
                  <option value="new">New Customers</option>
                  <option value="returning">Returning Buyers</option>
                  <option value="vip">VIP Members</option>
                </select>
              </div>
              <WithTooltip id="generate-upsell-tooltip" defaultText="Click to generate tailored product recommendations to offer at checkout.">
                  <button
                    onClick={handleGenerate}
                    disabled={!productName || isGenerating}
                    className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 ${(!productName || isGenerating) ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
                  >
                    {isGenerating ? 'Analyzing Product Data...' : 'Generate Upsell Items'}
                  </button>
              </WithTooltip>
            </div>
        </section>

        {/* AI Recommendations Preview */}
        <section className="w-full md:w-1/2 p-6 shadow-md flex flex-col border border-gray-100 rounded-2xl bg-white relative overflow-hidden">
            <h3 className="text-xl font-semibold font-outfit mb-6 flex items-center gap-2 text-[#1D1D1F]">
              <span className="text-yellow-500">✨</span> Suggested Offers
            </h3>

            {upsellRecommendations ? (
              <div className="flex-1 flex flex-col gap-4 z-10">
                {upsellRecommendations.map((item: any) => (
                    <div key={item.id} className="p-4 bg-gray-50 border border-gray-200 rounded-xl flex flex-col gap-2 relative">
                        <div className="absolute top-4 right-4 bg-green-100 text-green-700 font-bold text-xs px-2 py-1 rounded-full">
                            {item.increase}
                        </div>
                        <h4 className="font-bold text-gray-900">{item.name}</h4>
                        <p className="text-sm text-gray-600">{item.desc}</p>
                        <p className="font-semibold text-indigo-600">{item.price}</p>
                    </div>
                ))}

                <div className="mt-auto pt-4 border-t border-gray-100 flex flex-col gap-3">
                    <button className="w-full py-3 bg-gray-900 hover:bg-black text-white font-bold rounded-xl shadow-md transition-all flex items-center justify-center gap-2">
                        Apply to Store Checkout
                    </button>
                </div>
              </div>
            ) : (
              <div className="flex-1 flex flex-col items-center justify-center text-gray-400 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center z-10 bg-gray-50/50">
                <svg className="w-12 h-12 mb-3 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6" /></svg>
                <p className="text-sm font-medium">Enter a product to generate smart upsell recommendations.</p>
              </div>
            )}

            {/* Background elements */}
            <div className="absolute bottom-0 right-0 w-48 h-48 bg-indigo-50 rounded-tl-full -z-0"></div>
        </section>
      </main>

      {/* Soft Paywall Modal for Interactive Trial Extension */}
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
              AI-Powered Upsell Recommendations is a Pro feature. Upgrade to our Pro plan to boost your sales on autopilot.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
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

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
