import { PoweredByOHC } from "../components/PoweredByOHC";
"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ViralROICalculatorPage() {
  const router = useRouter();
  const [tenantId, setTenantId] = useState('default-team');
  const [serviceName, setServiceName] = useState('My Service');
  const [currency, setCurrency] = useState('$');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'e2e-tenant';
      setTenantId(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    } else {
      setRemoveBranding(e.target.checked);
    }
  };

  const embedUrl = `/api/v1/growth/viral-roi-calculator/embed?tenant=${encodeURIComponent(tenantId)}&serviceName=${encodeURIComponent(serviceName)}&currency=${encodeURIComponent(currency)}&theme=${theme}&branding=${!removeBranding}`;
  const absoluteEmbedUrl = `https://ohc.app/api/v1/growth/viral-roi-calculator/embed?tenant=${encodeURIComponent(tenantId)}&serviceName=${encodeURIComponent(serviceName)}&currency=${encodeURIComponent(currency)}&theme=${theme}&branding=${!removeBranding}`;

  const embedCode = `<iframe src="${absoluteEmbedUrl}" width="100%" height="450" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenantId)}&source=viral_roi_calculator" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleCopy = () => {
    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(embedCode);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  if (!isClient) return null;

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Viral ROI Calculator 📈</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 rounded-lg transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">

        {/* Configuration Panel */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
          <div className="text-left mb-2">
            <h2 className="text-3xl font-bold font-outfit text-gray-900 mb-2">ROI Calculator Generator</h2>
            <p className="text-gray-600">Embed a smart ROI calculator on your site to capture leads and drive sales. Built-in viral loop helps you acquire new customers!</p>
          </div>

          <div className="p-8 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-white/60 backdrop-blur-[40px] saturate-[200%] border border-white/40 rounded-3xl space-y-6">

            <div>
              <label htmlFor="serviceName" className="block text-sm font-medium text-gray-700 mb-2">Service or Product Name</label>
              <input
                type="text"
                id="serviceName"
                value={serviceName}
                onChange={(e) => setServiceName(e.target.value)}
                className="w-full px-4 py-3 border border-gray-300/50 rounded-xl bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all text-gray-900"
                placeholder="e.g. SEO Optimization"
              />
            </div>

            <div>
              <label htmlFor="currency" className="block text-sm font-medium text-gray-700 mb-2">Currency Symbol</label>
              <input
                type="text"
                id="currency"
                value={currency}
                onChange={(e) => setCurrency(e.target.value)}
                className="w-full px-4 py-3 border border-gray-300/50 rounded-xl bg-white/50 backdrop-blur-sm focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all text-gray-900"
                placeholder="e.g. $"
              />
            </div>

            <div>
               <label className="block text-sm font-medium text-gray-700 mb-2">Widget Theme</label>
               <div className="flex gap-4">
                  <label className="flex items-center gap-2 cursor-pointer">
                     <input type="radio" name="theme" value="light" checked={theme === 'light'} onChange={() => setTheme('light')} className="text-indigo-600 focus:ring-indigo-500" />
                     <span className="text-sm font-medium text-gray-700">Light</span>
                  </label>
                  <label className="flex items-center gap-2 cursor-pointer">
                     <input type="radio" name="theme" value="dark" checked={theme === 'dark'} onChange={() => setTheme('dark')} className="text-indigo-600 focus:ring-indigo-500" />
                     <span className="text-sm font-medium text-gray-700">Dark</span>
                  </label>
               </div>
            </div>

            <div className="flex items-center justify-between p-4 bg-gray-50/50 rounded-xl border border-gray-100">
               <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2 cursor-pointer">
                  Remove "Powered by OHC" Badge
                  {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
               </label>
               <div className="relative flex items-center">
                  <input
                     type="checkbox"
                     id="removeBranding"
                     checked={removeBranding}
                     onChange={handleRemoveBranding}
                     className="peer sr-only"
                  />
                  <div className="block h-6 w-11 cursor-pointer rounded-full bg-gray-300 transition-colors peer-checked:bg-[#0066FF] peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-[#0066FF] peer-focus:ring-offset-2"></div>
                  <div className="pointer-events-none absolute left-0.5 top-0.5 h-5 w-5 rounded-full bg-white transition-transform peer-checked:translate-x-5 shadow-sm"></div>
               </div>
            </div>

          </div>

          <div className="p-6 shadow-[0_8px_30px_rgb(0,0,0,0.12)] bg-white/60 backdrop-blur-[40px] saturate-[200%] border border-white/40 rounded-3xl">
             <h3 className="text-lg font-bold font-outfit text-gray-900 mb-2">Your Embed Code</h3>
             <p className="text-sm text-gray-600 mb-4">Copy this code and paste it into your website builder (Shopify, WordPress, Webflow, etc).</p>

             <div className="bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4">
                <pre>{embedCode}</pre>
             </div>

             <button
               id="copy-btn"
               onClick={handleCopy}
               className={`w-full py-4 rounded-xl font-bold transition-all shadow-md flex items-center justify-center gap-2 ${copied ? 'bg-green-100 text-green-700' : 'bg-[#0066FF] text-white hover:bg-[#0052CC]'}`}
             >
               {copied ? 'Copied to Clipboard!' : 'Copy Embed Code'}
             </button>
          </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex flex-col">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Live Preview</h2>
            <div className={`flex-1 rounded-3xl shadow-2xl border ${theme === 'dark' ? 'bg-gray-900 border-gray-800' : 'bg-white border-gray-200'} overflow-hidden min-h-[500px] flex items-center justify-center`}>
               <iframe src={embedUrl} width="100%" height="450" frameBorder="0" scrolling="no" style={{ border: 'none', overflow: 'hidden' }}></iframe>
            </div>
            {!removeBranding && (
               <div className="text-center mt-4">
                  <PoweredByOHC tenantId={tenantId} />
               </div>
            )}
        </section>

      </main>

      {/* Paywall Modal */}
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
               Make the ROI Calculator 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
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

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}