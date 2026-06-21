"use client";

import React, { useState } from 'react';

export function ProposalDraftCard() {
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [isApproved, setIsApproved] = useState(false);

    if (isApproved) {
        return null;
    }

    return (
        <>
            <div className="ohc-feed-card" style={{ padding: '15px', border: '1px solid #ccc', borderRadius: '8px', marginBottom: '15px', cursor: 'pointer' }} onClick={() => setIsModalOpen(true)}>
                <p>Carlos, new repair request from John. I drafted a quote for $150 with a $50 deposit based on your standard rates.</p>
                <button style={{ marginTop: '10px' }}>Review Proposal</button>
            </div>

            {isModalOpen && (
                <div className="ohc-modal" style={{ position: 'fixed', top: 0, left: 0, right: 0, bottom: 0, backgroundColor: 'rgba(0,0,0,0.5)', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                    <div style={{ backgroundColor: 'white', padding: '20px', borderRadius: '12px', width: '300px', backdropFilter: 'blur(10px)' }}>
                        <h2>Proposal Draft</h2>
                        <div style={{ margin: '15px 0' }}>
                            <p>Line Item 1: Custom Service - $150</p>
                            <p style={{ fontWeight: 'bold' }}>Deposit Required: $50</p>
                        </div>
                        <div style={{ display: 'flex', gap: '10px' }}>
                            <button onClick={() => setIsApproved(true)} style={{ backgroundColor: 'black', color: 'white', padding: '10px', borderRadius: '5px', flex: 1 }}>Approve & Send</button>
                            <button onClick={() => setIsModalOpen(false)} style={{ padding: '10px', borderRadius: '5px', flex: 1 }}>Edit Items</button>
                        </div>
                    </div>
                </div>
            )}
        </>
    );
}
