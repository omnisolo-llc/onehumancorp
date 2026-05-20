"use client";

import React, { useState } from 'react';

type ProductDraft = {
  title: string;
  description: string;
  category: string;
  tags: string[];
};

export default function CatalogPage() {
  const [fileUrl, setFileUrl] = useState<string | null>(null);
  const [isProcessing, setIsProcessing] = useState(false);
  const [draft, setDraft] = useState<ProductDraft | null>(null);

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      setFileUrl(URL.createObjectURL(file));
      setIsProcessing(true);
      setDraft(null);

      // Simulate a 5-second processing state
      setTimeout(() => {
        setIsProcessing(false);
        setDraft({
          title: "Artisan Sourdough Loaf",
          description: "A rustic, naturally leavened sourdough bread baked fresh daily. Made with organic stone-ground flour and filtered water, resulting in a perfectly crisp crust and chewy, open crumb. Best enjoyed toasted with cultured butter or alongside your favorite soup.",
          category: "Bakery",
          tags: ["organic", "vegan", "fresh", "sourdough"]
        });
      }, 5000);
    }
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/60 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <button
              onClick={() => window.history.back()}
              className="w-10 h-10 flex items-center justify-center rounded-full bg-white shadow-sm border border-gray-100 text-gray-500 hover:text-gray-900 transition-colors"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
              </svg>
            </button>
            <div>
              <h1 className="text-xl font-bold font-outfit text-gray-900 tracking-tight">Smart Catalog</h1>
              <p className="text-gray-500 text-xs font-medium uppercase tracking-wider mt-1">Instant Creation</p>
            </div>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 space-y-6 pb-24">

          <div className="bg-white rounded-2xl p-5 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] border border-gray-100">
             <p className="text-sm font-medium text-gray-700 mb-4 text-center">Upload an image of your product, and our AI will instantly generate the details.</p>

             {!fileUrl && (
               <label className="flex flex-col items-center justify-center w-full h-48 border-2 border-dashed border-gray-300 rounded-xl bg-gray-50 hover:bg-gray-100 cursor-pointer transition-colors">
                  <div className="flex flex-col items-center justify-center pt-5 pb-6">
                     <svg className="w-10 h-10 mb-3 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                       <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                     </svg>
                     <p className="text-sm text-gray-500 font-semibold">Tap to upload image</p>
                     <p className="text-xs text-gray-400 mt-1">PNG, JPG up to 10MB</p>
                  </div>
                  <input type="file" className="hidden" accept="image/*" onChange={handleFileUpload} />
               </label>
             )}

             {fileUrl && (
               <div className="relative w-full h-48 rounded-xl overflow-hidden border border-gray-200">
                  <img src={fileUrl} alt="Product upload" className="w-full h-full object-cover" />
                  {isProcessing && (
                     <div className="absolute inset-0 bg-black/40 flex flex-col items-center justify-center text-white p-4 backdrop-blur-sm">
                       <div className="w-10 h-10 border-4 border-white border-t-transparent rounded-full animate-spin mb-3"></div>
                       <p className="text-sm font-bold tracking-wide">Analyzing Image...</p>
                     </div>
                  )}
               </div>
             )}
          </div>

          {/* Processing Skeleton */}
          {isProcessing && (
            <div className="bg-white/60 backdrop-blur-[30px] rounded-2xl p-5 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] border border-white/40 animate-pulse">
               <div className="h-6 bg-gray-200 rounded-md w-3/4 mb-4"></div>
               <div className="h-4 bg-gray-200 rounded-md w-1/4 mb-6"></div>

               <div className="space-y-2 mb-6">
                 <div className="h-3 bg-gray-200 rounded w-full"></div>
                 <div className="h-3 bg-gray-200 rounded w-full"></div>
                 <div className="h-3 bg-gray-200 rounded w-5/6"></div>
               </div>

               <div className="flex gap-2">
                 <div className="h-6 w-16 bg-gray-200 rounded-full"></div>
                 <div className="h-6 w-16 bg-gray-200 rounded-full"></div>
                 <div className="h-6 w-16 bg-gray-200 rounded-full"></div>
               </div>
            </div>
          )}

          {/* Generated Draft */}
          {draft && !isProcessing && (
            <div id="product-draft-card" className="bg-white/60 backdrop-blur-[30px] rounded-2xl p-5 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] border border-white/40 relative overflow-hidden">
               <div className="absolute top-0 right-0 w-24 h-24 bg-gradient-to-bl from-blue-100 to-transparent -z-10 rounded-bl-full opacity-50"></div>

               <div className="flex justify-between items-start mb-1">
                 <h2 className="text-lg font-bold font-outfit text-gray-900">{draft.title}</h2>
                 <span className="px-2 py-1 rounded-md bg-blue-50 text-blue-700 text-[10px] font-bold uppercase tracking-wider border border-blue-100 flex items-center gap-1">
                   <span>✨</span> AI Draft
                 </span>
               </div>

               <p className="text-xs font-semibold text-blue-600 mb-4">{draft.category}</p>

               <p className="text-sm text-gray-700 leading-relaxed mb-6">
                 {draft.description}
               </p>

               <div className="flex flex-wrap gap-2 mb-6">
                 {draft.tags.map(tag => (
                   <span key={tag} className="px-2 py-1 rounded-md bg-gray-100 text-gray-600 text-xs font-medium border border-gray-200">
                     #{tag}
                   </span>
                 ))}
               </div>

               <button className="w-full py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all">
                 Save Product
               </button>
            </div>
          )}

        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
