"use client";

import React from 'react';
import { useCloudInvitation } from '../referrals/useCloudInvitation';
import { useClipboardFeedback } from '@/hooks/useClipboardFeedback';

const noPrivateDraft = () => {};

export function DashboardViralInviteWidget() {
  const invitation = useCloudInvitation(noPrivateDraft);
  const referralLink = invitation.link;
  const clipboard = useClipboardFeedback(referralLink);
  const shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(`Join me on OmniSolo OneHumanCorp: ${referralLink}`)}`;

  return (
    <div className="mb-6 omnisolo-growth-card p-6 backdrop-blur-[30px] saturate-[210%] bg-white/30 dark:bg-black/30 border border-white/20 dark:border-white/10 bg-gradient-to-r from-indigo-50/50 to-purple-50/50 dark:from-indigo-900/20 dark:to-purple-900/20 shadow-xl" data-testid="dashboard-viral-invite-widget">
      <div className="flex flex-col gap-4">
        <div>
          <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">Invite a Business Owner</h2>
          <p className="text-sm text-gray-600 dark:text-gray-300">
            Create an invitation for your verified account. Creating a link does not send it or confirm that anyone has joined. Referral rewards have not been verified.
          </p>
        </div>
        {!referralLink ? (
          <button
            id="dashboard-invite-btn"
            onClick={() => void invitation.create('pending')}
            disabled={invitation.phase !== 'ready'}
            className="w-full min-h-[44px] min-w-[44px] bg-[#0f766e] hover:bg-[#0d645d] text-white font-semibold py-3 px-6 rounded-xl transition-all shadow-md"
          >
            {invitation.phase === 'requesting' ? (
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
              aria-label="Invitation link"
              readOnly
              value={referralLink}
              className="w-full px-4 py-2 rounded-lg bg-white/50 dark:bg-black/20 border border-gray-200 dark:border-gray-700 text-gray-800 dark:text-gray-200"
            />
            <div className="flex flex-wrap gap-2">
              <button
                id="dashboard-copy-btn"
                onClick={() => void clipboard.copy(referralLink)}
                disabled={clipboard.state === 'pending'}
                className="flex-1 bg-white dark:bg-gray-800 text-gray-800 dark:text-white hover:bg-gray-50 border border-gray-200 py-2 px-4 rounded-lg font-medium transition-colors"
              >
                {clipboard.state === 'copied' ? 'Copied!' : clipboard.state === 'pending' ? 'Copying…' : 'Copy'}
              </button>
              <a
                id="dashboard-share-x-btn"
                href={shareUrl}
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 bg-black text-white hover:bg-gray-800 py-2 px-4 rounded-lg font-medium transition-colors"
              >
                Share on X
              </a>
            </div>
          </div>
        )}
        <p role="status" aria-label="Dashboard invitation status" className="text-sm">{invitation.message}</p>
        {referralLink && clipboard.message && <p role={clipboard.state === 'error' ? 'alert' : 'status'} className="text-sm">{clipboard.message}</p>}
      </div>
    </div>
  );
}
