"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function InvoiceGeneratorPage() {
  const [clientName, setClientName] = useState('');
  const [projectDetails, setProjectDetails] = useState('');
  const [amount, setAmount] = useState('');

  const [baseCurrency, setBaseCurrency] = useState('USD');
  const [transactionCurrency, setTransactionCurrency] = useState('USD');
  const [exchangeRate, setExchangeRate] = useState(1.0);

  const [shareLink, setShareLink] = useState('');
  const [copied, setCopied] = useState(false);
  const [tenantId, setTenantId] = useState('my-store');
  const [isSplitEnabled, setIsSplitEnabled] = useState(false);
  const [splitContact, setSplitContact] = useState('');
  const [splitPercentage, setSplitPercentage] = useState<number>(70);


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
      amount,
      baseCurrency,
      transactionCurrency,
      exchangeRate,
      splitPartnerId: isSplitEnabled ? splitContact : undefined,
      splitPercentage: isSplitEnabled ? splitPercentage : undefined
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
      <header className="px-4 md:px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40">
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
              <label htmlFor="client-name" className="block text-sm font-medium text-gray-700 mb-2">Client Name</label>
              <input
                id="client-name"
                type="text"
                value={clientName}
                onChange={(e) => setClientName(e.target.value)}
                placeholder="e.g. Acme Corp"
                className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>

            <div>
              <label htmlFor="project-details" className="block text-sm font-medium text-gray-700 mb-2">Project Details</label>
              <textarea
                id="project-details"
                value={projectDetails}
                onChange={(e) => setProjectDetails(e.target.value)}
                placeholder="e.g. Website Redesign and SEO Optimization"
                rows={4}
                className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>


            <div className="grid grid-cols-2 gap-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Base Currency</label>
                <input
                  type="text"
                  value={baseCurrency}
                  onChange={(e) => setBaseCurrency(e.target.value)}
                  placeholder="e.g. USD"
                  className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">Transaction Currency</label>
                <input
                  type="text"
                  value={transactionCurrency}
                  onChange={(e) => setTransactionCurrency(e.target.value)}
                  placeholder="e.g. EUR"
                  className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
                />
              </div>
            </div>

            <div>
              <label htmlFor="amount" className="block text-sm font-medium text-gray-700 mb-2">Amount ($)</label>
              <input
                id="amount"
                type="number"
                value={amount}
                onChange={(e) => setAmount(e.target.value)}
                placeholder="e.g. 1500.00"
                className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500"
              />
            </div>



            <div className="border-t border-gray-100 pt-6">
              <label className="flex items-center justify-between cursor-pointer">
                  <div className="text-gray-900 font-bold text-sm">
                      Split this payment
                  </div>
                  <div className="relative">
                      <input type="checkbox" className="sr-only" checked={isSplitEnabled} onChange={(e) => setIsSplitEnabled(e.target.checked)} />
                      <div className={`block w-10 h-6 rounded-full transition-colors ${isSplitEnabled ? 'bg-indigo-500' : 'bg-gray-300'}`}></div>
                      <div className={`dot absolute left-1 top-1 bg-white w-4 h-4 rounded-full transition-transform ${isSplitEnabled ? 'transform translate-x-4' : ''}`}></div>
                  </div>
              </label>

              {isSplitEnabled && (
                  <div className="mt-4 p-4 bg-gray-50 border border-gray-200 rounded-xl animate-fade-in">
                      <div className="mb-4">
                          <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Who gets a cut?</label>
                          <input
                              type="text"
                              placeholder="e.g. Sarah (Artist)"
                              value={splitContact}
                              onChange={(e) => setSplitContact(e.target.value)}
                              className="w-full bg-white border border-gray-200 rounded-lg px-3 py-2 text-gray-900 font-medium focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
                          />
                      </div>
                      <div className="mb-4">
                          <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">Their Percentage: {splitPercentage}%</label>
                          <input
                              type="range"
                              min="1"
                              max="99"
                              value={splitPercentage}
                              onChange={(e) => setSplitPercentage(parseInt(e.target.value))}
                              className="w-full accent-indigo-600"
                          />
                      </div>

                      <div className="bg-indigo-50 p-3 rounded-lg border border-indigo-100">
                         <p className="text-xs text-indigo-800 font-medium leading-relaxed">
                             If this pays for ${amount || '0'}, {splitContact || 'your partner'} gets ${((parseFloat(amount || '0') * splitPercentage) / 100).toFixed(2)}, you get ${((parseFloat(amount || '0') * (100 - splitPercentage)) / 100).toFixed(2)}.
                         </p>
                      </div>
                  </div>
              )}
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

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .glassmorphism {
            background: rgba(255, 255, 255, 0.65);
            backdrop-filter: blur(30px) saturate(210%);
            -webkit-backdrop-filter: blur(30px) saturate(210%);
            border: 1px solid rgba(255, 255, 255, 0.4);
        }
        @media (prefers-color-scheme: dark) {
            .glassmorphism {
                background: rgba(22, 22, 26, 0.7);
                backdrop-filter: blur(30px) saturate(210%);
                -webkit-backdrop-filter: blur(30px) saturate(210%);
                border: 1px solid rgba(255, 255, 255, 0.1);
            }
        }
        @keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
        .animate-fade-in { animation: fadeIn 0.3s ease-out forwards; }
      `}} />
    </div>
  );
}
