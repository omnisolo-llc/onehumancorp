"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';

export default function EmbedBuilderPage() {
    const [tenantId, setTenantId] = useState('');
    const [widgetType, setWidgetType] = useState('intake');
    const [theme, setTheme] = useState('light');
    const [embedCode, setEmbedCode] = useState('');
    const [copied, setCopied] = useState(false);
    const [showPaywall, setShowPaywall] = useState(false);
    const [hideBranding, setHideBranding] = useState(false);

    useEffect(() => {
        if (typeof window !== 'undefined') {
            const activeTenant = localStorage.getItem('ohc_active_tenant_id') || localStorage.getItem('tenant') || 'my-store';
            setTenantId(activeTenant);
        }
    }, []);

    useEffect(() => {
        if (!tenantId) return;

        const baseUrl = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
        let embedUrl = `${baseUrl}/embed/widget?tenant_id=${encodeURIComponent(tenantId)}&type=${encodeURIComponent(widgetType)}&theme=${encodeURIComponent(theme)}`;

        if (hideBranding) {
            embedUrl += `&hideBranding=true`;
        }

        const iframeCode = `<iframe src="${embedUrl}" width="100%" height="500" frameborder="0" style="border-radius: 16px; border: 1px solid #e5e7eb; box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); background: transparent;" title="OHC ${widgetType === 'intake' ? 'Intake' : widgetType === 'booking' ? 'Booking' : 'Quote'} Widget"></iframe>`;

        let fullCode = iframeCode;

        if (!hideBranding) {
            fullCode += `\n<div style="text-align: center; margin-top: 12px; font-family: sans-serif; font-size: 12px;">\n  <a href="${baseUrl}/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenantId)}" target="_blank" rel="noopener noreferrer" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a>\n</div>`;
        }

        setEmbedCode(fullCode);
    }, [tenantId, widgetType, theme, hideBranding]);

    const handleCopy = () => {
        navigator.clipboard.writeText(embedCode);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
    };

    const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.checked) {
            // Soft Paywall Logic
            setShowPaywall(true);
        } else {
            setHideBranding(false);
        }
    };

    const getPreviewThemeStyles = () => {
        if (theme === 'dark') {
            return {
                backgroundColor: '#111827',
                color: '#fff',
            };
        }
        return {
            backgroundColor: '#ffffff',
            color: '#111827',
        };
    };

    return (
        <AppShell title="Embed Builder">
            <div className="max-w-6xl mx-auto p-4 md:p-8 font-inter">
                <div className="mb-8 flex flex-col gap-2">
                    <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-blue-50 text-blue-700 text-sm font-semibold w-fit shadow-sm border border-blue-100">
                        <span>🚀 Growth Loop</span>
                    </div>
                    <h1 className="text-3xl md:text-4xl font-bold font-outfit text-gray-900 drop-shadow-sm">Interactive Embed Builder</h1>
                    <p className="text-gray-600 text-base md:text-lg">Generate a custom widget to embed on your external website. Capture leads, bookings, or quotes directly into OHC.</p>
                </div>

                <div className="flex flex-col lg:flex-row gap-8">
                    {/* Left Column: Configuration */}
                    <div className="w-full lg:w-1/3 flex flex-col gap-6">
                        <div className="glassmorphism p-6 rounded-[24px] border border-gray-200 shadow-xl bg-white/80">
                            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Configuration</h2>

                            <div className="mb-6">
                                <label className="block text-sm font-semibold text-gray-700 mb-2">Workspace ID</label>
                                <input
                                    type="text"
                                    value={tenantId}
                                    onChange={(e) => setTenantId(e.target.value)}
                                    className="w-full px-4 py-3 bg-gray-50 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-[#0066FF] focus:border-transparent transition-all shadow-inner"
                                    placeholder="your-workspace-id"
                                />
                                <p className="text-xs text-gray-500 mt-2">Connects widget submissions to your OHC workspace.</p>
                            </div>

                            <div className="mb-6">
                                <label className="block text-sm font-semibold text-gray-700 mb-2">Widget Type</label>
                                <div className="flex bg-gray-100 p-1 rounded-xl">
                                    {['intake', 'booking', 'quote'].map((type) => (
                                        <button
                                            key={type}
                                            onClick={() => setWidgetType(type)}
                                            className={`flex-1 py-2 text-sm font-medium rounded-lg transition-all capitalize ${
                                                widgetType === type
                                                    ? 'bg-white text-gray-900 shadow-sm'
                                                    : 'text-gray-500 hover:text-gray-700 hover:bg-gray-200/50'
                                            }`}
                                        >
                                            {type}
                                        </button>
                                    ))}
                                </div>
                            </div>

                            <div className="mb-6">
                                <label className="block text-sm font-semibold text-gray-700 mb-2">Theme</label>
                                <div className="flex bg-gray-100 p-1 rounded-xl">
                                    <button
                                        onClick={() => setTheme('light')}
                                        className={`flex-1 py-2 text-sm font-medium rounded-lg transition-all ${
                                            theme === 'light'
                                                ? 'bg-white text-gray-900 shadow-sm'
                                                : 'text-gray-500 hover:text-gray-700 hover:bg-gray-200/50'
                                        }`}
                                    >
                                        Light
                                    </button>
                                    <button
                                        onClick={() => setTheme('dark')}
                                        className={`flex-1 py-2 text-sm font-medium rounded-lg transition-all ${
                                            theme === 'dark'
                                                ? 'bg-gray-800 text-white shadow-sm'
                                                : 'text-gray-500 hover:text-gray-700 hover:bg-gray-200/50'
                                        }`}
                                    >
                                        Dark
                                    </button>
                                </div>
                            </div>

                            <div className="mb-4">
                                <label className="flex items-center gap-3 p-3 border border-gray-200 rounded-xl hover:bg-gray-50 cursor-pointer transition-colors">
                                    <div className="relative flex items-center">
                                        <input
                                            type="checkbox"
                                            checked={hideBranding}
                                            onChange={handleBrandingToggle}
                                            className="w-5 h-5 border-2 border-gray-300 rounded text-[#0071E3] focus:ring-[#0066FF] transition-colors"
                                        />
                                    </div>
                                    <div className="flex flex-col">
                                        <span className="text-sm font-semibold text-gray-900">Remove Branding</span>
                                        <span className="text-xs text-gray-500">Hide the "Powered by OHC" footer.</span>
                                    </div>
                                </label>
                            </div>
                        </div>

                        {/* Embed Code Snippet Area */}
                        <div className="glassmorphism p-6 rounded-[24px] border border-gray-200 shadow-xl bg-white/80">
                            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Embed Code</h2>
                            <div className="relative group">
                                <pre className="w-full h-40 p-4 bg-gray-900 border border-gray-700 rounded-xl font-mono text-xs text-gray-300 overflow-x-auto overflow-y-auto shadow-inner leading-relaxed">
                                    <code>{embedCode}</code>
                                </pre>
                                <button
                                    onClick={handleCopy}
                                    className="absolute top-3 right-3 p-2 bg-white/10 hover:bg-white/20 text-white rounded-lg backdrop-blur-[30px] saturate-[210%] transition-all opacity-0 group-hover:opacity-100 flex items-center gap-2 text-xs font-medium border border-white/10 shadow-lg"
                                >
                                    {copied ? (
                                        <>
                                            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                                            Copied!
                                        </>
                                    ) : (
                                        <>
                                            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
                                            Copy Code
                                        </>
                                    )}
                                </button>
                            </div>
                            <p className="text-xs text-gray-500 mt-4 text-center">Paste this HTML directly into your site builder (e.g., WordPress Custom HTML block, Wix HTML embed).</p>
                        </div>
                    </div>

                    {/* Right Column: Live Preview */}
                    <div className="w-full lg:w-2/3 flex flex-col">
                        <div className="flex-1 glassmorphism rounded-[24px] border border-gray-200 shadow-xl bg-white/80 overflow-hidden flex flex-col relative min-h-[600px]">
                            <div className="absolute top-0 left-0 w-full h-2 bg-gradient-to-r from-blue-400 via-indigo-500 to-purple-500 z-10"></div>
                            <div className="p-4 border-b border-gray-100 bg-gray-50/50 flex justify-between items-center z-10 relative">
                                <h3 className="text-sm font-bold text-gray-600 uppercase tracking-widest flex items-center gap-2">
                                    <span className="relative flex h-3 w-3">
                                      <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-blue-400 opacity-75"></span>
                                      <span className="relative inline-flex rounded-full h-3 w-3 bg-[#0066FF]"></span>
                                    </span>
                                    Live Preview
                                </h3>
                            </div>

                            <div className="flex-1 p-8 md:p-12 bg-gray-100/50 flex items-center justify-center relative overflow-hidden" style={{ backgroundImage: 'radial-gradient(#d1d5db 1px, transparent 1px)', backgroundSize: '24px 24px' }}>
                                {/* Widget Container */}
                                <div
                                    className="w-full max-w-[400px] h-[500px] shadow-2xl relative flex flex-col overflow-hidden transition-all duration-300"
                                    style={{ ...getPreviewThemeStyles(), border: theme === 'dark' ? '1px solid #374151' : '1px solid #e5e7eb' }}
                                >
                                    <iframe
                                        src={`/embed/widget?tenant_id=${encodeURIComponent(tenantId)}&type=${encodeURIComponent(widgetType)}&theme=${encodeURIComponent(theme)}${hideBranding ? '&hideBranding=true' : ''}`}
                                        className="w-full flex-1 border-none bg-transparent"
                                        title="Preview"
                                    />

                                    {!hideBranding && (
                                        <div className={`py-3 text-center text-xs font-semibold border-t ${theme === 'dark' ? 'border-gray-800 bg-gray-900/90' : 'border-gray-100 bg-white/90'} backdrop-blur-[30px] saturate-[210%] z-20`}>
                                            <a
                                                href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenantId)}`}
                                                target="_blank"
                                                rel="noopener noreferrer"
                                                className={`hover:underline transition-colors ${theme === 'dark' ? 'text-gray-400 hover:text-white' : 'text-gray-500 hover:text-gray-900'}`}
                                            >
                                                ⚡ Powered by OHC
                                            </a>
                                        </div>
                                    )}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            {/* Soft Paywall Modal */}
            {showPaywall && (
                <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-[30px] saturate-[210%] animate-in fade-in duration-200">
                    <div className="bg-white rounded-3xl p-8 max-w-md w-full shadow-2xl relative overflow-hidden border border-gray-100">
                        <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-[100px] -z-10"></div>

                        <button
                            onClick={() => {
                                setShowPaywall(false);
                                setHideBranding(false);
                            }}
                            className="absolute top-5 right-5 text-gray-400 hover:text-gray-600 transition-colors bg-white rounded-full p-1 hover:bg-gray-100"
                        >
                            <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>

                        <div className="text-center">
                            <div className="w-16 h-16 bg-blue-100 rounded-2xl flex items-center justify-center mx-auto mb-6 shadow-inner border border-blue-200">
                                <span className="text-3xl">✨</span>
                            </div>
                            <h2 className="text-2xl font-bold text-gray-900 mb-3 font-outfit">Upgrade to Pro</h2>
                            <p className="text-gray-600 mb-8 text-base">
                                White-label your embedded widgets. Upgrade to Pro to remove the <span className="font-semibold text-gray-900">"Powered by OHC"</span> branding completely.
                            </p>

                            <div className="space-y-3">
                                <button
                                    onClick={() => {
                                        window.location.href = '/pricing';
                                    }}
                                    className="w-full py-3.5 bg-[#0071E3] hover:bg-blue-700 text-white rounded-xl font-bold transition-all shadow-md hover:shadow-lg flex items-center justify-center gap-2"
                                >
                                    Upgrade Now
                                </button>
                                <button
                                    onClick={() => {
                                        setShowPaywall(false);
                                        setHideBranding(false);
                                    }}
                                    className="w-full py-3.5 bg-white border border-gray-200 hover:bg-gray-50 text-gray-700 rounded-xl font-semibold transition-colors"
                                >
                                    Keep Branding
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </AppShell>
    );
}
