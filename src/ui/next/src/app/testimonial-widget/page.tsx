'use client';

import React, { useState, useEffect } from 'react';
import Head from 'next/head';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function TestimonialWidgetGenerator() {
  const [tenant, setTenant] = useState('my-business');
  const [authorName, setAuthorName] = useState('Jane Doe');
  const [reviewText, setReviewText] = useState('Absolutely wonderful! They exceeded my expectations and I will definitely be coming back. Highly recommend to everyone.');
  const [rating, setRating] = useState('5');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  const embedUrl = `https://ohc.app/api/v1/growth/testimonial/embed?tenant=${encodeURIComponent(tenant)}&authorName=${encodeURIComponent(authorName)}&reviewText=${encodeURIComponent(reviewText)}&rating=${rating}&theme=${theme}`;
  const embedCode = `<iframe src="${embedUrl}" width="100%" height="250" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
      if (theme === 'dark') {
          return { background: '#1f2937', color: '#f9fafb', border: '1px solid #374151' };
      }
      return { background: '#ffffff', color: '#111827', border: '1px solid #e5e7eb' };
  };

  if (!isClient) return null; // Avoid hydration mismatch on the live preview

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      <Head>
        <title>Testimonial Widget Builder | OHC</title>
      </Head>

      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Testimonial Widget 🌟</h1>
        <button className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Controls */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-lg glassmorphism border border-white/40">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Widget Settings</h2>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Reviewer Name</label>
                    <input
                        type="text"
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                        placeholder="e.g. Jane Doe"
                        value={authorName}
                        onChange={(e) => setAuthorName(e.target.value)}
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Review Text</label>
                    <textarea
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] resize-none h-24"
                        placeholder="Write a glowing review..."
                        value={reviewText}
                        onChange={(e) => setReviewText(e.target.value)}
                    />
                </div>

                <div className="flex gap-4 mb-4">
                    <div className="flex-1">
                        <label className="block text-sm font-medium text-gray-700 mb-2">Rating (1-5)</label>
                        <select
                            className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                            value={rating}
                            onChange={(e) => setRating(e.target.value)}
                        >
                            <option value="5">5 Stars</option>
                            <option value="4">4 Stars</option>
                            <option value="3">3 Stars</option>
                            <option value="2">2 Stars</option>
                            <option value="1">1 Star</option>
                        </select>
                    </div>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex gap-2 p-1 bg-gray-100 rounded-lg">
                        <button
                            aria-pressed={theme === 'light'}
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'light' ? 'bg-white/65 backdrop-blur-[30px] saturate-[210%] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            aria-pressed={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'dark' ? 'bg-white/65 backdrop-blur-[30px] saturate-[210%] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Store ID (Tenant)</label>
                    <input
                        type="text"
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                        placeholder="e.g. my-store"
                        value={tenant}
                        onChange={(e) => setTenant(e.target.value)}
                    />
                </div>

                <button
                    onClick={() => setShowModal(true)}
                    className="w-full py-3 bg-indigo-600 text-white font-medium min-h-[44px] min-w-[44px] hover:bg-indigo-700 transition-colors shadow-sm"
                >
                    Get Widget Code
                </button>
            </div>

            <div className="p-6 glassmorphism border border-white/40">
                <h3 className="text-md font-semibold font-outfit mb-2 flex items-center gap-2">
                    <span className="text-xl">🏆</span> Build Trust
                </h3>
                <p className="text-sm text-gray-600 leading-relaxed">
                    Social proof drives sales. Add this widget to your landing page, blog, or Notion doc to showcase your best feedback and convert more visitors.
                </p>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-2/3">
            <div className="p-8 h-full flex flex-col items-center justify-center relative overflow-hidden bg-gradient-to-br from-gray-100 to-gray-200 border border-white/50 shadow-inner">
                <div className="absolute top-4 left-4 text-xs font-semibold text-gray-400 uppercase tracking-wider">Live Preview</div>

                {/* The Widget Preview */}
                <div className="relative w-full max-w-md shadow-xl overflow-hidden" style={getThemeStyles()}>
                    <div className="p-6 flex flex-col text-left">
                        <div className="text-2xl text-yellow-400 mb-3 tracking-widest">
                            {'★'.repeat(parseInt(rating)) + '☆'.repeat(5 - parseInt(rating))}
                        </div>

                        <p className="text-lg italic mb-5 leading-relaxed" style={{ color: theme === 'dark' ? '#d1d5db' : '#374151' }}>
                            "{reviewText}"
                        </p>

                        <div className="flex items-center gap-3 font-bold" style={{ color: theme === 'dark' ? '#f9fafb' : '#111827' }}>
                            <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center">
                                <svg className="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" /></svg>
                            </div>
                            {authorName}
                        </div>

                        <div className={`mt-5 pt-4 border-t flex justify-center ${theme === 'dark' ? 'border-gray-700' : 'border-gray-100'}`}>
                            <PoweredByOHC tenantId={tenant} />
                        </div>
                    </div>
                </div>

                <div className="mt-8 text-center max-w-md text-sm text-gray-500">
                    This preview shows exactly how the widget will look when embedded on your website using the generated iframe code.
                </div>
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="bg-white p-8 max-w-xl w-full shadow-2xl relative animate-in fade-in zoom-in-95 duration-200">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Testimonial</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website, blog, or Notion page.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 min-h-[44px] min-w-[44px] font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-[#0066FF]/20 focus:border-indigo-500 transition-all"
                    />
                    <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                         <button
                            onClick={handleCopy}
                            className="p-2 bg-white rounded-lg border shadow-sm text-gray-600 hover:text-indigo-600 transition-colors"
                            title="Copy to clipboard"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2 2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                        </button>
                    </div>
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm flex items-center justify-center gap-2"
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
          border: 1px solid rgba(255, 255, 255, 0.4);
        }
        @media (prefers-color-scheme: dark) {
          .glassmorphism {
            background: rgba(22, 22, 26, 0.7);
            border: 1px solid rgba(255, 255, 255, 0.1);
          }
        }
      `}} />
    </div>
  );
}