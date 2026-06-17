import React from 'react';

type InstagramDMCardProps = {
  approval: any;
};

export const InstagramDMCard: React.FC<InstagramDMCardProps> = ({ approval }) => {
  return (
    <div className="mb-4 p-4 rounded-xl glassmorphism flex flex-col gap-3" data-testid="instagram-dm-card">
      <div className="flex items-center gap-2 text-pink-600 font-semibold text-sm">
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
        Instagram DM
      </div>
      <div className="text-xs text-gray-500 font-medium">
        Customer: {(approval.proposed_action || approval.context_payload).customer_message}
      </div>
      <div className="text-xs text-gray-900 dark:text-gray-100 italic line-clamp-3 bg-white/50 dark:bg-black/20 p-2 rounded break-words">
        Draft: {(approval.proposed_action || approval.context_payload).draft_reply}
      </div>
    </div>
  );
};
