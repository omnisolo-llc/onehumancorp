"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function FlashSaleGeneratorPage() {
  const router = useRouter();
  const [saleTitle, setSaleTitle] = useState('Weekend Flash Sale!');
  const [discountCode, setDiscountCode] = useState('SAVE20');
  const [discountPercent, setDiscountPercent] = useState('20');
  const [endDate, setEndDate] = useState('');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [tenant, setTenant] = useState('my-store');
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [timeLeft, setTimeLeft] = useState({ hours: 24, minutes: 0, seconds: 0 });

  // Initialize with tomorrow's date
  useEffect(() => {
    const tomorrow = new Date();
    tomorrow.setDate(tomorrow.getDate() + 1);
    tomorrow.setHours(23, 59, 59, 0);
    const dateString = tomorrow.toISOString().slice(0, 16);
    setEndDate(dateString);
  }, []);

  // Timer logic for preview
  useEffect(() => {
    const timer = setInterval(() => {
      setTimeLeft(prev => {
        let { hours, minutes, seconds } = prev;
        if (seconds > 0) seconds--;
        else if (minutes > 0) { seconds = 59; minutes--; }
        else if (hours > 0) { seconds = 59; minutes = 59; hours--; }
        return { hours, minutes, seconds };
      });
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const embedCode = `<iframe src="https://ohc.app/api/v1/growth/flash-sale/embed?tenant=${tenant}&title=${encodeURIComponent(saleTitle)}&code=${encodeURIComponent(discountCode)}&percent=${encodeURIComponent(discountPercent)}&end=${encodeURIComponent(endDate)}&theme=${theme}" width="100%" height="250" style="border:none; border-radius:16px; overflow:hidden;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    return theme === 'light'
        ? { background: '#ffffff', color: '#1f2937', border: '1px solid #e5e7eb' }
        : { background: '#111827', color: '#f9fafb', border: '1px solid #374151' };
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-red-50 via-orange-50 to-amber-50">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Flash Sale Generator ⚡</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Settings Panel */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-lg glassmorphism border border-white/40">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Widget Settings</h2>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Sale Title</label>
                    <input
                        type="text"
                        value={saleTitle}
                        onChange={(e) => setSaleTitle(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#FF3B30]"
                        placeholder="e.g. 24-Hour Flash Sale!"
                    />
                </div>

                <div className="flex gap-4 mb-4">
                    <div className="flex-1">
                        <label className="block text-sm font-medium text-gray-700 mb-2">Discount Code</label>
                        <input
                            type="text"
                            value={discountCode}
                            onChange={(e) => setDiscountCode(e.target.value)}
                            className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#FF3B30] uppercase"
                            placeholder="e.g. FLASH20"
                        />
                    </div>
                    <div className="w-24">
                        <label className="block text-sm font-medium text-gray-700 mb-2">% Off</label>
                        <input
                            type="number"
                            value={discountPercent}
                            onChange={(e) => setDiscountPercent(e.target.value)}
                            className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#FF3B30]"
                            placeholder="20"
                        />
                    </div>
                </div>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">End Date & Time</label>
                    <input
                        type="datetime-local"
                        value={endDate}
                        onChange={(e) => setEndDate(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#FF3B30]"
                    />
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex gap-2 p-1 bg-gray-100 rounded-lg">
                        <button
                            aria-pressed={theme === 'light'}
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'light' ? 'bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            aria-pressed={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'dark' ? 'bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Store ID (Tenant)</label>
                    <input
                        type="text"
                        value={tenant}
                        onChange={(e) => setTenant(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#FF3B30]"
                        placeholder="e.g. my-store"
                    />
                </div>

                <button
                    onClick={() => setShowModal(true)}
                    className="w-full py-3 bg-[#FF3B30] text-white font-medium min-h-[44px] min-w-[44px] hover:bg-[#E02424] transition-colors shadow-sm"
                >
                    Get Widget
                </button>
            </div>

            <div className="p-6 glassmorphism border border-white/40">
                <h3 className="text-md font-semibold font-outfit mb-2 flex items-center gap-2">
                    <span className="text-xl">🔥</span> Create Urgency
                </h3>
                <p className="text-sm text-gray-600 leading-relaxed">
                    Flash sales create FOMO (Fear Of Missing Out). Stores using countdown widgets see up to a <strong>30% boost</strong> in conversion rates during the sale period.
                </p>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-2/3">
            <div className="p-8 h-full flex flex-col items-center justify-center relative overflow-hidden bg-gradient-to-br from-gray-100 to-gray-200 border border-white/50 shadow-inner">
                <div className="absolute top-4 left-4 text-xs font-semibold text-gray-400 uppercase tracking-wider">Live Preview</div>

                {/* The Widget Preview */}
                <div className="relative w-full max-w-md shadow-2xl overflow-hidden" style={getThemeStyles()}>
                    <div className="absolute top-0 left-0 w-full h-2 bg-gradient-to-r from-[#FF3B30] via-orange-500 to-yellow-500"></div>

                    <div className="p-6 flex flex-col items-center text-center">
                        <div className="w-12 h-12 bg-red-100 text-[#FF3B30] rounded-full flex items-center justify-center text-2xl mb-3 shadow-inner">
                            ⚡
                        </div>

                        <h3 className="text-xl font-bold font-outfit mb-1" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>
                            {saleTitle || 'Flash Sale'}
                        </h3>

                        <p className="text-sm mb-4" style={{ color: theme === 'dark' ? '#9ca3af' : '#4b5563' }}>
                            Get <strong className="text-[#FF3B30]">{discountPercent || '0'}% OFF</strong> your entire order
                        </p>

                        {/* Countdown Timer */}
                        <div className="flex gap-3 mb-5">
                            <div className="flex flex-col items-center">
                                <div className={`w-12 h-12 rounded-lg flex items-center justify-center font-mono text-xl font-bold ${theme === 'dark' ? 'bg-gray-800 text-white' : 'bg-gray-100 text-gray-900'} shadow-sm`}>
                                    {String(timeLeft.hours).padStart(2, '0')}
                                </div>
                                <span className="text-[10px] uppercase font-semibold mt-1 text-gray-400">Hours</span>
                            </div>
                            <div className="text-xl font-bold mt-2 text-gray-400">:</div>
                            <div className="flex flex-col items-center">
                                <div className={`w-12 h-12 rounded-lg flex items-center justify-center font-mono text-xl font-bold ${theme === 'dark' ? 'bg-gray-800 text-white' : 'bg-gray-100 text-gray-900'} shadow-sm`}>
                                    {String(timeLeft.minutes).padStart(2, '0')}
                                </div>
                                <span className="text-[10px] uppercase font-semibold mt-1 text-gray-400">Mins</span>
                            </div>
                            <div className="text-xl font-bold mt-2 text-gray-400">:</div>
                            <div className="flex flex-col items-center">
                                <div className={`w-12 h-12 rounded-lg flex items-center justify-center font-mono text-xl font-bold ${theme === 'dark' ? 'bg-gray-800 text-red-400' : 'bg-red-50 text-[#FF3B30]'} shadow-sm border border-red-100`}>
                                    {String(timeLeft.seconds).padStart(2, '0')}
                                </div>
                                <span className="text-[10px] uppercase font-semibold mt-1 text-gray-400">Secs</span>
                            </div>
                        </div>

                        {/* Discount Code & CTA */}
                        <div className="w-full flex items-stretch gap-2">
                            <div className={`flex-1 border-2 border-dashed ${theme === 'dark' ? 'border-gray-700 bg-gray-800/50' : 'border-gray-300 bg-gray-50'} min-h-[44px] min-w-[44px] flex items-center justify-center py-2 px-3 relative overflow-hidden group cursor-pointer`}
                                 onClick={() => {
                                     navigator.clipboard.writeText(discountCode);
                                     alert('Code copied to clipboard!');
                                 }}>
                                <span className="font-mono font-bold tracking-wider text-sm" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>
                                    {discountCode || 'CODE'}
                                </span>
                                <div className="absolute inset-0 bg-black/80 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                                    <span className="text-xs text-white font-bold">Click to copy</span>
                                </div>
                            </div>
                            <button
                                className="bg-[#FF3B30] hover:bg-[#E02424] text-white font-bold px-4 min-h-[44px] min-w-[44px] text-sm transition-colors shadow-md"
                            >
                                Shop Now
                            </button>
                        </div>
                    </div>
                </div>

                <div className="mt-4 text-center" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
                    <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noopener noreferrer" style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }}>
                        ⚡ Powered by OHC
                    </a>
                </div>
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="app-card p-8 max-w-xl w-full shadow-2xl relative animate-in fade-in zoom-in-95 duration-200">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Flash Sale</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website, blog, or Notion page.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 min-h-[44px] min-w-[44px] font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-[#FF3B30]/20 focus:border-[#FF3B30] transition-all"
                    />
                    <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                         <button
                            onClick={handleCopy}
                            className="p-2 bg-white rounded-lg border shadow-sm text-gray-600 hover:text-[#FF3B30] transition-colors"
                            title="Copy to clipboard"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                        </button>
                    </div>
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-[#FF3B30] hover:bg-[#E02424] text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium min-h-[44px] min-w-[44px] transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism {
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
          -webkit-backdrop-filter: blur(30px) saturate(210%);
        }
      `}} />
    </div>
  );
}
