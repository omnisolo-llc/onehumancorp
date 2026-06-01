"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function LinkInBioPage() {
  const router = useRouter();
  const [copied, setCopied] = useState(false);
  const [storeName, setStoreName] = useState('My Awesome Store');
  const [bio, setBio] = useState('Premium products for awesome people.');
  const [theme, setTheme] = useState('dark');

  const profileUrl = "ohc.store/my-store";

  const handleCopy = () => {
    navigator.clipboard.writeText(`https://${profileUrl}`);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeClasses = () => {
    switch (theme) {
      case 'light': return 'bg-white text-gray-900';
      case 'dark': return 'bg-gray-900 text-white';
      case 'gradient': return 'bg-gradient-to-br from-indigo-500 to-purple-600 text-white';
      default: return 'bg-gray-900 text-white';
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gray-50">
      <header className="px-6 py-4 flex items-center justify-between border-b bg-white/65 backdrop-blur-md border-gray-200 sticky top-0 z-50">
        <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Link-in-Bio Generator 🔗</h1>
        <Link href="/dashboard" className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </Link>
      </header>

      <main className="p-4 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Editor Settings */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
          <div className="p-6 bg-white rounded-2xl shadow-sm border border-gray-100">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Customize Your Profile</h2>
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Store Name</label>
                <input
                  type="text"
                  value={storeName}
                  onChange={(e) => setStoreName(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:outline-none"
                />
              </div>
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-1">Bio / Tagline</label>
                <textarea
                  rows={2}
                  value={bio}
                  onChange={(e) => setBio(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:outline-none resize-none"
                />
              </div>
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">Theme</label>
                <div className="flex gap-3">
                  <button onClick={() => setTheme('dark')} className={`w-10 h-10 rounded-full border-2 bg-gray-900 ${theme === 'dark' ? 'border-indigo-600' : 'border-transparent'}`} title="Dark Theme"></button>
                  <button onClick={() => setTheme('light')} className={`w-10 h-10 rounded-full border-2 bg-white ${theme === 'light' ? 'border-indigo-600' : 'border-gray-200'}`} title="Light Theme"></button>
                  <button onClick={() => setTheme('gradient')} className={`w-10 h-10 rounded-full border-2 bg-gradient-to-br from-indigo-500 to-purple-600 ${theme === 'gradient' ? 'border-indigo-600' : 'border-transparent'}`} title="Gradient Theme"></button>
                </div>
              </div>
            </div>
          </div>

          <div className="p-6 bg-white rounded-2xl shadow-sm border border-gray-100">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Share Your Link</h2>
            <p className="text-sm text-gray-600 mb-4">Add this link to your Instagram, TikTok, or Twitter bio to drive traffic directly to your storefront.</p>
            <div className="flex gap-2">
              <input
                type="text"
                readOnly
                value={profileUrl}
                className="flex-1 bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600 focus:outline-none font-mono"
              />
              <button
                onClick={handleCopy}
                className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
              >
                {copied ? 'Copied!' : 'Copy Link'}
              </button>
            </div>

            <div className="mt-4 pt-4 border-t border-gray-100">
                <p className="text-xs text-gray-500 mb-2 font-semibold uppercase tracking-wider">Share directly to:</p>
                <div className="flex gap-2">
                    <a href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(`Check out my new store! https://${profileUrl}`)}`} target="_blank" rel="noopener noreferrer" className="flex items-center gap-1 bg-black text-white px-3 py-1.5 rounded-md text-xs font-bold hover:bg-gray-800 transition-colors">
                        <svg className="w-3 h-3" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                        X (Twitter)
                    </a>
                </div>
            </div>
          </div>
        </section>

        {/* Mobile Preview */}
        <section className="w-full md:w-1/2 flex justify-center items-start">
          <div className="w-full max-w-[320px] aspect-[9/19] rounded-[2.5rem] border-[8px] border-gray-900 shadow-2xl relative overflow-hidden flex flex-col">
            {/* Notch */}
            <div className="absolute top-0 inset-x-0 h-6 bg-gray-900 rounded-b-3xl w-1/2 mx-auto z-10"></div>

            {/* Preview Content */}
            <div className={`flex-1 w-full flex flex-col p-6 items-center text-center overflow-y-auto ${getThemeClasses()} transition-colors duration-300`}>
                <div className="w-20 h-20 rounded-full bg-white/20 backdrop-blur-md shadow-inner flex items-center justify-center text-3xl mt-8 mb-4 border border-white/30">
                    🛍️
                </div>
                <h1 className="text-xl font-bold font-outfit mb-2">{storeName || 'My Awesome Store'}</h1>
                <p className={`text-sm mb-8 ${theme === 'light' ? 'text-gray-600' : 'text-gray-300'}`}>{bio || 'Premium products for awesome people.'}</p>

                <div className="w-full space-y-3 flex-1">
                    <div className={`w-full p-3 rounded-xl font-semibold text-sm cursor-pointer hover:scale-105 transition-transform ${theme === 'light' ? 'bg-gray-100 hover:bg-gray-200 text-gray-900' : 'bg-white/10 hover:bg-white/20 text-white backdrop-blur-md border border-white/10'}`}>
                        Shop the New Collection
                    </div>
                    <div className={`w-full p-3 rounded-xl font-semibold text-sm cursor-pointer hover:scale-105 transition-transform ${theme === 'light' ? 'bg-gray-100 hover:bg-gray-200 text-gray-900' : 'bg-white/10 hover:bg-white/20 text-white backdrop-blur-md border border-white/10'}`}>
                        Book an Appointment
                    </div>
                    <div className={`w-full p-3 rounded-xl font-semibold text-sm cursor-pointer hover:scale-105 transition-transform ${theme === 'light' ? 'bg-gray-100 hover:bg-gray-200 text-gray-900' : 'bg-white/10 hover:bg-white/20 text-white backdrop-blur-md border border-white/10'}`}>
                        Contact Us
                    </div>
                </div>

                <div className="mt-8 pb-4 opacity-80 flex flex-col items-center">
                    <span className="text-[10px] font-bold uppercase tracking-widest opacity-60 mb-1">Powered by OHC</span>
                </div>
            </div>
          </div>
        </section>
      </main>
      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
