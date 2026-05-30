'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

import { useEffect } from 'react';

export default function ViralBadgePage() {
  const router = useRouter();
  const [theme, setTheme] = useState<'light' | 'dark' | 'glass'>('glass');
  const [position, setPosition] = useState<'bottom-right' | 'bottom-left'>('bottom-right');
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState('my-store');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setTenant(localStorage.getItem('tenant') || 'my-store');
    }
  }, []);

  const referralLink = `https://ohc.store/join?ref=${tenant}`;

  const getBadgeStyles = () => {
    const base = "inline-flex items-center gap-2 px-3 py-2 rounded-full font-inter text-xs font-semibold shadow-lg transition-transform hover:-translate-y-1 cursor-pointer border";
    if (theme === 'light') {
      return `${base} bg-white text-gray-900 border-gray-200`;
    }
    if (theme === 'dark') {
      return `${base} bg-[#1D1D1F] text-white border-gray-800`;
    }
    // Glass
    return `${base} bg-white/20 text-white border-white/30 backdrop-blur-[20px] saturate-200 shadow-[0_8px_32px_rgba(0,0,0,0.1)]`;
  };

  const getPositionClass = () => {
    return position === 'bottom-right' ? 'bottom-6 right-6' : 'bottom-6 left-6';
  };

  const embedCode = `<!-- OHC Viral Badge -->
<a href="${referralLink}" target="_blank" rel="noopener noreferrer" style="text-decoration: none; position: fixed; ${position === 'bottom-right' ? 'bottom: 24px; right: 24px;' : 'bottom: 24px; left: 24px;'} z-index: 9999;">
  <div style="${theme === 'glass' ? 'background: rgba(255, 255, 255, 0.2); backdrop-filter: blur(20px) saturate(200%); border: 1px solid rgba(255, 255, 255, 0.3); color: white; box-shadow: 0 8px 32px rgba(0,0,0,0.1);' : theme === 'dark' ? 'background: #1D1D1F; border: 1px solid #333; color: white; box-shadow: 0 4px 12px rgba(0,0,0,0.1);' : 'background: white; border: 1px solid #eaeaea; color: #1D1D1F; box-shadow: 0 4px 12px rgba(0,0,0,0.1);'} display: inline-flex; align-items: center; gap: 8px; padding: 8px 12px; border-radius: 9999px; font-family: system-ui, -apple-system, sans-serif; font-size: 12px; font-weight: 600; transition: transform 0.2s;">
    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>
    Powered by OHC
  </div>
</a>
<!-- End OHC Viral Badge -->`;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)' }}>
        <h1 className="text-xl md:text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Viral Badge 🚀</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-3 py-1.5 md:px-4 md:py-2 bg-gray-200 rounded-md text-xs md:text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-4 md:p-8 flex-1 w-full max-w-5xl mx-auto flex flex-col md:flex-row gap-6 md:gap-8 items-start">
        {/* Editor Settings */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-bold font-outfit mb-2 text-[#1D1D1F]">Customize Your Badge</h2>
                <p className="text-sm text-gray-600 mb-6">
                   Embed this badge on your external website or blog. Every click uses your referral link, earning you <strong className="text-gray-900">$50 credit</strong> when someone launches their store!
                </p>

                <div className="flex flex-col gap-5">
                    <div>
                        <label className="block text-sm font-semibold text-gray-800 mb-2">Theme</label>
                        <div className="grid grid-cols-3 gap-3">
                            <button
                                onClick={() => setTheme('glass')}
                                className={`py-2 px-3 rounded-lg border-2 text-sm font-medium transition-all ${theme === 'glass' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'}`}
                            >
                                Glass
                            </button>
                            <button
                                onClick={() => setTheme('dark')}
                                className={`py-2 px-3 rounded-lg border-2 text-sm font-medium transition-all ${theme === 'dark' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'}`}
                            >
                                Dark
                            </button>
                            <button
                                onClick={() => setTheme('light')}
                                className={`py-2 px-3 rounded-lg border-2 text-sm font-medium transition-all ${theme === 'light' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'}`}
                            >
                                Light
                            </button>
                        </div>
                    </div>

                    <div>
                        <label className="block text-sm font-semibold text-gray-800 mb-2">Position on Screen</label>
                        <div className="grid grid-cols-2 gap-3">
                            <button
                                onClick={() => setPosition('bottom-left')}
                                className={`py-2 px-3 rounded-lg border-2 text-sm font-medium transition-all ${position === 'bottom-left' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'}`}
                            >
                                Bottom Left
                            </button>
                            <button
                                onClick={() => setPosition('bottom-right')}
                                className={`py-2 px-3 rounded-lg border-2 text-sm font-medium transition-all ${position === 'bottom-right' ? 'border-indigo-600 bg-indigo-50 text-indigo-700' : 'border-gray-200 bg-white text-gray-600 hover:bg-gray-50'}`}
                            >
                                Bottom Right
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-bold font-outfit mb-4 text-[#1D1D1F]">Embed Code</h2>
                <div className="bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4 border border-gray-800">
                    <pre className="whitespace-pre-wrap">{embedCode}</pre>
                </div>
                <button
                    onClick={() => {
                        navigator.clipboard.writeText(embedCode);
                        setCopied(true);
                        setTimeout(() => setCopied(false), 2000);
                    }}
                    className={`w-full py-3 rounded-xl text-sm font-bold transition-all shadow-sm ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                >
                    {copied ? 'Copied to Clipboard!' : 'Copy Embed HTML'}
                </button>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex flex-col gap-4">
             <h2 className="text-xl font-bold font-outfit text-gray-900 px-2">Live Preview</h2>

             {/* Simulated Website Background */}
             <div className="w-full h-[500px] rounded-2xl shadow-xl border border-gray-200 relative overflow-hidden flex flex-col bg-cover bg-center" style={{ backgroundImage: 'url("https://images.unsplash.com/photo-1557683316-973673baf926?q=80&w=1400&auto=format&fit=crop")' }}>

                 {/* Fake website content */}
                 <div className="flex-1 p-8 bg-black/40">
                     <div className="w-32 h-6 bg-white/20 rounded mb-8"></div>
                     <h1 className="text-4xl font-bold text-white mb-4">Your Beautiful Website</h1>
                     <p className="text-white/80 max-w-sm">
                         This is a preview of how the badge will appear floating on top of your content.
                     </p>
                 </div>

                 {/* The Badge */}
                 <div className={`absolute ${getPositionClass()}`}>
                     <div className={getBadgeStyles()}>
                         <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" viewBox="0 0 24 24"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"></polygon></svg>
                         Powered by OHC
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