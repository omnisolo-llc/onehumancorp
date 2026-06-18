"use client";

import React, { useState, useEffect } from 'react';

export function DashboardViralInviteWidget() {
  const [copied, setCopied] = useState(false);
  const [tenantId, setTenantId] = useState("default-team");
  const [referralLink, setReferralLink] = useState("");
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant');
      const finalTenant = storedTenant || "default-team";
      setTenantId(finalTenant);
    }
  }, []);

  const handleGenerate = async (e: React.MouseEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      if ((window as any).__TAURI__ && (window as any).__TAURI__.core) {
        const link = await (window as any).__TAURI__.core.invoke('generate_cloud_bridge_invite');
        setReferralLink(link);
      } else {
        const tenantId = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'default';
        const res = await fetch('/api/v1/growth/cloud-bridge/invite', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ team_id: tenantId, inviter_id: "owner", invitee_id: "pending" })
        });
        const data = await res.json();
        setReferralLink(data.invite_link || `https://ohc.app/invite/${tenantId}`);
      }
    } catch (err) {
      console.error(err);
      setReferralLink(`https://ohc.app/invite/${tenantId}`);
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
    const text = `Start your business on OHC! It's super easy. Use my link to get $50 off your first month: ${referralLink}`;
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(text)}`, '_blank');
  };

  const shareText = `Start your business on OHC! It's super easy. Use my link to get $50 off your first month: ${referralLink}`;

  return (
    <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 bg-gradient-to-r from-indigo-50/50 to-purple-50/50 dark:from-indigo-900/20 dark:to-purple-900/20" data-testid="dashboard-viral-invite-widget">
      <div className="flex flex-col gap-4">
        <div>
          <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Invite & Earn</h2>
          <p className="text-sm text-gray-600 dark:text-gray-300">
            Invite a fellow business owner to OHC. They get 1 month free, you get $50 credit.
          </p>
        </div>
        {!referralLink ? (
          <button
            id="dashboard-invite-btn"
            onClick={handleGenerate}
            disabled={loading}
            className="w-full min-h-[44px] min-w-[44px] bg-[#0f766e] hover:bg-[#0d645d] text-white font-semibold py-3 px-6 rounded-xl transition-colors"
          >
            {loading ? 'Generating...' : 'Get My Invite Link'}
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
      </div>
    </div>
  );
}
