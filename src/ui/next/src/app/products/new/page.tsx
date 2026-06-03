'use client';

import React, { useState } from 'react';
import Link from 'next/link';

export default function AutoCatalogPage() {
  const [loading, setLoading] = useState(false);
  const [enhancing, setEnhancing] = useState(false);
  const [uploadedImage, setUploadedImage] = useState<string | null>(null);
  const [variations, setVariations] = useState<string[]>([]);
  const [selectedVariation, setSelectedVariation] = useState<string | null>(null);

  const [productData, setProductData] = useState<{
    title: string;
    description: string;
    price: string;
    category: string;
    isSubscription?: boolean;
    subscriptionInterval?: string;
    subscriptionDiscount?: string;
  } | null>(null);
  const [published, setPublished] = useState(false);

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      const imageUrl = URL.createObjectURL(file);
      setUploadedImage(imageUrl);
    }
  };

  const handleMagicEnhance = async () => {
    setEnhancing(true);
    try {
      const response = await fetch('/api/magic-enhance', {
        method: 'POST',
      });
      const data = await response.json();
      setVariations(data.variations);
    } catch (error) {
      console.error('Error generating variations:', error);
    } finally {
      setEnhancing(false);
    }
  };

  const handleVariationSelect = (variation: string) => {
    setSelectedVariation(variation);
  };

  const handleContinueToCatalog = async () => {
    setLoading(true);
    try {
      const response = await fetch('/api/auto-catalog', {
        method: 'POST',
      });
      const data = await response.json();
      setProductData(data);
    } catch (error) {
      console.error('Error auto-cataloging:', error);
    } finally {
      setLoading(false);
    }
  };

  const handlePublish = async () => {
    if (!productData) return;

    setLoading(true);
    try {
      const response = await fetch('/api/product', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: productData.title,
          description: productData.description,
          price: productData.price,
          item_type: productData.category,
          is_subscription: productData.isSubscription,
          subscription_interval: productData.subscriptionInterval,
          subscription_discount: productData.subscriptionDiscount ? parseInt(productData.subscriptionDiscount) : undefined
        })
      });

      if (response.ok) {
        setPublished(true);
      }
    } catch (error) {
      console.error('Error publishing product:', error);
    } finally {
      setLoading(false);
    }
  };

  if (published) {
    return (
      <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col justify-center items-center font-inter">
         <div className="text-6xl mb-4">🎉</div>
         <h1 className="text-2xl font-bold mb-2">Product Published!</h1>
         <p className="text-gray-600 mb-6 text-center">Your new product is now live on your storefront.</p>
         <Link href="/dashboard" className="w-full max-w-xs py-3 bg-gray-900 text-white rounded-xl font-bold shadow-md hover:bg-black text-center">
            Return to Dashboard
         </Link>
      </div>
    );
  }

  return (
    <div className="p-4 max-w-[375px] mx-auto min-h-screen bg-gray-50 flex flex-col font-inter relative pb-20">
      <div className="flex items-center mb-6 border-b border-gray-200 pb-4">
        <Link href="/dashboard" className="text-blue-500 font-semibold mr-4">&lt; Back</Link>
        <h1 className="text-xl font-bold font-outfit text-gray-900">Add Product</h1>
      </div>

      {!uploadedImage && !loading && !productData && (
        <div className="flex-1 flex flex-col items-center justify-center">
          <label className="w-full aspect-square border-2 border-dashed border-gray-300 rounded-2xl flex flex-col items-center justify-center bg-white shadow-sm cursor-pointer hover:bg-gray-50 transition-colors">
            <div className="text-4xl mb-2">📷</div>
            <span className="font-semibold text-gray-800">Take a photo or upload</span>
            <input type="file" accept="image/*" className="hidden" onChange={handleFileUpload} />
          </label>
          <p className="text-sm text-gray-500 mt-4 text-center">
            Our AI Auto-Catalog Agent will instantly write the title, description, and suggest a price.
          </p>
        </div>
      )}

      {uploadedImage && !enhancing && variations.length === 0 && !loading && !productData && (
        <div className="flex-1 flex flex-col items-center gap-6">
          <div className="w-full aspect-square rounded-2xl overflow-hidden relative shadow-sm border border-gray-200">
            <img src={uploadedImage} alt="Uploaded product" className="w-full h-full object-cover" />
          </div>
          <button
            onClick={handleMagicEnhance}
            className="w-full py-4 bg-gradient-to-r from-blue-500 to-purple-600 text-white font-bold rounded-xl shadow-lg hover:opacity-90 transition-opacity flex items-center justify-center gap-2 text-lg"
            style={{
              boxShadow: '0 4px 15px rgba(0, 102, 255, 0.3)',
            }}
          >
            <span>✨</span> Magic Enhance
          </button>
          <p className="text-sm text-gray-500 text-center px-4">
            Transform your raw photo into a professional studio-quality image instantly.
          </p>
        </div>
      )}

      {enhancing && (
        <div className="flex-1 flex flex-col items-center justify-center gap-6">
           <div className="w-full aspect-square bg-gray-200 rounded-2xl animate-pulse flex items-center justify-center relative overflow-hidden">
              {uploadedImage && <img src={uploadedImage} alt="Original" className="w-full h-full object-cover opacity-30" />}
              <div className="absolute inset-0 flex items-center justify-center">
                <div className="text-6xl animate-spin" style={{ animationDuration: '3s' }}>🪄</div>
              </div>
           </div>
           <p className="text-lg font-semibold text-purple-600 animate-pulse text-center">AI is setting up the studio...</p>
        </div>
      )}

      {variations.length > 0 && !loading && !productData && (
        <div className="flex-1 flex flex-col gap-4">
          <h2 className="text-lg font-bold text-gray-900 text-center">Select a Variation</h2>
          <div className="grid grid-cols-2 gap-4">
            {variations.map((v, idx) => (
              <div
                key={idx}
                onClick={() => handleVariationSelect(v)}
                className={`aspect-square rounded-xl overflow-hidden border-2 cursor-pointer transition-all ${selectedVariation === v ? 'border-blue-500 shadow-md scale-[1.02]' : 'border-transparent shadow-sm hover:scale-[1.01]'}`}
              >
                <img src={v} alt={`Variation ${idx + 1}`} className="w-full h-full object-cover" />
              </div>
            ))}
          </div>

          <button
            onClick={handleContinueToCatalog}
            disabled={!selectedVariation}
            className={`w-full py-4 font-bold rounded-xl shadow-md transition-all mt-4 text-lg ${selectedVariation ? 'bg-gray-900 text-white hover:bg-black' : 'bg-gray-200 text-gray-400 cursor-not-allowed'}`}
          >
            Continue
          </button>
        </div>
      )}

      {loading && variations.length > 0 && (
        <div className="flex-1 flex flex-col items-center justify-center gap-6">
           <div className="w-full aspect-square bg-gray-200 rounded-2xl flex items-center justify-center overflow-hidden">
             {selectedVariation && <img src={selectedVariation} alt="Selected" className="w-full h-full object-cover opacity-50" />}
           </div>
           <div className="w-full space-y-4">
              <div className="h-6 bg-gray-200 rounded-md animate-pulse w-3/4"></div>
              <div className="h-20 bg-gray-200 rounded-md animate-pulse w-full"></div>
              <div className="h-10 bg-gray-200 rounded-md animate-pulse w-1/3"></div>
           </div>
           <p className="text-sm font-semibold text-blue-600 animate-pulse text-center">AutoDream AI is analyzing your photo...</p>
        </div>
      )}

      {productData && !loading && (
        <div className="flex-1 flex flex-col gap-6 animate-fade-in-up">
           <div className="w-full aspect-square bg-gray-200 rounded-2xl overflow-hidden relative shadow-sm border border-gray-200">
              {selectedVariation ? (
                 <img src={selectedVariation} alt="Final product" className="w-full h-full object-cover" />
              ) : (
                <div className="absolute inset-0 bg-gradient-to-tr from-blue-100 to-purple-100 flex items-center justify-center">
                   <div className="text-6xl">🧁</div>
                </div>
              )}
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
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                  />
              </div>
              <div>
                  <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Description</label>
                  <textarea
                    value={productData.description}
                    onChange={(e) => setProductData({...productData, description: e.target.value})}
                    rows={4}
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-sm text-gray-800 leading-relaxed focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                  />
              </div>
              <div className="flex gap-4">
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Price</label>
                      <div className="relative">
                          <span className="absolute left-3 top-2 text-gray-500 font-semibold">$</span>
                          <input
                            type="text"
                            value={productData.price}
                            onChange={(e) => setProductData({...productData, price: e.target.value})}
                            className="w-full bg-white/50 border border-white/60 rounded-[8px] pl-7 pr-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                          />
                      </div>
                  </div>
                  <div className="flex-1">
                      <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Category</label>
                      <input
                        type="text"
                        value={productData.category}
                        onChange={(e) => setProductData({...productData, category: e.target.value})}
                        className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                      />
                  </div>
              </div>
              <div className="mt-4 border-t border-white/40 pt-4">
                  <label className="flex items-center cursor-pointer">
                      <div className="relative">
                          <input type="checkbox" className="sr-only" checked={productData?.isSubscription || false} onChange={(e) => setProductData({...productData!, isSubscription: e.target.checked})} />
                          <div className={`block w-10 h-6 rounded-full transition-colors ${productData?.isSubscription ? 'bg-blue-500' : 'bg-gray-300'}`}></div>
                          <div className={`dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform ${productData?.isSubscription ? 'transform translate-x-4' : ''}`}></div>
                      </div>
                      <div className="ml-3 text-gray-800 font-semibold text-sm">
                          Offer as Subscription
                      </div>
                  </label>

                  {productData?.isSubscription && (
                      <div className="mt-4 flex gap-4 animate-fade-in-up">
                          <div className="flex-1">
                              <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Deliver every</label>
                              <select
                                  value={productData.subscriptionInterval || 'Month'}
                                  onChange={(e) => setProductData({...productData, subscriptionInterval: e.target.value})}
                                  className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                              >
                                  <option value="Week">Week</option>
                                  <option value="Month">Month</option>
                                  <option value="Year">Year</option>
                              </select>
                          </div>
                          <div className="flex-1">
                              <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Discount (%)</label>
                              <input
                                  type="text"
                                  value={productData.subscriptionDiscount || '10'}
                                  onChange={(e) => setProductData({...productData, subscriptionDiscount: e.target.value})}
                                  className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                              />
                          </div>
                      </div>
                  )}
              </div>
           </div>

           <button
             onClick={handlePublish}
             className="w-full py-3.5 bg-[#0066FF] text-white font-bold rounded-[8px] shadow-md hover:bg-blue-600 transition-colors text-lg"
           >
             Publish Product
           </button>
        </div>
      )}
    </div>
  );
}
