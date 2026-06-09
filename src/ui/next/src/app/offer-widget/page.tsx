"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function OfferWidgetPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('ohc-store');
  const [title, setTitle] = useState('Special Offer');
  const [description, setDescription] = useState('Get 20% off your first purchase!');
  const [buttonText, setButtonText] = useState('Claim Offer');
  const [buttonLink, setButtonLink] = useState('https://example.com/claim');
  const [theme, setTheme] = useState('light');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const activeTenant = localStorage.getItem('ohc_active_tenant_id');
      if (activeTenant) setTenant(activeTenant);
    }
  }, []);

  const embedCode = `<iframe src="${typeof window !== 'undefined' ? window.location.origin : ''}/embed/offer?tenant=${encodeURIComponent(tenant)}&theme=${theme}&title=${encodeURIComponent(title)}&desc=${encodeURIComponent(description)}&btn=${encodeURIComponent(buttonText)}&url=${encodeURIComponent(buttonLink)}${removeBranding ? '&branding=false' : ''}" width="100%" height="250" frameborder="0" style="border:none; border-radius:12px; background:transparent;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] font-inter">
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between bg-white border-b border-gray-200 sticky top-0 z-50 shadow-sm">
        <h1 className="text-xl font-bold font-outfit text-gray-900">Embeddable Offer Widget 🎁</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-100 rounded-lg text-sm font-medium hover:bg-gray-200 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-10 max-w-7xl mx-auto flex flex-col md:flex-row gap-8">
        {/* Settings Panel */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
          <div className="bg-white p-6 rounded-[24px] shadow-sm border border-gray-200">
            <h2 className="text-lg font-bold font-outfit text-gray-900 mb-6">Customize Widget</h2>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">Offer Title</label>
                <input
                  type="text"
                  value={title}
                  onChange={(e) => setTitle(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">Description</label>
                <textarea
                  value={description}
                  onChange={(e) => setDescription(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none h-20"
                />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-semibold text-gray-700 mb-2">Button Text</label>
                  <input
                    type="text"
                    value={buttonText}
                    onChange={(e) => setButtonText(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                </div>
                <div>
                  <label className="block text-sm font-semibold text-gray-700 mb-2">Theme</label>
                  <select
                    value={theme}
                    onChange={(e) => setTheme(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                  >
                    <option value="light">Light</option>
                    <option value="dark">Dark</option>
                  </select>
                </div>
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">Destination URL</label>
                <input
                  type="text"
                  value={buttonLink}
                  onChange={(e) => setButtonLink(e.target.value)}
                  className="w-full px-3 py-2 bg-gray-50 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              <div className="pt-4 border-t border-gray-100">
                <label className="flex items-center gap-2 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={removeBranding}
                    onChange={(e) => setRemoveBranding(e.target.checked)}
                    className="w-4 h-4 text-blue-600 rounded focus:ring-blue-500"
                  />
                  <span className="text-sm text-gray-700">Remove "Powered by OHC" branding</span>
                </label>
                <p className="text-xs text-gray-500 mt-1 ml-6">Requires Pro plan or higher.</p>
              </div>
            </div>

            <button
              onClick={() => setShowModal(true)}
              className="w-full mt-8 py-3 bg-blue-600 hover:bg-blue-700 text-white font-bold rounded-xl shadow-md transition-all active:scale-[0.98]"
            >
              Get Embed Code
            </button>
          </div>
        </div>

        {/* Live Preview Area */}
        <div className="w-full md:w-2/3 flex flex-col items-center">
          <h2 className="text-lg font-bold font-outfit text-gray-900 mb-4 self-start">Live Preview</h2>

          <div className="w-full max-w-2xl bg-white rounded-xl border border-gray-300 shadow-xl overflow-hidden mt-2">
            <div className="bg-gray-100 border-b border-gray-300 px-4 py-3 flex items-center gap-2">
              <div className="flex gap-1.5">
                <div className="w-3 h-3 rounded-full bg-red-400"></div>
                <div className="w-3 h-3 rounded-full bg-yellow-400"></div>
                <div className="w-3 h-3 rounded-full bg-green-400"></div>
              </div>
              <div className="ml-4 bg-white px-3 py-1 rounded border border-gray-200 text-xs text-gray-500 flex-1 text-center font-mono">
                yourwebsite.com/blog-post
              </div>
            </div>

            <div className="p-8 md:p-12" style={{ backgroundImage: 'radial-gradient(#e5e7eb 1px, transparent 1px)', backgroundSize: '20px 20px' }}>
              <div className="max-w-xl mx-auto space-y-6">
                <div className="h-4 w-3/4 bg-gray-200 rounded"></div>
                <div className="h-4 w-full bg-gray-200 rounded"></div>
                <div className="h-4 w-5/6 bg-gray-200 rounded"></div>

                {/* Actual Iframe Preview */}
                <div className="mt-8 rounded-xl border shadow-lg overflow-hidden flex flex-col items-center text-center bg-transparent">
                  <iframe
                    src={`/embed/offer?tenant=${encodeURIComponent(tenant)}&theme=${theme}&title=${encodeURIComponent(title)}&desc=${encodeURIComponent(description)}&btn=${encodeURIComponent(buttonText)}&url=${encodeURIComponent(buttonLink)}${removeBranding ? '&branding=false' : ''}`}
                    width="100%"
                    height="250"
                    frameBorder="0"
                    style={{ border: 'none', background: 'transparent' }}
                    title="Offer Widget Preview"
                  ></iframe>
                </div>

                <div className="h-4 w-4/5 bg-gray-200 rounded mt-8"></div>
                <div className="h-4 w-full bg-gray-200 rounded"></div>
              </div>
            </div>
          </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4">
          <div className="absolute inset-0 bg-black/50 backdrop-blur-sm" onClick={() => setShowModal(false)}></div>
          <div className="bg-white rounded-[24px] shadow-2xl p-8 max-w-2xl w-full relative z-10 animate-fade-in-up">
            <button
              onClick={() => setShowModal(false)}
              className="absolute top-6 right-6 text-gray-400 hover:text-gray-600"
            >
              <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Your Embed Code</h2>
            <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website, Notion, or blog.</p>

            <div className="relative group mb-6">
              <textarea
                readOnly
                value={embedCode}
                className="w-full h-32 p-4 bg-gray-50 border border-gray-200 rounded-xl font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>

            <div className="flex gap-4">
              <button
                onClick={handleCopy}
                className={`flex-1 py-3 rounded-xl font-bold text-sm transition-all shadow-sm flex items-center justify-center gap-2 ${copied ? 'bg-green-100 text-green-700' : 'bg-blue-600 text-white hover:bg-blue-700'}`}
              >
                {copied ? 'Copied to Clipboard!' : 'Copy Code'}
              </button>
              <button
                onClick={() => setShowModal(false)}
                className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-bold rounded-xl text-sm transition-colors"
              >
                Done
              </button>
            </div>
          </div>
        </div>
      )}


    </div>
  );
}
