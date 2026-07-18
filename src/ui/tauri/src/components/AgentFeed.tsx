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
            const response = await fetch('/api/v1/inbox/action_required');
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
            const response = await fetch(`/api/v1/inbox/action_required/${id}/approve`, {
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

    const handleEdit = async (id: string, newResponse: string) => {
        try {
            // Update the draft in the UI optimistically
            setDrafts(drafts.map(d => d.draft_id === id ? { ...d, response: newResponse } : d));

            const response = await fetch(`/api/v1/inbox/action_required/${id}`, {
                method: 'PATCH',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ response: newResponse }),
            });

            if (!response.ok) {
                console.error("Failed to update draft");
                // Rollback could be implemented here if needed, but optimistic UI is fine for now
            }
        } catch (err) {
            console.error(err);
        }
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
                <div className="w-full bg-red-500/10 border border-red-500/20 text-red-600 dark:text-red-400 p-4 rounded-[16px] shadow-sm flex flex-col gap-2">
                    <span className="font-medium">Failed to load feed</span>
                    <span className="text-sm opacity-80">{error}</span>
                    <button onClick={fetchDrafts} className="mt-2 text-sm underline hover:opacity-80 self-start">Try again</button>
                </div>
            </div>
        );
    }

    if (drafts.length === 0) {
        return (
            <div className="w-full max-w-[375px] mx-auto p-4 flex flex-col items-center">
                <h2 className="text-xl font-semibold mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] self-start">Agent Feed</h2>
                <div className="w-full bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-6 rounded-[16px] shadow-sm flex flex-col items-center justify-center gap-2">
                    <div className="w-12 h-12 rounded-full bg-black/5 dark:bg-white/5 flex items-center justify-center mb-2">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-[#1D1D1F]/40 dark:text-[#F5F5F7]/40">
                            <path d="M22 12h-4l-3 9L9 3l-3 9H2" />
                        </svg>
                    </div>
                    <span className="font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">All caught up</span>
                    <span className="text-sm text-[#1D1D1F]/60 dark:text-[#F5F5F7]/60 text-center">No pending actions required at this time.</span>
                </div>
            </div>
        );
    }

    return (
        <div className="w-full max-w-[375px] mx-auto p-4 flex flex-col items-center">
            <h2 className="text-xl font-semibold mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] self-start">Agent Feed</h2>
            {drafts.map((draft) => {
                const actionType = draft.action_type || draft.status;

                if (actionType === 'WeeklyPayoutSummary' || actionType === 'weekly_payout_summary') {
                    const parsedPayload = draft.proposed_action || draft.context_payload || {};
                    return (
                        <div key={draft.draft_id} className="w-full mb-4">
                            <PayoutSummaryCard
                                amount={parsedPayload.amount || 0}
                                periodStart={parsedPayload.period_start || ''}
                                periodEnd={parsedPayload.period_end || ''}
                                transactionCount={parsedPayload.transaction_count || 0}
                                onAcknowledge={() => handleApprove(draft.draft_id)}
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
                                onEdit={() => handleEdit(draft.draft_id, "Edited quote placeholder")}
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
