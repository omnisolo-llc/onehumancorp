"use client";

import React, { useState, useEffect } from 'react';

export function DashboardViralInviteWidget() {
  const [copied, setCopied] = useState(false);
  const [tenantId, setTenantId] = useState("default-team");
  const [referralLink, setReferralLink] = useState("/onboarding?ref=default-team&source=dashboard_invite");

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant');
      const finalTenant = storedTenant || "default-team";
      setTenantId(finalTenant);
      setReferralLink(`${window.location.origin}/onboarding?ref=${finalTenant}&source=dashboard_invite`);
    }
  }, []);

  const handleCopy = () => {
    if (navigator.clipboard) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const shareText = `Start your business on OHC! It's super easy. Use my link to get $50 off your first month: ${referralLink}`;

  return (
    <div className="mb-6 p-6 rounded-[16px] glassmorphism border border-white/40 dark:border-white/10 bg-gradient-to-r from-indigo-50/50 to-purple-50/50 dark:from-indigo-900/20 dark:to-purple-900/20" data-testid="dashboard-viral-invite-widget">
      <div className="flex flex-col md:flex-row items-center gap-6 justify-between">
        <div className="flex-1 text-center md:text-left">
          <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-indigo-100 dark:bg-indigo-900/50 text-indigo-700 dark:text-indigo-300 text-xs font-bold uppercase tracking-wider border border-indigo-200 dark:border-indigo-800">
             <span>🚀</span> Grow Your Network
          </div>
          <h2 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
            Refer & Earn $50
          </h2>
          <p className="text-sm text-gray-600 dark:text-gray-300">
            Invite another business owner to OHC. When they sign up, you both unlock a $50 credit.
          </p>
        </div>

        <div className="w-full md:w-auto flex flex-col gap-3 shrink-0">
          <div className="flex items-center gap-2 bg-white/70 dark:bg-black/40 p-1.5 rounded-xl border border-indigo-100 dark:border-indigo-800 shadow-sm">
            <input
              type="text"
              readOnly
              value={referralLink}
              className="bg-transparent border-none outline-none text-sm w-full md:w-48 text-gray-700 dark:text-gray-200 px-3 py-1"
            />
            <button
              onClick={handleCopy}
              className={`px-4 py-2 min-h-[40px] text-sm font-bold rounded-lg transition-all flex items-center justify-center gap-1 ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-600 text-white hover:bg-indigo-700'}`}
            >
              {copied ? 'Copied!' : 'Copy Link'}
            </button>
          </div>
          <a
            href={`https://wa.me/?text=${encodeURIComponent(shareText)}`}
            target="_blank"
            rel="noopener noreferrer"
            className="w-full py-2.5 bg-[#25D366] hover:bg-[#1ebd5a] text-white rounded-xl font-bold text-sm shadow-md transition-all flex items-center justify-center gap-2"
          >
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
            Share on WhatsApp
          </a>
        </div>
      </div>
    </div>
  );
}
