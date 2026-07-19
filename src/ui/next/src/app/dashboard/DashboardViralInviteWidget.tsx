"use client";

import React, { useState, useEffect } from 'react';

export function DashboardViralInviteWidget() {
  const [copied, setCopied] = useState(false);
  const [tenantId, setTenantId] = useState("default-team");
  const [referralLink, setReferralLink] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('business_display_name');
      const finalTenant = storedTenant || "default-team";
      setTenantId(finalTenant);
    }
  }, []);

  const handleGenerate = async (e: React.MouseEvent) => {
    e.preventDefault();
    setLoading(true);
    setError("");
    try {
      const w = window as unknown as { __TAURI__?: { core?: { invoke: (cmd: string) => Promise<string> } } };
      if (w.__TAURI__ && w.__TAURI__.core) {
        const link = await w.__TAURI__.core.invoke('generate_referral_link');
        setReferralLink(link);
      } else {
        const headers: Record<string, string> = { 'Content-Type': 'application/json' };

        const res = await fetch('/api/v1/growth/referrals/generate', {
          method: 'POST',
          headers,
          body: JSON.stringify({ custom_message: "" })
        });
        if (!res.ok) throw new Error('Referral service unavailable');
        const data = await res.json();
        if (typeof data.referral_link !== 'string' || !data.referral_link) throw new Error('Invalid referral response');
        setReferralLink(data.referral_link);
      }
    } catch {
      setError('A referral link could not be generated.');
    }
    setLoading(false);
  };

  const handleCopy = () => {
    if (navigator.clipboard) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleShareX = (e: React.MouseEvent) => {
    e.preventDefault();
    const text = `Start your business on OHC using my referral link: ${referralLink}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}`, '_blank');
  };

  return (
    <div className="mb-6 ohc-growth-card p-6 backdrop-blur-[30px] saturate-[210%] bg-white/30 dark:bg-black/30 border border-white/20 dark:border-white/10 bg-gradient-to-r from-indigo-50/50 to-purple-50/50 dark:from-indigo-900/20 dark:to-purple-900/20 shadow-xl" data-testid="dashboard-viral-invite-widget">
      <div className="flex flex-col gap-4">
        <div>
          <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Invite a Business Owner</h2>
          <p className="text-sm text-gray-600 dark:text-gray-300">
            Generate a referral link through the OHC referral service and share it.
          </p>
        </div>
        {!referralLink ? (
          <button
            id="dashboard-invite-btn"
            onClick={handleGenerate}
            disabled={loading}
            className="w-full min-h-[44px] min-w-[44px] bg-[#0f766e] hover:bg-[#0d645d] text-white font-semibold py-3 px-6 rounded-xl transition-all shadow-md"
          >
            {loading ? (
              <span className="flex items-center justify-center gap-2">
                <svg className="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Generating...
              </span>
            ) : 'Get My Invite Link'}
          </button>
        ) : (
          <div id="dashboard-invite-container" className="flex flex-col gap-3">
            <input
              id="dashboard-invite-link"
              type="text"
              readOnly
              value={referralLink}
              className="w-full px-4 py-2 rounded-lg bg-white/50 dark:bg-black/20 border border-gray-200 dark:border-gray-700 text-gray-800 dark:text-gray-200"
            />
            <div className="flex flex-wrap gap-2">
              <button
                id="dashboard-copy-btn"
                onClick={handleCopy}
                className="flex-1 bg-white dark:bg-gray-800 text-gray-800 dark:text-white hover:bg-gray-50 border border-gray-200 py-2 px-4 rounded-lg font-medium transition-colors"
              >
                {copied ? 'Copied!' : 'Copy'}
              </button>
              <button
                id="dashboard-share-x-btn"
                onClick={handleShareX}
                className="flex-1 bg-black text-white hover:bg-gray-800 py-2 px-4 rounded-lg font-medium transition-colors"
              >
                Share on X
              </button>
            </div>
          </div>
        )}
        {error && <p className="text-sm text-red-600" role="alert">{error}</p>}
      </div>
    </div>
  );
}
