"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ShareCardsPage() {
  const router = useRouter();
  const [storeName, setStoreName] = useState('My Awesome Store');
  const [tagline, setTagline] = useState('Discover the best products online.');
  const [theme, setTheme] = useState('gradient');
  const [copied, setCopied] = useState(false);
  const [shareLink, setShareLink] = useState('');

  useEffect(() => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
    setShareLink(`https://ohc.store/join?ref=${tenant}`);
  }, []);

  const getThemeStyles = () => {
    switch (theme) {
      case 'gradient':
        return { background: 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)', color: '#fff' };
      case 'dark':
        return { background: '#1D1D1F', color: '#F5F5F7' };
      case 'light':
        return { background: '#ffffff', color: '#1D1D1F', border: '1px solid #e5e7eb' };
      default:
        return { background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', color: '#fff' };
    }
  };

  const shareText = `Check out my storefront: ${storeName} - ${tagline} ${shareLink}`;

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Social Share Cards 🎴</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Editor Settings */}
        <section className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Card Settings</h2>
                <div className="flex flex-col gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Store Name</label>
                        <input
                            type="text"
                            value={storeName}
                            onChange={(e) => setStoreName(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Tagline</label>
                        <textarea
                            rows={2}
                            value={tagline}
                            onChange={(e) => setTagline(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-none"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                        <div className="flex gap-2">
                            <button onClick={() => setTheme('gradient')} className={`w-8 h-8 rounded-full border-2 ${theme === 'gradient' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)' }}></button>
                            <button onClick={() => setTheme('dark')} className={`w-8 h-8 rounded-full border-2 ${theme === 'dark' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: '#1D1D1F' }}></button>
                            <button onClick={() => setTheme('light')} className={`w-8 h-8 rounded-full border-2 ${theme === 'light' ? 'border-indigo-600' : 'border-gray-200'}`} style={{ background: '#ffffff' }}></button>
                            <button onClick={() => setTheme('purple')} className={`w-8 h-8 rounded-full border-2 ${theme === 'purple' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)' }}></button>
                        </div>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Share</h2>
                <div className="flex flex-col gap-3">
                    <button
                        onClick={() => {
                            navigator.clipboard.writeText(shareText);
                            setCopied(true);
                            setTimeout(() => setCopied(false), 2000);
                        }}
                        className={`w-full py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-200 text-gray-800 hover:bg-gray-300'}`}
                    >
                        {copied ? 'Copied Link!' : 'Copy Link'}
                    </button>
                    <a
                        href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="w-full flex items-center justify-center gap-2 bg-black text-white py-2 rounded-lg font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all"
                    >
                        <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                        Share to X
                    </a>
                    <a
                        href={`https://wa.me/?text=${encodeURIComponent(shareText)}`}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="w-full flex items-center justify-center gap-2 bg-[#25D366] text-white py-2 rounded-lg font-semibold text-sm shadow-sm hover:bg-[#20bd5a] transition-all"
                    >
                        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                        Share to WhatsApp
                    </a>
                </div>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-2/3 flex flex-col gap-4">
             <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Live Preview</h2>
             <div className="w-full aspect-[1.91/1] rounded-2xl shadow-xl flex flex-col justify-center items-center text-center p-12 overflow-hidden relative transition-all duration-300" style={getThemeStyles()}>
                 {/* Decorative elements */}
                 {theme !== 'light' && (
                     <>
                        <div className="absolute top-0 left-0 w-64 h-64 bg-white/10 rounded-full blur-3xl -translate-x-1/2 -translate-y-1/2"></div>
                        <div className="absolute bottom-0 right-0 w-64 h-64 bg-black/10 rounded-full blur-3xl translate-x-1/2 translate-y-1/2"></div>
                     </>
                 )}
                 {theme === 'light' && (
                      <>
                        <div className="absolute top-0 left-0 w-64 h-64 bg-indigo-50 rounded-full blur-3xl -translate-x-1/2 -translate-y-1/2"></div>
                        <div className="absolute bottom-0 right-0 w-64 h-64 bg-purple-50 rounded-full blur-3xl translate-x-1/2 translate-y-1/2"></div>
                     </>
                 )}

                 <div className="z-10 flex flex-col items-center">
                    <div className="w-20 h-20 mb-6 bg-white/20 rounded-2xl shadow-inner flex items-center justify-center backdrop-blur-md border border-white/30">
                        <span className="text-4xl">🛍️</span>
                    </div>
                    <h1 className="text-4xl sm:text-5xl md:text-6xl font-bold font-outfit mb-4 leading-tight tracking-tight drop-shadow-sm">
                        {storeName || 'Store Name'}
                    </h1>
                    <p className="text-lg sm:text-xl md:text-2xl font-medium opacity-90 max-w-lg leading-relaxed drop-shadow-sm">
                        {tagline || 'Your tagline goes here...'}
                    </p>
                 </div>

                 <div className="absolute bottom-6 left-6 right-6 flex justify-between items-center opacity-80">
                     <span className="text-sm font-semibold tracking-wider uppercase">⚡ Powered by OHC</span>
                     <span className="text-sm font-medium">{shareLink.replace('https://', '')}</span>
                 </div>
             </div>
             <p className="text-sm text-gray-500 text-center mt-2">
                 This is how your card will appear when shared on social platforms like Twitter, Facebook, and LinkedIn.
             </p>
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
