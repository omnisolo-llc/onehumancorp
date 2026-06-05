import React, { useState } from 'react';

export function MilestoneAlert({ pendingOrders, tenantId }: { pendingOrders: number, tenantId: string }) {
  const [copied, setCopied] = useState(false);

  // Show milestone only if they have >= 10 orders
  if (pendingOrders < 10) return null;

  const referralLink = `https://ohc.store/join?ref=${tenantId}&source=milestone`;

  const copyToClipboard = () => {
    navigator.clipboard.writeText(referralLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="milestone-alert mb-6 p-6 rounded-[16px] bg-gradient-to-r from-purple-500/10 to-pink-500/10 border border-purple-500/20 backdrop-blur-md shadow-sm">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          <div className="text-4xl">🎉</div>
          <div>
            <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white">10th Order Milestone Reached!</h3>
            <p className="text-sm text-gray-700 dark:text-gray-300">You're crushing it! Share your success and get a $50 credit when friends join.</p>
          </div>
        </div>
        <div className="flex gap-2">
          <button
            onClick={copyToClipboard}
            className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-white text-purple-700 border border-purple-200 hover:bg-purple-50'}`}
          >
            {copied ? 'Copied!' : 'Copy Link'}
          </button>
          <a
            href={`https://wa.me/?text=${encodeURIComponent(`I just hit my 10th order on OHC! Start your business here: ${referralLink}`)}`}
            target="_blank"
            rel="noopener noreferrer"
            className="px-4 py-2 rounded-lg text-sm font-semibold bg-[#25D366] text-white hover:bg-[#20bd5a] transition-all flex items-center justify-center gap-1"
          >
            WhatsApp
          </a>
        </div>
      </div>
    </div>
  );
}
