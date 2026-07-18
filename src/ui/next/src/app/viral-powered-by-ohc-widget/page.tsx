"use client";

import React, { useState, useEffect } from 'react';

export default function ViralPoweredByOHCWidgetPage() {
  const [tenant, setTenant] = useState('my-business');
    const [copied, setCopied] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);

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

  const referralLink = `https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}`;

  const embedCode = !hasPro ? `<!-- OHC Referral Footer Badge -->
<script>
  (function() {
    var b = document.createElement('a');
    b.href = '${referralLink}';
    b.target = '_blank';
    b.style.cssText = 'position:fixed;bottom:16px;right:16px;display:inline-flex;align-items:center;gap:6px;padding:6px 12px;background:rgba(0,0,0,0.8);color:#fff;border-radius:100px;font-family:sans-serif;font-size:13px;font-weight:500;text-decoration:none;z-index:9999;backdrop-filter:blur(10px);box-shadow:0 4px 12px rgba(0,0,0,0.1);';
    b.innerHTML = '<svg style="width:14px;height:14px;fill:currentColor;" viewBox="0 0 24 24"><path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/></svg> Powered by OHC';
    document.body.appendChild(b);
  })();
</script>` : '<!-- Badge removed on Pro Plan -->';

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  if (!isClient) return <div className="min-h-screen bg-indigo-50" />;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 items-center justify-center py-10 px-4">
      <div className="w-full max-w-4xl bg-white/80 backdrop-blur-xl rounded-[24px] shadow-sm border border-gray-100 flex flex-col lg:flex-row gap-8">
        <div className="flex-1 p-8">
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-6">Footer Badge Generator</h1>
          <p className="text-gray-600 mb-8 text-sm">Add a "Powered by OHC" badge to your website. If a visitor clicks it and signs up, you get a referral credit.</p>
          <div className="space-y-4">

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
              {!hasPro ? (
                  <div style={{ display: 'inline-flex', alignItems: 'center', gap: '6px', padding: '6px 12px', background: 'rgba(0,0,0,0.8)', color: '#fff', borderRadius: '100px', fontFamily: 'sans-serif', fontSize: '13px', fontWeight: 500, boxShadow: '0 4px 12px rgba(0,0,0,0.1)' }}>
                      <svg style={{ width: '14px', height: '14px', fill: 'currentColor' }} viewBox="0 0 24 24"><path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/></svg> Powered by OHC
                  </div>
              ) : (
                  <div className="text-gray-500 font-medium text-sm">Badge removed.</div>
              )}
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
