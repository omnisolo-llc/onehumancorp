"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';
import { useRouter } from 'next/navigation';

export default function DigitalBusinessCardGeneratorPage() {
  const router = useRouter();
  const [name, setName] = useState('');
  const [title, setTitle] = useState('');
  const [company, setCompany] = useState('');
  const [phone, setPhone] = useState('');
  const [email, setEmail] = useState('');
  const [website, setWebsite] = useState('');
  const [linkedin, setLinkedin] = useState('');
  const [themeColor, setThemeColor] = useState('#4F46E5');
  const [removeBranding, setRemoveBranding] = useState(false);

  const [shareLink, setShareLink] = useState('');
  const [copied, setCopied] = useState(false);
  const [tenantId, setTenantId] = useState('my-store');
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [hasSharedToUnlock, setHasSharedToUnlock] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const tenant = localStorage.getItem('tenant') || 'my-store';
      setTenantId(tenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');

      const hasShared = localStorage.getItem('ohc_dbc_shared') === 'true';
      setHasSharedToUnlock(hasShared);
      if (hasShared) {
        setRemoveBranding(true);
      }
    }
  }, []);

  const generateLink = () => {
    if (!name || !title) {
      alert('Please fill out at least your name and title.');
      return;
    }

    const data = {
      tenant: tenantId,
      name,
      title,
      company,
      phone,
      email,
      website,
      linkedin,
      themeColor,
      removeBranding: removeBranding && (hasPro || hasSharedToUnlock)
    };

    // Safely encode unicode string to base64url for URLs
    const utf8Encoded = encodeURIComponent(JSON.stringify(data));
    const base64Str = btoa(unescape(utf8Encoded));
    const base64UrlStr = base64Str.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');

    const url = `${window.location.origin}/digital-business-card/view?data=${base64UrlStr}`;
    setShareLink(url);
  };

  const handleCopy = () => {
    if (shareLink) {
      navigator.clipboard.writeText(shareLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.checked) {
      if (!hasPro && !hasSharedToUnlock) {
        setShowSoftPaywall(true);
        setRemoveBranding(false);
      } else {
        setRemoveBranding(true);
      }
    } else {
      setRemoveBranding(false);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7] dark:bg-[#121212]">
      <header className="px-4 md:px-6 py-4 flex items-center justify-between border-b sticky top-0 z-40 bg-white dark:bg-black/65 backdrop-blur-[30px] saturate-[210%] border-gray-200 dark:border-gray-800">
        <h1 className="text-xl md:text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-white tracking-tight">Digital Business Card Generator</h1>
        <Link href="/dashboard" className="px-3 py-1.5 md:px-4 md:py-2 bg-gray-200 dark:bg-gray-800 rounded-md text-xs md:text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-700 transition-colors dark:text-white">
          Back to Dashboard
        </Link>
      </header>

      <main className="p-4 md:p-8 flex-1 w-full max-w-6xl mx-auto flex flex-col lg:flex-row gap-8">

        {/* Form Section */}
        <section className="w-full lg:w-1/2 flex flex-col gap-6">
          <div className="bg-white dark:bg-[#1E1E1E] p-6 md:p-8 rounded-[24px] border border-gray-200 dark:border-gray-800 shadow-sm relative">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 dark:bg-indigo-900/10 rounded-bl-[100px] -z-10 pointer-events-none"></div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Build Your vCard</h2>
            <p className="text-gray-500 text-sm mb-8">
              Create a digital business card that clients can instantly save to their phone contacts. Share it via link or QR code.
            </p>

            <div className="space-y-5">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-5">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Full Name</label>
                  <input
                    type="text"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    placeholder="e.g. Jane Doe"
                    className="w-full bg-gray-50 dark:bg-[#2A2A2A] border border-gray-200 dark:border-gray-700 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:text-white transition-all"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Job Title</label>
                  <input
                    type="text"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                    placeholder="e.g. Founder & CEO"
                    className="w-full bg-gray-50 dark:bg-[#2A2A2A] border border-gray-200 dark:border-gray-700 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:text-white transition-all"
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Company</label>
                <input
                  type="text"
                  value={company}
                  onChange={(e) => setCompany(e.target.value)}
                  placeholder="e.g. Acme Corp"
                  className="w-full bg-gray-50 dark:bg-[#2A2A2A] border border-gray-200 dark:border-gray-700 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:text-white transition-all"
                />
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-5">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Phone Number</label>
                  <input
                    type="tel"
                    value={phone}
                    onChange={(e) => setPhone(e.target.value)}
                    placeholder="e.g. +1 (555) 123-4567"
                    className="w-full bg-gray-50 dark:bg-[#2A2A2A] border border-gray-200 dark:border-gray-700 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:text-white transition-all"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Email Address</label>
                  <input
                    type="email"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                    placeholder="e.g. jane@example.com"
                    className="w-full bg-gray-50 dark:bg-[#2A2A2A] border border-gray-200 dark:border-gray-700 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:text-white transition-all"
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">Website</label>
                <input
                  type="url"
                  value={website}
                  onChange={(e) => setWebsite(e.target.value)}
                  placeholder="e.g. example.com"
                  className="w-full bg-gray-50 dark:bg-[#2A2A2A] border border-gray-200 dark:border-gray-700 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:text-white transition-all"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">LinkedIn URL</label>
                <input
                  type="url"
                  value={linkedin}
                  onChange={(e) => setLinkedin(e.target.value)}
                  placeholder="e.g. linkedin.com/in/janedoe"
                  className="w-full bg-gray-50 dark:bg-[#2A2A2A] border border-gray-200 dark:border-gray-700 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 dark:text-white transition-all"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Theme Color</label>
                <div className="flex gap-3">
                  {['#4F46E5', '#000000', '#E11D48', '#059669', '#D97706'].map((color) => (
                    <button
                      key={color}
                      onClick={() => setThemeColor(color)}
                      className={`w-8 h-8 rounded-full transition-all ${themeColor === color ? 'ring-2 ring-offset-2 ring-indigo-500 dark:ring-offset-[#1E1E1E] scale-110' : 'hover:scale-105'}`}
                      style={{ backgroundColor: color }}
                      aria-label={`Select color ${color}`}
                    />
                  ))}
                </div>
              </div>

              <div className="pt-4 border-t border-gray-100 dark:border-gray-800">
                <label className="flex items-start gap-3 cursor-pointer group">
                  <input
                    type="checkbox"
                    checked={removeBranding}
                    onChange={handleBrandingToggle}
                    className="mt-1 w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500"
                  />
                  <div>
                    <span className="text-sm font-medium text-gray-900 dark:text-gray-200">Remove "Powered by OHC" branding</span>
                    <p className="text-xs text-gray-500 mt-1">Make the card 100% white-labeled. Requires Pro plan.</p>
                  </div>
                </label>
              </div>

              <button
                onClick={generateLink}
                className="w-full mt-2 py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all active:scale-[0.98] text-sm flex items-center justify-center gap-2"
              >
                Generate Shareable Link
              </button>
            </div>
          </div>
        </section>

        {/* Live Preview Section */}
        <section className="w-full lg:w-1/2 flex flex-col gap-6 items-center">
          <div className="w-full max-w-sm mx-auto">
            <h2 className="text-lg font-semibold font-outfit text-gray-900 dark:text-white mb-4 text-center">Mobile Preview</h2>

            {/* Phone Frame */}
            <div className="relative mx-auto border-gray-800 dark:border-gray-700 bg-gray-800 border-[10px] rounded-[2.5rem] h-[600px] w-[300px] shadow-2xl overflow-hidden">
              <div className="absolute top-0 inset-x-0 h-6 bg-gray-800 z-20 rounded-b-3xl w-40 mx-auto"></div>

              <div className="absolute inset-0 bg-gray-50 dark:bg-black overflow-y-auto overflow-x-hidden p-6 flex flex-col items-center custom-scrollbar">

                {/* Profile Avatar */}
                <div
                  className="w-24 h-24 rounded-full flex items-center justify-center text-4xl font-bold text-white shadow-lg mb-4 mt-6"
                  style={{ backgroundColor: themeColor }}
                >
                  {name ? name.charAt(0).toUpperCase() : 'J'}
                </div>

                <h1 className="text-2xl font-bold text-gray-900 dark:text-white text-center mb-1 font-outfit">{name || 'Jane Doe'}</h1>
                <p className="text-sm font-medium mb-1" style={{ color: themeColor }}>{title || 'Job Title'}</p>
                <p className="text-sm text-gray-500 dark:text-gray-400 mb-6 text-center">{company || 'Company Name'}</p>

                {/* Actions */}
                <div className="flex gap-3 w-full mb-8">
                  <button
                    className="flex-1 py-2.5 rounded-full text-white font-semibold text-sm shadow-md flex items-center justify-center gap-1"
                    style={{ backgroundColor: themeColor }}
                  >
                    Save vCard
                  </button>
                  <button
                    className="w-10 h-10 rounded-full flex items-center justify-center shadow-sm border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 text-gray-600 dark:text-gray-300"
                  >
                    🔗
                  </button>
                </div>

                {/* Contact Info */}
                <div className="w-full space-y-4">
                  {(phone || !name) && (
                    <div className="bg-white dark:bg-gray-900 p-4 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-800 flex items-center gap-4">
                      <div className="w-10 h-10 rounded-full bg-gray-50 dark:bg-gray-800 flex items-center justify-center flex-shrink-0" style={{ color: themeColor }}>
                        📱
                      </div>
                      <div className="overflow-hidden">
                        <p className="text-xs text-gray-500 dark:text-gray-400 font-medium uppercase tracking-wider mb-0.5">Mobile</p>
                        <p className="text-sm font-semibold text-gray-900 dark:text-white truncate">{phone || '+1 (555) 123-4567'}</p>
                      </div>
                    </div>
                  )}

                  {(email || !name) && (
                    <div className="bg-white dark:bg-gray-900 p-4 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-800 flex items-center gap-4">
                      <div className="w-10 h-10 rounded-full bg-gray-50 dark:bg-gray-800 flex items-center justify-center flex-shrink-0" style={{ color: themeColor }}>
                        ✉️
                      </div>
                      <div className="overflow-hidden">
                        <p className="text-xs text-gray-500 dark:text-gray-400 font-medium uppercase tracking-wider mb-0.5">Email</p>
                        <p className="text-sm font-semibold text-gray-900 dark:text-white truncate">{email || 'jane@example.com'}</p>
                      </div>
                    </div>
                  )}

                  {(website || !name) && (
                    <div className="bg-white dark:bg-gray-900 p-4 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-800 flex items-center gap-4">
                      <div className="w-10 h-10 rounded-full bg-gray-50 dark:bg-gray-800 flex items-center justify-center flex-shrink-0" style={{ color: themeColor }}>
                        🌐
                      </div>
                      <div className="overflow-hidden">
                        <p className="text-xs text-gray-500 dark:text-gray-400 font-medium uppercase tracking-wider mb-0.5">Website</p>
                        <p className="text-sm font-semibold text-gray-900 dark:text-white truncate">{website || 'example.com'}</p>
                      </div>
                    </div>
                  )}
                </div>

                {!removeBranding && (
                  <div className="mt-8 mb-4 text-center">
                    <span className="text-xs font-medium text-gray-400 dark:text-gray-500 hover:text-gray-600 dark:hover:text-gray-300 transition-colors cursor-pointer">
                      ⚡ Powered by OHC
                    </span>
                  </div>
                )}

              </div>
            </div>

            {/* Share Link Result */}
            {shareLink && (
              <div className="mt-8 p-4 bg-indigo-50 dark:bg-indigo-900/20 rounded-2xl border border-indigo-100 dark:border-indigo-800/50 animate-fade-in w-full">
                <p className="text-sm font-bold text-indigo-900 dark:text-indigo-300 mb-2">Your link is ready!</p>
                <div className="flex gap-2">
                  <input
                    type="text"
                    readOnly
                    value={shareLink}
                    className="flex-1 bg-white dark:bg-black border border-indigo-200 dark:border-indigo-800/50 rounded-xl px-3 py-2 text-xs text-gray-600 dark:text-gray-300 focus:outline-none"
                  />
                  <button
                    onClick={handleCopy}
                    className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white font-semibold rounded-xl transition-colors text-xs whitespace-nowrap shadow-sm"
                  >
                    {copied ? 'Copied!' : 'Copy'}
                  </button>
                </div>
                <div className="mt-3 flex justify-center">
                  <Link
                    href={shareLink}
                    target="_blank"
                    className="text-xs font-semibold text-indigo-600 dark:text-indigo-400 hover:underline flex items-center gap-1"
                  >
                    Open in new tab <span aria-hidden="true">&rarr;</span>
                  </Link>
                </div>
              </div>
            )}
          </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[9999] flex items-center justify-center p-4 backdrop-blur-[30px] saturate-[210%] animate-in fade-in duration-200">
          <div className="bg-white dark:bg-[#1E1E1E] w-full max-w-md rounded-3xl p-8 shadow-2xl relative overflow-hidden font-inter text-center border border-gray-100 dark:border-gray-800">
            <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-br from-indigo-100 to-purple-50 dark:from-indigo-900/30 dark:to-purple-900/10 rounded-bl-[100px] -z-10 opacity-60"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-indigo-50 dark:bg-indigo-900/30 rounded-2xl flex items-center justify-center mx-auto mb-6 border border-indigo-100 dark:border-indigo-800/50">
              <span className="text-3xl text-indigo-600 dark:text-indigo-400">🚀</span>
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 dark:text-gray-400 mb-8 text-sm leading-relaxed">
              Make the Digital Business Card 100% white-labeled. Upgrade to Pro to remove the "Powered by OHC" watermark and unlock full customization.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-3 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600 hover:bg-indigo-700 flex justify-center items-center gap-2"
            >
              Upgrade to Pro
            </button>

            <button
              onClick={() => {
                const shareText = "I just created my free Digital Business Card using OHC! Build yours here:";
                const shareUrl = "https://ohc.app";
                const intentUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}&url=${encodeURIComponent(shareUrl)}`;
                window.open(intentUrl, '_blank');

                // Set unlock state
                localStorage.setItem('ohc_dbc_shared', 'true');
                setHasSharedToUnlock(true);
                setRemoveBranding(true);
                setShowSoftPaywall(false);
              }}
              className="w-full py-4 rounded-xl font-bold text-indigo-600 bg-indigo-50 hover:bg-indigo-100 mb-3 transition-all flex justify-center items-center gap-2"
            >
              <span className="text-xl">🐦</span> Share to Unlock for Free
            </button>


            <button
              onClick={() => setShowSoftPaywall(false)}
              className="w-full py-3.5 px-4 bg-white dark:bg-transparent border border-gray-200 dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-xl font-medium transition-colors"
            >
              Keep Watermark
            </button>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fadeIn 0.3s ease-out forwards; }
        .custom-scrollbar::-webkit-scrollbar { width: 4px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(156, 163, 175, 0.3); border-radius: 4px; }
      `}} />
    </div>
  );
}
