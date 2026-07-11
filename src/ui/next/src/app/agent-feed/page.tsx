'use client';

import React, { useState } from 'react';

type CardStatus = 'pending' | 'processing' | 'success';

interface ActionCardProps {
  id: string;
  testId: string;
  title: string;
  description: string;
  actionText: string;
  onApprove: () => void;
}

const ActionCard: React.FC<ActionCardProps> = ({ testId, title, description, actionText, onApprove }) => {
  const [status, setStatus] = useState<CardStatus>('pending');

  const handleApprove = async () => {
    setStatus('processing');

    try {
      const response = await fetch('/api/operations/approve', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          tenantId: 'tenant-maya-123',
          actionType: testId.includes('booking') ? 'BOOKING_REQUEST' : 'INVENTORY_DEDUCTION',
          payload: {}
        }),
      });

      if (response.ok) {
        setStatus('success');
        onApprove();
      } else {
        setStatus('pending'); // Reset on failure
      }
    } catch (error) {
      setStatus('pending');
    }
  };

  let containerClass = "p-4 rounded-xl border border-white/20 shadow-lg relative overflow-hidden transition-all duration-300";
  // OHC Premium Token library constraint: Translucent Glass styling
  let bgClass = "bg-white/10 backdrop-blur-md";

  if (status === 'success') {
    containerClass += " status-success";
    bgClass = "bg-green-500/20 backdrop-blur-md border-green-400/30";
  } else if (status === 'processing') {
    containerClass += " status-processing";
    // Shimmer effect placeholder logic
    bgClass = "bg-white/15 backdrop-blur-md border-white/40 animate-pulse";
  }

  return (
    <div data-testid={testId} className={`${containerClass} ${bgClass} w-full max-w-sm mb-4 mx-auto`} style={{ backdropFilter: 'blur(12px)', backgroundColor: status === 'success' ? 'rgba(34, 197, 94, 0.2)' : 'rgba(255, 255, 255, 0.1)' }}>
      <h3 className="text-lg font-semibold text-white mb-2">{title}</h3>
      <p className="text-sm text-gray-200 mb-4">{description}</p>

      {status === 'pending' && (
        <div className="flex justify-between gap-2">
          <button
            onClick={handleApprove}
            className="flex-1 bg-white/20 hover:bg-white/30 text-white font-medium py-2 px-4 rounded-lg transition-colors backdrop-blur-sm"
          >
            {actionText}
          </button>
          <button className="bg-transparent border border-white/20 text-white font-medium py-2 px-4 rounded-lg hover:bg-white/5 transition-colors">
            Edit
          </button>
        </div>
      )}

      {status === 'processing' && (
        <div className="flex items-center justify-center py-2 text-white/80 text-sm font-medium status-text">
          Processing operation...
        </div>
      )}

      {status === 'success' && (
        <div className="flex items-center justify-center py-2 text-green-400 font-medium">
          <span className="status-text">{actionText === "Approve" && testId.includes("booking") ? "Confirmed" : "Deducted"}</span>
        </div>
      )}
    </div>
  );
};

export default function AgentFeedPage() {
  return (
    <div className="min-h-screen bg-gray-900 p-4 md:p-8 font-sans">
      <div className="max-w-md mx-auto">
        <header className="mb-8 mt-4">
          <h1 className="text-2xl font-bold text-white tracking-tight">Agent Feed</h1>
          <p className="text-gray-400 text-sm">Your autonomous operations manager</p>
        </header>

        <div className="space-y-4">
          <ActionCard
            id="1"
            testId="action-card-booking-intent"
            title="Booking Request"
            description="Maya, I received a DM from Sarah for a custom vegan cake. I have checked inventory (available) and drafted a quote for $45. Would you like to secure the Friday slot?"
            actionText="Approve"
            onApprove={() => console.log('Booking approved')}
          />

          <ActionCard
            id="2"
            testId="action-card-inventory-intent"
            title="Inventory Adjustment"
            description="Two units of 'cake-1' were sold offline via manual POS entry. Should I deduct them from central inventory to keep everything synced?"
            actionText="Approve"
            onApprove={() => console.log('Inventory deducted')}
          />
        </div>
      </div>
    </div>
  );
}
