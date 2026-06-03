"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function AffiliatesHubPage() {
  const router = useRouter();

  const [stats, setStats] = useState({ total_affiliates: 0, total_commission_cents: 0 });
  const [loading, setLoading] = useState(true);
  const [tenant, setTenant] = useState('my-store');

  const [newAffiliateEmail, setNewAffiliateEmail] = useState('');
  const [newAffiliateCommission, setNewAffiliateCommission] = useState(15);
  const [newAffiliateDiscount, setNewAffiliateDiscount] = useState(10);

  const [isGenerating, setIsGenerating] = useState(false);
  const [generatedLink, setGeneratedLink] = useState('');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      setTenant(localStorage.getItem('tenant') || 'my-store');
    }

    const fetchStats = async () => {
      try {
        const res = await fetch('/api/v1/growth/affiliate/stats', {
          headers: {
            'Authorization': `Bearer ${typeof localStorage !== 'undefined' ? localStorage.getItem('token') : ''}`
          }
        });
        if (res.ok) {
          const data = await res.json();
          setStats(data);
        }
      } catch (e) {
        console.error("Failed to fetch affiliate stats", e);
      } finally {
        setLoading(false);
      }
    };

    fetchStats();
  }, []);

  const handleGenerateLink = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsGenerating(true);
    setGeneratedLink('');

    try {
      const res = await fetch('/api/v1/growth/affiliate/generate-link', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${typeof localStorage !== 'undefined' ? localStorage.getItem('token') : ''}`
        },
        body: JSON.stringify({
          customer_id: newAffiliateEmail || 'anonymous',
          discount_percentage: newAffiliateDiscount,
          commission_percentage: newAffiliateCommission,
        })
      });

      if (res.ok) {
        const data = await res.json();
        setGeneratedLink(data.affiliate_link);
        // Refresh stats
        setStats(prev => ({...prev, total_affiliates: prev.total_affiliates + 1}));
      }
    } catch (e) {
      console.error("Failed to generate affiliate link", e);
    } finally {
      setIsGenerating(false);
    }
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(generatedLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <div className="flex items-center gap-4">
            <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Affiliate & Partner Hub</h1>
            <div className="flex items-center gap-2 px-3 py-1 bg-blue-50 rounded-full border border-blue-100">
                <span className="text-xs font-medium text-blue-600">Growth Loop</span>
            </div>
         </div>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-white rounded-xl text-sm font-semibold shadow-sm border border-gray-200 hover:bg-gray-50 transition-colors">
                Back to Dashboard
             </button>
         </div>
      </header>

      <main className="max-w-6xl mx-auto p-6 md:p-12">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-10">
            <div className="col-span-1 md:col-span-2 p-8 rounded-[24px] shadow-sm border flex flex-col justify-center" style={{ background: 'linear-gradient(135deg, #1D1D1F 0%, #434345 100%)', borderColor: 'rgba(255,255,255,0.1)' }}>
                <h2 className="text-3xl font-bold font-outfit text-white mb-3">Scale with Partners</h2>
                <p className="text-gray-300 mb-6 text-lg max-w-xl">Turn your best customers, influencers, and creators into an extended sales team. Generate custom tracking links and reward them for driving revenue.</p>
                <div className="flex items-center gap-4">
                    <div className="px-4 py-2 bg-white/10 rounded-lg text-white font-medium text-sm border border-white/20">
                        Zero Setup Fees
                    </div>
                    <div className="px-4 py-2 bg-white/10 rounded-lg text-white font-medium text-sm border border-white/20">
                        Automated Payouts
                    </div>
                </div>
            </div>

            <div className="col-span-1 p-6 rounded-[24px] shadow-sm border flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.7)', backdropFilter: 'blur(20px)', borderColor: 'rgba(0,0,0,0.05)' }}>
                <div>
                    <h3 className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-2">Total Affiliates</h3>
                    <div className="text-4xl font-bold font-outfit text-gray-900 mb-6">
                        {loading ? '...' : stats.total_affiliates}
                    </div>
                </div>
                <div>
                    <h3 className="text-sm font-semibold text-gray-500 uppercase tracking-wider mb-2">Total Commissions Paid</h3>
                    <div className="text-4xl font-bold font-outfit text-green-600">
                        ${loading ? '...' : (stats.total_commission_cents / 100).toFixed(2)}
                    </div>
                </div>
            </div>
        </div>

        <div className="p-8 rounded-[24px] shadow-sm border bg-white" style={{ borderColor: 'rgba(0,0,0,0.05)' }}>
            <div className="mb-8">
                <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Invite a New Affiliate</h2>
                <p className="text-gray-600">Generate a unique referral link with custom discount and commission rates.</p>
            </div>

            <form onSubmit={handleGenerateLink} className="space-y-6 max-w-2xl">
                <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">Affiliate Email / Identifier (Optional)</label>
                    <input
                        type="text"
                        value={newAffiliateEmail}
                        onChange={(e) => setNewAffiliateEmail(e.target.value)}
                        placeholder="e.g. partner@example.com"
                        className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-shadow bg-gray-50"
                    />
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Commission Rate (%)</label>
                        <div className="relative">
                            <input
                                type="number"
                                min="1"
                                max="100"
                                value={newAffiliateCommission}
                                onChange={(e) => setNewAffiliateCommission(parseInt(e.target.value) || 0)}
                                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-shadow bg-gray-50"
                            />
                            <div className="absolute inset-y-0 right-0 pr-4 flex items-center pointer-events-none">
                                <span className="text-gray-500 font-medium">%</span>
                            </div>
                        </div>
                        <p className="text-xs text-gray-500 mt-2">What the affiliate earns per sale.</p>
                    </div>

                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-2">Customer Discount (%)</label>
                        <div className="relative">
                            <input
                                type="number"
                                min="0"
                                max="100"
                                value={newAffiliateDiscount}
                                onChange={(e) => setNewAffiliateDiscount(parseInt(e.target.value) || 0)}
                                className="w-full px-4 py-3 rounded-xl border border-gray-200 focus:outline-none focus:ring-2 focus:ring-blue-500 transition-shadow bg-gray-50"
                            />
                            <div className="absolute inset-y-0 right-0 pr-4 flex items-center pointer-events-none">
                                <span className="text-gray-500 font-medium">%</span>
                            </div>
                        </div>
                        <p className="text-xs text-gray-500 mt-2">The discount given to the buyer.</p>
                    </div>
                </div>

                <button
                    type="submit"
                    disabled={isGenerating}
                    className={`px-6 py-3 rounded-xl text-white font-bold transition-all w-full md:w-auto ${isGenerating ? 'bg-blue-400 cursor-not-allowed' : 'bg-blue-600 hover:bg-blue-700 shadow-md hover:shadow-lg'}`}
                >
                    {isGenerating ? 'Generating...' : 'Generate Affiliate Link'}
                </button>
            </form>

            {generatedLink && (
                <div className="mt-8 p-6 bg-blue-50 border border-blue-100 rounded-2xl animate-fade-in">
                    <h3 className="text-lg font-bold text-blue-900 mb-2">Link Generated Successfully! 🎉</h3>
                    <p className="text-sm text-blue-700 mb-4">Share this unique link with your partner. When a customer uses it, they'll get {newAffiliateDiscount}% off, and the affiliate will earn {newAffiliateCommission}% commission.</p>
                    <div className="flex flex-col md:flex-row gap-3">
                        <input
                            type="text"
                            readOnly
                            value={generatedLink}
                            className="flex-1 px-4 py-3 rounded-xl border border-blue-200 bg-white text-gray-800 font-medium focus:outline-none"
                        />
                        <button
                            onClick={copyToClipboard}
                            className={`px-6 py-3 rounded-xl font-bold shadow-sm transition-all flex items-center justify-center gap-2 min-w-[140px] ${copied ? 'bg-green-500 text-white border-green-500' : 'bg-white text-blue-600 border border-blue-200 hover:bg-blue-50'}`}
                        >
                            {copied ? 'Copied!' : 'Copy Link'}
                        </button>
                        <a
                            href={`mailto:?subject=${encodeURIComponent(`Join my Affiliate Program`)}&body=${encodeURIComponent(`Hi,\n\nI'd love for you to join our partner program. You'll earn ${newAffiliateCommission}% on every sale you refer!\n\nHere is your unique link: ${generatedLink}\n\nThanks,\n${tenant}`)}`}
                            className="px-6 py-3 rounded-xl font-bold bg-blue-600 text-white shadow-sm hover:bg-blue-700 transition-all text-center flex items-center justify-center"
                        >
                            Email Partner
                        </a>
                    </div>
                </div>
            )}
        </div>
      </main>
    </div>
  );
}
