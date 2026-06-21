"use client";

import React, { useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function ProposalContent() {
    const searchParams = useSearchParams();
    const proposalId = searchParams.get('id');
    const [isLoading, setIsLoading] = useState(false);

    const handlePayDeposit = async () => {
        setIsLoading(true);
        // Normally this would call an API route to generate a Stripe Checkout URL
        // and then redirect the user.
        setTimeout(() => {
            alert('Redirecting to Stripe for deposit payment...');
            setIsLoading(false);
        }, 1000);
    };

    return (
        <div style={{ maxWidth: '375px', margin: '0 auto', padding: '20px', fontFamily: 'sans-serif' }}>
            <h1 style={{ fontSize: '24px', fontWeight: 'bold' }}>Your Quote</h1>
            <p style={{ color: '#666' }}>Proposal ID: {proposalId}</p>

            <div style={{ marginTop: '20px', borderTop: '1px solid #eee', paddingTop: '20px' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '10px' }}>
                    <span>Custom Service</span>
                    <span>$150.00</span>
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', fontWeight: 'bold', fontSize: '18px', marginTop: '10px' }}>
                    <span>Total</span>
                    <span>$150.00</span>
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', color: '#0066cc', fontWeight: 'bold', marginTop: '10px' }}>
                    <span>Deposit Due</span>
                    <span>$50.00</span>
                </div>
            </div>

            <button
                onClick={handlePayDeposit}
                disabled={isLoading}
                style={{
                    width: '100%',
                    padding: '15px',
                    backgroundColor: '#000',
                    color: '#fff',
                    border: 'none',
                    borderRadius: '8px',
                    marginTop: '30px',
                    fontSize: '16px',
                    fontWeight: 'bold',
                    cursor: 'pointer'
                }}
            >
                {isLoading ? 'Processing...' : 'Pay Deposit'}
            </button>
        </div>
    );
}

export default function CustomerProposalView() {
    return (
        <Suspense fallback={<div>Loading proposal...</div>}>
            <ProposalContent />
        </Suspense>
    );
}
