"use client";

import { useState } from 'react';
import CapitalOfferCard from './CapitalOfferCard';
import RepaymentTracker from './RepaymentTracker';

export default function CapitalPage() {
  const [offerAccepted, setOfferAccepted] = useState(false);

  // Mock data for the demo
  const offerAmount = 300;
  const flatFee = 30;
  const totalOwed = offerAmount + flatFee;

  return (
    <div className="min-h-screen bg-gray-50 flex items-center justify-center p-4">
      <div className="w-full max-w-[375px] mx-auto space-y-6">
        {!offerAccepted ? (
          <CapitalOfferCard
            amount={offerAmount}
            fee={flatFee}
            onAccept={() => setOfferAccepted(true)}
          />
        ) : (
          <RepaymentTracker
            totalOwed={totalOwed}
            amountRepaid={150} // Mock repaid amount for UI display
          />
        )}
      </div>
    </div>
  );
}
