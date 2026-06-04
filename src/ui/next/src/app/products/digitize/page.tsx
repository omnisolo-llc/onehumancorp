'use client';
import { useState } from 'react';
import Link from 'next/link';

export default function DigitizeProduct() {
  const [loading, setLoading] = useState(false);
  const [productData, setProductData] = useState<any>(null);
  const [published, setPublished] = useState(false);
  const [imagePreview, setImagePreview] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleFileUpload = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!e.target.files || e.target.files.length === 0) return;

    setLoading(true);
    setError(null);
    const file = e.target.files[0];

    const reader = new FileReader();
    reader.onload = (e) => {
      setImagePreview(e.target?.result as string);
    };
    reader.readAsDataURL(file);

    try {
      const response = await fetch('/api/v1/catalog/digitize', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ image: 'base64_encoded_image_placeholder' })
      });

      const result = await response.json();

      if (response.ok && result.success) {
          setProductData(result.data);
      } else {
          setError(result.message || "Failed to digitize product.");
      }
    } catch (err) {
      console.error('Error auto-cataloging:', err);
      setError("An unexpected error occurred while digitizing the product.");
    } finally {
      setLoading(false);
    }
  };

  const handlePublish = async () => {
    setLoading(true);
    setError(null);
    try {
      const response = await fetch('/api/v1/catalog/product', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: productData.title,
          price: productData.price,
          description: productData.description,
          item_type: productData.category,
          is_subscription: productData.isSubscription || false,
          subscription_interval: productData.subscriptionInterval,
          subscription_discount: productData.subscriptionDiscount ? parseInt(productData.subscriptionDiscount) : undefined
        })
      });

      if (response.ok) {
        setPublished(true);
      } else {
        const errorData = await response.json().catch(() => ({}));
        setError(errorData.message || "Failed to publish product.");
      }
    } catch (err) {
      console.error('Error publishing product:', err);
      setError("An unexpected error occurred while publishing the product.");
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
        <h1 className="text-xl font-bold font-outfit text-gray-900">Digitize Product</h1>
      </div>

      {error && (
        <div className="mb-4 p-3 bg-red-100 text-red-700 rounded-lg text-sm">
          {error}
        </div>
      )}

      {!loading && !productData && (
        <div className="flex-1 flex flex-col items-center justify-center">
          <label className="w-full aspect-square border-2 border-dashed border-gray-300 rounded-2xl flex flex-col items-center justify-center bg-white shadow-sm cursor-pointer hover:bg-gray-50 transition-colors">
            <div className="text-4xl mb-2">📷</div>
            <span className="font-semibold text-gray-800">Tap to Digitize</span>
            <input type="file" accept="image/*" className="hidden" onChange={handleFileUpload} />
          </label>
          <p className="text-sm text-gray-500 mt-4 text-center">
            AI will instantly remove the background, enhance the image, and generate a description.
          </p>
        </div>
      )}

      {loading && (
        <div className="flex-1 flex flex-col items-center justify-center gap-6">
           <div className="w-full aspect-square bg-gray-200 rounded-2xl animate-pulse flex items-center justify-center relative overflow-hidden">
               {imagePreview && <img src={imagePreview} className="absolute inset-0 w-full h-full object-cover opacity-50" alt="Processing preview" />}
              <div className="absolute inset-0 bg-gradient-to-b from-transparent via-blue-400/30 to-transparent animate-scan"></div>
              <div className="text-4xl animate-bounce relative z-10">✨</div>
           </div>
           <div className="w-full space-y-4">
              <div className="h-6 bg-gray-200 rounded-md animate-pulse w-3/4"></div>
              <div className="h-20 bg-gray-200 rounded-md animate-pulse w-full"></div>
              <div className="h-10 bg-gray-200 rounded-md animate-pulse w-1/3"></div>
           </div>
           <p className="text-sm font-semibold text-blue-600 animate-pulse text-center">Digitizing and extracting metadata...</p>
        </div>
      )}

      {productData && !loading && (
        <div className="flex-1 flex flex-col gap-6 animate-fade-in-up">
           <div className="w-full aspect-square bg-gray-200 rounded-2xl overflow-hidden relative">
              {imagePreview ? (
                  <img src={imagePreview} className="absolute inset-0 w-full h-full object-cover" alt="Digitized product" />
              ) : (
                  <div className="absolute inset-0 bg-gradient-to-tr from-blue-100 to-purple-100 flex items-center justify-center">
                    <div className="text-6xl">✨</div>
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
                 <span>✨</span> AI Draft
              </div>
              <div>
                  <label htmlFor="product-title" className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Title</label>
                  <input
                    id="product-title"
                    type="text"
                    value={productData.title}
                    onChange={(e) => setProductData({...productData, title: e.target.value})}
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                  />
              </div>
              <div>
                  <label htmlFor="product-description" className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Description</label>
                  <textarea
                    id="product-description"
                    value={productData.description}
                    onChange={(e) => setProductData({...productData, description: e.target.value})}
                    rows={4}
                    className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-sm text-gray-800 leading-relaxed focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                  />
              </div>
              <div className="flex gap-4">
                  <div className="flex-1">
                      <label htmlFor="product-price" className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Price</label>
                      <div className="relative">
                          <span className="absolute left-3 top-2 text-gray-500 font-semibold">$</span>
                          <input
                            id="product-price"
                            type="text"
                            value={productData.price}
                            onChange={(e) => setProductData({...productData, price: e.target.value})}
                            className="w-full bg-white/50 border border-white/60 rounded-[8px] pl-7 pr-3 py-2 text-gray-900 font-semibold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                          />
                      </div>
                  </div>
                  <div className="flex-1">
                      <label htmlFor="product-category" className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Category</label>
                      <input
                        id="product-category"
                        type="text"
                        value={productData.category}
                        onChange={(e) => setProductData({...productData, category: e.target.value})}
                        className="w-full bg-white/50 border border-white/60 rounded-[8px] px-3 py-2 text-gray-900 font-semibold text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all"
                      />
                  </div>
              </div>
              <div className="mt-2">
                 <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1">Variants (Sizes, Colors)</label>
                 <div className="flex gap-2 mt-1">
                    <span className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-semibold">Small</span>
                    <span className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-semibold">Medium</span>
                    <span className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-xs font-semibold">Large</span>
                    <span className="px-3 py-1 bg-gray-100 text-gray-500 rounded-full text-xs font-semibold border border-dashed border-gray-300">+ Add</span>
                 </div>
              </div>
           </div>

           <button
             onClick={handlePublish}
             className="w-full py-3.5 bg-[#0066FF] text-white font-bold rounded-[8px] shadow-md hover:bg-blue-600 transition-colors text-lg"
           >
             Publish to Store & Instagram
           </button>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @keyframes scan {
          0% { transform: translateY(-100%); }
          100% { transform: translateY(100%); }
        }
        .animate-scan {
          animation: scan 2s linear infinite;
        }
      `}} />
    </div>
  );
}
