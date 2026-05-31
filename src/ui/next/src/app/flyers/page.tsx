"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function FlyerDesignerPage() {
  const router = useRouter();
  const [businessName, setBusinessName] = useState('');
  const [tagline, setTagline] = useState('');
  const [themeColor, setThemeColor] = useState('#4F46E5');
  const [flyerSvg, setFlyerSvg] = useState<string | null>(null);
  const [isGenerating, setIsGenerating] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [storeUrl, setStoreUrl] = useState('');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const savedName = localStorage.getItem('business_name') || 'My Awesome Store';
      setBusinessName(savedName);
      setTagline('We are open for business!');
      setHasPro(localStorage.getItem('has_pro') === 'true');
      const tenant = localStorage.getItem('tenant') || 'my-store';
      setStoreUrl(`https://ohc.store/${tenant}`);
    }
  }, []);

  const generateFlyer = async () => {
    setIsGenerating(true);
    try {
      const response = await fetch('/api/v1/growth/flyer/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          business_name: businessName,
          tagline: tagline,
          store_url: storeUrl,
          theme_color: themeColor
        })
      });
      if (response.ok) {
        const data = await response.json();
        setFlyerSvg(data.svg);
      }
    } catch (e) {
      console.error("Failed to generate flyer", e);
    } finally {
      setIsGenerating(false);
    }
  };

  useEffect(() => {
    if (businessName && tagline && storeUrl) {
      const timer = setTimeout(() => {
        generateFlyer();
      }, 500);
      return () => clearTimeout(timer);
    }
  }, [businessName, tagline, themeColor, storeUrl]);

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just designed a beautiful flyer for my business on One Human Corp! 🚀 Launch your business today: ohc://join?ref=' + tenant)}`, '_blank');
    if (typeof localStorage !== 'undefined') {
        localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowSoftPaywall(false);
  };

  const handleDownload = () => {
    if (!flyerSvg) return;
    const blob = new Blob([flyerSvg], { type: 'image/svg+xml' });
    const url = URL.createObjectURL(blob);
    const link = document.createElement('a');
    link.href = url;
    link.download = `${businessName.replace(/\s+/g, '_')}_flyer.svg`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Flyer Designer 🎨</h1>
        <div className="flex items-center gap-3">
            <button
              onClick={() => router.push('/dashboard')}
              className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
            >
              Back to Dashboard
            </button>
            <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
        </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col lg:row gap-8 lg:flex-row">

        {/* Settings Panel */}
        <section className="w-full lg:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-md bg-white/70 backdrop-blur-xl border border-white/40 rounded-2xl">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Customize Your Flyer</h2>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-semibold text-gray-700 mb-1">Business Name</label>
                        <input
                            type="text"
                            value={businessName}
                            onChange={(e) => setBusinessName(e.target.value)}
                            className="w-full px-4 py-2 bg-white/50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            placeholder="Enter business name"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-semibold text-gray-700 mb-1">Tagline</label>
                        <input
                            type="text"
                            value={tagline}
                            onChange={(e) => setTagline(e.target.value)}
                            className="w-full px-4 py-2 bg-white/50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            placeholder="Enter a catchy tagline"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-semibold text-gray-700 mb-2">Theme Color</label>
                        <div className="flex flex-wrap gap-3">
                            {['#4F46E5', '#EF4444', '#10B981', '#F59E0B', '#3B82F6', '#EC4899'].map(color => (
                                <button
                                    key={color}
                                    onClick={() => setThemeColor(color)}
                                    className={`w-10 h-10 rounded-full border-2 transition-transform ${themeColor === color ? 'border-gray-900 scale-110' : 'border-transparent hover:scale-105'}`}
                                    style={{ backgroundColor: color }}
                                />
                            ))}
                        </div>
                    </div>

                    <div className="pt-4">
                        <button
                            onClick={() => {
                                if (!hasPro) {
                                    setShowSoftPaywall(true);
                                } else {
                                    // Pro logic would go here
                                    alert("Feature unlocked! You can now remove branding.");
                                }
                            }}
                            className="w-full py-3 px-4 rounded-xl border border-dashed border-indigo-300 text-indigo-600 font-semibold text-sm hover:bg-indigo-50 transition-colors flex items-center justify-center gap-2"
                        >
                            <span>✨</span> Remove OHC Branding
                        </button>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md bg-white/70 backdrop-blur-xl border border-white/40 rounded-2xl">
                <h3 className="font-bold font-outfit text-gray-900 mb-2">Ready to Print?</h3>
                <p className="text-sm text-gray-500 mb-4 leading-relaxed">
                    Download your flyer as a high-quality SVG and print it to display in your shop or community boards.
                </p>
                <button
                    onClick={handleDownload}
                    disabled={!flyerSvg}
                    className="w-full py-3 bg-gray-900 text-white font-bold rounded-xl shadow-lg hover:bg-black transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
                >
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
                    Download SVG
                </button>
            </div>
        </section>

        {/* Preview Panel */}
        <section className="w-full lg:w-2/3 flex flex-col gap-4">
            <div className="flex items-center justify-between">
                <h2 className="text-xl font-bold font-outfit text-gray-900">Live Preview</h2>
                {isGenerating && <span className="text-sm text-indigo-600 font-medium animate-pulse">Updating...</span>}
            </div>

            <div className="w-full bg-white rounded-3xl shadow-2xl overflow-hidden border border-gray-100 flex items-center justify-center p-8 min-h-[600px]">
                {flyerSvg ? (
                    <div
                        className="w-full max-w-[450px] shadow-2xl transition-all duration-300 transform hover:scale-[1.01]"
                        dangerouslySetInnerHTML={{ __html: flyerSvg }}
                    />
                ) : (
                    <div className="flex flex-col items-center gap-4 text-gray-400">
                        <div className="w-16 h-16 border-4 border-gray-100 border-t-indigo-500 rounded-full animate-spin"></div>
                        <p className="font-medium">Designing your flyer...</p>
                    </div>
                )}
            </div>
            <p className="text-sm text-gray-400 text-center">
                This is a high-resolution preview. Download the SVG for professional printing.
            </p>
        </section>

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

            <div className="text-5xl mb-4">✨</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Professional Flyers</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Removing the "Powered by OHC" branding and unlocking premium flyer templates is a Pro feature. Upgrade to boost your brand's authority.
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
