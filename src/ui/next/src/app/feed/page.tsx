"use client";

import React from 'react';
import { ProposalDraftCard } from '../../components/feed/ProposalDraftCard';

export default function FeedPage() {
    return (
        <div style={{ maxWidth: '375px', margin: '0 auto', padding: '20px' }}>
            <h1>Your Feed</h1>
            <ProposalDraftCard />
        </div>
    );
}
