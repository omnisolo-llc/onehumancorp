'use client';

import React from 'react';
import Link from 'next/link';
import { useOfferingStore } from './store';

export default function NewOfferingPage() {
  const {
    intent,
    loading,
    productData,
    error,
    isSuccess,
    setIntent,
    setProductData,
    generateOffering,
    publishOffering,
  } = useOfferingStore();

  const handleGenerate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!intent.trim()) return;
    await generateOffering(intent);
  };

  const handlePublish = async () => {
    if (productData) {
      await publishOffering(productData);
    }
  };

  if (isSuccess) {
    return (
      <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter justify-center items-center">
         <div className="w-20 h-20 bg-green-100 rounded-full flex items-center justify-center mb-6 animate-bounce">
            <span className="text-4xl">🎉</span>
         </div>
         <h1 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Published!</h1>
         <p className="text-sm text-gray-600 mb-6 text-center">Your new offering is now live on your storefront.</p>
         <Link href="/bio/demo-tenant" className="w-full max-w-xs py-3 bg-blue-600 text-white rounded-xl font-bold shadow-md hover:bg-blue-700 text-center min-h-[44px] mb-3">
            View Storefront
         </Link>
         <Link href="/dashboard" className="w-full max-w-xs py-3 bg-gray-900 text-white rounded-xl font-bold shadow-md hover:bg-black text-center min-h-[44px]">
            Return to Dashboard
         </Link>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter relative pb-20">
      <div className="flex items-center mb-6 border-b border-gray-200 pb-4">
        <Link href="/dashboard" className="text-blue-500 font-semibold mr-4 min-h-[44px] flex items-center">&lt; Back</Link>
        <h1 className="text-xl font-bold font-outfit text-gray-900">Add Offering</h1>
      </div>

      {error && (
        <div className="mb-4 rounded-[8px] border border-red-200 bg-red-50 px-4 py-3 text-sm font-semibold text-red-700">
          {error}
        </div>
      )}

      {!loading && !productData && (
        <div className="flex-1 flex flex-col items-center justify-center">
          <form onSubmit={handleGenerate} className="w-full flex flex-col gap-4">
             <label className="text-gray-800 font-semibold text-center text-lg">What do you want to offer?</label>
             <textarea
               value={intent}
               onChange={(e) => setIntent(e.target.value)}
               placeholder="e.g. Guitar lessons for beginners, 1 hour"
               rows={4}
               className="w-full border border-gray-300 rounded-2xl p-4 bg-white shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 text-gray-800 resize-none min-h-[44px]"
             />
             <button
               type="submit"
               disabled={!intent.trim()}
               className="w-full rounded-[16px] px-4 py-4 text-white font-bold shadow-sm transition-colors bg-blue-600 hover:bg-blue-700 disabled:bg-gray-300 disabled:cursor-not-allowed min-h-[44px]"
             >
               Generate
             </button>
          </form>
          <p className="text-sm text-gray-500 mt-4 text-center">
            Our AI Agents will instantly write the title, description, category, and suggest a price.
          </p>
        </div>
      )}

      {loading && (
        <div className="flex-1 flex flex-col items-center justify-center gap-6">
           <div className="w-full aspect-square bg-gray-200 rounded-2xl animate-pulse flex items-center justify-center">
              <div className="text-4xl animate-bounce">✨</div>
           </div>
           <div className="w-full space-y-4">
              <div className="h-6 bg-gray-200 rounded-md animate-pulse w-3/4"></div>
              <div className="h-20 bg-gray-200 rounded-md animate-pulse w-full"></div>
              <div className="h-10 bg-gray-200 rounded-md animate-pulse w-1/3"></div>
           </div>
           <p className="text-sm font-semibold text-blue-600 animate-pulse text-center">AI is drafting your offering...</p>
        </div>
      )}

      {productData && !loading && (
        <div className="flex-1 flex flex-col gap-6 animate-fade-in-up">
           <div className="w-full aspect-square bg-gray-200 rounded-2xl overflow-hidden relative">
              <div className="absolute inset-0 bg-gradient-to-tr from-blue-100 to-purple-100 flex items-center justify-center">
                 <div className="text-6xl">{productData.type === 'Service' ? '🎸' : '📦'}</div>
              </div>
           </div>

           {/* Glassmorphism Card */}
           <div className="p-5 rounded-[16px] shadow-lg flex flex-col gap-4 relative overflow-hidden"
                style={{
                   background: 'rgba(255, 255, 255, 0.65)',
                   backdropFilter: 'blur(30px) saturate(210%)',
                   border: '1px solid rgba(255, 255, 255, 0.4)'
                }}>
              <div className="absolute top-2 right-2 px-2 py-1 bg-gradient-to-r from-blue-500 to-purple-500 text-white text-[10px] font-bold rounded-full uppercase tracking-wider shadow-sm flex items-center gap-1">
                 <span>✨</span> AI Generated
              </div>
              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Title</label>
                  <input
                    type="text"
                    value={productData.title}
                    onChange={(e) => setProductData({...productData, title: e.target.value})}
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all min-h-[44px]"
                  />
              </div>
              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Description</label>
                  <textarea
                    value={productData.description}
                    onChange={(e) => setProductData({...productData, description: e.target.value})}
                    rows={4}
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-sm text-gray-800 leading-relaxed focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all min-h-[44px]"
                  />
              </div>
              <div className="flex gap-4">
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Price</label>
                      <div className="relative">
                          <span className="absolute left-3 top-3 text-gray-500 font-semibold">$</span>
                          <input
                            type="text"
                            value={productData.price}
                            onChange={(e) => setProductData({...productData, price: e.target.value})}
                            className="w-full bg-white/50 border border-white/60 rounded-[8px] pl-7 pr-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all min-h-[44px]"
                          />
                      </div>
                  </div>
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Type</label>
                      <input
                        type="text"
                        value={productData.type}
                        onChange={(e) => setProductData({...productData, type: e.target.value})}
                        className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all min-h-[44px]"
                      />
                  </div>
              </div>
              <div className="mt-4 border-t border-white/40 pt-4">
                  <label className="flex items-center cursor-pointer min-h-[44px]">
                      <div className="relative">
                          <input type="checkbox" className="sr-only" checked={productData?.isSubscription || false} onChange={(e) => setProductData({...productData!, isSubscription: e.target.checked})} />
                          <div className={`block w-10 h-6 rounded-full transition-colors ${productData?.isSubscription ? 'bg-blue-500' : 'bg-gray-300'}`}></div>
                          <div className={`dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform ${productData?.isSubscription ? 'transform translate-x-4' : ''}`}></div>
                      </div>
                      <div className="ml-3 text-gray-800 font-semibold text-sm">
                          Offer as Subscription
                      </div>
                  </label>
              </div>
           </div>

           <button
             onClick={handlePublish}
             className="w-full py-4 bg-[#0066FF] text-white font-bold rounded-[8px] shadow-md hover:bg-blue-600 transition-colors text-lg min-h-[44px]"
           >
             Publish {productData.type}
           </button>
        </div>
      )}
    </div>
  );
}
