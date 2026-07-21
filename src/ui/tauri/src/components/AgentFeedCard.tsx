import React, { useState } from 'react';

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
    action_type?: string;
    proposed_action?: Record<string, any>;
    context_payload?: Record<string, any>;
}

interface AgentFeedCardProps {
    draft: ActionRequiredDraft;
    onApprove: (id: string) => void;
    onEdit: (id: string, newResponse: string) => void;
}

export const AgentFeedCard: React.FC<AgentFeedCardProps> = ({ draft, onApprove, onEdit }) => {
    const [isEditing, setIsEditing] = useState(false);
    const [editedResponse, setEditedResponse] = useState(draft.response);

    const handleSave = () => {
        onEdit(draft.draft_id, editedResponse);
        setIsEditing(false);
    };

    const handleCancel = () => {
        setEditedResponse(draft.response);
        setIsEditing(false);
    };

    return (
        <div className="w-full max-w-[375px] mx-auto rounded-[16px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] p-4 mb-4 flex flex-col gap-3 shadow-sm">
            <div className="flex justify-between items-center">
                <div className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">
                    Message from {draft.customer_name || 'Unknown User'}
                </div>
                <div className="text-xs text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 uppercase">
                    {draft.source}
                </div>
            </div>

            {isEditing ? (
                <div className="flex flex-col gap-2">
                    <textarea
                        value={editedResponse}
                        onChange={(e) => setEditedResponse(e.target.value)}
                        className="text-sm w-full min-h-[100px] text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/60 dark:bg-black/40 backdrop-blur-[30px] backdrop-saturate-[210%] border border-blue-500/50 p-3 rounded-[8px] focus:outline-none focus:ring-2 focus:ring-blue-500/50"
                        data-testid="feed-edit-textarea"
                    />
                    <div className="flex gap-2 mt-1">
                        <button
                            onClick={handleSave}
                            data-testid="feed-save-btn"
                            className="flex-1 min-h-[44px] bg-[#0066FF] hover:bg-blue-600 text-white rounded-[8px] text-sm font-medium transition-colors"
                        >
                            Save
                        </button>
                        <button
                            onClick={handleCancel}
                            data-testid="feed-cancel-btn"
                            className="flex-1 min-h-[44px] bg-white/50 dark:bg-gray-800/50 hover:bg-white/80 dark:hover:bg-gray-700/50 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[8px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-sm font-medium transition-colors"
                        >
                            Cancel
                        </button>
                    </div>
                </div>
            ) : (
                <div className="text-sm text-[#1D1D1F]/80 dark:text-[#F5F5F7]/80 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-3 rounded-[8px]">
                    {draft.response}
                </div>
            )}

            {!isEditing && (
                <div className="flex flex-col gap-2 mt-2">
                    <button
                        onClick={() => onApprove(draft.draft_id)} data-testid="feed-approve-btn"
                        className="w-full min-h-[44px] bg-[#0066FF] hover:bg-blue-600 text-white rounded-[8px] font-medium transition-colors"
                    >
                        Approve & Send
                    </button>
                    <button
                        onClick={() => setIsEditing(true)} data-testid="feed-dismiss-btn"
                        className="w-full min-h-[44px] bg-white/50 dark:bg-gray-800/50 hover:bg-white/80 dark:hover:bg-gray-700/50 text-[#1D1D1F] dark:text-[#F5F5F7] rounded-[8px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] font-medium transition-colors"
                    >
                        Edit Draft
                    </button>
                </div>
            )}
        </div>
    );
};
