import React, { useState } from 'react';

export function OneTapReferral({ tenantId, source }: { tenantId: string, source: string }) {
  const [copied, setCopied] = useState(false);
  const referralLink = `https://ohc.store/join?ref=${tenantId}&source=${source}`;

  const copyToClipboard = () => {
    navigator.clipboard.writeText(referralLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="one-tap-referral p-4 bg-indigo-50/50 backdrop-blur-sm border border-indigo-100 rounded-xl shadow-sm text-center">
      <h3 className="font-bold font-outfit text-indigo-900 mb-1">Refer & Earn $50</h3>
      <p className="text-xs text-indigo-700 mb-3">Invite a friend to OHC and you both get rewarded!</p>
      <div className="flex gap-2 justify-center">
         <button onClick={copyToClipboard} className={`flex-1 py-2 px-3 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-white text-indigo-700 border border-indigo-200 hover:bg-indigo-50'}`}>
            {copied ? 'Copied!' : 'Copy Link'}
         </button>
         <a href={`https://wa.me/?text=${encodeURIComponent(`Start your business on OHC! Use my link: ${referralLink}`)}`} target="_blank" rel="noopener noreferrer" className="flex-1 py-2 px-3 rounded-lg text-sm font-semibold bg-[#25D366] text-white hover:bg-[#20bd5a] transition-all flex items-center justify-center gap-1">
            WhatsApp
         </a>
      </div>
    </div>
  );
}
