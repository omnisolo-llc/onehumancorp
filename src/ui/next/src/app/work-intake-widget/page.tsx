"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function WorkIntakeWidgetPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-business');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [title, setTitle] = useState('Work Request');
  const [copied, setCopied] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [isUnlocking, setIsUnlocking] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-business';
      setTenant(storedTenant);
    }
    document.title = "Embed Work Intake | OHC";
  }, []);

  const embedUrl = `https://ohc.app/api/v1/growth/work-intake/embed?tenant=${tenant}&theme=${theme}&title=${encodeURIComponent(title)}`;
  const embedCode = `<iframe src="${embedUrl}" width="320" height="400" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleShareToUnlock = async () => {
    setIsUnlocking(true);
    try {
      let referralLink = `${window.location.origin}/onboarding?ref=${tenant}`;
      try {
        const res = await fetch("/api/v1/growth/referrals/generate", { method: "POST" });
        if (res.ok) {
          const data = await res.json();
          if (data.referral_link) {
            referralLink = data.referral_link;
          }
        }
      } catch (err) {
        console.warn("Failed to generate referral link, using fallback", err);
      }

      const text = `I just built a custom Work Intake widget for my business on OneHumanCorp! 🚀\n\nStart your own business and get $50 off: ${referralLink}`;
      window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}`, "_blank");

      // Simulate a verification delay
      setTimeout(() => {
        setRemoveBranding(true);
        setShowSoftPaywall(false);
        setIsUnlocking(false);
      }, 1500);
    } catch (e) {
      console.error(e);
      setIsUnlocking(false);
    }
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return { background: '#1D1D1F', color: '#ffffff', borderColor: '#333333' };
    }
    return { background: '#ffffff', color: '#111827', borderColor: '#e5e7eb' };
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50/50 via-white/50 to-blue-50/50">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-3">
             <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Work-Intake Widget 📋</h1>
             <span className="bg-blue-100 text-blue-800 text-xs font-semibold px-2 py-1 rounded">Lead Capture Loop</span>
         </div>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Editor Sidebar */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 app-card shadow-lg">
                <h2 className="text-lg font-semibold font-outfit mb-4">Widget Settings</h2>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex bg-gray-100 p-1 rounded-lg">
                        <button
                            aria-pressed={theme === 'light'}
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'light' ? 'bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            aria-pressed={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'dark' ? 'bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Tenant ID</label>
                    <input
                        type="text"
                        value={tenant}
                        onChange={(e) => setTenant(e.target.value)}
                        className="w-full px-3 py-2 bg-white dark:bg-black/20 border border-gray-200 dark:border-gray-700 rounded-[8px] min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] backdrop-blur-[30px] saturate-[210%] transition-all"
                        placeholder="e.g. my-business"
                    />
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Form Title</label>
                    <input
                        type="text"
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        className="w-full px-3 py-2 bg-white dark:bg-black/20 border border-gray-200 dark:border-gray-700 rounded-[8px] min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF] backdrop-blur-[30px] saturate-[210%] transition-all"
                        placeholder="e.g. Work Request"
                    />
                </div>

                <div className="mb-6">
                    <label className="flex items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={(e) => {
                              if (e.target.checked) {
                                  setShowSoftPaywall(true);
                                  setRemoveBranding(false);
                              } else {
                                  setRemoveBranding(false);
                              }
                          }}
                            className="w-4 h-4 text-[#0071E3] rounded focus:ring-[#0066FF]"
                        />
                        <span className="text-sm text-gray-700">Remove "Powered by OHC" branding</span>
                    </label>
                    <p className="text-xs text-gray-500 mt-1 ml-6">Requires Pro plan or higher.</p>
                </div>
            </div>

            <div className="p-6 app-card shadow-lg flex flex-col justify-center gap-4">
               <h3 className="font-semibold text-gray-900">Embed on Your Website</h3>
               <p className="text-sm text-gray-600">Copy this code snippet to add the widget directly to your own site, Notion document, or blog.</p>
               <button
                  onClick={() => setShowModal(true)}
                  className="w-full py-3 bg-[#0071E3] hover:bg-blue-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm"
               >
                  Get Widget Code
               </button>
            </div>
        </div>

        {/* Live Preview */}
        <section className="w-full md:w-2/3 flex flex-col gap-4 items-center">
             <h2 className="text-xl font-semibold font-outfit self-start" style={{ color: '#1D1D1F' }}>Live Preview</h2>

             {/* Realistic environment wrapper */}
             <div className="w-full max-w-2xl app-card min-h-[44px] min-w-[44px] overflow-hidden shadow-2xl relative mt-4">
                 <div className="bg-gray-100 border-b border-gray-300 px-4 py-3 flex items-center gap-2">
                     <div className="flex gap-1.5">
                         <div className="w-3 h-3 rounded-full bg-red-400"></div>
                         <div className="w-3 h-3 rounded-full bg-yellow-400"></div>
                         <div className="w-3 h-3 rounded-full bg-green-400"></div>
                     </div>
                     <div className="ml-4 bg-white px-3 py-1 rounded border border-gray-200 text-xs text-gray-500 flex-1 text-center font-mono">
                         yourwebsite.com
                     </div>
                 </div>

                 <div className="p-8 min-h-[500px] flex flex-col lg:flex-row items-center justify-center gap-12" style={{ backgroundImage: 'radial-gradient(#e5e7eb 1px, transparent 1px)', backgroundSize: '20px 20px' }}>

                    <div className="text-left flex-1 max-w-sm hidden lg:block">
                        <h3 className="text-3xl font-bold mb-4 text-gray-800">Ready to start?</h3>
                        <p className="text-gray-600 mb-6">Drop your information in the form and we'll get right back to you. This form connects directly to your OHC workspace.</p>
                        <div className="h-4 w-32 bg-gray-200 rounded mb-2"></div>
                        <div className="h-4 w-48 bg-gray-200 rounded"></div>
                    </div>

                    {/* The Actual Widget Iframe Preview */}
                    <div className="flex-shrink-0" style={{ filter: 'drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))' }}>
                        <iframe
                            src={`/api/v1/growth/work-intake/embed?tenant=${tenant}&theme=${theme}&title=${encodeURIComponent(title)}`}
                            width="320"
                            height="400"
                            frameBorder="0"
                            scrolling="no"
                            style={{ border: 'none', borderRadius: '16px', backgroundColor: 'transparent' }}
                        />
                        {!removeBranding && (
                            <div style={{ fontFamily: 'sans-serif', textAlign: 'center', fontSize: '12px', marginTop: '8px' }}>
                                <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noreferrer" style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }}>⚡ Powered by OHC</a>
                            </div>
                        )}
                    </div>

                 </div>
             </div>
        </section>
      </main>
      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[9999] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-indigo-100 flex items-center justify-center text-3xl shadow-inner text-indigo-600 mx-auto mb-6">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Make the Work Intake Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 min-h-[44px] min-w-[44px] font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600 hover:bg-indigo-700"
            >
              Upgrade to Pro
            </button>
            <p className="text-sm font-medium text-gray-500 mb-3">or</p>
            <button
              onClick={handleShareToUnlock}
              disabled={isUnlocking}
              className="w-full py-4 min-h-[44px] min-w-[44px] font-bold text-gray-700 bg-gray-100 hover:bg-gray-200 transition-all flex items-center justify-center gap-2"
            >
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              {isUnlocking ? 'Verifying Share...' : 'Share on X to Unlock'}
            </button>
          </div>
        </div>
      )}


      {/* Embed Code Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4">
            <div className="absolute inset-0 bg-black/40 backdrop-blur-[30px] saturate-[210%]" onClick={() => setShowModal(false)}></div>
            <div className="app-card shadow-2xl p-8 max-w-xl w-full relative z-10 animate-fade-in-up">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Work-Intake Widget</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website to capture leads instantly.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 min-h-[44px] min-w-[44px] font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] transition-all"
                    />
                    <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                         <button
                            onClick={handleCopy}
                            className="p-2 bg-white rounded-lg border shadow-sm text-gray-600 hover:text-[#0071E3] transition-colors"
                            title="Copy to clipboard"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2 2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                        </button>
                    </div>
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-[#0071E3] hover:bg-blue-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm flex items-center justify-center gap-2"
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

        @keyframes fade-in-up {
          0% { opacity: 0; transform: translateY(20px); }
          100% { opacity: 1; transform: translateY(0); }
        }
        .animate-fade-in-up { animation: fade-in-up 0.2s ease-out forwards; }
      `}} />
    </div>
  );
}
