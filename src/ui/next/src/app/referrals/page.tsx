'use client';
import { useState } from 'react';

export default function Referrals() {
  const [copied, setCopied] = useState(false);
  const [inviteCopied, setInviteCopied] = useState(false);
  const [tenantId] = useState('DEFAULT');

  const referralLink = `ohc://join?ref=${tenantId}`;
  const inviteMessage = `Join my team on OHC! Use my referral link: ${referralLink}`;

  return (
    <div className="min-h-screen bg-gray-50 p-8 flex flex-col items-center">
      <h1 className="text-3xl font-bold mb-8">Referral Dashboard</h1>
      <div className="bg-white p-6 rounded-xl shadow-md w-full max-w-2xl">
        <h2 className="text-xl font-semibold mb-4">Your Referral Link</h2>
        <div className="flex gap-4 mb-6">
          <code id="referral-link" className="flex-1 bg-gray-100 p-3 rounded text-sm break-all">
            {referralLink}
          </code>
          <button
            onClick={() => {
              navigator.clipboard.writeText(referralLink);
              setCopied(true);
              setTimeout(() => setCopied(false), 2000);
              alert('Copied');
            }}
            className="bg-indigo-600 text-white px-6 py-2 rounded font-medium hover:bg-indigo-700 transition"
          >
            Copy
          </button>
        </div>

        <div className="space-y-4">
          <button
            onClick={() => {
              navigator.clipboard.writeText(inviteMessage);
              setInviteCopied(true);
              setTimeout(() => setInviteCopied(false), 2000);
            }}
            className="w-full bg-gray-900 text-white px-4 py-3 rounded-lg font-medium hover:bg-gray-800 transition"
          >
            Copy Invite Message
          </button>
          {inviteCopied && <p className="text-green-600 text-sm text-center">Invite message copied!</p>}

          <div className="flex gap-4 pt-4 border-t">
            <button className="flex-1 border border-gray-300 px-4 py-2 rounded hover:bg-gray-50 transition">
              View Referral Logs
            </button>
            <button className="flex-1 border border-gray-300 px-4 py-2 rounded hover:bg-gray-50 transition">
              Export Data
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
