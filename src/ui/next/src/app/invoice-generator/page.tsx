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

  useEffect(() => {
    const tenant = localStorage.getItem('tenant') || 'my-store';
    setTenantId(tenant);
  }, []);

  const generateLink = () => {
    if (!clientName || !projectDetails || !amount) {
      alert('Please fill out all fields.');
      return;
    }

    const data = {
      tenant: tenantId,
      clientName,
      projectDetails,
      amount
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
      <header className="px-4 md:px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 glassmorphism/65 backdrop-blur-md border-white/40">
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
                className="w-full glassmorphism border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Project Details</label>
              <textarea
                value={projectDetails}
                onChange={(e) => setProjectDetails(e.target.value)}
                placeholder="e.g. Website Redesign and SEO Optimization"
                rows={4}
                className="w-full glassmorphism border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Amount ($)</label>
              <input
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="e.g. 1500.00"
                className="w-full glassmorphism border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
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
                  className="flex-1 glassmorphism border border-indigo-200 rounded-lg px-4 py-3 text-sm text-gray-600"
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
                  className="px-6 py-3 glassmorphism border border-gray-300 hover:bg-gray-50 text-gray-800 font-bold rounded-lg transition-colors text-sm whitespace-nowrap text-center"
                >
                  Preview Invoice
                </Link>
              </div>
            </div>
          )}
        </section>
      </main>

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
