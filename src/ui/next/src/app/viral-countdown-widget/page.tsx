"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function ViralCountdownWidgetPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-business');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [eventName, setEventName] = useState('Product Launch');

  // Set default target date to tomorrow
  const [targetDate, setTargetDate] = useState(() => {
    const d = new Date();
    d.setDate(d.getDate() + 1);
    return d.toISOString().slice(0, 16);
  });

  const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [origin, setOrigin] = useState('https://ohc.app');

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setOrigin(window.location.origin);
      if (typeof localStorage !== 'undefined') {
        const storedTenant = localStorage.getItem('tenant');
        if (storedTenant) setTenant(storedTenant);
        setHasPro(localStorage.getItem('has_pro') === 'true');
      }
    }
    document.title = "Countdown Widget | OHC";
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    }
  };

  const embedUrl = `${origin}/api/v1/growth/viral-countdown-widget/embed?tenant=${encodeURIComponent(tenant)}&theme=${encodeURIComponent(theme)}&event=${encodeURIComponent(eventName)}&target=${encodeURIComponent(targetDate)}&branding=${!hasPro}`;
  const embedCode = `<iframe src="${embedUrl}" width="100%" height="220" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter items-center justify-center py-10 px-4" style={{ background: 'linear-gradient(to bottom right, #eff6ff, #fdf4ff, #fff1f2)' }}>
      <div className="w-full max-w-5xl bg-white/80 backdrop-blur-xl rounded-[24px] shadow-sm border border-gray-100 flex flex-col md:flex-row gap-8">
        <div className="flex-1 p-8">
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6">Viral Countdown Widget</h1>
          <div className="space-y-4">
             <div>
                <label htmlFor="eventName" className="block text-sm font-medium text-gray-700 mb-1">Event Name</label>
                <input id="eventName" type="text" value={eventName} onChange={(e) => setEventName(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 text-black" />
             </div>
             <div>
                <label htmlFor="targetDate" className="block text-sm font-medium text-gray-700 mb-1">Target Date & Time</label>
                <input id="targetDate" type="datetime-local" value={targetDate} onChange={(e) => setTargetDate(e.target.value)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 text-black" />
             </div>
             <div>
                <label htmlFor="themeSelect" className="block text-sm font-medium text-gray-700 mb-1">Theme</label>
                <select id="themeSelect" value={theme} onChange={(e) => setTheme(e.target.value as any)} className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white text-black">
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                </select>
             </div>
             <div className="flex items-center gap-2 mt-4 pt-4 border-t border-gray-200">
                <input
                    type="checkbox"
                    id="removeBranding"
                    checked={hasPro}
                    onChange={handleRemoveBranding}
                    className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                />
                <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2">
                    Remove "Powered by OHC" Badge
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

        <div className="flex-1 flex flex-col p-8">
           <h2 className="text-xl font-semibold font-outfit text-gray-900 mb-4">Live Preview</h2>
           <div className="flex-1 bg-gray-100 rounded-2xl shadow-inner border-2 border-dashed border-gray-300 relative overflow-hidden flex items-center justify-center p-6 min-h-[400px]">
              <iframe
                src={embedUrl}
                className="w-full max-w-sm rounded-xl shadow-md bg-transparent"
                style={{ height: '220px', border: 'none' }}
                title="Preview"
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
               Make the Countdown Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
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
