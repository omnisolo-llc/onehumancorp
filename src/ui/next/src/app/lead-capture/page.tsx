'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function LeadCapturePage() {
  const router = useRouter();
  const [headline, setHeadline] = useState('Wait! Before you go...');
  const [subheadline, setSubheadline] = useState('Get 15% off your first order.');
  const [discountCode, setDiscountCode] = useState('WELCOME15');
  const [buttonText, setButtonText] = useState('Claim My Offer');
  const [themeColor, setThemeColor] = useState('#4F46E5'); // Default indigo-600
  const [copied, setCopied] = useState(false);

  const embedCode = `<script>
  window.OHC_LEAD_CAPTURE = {
    headline: "${headline}",
    subheadline: "${subheadline}",
    buttonText: "${buttonText}",
    themeColor: "${themeColor}",
    tenant: "${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}"
  };
</script>
<script src="https://ohc.app/api/v1/growth/lead-capture/widget.js" async></script>`;

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Lead Capture Builder 🧲</h1>
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
        {/* Editor Settings */}
        <section className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Popup Settings</h2>
                <div className="flex flex-col gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Headline</label>
                        <input
                            type="text"
                            value={headline}
                            onChange={(e) => setHeadline(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Subheadline</label>
                        <textarea
                            rows={2}
                            value={subheadline}
                            onChange={(e) => setSubheadline(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-none"
                        />
                    </div>
                     <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Button Text</label>
                        <input
                            type="text"
                            value={buttonText}
                            onChange={(e) => setButtonText(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Theme Color</label>
                        <div className="flex gap-2">
                            <button onClick={() => setThemeColor('#4F46E5')} className={`w-8 h-8 rounded-full border-2 ${themeColor === '#4F46E5' ? 'border-gray-900' : 'border-transparent'}`} style={{ background: '#4F46E5' }}></button>
                            <button onClick={() => setThemeColor('#10B981')} className={`w-8 h-8 rounded-full border-2 ${themeColor === '#10B981' ? 'border-gray-900' : 'border-transparent'}`} style={{ background: '#10B981' }}></button>
                            <button onClick={() => setThemeColor('#F59E0B')} className={`w-8 h-8 rounded-full border-2 ${themeColor === '#F59E0B' ? 'border-gray-900' : 'border-transparent'}`} style={{ background: '#F59E0B' }}></button>
                            <button onClick={() => setThemeColor('#EF4444')} className={`w-8 h-8 rounded-full border-2 ${themeColor === '#EF4444' ? 'border-gray-900' : 'border-transparent'}`} style={{ background: '#EF4444' }}></button>
                            <button onClick={() => setThemeColor('#111827')} className={`w-8 h-8 rounded-full border-2 ${themeColor === '#111827' ? 'border-gray-500' : 'border-transparent'}`} style={{ background: '#111827' }}></button>
                        </div>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Embed on Your Site</h2>
                <p className="text-sm text-gray-600 mb-4">Copy this snippet and paste it just before the closing <code>&lt;/body&gt;</code> tag of your website.</p>
                <div className="bg-gray-900 text-gray-300 p-3 rounded-lg font-mono text-xs overflow-x-auto mb-4 whitespace-pre">
                    {embedCode}
                </div>
                <button
                    onClick={() => {
                        navigator.clipboard.writeText(embedCode);
                        setCopied(true);
                        setTimeout(() => setCopied(false), 2000);
                    }}
                    className={`w-full py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-200 text-gray-800 hover:bg-gray-300'}`}
                >
                    {copied ? 'Copied Code!' : 'Copy Embed Code'}
                </button>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-2/3 flex flex-col gap-4">
             <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Live Preview</h2>
             <div className="w-full h-[600px] bg-gray-200/50 rounded-2xl shadow-inner border border-gray-300 relative flex items-center justify-center overflow-hidden" style={{ backgroundImage: 'url("data:image/svg+xml,%3Csvg width=\\"20\\" height=\\"20\\" viewBox=\\"0 0 20 20\\" xmlns=\\"http://www.w3.org/2000/svg\\"%3E%3Cg fill=\\"%239C92AC\\" fill-opacity=\\"0.1\\" fill-rule=\\"evenodd\\"%3E%3Ccircle cx=\\"3\\" cy=\\"3\\" r=\\"3\\"/>%3Ccircle cx=\\"13\\" cy=\\"13\\" r=\\"3\\"/>%3C/g%3E%3C/svg%3E")' }}>

                {/* Popup Overlay */}
                <div className="absolute inset-0 bg-black/40 flex items-center justify-center p-4 z-10">
                    <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden animate-in fade-in zoom-in duration-300">
                        {/* Decorative Shape */}
                        <div className="absolute top-0 right-0 w-32 h-32 opacity-10 rounded-bl-full -z-0" style={{ backgroundColor: themeColor }}></div>

                        <button className="absolute top-4 right-4 text-gray-400 hover:text-gray-600 transition-colors">
                            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" /></svg>
                        </button>

                        <div className="text-center relative z-10 mb-6">
                            <h3 className="text-2xl sm:text-3xl font-bold font-outfit text-gray-900 mb-2 leading-tight">
                                {headline || 'Headline'}
                            </h3>
                            <p className="text-gray-600">
                                {subheadline || 'Subheadline'}
                            </p>
                        </div>

                        <div className="flex flex-col gap-3 relative z-10">
                            <input
                                type="email"
                                placeholder="Enter your email address"
                                className="w-full px-4 py-3 border border-gray-300 rounded-xl focus:outline-none focus:ring-2"
                                style={{ '--tw-ring-color': themeColor } as React.CSSProperties}
                            />
                            <button
                                className="w-full px-4 py-3 text-white font-bold rounded-xl shadow-md transition-transform hover:-translate-y-0.5 active:translate-y-0"
                                style={{ backgroundColor: themeColor }}
                            >
                                {buttonText || 'Button Text'}
                            </button>
                        </div>

                        <div className="mt-6 text-center text-xs text-gray-400 relative z-10">
                            No thanks, I prefer paying full price.
                        </div>

                        {/* Viral Loop Badge */}
                        <div className="absolute bottom-2 left-0 right-0 flex justify-center opacity-70 hover:opacity-100 transition-opacity">
                            <a href={`https://ohc.store/join?ref=${typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store'}`} target="_blank" rel="noopener noreferrer" className="flex items-center gap-1 text-[10px] font-semibold text-gray-500 uppercase tracking-wider bg-gray-50 px-2 py-1 rounded-full border border-gray-100">
                                ⚡ Powered by OHC
                            </a>
                        </div>
                    </div>
                </div>

                <div className="absolute bottom-4 left-4 right-4 text-center text-sm text-gray-500 bg-white/80 backdrop-blur px-4 py-2 rounded-full shadow-sm">
                    This is how the popup will appear to visitors on your site.
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
