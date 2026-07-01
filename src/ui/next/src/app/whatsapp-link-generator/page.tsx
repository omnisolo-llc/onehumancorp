'use client';

import React, { useState, useEffect } from 'react';
import Head from 'next/head';
import '../globals.css';

export default function WhatsAppLinkGeneratorPage() {
  const [phoneNumber, setPhoneNumber] = useState('');
  const [message, setMessage] = useState('');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [copied, setCopied] = useState(false);
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [tenant, setTenant] = useState("default");

  useEffect(() => {
    if (typeof localStorage !== "undefined") {
      const storedTenant = localStorage.getItem("tenant");
      if (storedTenant) setTenant(storedTenant);
    }
  }, []);

  const brandingText = `⚡ Powered by OHC`;

  const finalMessage = React.useMemo(() => {
    let finalStr = message.trim();
    if (!removeBranding) {
      if (finalStr.length > 0) {
        finalStr += `\n\n${brandingText}`;
      } else {
        finalStr = brandingText;
      }
    }
    return finalStr;
  }, [message, removeBranding]);

  const cleanPhoneNumber = phoneNumber.replace(/\D/g, '');
  const generatedLink = cleanPhoneNumber
    ? `https://wa.me/${cleanPhoneNumber}?text=${encodeURIComponent(finalMessage)}\n\nhttps://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`
    : '';

  const handleCopy = async () => {
    if (!generatedLink) return;
    try {
      await navigator.clipboard.writeText(generatedLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy text: ', err);
    }
  };

  const handleBrandingToggle = () => {
    if (!removeBranding) {
      setShowPaywall(true);
    } else {
      setRemoveBranding(false);
    }
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return { backgroundColor: '#1f2937', color: '#f9fafb' };
    }
    return { backgroundColor: '#ffffff', color: '#111827' };
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900 font-inter">
      <Head>
        <title>WhatsApp Link Generator | OHC</title>
      </Head>

      <nav className="p-6 border-b border-gray-100 bg-white/80 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-10">
        <div className="max-w-6xl mx-auto flex justify-between items-center">
            <div className="flex items-center gap-2">
                <div className="w-8 h-8 bg-indigo-600 rounded-lg flex items-center justify-center text-white font-bold text-xl shadow-lg shadow-indigo-600/20">
                    O
                </div>
                <span className="font-outfit font-bold text-xl tracking-tight">OHC</span>
            </div>
            <a href="/dashboard" className="text-sm font-medium text-gray-500 hover:text-indigo-600 transition-colors">
                Back to Dashboard
            </a>
        </div>
      </nav>

      <main className="max-w-6xl mx-auto p-6 pt-12 flex flex-col md:flex-row gap-12">
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-8 shadow-xl border border-gray-100 bg-white/60 backdrop-blur-[30px] saturate-[210%]">
                <h1 className="text-3xl font-bold font-outfit mb-2 bg-clip-text text-transparent bg-gradient-to-r from-indigo-600 to-purple-600">
                    WhatsApp Link Generator 📱
                </h1>
                <p className="text-gray-500 mb-8 text-sm">
                    Create click-to-chat links for WhatsApp to help customers reach you instantly.
                </p>

                <div className="mb-6">
                    <label htmlFor="phoneNumber" className="block text-sm font-medium text-gray-700 mb-2">WhatsApp Phone Number</label>
                    <input
                        id="phoneNumber"
                        type="text"
                        className="w-full px-4 py-3 bg-white border border-gray-200 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]/20 focus:border-indigo-500 transition-all font-inter"
                        placeholder="e.g. 1234567890"
                        value={phoneNumber}
                        onChange={(e) => setPhoneNumber(e.target.value)}
                    />
                    <p className="text-xs text-gray-400 mt-2">Include the country code without any '+' or '-' signs.</p>
                </div>

                <div className="mb-6">
                    <label htmlFor="message" className="block text-sm font-medium text-gray-700 mb-2">Pre-filled Message</label>
                    <textarea
                        id="message"
                        className="w-full px-4 py-3 bg-white border border-gray-200 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]/20 focus:border-indigo-500 transition-all font-inter resize-none h-32"
                        placeholder="Hello, I would like to inquire about your services."
                        value={message}
                        onChange={(e) => setMessage(e.target.value)}
                    />
                </div>

                <div className="mb-6">
                    <label className="flex items-center gap-3 cursor-pointer group">
                        <div className="relative flex items-center justify-center w-6 h-6">
                            <input
                                type="checkbox"
                                className="peer sr-only"
                                checked={removeBranding}
                                onChange={handleBrandingToggle}
                            />
                            <div className="w-5 h-5 border-2 border-gray-300 rounded transition-colors peer-checked:bg-indigo-600 peer-checked:border-indigo-600 group-hover:border-indigo-400"></div>
                            <svg className="absolute w-3 h-3 text-white opacity-0 peer-checked:opacity-100 pointer-events-none transition-opacity" fill="none" viewBox="0 0 24 24" stroke="currentColor" strokeWidth="3">
                                <path strokeLinecap="round" strokeLinejoin="round" d="M5 13l4 4L19 7" />
                            </svg>
                        </div>
                        <span className="text-sm font-medium text-gray-700">Remove "Powered by OHC" Badge (Pro)</span>
                    </label>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme (For QR/Preview)</label>
                    <div className="flex gap-2 p-1 bg-gray-100 rounded-lg">
                        <button
                            aria-label="Light theme"
                            aria-pressed={theme === 'light'}
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'light' ? 'bg-white/65 backdrop-blur-[30px] saturate-[210%] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            aria-label="Dark theme"
                            aria-pressed={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'dark' ? 'bg-white/65 backdrop-blur-[30px] saturate-[210%] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

                <button
                    onClick={() => setShowModal(true)}
                    disabled={!phoneNumber}
                    className={`w-full py-3 text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm ${phoneNumber ? 'bg-indigo-600 hover:bg-indigo-700' : 'bg-indigo-300 cursor-not-allowed'}`}
                >
                    Get Link
                </button>
            </div>

            <div className="p-6 bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 shadow-sm">
                <h3 className="text-md font-semibold font-outfit mb-2 flex items-center gap-2">
                    <span className="text-xl">🚀</span> Instant Chat
                </h3>
                <p className="text-sm text-gray-600 leading-relaxed">
                    Make it easy for leads to text you instantly. Add this link to your Instagram bio, Linktree, or website.
                </p>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-2/3">
            <div className="p-8 h-full flex flex-col items-center justify-center relative overflow-hidden bg-gradient-to-br from-gray-100 to-gray-200 border border-white/50 shadow-inner">
                <div className="absolute top-4 left-4 text-xs font-semibold text-gray-400 uppercase tracking-wider">Live Preview</div>

                {/* The Widget Preview */}
                <div className="relative w-full max-w-sm shadow-xl overflow-hidden font-inter flex flex-col" style={getThemeStyles()}>
                    <div className="bg-[#075e54] p-4 flex items-center gap-3 text-white">
                        <div className="w-10 h-10 rounded-full bg-white/20 flex items-center justify-center">
                            <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" /></svg>
                        </div>
                        <div>
                            <div className="font-semibold text-sm">+{cleanPhoneNumber || '1234567890'}</div>
                            <div className="text-xs text-white/70">Online</div>
                        </div>
                    </div>

                    <div className="flex-1 p-4 bg-[#e5ddd5] flex flex-col justify-end min-h-[200px]" style={{ backgroundImage: 'url("/whatsapp-bg.png")', backgroundSize: 'cover' }}>
                        <div className="self-end max-w-[85%] rounded-lg p-3 shadow-sm" style={{ backgroundColor: '#dcf8c6', color: '#303030' }}>
                            <p className="text-sm whitespace-pre-wrap">{finalMessage}</p>
                            <div className="text-[10px] text-right mt-1 opacity-70">12:00 PM</div>
                        </div>
                    </div>
                </div>

                <div className="mt-8 text-center max-w-md text-sm text-gray-500">
                    This is what the customer will see when they click your link.
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

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Your WhatsApp Link</h2>
                <p className="text-gray-600 mb-6">Copy this link and share it anywhere.</p>

                <div className="relative group mb-6">
                    <textarea
                        readOnly
                        value={generatedLink}
                        className="w-full h-24 p-4 bg-gray-50 border border-gray-200 min-h-[44px] min-w-[44px] font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-[#0066FF]/20 focus:border-indigo-500 transition-all"
                    />
                </div>

                <div className="flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Link'}
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

      {/* Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="bg-white p-8 max-w-md w-full shadow-2xl relative animate-in fade-in zoom-in-95 duration-200">
                <button
                    aria-label="Close modal"
                    onClick={() => setShowPaywall(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <div className="w-16 h-16 bg-gradient-to-br from-amber-100 to-orange-100 flex items-center justify-center mb-6 shadow-inner border border-amber-200/50">
                    <span className="text-3xl">⭐</span>
                </div>

                <h2 className="text-2xl font-bold font-outfit mb-3 text-gray-900">Upgrade to Pro</h2>
                <p className="text-gray-600 mb-6 leading-relaxed">
                    Make your links 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark and unlock advanced analytics.
                </p>

                <div className="space-y-4 mb-8">
                    <div className="flex items-center gap-3 text-sm text-gray-700">
                        <svg className="w-5 h-5 text-[#34C759] flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                        Remove OHC branding
                    </div>
                    <div className="flex items-center gap-3 text-sm text-gray-700">
                        <svg className="w-5 h-5 text-[#34C759] flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                        Custom URL slugs
                    </div>
                    <div className="flex items-center gap-3 text-sm text-gray-700">
                        <svg className="w-5 h-5 text-[#34C759] flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path></svg>
                        Click analytics
                    </div>
                </div>

                <div className="flex gap-3">
                    <button className="flex-1 py-3 bg-gradient-to-r from-gray-900 to-gray-800 text-white font-medium min-h-[44px] min-w-[44px] hover:from-black hover:to-gray-900 transition-all shadow-md hover:shadow-lg transform hover:-translate-y-0.5">
                        Upgrade Now
                    </button>
                </div>
            </div>
        </div>
      )}

      {/* Persistent Footer Growth Loop */}
      <footer className="mt-12 py-8 border-t border-gray-200 text-center">
          <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`}  target="_blank" rel="noopener noreferrer" className="inline-flex items-center gap-2 text-sm font-semibold text-gray-500 hover:text-indigo-600 transition-colors">
              <span className="text-base">⚡</span> Powered by OHC
          </a>
      </footer>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
