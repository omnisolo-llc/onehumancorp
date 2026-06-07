'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function ReviewWallPage() {
  const [tenant, setTenant] = useState('my-business');
  const [theme, setTheme] = useState('light');
  const [embedCode, setEmbedCode] = useState('');
  const [previewUrl, setPreviewUrl] = useState('');

  useEffect(() => {
    const url = `https://ohc.app/api/v1/growth/review-wall/embed?tenant=${encodeURIComponent(tenant)}&theme=${encodeURIComponent(theme)}`;
    const code = `<iframe src="${url}" width="100%" height="400" style="border:none; border-radius:12px; overflow:hidden;"></iframe>`;
    setEmbedCode(code);
    setPreviewUrl(`/api/v1/growth/review-wall/embed?tenant=${encodeURIComponent(tenant)}&theme=${encodeURIComponent(theme)}`);
  }, [tenant, theme]);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(embedCode);
    alert('Embed code copied to clipboard!');
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Review Wall Widget ⭐</h1>
         <div className="flex items-center gap-3">
             <Link href="/dashboard" className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </Link>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8">
        <div className="bg-gradient-to-r from-blue-50 to-indigo-50 border border-blue-100 rounded-2xl p-6 shadow-sm">
           <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Embed Your Best Reviews</h2>
           <p className="text-gray-600 text-sm">
             Generate a dynamic Review Wall to display your top customer ratings on any website. Build trust and drive more sales.
           </p>
        </div>

        <div className="flex flex-col md:flex-row gap-8">
          <section className="w-full md:w-1/2 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Widget Settings</h3>
            <div className="flex flex-col gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Tenant ID</label>
                <input
                  type="text"
                  value={tenant}
                  onChange={(e) => setTenant(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Theme</label>
                <select
                  value={theme}
                  onChange={(e) => setTheme(e.target.value)}
                  className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                </select>
              </div>
              <div className="mt-4">
                <label className="block text-sm font-medium text-gray-700 mb-1">Embed Code</label>
                <textarea
                  readOnly
                  value={embedCode}
                  className="w-full h-32 px-4 py-2 border border-gray-300 rounded-lg bg-gray-50 text-sm font-mono text-gray-600 focus:outline-none"
                />
              </div>
              <button
                onClick={copyToClipboard}
                className="w-full py-3 mt-2 text-white font-semibold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 bg-blue-600 hover:bg-blue-700"
              >
                Copy Embed Code
              </button>
            </div>
          </section>

          <section className="w-full md:w-1/2 p-6 shadow-md flex flex-col" style={{ background: '#ffffff', border: '1px solid rgba(0, 0, 0, 0.05)', borderRadius: '16px' }}>
            <h3 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>
              Live Preview
            </h3>
            <div className="flex-1 bg-gray-50 rounded-xl overflow-hidden border border-gray-200">
              <iframe
                src={previewUrl}
                width="100%"
                height="400"
                style={{ border: 'none' }}
                title="Review Wall Preview"
              />
            </div>
          </section>
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
