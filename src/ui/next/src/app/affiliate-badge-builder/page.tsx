"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function AffiliateBadgeBuilderPage() {
  const [tenantId, setTenantId] = useState('my-store');
  const [theme, setTheme] = useState('dark');
  const [text, setText] = useState('Powered by OHC');
  const [copied, setCopied] = useState(false);
  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    const tenant = localStorage.getItem('tenant') || 'my-store';
    setTenantId(tenant);
  }, []);

  const badgeUrl = `/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenantId)}&source=affiliate_badge`;

  const getBadgeStyle = () => {
    if (theme === 'dark') {
      return 'background-color: #111827; color: #ffffff; border: 1px solid #374151;';
    } else if (theme === 'light') {
      return 'background-color: #ffffff; color: #111827; border: 1px solid #e5e7eb; box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);';
    } else {
      // Indigo theme
      return 'background-color: #4f46e5; color: #ffffff; border: 1px solid #4338ca;';
    }
  };

  const getIconColor = () => {
    if (theme === 'dark') return '#fbbf24'; // Yellow
    if (theme === 'light') return '#4f46e5'; // Indigo
    return '#fbbf24'; // Yellow
  };

  const embedCode = `<!-- OHC Affiliate Badge -->
<a href="${typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app'}${badgeUrl}" target="_blank" rel="noopener noreferrer" style="display: inline-flex; align-items: center; justify-content: center; gap: 8px; padding: 8px 16px; border-radius: 9999px; font-family: system-ui, -apple-system, sans-serif; font-size: 13px; font-weight: 600; text-decoration: none; transition: all 0.2s ease; ${getBadgeStyle()}" onmouseover="this.style.opacity='0.9'; this.style.transform='translateY(-1px)';" onmouseout="this.style.opacity='1'; this.style.transform='none';">
  <span style="font-size: 16px; color: ${getIconColor()};">⚡</span>
  ${text}
</a>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-4 md:px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white backdrop-blur-[30px] saturate-[210%] border-white/40">
        <h1 className="text-xl md:text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Affiliate Badge Builder</h1>
        <Link href="/dashboard" className="px-3 py-1.5 md:px-4 md:py-2 bg-gray-200 rounded-md text-xs md:text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </Link>
      </header>

      <main className="p-4 md:p-8 flex-1 w-full max-w-6xl mx-auto">
        <div className="text-center mb-10 max-w-2xl mx-auto">
          <div className="w-16 h-16 mx-auto bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center text-3xl shadow-lg mb-6 text-white">
            ⚡
          </div>
          <h2 className="text-3xl md:text-4xl font-bold font-outfit text-gray-900 mb-4 tracking-tight">Share OHC & Earn Credits</h2>
          <p className="text-gray-600 text-lg leading-relaxed">
            Create a custom affiliate badge to put on your website, blog, or Link-in-Bio. When another business signs up through your badge, you earn $50 in OHC platform credits!
          </p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 items-start">
          {/* Controls */}
          <div className="app-card p-6 shadow-xl w-full">
            <h3 className="text-xl font-bold font-outfit text-gray-900 mb-6">Customize Your Badge</h3>

            <div className="space-y-6">
              <div>
                <label htmlFor="badge-text" className="block text-sm font-semibold text-gray-700 mb-2">Badge Text</label>
                <input
                  id="badge-text"
                  type="text"
                  value={text}
                  onChange={(e) => setText(e.target.value)}
                  maxLength={30}
                  className="w-full px-4 py-3 bg-white border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 text-gray-900 font-medium"
                />
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-3">Theme Style</label>
                <div className="grid grid-cols-3 gap-3">
                  <button
                    onClick={() => setTheme('dark')}
                    className={`py-3 px-4 rounded-xl border-2 transition-all font-medium flex items-center justify-center gap-2 ${theme === 'dark' ? 'border-gray-900 bg-gray-50 text-gray-900' : 'border-gray-200 bg-white text-gray-600 hover:border-gray-300'}`}
                  >
                    <div className="w-4 h-4 rounded-full bg-gray-900 border border-gray-700"></div>
                    Dark
                  </button>
                  <button
                    onClick={() => setTheme('light')}
                    className={`py-3 px-4 rounded-xl border-2 transition-all font-medium flex items-center justify-center gap-2 ${theme === 'light' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:border-gray-300'}`}
                  >
                    <div className="w-4 h-4 rounded-full bg-white border border-gray-300"></div>
                    Light
                  </button>
                  <button
                    onClick={() => setTheme('indigo')}
                    className={`py-3 px-4 rounded-xl border-2 transition-all font-medium flex items-center justify-center gap-2 ${theme === 'indigo' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:border-gray-300'}`}
                  >
                    <div className="w-4 h-4 rounded-full bg-indigo-600"></div>
                    Indigo
                  </button>
                </div>
              </div>

              <div className="pt-6 border-t border-gray-100">
                <button
                  onClick={() => setShowModal(true)}
                  className="w-full py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md hover:shadow-lg transition-all text-sm flex items-center justify-center gap-2"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" /></svg>
                  Get Embed Code
                </button>
              </div>
            </div>
          </div>

          {/* Preview */}
          <div className="app-card p-6 shadow-xl w-full flex flex-col justify-center min-h-[300px] relative overflow-hidden group">
             <div className="absolute top-4 right-4 px-3 py-1 bg-indigo-100 text-indigo-700 text-xs font-bold rounded-full tracking-wide">LIVE PREVIEW</div>

             <div className="text-center">
                 <h3 className="text-sm font-semibold text-gray-500 uppercase tracking-widest mb-8">How it looks on your site</h3>
                 <div
                     className="inline-flex items-center justify-center gap-2 px-4 py-2 rounded-full font-semibold transition-all hover:-translate-y-[1px] hover:opacity-90 cursor-pointer"
                     style={{
                         backgroundColor: theme === 'dark' ? '#111827' : theme === 'light' ? '#ffffff' : '#4f46e5',
                         color: theme === 'dark' ? '#ffffff' : theme === 'light' ? '#111827' : '#ffffff',
                         border: theme === 'light' ? '1px solid #e5e7eb' : theme === 'dark' ? '1px solid #374151' : '1px solid #4338ca',
                         boxShadow: theme === 'light' ? '0 1px 2px 0 rgba(0, 0, 0, 0.05)' : 'none',
                         fontSize: '13px',
                     }}
                 >
                     <span style={{ fontSize: '16px', color: theme === 'dark' || theme === 'indigo' ? '#fbbf24' : '#4f46e5' }}>⚡</span>
                     {text}
                 </div>
             </div>
          </div>
        </div>
      </main>

      {/* Code Generation Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-[30px] saturate-[210%] animate-in fade-in">
          <div className="bg-white rounded-3xl p-6 md:p-8 max-w-2xl w-full shadow-2xl relative">
            <button
              onClick={() => setShowModal(false)}
              className="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-full bg-gray-100 hover:bg-gray-200 text-gray-500 transition-colors"
            >
              ✕
            </button>

            <h3 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Embed Badge</h3>
            <p className="text-gray-600 mb-6 text-sm">Paste this HTML snippet into your website, blog, or Link-in-Bio provider.</p>

            <div className="relative group">
              <pre className="bg-gray-50 p-4 rounded-xl text-sm text-gray-800 font-mono overflow-x-auto border border-gray-200 whitespace-pre-wrap break-all h-48 focus:outline-none">
                {embedCode}
              </pre>
              <button
                onClick={handleCopy}
                className="absolute top-4 right-4 px-4 py-2 bg-white hover:bg-gray-50 border border-gray-200 rounded-lg text-sm font-semibold shadow-sm transition-colors text-gray-900 flex items-center gap-2"
              >
                {copied ? 'Copied!' : 'Copy Code'}
              </button>
            </div>

            <div className="mt-6 flex flex-col sm:flex-row items-center gap-4 p-4 bg-indigo-50 border border-indigo-100 rounded-xl">
               <div className="w-12 h-12 bg-white rounded-full flex items-center justify-center text-xl shadow-sm shrink-0">
                  🎁
               </div>
               <div>
                  <h4 className="font-bold text-indigo-900 text-sm">Earn while you grow</h4>
                  <p className="text-xs text-indigo-800 mt-1 leading-relaxed">
                    This badge contains your unique affiliate ID ({tenantId}). Whenever someone clicks it and subscribes to OHC, we'll automatically add $50 to your account balance.
                  </p>
               </div>
            </div>
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