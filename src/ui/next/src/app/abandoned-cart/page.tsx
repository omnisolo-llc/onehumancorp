"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function AbandonedCartPage() {
  const router = useRouter();
  const [customerName, setCustomerName] = useState('');
  const [cartValue, setCartValue] = useState('');
  const [result, setResult] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);

  const handleGenerate = async () => {
    setIsGenerating(true);
    try {
      const response = await fetch('/api/v1/growth/campaign/generate-cart', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          customer_name: customerName,
          cart_value: cartValue,
        }),
      });

      if (response.ok) {
        const data = await response.json();
        setResult(data.message);
      } else {
        setResult('Error generating campaign.');
      }
    } catch (error) {
      console.error(error);
      setResult('Error generating campaign.');
    } finally {
      setIsGenerating(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Abandoned Cart Campaign 🛒</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-2xl mx-auto w-full flex flex-col gap-8">
        <section className="mb-6 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <div className="flex items-center gap-4 mb-4">
            <h2 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>Generate Campaign</h2>
          </div>
          <div className="flex flex-col gap-4">
            <div>
              <label htmlFor="customer-name" className="block text-sm font-medium text-gray-700 mb-1">Customer Name</label>
              <input
                id="customer-name"
                type="text"
                value={customerName}
                onChange={(e) => setCustomerName(e.target.value)}
                placeholder="e.g. Maya"
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>
            <div>
              <label htmlFor="cart-value" className="block text-sm font-medium text-gray-700 mb-1">Cart Value</label>
              <input
                id="cart-value"
                type="text"
                value={cartValue}
                onChange={(e) => setCartValue(e.target.value)}
                placeholder="e.g. $100.00"
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>
            <button
              onClick={handleGenerate}
              disabled={!customerName || !cartValue || isGenerating}
              className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all ${(!customerName || !cartValue || isGenerating) ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
            >
              {isGenerating ? 'Generating...' : 'Generate AI Campaign'}
            </button>
          </div>
        </section>

        {result && (
          <section id="promo-result" className="p-6 shadow-sm flex flex-col items-start relative overflow-hidden" style={{ background: '#ffffff', color: '#111827', borderRadius: '16px', border: '1px solid rgba(0,0,0,0.1)' }}>
            <h3 className="text-2xl font-bold font-outfit mb-4">Generated Campaign</h3>
            <pre className="text-sm whitespace-pre-wrap font-inter w-full overflow-x-auto text-left" style={{ fontFamily: 'inherit' }}>
                {result}
            </pre>
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
