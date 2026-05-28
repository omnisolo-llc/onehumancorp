"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function CartRecoveryPage() {
  const router = useRouter();
  const [selectedCart, setSelectedCart] = useState<any>(null);
  const [generatedMessage, setGeneratedMessage] = useState<string>('');
  const [copied, setCopied] = useState(false);

  const abandonedCarts = [
    { id: '1', customer: 'Alice Johnson', email: 'alice@example.com', value: '$85.00', items: ['Handmade Ceramic Mug', 'Organic Coffee Blend'], time: '2 hours ago' },
    { id: '2', customer: 'Bob Smith', email: 'bob@example.com', value: '$120.00', items: ['Premium Leather Wallet'], time: '5 hours ago' },
    { id: '3', customer: 'Charlie Brown', email: 'charlie@example.com', value: '$45.50', items: ['Scented Candle Set'], time: '1 day ago' },
  ];

  const handleGenerateCampaign = (cart: any) => {
    const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'my-store' : 'my-store';
    const message = `Hi ${cart.customer},\n\nWe noticed you left some great items in your cart totaling ${cart.value} (including the ${cart.items[0]}). Did you have any questions or need help checking out?\n\nAs a special thank you for shopping with us, here is a 10% discount code to complete your purchase: COMEBACK10\n\n🔥 UNLOCK 20% OFF! 🔥\nWant an even bigger discount? Share our store with your friends on social media and unlock a 20% off code instantly!\nClick here to share & unlock: https://ohc.store/unlock?ref=${tenantId}\n\nClick here to securely finish your checkout: https://ohc.store/checkout/recover\n\nWarmly,\nThe Team\n\n⚡ Powered by OHC`;
    setGeneratedMessage(message);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Abandoned Cart Recovery 🛒</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Cart List */}
        <section className="w-full md:w-1/3 flex flex-col gap-4">
          <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Recent Abandoned Carts</h2>
          <div className="flex flex-col gap-3">
            {abandonedCarts.map(cart => (
              <div
                key={cart.id}
                onClick={() => setSelectedCart(cart)}
                className={`p-4 rounded-xl border transition-all cursor-pointer ${selectedCart?.id === cart.id ? 'bg-indigo-50 border-indigo-300 shadow-md ring-2 ring-indigo-500' : 'bg-white border-gray-200 hover:border-indigo-200 hover:shadow-sm'}`}
              >
                <div className="flex justify-between items-start mb-2">
                  <h3 className="font-bold text-gray-900">{cart.customer}</h3>
                  <span className="text-sm font-semibold text-green-600 bg-green-50 px-2 py-0.5 rounded">{cart.value}</span>
                </div>
                <p className="text-xs text-gray-500 mb-2">{cart.time}</p>
                <p className="text-sm text-gray-700 truncate">{cart.items.join(', ')}</p>
              </div>
            ))}
          </div>
        </section>

        {/* Campaign Generator */}
        <section className="w-full md:w-2/3 flex flex-col gap-6">
          <div className="p-6 md:p-8 shadow-md bg-white rounded-2xl border border-gray-100 h-full flex flex-col">
            {selectedCart ? (
              <>
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6">Recover Cart: {selectedCart.customer}</h2>
                <div className="bg-gray-50 p-4 rounded-xl mb-6 border border-gray-200">
                    <h3 className="text-sm font-semibold text-gray-700 uppercase tracking-wide mb-2">Cart Details</h3>
                    <ul className="list-disc list-inside text-sm text-gray-600 mb-2">
                        {selectedCart.items.map((item: string, i: number) => <li key={i}>{item}</li>)}
                    </ul>
                    <p className="text-sm font-medium text-gray-900">Total: {selectedCart.value}</p>
                </div>

                <div className="mb-6 flex-1 flex flex-col">
                  <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Viral Recovery Campaign</h3>
                  <p className="text-sm text-gray-600 mb-4">Generate an email that incentivizes the customer to complete their purchase by offering a higher discount if they share your store on social media.</p>

                  <button
                    onClick={() => handleGenerateCampaign(selectedCart)}
                    className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-sm transition-all hover:-translate-y-0.5 mb-6"
                  >
                    Generate Viral Campaign Message
                  </button>

                  {generatedMessage && (
                      <div className="flex-1 flex flex-col">
                           <textarea
                                value={generatedMessage}
                                onChange={(e) => setGeneratedMessage(e.target.value)}
                                className="w-full flex-1 min-h-[250px] p-4 bg-gray-50 border border-gray-200 rounded-xl text-sm text-gray-800 focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-y mb-4"
                           />
                           <div className="flex gap-4">
                               <button
                                    onClick={() => {
                                        navigator.clipboard.writeText(generatedMessage);
                                        setCopied(true);
                                        setTimeout(() => setCopied(false), 2000);
                                    }}
                                    className={`flex-1 py-3 rounded-xl font-bold transition-all shadow-sm ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                               >
                                    {copied ? 'Copied Message!' : 'Copy to Clipboard'}
                               </button>
                               <a
                                    href={`mailto:${selectedCart.email}?subject=Complete your purchase & unlock 20% off!&body=${encodeURIComponent(generatedMessage)}`}
                                    className="flex-1 py-3 bg-blue-100 text-blue-700 hover:bg-blue-200 rounded-xl font-bold transition-all shadow-sm text-center flex items-center justify-center gap-2"
                               >
                                    Open in Email Client
                               </a>
                           </div>
                      </div>
                  )}
                </div>
              </>
            ) : (
              <div className="flex flex-col items-center justify-center h-full text-gray-400">
                <span className="text-6xl mb-4">🛒</span>
                <p className="text-lg font-medium">Select a cart to recover</p>
              </div>
            )}
          </div>
        </section>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
