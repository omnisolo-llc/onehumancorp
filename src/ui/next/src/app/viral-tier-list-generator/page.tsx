'use client';

import React, { useState, useEffect } from 'react';
import Head from 'next/head';
import Link from 'next/link';

export default function ViralTierListGeneratorPage() {
  const [hasPro, setHasPro] = useState(false);
  const [tenant, setTenant] = useState('demo-store');

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
      const t = localStorage.getItem('tenant_id') || localStorage.getItem('tenant');
      if (t) setTenant(t);
    }
  }, []);

  const [title, setTitle] = useState('My Favorite Coffees');
  const [description, setDescription] = useState('Here are the best coffees I had this year.');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [generatedLink, setGeneratedLink] = useState('');

  const handleBrandingToggle = () => {
    if (!hasPro) {
      if (!removeBranding) {
        setShowSoftPaywall(true);
      } else {
        setRemoveBranding(false);
      }
    } else {
      setRemoveBranding(!removeBranding);
    }
  };

  const generateLink = () => {
    const url = `${window.location.origin}/tier-list?tenant=${tenant}&title=${encodeURIComponent(title)}&desc=${encodeURIComponent(description)}&branding=${!removeBranding}`;
    setGeneratedLink(url);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col p-6 font-inter">
      <Head>
        <title>Viral Tier List Generator</title>
      </Head>
      <main className="max-w-4xl mx-auto w-full">
        <h1 className="text-3xl font-bold font-outfit mb-8">Viral Tier List Generator</h1>

        <div className="bg-white rounded-2xl shadow-sm p-6 mb-8 border border-gray-100">
          <h2 className="text-xl font-semibold mb-4">Tier List Details</h2>
          <div className="space-y-4">
            <div>
              <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-1">List Title</label>
              <input
                id="title"
                type="text"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none"
                placeholder="e.g., Best Coffees of 2024"
              />
            </div>
            <div>
              <label htmlFor="description" className="block text-sm font-medium text-gray-700 mb-1">Description</label>
              <input
                id="description"
                type="text"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                className="w-full px-4 py-2 border border-gray-200 rounded-lg focus:ring-2 focus:ring-blue-500 focus:outline-none"
                placeholder="e.g., A definitive ranking of my favorites."
              />
            </div>
          </div>

          <div className="mt-6 pt-6 border-t border-gray-100 flex items-center justify-between">
             <div>
                <label className="font-semibold text-gray-900 flex items-center gap-2 cursor-pointer" onClick={handleBrandingToggle}>
                   Remove "Powered by OHC" Badge
                   {!hasPro && <span className="bg-gray-900 text-white text-[10px] uppercase font-bold px-2 py-0.5 rounded">Pro</span>}
                </label>
                <p className="text-sm text-gray-500 mt-1">Hide the watermark from the public view.</p>
             </div>
             <div
                className={`w-12 h-6 rounded-full flex items-center px-1 transition-colors cursor-pointer ${removeBranding ? 'bg-blue-600' : 'bg-gray-300'}`}
                onClick={handleBrandingToggle}
                data-testid="branding-toggle"
             >
                <div className={`w-4 h-4 bg-white rounded-full transition-transform ${removeBranding ? 'translate-x-6' : ''}`}></div>
             </div>
          </div>

          <div className="mt-8">
            <button
              onClick={generateLink}
              className="w-full py-3 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-lg transition-colors"
            >
              Generate Share Link
            </button>
          </div>

          {generatedLink && (
            <div className="mt-6 p-4 bg-gray-50 border border-gray-200 rounded-lg">
               <h3 className="font-semibold text-sm mb-2 text-gray-700">Your Viral Link:</h3>
               <input
                 type="text"
                 readOnly
                 value={generatedLink}
                 className="w-full px-3 py-2 border border-gray-300 rounded text-sm bg-white"
                 data-testid="generated-link"
               />
            </div>
          )}
        </div>

        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
          <div className="p-4 border-b border-gray-100 bg-gray-50">
             <h2 className="text-lg font-semibold text-center">Live Preview</h2>
          </div>
          <div className="p-8 flex flex-col items-center text-center">
             <div className="w-full max-w-md bg-white border border-gray-200 rounded-xl shadow-lg p-6 relative">
                <h1 className="text-2xl font-bold font-outfit mb-2">{title || 'Tier List Title'}</h1>
                <p className="text-gray-600 mb-6">{description || 'Tier list description'}</p>

                <div className="space-y-2 mb-8">
                   <div className="flex rounded overflow-hidden border border-red-200">
                     <div className="w-16 bg-red-500 text-white flex items-center justify-center font-bold text-lg">S</div>
                     <div className="flex-1 bg-gray-50 p-2 min-h-[40px]"></div>
                   </div>
                   <div className="flex rounded overflow-hidden border border-orange-200">
                     <div className="w-16 bg-orange-400 text-white flex items-center justify-center font-bold text-lg">A</div>
                     <div className="flex-1 bg-gray-50 p-2 min-h-[40px]"></div>
                   </div>
                   <div className="flex rounded overflow-hidden border border-yellow-200">
                     <div className="w-16 bg-yellow-400 text-white flex items-center justify-center font-bold text-lg">B</div>
                     <div className="flex-1 bg-gray-50 p-2 min-h-[40px]"></div>
                   </div>
                </div>

                {!removeBranding && (
                   <div className="mt-4 pt-4 border-t border-gray-100 text-center">
                      <Link href={`/onboarding?ref=${tenant}`} className="text-xs font-semibold text-gray-400 uppercase tracking-widest hover:text-gray-600 transition-colors" data-testid="preview-branding">
                         ⚡ Powered by OHC
                      </Link>
                   </div>
                )}
             </div>
          </div>
        </div>
      </main>

      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative text-center">
            <div className="w-16 h-16 bg-indigo-100 rounded-2xl flex items-center justify-center text-3xl shadow-inner text-indigo-600 mx-auto mb-6">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Make the Viral Tier List 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <Link href="/pricing" className="block w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md bg-gray-900 hover:bg-black">
              Upgrade to Pro ($79/mo)
            </Link>

            <button
              onClick={() => setShowSoftPaywall(false)}
              className="w-full py-3.5 rounded-xl font-bold transition-all bg-gray-100 hover:bg-gray-200 text-gray-900"
            >
              Keep Branding
            </button>
          </div>
        </div>
      )}
    </div>
  );
}