"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function CartRecoveryPage() {
  const router = useRouter();
  const [selectedCart, setSelectedCart] = useState<string | null>(null);
  const [discount, setDiscount] = useState('15');
  const [generatedMessage, setGeneratedMessage] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [isSent, setIsSent] = useState(false);

  const mockCarts = [
    { id: '1', customer: 'Sarah Jenkins', amount: '$120.50', items: 3, time: '2 hours ago' },
    { id: '2', customer: 'Michael Chen', amount: '$45.00', items: 1, time: '5 hours ago' },
    { id: '3', customer: 'Emma Thompson', amount: '$210.00', items: 4, time: '1 day ago' },
  ];

  const handleGenerate = () => {
    setIsGenerating(true);
    // Simulate AI generation delay
    setTimeout(() => {
      const cart = mockCarts.find(c => c.id === selectedCart) || mockCarts[0];
      setGeneratedMessage(
        `Hi ${cart.customer.split(' ')[0]},\n\n` +
        `We noticed you left some great items in your cart. We don't want you to miss out!\n\n` +
        `Come back and complete your purchase today, and enjoy a special ${discount}% OFF your entire order.\n\n` +
        `Use code: COMEBACK${discount} at checkout.\n\n` +
        `Click here to resume your checkout: [Cart Link]\n\n` +
        `Cheers,\n` +
        `The ${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'Store' : 'Store'} Team`
      );
      setIsGenerating(false);
      setIsSent(false);
    }, 1000);
  };

  const handleSend = () => {
    // Simulate sending
    setIsSent(true);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>AI Cart Recovery 🛒</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">
        <div className="bg-gradient-to-r from-green-50 to-emerald-50 border border-green-100 rounded-2xl p-6 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Recover Lost Sales</h2>
           <p className="text-gray-600 text-sm">
             On average, 70% of shopping carts are abandoned. Use our AI to craft personalized, high-converting win-back messages to recover revenue instantly.
           </p>
        </div>

        <div className="flex flex-col md:flex-row gap-8">
          {/* Campaign Settings & Abandoned Carts */}
          <section className="w-full md:w-1/2 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Abandoned Carts</h3>

            <div className="space-y-3 mb-6">
              {mockCarts.map(cart => (
                <div
                  key={cart.id}
                  onClick={() => setSelectedCart(cart.id)}
                  className={`p-4 border rounded-xl cursor-pointer transition-all ${selectedCart === cart.id ? 'border-green-500 bg-green-50' : 'border-gray-200 bg-white hover:border-green-300'}`}
                >
                  <div className="flex justify-between items-start">
                    <div>
                      <h4 className="font-semibold text-gray-900">{cart.customer}</h4>
                      <p className="text-xs text-gray-500">{cart.items} items • {cart.time}</p>
                    </div>
                    <span className="font-bold text-gray-900">{cart.amount}</span>
                  </div>
                </div>
              ))}
            </div>

            <h3 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Offer Details</h3>
            <div className="flex flex-col gap-4">
              <div>
                <label htmlFor="discount" className="block text-sm font-medium text-gray-700 mb-1">Discount Incentive (%)</label>
                <input
                  id="discount"
                  type="number"
                  value={discount}
                  onChange={(e) => setDiscount(e.target.value)}
                  placeholder="e.g. 15"
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-green-500"
                />
              </div>
              <button
                onClick={handleGenerate}
                disabled={!selectedCart || isGenerating}
                className={`w-full py-3 mt-4 text-white font-semibold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 ${(!selectedCart || isGenerating) ? 'bg-green-400 cursor-not-allowed' : 'bg-green-600 hover:bg-green-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
              >
                {isGenerating ? (
                  <>
                    <svg className="animate-spin -ml-1 mr-2 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                      <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    Drafting with AI...
                  </>
                ) : (
                  <>
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                    Generate Recovery Message
                  </>
                )}
              </button>
            </div>
          </section>

          {/* AI Draft Preview */}
          <section className="w-full md:w-1/2 p-6 shadow-md flex flex-col" style={{ background: '#ffffff', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4 flex items-center gap-2" style={{ color: '#1D1D1F' }}>
              <span className="text-green-500">✨</span> AI Generated Message
            </h3>

            {generatedMessage ? (
              <div className="flex-1 flex flex-col">
                <div className="flex-1 bg-gray-50 border border-gray-100 rounded-xl p-4 mb-4">
                  <pre className="whitespace-pre-wrap text-sm text-gray-700 font-inter font-medium" style={{ fontFamily: 'inherit' }}>
                    {generatedMessage}
                  </pre>
                </div>

                {isSent ? (
                  <div className="w-full py-3 bg-green-50 text-green-700 font-bold rounded-xl text-center border border-green-200">
                    ✅ Recovery message sent to {mockCarts.find(c => c.id === selectedCart)?.customer}!
                  </div>
                ) : (
                  <button
                    onClick={handleSend}
                    className="w-full py-3 bg-gray-900 hover:bg-black text-white font-bold rounded-xl shadow-md transition-all flex items-center justify-center gap-2"
                  >
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" /></svg>
                    Send Recovery SMS/Email
                  </button>
                )}
              </div>
            ) : (
              <div className="flex-1 flex flex-col items-center justify-center text-gray-400 border-2 border-dashed border-gray-200 rounded-xl p-6 text-center">
                <svg className="w-12 h-12 mb-3 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17m0 0a2 2 0 100 4 2 2 0 000-4zm-8 2a2 2 0 11-4 0 2 2 0 014 0z" /></svg>
                <p className="text-sm font-medium">Select an abandoned cart to generate a personalized recovery offer.</p>
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
