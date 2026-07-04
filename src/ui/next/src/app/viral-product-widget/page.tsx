"use client";

import React, { useState, useEffect } from 'react';
import { AppShell } from '../components/AppShell';

export default function ViralProductWidgetPage() {
    const [tenant, setTenant] = useState('my-store');
    const [theme, setTheme] = useState<'light' | 'dark'>('light');
    const [productName, setProductName] = useState('Premium Artisan Coffee');
    const [price, setPrice] = useState('$24.99');
    const [description, setDescription] = useState('A rich, dark roast sourced from the best beans.');
    const [imageUrl, setImageUrl] = useState('https://images.unsplash.com/photo-1559525839-b184a4d698c7?ixlib=rb-4.0.3&auto=format&fit=crop&w=800&q=80');
    const [copied, setCopied] = useState(false);
    const [hasPro, setHasPro] = useState(false);
    const [showPaywall, setShowPaywall] = useState(false);
    const [hideBranding, setHideBranding] = useState(false);
    const [isClient, setIsClient] = useState(false);

    useEffect(() => {
        setIsClient(true);
        if (typeof localStorage !== 'undefined') {
            const storedTenant = localStorage.getItem('tenant') || 'my-store';
            setTenant(storedTenant);
            const proStatus = localStorage.getItem('has_pro') === 'true';
            setHasPro(proStatus);
        }
        document.title = "Viral Product Widget | OHC";
    }, []);

    const handleBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (!hasPro) {
            e.preventDefault();
            setShowPaywall(true);
            return;
        }
        setHideBranding(e.target.checked);
    };

    const embedUrl = `/api/v1/growth/viral-product-widget/embed?tenant=${encodeURIComponent(tenant)}&productName=${encodeURIComponent(productName)}&price=${encodeURIComponent(price)}&description=${encodeURIComponent(description)}&imageUrl=${encodeURIComponent(imageUrl)}&theme=${theme}&branding=${!hideBranding}`;

    // Construct the HTML code block for the user to copy
    const absoluteEmbedUrl = `https://ohc.app/api/v1/growth/viral-product-widget/embed?tenant=${encodeURIComponent(tenant)}&productName=${encodeURIComponent(productName)}&price=${encodeURIComponent(price)}&description=${encodeURIComponent(description)}&imageUrl=${encodeURIComponent(imageUrl)}&theme=${theme}&branding=${!hideBranding}`;
    const embedCode = `<iframe src="${absoluteEmbedUrl}" width="100%" height="450" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>`;

    const handleCopy = () => {
        navigator.clipboard.writeText(embedCode);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
    };

    if (!isClient) return null;

    return (
        <AppShell title="Viral Product Widget" subtitle="Embed products to drive sales and traffic.">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 animate-in fade-in duration-500 font-inter">

                <div className="mb-8">
                    <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Viral Product Widget Builder</h1>
                    <p className="mt-2 text-gray-600 max-w-2xl">Create an embeddable product card for your blog, partner sites, or Link-in-Bio to drive seamless checkouts and viral referrals.</p>
                </div>

                <div className="flex flex-col lg:flex-row gap-8">
                    {/* Left Column: Configuration */}
                    <div className="w-full lg:w-1/3 flex flex-col gap-6">
                        <div className="app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-xl rounded-2xl p-6">
                            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6 flex items-center gap-2">
                                <span className="p-2 bg-indigo-100 text-indigo-600 rounded-lg">⚙️</span>
                                Configuration
                            </h2>

                            <div className="space-y-5">
                                <div>
                                    <label className="block text-sm font-semibold text-gray-900 mb-1.5">Product Name</label>
                                    <input
                                        type="text"
                                        value={productName}
                                        onChange={(e) => setProductName(e.target.value)}
                                        className="w-full px-4 py-2.5 bg-white/60 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-transparent transition-all"
                                        placeholder="E.g., Premium Artisan Coffee"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-semibold text-gray-900 mb-1.5">Price</label>
                                    <input
                                        type="text"
                                        value={price}
                                        onChange={(e) => setPrice(e.target.value)}
                                        className="w-full px-4 py-2.5 bg-white/60 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-transparent transition-all"
                                        placeholder="$24.99"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-semibold text-gray-900 mb-1.5">Description</label>
                                    <textarea
                                        value={description}
                                        onChange={(e) => setDescription(e.target.value)}
                                        className="w-full px-4 py-2.5 bg-white/60 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-transparent transition-all resize-none h-24"
                                        placeholder="Short engaging description..."
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-semibold text-gray-900 mb-1.5">Image URL</label>
                                    <input
                                        type="text"
                                        value={imageUrl}
                                        onChange={(e) => setImageUrl(e.target.value)}
                                        className="w-full px-4 py-2.5 bg-white/60 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-transparent transition-all"
                                        placeholder="https://example.com/image.jpg"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-semibold text-gray-900 mb-1.5">Theme</label>
                                    <select
                                        value={theme}
                                        onChange={(e) => setTheme(e.target.value as 'light' | 'dark')}
                                        className="w-full px-4 py-2.5 bg-white/60 border border-gray-200 rounded-xl focus:ring-2 focus:ring-[#0066FF] focus:border-transparent transition-all appearance-none"
                                    >
                                        <option value="light">Light</option>
                                        <option value="dark">Dark</option>
                                    </select>
                                </div>

                                <div className="pt-4 border-t border-gray-100/50">
                                    <label className="flex items-start gap-3 cursor-pointer group">
                                        <div className="flex items-center h-6">
                                            <input
                                                type="checkbox"
                                                id="removeBranding"
                                                checked={hideBranding}
                                                onChange={handleBrandingToggle}
                                                className="w-5 h-5 border-2 border-gray-300 rounded text-[#0071E3] focus:ring-[#0066FF] transition-colors"
                                            />
                                        </div>
                                        <div className="flex flex-col">
                                            <span className="text-sm font-semibold text-gray-900 flex items-center gap-2">
                                                Remove Branding
                                                {!hasPro && <span className="bg-gradient-to-r from-amber-200 to-yellow-400 text-yellow-900 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider shadow-sm">PRO</span>}
                                            </span>
                                            <span className="text-xs text-gray-500">Hide the "Powered by OHC" footer.</span>
                                        </div>
                                    </label>
                                </div>
                            </div>
                        </div>

                        {/* Embed Code Section */}
                        <div className="app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-xl rounded-2xl p-6">
                            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Embed Code</h2>
                            <div className="relative group">
                                <pre className="w-full h-32 p-4 bg-gray-900 border border-gray-700 rounded-xl font-mono text-xs text-gray-300 overflow-x-auto overflow-y-auto shadow-inner leading-relaxed">
                                    <code>{embedCode}</code>
                                </pre>
                                <button
                                    onClick={handleCopy}
                                    className="absolute top-3 right-3 p-2 bg-white/10 hover:bg-white/20 text-white rounded-lg backdrop-blur-[30px] saturate-[210%] transition-all opacity-0 group-hover:opacity-100 flex items-center gap-2 text-xs font-medium border border-white/10 shadow-lg"
                                >
                                    {copied ? 'Copied!' : 'Copy Code'}
                                </button>
                            </div>
                        </div>
                    </div>

                    {/* Right Column: Live Preview */}
                    <div className="w-full lg:w-2/3 flex flex-col">
                        <div className="flex-1 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-xl rounded-2xl overflow-hidden flex flex-col relative min-h-[600px]">
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
                                <div className="w-full max-w-[400px] h-[450px] shadow-2xl relative flex flex-col overflow-hidden transition-all duration-300 rounded-2xl bg-transparent">
                                    <iframe
                                        src={embedUrl}
                                        className="w-full h-full border-none bg-transparent"
                                        title="Preview"
                                    />
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
                            onClick={() => setShowPaywall(false)}
                            className="absolute top-5 right-5 text-gray-400 hover:text-gray-600 transition-colors bg-white rounded-full p-1 hover:bg-gray-100"
                            aria-label="Close"
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
                                White-label your embedded product widgets. Upgrade to Pro to remove the <span className="font-semibold text-gray-900">"Powered by OHC"</span> branding completely.
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
                                    onClick={() => setShowPaywall(false)}
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