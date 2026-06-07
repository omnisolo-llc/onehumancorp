"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function AffiliateProgramPage() {
  const [loading, setLoading] = useState(true);
  const [affiliates, setAffiliates] = useState([]);
  const [error, setError] = useState<string | null>(null);
  const [tenantId, setTenantId] = useState<string>('my-store');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store';
      setTenantId(storedTenant);
    }

    const fetchAffiliates = async () => {
      try {
        setLoading(true);
        const res = await fetch('/api/v1/growth/affiliates');
        if (res.ok) {
          const data = await res.json();
          setAffiliates(data.affiliates || []);
        } else {
          // If the endpoint doesn't exist yet or fails, we fail gracefully with an empty state
          setAffiliates([]);
        }
      } catch (err: any) {
        setAffiliates([]);
        setError(err.message || 'Failed to load affiliates.');
      } finally {
        setLoading(false);
      }
    };

    fetchAffiliates();
  }, []);

  const signupLink = typeof window !== 'undefined' ? `${window.location.origin}/affiliate-signup?ref=${tenantId}` : `/affiliate-signup?ref=${tenantId}`;

  const copySignupLink = () => {
    if (typeof navigator !== 'undefined' && navigator.clipboard) {
      navigator.clipboard.writeText(signupLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#000000] font-inter pb-20">
      <header className="bg-white/80 dark:bg-[#1D1D1F]/80 backdrop-blur-md sticky top-0 z-40 border-b border-gray-200 dark:border-white/10">
        <div className="max-w-4xl mx-auto px-4 h-16 flex items-center justify-between">
          <div className="flex items-center gap-4">
            <Link href="/dashboard" className="w-10 h-10 rounded-full bg-gray-100 dark:bg-white/5 flex items-center justify-center hover:bg-gray-200 dark:hover:bg-white/10 transition-colors">
              <svg className="w-5 h-5 text-gray-900 dark:text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
            </Link>
            <h1 className="text-lg font-bold font-outfit text-gray-900 dark:text-white tracking-tight">Affiliate Program</h1>
          </div>
        </div>
      </header>

      <main className="max-w-4xl mx-auto px-4 pt-8">
        <div className="glassmorphism p-8 rounded-[24px] border border-white/40 dark:border-white/10 shadow-lg mb-8 bg-white dark:bg-[#1D1D1F]">
          <div className="flex items-center gap-4 mb-6">
            <div className="w-14 h-14 rounded-2xl bg-indigo-50 dark:bg-indigo-900/30 flex items-center justify-center text-3xl shadow-sm">
              🤝
            </div>
            <div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white">Partner Program</h2>
              <p className="text-sm text-gray-600 dark:text-gray-400 mt-1">Turn your best customers into your sales team.</p>
            </div>
          </div>

          <div className="mb-8 p-6 bg-indigo-50/50 dark:bg-indigo-900/10 rounded-xl border border-indigo-100 dark:border-indigo-800/30">
            <h3 className="text-sm font-semibold text-indigo-900 dark:text-indigo-300 uppercase tracking-wide mb-3">Invite Affiliates</h3>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">Share this link with influencers or top customers to let them generate their own referral codes.</p>

            <div className="flex flex-col sm:flex-row gap-3">
              <div className="flex-1 bg-white dark:bg-black/50 border border-gray-200 dark:border-gray-700 rounded-xl px-4 py-3 flex items-center">
                <span className="text-gray-800 dark:text-gray-200 font-mono text-sm break-all truncate">{signupLink}</span>
              </div>
              <button
                onClick={copySignupLink}
                className={`px-6 py-3 rounded-xl text-sm font-bold transition-all sm:w-auto w-full ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
              >
                {copied ? 'Copied!' : 'Copy Link'}
              </button>
            </div>
          </div>

          <div>
            <h3 className="text-lg font-bold font-outfit text-gray-900 dark:text-white mb-4">Active Affiliates</h3>

            {loading ? (
              <div className="p-12 text-center">
                <div className="w-8 h-8 border-4 border-indigo-200 border-t-indigo-600 rounded-full animate-spin mx-auto mb-4"></div>
                <p className="text-sm text-gray-500 font-medium">Loading affiliates...</p>
              </div>
            ) : error ? (
              <div className="p-8 text-center bg-red-50 dark:bg-red-900/20 rounded-xl border border-red-100 dark:border-red-800/30">
                <p className="text-red-600 dark:text-red-400 text-sm font-medium">{error}</p>
              </div>
            ) : affiliates.length === 0 ? (
              <div className="p-12 text-center bg-gray-50 dark:bg-white/5 rounded-xl border border-gray-100 dark:border-white/10">
                <div className="text-4xl mb-4">📉</div>
                <h4 className="text-lg font-bold text-gray-900 dark:text-white mb-2">No active affiliates yet</h4>
                <p className="text-sm text-gray-500 max-w-sm mx-auto">Share your signup link above to start recruiting partners who will promote your products.</p>
              </div>
            ) : (
              <div className="overflow-x-auto">
                <table className="w-full text-left border-collapse">
                  <thead>
                    <tr className="border-b border-gray-200 dark:border-gray-800">
                      <th className="py-3 px-4 text-xs font-semibold text-gray-500 uppercase tracking-wider">Affiliate Code</th>
                      <th className="py-3 px-4 text-xs font-semibold text-gray-500 uppercase tracking-wider">Customer ID</th>
                      <th className="py-3 px-4 text-xs font-semibold text-gray-500 uppercase tracking-wider">Commission</th>
                    </tr>
                  </thead>
                  <tbody>
                    {affiliates.map((affiliate: any) => (
                      <tr key={affiliate.id} className="border-b border-gray-100 dark:border-gray-800 hover:bg-gray-50 dark:hover:bg-white/5">
                        <td className="py-4 px-4 text-sm font-medium text-gray-900 dark:text-white">{affiliate.affiliate_code}</td>
                        <td className="py-4 px-4 text-sm text-gray-600 dark:text-gray-400">{affiliate.customer_id}</td>
                        <td className="py-4 px-4 text-sm font-medium text-green-600">{affiliate.commission_percentage}%</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        </div>
      </main>
    </div>
  );
}