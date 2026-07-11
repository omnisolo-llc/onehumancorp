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

    if (loading) return <div className="p-4 text-center">Loading feed...</div>;
    if (error) return <div className="p-4 text-center text-[#FF3B30]">{error}</div>;
    if (drafts.length === 0) return <div className="p-4 text-center text-gray-500">No pending actions!</div>;

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
