"use client";

import React, { useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";

export default function BookingWidgetBuilder() {
  const router = useRouter();
  const [tenant, setTenant] = useState("my-store");
  const [theme, setTheme] = useState<"light" | "dark">("light");
  const [removeBranding, setRemoveBranding] = useState(false);
  const [serviceName, setServiceName] = useState("Service Consultation");
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [previewStatus, setPreviewStatus] = useState("");

  const embedUrl = `https://ohc.app/api/v1/growth/booking/embed?tenant=${tenant}&theme=${theme}&service=${encodeURIComponent(serviceName)}`;
  const embedCode = `<iframe src="${embedUrl}" width="320" height="400" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `\n<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
        return {
            backgroundColor: '#111827',
            color: '#f9fafb',
            borderColor: '#374151'
        };
    }
    return {
        backgroundColor: '#ffffff',
        color: '#111827',
        borderColor: '#e5e7eb'
    };
  };

  return (
    <div className="min-h-screen bg-gray-50 font-inter text-gray-900 pb-20">
      {/* Top Nav */}
      <nav className="sticky top-0 z-40 bg-white/80 backdrop-blur-[30px] saturate-[210%] border-b border-gray-200 px-6 py-4 flex items-center justify-between">
        <div className="flex items-center gap-4">
            <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path></svg>
            </Link>
            <h1 className="text-xl font-bold font-outfit text-gray-900 flex items-center gap-2">
                Booking Widget
                <span className="bg-blue-100 text-blue-700 text-xs font-bold px-2 py-0.5 rounded uppercase tracking-wider">New Growth Loop</span>
            </h1>
        </div>
      </nav>

      <main className="max-w-6xl mx-auto px-6 py-8 flex flex-col md:flex-row gap-8">

        {/* Configuration Panel */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', boxShadow: '0 4px 24px rgba(0,0,0,0.04)' }}>
                <h2 className="text-lg font-bold font-outfit mb-6">Widget Settings</h2>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                    <div className="flex bg-gray-100 p-1 rounded-lg">
                        <button
                            aria-pressed={theme === 'light'}
                            onClick={() => setTheme('light')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'light' ? 'bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Light
                        </button>
                        <button
                            aria-pressed={theme === 'dark'}
                            onClick={() => setTheme('dark')}
                            className={`flex-1 py-2 text-sm font-medium min-h-[44px] min-w-[44px] transition-all ${theme === 'dark' ? 'bg-white backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                        >
                            Dark
                        </button>
                    </div>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Tenant ID</label>
                    <input
                        type="text"
                        value={tenant}
                        onChange={(e) => setTenant(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                        placeholder="e.g. my-store"
                    />
                    <p className="text-xs text-gray-500 mt-2">Used to link the widget to your store.</p>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Service Name</label>
                    <input
                        type="text"
                        value={serviceName}
                        onChange={(e) => setServiceName(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 min-h-[44px] min-w-[44px] focus:outline-none focus:ring-2 focus:ring-[#0066FF]"
                        placeholder="e.g. Service Consultation"
                    />
                </div>

                <div className="mb-6">
                    <label className="flex items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={(e) => setRemoveBranding(e.target.checked)}
                            className="w-4 h-4 text-[#0071E3] border-gray-300 rounded focus:ring-[#0066FF]"
                        />
                        <span className="text-sm font-medium text-gray-700">Remove "Powered by OHC" Badge (Pro)</span>
                    </label>
                </div>

                <button
                    onClick={() => setShowModal(true)}
                    className="w-full py-3 bg-[#0071E3] text-white font-medium min-h-[44px] min-w-[44px] hover:bg-blue-700 transition-colors shadow-sm"
                >
                    Get Widget
                </button>
            </div>

            <div className="p-6" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', boxShadow: '0 4px 24px rgba(0,0,0,0.04)' }}>
                <h3 className="text-md font-semibold font-outfit mb-2 flex items-center gap-2">
                    <span className="text-xl">📅</span> Why embed?
                </h3>
                <p className="text-sm text-gray-600 leading-relaxed">
                    Allow customers to book your services directly from your own website, partner blogs, or Notion pages.
                </p>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-2/3 flex flex-col items-center">
            <h2 className="text-xl font-semibold font-outfit self-start mb-4" style={{ color: '#1D1D1F' }}>Live Preview</h2>
            <div className="w-full p-8 h-full flex flex-col items-center justify-center relative overflow-hidden" style={{ background: 'linear-gradient(135deg, #f3f4f6 0%, #e5e7eb 100%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>

                <div className="relative z-10 w-[320px] h-[400px]" style={{ ...getThemeStyles(), borderRadius: '16px', boxShadow: '0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04)' }}>
                    {/* Mock Widget Content for Preview */}
                    <div className="w-full h-48 bg-gradient-to-br from-blue-500 to-cyan-500 rounded-t-[16px] relative flex items-center justify-center">
                        <span className="text-4xl text-white">📅</span>
                        <div className="absolute top-3 right-3 bg-white/20 backdrop-blur-[30px] saturate-[210%] border border-white/30 text-white text-xs font-bold px-3 py-1 rounded-full">
                            Book Now
                        </div>
                    </div>
                    <div className="p-5 flex flex-col h-[208px]">
                        <h4 className="font-bold text-lg font-outfit mb-1" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>{serviceName}</h4>
                        <p className="text-sm mb-4 line-clamp-2" style={{ color: theme === 'dark' ? '#d1d5db' : '#4b5563' }}>Schedule your appointment with us easily. Tell us what you need and we will get right back to you.</p>

                        <button
                            type="button"
                            onClick={() => {
                                setPreviewStatus('Preview redirected to booking flow.');
                                router.push('/booking');
                            }}
                            className="w-full mt-auto py-2.5 bg-[#0071E3] hover:bg-blue-700 text-white font-medium rounded-lg text-sm flex items-center justify-center gap-2 transition-colors"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                            Request a Service
                        </button>
                        {previewStatus && <p className="mt-2 text-xs font-semibold text-[#0071E3]" role="status">{previewStatus}</p>}
                    </div>
                </div>
                {!removeBranding && (
                    <div className="mt-2 text-center" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
                        <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}`} target="_blank" rel="noopener noreferrer" style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }}>
                            ⚡ Powered by OHC
                        </a>
                    </div>
                )}
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="app-card p-8 max-w-xl w-full shadow-2xl relative animate-in fade-in zoom-in-95 duration-200">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Booking Widget</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website, blog, or Notion page.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 min-h-[44px] min-w-[44px] font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] transition-all"
                    />
                    <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                         <button
                            onClick={handleCopy}
                            className="p-2 bg-white rounded-lg border shadow-sm text-gray-600 hover:text-[#0071E3] transition-colors"
                            title="Copy to clipboard"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2 2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                        </button>
                    </div>
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-[#0071E3] hover:bg-blue-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium min-h-[44px] min-w-[44px] transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}
    </div>
  );
}
