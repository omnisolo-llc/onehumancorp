"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ViralBeforeAfterSliderPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-business');
  const [title, setTitle] = useState('Our Work');
  const [beforeUrl, setBeforeUrl] = useState('https://images.unsplash.com/photo-1584622650111-993a426fbf0a?auto=format&fit=crop&q=80&w=800');
  const [afterUrl, setAfterUrl] = useState('https://images.unsplash.com/photo-1527515637462-cff94eecc1ac?auto=format&fit=crop&q=80&w=800');
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);
  const [showModal, setShowModal] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-business';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
    document.title = "Before & After Slider | OHC";
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    }
  };

  const embedUrl = `https://ohc.app/api/v1/growth/viral-before-after/embed?tenant=${tenant}&title=${encodeURIComponent(title)}&before=${encodeURIComponent(beforeUrl)}&after=${encodeURIComponent(afterUrl)}&branding=${!hasPro}`;
  const embedCode = `<iframe src="${embedUrl}" width="100%" height="450" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isClient) return null;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gray-50 items-center justify-center py-10 px-4">
      <main className="w-full max-w-5xl bg-white rounded-3xl shadow-xl border border-gray-100 flex flex-col md:flex-row overflow-hidden">
        {/* Configuration Panel */}
        <div className="w-full md:w-1/2 p-8 border-r border-gray-100 flex flex-col">
            <div className="mb-8">
                <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">Before & After Slider</h1>
                <p className="text-gray-500">Showcase your best work and drive leads.</p>
            </div>

            <div className="space-y-5 flex-1">
                <div>
                    <label className="block text-sm font-semibold text-gray-700 mb-1">Widget Title</label>
                    <input
                        type="text"
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 transition-shadow"
                    />
                </div>
                <div>
                    <label className="block text-sm font-semibold text-gray-700 mb-1">Before Image URL</label>
                    <input
                        type="text"
                        value={beforeUrl}
                        onChange={(e) => setBeforeUrl(e.target.value)}
                        className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 transition-shadow"
                        placeholder="https://..."
                    />
                </div>
                <div>
                    <label className="block text-sm font-semibold text-gray-700 mb-1">After Image URL</label>
                    <input
                        type="text"
                        value={afterUrl}
                        onChange={(e) => setAfterUrl(e.target.value)}
                        className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 transition-shadow"
                        placeholder="https://..."
                    />
                </div>

                <div className="pt-4 border-t border-gray-100 mt-4">
                    <label className="flex items-center gap-3 cursor-pointer group">
                        <div className="relative flex items-center">
                            <input
                                type="checkbox"
                                id="removeBranding"
                                checked={hasPro}
                                onChange={handleRemoveBranding}
                                className="w-5 h-5 text-blue-600 border-gray-300 rounded focus:ring-blue-500 transition-colors"
                            />
                        </div>
                        <span className="text-sm font-medium text-gray-700 group-hover:text-gray-900 transition-colors flex items-center gap-2">
                            Remove "Powered by OHC" Badge
                            {!hasPro && <span className="bg-gradient-to-r from-amber-200 to-yellow-400 text-yellow-900 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider shadow-sm">PRO</span>}
                        </span>
                    </label>
                </div>
            </div>

            <div className="mt-8">
                <button
                    onClick={() => setShowModal(true)}
                    className="w-full py-3 bg-gray-900 hover:bg-black text-white font-semibold rounded-xl min-h-[44px] min-w-[44px] transition-all shadow-md hover:shadow-lg flex items-center justify-center gap-2"
                >
                    Get Widget Embed Code
                </button>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-1/2 bg-gray-100 p-8 flex flex-col">
            <div className="text-xs font-semibold text-gray-400 uppercase tracking-wider mb-4">Live Preview</div>
            <div className="flex-1 w-full bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden relative min-h-[450px]">
                <iframe
                    src={`/api/v1/growth/viral-before-after/embed?tenant=${tenant}&title=${encodeURIComponent(title)}&before=${encodeURIComponent(beforeUrl)}&after=${encodeURIComponent(afterUrl)}&branding=${!hasPro}`}
                    className="absolute inset-0 w-full h-full border-none"
                    title="Live Preview"
                />
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
            <div className="bg-white p-8 rounded-2xl max-w-xl w-full shadow-2xl relative">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Slider</h2>
                <p className="text-gray-600 mb-6 text-sm">Copy and paste this HTML snippet into your website to embed the before & after slider.</p>

                <textarea
                    readOnly
                    value={embedCode}
                    className="w-full h-32 p-4 bg-gray-50 border border-gray-200 rounded-lg font-mono text-xs text-gray-800 resize-none focus:outline-none mb-4"
                />

                <div className="flex gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-xl transition-colors shadow-sm"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium rounded-xl transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}

      {/* Paywall */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white rounded-2xl max-w-md p-8 shadow-2xl relative text-center">
             <div className="w-16 h-16 bg-gradient-to-br from-amber-400 to-orange-500 rounded-2xl flex items-center justify-center text-3xl shadow-lg mx-auto mb-6 text-white font-bold">
               PRO
             </div>
             <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Remove Branding</h2>
             <p className="text-gray-600 mb-6 text-sm leading-relaxed">
               Make the Before & After Slider 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
             </p>
             <button
               onClick={() => { setShowPaywall(false); window.location.href = '/pricing'; }}
               className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
               style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
             >
               Upgrade to Pro
             </button>
             <button
               onClick={() => setShowPaywall(false)}
               className="mt-2 text-gray-500 hover:text-gray-700 font-medium text-sm w-full"
             >
               Cancel
             </button>
          </div>
        </div>
      )}
    </div>
  );
}
