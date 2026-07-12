import React, { useEffect, useState } from 'react';
import { AgentFeedCard, ActionRequiredDraft } from './AgentFeedCard';
import { PayoutSummaryCard } from './owner_feed/PayoutSummaryCard';
import { ReviewDraftQuoteCard } from './owner_feed/ReviewDraftQuoteCard';

export const AgentFeed: React.FC = () => {
    const [drafts, setDrafts] = useState<ActionRequiredDraft[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const fetchDrafts = async () => {
        try {
            setLoading(true);
            const response = await fetch('/api/inbox/action_required');
            if (!response.ok) {
                throw new Error('Failed to fetch action required drafts');
            }
            const data = await response.json();
            setDrafts(data);
            setError(null);
        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchDrafts();
    }, []);

    const handleApprove = async (id: string) => {
        try {
            const response = await fetch(`/api/inbox/action_required/${id}/approve`, {
                method: 'POST',
            });
            if (response.ok) {
                // Remove the draft from the UI optimistically
                setDrafts(drafts.filter(d => d.draft_id !== id));
            } else {
                console.error("Failed to approve draft");
            }
        } catch (err) {
            console.error(err);
        }
    };

    const handleEdit = (id: string) => {
        // Implement edit logic, maybe a modal or inline editing
        console.info("Edit draft", id);
    };


    if (loading) {
        return (
            <div className="w-full max-w-[375px] mx-auto p-4 flex flex-col items-center">
                <h2 className="text-xl font-semibold mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] self-start">Agent Feed</h2>
                <div className="w-full bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-4 shadow-sm border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] flex flex-col gap-3">
                    <div className="flex justify-between items-center animate-pulse">
                        <div className="h-4 bg-black/10 dark:bg-white/10 rounded w-1/3"></div>
                        <div className="h-3 bg-black/10 dark:bg-white/10 rounded w-1/4"></div>
                    </div>
                    <div className="h-16 bg-white/40 dark:bg-black/20 rounded-[8px] animate-pulse"></div>
                    <div className="flex flex-col gap-2 mt-2">
                        <div className="h-11 bg-black/10 dark:bg-white/10 rounded-[8px] animate-pulse"></div>
                        <div className="h-11 bg-black/10 dark:bg-white/10 rounded-[8px] animate-pulse"></div>
                    </div>
                </div>
            </div>
        );
    }


    if (error) {
        return (
            <div className="w-full max-w-[375px] mx-auto p-4 flex flex-col items-center">
                <h2 className="text-xl font-semibold mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] self-start">Agent Feed</h2>
                <div className="w-full bg-[#FF3B30]/10 dark:bg-[#FF3B30]/20 backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-4 shadow-sm border border-[#FF3B30]/30 text-center">
                    <p className="text-[#FF3B30] text-sm font-medium">{error}</p>
                </div>
            </div>
        );
    }


    if (drafts.length === 0) {
        return (
            <div className="w-full max-w-[375px] mx-auto p-4 flex flex-col items-center">
                <h2 className="text-xl font-semibold mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] self-start">Agent Feed</h2>
                <div className="w-full bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-8 shadow-sm border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] text-center flex flex-col items-center justify-center min-h-[200px]">
                    <svg className="w-12 h-12 text-[#34C759] mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
                    <p className="text-[#1D1D1F] dark:text-[#F5F5F7] font-medium">All caught up!</p>
                    <p className="text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 mt-1">No pending actions right now.</p>
                </div>
            </div>
        );
    }


    return (
        <div className="w-full max-w-[375px] mx-auto p-4 flex flex-col items-center">
            <h2 className="text-xl font-semibold mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] self-start">Agent Feed</h2>
            {drafts.map(draft => {
                const actionType = draft.action_type || draft.proposed_action?.action_type || draft.context_payload?.feature_type;

                if (actionType === 'PayoutSummary' || actionType === 'payout_summary') {
                    const parsedPayload = draft.proposed_action || draft.context_payload || {};
                    return (
                        <div key={draft.draft_id} className="w-full mb-4">
                            <PayoutSummaryCard
                                totalUsdEarned={parsedPayload.total_usd_earned || 0}
                                totalEurEarned={parsedPayload.total_eur_earned || 0}
                                totalUsdPayout={parsedPayload.total_usd_payout || 0}
                                onViewDetails={() => handleApprove(draft.draft_id)}
                            />
                        </div>
                    );
                }

                if (actionType === 'ReviewDraftQuote' || actionType === 'review_draft_quote') {
                    const parsedPayload = draft.proposed_action || draft.context_payload || {};
                    return (
                        <div key={draft.draft_id} className="w-full mb-4">
                            <ReviewDraftQuoteCard
                                customerName={parsedPayload.customer_name || draft.customer_name || 'Customer'}
                                projectDescription={parsedPayload.project_description || 'Project'}
                                totalCost={parsedPayload.total_cost || 0}
                                onApprove={() => handleApprove(draft.draft_id)}
                                onEdit={() => handleEdit(draft.draft_id)}
                            />
                        </div>
                    );
                }

                return (
                    <AgentFeedCard
                        key={draft.draft_id}
                        draft={draft}
                        onApprove={handleApprove}
                        onEdit={handleEdit}
                    />
                );
            })}
        </div>
    );
};
