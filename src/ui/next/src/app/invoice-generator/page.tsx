"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function InvoiceGeneratorPage() {
  const [clientName, setClientName] = useState('');
  const [projectDetails, setProjectDetails] = useState('');
  const [amount, setAmount] = useState('');
  const [shareLink, setShareLink] = useState('');
  const [copied, setCopied] = useState(false);
  const [tenantId, setTenantId] = useState('my-store');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showPaywall, setShowPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);

  useEffect(() => {
    const tenant = localStorage.getItem('tenant') || 'my-store';
    setTenantId(tenant);
    if (typeof window !== 'undefined') {
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleRemoveBrandingToggle = () => {
    if (!hasPro) {
      setShowPaywall(true);
    } else {
      setRemoveBranding(!removeBranding);
    }
  };

  const claimTrialExtension = () => {
    const tenant = typeof window !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    const referralUrl = typeof window !== 'undefined' ? `${window.location.origin}/onboarding?ref=${tenant}` : '';
    if (typeof window !== 'undefined') {
      window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ' + referralUrl)}`, '_blank');
      localStorage.setItem('has_pro', 'true');
    }
    setHasPro(true);
    setShowPaywall(false);
    setRemoveBranding(true);
  };

  const generateLink = () => {
    if (!clientName || !projectDetails || !amount) {
      alert('Please fill out all fields.');
      return;
    }

    const data = {
      tenant: tenantId,
      clientName,
      projectDetails,
      amount,
      removeBranding
    };

    // Safely encode unicode string to base64url for URLs
    const utf8Encoded = encodeURIComponent(JSON.stringify(data));
    const base64Str = btoa(unescape(utf8Encoded));
    const base64UrlStr = base64Str.replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');

    const url = `${window.location.origin}/invoice-generator/view?data=${base64UrlStr}`;
    setShareLink(url);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(shareLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-4 md:px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-xl md:text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Invoice Generator</h1>
        <Link href="/dashboard" className="px-3 py-1.5 md:px-4 md:py-2 bg-gray-200 rounded-md text-xs md:text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </Link>
      </header>

      <main className="p-4 md:p-8 flex-1 w-full max-w-4xl mx-auto flex flex-col gap-8">
        <section className="glassmorphism p-8 md:p-10 border border-white/40 dark:border-white/10 relative" style={{ borderRadius: '16px' }}>
          <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-6">Create Professional Invoice</h2>
          <p className="text-gray-600 mb-8 text-sm leading-relaxed">
            Generate an invoice with a viral loop built-in. Share the link with your client, and they'll see a professional invoice powered by OHC.
          </p>

          <div className="flex flex-col gap-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Client Name</label>
              <input
                type="text"
                value={clientName}
                onChange={(e) => setClientName(e.target.value)}
                placeholder="e.g. Acme Corp"
                className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Project Details</label>
              <textarea
                value={projectDetails}
                onChange={(e) => setProjectDetails(e.target.value)}
                placeholder="e.g. Website Redesign and SEO Optimization"
                rows={4}
                className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Amount ($)</label>
              <input
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="e.g. 1500.00"
                className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div className="flex items-center justify-between p-4 bg-gray-50 rounded-xl border border-gray-100">
              <div>
                <label className="text-sm font-bold text-gray-900 block">White-label Invoice</label>
                <span className="text-xs text-gray-500">Remove OHC branding from the customer view</span>
              </div>
              <label className="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" className="sr-only peer" checked={removeBranding} onChange={handleRemoveBrandingToggle} />
                <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-indigo-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-indigo-600"></div>
              </label>
            </div>

            <button
              onClick={generateLink}
              className="mt-4 w-full md:w-auto px-8 py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all self-start text-sm flex items-center justify-center gap-2"
            >
              Generate Shareable Invoice
            </button>
          </div>

          {shareLink && (
            <div className="mt-8 p-6 bg-indigo-50 rounded-xl border border-indigo-100 animate-fade-in">
              <h3 className="text-lg font-bold font-outfit text-indigo-900 mb-4">Your Invoice is Ready!</h3>
              <div className="flex flex-col md:flex-row gap-4">
                <input
                  type="text"
                  readOnly
                  value={shareLink}
                  className="flex-1 bg-white border border-indigo-200 rounded-lg px-4 py-3 text-sm text-gray-600"
                />
                <button
                  onClick={handleCopy}
                  className="px-6 py-3 bg-black hover:bg-gray-800 text-white font-bold rounded-lg transition-colors text-sm whitespace-nowrap"
                >
                  {copied ? 'Copied!' : 'Copy Link'}
                </button>
                <Link
                  href={shareLink}
                  target="_blank"
                  className="px-6 py-3 bg-white border border-gray-300 hover:bg-gray-50 text-gray-800 font-bold rounded-lg transition-colors text-sm whitespace-nowrap text-center"
                >
                  Preview Invoice
                </Link>
              </div>
            </div>
          )}
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showPaywall && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4">
          <div className="absolute inset-0 bg-gray-900/40 backdrop-blur-sm" onClick={() => setShowPaywall(false)}></div>
          <div className="relative bg-white rounded-2xl shadow-2xl max-w-md w-full p-6 animate-fade-in overflow-hidden border border-gray-100">
            <div className="absolute top-0 right-0 p-4">
              <button onClick={() => setShowPaywall(false)} className="text-gray-400 hover:text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-full p-2 transition-colors">
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12"></path></svg>
              </button>
            </div>

            <div className="w-12 h-12 rounded-xl bg-indigo-100 text-indigo-600 flex items-center justify-center mb-4">
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
            </div>

            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Upgrade to Pro</h2>
            <p className="text-gray-600 text-sm mb-6">
              White-label invoices are a Pro feature. Upgrade to remove OHC branding and look even more professional to your clients.
            </p>

            <div className="space-y-3">
              <button className="w-full py-3 bg-black hover:bg-gray-800 text-white font-bold rounded-xl text-sm transition-colors shadow-lg">
                View Pricing
              </button>
              <div className="relative flex items-center py-2">
                <div className="flex-grow border-t border-gray-200"></div>
                <span className="flex-shrink-0 mx-4 text-gray-400 text-xs font-medium uppercase">Or</span>
                <div className="flex-grow border-t border-gray-200"></div>
              </div>
              <button
                onClick={claimTrialExtension}
                className="w-full py-3 bg-gradient-to-r from-indigo-500 to-purple-600 hover:from-indigo-600 hover:to-purple-700 text-white font-bold rounded-xl text-sm transition-all shadow-md flex items-center justify-center gap-2 hover:-translate-y-0.5"
              >
                <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                Share to get 7 Days Pro
              </button>
            </div>
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
        }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fadeIn 0.3s ease-out forwards; }
      `}} />
    </div>
  );
}
