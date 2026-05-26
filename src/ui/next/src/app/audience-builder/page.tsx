"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function AudienceBuilderPage() {
  const router = useRouter();
  const [headline, setHeadline] = useState('Join our VIP List');
  const [discountCode, setDiscountCode] = useState('WELCOME10');
  const [theme, setTheme] = useState('light');
  const [tenant, setTenant] = useState('my-store');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
       const storedTenant = localStorage.getItem('tenant') || 'my-store';
       setTenant(storedTenant);
    }
  }, []);

  const embedUrl = `https://ohc.app/api/v1/growth/audience/embed?tenant=${tenant}&theme=${theme}&headline=${encodeURIComponent(headline)}&discount=${encodeURIComponent(discountCode)}`;

  const embedCode = `<iframe src="${embedUrl}" width="100%" height="450" frameborder="0" style="border-radius: 16px; overflow: hidden; border: none; max-width: 450px;"></iframe>`;

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Audience Builder 🧲</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Settings */}
        <section className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Modal Settings</h2>
                <p className="text-sm text-gray-600 mb-6">Capture leads by offering a small discount in exchange for their email address.</p>

                <div className="flex flex-col gap-5">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Headline</label>
                        <input
                            type="text"
                            value={headline}
                            onChange={(e) => setHeadline(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            placeholder="e.g. Get 10% Off!"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Discount Code to Reveal</label>
                        <input
                            type="text"
                            value={discountCode}
                            onChange={(e) => setDiscountCode(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 font-mono uppercase"
                            placeholder="e.g. WELCOME10"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                        <div className="flex gap-3">
                            <button
                              onClick={() => setTheme('light')}
                              className={`flex-1 py-2 border rounded-lg text-sm font-medium transition-colors ${theme === 'light' ? 'bg-indigo-50 border-indigo-600 text-indigo-700' : 'bg-white border-gray-300 text-gray-700 hover:bg-gray-50'}`}
                            >
                              Light
                            </button>
                            <button
                              onClick={() => setTheme('dark')}
                              className={`flex-1 py-2 border rounded-lg text-sm font-medium transition-colors ${theme === 'dark' ? 'bg-gray-900 border-gray-900 text-white' : 'bg-white border-gray-300 text-gray-700 hover:bg-gray-50'}`}
                            >
                              Dark
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Embed on your site</h2>
                <p className="text-sm text-gray-600 mb-4">Paste this code into your website's HTML, or directly into a popup builder.</p>

                <div className="bg-gray-900 text-gray-300 p-3 rounded-lg font-mono text-xs overflow-x-auto mb-4 whitespace-pre-wrap break-all">
                    {embedCode}
                </div>

                <button
                    onClick={() => {
                        navigator.clipboard.writeText(embedCode);
                        setCopied(true);
                        setTimeout(() => setCopied(false), 2000);
                    }}
                    className={`w-full py-3 rounded-xl text-sm font-bold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                >
                    {copied ? 'Copied to Clipboard!' : 'Copy Embed Code'}
                </button>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-2/3 flex flex-col gap-4">
             <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Live Preview</h2>
             <div className="flex-1 w-full bg-gray-200 rounded-2xl shadow-inner flex items-center justify-center p-8 overflow-hidden relative min-h-[500px]" style={{ backgroundImage: 'url("https://www.transparenttextures.com/patterns/cubes.png")' }}>
                 {/* Decorative background representing a website */}
                 <div className="absolute inset-0 bg-white/40 backdrop-blur-sm"></div>

                 <div className="relative z-10 w-full flex justify-center">
                    <iframe
                        src={`/api/v1/growth/audience/embed?tenant=${tenant}&theme=${theme}&headline=${encodeURIComponent(headline)}&discount=${encodeURIComponent(discountCode)}`}
                        width="100%"
                        height="450"
                        frameBorder="0"
                        style={{ maxWidth: '400px', borderRadius: '16px', boxShadow: '0 25px 50px -12px rgba(0, 0, 0, 0.25)' }}
                    ></iframe>
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