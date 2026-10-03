"use client";

import React from 'react';
import Link from 'next/link';

export default function ManagingQuotesInvoicesPage() {
  return (
    <div className="min-h-screen bg-[#F5F5F7] py-6 sm:py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <div className="mb-6">
          <Link href="/help">
            <button className="flex items-center text-blue-600 hover:text-blue-800 transition-colors font-medium text-sm">
              <svg className="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
              </svg>
              Back to Help Center
            </button>
          </Link>
        </div>
        <div className="bg-white/65 dark:bg-[#16161a]/70 backdrop-blur-[30px] saturate-[210%] border border-white/40 dark:border-white/10 rounded-3xl p-8 sm:p-12 shadow-[0_4px_24px_rgba(0,0,0,0.04)]">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-gray-900 dark:text-white mb-6 tracking-tight">How to Send Proposals and Collect Payments Securely</h1>

          <div className="prose prose-lg prose-blue max-w-none text-gray-700 dark:text-gray-300">
            <p>
              As a small business owner, it's critical to know that the proposals you send to your clients contain the exact scope and pricing you agreed upon, and that payment links are secure and verifiable.
            </p>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mt-8 mb-4">Current Capabilities</h2>
            <p>
              OmniSolo enforces strict controls on how proposals and invoices are generated:
            </p>
            <ul className="list-disc pl-6 space-y-2 mb-6">
              <li>
                <strong>No More Placeholder Proposals:</strong> OmniSolo ensures that the quote generated directly reflects the customer's inquiry and your explicit business rules. We validate owner-supplied line items and deposits with checked math. If pricing information is missing, the system will mark it as <code>NEEDS_PRICING</code> for your review, rather than guessing or fabricating a fixed amount. Optional, unselected items are strictly excluded from committed totals.
              </li>
              <li>
                <strong>Secure Checkout Links:</strong> When you create an invoice, OmniSolo generates a real, secure session with your connected payment provider (like Stripe). It will not generate fictitious checkout URLs. If a provider is unavailable, it will explicitly state the pending/unavailable status, ensuring you never send a broken link to a client.
              </li>
              <li>
                <strong>Drafts vs. Sent Reminders:</strong> The system clearly distinguishes between drafting a reminder and actually delivering it. We only persist actual, source-grounded drafts. Once an invoice is paid or canceled, stale drafts are automatically retired so you don't accidentally ask a customer twice.
              </li>
              <li>
                <strong>Your Authority:</strong> These actions operate under your standing authority. The system enforces your hard spend reservations, prevents unauthorized effect on your accounts, and allows you to revoke or stop actions at any time.
              </li>
            </ul>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mt-8 mb-4">How to Use This Feature</h2>
            <ol className="list-decimal pl-6 space-y-2 mb-6">
              <li>Open a <strong>Lead</strong> or <strong>Inquiry</strong> in the OmniSolo app.</li>
              <li>Click <strong>Generate Proposal</strong>. Review the line items carefully. If any items are marked <code>NEEDS_PRICING</code>, fill in the correct amounts.</li>
              <li><strong>Approve</strong> the proposal to finalize the scope and price.</li>
              <li>When ready, click <strong>Create Invoice</strong>. OmniSolo will securely connect to your payment provider to generate a verifiable checkout link.</li>
              <li>The resulting email/SMS draft will be placed in your outbox for final review before sending.</li>
            </ol>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mt-8 mb-4">Cost & Expectations</h2>
            <p>
              These actions use your regular OHC subscription. There are no hidden markup fees on your customer's invoice. Note that standard payment processor fees (like Stripe's transaction fee) still apply and are managed directly in your provider dashboard.
            </p>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 dark:text-white mt-8 mb-4">Exceptions and Recovery</h2>
            <p>
              If your payment provider disconnects or fails to create a link, the invoice status will remain "Draft/Pending Provider". You can simply try generating the link again later; the system is designed to retry safely without creating duplicate charges.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
