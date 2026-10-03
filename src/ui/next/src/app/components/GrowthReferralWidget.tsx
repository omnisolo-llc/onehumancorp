"use client";

import React from 'react';
import { useClipboardFeedback } from '@/hooks/useClipboardFeedback';
import { Card, CardContent } from "@/components/ui/card";
import { useCloudInvitation } from '../referrals/useCloudInvitation';
import { RecordedOrderMilestone } from './RecordedOrderMilestone';
import { PublishedStorefrontEmbed } from './PublishedStorefrontEmbed';

const noPrivateDraft = () => {};

export default function GrowthReferralWidget() {
  const invitation = useCloudInvitation(noPrivateDraft);
  const referralLink = invitation.link;
  const inviteCopy = useClipboardFeedback(referralLink);


  const handleCopy = () => { void inviteCopy.copy(referralLink); };

  const handleWhatsApp = () => {
    if (referralLink) {
      const url = `https://wa.me/?text=${encodeURIComponent(
        `Hey! I use OmniSolo OneHumanCorp to run my business. It's super easy. Check it out: ${referralLink}`
      )}`;
      window.open(url, '_blank');
    }
  };

  const handleTwitter = () => {
    if (referralLink) {
      const url = `https://twitter.com/intent/tweet?text=${encodeURIComponent(
        `Hey! I use OmniSolo OneHumanCorp to run my business. It's super easy. Check it out: ${referralLink}\n\n⚡ Powered by OmniSolo`
      )}`;
      window.open(url, '_blank');
    }
  };

  return (
    <div className="omnisolo-growth-card flex flex-col gap-8">
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
                Create an invitation link for your verified account. Creating a link does not send it or confirm that anyone has joined.
              </p>
            </div>

            <div className="w-full md:w-auto">
              {!referralLink ? (
                <button
                  onClick={() => void invitation.create('pending-invite')}
                  disabled={invitation.phase !== 'ready'}
                  aria-label="Unlock Cloud Collaboration"
                  className="w-full md:w-auto app-button min-h-[44px] bg-indigo-600 hover:bg-indigo-700 text-white border-none py-3 px-6 text-base rounded-md"
                >
                  {invitation.phase === 'requesting' ? 'Generating...' : 'Invite to Cloud Team'}
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
                      disabled={inviteCopy.state === 'pending'}
                      className="px-4 py-2 bg-gray-100 min-h-[44px] hover:bg-gray-200 dark:bg-gray-800 dark:hover:bg-gray-700 text-gray-800 dark:text-gray-200 text-sm font-medium rounded-md transition-colors"
                    >
                      {inviteCopy.state === 'copied' ? 'Copied!' : 'Copy'}
                    </button>
                    {inviteCopy.message && <p role={inviteCopy.state === 'error' ? 'alert' : 'status'}>{inviteCopy.message}</p>}
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
              <p role="status" aria-label="Team invitation status" className="text-sm mt-2">{invitation.message}</p>
            </div>
          </div>
        </CardContent>
      </Card>

      <PublishedStorefrontEmbed />

      <RecordedOrderMilestone invitation={invitation} />
    </div>
  );
}
