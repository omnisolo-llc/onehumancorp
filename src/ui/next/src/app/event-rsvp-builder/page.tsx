"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function EventRSVPBuilderPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('demo-business');
  const [hasPro, setHasPro] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [isClient, setIsClient] = useState(false);

  // Form State
  const [eventTitle, setEventTitle] = useState('Summer Pop-up Shop');
  const [eventDate, setEventDate] = useState('Saturday, August 15th @ 12 PM');
  const [eventLocation, setEventLocation] = useState('Main Street Plaza');
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [hideBranding, setHideBranding] = useState(false);

  // Modal State
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    setIsClient(true);
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'demo-business';
      setTenant(storedTenant);
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const embedUrl = `/api/v1/growth/event-rsvp/embed?tenant=${encodeURIComponent(tenant)}&title=${encodeURIComponent(eventTitle)}&date=${encodeURIComponent(eventDate)}&location=${encodeURIComponent(eventLocation)}&theme=${theme}&branding=${!hideBranding}`;
  const absoluteEmbedUrl = `https://ohc.app${embedUrl}`;
  const embedCode = `<iframe src="${absoluteEmbedUrl}" width="100%" height="450" frameborder="0" scrolling="no" style="border:none; border-radius:16px; overflow:hidden;"></iframe>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return { backgroundColor: '#1f2937', color: '#f9fafb', borderColor: '#374151' };
    }
    return { backgroundColor: '#ffffff', color: '#111827', borderColor: '#e5e7eb' };
  };

  if (!isClient) return null;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 py-10 px-4 md:px-8">
      {/* Header */}
      <header className="max-w-6xl mx-auto w-full mb-8 flex items-center justify-between">
        <div>
            <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">Event RSVP Builder 🎉</h1>
            <p className="text-gray-600">Create an embeddable RSVP form for your next pop-up or webinar.</p>
        </div>
        <button
          onClick={() => router.push('/dashboard')}
          className="text-gray-600 hover:text-indigo-600 font-medium text-sm flex items-center gap-2 transition-colors bg-white/50 px-4 py-2 rounded-lg"
        >
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          Dashboard
        </button>
      </header>

      <main className="max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8">
        {/* Editor */}
        <div className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 glassmorphism border border-white/40">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-6">Event Details</h2>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Event Title</label>
                        <input
                            type="text"
                            value={eventTitle}
                            onChange={(e) => setEventTitle(e.target.value)}
                            className="w-full px-4 py-2 min-h-[44px] border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white/80"
                            placeholder="e.g. Summer Pop-up"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Date & Time</label>
                        <input
                            type="text"
                            value={eventDate}
                            onChange={(e) => setEventDate(e.target.value)}
                            className="w-full px-4 py-2 min-h-[44px] border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white/80"
                            placeholder="e.g. Aug 15 @ 12 PM"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Location</label>
                        <input
                            type="text"
                            value={eventLocation}
                            onChange={(e) => setEventLocation(e.target.value)}
                            className="w-full px-4 py-2 min-h-[44px] border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 bg-white/80"
                            placeholder="e.g. Main Street Plaza or Zoom Link"
                        />
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Theme</label>
                        <div className="flex gap-2 p-1 bg-gray-100 rounded-lg border border-gray-200 min-h-[44px]">
                            <button
                                onClick={() => setTheme('light')}
                                className={`flex-1 py-1 px-3 rounded-md text-sm font-medium transition-all ${theme === 'light' ? 'bg-white shadow-sm text-gray-900' : 'text-gray-500 hover:text-gray-700'}`}
                            >
                                Light
                            </button>
                            <button
                                onClick={() => setTheme('dark')}
                                className={`flex-1 py-1 px-3 rounded-md text-sm font-medium transition-all ${theme === 'dark' ? 'bg-gray-800 shadow-sm text-white' : 'text-gray-500 hover:text-gray-700'}`}
                            >
                                Dark
                            </button>
                        </div>
                    </div>
                </div>

                <div className="mt-6 pt-6 border-t border-gray-200">
                    <div className="flex items-center gap-2 mb-4">
                        <input
                            type="checkbox"
                            id="removeBranding"
                            checked={hideBranding}
                            onChange={(e) => {
                                if (!hasPro) {
                                    e.preventDefault();
                                    setShowPaywall(true);
                                } else {
                                    setHideBranding(e.target.checked);
                                }
                            }}
                            className="w-4 h-4 text-indigo-600 border-gray-300 rounded focus:ring-indigo-500"
                        />
                        <label htmlFor="removeBranding" className="text-sm font-medium text-gray-700 flex items-center gap-2 cursor-pointer">
                            Remove "Powered by OHC"
                            {!hasPro && <span className="bg-yellow-100 text-yellow-800 text-[10px] font-bold px-2 py-0.5 rounded uppercase tracking-wider">PRO</span>}
                        </label>
                    </div>

                    <button
                        onClick={() => setShowModal(true)}
                        className="w-full py-3 bg-indigo-600 text-white font-medium min-h-[44px] hover:bg-indigo-700 transition-colors rounded-xl shadow-sm"
                    >
                        Get Widget Code
                    </button>
                </div>
            </div>
        </div>

        {/* Live Preview */}
        <div className="w-full md:w-2/3">
            <div className="p-8 h-full flex flex-col items-center justify-center relative overflow-hidden bg-white/40 border border-white/50 rounded-[24px] shadow-inner">
                <div className="absolute top-4 left-4 text-xs font-semibold text-gray-400 uppercase tracking-wider">Live Preview</div>

                <div className="relative w-full max-w-sm shadow-2xl rounded-[16px] overflow-hidden" style={getThemeStyles()}>
                     <iframe
                        src={embedUrl}
                        title="Event RSVP Builder Preview"
                        width="100%"
                        height="450"
                        frameBorder="0"
                        scrolling="no"
                        style={{ border: 'none', backgroundColor: 'transparent' }}
                     />
                </div>

                <div className="mt-8 text-center max-w-md text-sm text-gray-500">
                    This preview shows exactly how the RSVP widget will look when embedded on your website.
                </div>
            </div>
        </div>
      </main>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
            <div className="bg-white p-8 max-w-xl w-full rounded-2xl shadow-2xl relative animate-in fade-in zoom-in-95 duration-200">
                <button
                    aria-label="Close modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed RSVP Widget</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website or Notion page.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-32 p-4 bg-gray-50 border border-gray-200 rounded-xl min-h-[44px] font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
                    />
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium min-h-[44px] transition-colors rounded-xl shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium min-h-[44px] rounded-xl transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}

      {/* Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[9999] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
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
              Make the Event RSVP Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 min-h-[44px] rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600"
            >
              Upgrade to Pro
            </button>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism {
          background: rgba(255, 255, 255, 0.65);
          backdrop-filter: blur(30px) saturate(210%);
          -webkit-backdrop-filter: blur(30px) saturate(210%);
          border-radius: 24px;
        }
      `}} />
    </div>
  );
}
