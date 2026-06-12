"use client";

import React, { useState, useEffect } from "react";

export function LeadMagnetWidget() {
  const [tenantId, setTenantId] = useState("default-team");
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    let currentTenant = "default-team";
    if (typeof localStorage !== "undefined") {
      currentTenant = localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default-team";
      setTenantId(currentTenant);
    }
  }, []);

  const handleCopy = () => {
    const embedCode = `<iframe src="https://ohc.app/api/v1/growth/lead-magnet/embed?tenant=${tenantId}" width="100%" height="400" frameborder="0" style="border-radius: 12px; border: 1px solid #eaeaea;"></iframe>\n<div style="text-align:center; font-size:12px; margin-top:8px;">\n  <a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}" target="_blank" style="color:#6b7280;text-decoration:none;">⚡ Powered by OHC</a>\n</div>`;
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 3000);
  };

  return (
    <div className="glassmorphism p-6 rounded-[16px] border border-white/40 dark:border-white/10 shadow-lg mb-6 flex flex-col md:flex-row gap-6 items-center">
      <div className="flex-1">
        <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-purple-50 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300 text-sm font-semibold border border-purple-200 dark:border-purple-800">
          <span className="w-2 h-2 rounded-full bg-purple-500 animate-pulse"></span>
          Viral Lead Magnet Embed
        </div>
        <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
          Grow Your Email List
        </h2>
        <p className="text-gray-600 dark:text-gray-300 text-sm flex items-center gap-2">
          <svg className="w-4 h-4 text-purple-500 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" /></svg>
          Offer a free guide or resource to visitors on your website. When they sign up, they are added to your audience, and every signup also grows your referral points via the "Powered by OHC" link!
        </p>
      </div>

      <div className="w-full md:w-auto shrink-0 flex flex-col gap-3">
        <button
          onClick={handleCopy}
          className="w-full app-button min-h-[44px] bg-purple-600 hover:bg-purple-700 text-white border-none py-3 px-6 text-sm font-bold shadow-md transition-all flex items-center justify-center gap-2 rounded-xl"
        >
          {copied ? (
            <>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
              Embed Code Copied!
            </>
          ) : (
            <>
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
              Copy Embed Code
            </>
          )}
        </button>
      </div>
    </div>
  );
}