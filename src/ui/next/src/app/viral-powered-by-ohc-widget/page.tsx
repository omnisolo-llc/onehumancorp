"use client";

import React, { useState, useEffect } from 'react';

export default function ViralPoweredByOHCWidgetPage() {
  const [tenant, setTenant] = useState('my-business');
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);

  const [title, setTitle] = useState('My Awesome Tool');
  const [theme, setTheme] = useState('light');

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-business';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
    if (typeof document !== 'undefined') {
      document.title = "Viral Widget | OHC";
    }
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    }
  };

  const iframeSrc = `/api/widgets/viral-preview?title=${encodeURIComponent(title)}&theme=${theme}&branding=${!hasPro}&ref=${encodeURIComponent(tenant)}`;

  const embedCode = `<!-- OHC Viral Widget -->
<iframe src="https://ohc.app${iframeSrc}" width="100%" height="400" style="border:none; border-radius: 12px; box-shadow: 0 4px 12px rgba(0,0,0,0.1);"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isClient) return <div className="min-h-screen bg-indigo-50" />;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 items-center justify-center py-10 px-4">
      <div className="w-full max-w-4xl bg-white/80 backdrop-blur-xl rounded-[24px] shadow-sm border border-gray-100 flex flex-col lg:flex-row gap-8">
        <div className="flex-1 p-8 flex flex-col">
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6">Viral Widget Builder</h1>
          <p className="text-gray-600 mb-8 text-sm">Design your widget. If a visitor clicks the branding and signs up, you get a referral credit.</p>

          <div className="space-y-6 flex-1">
             <div>
               <label htmlFor="widgetTitle" className="block text-sm font-medium text-gray-700 mb-1">Widget Title</label>
               <input
                 type="text"
                 id="widgetTitle"
                 value={title}
                 onChange={(e) => setTitle(e.target.value)}
                 className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition-all text-sm"
                 placeholder="e.g. My Awesome Viral Tool"
               />
             </div>

             <div>
               <label htmlFor="widgetTheme" className="block text-sm font-medium text-gray-700 mb-1">Theme</label>
               <select
                 id="widgetTheme"
                 value={theme}
                 onChange={(e) => setTheme(e.target.value)}
                 className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition-all text-sm bg-white"
               >
                 <option value="light">Light</option>
                 <option value="dark">Dark</option>
               </select>
             </div>

             <div className="flex items-center gap-2 pt-4 border-t border-gray-200">
                <input
                    type="checkbox"
                    id="removeBranding"
                    checked={hasPro}
                    onChange={handleRemoveBranding}
                    className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                />
                <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2">
                    Remove "Powered by OHC"
                    {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                </label>
             </div>
          </div>

          <div className="mt-8 bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4">
             <pre>{embedCode}</pre>
          </div>
          <button
             onClick={handleCopy}
             className={`w-full py-3 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
          >
             {copied ? 'Copied to Clipboard!' : 'Copy Embed Code'}
          </button>
        </div>

        <div className="flex-1 flex flex-col p-8 bg-gray-50 rounded-r-[24px] lg:border-l border-gray-200">
           <h2 className="text-xl font-semibold font-outfit text-gray-900 mb-4">Live Preview</h2>
           <div className="flex-1 rounded-2xl shadow-inner border border-gray-300 relative overflow-hidden flex items-center justify-center min-h-[400px] w-full max-w-[375px] mx-auto bg-white">
              <iframe
                src={iframeSrc}
                className="w-full h-full border-none"
                title="Widget Preview"
              />
           </div>
        </div>
      </div>

      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white rounded-2xl max-w-md p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center">
             <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>
             <div className="flex justify-end mb-2">
               <button
                 aria-label="Close paywall"
                 onClick={() => setShowPaywall(false)}
                 className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
               >
                 <span className="text-xl leading-none">&times;</span>
               </button>
             </div>
             <div className="w-16 h-16 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center text-3xl shadow-lg mx-auto mb-6 text-white font-bold">
               PRO
             </div>
             <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Remove Branding</h2>
             <p className="text-gray-600 mb-6 text-sm leading-relaxed">
               Make the Viral Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
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
