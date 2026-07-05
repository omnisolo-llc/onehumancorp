import React, { useEffect, useState } from 'react';
import { AgentFeedCard, ActionRequiredDraft } from './AgentFeedCard';

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
            const response = await fetch(\`/api/inbox/action_required/\${id}/approve\`, {
                method: 'POST',
            });
            if (response.ok) {
                setDrafts(drafts.filter(d => d.draft_id !== id));
            } else {
                setError("Failed to approve draft");
            }
        } catch (err: any) {
            setError(err.message);
        }
    };

    const handleEdit = async (id: string) => {
        const draftToEdit = drafts.find(d => d.draft_id === id);
        if (draftToEdit) {
            const newResponse = window.prompt("Edit draft response:", draftToEdit.response);
            if (newResponse !== null && newResponse !== draftToEdit.response) {
                try {
                    const response = await fetch(\`/api/inbox/action_required/\${id}\`, {
                        method: 'PATCH',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({ response: newResponse }),
                    });
                    if (response.ok) {
                        setDrafts(drafts.map(d =>
                            d.draft_id === id ? { ...d, response: newResponse } : d
                        ));
                    } else {
                        setError("Failed to update draft");
                    }
                } catch (err: any) {
                    setError(err.message);
                }
            }
        }
    };

    if (loading) return <div className="p-4 text-center">Loading feed...</div>;
    if (error) return <div className="p-4 text-center text-[#FF3B30]">{error}</div>;
    if (drafts.length === 0) return <div className="p-4 text-center text-gray-500">No pending actions!</div>;

    return (
        <div className="w-full max-w-[375px] mx-auto p-4 flex flex-col items-center">
            <h2 className="text-xl font-semibold mb-4 text-[#1D1D1F] dark:text-[#F5F5F7] self-start">Agent Feed</h2>
            {drafts.map(draft => (
                <AgentFeedCard
                    key={draft.draft_id}
                    draft={draft}
                    onApprove={handleApprove}
                    onEdit={handleEdit}
                />
            ))}
        </div>
    );
};
