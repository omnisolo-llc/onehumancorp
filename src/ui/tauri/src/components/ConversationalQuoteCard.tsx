import React from 'react';

export interface ConversationalQuoteDraft {
    id: string;
    customer_name?: string;
    original_message: string;
    proposed_response: string;
    suggested_price?: number;
    deposit_required?: number;
    missing_fields?: string[];
    is_quote: boolean;
}

interface ConversationalQuoteCardProps {
    draft: ConversationalQuoteDraft;
    onApprove: (id: string) => void;
    onEdit: (id: string) => void;
}

export const ConversationalQuoteCard: React.FC<ConversationalQuoteCardProps> = ({ draft, onApprove, onEdit }) => {
    return (
        <div className="w-full max-w-[375px] mx-auto rounded-[16px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] p-4 mb-4 flex flex-col gap-3 shadow-sm">
            <div className="flex justify-between items-center">
                <div className="text-sm font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] flex items-center gap-2">
                    <span className="bg-blue-100 text-blue-800 text-[10px] uppercase font-bold px-2 py-0.5 rounded-full dark:bg-blue-900 dark:text-blue-200">
                        {draft.is_quote ? 'Quote Ready' : 'Info Needed'}
                    </span>
                    {draft.customer_name || 'Customer'}
                </div>
            </div>

            <div className="text-xs text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60">
                "{draft.original_message}"
            </div>

            <div className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-3 rounded-[8px]">
                {draft.proposed_response}
            </div>

            {draft.is_quote && (
                <div className="flex justify-between items-center bg-green-50 dark:bg-green-900/20 p-2 rounded-md border border-green-200 dark:border-green-800">
                    <div className="text-xs text-green-800 dark:text-green-200 font-medium">Proposed Quote: ${draft.suggested_price}</div>
                    <div className="text-[10px] text-green-700 dark:text-green-300">Deposit: ${draft.deposit_required}</div>
                </div>
            )}

            {!draft.is_quote && draft.missing_fields && draft.missing_fields.length > 0 && (
                <div className="text-[10px] text-amber-600 dark:text-amber-400 font-medium">
                    Missing: {draft.missing_fields.join(', ')}
                </div>
            )}

            <div className="flex flex-col gap-2 mt-2">
                <button
                    onClick={() => onApprove(draft.id)}
                    className="w-full min-h-[44px] bg-[#0066FF] hover:bg-blue-600 text-white rounded-[8px] font-medium transition-colors touch-manipulation"
                >
                    Approve & Send
                </button>
                <button
                    onClick={() => onEdit(draft.id)}
                    className="w-full min-h-[44px] bg-white/50 dark:bg-gray-800/50 hover:bg-white/80 dark:hover:bg-gray-700/50 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[8px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] font-medium transition-colors touch-manipulation"
                >
                    Edit Draft
                </button>
            </div>
        </div>
    );
};
