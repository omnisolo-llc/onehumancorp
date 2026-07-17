"use client";

import React, { useState, useEffect, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';

function CustomerProposalViewContent() {
    const searchParams = useSearchParams();
    const proposalId = searchParams.get('id');
    const [isLoading, setIsLoading] = useState(false);
    const [proposal, setProposal] = useState<any>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        if (!proposalId) return;
        async function fetchProposal() {
            try {
                const res = await fetch(`/api/v1/proposals/${proposalId}`);
                if (!res.ok) throw new Error('Failed to fetch proposal');
                const data = await res.json();
                setProposal(data);
            } catch (err: any) {
                setError(err.message);
            }
        }
        fetchProposal();
    }, [proposalId]);

    const handleAccept = async () => {
        if (!proposalId) return;
        setIsLoading(true);
        try {
            const res = await fetch(`/api/v1/proposals/${proposalId}/accept`, { method: 'POST' });
            if (!res.ok) throw new Error('Failed to accept proposal');
            const data = await res.json();
            if (data.stripe_payment_link) {
                window.location.href = data.stripe_payment_link;
            } else {
                alert('Proposal Accepted!');
                window.location.reload();
            }
        } catch (err: any) {
            alert(err.message);
        } finally {
            setIsLoading(false);
        }
    };

    if (error) return <div className="p-4 text-center text-[#FF3B30]">{error}</div>;
    if (!proposal) return <div className="p-4 text-center">Loading...</div>;

    return (
        <div style={{ maxWidth: '375px', margin: '0 auto', padding: '20px', fontFamily: 'sans-serif' }}>
            <h1 style={{ fontSize: '24px', fontWeight: 'bold' }}>Your Proposal</h1>
            <p style={{ color: '#666' }}>Proposal ID: {proposalId}</p>

            <div style={{ marginTop: '20px', borderTop: '1px solid #eee', paddingTop: '20px' }}>
                {proposal.line_items?.map((item: any) => (
                    <div key={item.id} style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '10px' }}>
                        <span>{item.description} (x{item.quantity})</span>
                        <span>${(item.unit_price_cents * item.quantity / 100).toFixed(2)}</span>
                    </div>
                ))}

                <div style={{ display: 'flex', justifyContent: 'space-between', fontWeight: 'bold', marginTop: '20px' }}>
                    <span>Total</span>
                    <span>${(proposal.proposal.total_amount_cents / 100).toFixed(2)}</span>
                </div>
            </div>

            {proposal.proposal.status !== 'ACCEPTED' ? (
                <button
                    onClick={handleAccept}
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
                        opacity: isLoading ? 0.7 : 1,
                        cursor: 'pointer'
                    }}
                >
                    {isLoading ? 'Processing...' : 'Approve & Pay Invoice'}
                </button>
            ) : (
                <div style={{ marginTop: '30px', padding: '15px', backgroundColor: '#e6f4ea', color: '#137333', borderRadius: '8px', textAlign: 'center', fontWeight: 'bold' }}>
                    Proposal Accepted
                </div>
            )}
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
