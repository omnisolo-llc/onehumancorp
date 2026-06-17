import React from 'react';

type InstagramDMCardProps = {
  approval: any;
};

export const InstagramDMCard: React.FC<InstagramDMCardProps> = ({ approval }) => {
  return (
    <div className="mb-4 p-4 rounded-[16px] bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="instagram-dm-card">
      <div className="flex items-center gap-2 text-pink-600 font-semibold text-sm">
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
        Instagram DM
      </div>
      <div className="text-xs text-gray-500 font-medium">
        Customer: {(approval.proposed_action || approval.context_payload).customer_message}
      </div>
      <div className="text-xs text-[#1D1D1F] dark:text-[#F5F5F7] italic line-clamp-3 bg-white/50 dark:bg-black/20 p-3 rounded-[8px] break-words shadow-sm">
        Draft: {(approval.proposed_action || approval.context_payload).draft_reply}
      </div>
    </div>
  );
};
