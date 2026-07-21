import React from 'react';
import { motion } from 'framer-motion';

export interface ActionCardProps {
    id: string;
    context: string;
    proposedAction: string;
    onApprove: (id: string) => void;
    onEdit: (id: string) => void;
    onDismiss: (id: string) => void;
}

export const ActionCard: React.FC<ActionCardProps> = ({
    id,
    context,
    proposedAction,
    onApprove,
    onEdit,
    onDismiss,
}) => {
    return (
        <motion.div
            className="w-full bg-white/70 backdrop-blur-md rounded-2xl p-4 shadow-sm border border-gray-200/50 my-2"
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95 }}
            layout
        >
            <div className="text-sm text-gray-500 mb-2 font-medium">
                Context
            </div>
            <div className="text-gray-800 mb-4 bg-gray-50/50 p-3 rounded-xl border border-gray-100">
                {context}
            </div>

            <div className="text-sm text-blue-600 mb-2 font-semibold flex items-center gap-2">
                ✨ Proposed Action
            </div>
            <div className="text-gray-900 mb-6 font-medium text-lg bg-blue-50/30 p-3 rounded-xl border border-blue-100/50">
                {proposedAction}
            </div>

            <div className="flex gap-2 w-full">
                <button
                    className="flex-1 bg-gray-100 hover:bg-gray-200 text-gray-700 py-3 rounded-xl font-medium min-h-[44px] transition-colors"
                    onClick={() => onDismiss(id)}
                >
                    Dismiss
                </button>
                <button
                    className="flex-1 bg-gray-100 hover:bg-gray-200 text-gray-700 py-3 rounded-xl font-medium min-h-[44px] transition-colors"
                    onClick={() => onEdit(id)}
                >
                    Edit
                </button>
                <button
                    className="flex-[2] bg-blue-600 hover:bg-blue-700 text-white py-3 rounded-xl font-semibold shadow-sm min-h-[44px] transition-colors"
                    onClick={() => onApprove(id)}
                >
                    Approve & Send
                </button>
            </div>
        </motion.div>
    );
};
