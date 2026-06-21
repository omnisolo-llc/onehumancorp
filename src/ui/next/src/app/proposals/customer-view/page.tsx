"use client";

import React, { useState, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function CustomerProposalViewContent() {
    const searchParams = useSearchParams();
    const proposalId = searchParams.get('id');
    const [isLoading, setIsLoading] = useState(false);

    const handlePayDeposit = async () => {
        setIsLoading(true);
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
                <div style={{ display: 'flex', justifyContent: 'space-between', fontWeight: 'bold', marginTop: '20px' }}>
                    <span>Total</span>
                    <span>$150.00</span>
                </div>
                <div style={{ display: 'flex', justifyContent: 'space-between', fontWeight: 'bold', marginTop: '10px', color: '#0066FF' }}>
                    <span>Required Deposit</span>
                    <span>$50.00</span>
                </div>
            </div>

            <button
                onClick={handlePayDeposit}
                disabled={isLoading}
                style={{
                    width: '100%',
                    backgroundColor: '#0066FF',
                    color: 'white',
                    padding: '15px',
                    borderRadius: '8px',
                    border: 'none',
                    fontWeight: 'bold',
                    marginTop: '30px',
                    opacity: isLoading ? 0.7 : 1
                }}
            >
                {isLoading ? 'Processing...' : 'Pay $50 Deposit'}
            </button>
        </div>
    );
}

export default function CustomerProposalView() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <CustomerProposalViewContent />
    </Suspense>
  );
}
