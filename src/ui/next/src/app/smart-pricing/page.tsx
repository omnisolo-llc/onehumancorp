"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function SmartPricingPage() {
  const router = useRouter();
  const [enabled, setEnabled] = useState(false);
  const [discountPerishables, setDiscountPerishables] = useState(false);
  const [surgePricing, setSurgePricing] = useState(false);
  const [maxAdjustment, setMaxAdjustment] = useState(20);

  const [productName, setProductName] = useState('');
  const [currentPrice, setCurrentPrice] = useState('');
  const [recommendedPrice, setRecommendedPrice] = useState<string | null>(null);
  const [explanation, setExplanation] = useState<string | null>(null);
  const [isAnalyzing, setIsAnalyzing] = useState(false);

  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);

  React.useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const basePrice = 10.0;
  const minPrice = (basePrice * (1 - maxAdjustment / 100)).toFixed(2);
  const maxPrice = (basePrice * (1 + maxAdjustment / 100)).toFixed(2);

  const handleAnalyze = async () => {
    if (!hasPro) {
        setShowSoftPaywall(true);
        return;
    }
    setIsAnalyzing(true);
    setRecommendedPrice(null);
    setExplanation(null);

    try {
        const res = await fetch('/api/v1/growth/pricing/optimize', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ product_name: productName, current_price: currentPrice })
        });
        const data = await res.json();
        setRecommendedPrice(data.recommended_price);
        setExplanation(data.explanation);
    } catch (e) {
        console.error("Analysis failed", e);
    } finally {
        setIsAnalyzing(false);
    }
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI pricing tools for my business on One Human Corp! Start your own business today: ohc://join?ref=' + tenant)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    handleAnalyze();
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Smart Pricing Optimizer 💸</h1>
        <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-3xl mx-auto w-full flex flex-col gap-8">
        <section className="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-100 rounded-2xl p-6 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Maximize Your Profit Margins</h2>
           <p className="text-gray-600 text-sm">
             Let our AI analyze market trends and competitor pricing to suggest the optimal price point for your products. Boost your revenue instantly.
           </p>
        </section>

        <section className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <div className="flex items-center gap-4 mb-4">
            <h3 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>AI Price Analysis</h3>
            <div className="flex items-center gap-2 px-3 py-1 bg-yellow-50 rounded-full border border-yellow-100">
                <span className="text-xs font-medium text-yellow-600">Pro Feature</span>
            </div>
          </div>

          <div className="flex flex-col gap-4">
             <div>
              <label htmlFor="product-name" className="block text-sm font-medium text-gray-700 mb-1">Product Name</label>
              <input
                id="product-name"
                type="text"
                value={productName}
                onChange={(e) => setProductName(e.target.value)}
                placeholder="e.g. Handmade Ceramic Mug"
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>
            <div>
              <label htmlFor="current-price" className="block text-sm font-medium text-gray-700 mb-1">Current Price ($)</label>
              <input
                id="current-price"
                type="number"
                value={currentPrice}
                onChange={(e) => setCurrentPrice(e.target.value)}
                placeholder="e.g. 15.00"
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>
            <button
                onClick={handleAnalyze}
                disabled={isAnalyzing || !productName}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 ${isAnalyzing || !productName ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 hover:shadow-xl hover:-translate-y-0.5'}`}
              >
                {isAnalyzing ? (
                   <>
                    <svg className="animate-spin -ml-1 mr-2 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Analyzing Market Data...
                  </>
                ) : (
                  <>
                     <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                     Analyze Market & Optimize Pricing
                  </>
                )}
              </button>
          </div>
        </section>

        {recommendedPrice && explanation && (
            <section className="p-6 shadow-md flex flex-col gap-4 animate-fade-in-up" style={{ background: 'linear-gradient(135deg, #e0c3fc 0%, #8ec5fc 100%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h3 className="text-xl font-bold font-outfit flex items-center gap-2 text-gray-900">
                    <span className="text-2xl">✨</span> Smart Pricing Recommendation
                </h3>

                <div className="flex gap-4 items-center bg-white/60 p-4 rounded-xl border border-white/40">
                    <div className="flex-1 text-center">
                       <p className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-1">Current</p>
                       <p className="text-2xl font-bold text-gray-500 line-through">${parseFloat(currentPrice).toFixed(2)}</p>
                    </div>
                    <div className="text-2xl text-indigo-500 font-bold">➔</div>
                    <div className="flex-1 text-center">
                       <p className="text-sm font-semibold text-indigo-600 uppercase tracking-wider mb-1">Optimal</p>
                       <p className="text-4xl font-bold font-outfit text-indigo-700">${parseFloat(recommendedPrice).toFixed(2)}</p>
                    </div>
                </div>

                <div className="bg-white/80 p-4 rounded-xl text-gray-800 font-medium leading-relaxed shadow-sm">
                    {explanation}
                </div>
            </section>
        )}

        <div className="p-6 shadow-sm rounded-2xl flex items-center justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
          <div>
            <h3 className="text-lg font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Enable Smart Pricing</h3>
            <p className="text-sm text-gray-500 mt-1">Turn on autonomous hyper-local dynamic pricing.</p>
          </div>
          <button
            data-testid="enable-smart-pricing-toggle"
            onClick={() => setEnabled(!enabled)}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${enabled ? 'bg-green-500' : 'bg-gray-300'}`}
          >
            <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${enabled ? 'translate-x-6' : 'translate-x-1'}`} />
          </button>
        </div>

        {enabled && (
          <div className="p-6 shadow-sm rounded-2xl flex flex-col gap-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
            <h3 className="text-lg font-semibold font-outfit border-b pb-2" style={{ color: '#1D1D1F', borderColor: 'rgba(0,0,0,0.1)' }}>Configuration</h3>

            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium" style={{ color: '#1D1D1F' }}>Auto-discount perishables 2 hours before closing</p>
                <p className="text-xs text-gray-500 mt-1">Clear out remaining inventory today.</p>
              </div>
              <button
                data-testid="discount-perishables-toggle"
                onClick={() => setDiscountPerishables(!discountPerishables)}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${discountPerishables ? 'bg-blue-500' : 'bg-gray-300'}`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${discountPerishables ? 'translate-x-6' : 'translate-x-1'}`} />
              </button>
            </div>

            <div className="flex items-center justify-between">
              <div>
                <p className="font-medium" style={{ color: '#1D1D1F' }}>Surge pricing during high demand</p>
                <p className="text-xs text-gray-500 mt-1">Charge a premium during peak rush hours.</p>
              </div>
              <button
                data-testid="surge-pricing-toggle"
                onClick={() => setSurgePricing(!surgePricing)}
                className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${surgePricing ? 'bg-blue-500' : 'bg-gray-300'}`}
              >
                <span className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${surgePricing ? 'translate-x-6' : 'translate-x-1'}`} />
              </button>
            </div>

            <div className="mt-4">
              <div className="flex justify-between items-center mb-2">
                <label className="font-medium" style={{ color: '#1D1D1F' }}>Maximum price adjustment bounds (+/-)</label>
                <span className="font-bold text-blue-600">{maxAdjustment}%</span>
              </div>
              <input
                type="range"
                min="5"
                max="50"
                step="5"
                value={maxAdjustment}
                onChange={(e) => setMaxAdjustment(parseInt(e.target.value))}
                className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                data-testid="price-bounds-slider"
              />

              <div className="mt-6 p-4 rounded-xl border shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                <p className="text-sm font-semibold mb-3 text-gray-700">Preview: How a $10.00 item might fluctuate</p>
                <div className="flex justify-between items-center px-2">
                  <div className="text-center">
                    <p className="text-xs text-gray-500 mb-1">Floor</p>
                    <p className="font-bold text-green-600" data-testid="preview-min-price">${minPrice}</p>
                  </div>
                  <div className="flex-1 border-t-2 border-dashed border-gray-300 mx-4 relative">
                     <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 bg-white px-2 text-xs font-bold text-gray-800 rounded shadow-sm border">$10.00</div>
                  </div>
                  <div className="text-center">
                    <p className="text-xs text-gray-500 mb-1">Ceiling</p>
                    <p className="font-bold text-orange-500" data-testid="preview-max-price">${maxPrice}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}
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
              Smart Pricing Optimizer is a Pro feature. Upgrade to our Pro plan to maximize your profit margins with AI.
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
