"use client";

import React, { useState, useEffect } from 'react';
import { PoweredByOHC } from '../components/PoweredByOHC';
import { useRouter } from 'next/navigation';

export default function ViralFreeShippingUnlock() {
    const router = useRouter();
    const [tenantId, setTenantId] = useState("default-team");
    const [minSpend, setMinSpend] = useState(50);
    const [sharesRequired, setSharesRequired] = useState(3);
    const [theme, setTheme] = useState<'light' | 'dark'>('light');
    const [hideBranding, setHideBranding] = useState(false);
    const [hasPro, setHasPro] = useState(false);
    const [showPaywall, setShowPaywall] = useState(false);
    const [copied, setCopied] = useState(false);

    useEffect(() => {
        if (typeof window !== 'undefined') {
            const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant');
            if (storedTenant) setTenantId(storedTenant);
            setHasPro(localStorage.getItem('has_pro') === 'true');
        }
    }, []);

    const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (!hasPro && e.target.checked) {
            e.preventDefault();
            setShowPaywall(true);
        } else {
            setHideBranding(e.target.checked);
        }
    };

    const getEmbedCode = () => {
        const baseUrl = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
        const iframeUrl = `${baseUrl}/embed/free-shipping?tenant_id=${encodeURIComponent(tenantId)}&min_spend=${minSpend}&shares=${sharesRequired}&theme=${theme}${hideBranding ? '&hideBranding=true' : ''}`;

        let code = `<iframe src="${iframeUrl}" width="100%" height="200" style="border:none;border-radius:12px;overflow:hidden;" title="Unlock Free Shipping"></iframe>`;

        if (!hideBranding) {
            const referralLink = `${baseUrl}/onboarding?ref=${encodeURIComponent(tenantId)}&source=free_shipping_widget`;
            code += `\n<div style="text-align:center;margin-top:8px;font-family:sans-serif;font-size:12px;">\n  <a href="${referralLink}" target="_blank" rel="noopener noreferrer" style="color:#6b7280;text-decoration:none;">⚡ Powered by OHC</a>\n</div>`;
        }

        return code;
    };

    const handleCopy = () => {
        if (navigator.clipboard) {
            navigator.clipboard.writeText(getEmbedCode());
            setCopied(true);
            setTimeout(() => setCopied(false), 2000);
        }
    };

    return (
        <div className="flex flex-col min-h-screen font-inter bg-gray-50/50">
            {/* Header */}
            <header className="bg-white/80 backdrop-blur-xl border-b border-gray-200 px-6 py-4 flex items-center justify-between sticky top-0 z-40">
                <div className="flex items-center gap-3">
                    <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">Viral Free Shipping Unlock 🚚</h1>
                    <span className="bg-indigo-100 text-indigo-700 text-xs font-bold px-2 py-1 rounded-md uppercase tracking-wider">Growth Widget</span>
                </div>
                <button
                    onClick={() => router.push('/dashboard')}
                    className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-700 rounded-xl text-sm font-semibold transition-colors"
                >
                    Back to Dashboard
                </button>
            </header>

            <main className="flex-1 max-w-7xl mx-auto w-full p-6 lg:p-8">
                <div className="flex flex-col lg:flex-row gap-8">
                    {/* Left Column: Configuration */}
                    <div className="w-full lg:w-1/3 flex flex-col gap-6">
                        <div className="bg-white rounded-[24px] border border-gray-200 shadow-sm p-6">
                            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Widget Settings</h2>
                            <p className="text-sm text-gray-500 mb-6">Incentivize customers to share your store to unlock free shipping.</p>

                            <div className="space-y-5">
                                <div>
                                    <label className="block text-sm font-semibold text-gray-900 mb-1.5">Minimum Spend ($)</label>
                                    <input
                                        type="number"
                                        min="0"
                                        value={minSpend}
                                        onChange={(e) => setMinSpend(parseInt(e.target.value) || 0)}
                                        className="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 transition-all outline-none"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-semibold text-gray-900 mb-1.5">Shares Required to Unlock</label>
                                    <input
                                        type="number"
                                        min="1"
                                        max="10"
                                        value={sharesRequired}
                                        onChange={(e) => setSharesRequired(parseInt(e.target.value) || 1)}
                                        className="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 transition-all outline-none"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-semibold text-gray-900 mb-1.5">Widget Theme</label>
                                    <select
                                        value={theme}
                                        onChange={(e) => setTheme(e.target.value as 'light' | 'dark')}
                                        className="w-full px-4 py-2.5 bg-gray-50 border border-gray-200 rounded-xl focus:ring-2 focus:ring-indigo-500 transition-all outline-none appearance-none"
                                    >
                                        <option value="light">Light Theme</option>
                                        <option value="dark">Dark Theme</option>
                                    </select>
                                </div>

                                <div className="pt-4 border-t border-gray-100">
                                    <label className="flex items-start gap-3 cursor-pointer group">
                                        <div className="flex items-center h-6">
                                            <input
                                                type="checkbox"
                                                id="removeBranding"
                                                checked={hideBranding}
                                                onChange={handleBrandingToggle}
                                                className="w-5 h-5 border-2 border-gray-300 rounded text-indigo-600 focus:ring-indigo-500 transition-colors"
                                            />
                                        </div>
                                        <div className="flex flex-col">
                                            <span className="text-sm font-semibold text-gray-900 flex items-center gap-2">
                                                Remove Branding
                                                {!hasPro && <span className="bg-amber-100 text-amber-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                                            </span>
                                            <span className="text-xs text-gray-500 mt-0.5">Hide the "Powered by OHC" footer.</span>
                                        </div>
                                    </label>
                                </div>
                            </div>
                        </div>

                        {/* Embed Code Snippet */}
                        <div className="bg-gray-900 rounded-[24px] shadow-xl p-6 relative overflow-hidden">
                            <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-indigo-500 to-purple-500"></div>
                            <h2 className="text-lg font-bold font-outfit text-white mb-3">Embed Code</h2>
                            <p className="text-xs text-gray-400 mb-4">Paste this HTML snippet into your website builder (e.g., WordPress Custom HTML block, Wix HTML embed) where you want the widget to appear.</p>

                            <div className="relative group">
                                <pre className="w-full h-32 p-4 bg-black/50 border border-gray-800 rounded-xl font-mono text-xs text-gray-300 overflow-auto shadow-inner leading-relaxed">
                                    <code>{getEmbedCode()}</code>
                                </pre>
                                <button
                                    onClick={handleCopy}
                                    className="absolute top-3 right-3 p-2 bg-gray-800 hover:bg-gray-700 text-white rounded-lg transition-all flex items-center gap-2 text-xs font-medium border border-gray-700"
                                >
                                    {copied ? 'Copied!' : 'Copy Code'}
                                </button>
                            </div>
                        </div>
                    </div>

                    {/* Right Column: Live Preview */}
                    <div className="w-full lg:w-2/3 flex flex-col">
                        <div className="flex-1 bg-white rounded-[24px] border border-gray-200 shadow-sm overflow-hidden flex flex-col">
                            <div className="p-4 border-b border-gray-100 bg-gray-50 flex items-center justify-between">
                                <div className="flex items-center gap-2">
                                    <div className="flex gap-1.5">
                                        <div className="w-3 h-3 rounded-full bg-red-400"></div>
                                        <div className="w-3 h-3 rounded-full bg-amber-400"></div>
                                        <div className="w-3 h-3 rounded-full bg-green-400"></div>
                                    </div>
                                    <span className="ml-2 text-xs font-semibold text-gray-500 uppercase tracking-widest">Live Preview</span>
                                </div>
                            </div>

                            <div className="flex-1 p-8 bg-gray-100 flex items-center justify-center relative overflow-hidden" style={{ backgroundImage: 'radial-gradient(#d1d5db 1px, transparent 1px)', backgroundSize: '24px 24px' }}>
                                {/* Simulated Storefront Container */}
                                <div className="w-full max-w-[420px] bg-white rounded-2xl shadow-xl overflow-hidden border border-gray-200 flex flex-col h-[500px]">
                                    <div className="p-4 border-b border-gray-100 flex justify-between items-center bg-white">
                                        <div className="font-bold text-gray-800">Your Store</div>
                                        <div className="text-xs bg-gray-100 px-2 py-1 rounded text-gray-600">Cart: ${minSpend}</div>
                                    </div>

                                    <div className="p-6 flex-1 flex flex-col bg-gray-50">
                                        <div className="bg-white p-4 rounded-xl shadow-sm border border-gray-100 mb-6">
                                            <div className="h-4 bg-gray-200 rounded w-3/4 mb-3"></div>
                                            <div className="h-4 bg-gray-200 rounded w-1/2"></div>
                                        </div>

                                        {/* Widget Preview Render */}
                                        <div className={`mt-auto rounded-xl p-5 shadow-md border ${theme === 'dark' ? 'bg-gray-900 border-gray-800 text-white' : 'bg-white border-indigo-100 text-gray-900'}`}>
                                            <div className="flex items-center justify-between mb-3">
                                                <h3 className="font-bold flex items-center gap-2">
                                                    <span>🎁</span> Unlock Free Shipping
                                                </h3>
                                                <span className={`text-xs font-bold px-2 py-1 rounded-full ${theme === 'dark' ? 'bg-gray-800 text-gray-300' : 'bg-indigo-50 text-indigo-600'}`}>
                                                    0 / {sharesRequired}
                                                </span>
                                            </div>
                                            <p className={`text-sm mb-4 ${theme === 'dark' ? 'text-gray-400' : 'text-gray-600'}`}>
                                                Share with {sharesRequired} friends to get free shipping on your order of ${minSpend}+!
                                            </p>
                                            <button className={`w-full py-2.5 rounded-lg font-bold text-sm transition-all shadow-sm ${theme === 'dark' ? 'bg-white text-gray-900 hover:bg-gray-100' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}>
                                                Share Now
                                            </button>
                                        </div>

                                        {!hideBranding && (
                                            <div className="mt-3 text-center">
                                                <span className="text-[11px] text-gray-400 font-medium">⚡ Powered by OHC</span>
                                            </div>
                                        )}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </main>

            {/* Soft Paywall Modal */}
            {showPaywall && (
                <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm animate-in fade-in duration-200">
                    <div className="bg-white rounded-3xl p-8 max-w-md w-full shadow-2xl relative overflow-hidden">
                        <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-[100px] -z-10"></div>

                        <button
                            onClick={() => { setShowPaywall(false); setHideBranding(false); }}
                            className="absolute top-5 right-5 text-gray-400 hover:text-gray-600 bg-gray-50 rounded-full p-1.5 hover:bg-gray-100 transition-colors"
                        >
                            <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>

                        <div className="text-center">
                            <div className="w-16 h-16 bg-indigo-100 text-indigo-600 rounded-2xl flex items-center justify-center mx-auto mb-6 shadow-inner">
                                <svg className="w-8 h-8" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                                </svg>
                            </div>
                            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
                            <p className="text-gray-600 mb-8 text-sm leading-relaxed">
                                White-label your growth widgets. Upgrade to Pro to remove the <span className="font-semibold text-gray-900">"Powered by OHC"</span> branding completely and capture 100% of your brand value.
                            </p>

                            <div className="space-y-3">
                                <button
                                    onClick={() => window.location.href = '/pricing'}
                                    className="w-full py-3.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl font-bold transition-all shadow-md"
                                >
                                    Upgrade Now
                                </button>
                                <button
                                    onClick={() => { setShowPaywall(false); setHideBranding(false); }}
                                    className="w-full py-3.5 bg-white border border-gray-200 hover:bg-gray-50 text-gray-700 rounded-xl font-semibold transition-colors"
                                >
                                    Keep Branding
                                </button>
                            </div>
                        </div>
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
