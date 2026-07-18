"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ViralTrustBadgeBuilderPage() {
  const router = useRouter();
  const [businessName, setBusinessName] = useState('My Store');
  const [statLabel, setStatLabel] = useState('Happy Customers');
  const [statValue, setStatValue] = useState('500+');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');

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
      document.title = "Trust Badge Builder | OHC";
    }
  }, []);

  const handleRemoveBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowPaywall(true);
    }
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return {
        background: '#1D1D1F',
        color: '#FFFFFF',
        borderColor: '#424245'
      };
    }
    return {
      background: '#FFFFFF',
      color: '#1D1D1F',
      borderColor: '#E5E5EA'
    };
  };

  const referralLink = `https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenant)}`;

  const getEmbedCode = () => {
    const badgeHtml = `
<div style="display:inline-flex;align-items:center;gap:12px;padding:8px 16px;background:${theme === 'dark' ? '#1D1D1F' : '#FFFFFF'};color:${theme === 'dark' ? '#FFFFFF' : '#1D1D1F'};border:1px solid ${theme === 'dark' ? '#424245' : '#E5E5EA'};border-radius:100px;font-family:system-ui,-apple-system,sans-serif;box-shadow:0 2px 8px rgba(0,0,0,0.05);transition:transform 0.2s;cursor:pointer;" onmouseover="this.style.transform='scale(1.02)'" onmouseout="this.style.transform='scale(1)'">
  <div style="display:flex;align-items:center;justify-content:center;width:32px;height:32px;background:#E0E7FF;color:#4F46E5;border-radius:50%;font-size:16px;">
    <svg style="width:18px;height:18px;fill:none;stroke:currentColor;stroke-width:2;stroke-linecap:round;stroke-linejoin:round;" viewBox="0 0 24 24"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
  </div>
  <div style="display:flex;flex-direction:column;">
    <span style="font-size:12px;opacity:0.7;font-weight:500;text-transform:uppercase;letter-spacing:0.5px;">${statLabel}</span>
    <span style="font-size:14px;font-weight:700;">${statValue} at ${businessName}</span>
  </div>
</div>
`;

    let fullCode = badgeHtml;

    if (!hasPro) {
        fullCode += `\n<div style="margin-top:8px;font-size:11px;color:#6B7280;font-family:system-ui,sans-serif;"><a href="${referralLink}" target="_blank" style="color:inherit;text-decoration:none;">⚡ Powered by OHC - Get this badge</a></div>`;
    }

    return fullCode.trim();
  };

  const claimTrialExtension = () => {
    const shareText = encodeURIComponent(`I just created a custom Trust Badge for my business using @OneHumanCorp! Get yours here: https://ohc.app/viral-trust-badge-builder`);
    window.open(`https://twitter.com/intent/tweet?text=${shareText}`, '_blank');

    setTimeout(() => {
        if (typeof localStorage !== 'undefined') {
            localStorage.setItem('has_pro', 'true');
            setHasPro(true);
            setShowPaywall(false);
            alert("Thanks for sharing! You've unlocked 7 days of Pro access.");
        }
    }, 3000);
  };

  if (!isClient) return <div className="min-h-screen bg-gray-50" />;

  return (
    <div className="min-h-screen flex flex-col font-inter bg-gray-50">
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-4 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-b border-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Trust Badge Builder 🛡️</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Settings Panel */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Badge Settings</h2>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Business Name</label>
                        <input
                            type="text"
                            value={businessName}
                            onChange={(e) => setBusinessName(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Stat Label</label>
                        <input
                            type="text"
                            value={statLabel}
                            onChange={(e) => setStatLabel(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            placeholder="e.g. Happy Customers, Orders Shipped"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Stat Value</label>
                        <input
                            type="text"
                            value={statValue}
                            onChange={(e) => setStatValue(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            placeholder="e.g. 500+, 10k+, 4.9/5"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                        <div className="flex gap-2">
                            <button aria-label="Light theme" aria-pressed={theme === 'light'} onClick={() => setTheme('light')} className={`w-8 h-8 rounded-full border-2 ${theme === 'light' ? 'border-indigo-600' : 'border-gray-300'}`} style={{ background: '#ffffff' }}></button>
                            <button aria-label="Dark theme" aria-pressed={theme === 'dark'} onClick={() => setTheme('dark')} className={`w-8 h-8 rounded-full border-2 ${theme === 'dark' ? 'border-indigo-600' : 'border-gray-300'}`} style={{ background: '#1D1D1F' }}></button>
                        </div>
                    </div>
                    <div className="flex items-center gap-2 mt-2 pt-4 border-t border-gray-200">
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
            </div>

            <div className="p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Embed on Your Site</h2>
                <div className="bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4">
                    <pre id="embed-code">
                        {getEmbedCode()}
                    </pre>
                </div>
                <button
                    onClick={() => {
                        navigator.clipboard.writeText(getEmbedCode());
                        setCopied(true);
                        setTimeout(() => setCopied(false), 2000);
                    }}
                    className={`w-full py-3 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                >
                    {copied ? 'Copied to Clipboard!' : 'Copy Embed Code'}
                </button>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-1/2 flex flex-col gap-4">
             <h2 className="text-xl font-semibold font-outfit" style={{ color: '#1D1D1F' }}>Live Preview</h2>

             <div className="w-full h-[600px] bg-gray-100 rounded-2xl shadow-inner border-2 border-dashed border-gray-300 relative overflow-hidden flex items-end justify-start p-6">
                 {/* Decorative background to look like a website */}
                 <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'linear-gradient(45deg, #ccc 25%, transparent 25%, transparent 75%, #ccc 75%, #ccc), linear-gradient(45deg, #ccc 25%, transparent 25%, transparent 75%, #ccc 75%, #ccc)', backgroundSize: '20px 20px', backgroundPosition: '0 0, 10px 10px' }}></div>

                 {/* The actual widget preview */}
                 <div className="z-10 flex flex-col mb-4">
                     <div
                        className="inline-flex items-center gap-3 px-4 py-2 rounded-full shadow-md transition-all duration-300 hover:scale-[1.02] cursor-pointer"
                        style={getThemeStyles()}
                     >
                         <div className="w-8 h-8 rounded-full bg-indigo-100 text-indigo-600 flex items-center justify-center">
                             <svg className="w-4 h-4" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" viewBox="0 0 24 24"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path><polyline points="22 4 12 14.01 9 11.01"></polyline></svg>
                         </div>
                         <div className="flex flex-col">
                             <span className="text-xs opacity-70 font-medium uppercase tracking-wide">{statLabel}</span>
                             <span className="text-sm font-bold">{statValue} at {businessName}</span>
                         </div>
                     </div>
                     {!hasPro && (
                         <div className="mt-2 ml-4">
                             <PoweredByOHC tenantId="trust-badge" />
                         </div>
                     )}
                 </div>
             </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center bg-white">
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
              Make the Trust Badge 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
            >
              Upgrade to Pro
            </button>

            <div className="my-4 text-gray-400 font-medium text-sm">OR</div>

            <button
              onClick={claimTrialExtension}
              className="w-full py-3.5 rounded-xl font-bold transition-all shadow-sm bg-black text-white border-2 border-black hover:bg-gray-800 flex items-center justify-center gap-2"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X to get 7 Days Free
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
