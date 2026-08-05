"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ViralBundleBuilderPage() {
  const router = useRouter();
  const [tenantId, setTenantId] = useState('demo-store');
  const [hasPro, setHasPro] = useState(false);
  const [isClient, setIsClient] = useState(false);

  // Form State
  const [bundleTitle, setBundleTitle] = useState('Build Your Dream Bundle');
  const [bundleDiscount, setBundleDiscount] = useState('15%');
  const [viralReward, setViralReward] = useState('10% Extra Off');
  const [sharesRequired, setSharesRequired] = useState(3);
  const [removeBranding, setRemoveBranding] = useState(false);

  // UI State
  const [showPaywall, setShowPaywall] = useState(false);
  const [copied, setCopied] = useState(false);
  const [theme, setTheme] = useState('light');

  useEffect(() => {
    setIsClient(true);
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'demo-store';
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

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const claimTrialExtension = async () => {
    try {
      const res = await fetch('/api/v1/growth/trial-extension/claim', { method: 'POST' });
      if (res.ok) {
        setHasPro(true);
        if (typeof localStorage !== 'undefined') localStorage.setItem('has_pro', 'true');
        setShowPaywall(false);
        setRemoveBranding(true);
      }
    } catch (e) {
      console.error(e);
    }
  };

  const embedUrl = `https://ohc.app/api/v1/growth/viral-bundle/embed?tenant=${tenantId}&title=${encodeURIComponent(bundleTitle)}&discount=${encodeURIComponent(bundleDiscount)}&reward=${encodeURIComponent(viralReward)}&shares=${sharesRequired}&theme=${theme}&branding=${!removeBranding}`;
  const embedCode = `<iframe src="${embedUrl}" width="100%" height="600" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}&source=viral_bundle_builder" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  if (!isClient) return <div className="min-h-screen bg-gray-50 dark:bg-gray-900" />;

  const getThemeStyles = () => {
      if (theme === 'dark') {
          return { bg: 'bg-[#1D1D1F]', text: 'text-white', border: 'border-gray-700', muted: 'text-gray-400', cardBg: 'bg-gray-800' };
      }
      return { bg: 'bg-white', text: 'text-gray-900', border: 'border-gray-200', muted: 'text-gray-500', cardBg: 'bg-gray-50' };
  };
  const themeStyles = getThemeStyles();

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gray-50 dark:bg-[#000000]">
      <header className="bg-white dark:bg-[#1D1D1F] border-b border-gray-200 dark:border-gray-800 px-6 py-4 flex items-center justify-between sticky top-0 z-10 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-white tracking-tight">Viral Bundle Builder 📦</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 dark:bg-gray-800 dark:text-white min-h-[44px] min-w-[44px] text-sm font-medium hover:bg-gray-300 dark:hover:bg-gray-700 transition-colors rounded-lg"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-7xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Editor Settings */}
        <section className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-md bg-white/65 dark:bg-[#1D1D1F]/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl">
                <h2 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-6">Bundle Settings</h2>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Headline</label>
                        <input
                            type="text"
                            value={bundleTitle}
                            onChange={(e) => setBundleTitle(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 dark:bg-gray-900 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Base Discount</label>
                        <input
                            type="text"
                            value={bundleDiscount}
                            onChange={(e) => setBundleDiscount(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 dark:bg-gray-900 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            placeholder="e.g. 15% Off"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Viral Reward (Extra Off)</label>
                        <input
                            type="text"
                            value={viralReward}
                            onChange={(e) => setViralReward(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 dark:bg-gray-900 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                            placeholder="e.g. 10% Extra Off"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Shares to Unlock Reward</label>
                        <input
                            type="number"
                            min="1"
                            max="10"
                            value={sharesRequired}
                            onChange={(e) => setSharesRequired(parseInt(e.target.value) || 1)}
                            className="w-full px-4 py-2 border border-gray-300 dark:border-gray-700 dark:bg-gray-900 dark:text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Theme</label>
                        <div className="flex gap-2">
                            <button aria-label="Light theme" aria-pressed={theme === 'light'} onClick={() => setTheme('light')} className={`w-8 h-8 rounded-full border-2 ${theme === 'light' ? 'border-indigo-600' : 'border-gray-300'}`} style={{ background: '#ffffff' }}></button>
                            <button aria-label="Dark theme" aria-pressed={theme === 'dark'} onClick={() => setTheme('dark')} className={`w-8 h-8 rounded-full border-2 ${theme === 'dark' ? 'border-indigo-600' : 'border-gray-300'}`} style={{ background: '#1D1D1F' }}></button>
                        </div>
                    </div>
                    <div className="flex items-center gap-2 mt-2 pt-4 border-t border-gray-200 dark:border-gray-700">
                        <input
                            type="checkbox"
                            id="removeBranding"
                            checked={removeBranding}
                            onChange={handleRemoveBranding}
                            className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                        />
                        <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 dark:text-gray-300 flex items-center gap-2">
                            Remove "Powered by OHC" Badge
                            {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                        </label>
                    </div>
                </div>
            </div>

            <div className="p-6 shadow-md bg-white/65 dark:bg-[#1D1D1F]/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-2xl">
                <h2 className="text-xl font-semibold font-outfit mb-4 text-gray-900 dark:text-white">Embed on Your Site</h2>
                <div className="bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4 border border-gray-700">
                    <pre id="embed-code">
                        {embedCode}
                    </pre>
                </div>
                <button
                    onClick={handleCopy}
                    className={`w-full py-3 rounded-lg text-sm font-semibold transition-all shadow-sm min-h-[44px] ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
                >
                    {copied ? 'Copied to Clipboard!' : 'Copy Embed Code'}
                </button>
            </div>
        </section>

        {/* Live Preview */}
        <section className="w-full md:w-2/3 flex flex-col gap-4">
             <div className="flex items-center justify-between">
                <h2 className="text-xl font-semibold font-outfit text-gray-900 dark:text-white">Live Preview</h2>
                <div className="flex gap-1.5">
                    <div className="w-3 h-3 rounded-full bg-red-400"></div>
                    <div className="w-3 h-3 rounded-full bg-amber-400"></div>
                    <div className="w-3 h-3 rounded-full bg-green-400"></div>
                </div>
             </div>

             <div className="flex-1 w-full bg-gray-100 dark:bg-gray-800 rounded-2xl shadow-inner border-2 border-dashed border-gray-300 dark:border-gray-600 relative overflow-hidden flex items-center justify-center p-4 min-h-[600px]">
                 {/* The actual widget preview */}
                 <div className={`w-full max-w-lg ${themeStyles.bg} ${themeStyles.text} rounded-[24px] shadow-2xl overflow-hidden border ${themeStyles.border} transition-colors duration-300`}>

                    {/* Header */}
                    <div className="p-6 text-center border-b border-gray-100 dark:border-gray-800">
                        <div className="w-16 h-16 bg-gradient-to-br from-indigo-500 to-purple-600 text-white rounded-2xl flex items-center justify-center text-2xl mb-4 mx-auto shadow-lg">
                            📦
                        </div>
                        <h3 className="text-2xl font-bold font-outfit mb-2">{bundleTitle}</h3>
                        <p className={`text-sm ${themeStyles.muted}`}>Bundle 3 items to get <span className="font-bold text-indigo-500">{bundleDiscount}</span>.</p>
                    </div>

                    {/* Bundle Selection Mock */}
                    <div className="p-6">
                        <div className="grid grid-cols-3 gap-3 mb-6">
                            {[1, 2, 3].map(i => (
                                <div key={i} className={`aspect-square rounded-xl border-2 border-dashed ${themeStyles.border} ${themeStyles.cardBg} flex items-center justify-center text-2xl`}>
                                    {i === 1 ? '👕' : i === 2 ? '👖' : '+'}
                                </div>
                            ))}
                        </div>

                        {/* Viral Unlock Section */}
                        <div className={`mt-2 p-5 rounded-2xl bg-gradient-to-br from-pink-50 to-orange-50 dark:from-pink-900/20 dark:to-orange-900/20 border border-pink-100 dark:border-pink-800/30 text-center relative overflow-hidden`}>
                            <div className="absolute top-0 right-0 w-24 h-24 bg-pink-500/10 rounded-bl-full -z-0"></div>

                            <h4 className="font-bold text-gray-900 dark:text-white mb-2 relative z-10 flex items-center justify-center gap-2">
                                <span>🚀</span> Unlock {viralReward}
                            </h4>
                            <p className="text-sm text-gray-600 dark:text-gray-300 mb-4 relative z-10">
                                Share this bundle with {sharesRequired} friends to unlock the ultimate discount!
                            </p>

                            <div className="w-full mb-4 relative z-10">
                                <div className="h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                                    <div className="h-full bg-gradient-to-r from-pink-500 to-orange-500 w-1/3 rounded-full"></div>
                                </div>
                                <div className="flex justify-between text-[10px] font-bold text-gray-500 dark:text-gray-400 uppercase tracking-wider mt-2">
                                    <span>0 Shares</span>
                                    <span className="text-pink-500">1 / {sharesRequired}</span>
                                    <span>{sharesRequired} Shares</span>
                                </div>
                            </div>

                            <div className="flex flex-col gap-2 relative z-10">
                                <button className="w-full py-3 rounded-xl bg-[#25D366] text-white font-bold text-sm shadow-md hover:bg-[#20bd5a] transition-colors flex items-center justify-center gap-2">
                                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                                    Share to WhatsApp
                                </button>
                                <button className="w-full py-3 rounded-xl bg-black text-white font-bold text-sm shadow-md hover:bg-gray-800 transition-colors flex items-center justify-center gap-2">
                                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"></path></svg>
                                    Share on X
                                </button>
                            </div>
                        </div>

                        <button className="w-full py-4 mt-4 rounded-xl font-bold bg-indigo-600 text-white shadow-lg shadow-indigo-600/20 hover:bg-indigo-700 hover:shadow-indigo-600/30 transition-all active:scale-[0.98]">
                            Add Bundle to Cart
                        </button>
                    </div>

                    {!removeBranding && (
                        <div className={`p-3 text-center border-t ${themeStyles.border} bg-gray-50/50 dark:bg-gray-900/50`}>
                            <span className="text-[11px] font-bold tracking-wide uppercase text-gray-500 dark:text-gray-400">⚡ Powered by OHC</span>
                        </div>
                    )}
                 </div>
             </div>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4 backdrop-blur-sm">
          <div className="app-card w-full max-w-md bg-white rounded-3xl p-8 shadow-2xl relative overflow-hidden font-inter text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>

            <button
                aria-label="Close paywall"
                onClick={() => setShowPaywall(false)}
                className="absolute top-4 right-4 text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
            >
                <span className="text-xl leading-none">&times;</span>
            </button>

            <div className="w-16 h-16 bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center text-3xl shadow-lg mx-auto mb-6 text-white font-bold">
              PRO
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Remove Branding</h2>
            <p className="text-gray-600 mb-8 text-sm leading-relaxed">
              Make the Viral Bundle Builder 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark and unlock premium widget themes.
            </p>

            <button
              onClick={() => { setShowPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 min-h-[56px]"
              style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
            >
              Upgrade to Pro
            </button>

            <button
              onClick={claimTrialExtension}
              className="w-full py-4 rounded-xl font-bold transition-all shadow-sm bg-black text-white hover:bg-gray-800 flex items-center justify-center gap-2 min-h-[56px]"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
              Share on X for 7-Day Pro Trial
            </button>
          </div>
        </div>
      )}

      <PoweredByOHC tenantId={tenantId} />

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
