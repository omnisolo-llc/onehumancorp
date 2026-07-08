"use client";

import React, { useState } from 'react';
import { Card, CardContent } from "@/components/ui/card";

export default function GrowthReferralWidget() {
  const [loading, setLoading] = useState(false);
  const [referralLink, setReferralLink] = useState('');
  const [error, setError] = useState('');
  const [copied, setCopied] = useState(false);

  const generateLink = async () => {
    setLoading(true);
    setError('');
    try {
      const tenantId = typeof window !== 'undefined' ? (localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'default-team') : 'default-team';
      const inviterId = typeof window !== 'undefined' ? (localStorage.getItem('user_id') || 'local-user') : 'local-user';

      const res = await fetch('/api/v1/growth/cloud-bridge/invite', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          team_id: tenantId,
          inviter_id: inviterId,
          invitee_id: 'pending-invite',
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

  const handleTwitter = () => {
    if (referralLink) {
      const url = `https://twitter.com/intent/tweet?text=${encodeURIComponent(
        `Hey! I use OneHumanCorp to run my business. It's super easy. Check it out: ${referralLink}\n\n⚡ Powered by OHC`
      )}`;
      window.open(url, '_blank');
    }
  };

  return (
    <div className="ohc-growth-card flex flex-col gap-8">
      <Card className="mb-6 border-white/20 dark:border-white/10 shadow-xl overflow-hidden backdrop-blur-[30px] saturate-[210%] bg-white/30 dark:bg-black/30">
        <CardContent className="p-6">
          <div className="flex flex-col md:flex-row gap-6 items-center">
            <div className="flex-1">
              <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-300 text-sm font-semibold">
                <span>🚀 Sovereign-to-Cloud Bridge</span>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
                Grow Your Team
              </h2>
              <p className="text-gray-600 dark:text-gray-300 text-sm flex items-center gap-2">
                <svg className="w-4 h-4 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" /></svg>
                Bridge your local sovereignty with cloud-native collaboration. Invite a member to a shared multi-tenant space while maintaining Zero Data Leakage locally.
              </p>
            </div>

            <div className="w-full md:w-auto">
              {!referralLink ? (
                <button
                  onClick={generateLink}
                  disabled={loading}
                  className="w-full md:w-auto app-button min-h-[44px] bg-indigo-600 hover:bg-indigo-700 text-white border-none py-3 px-6 text-base rounded-md"
                >
                  {loading ? 'Generating...' : 'Invite to Cloud Team'}
                </button>
              ) : (
                <div className="flex flex-col gap-3 w-full md:w-auto">
                  <div className="flex items-center gap-2 bg-white/50 dark:bg-black/20 p-2 rounded-lg border border-gray-200 dark:border-gray-700">
                    <input id="cloud-bridge-invite-link"
                      type="text"
                      readOnly
                      value={referralLink}
                      className="bg-transparent border-none outline-none text-sm w-full md:w-48 text-gray-700 dark:text-gray-200 px-2"
                    />
                    <button
                      onClick={handleCopy}
                      className="px-4 py-2 bg-gray-100 min-h-[44px] hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-800 dark:text-gray-200 text-sm font-medium rounded-md transition-colors"
                    >
                      {copied ? 'Copied!' : 'Copy'}
                    </button>
                  </div>
                  <button
                    onClick={handleWhatsApp}
                    className="w-full app-button min-h-[44px] bg-[#25D366] hover:bg-[#1ebd5a] text-white border-none py-2 text-sm flex items-center justify-center gap-2 rounded-md"
                  >
                    Share on WhatsApp
                  </button>
                  <button
                    onClick={handleTwitter}
                    className="w-full app-button min-h-[44px] bg-black hover:bg-gray-800 text-white border-none py-2 text-sm flex items-center justify-center gap-2 shadow-sm transition-all rounded-md"
                  >
                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                    Share on X (Twitter)
                  </button>
                </div>
              )}
              {error && <p className="text-[#FF3B30] text-sm mt-2">{error}</p>}
            </div>
          </div>
        </CardContent>
      </Card>

      <Card className="border-white/20 dark:border-white/10 shadow-xl overflow-hidden backdrop-blur-[30px] saturate-[210%] bg-white/30 dark:bg-black/30">
        <CardContent className="p-6">
          <div className="flex flex-col md:flex-row gap-6 items-center">
            <div className="flex-1">
              <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-blue-50 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 text-sm font-semibold">
                <span>🌐 Viral Storefront Embed</span>
              </div>
              <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
                Embed Your Business
              </h2>
              <p className="text-gray-600 dark:text-gray-300 text-sm flex items-center gap-2">
                <svg className="w-4 h-4 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" /></svg>
                Put your storefront anywhere. Includes a built-in referral loop to reward you when other owners join through your embed.
              </p>
            </div>

            <div className="w-full md:w-auto">
              <button
                onClick={() => {
                  const tenantId = typeof window !== 'undefined' ? (localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'default-team') : 'default-team';
                  navigator.clipboard.writeText(`<iframe src="https://ohc.app/api/v1/growth/storefront/embed?tenant=${tenantId}" width="100%" height="600" frameborder="0" style="border-radius: 12px; border: 1px solid #eaeaea;"></iframe>\n<div style="text-align:center; font-size:12px; margin-top:8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}" target="_blank" style="color:#6b7280;text-decoration:none;">⚡ Powered by OHC</a></div>`);
                  alert('Embed code copied to clipboard!');
                }}
                className="w-full app-button min-h-[44px] bg-[#0071E3] hover:bg-blue-700 text-white border-none py-3 px-6 text-sm rounded-md"
              >
                Copy Embed Code
              </button>
            </div>
          </div>
        </CardContent>
      </Card>

      <div className="mt-8 pt-6 border-t border-white/20 dark:border-white/10">
        <div className="flex flex-col md:flex-row gap-6 items-center">
          <div className="flex-1">
            <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
              🎉 10th Order! Share your success
            </h3>
            <p className="text-gray-600 dark:text-gray-300 text-sm mb-4">
              You just hit a major milestone! Let your network know that your business is booming.
            </p>
            <a
              href={`https://wa.me/?text=${encodeURIComponent('I just hit my 10th order on OHC! Check it out: https://ohc.app/invite/default-team')}`}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 app-button min-h-[44px] bg-[#25D366] hover:bg-[#1ebd5a] text-white py-2 px-4 rounded-md text-sm font-semibold transition-colors"
            >
              <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
              Share to WhatsApp
            </a>
          </div>
          <div className="w-full md:w-1/2 rounded-xl overflow-hidden shadow-md border border-white/20">
            <img
              src="/api/v1/growth/milestone/card?milestone_id=10th_order&tenant=default-team"
              alt="10th Order Milestone"
              className="w-full h-auto object-cover"
            />
          </div>
        </div>
      </div>
    </div>
  );
}
