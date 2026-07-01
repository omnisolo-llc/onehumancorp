"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function ProposalGeneratorPage() {
  const [clientName, setClientName] = useState('');
  const [projectScope, setProjectScope] = useState('');
  const [amount, setAmount] = useState('');
  const [timeline, setTimeline] = useState('');
  const [shareLink, setShareLink] = useState('');
  const [copied, setCopied] = useState(false);
  const [tenantId, setTenantId] = useState('my-store');

  useEffect(() => {
    const tenant = localStorage.getItem('tenant') || 'my-store';
    setTenantId(tenant);
  }, []);

  const [isGenerating, setIsGenerating] = useState(false);

  const generateLink = async () => {
    if (!clientName || !projectScope || !amount || !timeline) {
      alert('Please fill out all fields.');
      return;
    }

    setIsGenerating(true);
    try {
      const data = {
        tenant: tenantId,
        clientName,
        projectScope,
        amount,
        timeline,
      };

      const res = await fetch('/api/v1/proposals', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': tenantId,
        },
        body: JSON.stringify(data),
      });

      if (!res.ok) {
        throw new Error('Failed to create proposal');
      }

      const result = await res.json();
      const url = `${window.location.origin}/proposal-generator/view?id=${result.id}`;
      setShareLink(url);
    } catch (e) {
      console.error(e);
      alert('Error creating proposal. Please try again.');
    } finally {
      setIsGenerating(false);
    }
  };

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(shareLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy text: ', err);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Proposal Generator 📝</h1>
         <div className="flex items-center gap-3">
             <Link href="/dashboard" className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </Link>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>

      <main className="p-6 md:p-10 flex-1 max-w-4xl mx-auto w-full">
        <div className="mb-10 text-center">
          <h2 className="text-3xl font-bold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Create Professional Proposal</h2>
          <p className="text-gray-600">Generate a beautiful, shareable proposal link for your client. When they view it, they can approve the project directly.</p>
        </div>

        <div className="glassmorphism glass-card p-6 md:p-8">
          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Client or Company Name</label>
              <input
                type="text"
                value={clientName}
                onChange={(e) => setClientName(e.target.value)}
                placeholder="e.g. Acme Corp"
                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
              />
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Project Scope / Details</label>
              <textarea
                value={projectScope}
                onChange={(e) => setProjectScope(e.target.value)}
                placeholder="e.g. Website Redesign, SEO Optimization, and Content Strategy"
                rows={4}
                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
              />
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">Total Amount ($)</label>
                  <input
                    type="number"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                    placeholder="e.g. 2500.00"
                    className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-2">Estimated Timeline</label>
                  <input
                    type="text"
                    value={timeline}
                    onChange={(e) => setTimeline(e.target.value)}
                    placeholder="e.g. 4-6 Weeks"
                    className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all"
                  />
                </div>
            </div>

            <div className="pt-4">
              <button
                onClick={generateLink}
                disabled={isGenerating}
                className="w-full py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md transition-all text-lg disabled:bg-indigo-400"
              >
                {isGenerating ? 'Generating...' : 'Generate Shareable Proposal'}
              </button>
            </div>
          </div>
        </div>

        {shareLink && (
          <div className="mt-8 bg-green-50 rounded-2xl p-6 border border-green-100 animate-fade-in-up text-center">
            <h3 className="text-xl font-bold text-green-800 mb-2">Your Proposal is Ready!</h3>
            <p className="text-green-700 mb-6">Share this secure link with your client.</p>

            <div className="flex flex-col md:flex-row gap-4 items-center justify-center">
              <input
                type="text"
                readOnly
                value={shareLink}
                className="w-full md:w-2/3 px-4 py-3 rounded-xl border border-green-200 bg-white focus:outline-none text-gray-700"
              />
              <button
                onClick={handleCopy}
                className="w-full md:w-auto px-6 py-3 bg-green-600 hover:bg-green-700 text-white font-bold rounded-xl shadow-sm transition-all"
              >
                {copied ? 'Copied!' : 'Copy Link'}
              </button>
            </div>

            <div className="mt-6">
              <Link href={shareLink} target="_blank" className="text-indigo-600 hover:text-indigo-800 font-medium underline">
                Preview Proposal
              </Link>
            </div>
          </div>
        )}
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @keyframes fade-in-up {
          0% { opacity: 0; transform: translateY(10px); }
          100% { opacity: 1; transform: translateY(0); }
        }
        .animate-fade-in-up { animation: fade-in-up 0.4s ease-out forwards; }
      `}} />
    </div>
  );
}