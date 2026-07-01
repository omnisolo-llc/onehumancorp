"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ShareToUnlockGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [campaignTitle, setCampaignTitle] = useState('Secret Weekend Deal');
  const [reward, setReward] = useState('20% Off Your Entire Order');
  const [hiddenCode, setHiddenCode] = useState('SECRET20');
  const [shareMessage, setShareMessage] = useState('I just unlocked a secret 20% discount!');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const savedTenant = localStorage.getItem('tenant');
      if (savedTenant) setTenant(savedTenant);
    }
  }, []);

  const generatedLink = `${typeof window !== 'undefined' ? window.location.origin : ''}/unlock?tenant=${tenant}&title=${encodeURIComponent(campaignTitle)}&reward=${encodeURIComponent(reward)}&code=${encodeURIComponent(hiddenCode)}&msg=${encodeURIComponent(shareMessage)}&theme=${theme}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(generatedLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    return theme === 'light'
        ? { background: '#ffffff', color: '#1f2937', border: '1px solid #e5e7eb' }
        : { background: '#111827', color: '#f9fafb', border: '1px solid #374151' };
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Share-to-Unlock Generator 🔓</h1>
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
            <div className="p-6 shadow-lg bg-white/80 backdrop-blur-[30px] saturate-[210%] border border-white/40">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Campaign Settings</h2>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Campaign Title</label>
                    <input
                        type="text"
                        value={campaignTitle}
                        onChange={(e) => setCampaignTitle(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                        placeholder="e.g. Secret Weekend Deal"
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Reward Description</label>
                    <input
                        type="text"
                        value={reward}
                        onChange={(e) => setReward(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                        placeholder="e.g. 20% Off Your Entire Order"
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Hidden Discount Code</label>
                    <input
                        type="text"
                        value={hiddenCode}
                        onChange={(e) => setHiddenCode(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] uppercase"
                        placeholder="e.g. SECRET20"
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Pre-filled Share Message</label>
                    <textarea
                        value={shareMessage}
                        onChange={(e) => setShareMessage(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                        rows={2}
                        placeholder="e.g. I just unlocked a secret 20% discount!"
                    />
                </div>

                <div className="mb-4">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex gap-2 border p-1 min-h-[44px] min-w-[44px] bg-gray-50 border-gray-200">
                        <button
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-1 px-3 rounded text-sm font-medium transition-all ${theme === 'light' ? 'bg-white/65 backdrop-blur-[30px] saturate-[210%] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-1 px-3 rounded text-sm font-medium transition-all ${theme === 'dark' ? 'bg-gray-800 shadow-sm text-white' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

            </div>

            <div className="p-6 shadow-lg bg-indigo-50 border border-indigo-100">
                <h3 className="font-bold text-indigo-900 mb-2 flex items-center gap-2">
                    <span className="text-xl">🚀</span> Share Your Link
                </h3>
                <p className="text-sm text-indigo-800 mb-4">
                    Post this link on social media. When customers click it, they'll have to share your business to unlock the discount!
                </p>

                <div className="flex items-center gap-2 bg-white min-h-[44px] min-w-[44px] border border-indigo-200 p-1 mb-4 overflow-hidden">
                    <div className="px-2 py-1 text-xs text-gray-500 truncate flex-1 font-mono">
                        {generatedLink}
                    </div>
                </div>

                <button
                    onClick={handleCopy}
                    className="w-full py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors"
                >
                    {copied ? 'Copied!' : 'Copy Link'}
                </button>
            </div>
        </div>

        {/* Live Preview Panel */}
        <div className="w-full md:w-2/3 flex flex-col">
            <div className="flex-1 shadow-xl overflow-hidden flex flex-col bg-gray-100 border border-gray-200 relative">
                <div className="bg-gray-200 py-3 px-4 flex items-center gap-2 border-b border-gray-300">
                    <div className="flex gap-1.5">
                        <div className="w-3 h-3 rounded-full bg-red-400"></div>
                        <div className="w-3 h-3 rounded-full bg-amber-400"></div>
                        <div className="w-3 h-3 rounded-full bg-green-400"></div>
                    </div>
                    <div className="mx-auto bg-white/60 text-xs text-gray-500 px-4 py-1 rounded-full w-1/2 text-center truncate">
                        Preview: Your Share-to-Unlock Page
                    </div>
                </div>

                <div className="flex-1 flex items-center justify-center p-8 bg-gray-50 overflow-y-auto">
                    {/* Simulated Widget Preview */}
                    <div
                        className="w-full max-w-sm shadow-2xl p-8 flex flex-col items-center relative overflow-hidden transition-all duration-300"
                        style={getThemeStyles()}
                    >
                        <div className="w-16 h-16 bg-purple-100 text-purple-600 rounded-full flex items-center justify-center text-3xl mb-6 shadow-inner">
                            🎁
                        </div>
                        <h2 className="text-2xl font-bold font-outfit text-center mb-2" id="preview-title" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>
                            {campaignTitle || 'Secret Deal'}
                        </h2>
                        <p className="text-center text-sm mb-8" style={{ color: theme === 'dark' ? '#9ca3af' : '#4b5563' }}>
                            Unlock your special reward: <br/>
                            <strong id="preview-reward-text" className="text-purple-500">{reward || '20% Off'}</strong>
                        </p>

                        <div className="w-full p-4 min-h-[44px] min-w-[44px] border-2 border-dashed flex items-center justify-center mb-8 relative"
                             style={{ borderColor: theme === 'dark' ? '#4b5563' : '#d1d5db', background: theme === 'dark' ? '#1f2937' : '#f3f4f6' }}>
                             <span id="preview-code" className="font-mono text-xl tracking-widest font-bold filter blur-sm select-none opacity-50" style={{ color: theme === 'dark' ? '#fff' : '#000' }}>
                                 {hiddenCode || 'SECRET'}
                             </span>
                             <div id="locked-badge" className="absolute inset-0 flex items-center justify-center">
                                 <span className="text-xs font-bold uppercase tracking-wider px-3 py-1 bg-gray-900 text-white rounded-full">
                                     Locked
                                 </span>
                             </div>
                        </div>

                        <div className="w-full space-y-3 mb-6">
                            <button className="w-full py-3 px-4 bg-black hover:bg-gray-800 text-white font-medium min-h-[44px] min-w-[44px] transition-colors flex items-center justify-center gap-2">
                                Share on X to Unlock
                            </button>
                            <button className="w-full py-3 px-4 bg-[#25D366] hover:bg-[#128C7E] text-white font-medium min-h-[44px] min-w-[44px] transition-colors flex items-center justify-center gap-2">
                                Share on WhatsApp
                            </button>
                        </div>

                        <div className="mt-4 pt-4 border-t w-full text-center" style={{ borderColor: theme === 'dark' ? '#374151' : '#e5e7eb' }}>
                            <span className="text-xs font-semibold tracking-wide" style={{ color: '#6b7280' }}>⚡ Powered by OHC</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
