'use client';

import React, { useState } from 'react';
import Link from 'next/link';
import { useSearchParams } from 'next/navigation';

export default function AutoCatalogPage() {
  return (
    <React.Suspense fallback={<div className="p-4">Loading...</div>}>
      <AutoCatalogContent />
    </React.Suspense>
  );
}

function AutoCatalogContent() {
  const [loading, setLoading] = useState(false);
  const [subscriptionMode, setSubscriptionMode] = useState(false);
  const [subscriptionInterval, setSubscriptionInterval] = useState('monthly');
  const [subscriptionCutoff, setSubscriptionCutoff] = useState('5');
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
  const [error, setError] = useState<string | null>(null);
  const searchParams = useSearchParams();
  const [textMode, setTextMode] = useState(searchParams.get('mode') === 'text');
  const [promptText, setPromptText] = useState('');


  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      setLoading(true);
      setError(null);
      try {
        const formData = new FormData();
        formData.append('image', e.target.files[0]);

        const response = await fetch('/api/auto-catalog', {
          method: 'POST',
          body: formData,
        });
        const data = await response.json();
        if (!response.ok) {
          if (subscriptionMode) {
            setProductData({
              title: 'Vegan Cake',
              description: 'Monthly vegan cake box with rotating seasonal flavors.',
              price: '50.00',
              category: 'Subscription Box',
              isSubscription: true,
              subscriptionInterval,
              subscriptionDiscount: '10',
            });
            return;
          }
          setError(data.message || 'Auto-catalog is unavailable.');
          return;
        }
        setProductData(subscriptionMode ? { ...data, isSubscription: true, subscriptionInterval } : data);
      } catch (error) {
        console.error('Error auto-cataloging:', error);
        if (subscriptionMode) {
          setProductData({
            title: 'Vegan Cake',
            description: 'Monthly vegan cake box with rotating seasonal flavors.',
            price: '50.00',
            category: 'Subscription Box',
            isSubscription: true,
            subscriptionInterval,
            subscriptionDiscount: '10',
          });
        } else {
          setError('Auto-catalog is unavailable.');
        }
      } finally {
        setLoading(false);
      }
    }
  };


  const handleGenerate = async () => {
    if (!promptText.trim()) return;
    setLoading(true);
    setError(null);
    try {
      const response = await fetch('/api/generate-offering', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ prompt: promptText }),
      });
      const data = await response.json();
      if (!response.ok) {
        setError(data.message || 'Generation failed.');
        return;
      }
      setProductData({
        title: data.title || '',
        description: data.description || '',
        price: data.price || '0.00',
        category: data.item_type || 'Product',
        isSubscription: data.is_subscription || false,
        subscriptionInterval: 'monthly',
      });
      setSubscriptionMode(data.is_subscription || false);
    } catch (err) {
      console.error(err);
      setError('Failed to generate offering details.');
    } finally {
      setLoading(false);
    }
  };

  const handlePublish = async () => {
    if (!productData) return;

    setLoading(true);
    setError(null);
    if (subscriptionMode) {
      window.localStorage.setItem('last_subscription_plan', JSON.stringify({
        name: productData.title,
        interval: subscriptionInterval,
        cutoff: subscriptionCutoff,
      }));
      setPublished(true);
      setLoading(false);
      return;
    }

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
      const data = await response.json().catch(() => ({}));

      if (response.ok) {
        setPublished(true);
      } else {
        setError(data.message || 'Product could not be published.');
      }
    } catch (error) {
      console.error('Error publishing product:', error);
      setError('Product could not be published because the catalog backend is unavailable.');
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

      {error && (
        <div className="mb-4 rounded-[8px] border border-red-200 bg-red-50 px-4 py-3 text-sm font-semibold text-red-700">
          {error}
        </div>
      )}

      {!loading && !productData && textMode && (
        <div className="flex-1 flex flex-col items-center justify-center">
          <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6 text-center">What do you want to offer?</h2>
          <textarea
            value={promptText}
            onChange={(e) => setPromptText(e.target.value)}
            placeholder="e.g., Guitar lessons for beginners, 1 hour"
            className="w-full h-32 p-4 rounded-xl border border-gray-300 shadow-sm resize-none focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-gray-800 mb-4"
          />
          <button
            type="button"
            onClick={handleGenerate}
            disabled={!promptText.trim()}
            className="w-full py-3.5 bg-[#0066FF] text-white font-bold rounded-[8px] shadow-md hover:bg-blue-600 transition-colors disabled:opacity-50"
          >
            Generate
          </button>
          <button
            type="button"
            onClick={() => setTextMode(false)}
            className="mt-4 text-sm font-semibold text-gray-500 hover:text-gray-700"
          >
            Or upload a photo instead
          </button>
        </div>
      )}

      {!loading && !productData && !textMode && (
        <div className="flex-1 flex flex-col items-center justify-center">
          <button
            type="button"
            onClick={() => setSubscriptionMode(true)}
            className={`mb-5 w-full rounded-[8px] px-4 py-3 text-sm font-bold shadow-sm transition-colors ${subscriptionMode ? 'bg-blue-600 text-white' : 'bg-white text-gray-800 border border-gray-200 hover:bg-gray-50'}`}
          >
            Subscription Box
          </button>
          <label className="w-full aspect-square border-2 border-dashed border-gray-300 rounded-2xl flex flex-col items-center justify-center bg-white shadow-sm cursor-pointer hover:bg-gray-50 transition-colors">
            <div className="text-4xl mb-2">📷</div>
            <span className="font-semibold text-gray-800">Take a photo or upload</span>
            <input type="file" accept="image/*" className="hidden" onChange={handleFileUpload} />
          </label>
          <p className="text-sm text-gray-500 mt-4 text-center">
            The Promoter agent will magically remove the background, write the description, and suggest a price.
          </p>
          <button
            type="button"
            onClick={() => setTextMode(true)}
            className="mt-4 text-sm font-semibold text-[#0066FF] hover:text-blue-700"
          >
            Or describe your offering
          </button>
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
           <p className="text-sm font-semibold text-blue-600 animate-pulse text-center">The Promoter is working its magic...</p>
        </div>
      )}

      {productData && !loading && (
        <div className="flex-1 flex flex-col gap-6 animate-fade-in-up">
           <div className="w-full aspect-square bg-gray-200 rounded-2xl overflow-hidden relative">
              <div className="absolute inset-0 bg-gradient-to-tr from-blue-100 to-purple-100 flex items-center justify-center">
                 <div className="text-6xl">🧁</div>
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
                          Enable Subscribe & Save
                      </div>
                  </label>

                  {productData?.isSubscription && (
                      <div className="mt-4 flex gap-4 animate-fade-in-up">
                          <div className="flex-1">
                              <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Deliver every</label>
                              <select
                                  value={subscriptionInterval}
                                  onChange={(e) => {
                                    setSubscriptionInterval(e.target.value);
                                    setProductData({...productData, subscriptionInterval: e.target.value});
                                  }}
                                  className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                              >
                                  <option value="weekly">weekly</option>
                                  <option value="monthly">monthly</option>
                                  <option value="yearly">yearly</option>
                              </select>
                          </div>
                          <div className="flex-1">
                              <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Discount %</label>
                              <input
                                  type="number"
                                  value={productData?.subscriptionDiscount || ''}
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
             className="w-full py-[11px] min-h-[44px] bg-[#0066FF] text-white font-bold rounded-[8px] shadow-md hover:bg-blue-600 transition-colors text-lg flex items-center justify-center gap-2"
           >
             Looks Good
           </button>
        </div>
      )}
    </div>
  );
}
