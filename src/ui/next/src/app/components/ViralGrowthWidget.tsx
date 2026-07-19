import React, { useState } from 'react';
import Link from 'next/link';

export default function ViralGrowthWidget({
  tenantId = 'ohc',
}: {
  tenantId?: string;
}) {
  const [copied, setCopied] = useState(false);

  const referralLink = `https://ohc.app/join/${tenantId}`;

  const handleCopy = () => {
    navigator.clipboard.writeText(referralLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="ohc-growth-card flex flex-col items-center justify-center text-center">
      <div className="w-16 h-16 bg-gradient-to-br from-indigo-100 to-purple-100 dark:from-indigo-900/30 dark:to-purple-900/30 rounded-2xl flex items-center justify-center mb-4 shadow-sm border border-indigo-200 dark:border-indigo-800">
        <svg className="w-8 h-8 text-indigo-600 dark:text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
        </svg>
      </div>

      <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
        Invite Your Network
      </h3>

      <p className="text-sm text-gray-600 dark:text-gray-300 mb-6 max-w-sm">
        Share your referral link with another business owner. Reward eligibility is determined by the referral service.
      </p>

      <div className="flex w-full max-w-md items-center gap-2 bg-white dark:bg-black/40 p-2 rounded-xl border border-gray-200 dark:border-gray-800 mb-6">
        <span className="flex-1 text-sm font-mono text-gray-500 truncate px-2 select-all">
          {referralLink}
        </span>
        <button
          onClick={handleCopy}
          className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm font-semibold transition-colors shadow-sm whitespace-nowrap"
        >
          {copied ? 'Copied!' : 'Copy Link'}
        </button>
      </div>

      <div className="grid grid-cols-2 w-full gap-3 max-w-md">
        <a
          href={`https://twitter.com/intent/tweet?text=I%20run%20my%20business%20on%20OHC.%20Join%20me!&url=${encodeURIComponent(referralLink)}`}
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center justify-center gap-2 py-3 bg-[#000000] hover:bg-gray-800 text-white rounded-xl text-sm font-bold transition-all shadow-sm"
        >
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
          Post
        </a>
        <a
          href={`https://wa.me/?text=${encodeURIComponent(`I run my business on OHC. Join me! ${referralLink}`)}`}
          target="_blank"
          rel="noopener noreferrer"
          className="flex items-center justify-center gap-2 py-3 bg-[#25D366] hover:bg-[#20bd5a] text-white rounded-xl text-sm font-bold transition-all shadow-sm"
        >
          <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
          Share
        </a>
      </div>
    </div>
  );
}
