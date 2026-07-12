"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function SpinToWinGeneratorPage() {
  const router = useRouter();
  const [discounts, setDiscounts] = useState('10%, 20%, Free Shipping, No Luck, 5%, 15%');
  const [campaignName, setCampaignName] = useState('Summer Spin to Win');
  const [reward, setReward] = useState('20% Off');
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState('DEFAULT');
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [embedCode, setEmbedCode] = useState('');

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setTenant(localStorage.getItem('tenant_id') || 'DEFAULT');
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleGenerate = () => {
    const prizes = discounts.split(',').map(d => d.trim()).filter(d => d);
    const prizeListStr = encodeURIComponent(JSON.stringify(prizes));
    const origin = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
    const iframeSrc = `${origin}/api/v1/growth/spin-to-win/embed?campaign=${encodeURIComponent(campaignName)}&reward=${encodeURIComponent(reward)}&tenant=${encodeURIComponent(tenant)}`;

    let code = `<!-- OHC Spin to Win Widget -->
<iframe src="${iframeSrc}" style="border: none; width: 100%; max-width: 400px; height: 350px;"></iframe>`;

    setEmbedCode(code);
    setShowModal(true);
    setCopied(false);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  };

  const handleToggleBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowSoftPaywall(true);
      return;
    }
  };

  const claimTrialExtension = () => {
    const text = `I'm capturing more leads with my OHC Spin to Win widget! Get 1 month free when you join: ${window.location.origin}/onboarding?ref=${tenant}\n\n⚡ Powered by OHC`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}`, '_blank');
    setShowSoftPaywall(false);
  };

  return (
    <div className="min-h-screen flex flex-col font-inter container glassmorphism" style={{ backgroundColor: '#F5F5F7', maxWidth: '450px', margin: '0 auto' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Spin to Win Generator 🎡</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col md:flex-row gap-8">
        <div className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="app-card p-6 shadow-sm border border-gray-100 bg-white">
                <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900">Configure Wheel</h2>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Campaign Name</label>
                        <input
                            id="campaign-name"
                            type="text"
                            placeholder="Summer Spin to Win"
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-black"
                            value={campaignName}
                            onChange={(e) => setCampaignName(e.target.value)}
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Reward</label>
                        <input
                            id="reward"
                            type="text"
                            placeholder="20% Off"
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-black"
                            value={reward}
                            onChange={(e) => setReward(e.target.value)}
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Prizes (comma separated)</label>
                        <input
                            type="text"
                            placeholder="10%, 20%, Free Shipping"
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-black"
                            value={discounts}
                            onChange={(e) => setDiscounts(e.target.value)}
                        />
                    </div>

                    <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg border border-gray-200 mt-4">
                        <div>
                            <p className="text-sm font-semibold text-gray-900">Remove OHC Branding</p>
                            <p className="text-xs text-gray-500">Requires Pro subscription</p>
                        </div>
                        <label className="relative inline-flex items-center cursor-pointer">
                            <input type="checkbox" className="sr-only peer" checked={hasPro} onChange={handleToggleBranding} />
                            <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-[#0071E3]"></div>
                        </label>
                    </div>

                    <button
                        onClick={handleGenerate}
                        disabled={!discounts}
                        className="w-full py-3 mt-4 bg-[#0071E3] hover:bg-blue-700 text-white font-bold rounded-xl transition-all shadow-md disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        Generate Widget
                    </button>
                </div>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-1/2">
            <div className="p-8 h-full flex flex-col items-center justify-center relative overflow-hidden bg-white border border-gray-200 shadow-sm">
                <div className="absolute top-4 left-4 text-xs font-semibold text-gray-400 uppercase tracking-wider">Live Preview</div>

                <div className="w-full max-w-sm border border-dashed border-gray-300 p-6 rounded-xl text-center bg-gray-50">
                    <h3 className="text-2xl font-bold text-gray-900 mb-2">Spin to Win!</h3>
                    <p className="text-sm text-gray-600 mb-4">Enter your email to spin the wheel.</p>
                    <input type="email" placeholder="Enter email" className="w-full px-4 py-2 mb-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] text-black" disabled />
                    <button className="w-full py-2 bg-[#0071E3] text-white font-bold rounded-lg shadow-sm" disabled>SPIN NOW</button>
                </div>

                {!hasPro && (
                    <div className="mt-6 text-center" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
                        <PoweredByOHC tenantId={tenant} />
                    </div>
                )}
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="app-card p-8 max-w-xl w-full shadow-2xl relative animate-in fade-in bg-white">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Spin to Win</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website.</p>

                <div className="relative group">
                    <pre className="w-full h-40 p-4 bg-gray-50 border border-gray-200 font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all overflow-auto">
                        <code style={{ whiteSpace: 'pre-wrap' }}>{embedCode}</code>
                    </pre>
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-[#0071E3] hover:bg-blue-700 text-white font-medium transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center bg-white">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="text-5xl mb-4">✨</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Removing OHC branding is a Pro feature. Upgrade to our Pro plan to customize your widgets.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
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
              Share on X to get 7 Days Free
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
