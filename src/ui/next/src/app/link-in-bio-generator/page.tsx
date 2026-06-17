"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function LinkInBioGeneratorPage() {
  const router = useRouter();

  const [storeName, setStoreName] = useState('My Store');
  const [bio, setBio] = useState('Welcome to my storefront!');
  const [links, setLinks] = useState([
    { id: '1', title: 'Visit My Store', url: '/website-builder' },
    { id: '2', title: 'Book an Appointment', url: '/booking' },
  ]);
  const [theme, setTheme] = useState('gradient');
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState('my-store');

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
        const storedName = localStorage.getItem('business_name');
        if (storedName) setStoreName(storedName);

        const storedTenant = localStorage.getItem('tenant') || localStorage.getItem('tenant_id') || 'my-store';
        setTenant(storedTenant);
    }
  }, []);

  const addLink = () => {
    setLinks([...links, { id: Date.now().toString(), title: 'New Link', url: 'https://' }]);
  };

  const updateLink = (id: string, field: 'title' | 'url', value: string) => {
    setLinks(links.map(link => link.id === id ? { ...link, [field]: value } : link));
  };

  const removeLink = (id: string) => {
    setLinks(links.filter(link => link.id !== id));
  };

  // Save to local storage whenever settings change to simulate backend persistence
  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
        const payload = {
            storeName,
            bio,
            links,
            theme
        };
        localStorage.setItem(`ohc_bio_${tenant}`, JSON.stringify(payload));
    }
  }, [storeName, bio, links, theme, tenant]);

  const shareLink = `http://localhost:3000/bio/${tenant}`;

  const getThemeStyles = () => {
      switch(theme) {
          case 'dark': return { background: '#1D1D1F', color: '#ffffff' };
          case 'light': return { background: '#ffffff', color: '#1D1D1F', border: '1px solid #e5e7eb' };
          case 'purple': return { background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', color: '#ffffff' };
          case 'gradient': default: return { background: 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)', color: '#1D1D1F' };
      }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Link-in-Bio Generator 🔗</h1>
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
        <section className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Profile Details</h2>
                <div className="flex flex-col gap-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Business Name</label>
                        <input
                            aria-label="Business name"
                            type="text"
                            value={storeName}
                            onChange={(e) => setStoreName(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Bio / Tagline</label>
                        <textarea
                            aria-label="Bio tagline"
                            rows={2}
                            value={bio}
                            onChange={(e) => setBio(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-none"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                        <div className="flex gap-2">
                            <button aria-label="Gradient theme" aria-pressed={theme === 'gradient'} onClick={() => setTheme('gradient')} className={`w-8 h-8 rounded-full border-2 ${theme === 'gradient' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: 'linear-gradient(135deg, #a18cd1 0%, #fbc2eb 100%)' }}></button>
                            <button aria-label="Dark theme" aria-pressed={theme === 'dark'} onClick={() => setTheme('dark')} className={`w-8 h-8 rounded-full border-2 ${theme === 'dark' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: '#1D1D1F' }}></button>
                            <button aria-label="Light theme" aria-pressed={theme === 'light'} onClick={() => setTheme('light')} className={`w-8 h-8 rounded-full border-2 ${theme === 'light' ? 'border-indigo-600' : 'border-gray-200'}`} style={{ background: '#ffffff' }}></button>
                            <button aria-label="Purple theme" aria-pressed={theme === 'purple'} onClick={() => setTheme('purple')} className={`w-8 h-8 rounded-full border-2 ${theme === 'purple' ? 'border-indigo-600' : 'border-transparent'}`} style={{ background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)' }}></button>
                        </div>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Links</h2>
                <div className="flex flex-col gap-4">
                    {links.map((link, index) => (
                        <div key={link.id} className="p-4 border border-gray-200 rounded-lg bg-white">
                            <div className="flex justify-between items-center mb-2">
                                <span className="text-xs font-bold text-gray-500 uppercase tracking-wide">Link {index + 1}</span>
                                <button onClick={() => removeLink(link.id)} className="text-red-500 hover:text-red-700 text-xs font-medium">Remove</button>
                            </div>
                            <input
                                aria-label={`Link ${index + 1} title`}
                                type="text"
                                placeholder="Title (e.g. Visit my Shop)"
                                value={link.title}
                                onChange={(e) => updateLink(link.id, 'title', e.target.value)}
                                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 text-sm mb-2"
                            />
                            <input
                                aria-label={`Link ${index + 1} URL`}
                                type="url"
                                placeholder="URL (e.g. https://...)"
                                value={link.url}
                                onChange={(e) => updateLink(link.id, 'url', e.target.value)}
                                className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 text-sm"
                            />
                        </div>
                    ))}
                    <button
                        onClick={addLink}
                        className="w-full py-3 rounded-lg border-2 border-dashed border-gray-300 text-gray-600 font-medium hover:bg-gray-50 transition-colors"
                    >
                        + Add Another Link
                    </button>
                </div>
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Publish & Share</h2>
                <div className="flex flex-col gap-3">
                    <button
                        onClick={() => {
                            navigator.clipboard.writeText(shareLink);
                            setCopied(true);
                            setTimeout(() => setCopied(false), 2000);
                        }}
                        className={`w-full py-3 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                    >
                        {copied ? 'Copied Link!' : 'Copy Link-in-Bio URL'}
                    </button>
                    <p className="text-xs text-gray-500 text-center mt-1">Add this link to your Instagram, TikTok, or Twitter profile.</p>
                </div>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex justify-center items-start">
             <div className="w-[375px] h-[812px] bg-white rounded-[40px] shadow-2xl overflow-hidden relative border-[8px] border-gray-900 flex flex-col items-center">
                 {/* Notch */}
                 <div className="absolute top-0 w-40 h-6 bg-gray-900 rounded-b-2xl z-50"></div>

                 <div className="w-full h-full flex flex-col items-center overflow-y-auto pt-16 pb-12 px-6 transition-all duration-300" style={getThemeStyles()}>

                     <div className="w-24 h-24 rounded-full bg-white/20 shadow-inner flex items-center justify-center backdrop-blur-md mb-4 mt-4 border border-white/30 text-4xl">
                         ✨
                     </div>

                     <h1 className="text-2xl font-bold font-outfit mb-2 text-center drop-shadow-sm">
                         {storeName || 'My Store'}
                     </h1>

                     <p className="text-sm font-medium opacity-90 text-center mb-8 max-w-xs drop-shadow-sm">
                         {bio || 'Welcome to my storefront!'}
                     </p>

                     <div className="w-full flex flex-col gap-4">
                         {links.map((link) => (
                             <a
                                 key={link.id}
                                 href={link.url}
                                 target="_blank"
                                 rel="noopener noreferrer"
                                 className="w-full py-4 px-6 rounded-[16px] text-center font-semibold text-sm transition-transform hover:scale-[1.02] active:scale-95 shadow-sm"
                                 style={{
                                     background: theme === 'light' ? '#f3f4f6' : 'rgba(255, 255, 255, 0.15)',
                                     border: theme === 'light' ? '1px solid #e5e7eb' : '1px solid rgba(255, 255, 255, 0.3)',
                                     backdropFilter: 'blur(10px)',
                                     color: theme === 'light' ? '#111827' : '#ffffff'
                                 }}
                             >
                                 {link.title || 'Untitled Link'}
                             </a>
                         ))}
                     </div>

                     <div className="mt-auto pt-10 pb-6 w-full flex justify-center">
                         <a href={`https://ohc.store/join?ref=${tenant}`} className="text-xs font-semibold tracking-wide uppercase opacity-70 hover:opacity-100 transition-opacity">
                             ⚡ Powered by OHC
                         </a>
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
