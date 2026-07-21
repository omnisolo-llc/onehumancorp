'use client';

import React, { useState } from 'react';
import { ActionCard } from '../../components/feed/ActionCard';
import { motion, AnimatePresence } from 'framer-motion';

export default function UniversalFeed() {
    const [drafts, setDrafts] = useState([
        {
            id: 'draft-1',
            context: 'Customer DM on Instagram: "Do you have any vegan cakes available for pickup today?"',
            proposedAction: 'Reply: "Yes, we have vegan cakes available! Would you like to order?" + Payment Link',
        },
        {
            id: 'draft-2',
            context: 'New booking request from John for Handyman Service (2 hours) next Tuesday.',
            proposedAction: 'Confirm booking and send $50 deposit invoice.',
        }
    ]);

    const handleApprove = (id: string) => {
        setDrafts(drafts.filter(d => d.id !== id));
        // In real app, call API
    };

    const handleEdit = (id: string) => {
        // Open edit modal
        console.log('Edit', id);
    };

    const handleDismiss = (id: string) => {
        setDrafts(drafts.filter(d => d.id !== id));
        // In real app, call API to reject
    };

    return (
        <div className="min-h-screen bg-gray-50 max-w-[375px] mx-auto overflow-hidden relative border-x border-gray-200">
            {/* Header */}
            <div className="sticky top-0 z-10 bg-white/80 backdrop-blur-xl border-b border-gray-200/50 p-4">
                <h1 className="text-xl font-bold text-gray-900">Agent Feed</h1>
                <p className="text-sm text-gray-500">2 items need your attention</p>
            </div>

            {/* Feed Content */}
            <div className="p-4 flex flex-col gap-2">
                <AnimatePresence>
                    {drafts.length === 0 ? (
                        <motion.div
                            className="flex flex-col items-center justify-center py-20 text-center"
                            initial={{ opacity: 0 }}
                            animate={{ opacity: 1 }}
                        >
                            <div className="text-4xl mb-4">✨</div>
                            <h2 className="text-lg font-semibold text-gray-900 mb-1">All caught up!</h2>
                            <p className="text-gray-500 text-sm">Your agents are monitoring for new tasks.</p>
                        </motion.div>
                    ) : (
                        drafts.map(draft => (
                            <ActionCard
                                key={draft.id}
                                id={draft.id}
                                context={draft.context}
                                proposedAction={draft.proposedAction}
                                onApprove={handleApprove}
                                onEdit={handleEdit}
                                onDismiss={handleDismiss}
                            />
                        ))
                    )}
                </AnimatePresence>
            </div>
        </div>
    );
}
