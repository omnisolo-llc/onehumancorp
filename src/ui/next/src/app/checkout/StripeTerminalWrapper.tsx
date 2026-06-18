"use client";
import React, { useState } from 'react';
import StripeTerminalClient from '../pos/terminal/StripeTerminalClient';

interface StripeTerminalWrapperProps {
  amount: number;
  productId?: string;
  tenantId: string;
  onSuccess: () => void;
}

export default function StripeTerminalWrapper({ amount, productId, tenantId, onSuccess }: StripeTerminalWrapperProps) {
  const [showTerminal, setShowTerminal] = useState(false);

  if (showTerminal) {
    return (
      <StripeTerminalClient
        amount={amount}
        productId={productId}
        tenantId={tenantId}
        onSuccess={onSuccess}
      />
    );
  }

  return (
    <button
      onClick={() => setShowTerminal(true)}
      className="w-full px-4 py-3 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-colors shadow-sm flex items-center justify-center gap-2 mt-4"
    >
      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"></path></svg>
      Pay with Tap to Pay
    </button>
  );
}
