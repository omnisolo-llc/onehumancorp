"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ViralPostGeneratorPage() {
  const router = useRouter();
  const [productIdea, setProductIdea] = useState('');
  const [result, setResult] = useState('');
  const [isGenerating, setIsGenerating] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [generationCount, setGenerationCount] = useState(0);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
      setGenerationCount(parseInt(localStorage.getItem('viral_post_count') || '0', 10));
    }
  }, []);

  const handleGenerate = () => {
    if (!hasPro && generationCount >= 1) {
      setShowSoftPaywall(true);
      return;
    }

    setIsGenerating(true);
    setTimeout(() => {
      const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
      const storeLink = `https://${tenant}.ohc.store`;

      const ideas = [
        `OMG guys, I just launched my new ${productIdea} and I'm OBSESSED! 😍🔥 It's exactly what you need right now.\n\nGrab yours before they sell out! 👇\n${storeLink}\n\n#SmallBusiness #${productIdea.replace(/\s+/g, '')} #MustHave`,
        `🚨 DROP ALERT! 🚨 The wait is finally over. The best ${productIdea} is now live on my store! ✨💯\n\nCheck it out here: ${storeLink}\n\n#ShopLocal #NewDrop #${productIdea.replace(/\s+/g, '')}`,
        `Everyone's been asking about my ${productIdea}, so I finally put it up on my shop! 🛍️💖\n\nLink in bio or tap here to shop: ${storeLink}\n\n#Trending #${productIdea.replace(/\s+/g, '')} #SupportSmallBusiness`
      ];

      const randomIdea = ideas[Math.floor(Math.random() * ideas.length)];
      setResult(randomIdea);
      setIsGenerating(false);

      const newCount = generationCount + 1;
      setGenerationCount(newCount);
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('viral_post_count', newCount.toString());
      }
    }, 1200);
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ohc://join?ref=' + tenant)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
    setTimeout(() => {
      alert('Your 7-day Pro trial has been activated.');
      handleGenerate();
    }, 500);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>AI Viral Post Generator 🚀</h1>
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
            <h2 className="text-xl font-semibold font-outfit m-0" style={{ color: '#1D1D1F' }}>Generate a Post</h2>
            <div className="flex items-center gap-2 px-3 py-1 bg-purple-50 rounded-full border border-purple-100">
                <span className="text-xs font-medium text-purple-600">Growth Tool</span>
            </div>
          </div>
          <p className="text-sm text-gray-600 mb-4">Turn your products into viral social media posts optimized for X and WhatsApp. Get your first customer today!</p>
          <div className="flex flex-col gap-4">
            <div>
              <label htmlFor="product-idea" className="block text-sm font-medium text-gray-700 mb-1">What are you selling?</label>
              <input
                id="product-idea"
                type="text"
                value={productIdea}
                onChange={(e) => setProductIdea(e.target.value)}
                placeholder="e.g. Handmade Soy Candles"
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <button
              onClick={handleGenerate}
              disabled={!productIdea || isGenerating}
              className={`w-full py-3 mt-2 text-white font-semibold rounded-xl shadow-lg transition-all ${(!productIdea || isGenerating) ? 'bg-indigo-400 cursor-not-allowed' : 'bg-indigo-600 hover:bg-indigo-700 hover:shadow-xl hover:-translate-y-0.5 active:translate-y-0'}`}
            >
              {isGenerating ? 'Generating Magic...' : 'Generate Viral Post'}
            </button>
          </div>
        </section>

        {result && (
          <section id="promo-result" className="p-6 shadow-sm flex flex-col gap-4 relative overflow-hidden" style={{ background: '#ffffff', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
            <h3 className="text-xl font-bold font-outfit text-gray-900">Your AI Draft</h3>

            <div className="bg-gray-50 border border-gray-100 rounded-xl p-4">
               <pre className="whitespace-pre-wrap text-sm text-gray-800 font-inter font-medium leading-relaxed" style={{ fontFamily: 'inherit' }}>
                 {result}
               </pre>
            </div>

            <div className="flex flex-col sm:flex-row gap-3 mt-2">
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(result)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex-1 flex items-center justify-center gap-2 bg-black text-white py-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                  Post to X
                </a>
                <a
                  href={`https://wa.me/?text=${encodeURIComponent(result)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex-1 flex items-center justify-center gap-2 bg-[#25D366] text-white py-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                  Send to WhatsApp
                </a>
            </div>
          </section>
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

            <div className="text-5xl mb-4">🚀</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              You've used your free viral post generation. Upgrade to our Pro plan to unlock unlimited AI marketing tools and acquire more customers!
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
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm hover:bg-gray-50 flex items-center justify-center gap-2"
              style={{ color: '#1DA1F2', border: '2px solid #1DA1F2', background: 'white' }}
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
