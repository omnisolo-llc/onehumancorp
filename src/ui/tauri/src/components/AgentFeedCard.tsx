import React from 'react';

export interface ActionRequiredDraft {
    draft_id: string;
    work_item_id: string;
    tenant_id: string;
    customer_id: string;
    customer_name?: string;
    source: string;
    response: string;
    status: string;
    created_at?: string;
}

interface AgentFeedCardProps {
    draft: ActionRequiredDraft;
    onApprove: (id: string) => void;
    onEdit: (id: string) => void;
}

export const AgentFeedCard: React.FC<AgentFeedCardProps> = ({ draft, onApprove, onEdit }) => {
    return (
        <div className="w-full max-w-[375px] mx-auto rounded-[16px] border border-white/40 dark:border-white/10 bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] p-4 mb-4 flex flex-col gap-3 shadow-sm">
            <div className="flex justify-between items-center">
                <div className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">
                    Message from {draft.customer_name || 'Unknown User'}
                </div>
                <div className="text-xs text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 uppercase">
                    {draft.source}
                </div>
            </div>

            <div className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 p-3 rounded-[8px]">
                {draft.response}
            </div>

            <div className="flex flex-col gap-2 mt-2">
                <button
                    onClick={() => onApprove(draft.draft_id)}
                    className="w-full min-h-[44px] bg-[#0066FF] hover:bg-blue-600 text-white rounded-[8px] font-medium transition-colors"
                >
                    Approve & Send
                </button>
                <button
                    onClick={() => onEdit(draft.draft_id)}
                    className="w-full min-h-[44px] bg-white/50 dark:bg-gray-800/50 hover:bg-white/80 dark:hover:bg-gray-700/50 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[8px] border border-white/40 dark:border-white/10 font-medium transition-colors"
                >
                    Edit Draft
                </button>
            </div>
        </div>
    );
};
