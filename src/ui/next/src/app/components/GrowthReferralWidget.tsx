"use client";

import React, { useState } from 'react';

export default function GrowthReferralWidget() {
  const [loading, setLoading] = useState(false);
  const [referralLink, setReferralLink] = useState('');
  const [copied, setCopied] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const generateLink = async () => {
    setLoading(true);
    setError(null);
    try {
      const tenantId = typeof window !== 'undefined' ? (localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'default-team') : 'default-team';
      const inviterId = typeof window !== 'undefined' ? (localStorage.getItem('user_id') || 'local-user') : 'local-user';

      const res = await fetch('/api/v1/growth/team-invites', {

        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          team_id: tenantId,
          inviter_id: inviterId,
        }),
      });
      if (!res.ok) {
        throw new Error('Failed to generate invite');
      }
      const data = await res.json();
      setReferralLink(data.invite_link);
    } catch (err: any) {
      setError(err.message || 'Something went wrong');
    } finally {
      setLoading(false);
    }
  };

  const handleCopy = () => {
    if (referralLink) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleWhatsApp = () => {
    if (referralLink) {
      const url = `https://wa.me/?text=${encodeURIComponent(
        `Hey! I use OneHumanCorp to run my business. It's super easy. Check it out: ${referralLink}`
      )}`;
      window.open(url, '_blank');
    }
  };

  return (
    <div className="glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10 shadow-lg mb-6">
      <div className="flex flex-col md:flex-row gap-6 items-center">
        <div className="flex-1">
          <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-300 text-sm font-semibold">
            <span>🚀 Sovereign-to-Cloud Bridge</span>
          </div>
          <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
            Invite your network, grow together
          </h2>
          <p className="text-gray-600 dark:text-gray-300 text-sm">
            Share your unique invite link with other business owners. When they sign up, you both get premium credits.
          </p>
        </div>

        <div className="w-full md:w-auto">
          {!referralLink ? (
            <button
              onClick={generateLink}
              disabled={loading}
              className="w-full md:w-auto app-button bg-indigo-600 hover:bg-indigo-700 text-white border-none py-3 px-6 text-base"
            >
              {loading ? 'Generating...' : 'Get My Invite Link'}
            </button>
          ) : (
            <div className="flex flex-col gap-3 w-full md:w-auto">
              <div className="flex items-center gap-2 bg-white/50 dark:bg-black/20 p-2 rounded-lg border border-gray-200 dark:border-gray-700">
                <input
                  type="text"
                  readOnly
                  value={referralLink}
                  className="bg-transparent border-none outline-none text-sm w-full md:w-48 text-gray-700 dark:text-gray-200 px-2"
                />
                <button
                  onClick={handleCopy}
                  className="px-4 py-2 bg-gray-100 hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-800 dark:text-gray-200 text-sm font-medium rounded-md transition-colors"
                >
                  {copied ? 'Copied!' : 'Copy'}
                </button>
              </div>
              <button
                onClick={handleWhatsApp}
                className="w-full app-button bg-[#25D366] hover:bg-[#1ebd5a] text-white border-none py-2 text-sm flex items-center justify-center gap-2"
              >
                Share on WhatsApp
              </button>
            </div>
          )}
          {error && <p className="text-red-500 text-sm mt-2">{error}</p>}
        </div>
      </div>
    </div>
  );
}
