"use client";

import React from 'react';
import { useRouter } from 'next/navigation';

export default function ConnectedAccountsStandingAuthorityArticle() {
  const router = useRouter();

  return (
    <div className="min-h-screen bg-[#F5F5F7] py-12 px-4 sm:px-6 lg:px-8 font-inter">
      <div className="max-w-4xl mx-auto">
        <div className="app-card backdrop-blur-[30px] saturate-[210%] bg-white/65 dark:bg-[#16161a]/70 border border-white/40 dark:border-white/10 p-8 sm:p-12 rounded-3xl shadow-[0_4px_24px_rgba(0,0,0,0.04)]">
          <h1 className="text-3xl sm:text-4xl font-extrabold font-outfit text-[#1D1D1F] mb-6 tracking-tight">
            Connected Accounts and Standing Authority
          </h1>
          <div className="prose prose-lg dark:prose-invert">
            <h2>Setup</h2>
            <p>To set up your business, you need to configure your connected accounts and establish your standing authority. This ensures that the AI team can operate safely and securely on your behalf.</p>

            <h2>Connected Accounts</h2>
            <ul>
              <li>Link your Google Workspace and Stripe accounts.</li>
              <li>Verify your connection settings to ensure the AI team can access your tools.</li>
            </ul>

            <h2>Standing Authority</h2>
            <ul>
              <li>Define the scope of the AI team's authority.</li>
              <li>Set specific rules for external actions, such as sending emails or making payments.</li>
              <li>Establish a budget and spending limits for the AI team.</li>
            </ul>

            <h2>Evidence</h2>
            <ul>
              <li>Maintain a record of all actions taken by the AI team.</li>
              <li>Review the evidence feed to verify that the team is completing work as expected.</li>
            </ul>

            <h2>Cost</h2>
            <ul>
              <li>Monitor the cost of the AI team's operations.</li>
              <li>Understand the pricing structure and how usage is billed.</li>
            </ul>

            <h2>Exceptions and Recovery</h2>
            <ul>
              <li>Learn how to handle exceptions and errors that may occur during execution.</li>
              <li>Understand the recovery process and how to resume work after a failure.</li>
            </ul>
          </div>
          <div className="pt-6 border-t border-gray-200/50 mt-10">
            <button
              onClick={() => router.push('/help')}
              className="inline-flex items-center px-6 py-3 bg-white hover:bg-gray-50 text-gray-900 font-bold rounded-xl border border-gray-200 shadow-sm transition-all active:scale-95"
            >
              <svg className="w-5 h-5 mr-2 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
              Back to Help Center
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
