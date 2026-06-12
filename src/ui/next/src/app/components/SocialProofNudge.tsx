"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

interface Order {
  id: string;
  total_amount: number;
  status: string;
  created_at: string;
}

export function SocialProofNudge({ tenantId }: { tenantId: string }) {
  const [orders, setOrders] = useState<Order[]>([]);
  const [currentOrderIndex, setCurrentOrderIndex] = useState(0);
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    async function fetchOrders() {
      try {
        const res = await fetch(`/api/v1/growth/social-proof?tenant_id=${tenantId}`);
        if (res.ok) {
          const data = await res.json();
          if (data.orders && data.orders.length > 0) {
            setOrders(data.orders);
          }
        }
      } catch (err) {
        console.error("Failed to fetch social proof orders", err);
      }
    }
    fetchOrders();
  }, [tenantId]);

  useEffect(() => {
    if (orders.length === 0) return;

    // Wait a bit before showing the first nudge
    const initialTimer = setTimeout(() => setIsVisible(true), 3000);

    // Rotate through orders every 8 seconds, showing for 5 seconds
    const interval = setInterval(() => {
      setIsVisible(false);
      setTimeout(() => {
        setCurrentOrderIndex((prevIndex) => (prevIndex + 1) % orders.length);
        setIsVisible(true);
      }, 500); // 500ms fade out before showing the next one
    }, 8000);

    return () => {
      clearTimeout(initialTimer);
      clearInterval(interval);
    };
  }, [orders]);

  if (orders.length === 0) return null;

  const currentOrder = orders[currentOrderIndex];

  return (
    <div
      className={`fixed bottom-4 left-4 z-50 transition-all duration-500 transform ${
        isVisible ? "translate-y-0 opacity-100" : "translate-y-10 opacity-0 pointer-events-none"
      }`}
      style={{ maxWidth: "320px" }}
      data-testid="social-proof-nudge"
    >
      <div className="bg-white/80 backdrop-blur-md border border-gray-200 shadow-xl rounded-xl p-4 flex flex-col gap-2">
        <div className="flex items-start gap-3">
          <div className="w-10 h-10 rounded-full bg-indigo-100 flex items-center justify-center text-xl shrink-0">
            🎉
          </div>
          <div>
            <p className="text-sm text-gray-800 font-medium font-inter leading-snug">
              Someone just bought an order for <span className="font-bold">${currentOrder.total_amount.toFixed(2)}</span>!
            </p>
            <p className="text-xs text-gray-500 mt-1">
              Verified Purchase • {new Date(currentOrder.created_at).toLocaleDateString()}
            </p>
          </div>
        </div>

        <div className="pt-2 mt-1 border-t border-gray-100">
            <Link
              href={`/onboarding?ref=${tenantId}&source=social_proof_nudge`}
              target="_blank"
              className="text-xs font-semibold text-indigo-600 hover:text-indigo-800 flex items-center gap-1 group"
              onClick={() => {
                fetch('/api/v1/growth/referrals/click', {
                  method: 'POST',
                  headers: { 'Content-Type': 'application/json' },
                  // Note: The backend expects an ID string, not referrer_id
                  body: JSON.stringify({ id: tenantId })
                }).catch(err => console.error('Failed to track referral click:', err));
              }}
            >
              ⚡ Built with OHC. <span className="group-hover:underline">Start your own store and get $50.</span>
            </Link>
        </div>

        <button
          onClick={() => setIsVisible(false)}
          className="absolute top-2 right-2 text-gray-400 hover:text-gray-600"
          aria-label="Close"
        >
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      </div>
    </div>
  );
}
