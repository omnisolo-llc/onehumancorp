"use client";

import React from 'react';
import Link from 'next/link';

// Keep the caller contract while referral programs acquire durable, operator-
// configured source receipts. A tenant hint or conversion counter grants no access.
export const ViralUpgradePaywallWidget: React.FC<{ tenantId?: string }> = () => {
  return (
    <section className="rounded-xl border border-indigo-200 bg-indigo-50/50 p-6">
      <h3 className="text-xl font-bold text-gray-900">Referral rewards</h3>
      <p role="status" className="mt-2 text-sm text-gray-600">
        Referral rewards are unavailable until a verified program is configured.
      </p>
      <Link href="/pricing" className="mt-4 inline-flex min-h-[44px] items-center rounded-xl bg-indigo-600 px-4 py-2 font-semibold text-white">
        Review plans
      </Link>
    </section>
  );
};
