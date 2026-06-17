import React from 'react';

type AmbassadorReplyCardProps = {
  approval: any;
};

export const AmbassadorReplyCard: React.FC<AmbassadorReplyCardProps> = ({ approval }) => {
  return (
    <div className="mb-4 p-4 rounded-[16px] bg-white/65 dark:bg-[#16161A]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 flex flex-col gap-3" data-testid="ambassador-reply-card">
      <div className="flex items-center gap-2 text-[#0066FF] font-semibold text-sm">
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
        </svg>
        Customer Inquiry
      </div>
      <div className="bg-white/50 dark:bg-black/20 p-3 rounded-[8px] text-xs text-[#1D1D1F] dark:text-[#F5F5F7] italic shadow-sm">
        "{(approval.proposed_action || approval.context_payload).original_message}"
      </div>
      <div className="text-[#0066FF] font-semibold text-sm mt-2 flex items-center gap-2">
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
        </svg>
        Draft Reply
      </div>
      <div className="bg-[#0066FF] p-3 rounded-[8px] text-xs text-white shadow-inner">
        {(approval.proposed_action || approval.context_payload).generated_response}
      </div>
    </div>
  );
};
