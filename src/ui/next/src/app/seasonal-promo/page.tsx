"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function SeasonalPromoPage() {
  const router = useRouter();
  const [occasion, setOccasion] = useState('');
  const [discount, setDiscount] = useState('');
  const [result, setResult] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);

  const handleGenerate = () => {
    setIsGenerating(true);
    const code = occasion.substring(0, 8).toUpperCase().replace(/[^A-Z]/g, '') + discount;
    setResult(`${occasion} Special! ${discount}% OFF\nUse code: ${code}`);
    setIsGenerating(false);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Seasonal Promotion Generator ✨</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-[8px] text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-2xl mx-auto w-full flex flex-col gap-8">
        <section className="mb-6 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Create Campaign</h2>
          <div className="flex flex-col gap-4">
            <div>
              <label htmlFor="promo-occasion" className="block text-sm font-medium text-gray-700 mb-1">Occasion</label>
              <input
                id="promo-occasion"
                type="text"
                value={occasion}
                onChange={(e) => setOccasion(e.target.value)}
                placeholder="e.g. Winter Wonderland"
                className="w-full px-4 py-2 border border-gray-300 rounded-[8px] focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>
            <div>
              <label htmlFor="promo-discount" className="block text-sm font-medium text-gray-700 mb-1">Discount (%)</label>
              <input
                id="promo-discount"
                type="number"
                value={discount}
                onChange={(e) => setDiscount(e.target.value)}
                placeholder="e.g. 25"
                className="w-full px-4 py-2 border border-gray-300 rounded-[8px] focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>
            <button
              onClick={handleGenerate}
              disabled={!occasion || !discount || isGenerating}
              className={`w-full py-3 mt-4 text-white font-semibold rounded-[16px] shadow-lg transition-all ${(!occasion || !discount || isGenerating) ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
            >
              {isGenerating ? 'Generating...' : 'Generate Campaign'}
            </button>
          </div>
        </section>

        {result && (
          <section id="promo-result" className="p-6 shadow-sm flex flex-col items-center justify-center text-center relative overflow-hidden" style={{ background: 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)', color: '#fff', borderRadius: '16px' }}>
            <div className="absolute top-0 right-0 w-32 h-32 bg-white/10 rounded-bl-full -z-10"></div>
            <h3 className="text-2xl font-bold font-outfit mb-2">Your Promo Code</h3>
            <p className="text-lg whitespace-pre-wrap font-semibold">{result}</p>
          </section>
        )}
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
