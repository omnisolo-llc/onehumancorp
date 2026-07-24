import React from 'react';

type InstagramDMCardProps = {
  onApprove?: () => void;
  onDismiss?: () => void;
  approval: any;
};

function formatCurrency(cents: number | undefined) {
  if (cents === undefined || isNaN(cents)) return '';
  return `$${(cents / 100).toFixed(2)}`;
}

export const InstagramDMCard: React.FC<InstagramDMCardProps> = ({ approval, onApprove, onDismiss }) => {
  const payload = approval.proposed_action || approval.context_payload;
  const draftReply = payload?.draft_reply || payload?.generated_response;
  const isQuote = payload?.feature_type === 'quote_draft' || payload?.action_type === 'Draft Quote' || payload?.total_amount_cents !== undefined;

  return (
    <div className="mb-4 p-4 max-w-[375px] w-full mx-auto bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3 shadow-sm" data-testid="instagram-dm-card">
      <div className="flex items-center gap-2 text-pink-600 font-semibold text-sm">
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
        </svg>
        <strong className="font-outfit tracking-wide">Instagram DM</strong>
      </div>
      <div className="text-xs text-gray-500 dark:text-gray-400 font-medium break-words">
        Customer: <div className="triage-context inline break-words">{payload?.customer_message || payload?.original_message || payload?.description}</div>
      </div>
      <div className="text-sm text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 p-3 rounded-[8px] break-words shadow-sm">
        <div className="font-semibold text-xs uppercase mb-1 block">Draft Reply:</div>
        <div className="whitespace-pre-wrap">{draftReply}</div>

        {isQuote && payload?.total_amount_cents !== undefined && (
          <div className="mt-3 pt-3 border-t border-gray-200 dark:border-gray-700">
            <div className="flex justify-between items-center text-xs font-semibold mb-1">
              <span className="text-gray-500 dark:text-gray-400">Quote Total:</span>
              <span>{formatCurrency(payload.total_amount_cents)}</span>
            </div>
            {payload.required_deposit_cents > 0 && (
              <div className="flex justify-between items-center text-xs font-semibold text-pink-600">
                <span>Required Deposit:</span>
                <span>{formatCurrency(payload.required_deposit_cents)}</span>
              </div>
            )}
            <div className="text-[10px] text-gray-400 mt-2 italic">A Stripe payment link will be automatically generated and appended to your reply.</div>
          </div>
        )}
      </div>

      <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
        {onApprove && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onApprove();
            }}
            className="triage-btn-approve flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-pink-600 text-white font-medium hover:bg-pink-700 transition-all duration-200 shadow-md flex items-center justify-center"
            aria-label="Approve & Send" data-testid="feed-approve-btn" id="approve-instagram-dm"
          >
            Approve & Send
          </button>
        )}
        {onDismiss && (
          <button
            onClick={(e) => {
              e.stopPropagation();
              onDismiss();
            }}
            className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
            aria-label="Dismiss"
            data-testid="dismiss-instagram-dm"
          >
            Dismiss
          </button>
        )}
      </div>
    </div>
  );
};
