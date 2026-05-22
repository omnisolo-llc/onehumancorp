'use client';

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function ReferralDashboard() {
  const router = useRouter();
  const [copiedLink, setCopiedLink] = useState(false);
  const [copiedMessage, setCopiedMessage] = useState(false);
  const referralLink = 'ohc://join?ref=DEFAULT';
  const inviteMessage = `Launch your business online instantly with OHC! Use my invite link: ${referralLink}`;

  const handleCopyLink = () => {
    navigator.clipboard.writeText(referralLink);
    setCopiedLink(true);
    setTimeout(() => setCopiedLink(false), 2000);
  };

  const handleCopyMessage = () => {
    navigator.clipboard.writeText(inviteMessage);
    setCopiedMessage(true);
    setTimeout(() => setCopiedMessage(false), 2000);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <header className="bg-white border-b px-6 py-4 flex items-center justify-between">
        <nav className="flex items-center gap-4">
          <button onClick={() => router.back()} className="text-gray-600 hover:text-gray-900">&lt; Back</button>
          <a href="/agents" className="text-sm font-medium text-gray-600 hover:text-gray-900">Agents</a>
          <a href="/dashboard" className="text-sm font-medium text-gray-600 hover:text-gray-900">Dashboard</a>
        </nav>
        <h1 className="text-xl font-semibold text-gray-900 font-outfit">Referral Dashboard</h1>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">
        <section className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h2 className="text-lg font-semibold mb-4">Your Referral Link</h2>
          <div className="flex gap-4 items-center">
            <div id="referral-link" className="flex-1 bg-gray-50 border border-gray-200 rounded-lg px-4 py-3 text-gray-700 font-mono text-sm">
              {referralLink}
            </div>
            <button
              onClick={handleCopyLink}
              className="bg-indigo-600 hover:bg-indigo-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
            >
              {copiedLink ? 'Copied!' : 'Copy'}
            </button>
          </div>
        </section>

        <section className="bg-white p-6 rounded-2xl shadow-sm border border-gray-100">
          <h2 className="text-lg font-semibold mb-4">Invite Message</h2>
          <div className="bg-gray-50 border border-gray-200 rounded-lg p-4 mb-4 text-gray-700 text-sm">
            {inviteMessage}
          </div>
          <button
            onClick={handleCopyMessage}
            className="w-full sm:w-auto bg-gray-900 hover:bg-black text-white px-6 py-3 rounded-lg font-medium transition-colors"
          >
            Copy Invite Message
          </button>
          {copiedMessage && <p className="mt-2 text-green-600 text-sm font-medium">Invite message copied!</p>}
        </section>

        <section className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <button className="bg-white p-4 rounded-xl border border-gray-200 hover:border-indigo-300 hover:shadow-md transition-all font-medium text-gray-700 flex items-center justify-center gap-2">
            View Referral Logs
          </button>
          <button className="bg-white p-4 rounded-xl border border-gray-200 hover:border-indigo-300 hover:shadow-md transition-all font-medium text-gray-700 flex items-center justify-center gap-2">
            Export Data
          </button>
        </section>
      </main>
    </div>
  );
}
