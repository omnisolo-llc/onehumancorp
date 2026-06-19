import React from 'react';

type InstagramDMCardProps = {
  approval: any;
  onApprove?: (id: string, actionType: string) => void;
};

export const InstagramDMCard: React.FC<InstagramDMCardProps> = ({ approval, onApprove }) => {
  const payload = approval.proposed_action || approval.context_payload || {};
  const contextSummary = payload.context || payload.context_summary || 'No prior context found.';

  return (
    <div className="mb-4 p-4 rounded-[16px] bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="instagram-dm-card">
      <div className="flex items-center gap-2 text-pink-600 font-semibold text-sm">
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
        Instagram DM
      </div>
      <div className="text-xs text-gray-500 font-medium">
        Context: {contextSummary}
      </div>
      <div className="text-xs text-gray-500 font-medium">
        Customer: {payload.customer_message}
      </div>
      <div className="text-xs text-[#1D1D1F] dark:text-[#F5F5F7] italic line-clamp-3 bg-white/50 dark:bg-black/20 p-3 rounded-[8px] break-words shadow-sm">
        Draft: {payload.draft_reply}
      </div>
      <div className="flex gap-2 mt-2">
        <button
          className="flex-1 bg-black dark:bg-white text-white dark:text-black font-semibold rounded-full h-[44px] min-h-[44px] flex items-center justify-center text-sm shadow-sm active:scale-95 transition-transform"
          onClick={() => onApprove && onApprove(approval.id, 'Approve Draft')}
          data-testid="approve-instagram-dm"
        >
          Send Draft
        </button>
        <button
          className="flex-1 bg-gray-100 dark:bg-zinc-800 text-black dark:text-white font-semibold rounded-full h-[44px] min-h-[44px] flex items-center justify-center text-sm active:scale-95 transition-transform"
        >
          Edit
        </button>
      </div>
    </div>
  );
};
