import React, { useState } from 'react';
import { AgentFeedCard, FeedItem } from '../AgentFeedCard';

interface DraftInvoiceActionCardProps {
    item: FeedItem;
    onApprove: (id: string) => void;
    onEdit: (id: string) => void;
    onDiscard: (id: string) => void;
}

export const DraftInvoiceActionCard: React.FC<DraftInvoiceActionCardProps> = ({ item, onApprove, onEdit, onDiscard }) => {
    const payload = item.payload || {};
    const amount = payload.amount || 0;
    const clientName = payload.clientName || 'Unknown Client';
    const channel = payload.channel || 'SMS';
    const description = payload.description || 'Service provided';

    const [isSubmitting, setIsSubmitting] = useState(false);

    const handleApprove = async () => {
        setIsSubmitting(true);
        await onApprove(item.id);
        setIsSubmitting(false);
    };

    return (
        <AgentFeedCard item={item} onDismiss={() => onDiscard(item.id)}>
            <div className="flex flex-col gap-3 p-4 bg-white/60 backdrop-blur-md rounded-xl shadow-sm border border-white/40">
                <div className="flex justify-between items-start">
                    <h3 className="text-lg font-bold text-gray-900">Draft Invoice for {clientName}</h3>
                    <span className="bg-indigo-100 text-indigo-800 text-xs px-2 py-1 rounded-full font-medium">
                        {channel}
                    </span>
                </div>

                <div className="bg-gray-50/80 rounded-lg p-3 my-2 border border-gray-100">
                    <div className="flex justify-between text-sm mb-1">
                        <span className="text-gray-500 font-medium">Details:</span>
                        <span className="text-gray-900 font-semibold">{description}</span>
                    </div>
                    <div className="flex justify-between text-sm">
                        <span className="text-gray-500 font-medium">Total:</span>
                        <span className="text-gray-900 font-bold">${amount.toFixed(2)}</span>
                    </div>
                </div>

                <div className="flex gap-2 mt-2">
                    <button
                        onClick={handleApprove}
                        disabled={isSubmitting}
                        className="flex-1 min-h-[44px] bg-indigo-600 hover:bg-indigo-700 active:bg-indigo-800 text-white font-bold rounded-lg transition-colors flex items-center justify-center disabled:opacity-70 disabled:cursor-not-allowed text-sm"
                    >
                        {isSubmitting ? 'Sending...' : 'Approve & Send'}
                    </button>
                    <button
                        onClick={() => onEdit(item.id)}
                        disabled={isSubmitting}
                        className="min-h-[44px] px-4 bg-white border border-gray-300 hover:bg-gray-50 text-gray-700 font-medium rounded-lg transition-colors flex items-center justify-center text-sm"
                    >
                        Edit
                    </button>
                    <button
                        onClick={() => onDiscard(item.id)}
                        disabled={isSubmitting}
                        className="min-h-[44px] px-4 bg-white border border-red-200 hover:bg-red-50 text-red-600 font-medium rounded-lg transition-colors flex items-center justify-center text-sm"
                    >
                        Discard
                    </button>
                </div>
            </div>
        </AgentFeedCard>
    );
};
