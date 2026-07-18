"use client";

import React, { useState } from 'react';
import Link from 'next/link';

export function AIFeaturePaywallWidget() {
  const [referralLink, setReferralLink] = useState("");
  const [generating, setGenerating] = useState(false);
  const [copied, setCopied] = useState(false);
  const [referralError, setReferralError] = useState<string | null>(null);

  const handleGenerateLink = async () => {
    setGenerating(true);
    setReferralError(null);
    try {
      const response = await fetch('/api/v1/growth/referrals/generate', { method: 'POST' });
      if (!response.ok) throw new Error('Referral link generation is unavailable.');
      const data = await response.json();
      if (typeof data.referral_link !== 'string' || !data.referral_link) throw new Error('Referral link generation is unavailable.');
      setReferralLink(data.referral_link);
    } catch {
      setReferralError('Referral link generation is unavailable.');
    } finally {
      setGenerating(false);
    }
  };

  const handleCopy = () => {
    if (navigator.clipboard && referralLink) {
      navigator.clipboard.writeText(referralLink);
      setCopied(true);

      setTimeout(() => setCopied(false), 2000);
    }
  };

  const shareText = `Start your business on OHC! Use my link to get $50 off your first month: ${referralLink}`;

  const handleWhatsAppShare = () => {
     window.open(`https://wa.me/?text=${encodeURIComponent(shareText)}`, '_blank');
  };

  const handleTwitterShare = () => {
     window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`, '_blank');
  };

  return (
    <div className="mb-6 ohc-growth-card rounded-[12px] bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm p-6 border border-indigo-200/60 dark:border-indigo-800/60 bg-white/40 dark:bg-black/20 backdrop-blur-[30px] saturate-[210%] relative overflow-hidden" data-testid="ai-feature-paywall">
      {/* Blurred "premium" background effect */}
      <div className="absolute inset-0 pointer-events-none opacity-10 bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCI+CjxjaXJjbGUgY3g9IjIiIGN5PSIyIiByPSIyIiBmaWxsPSIjNGE0ZWRlIiBvcGFjaXR5PSIwLjQiLz4KPC9zdmc+')]"></div>

      <div className="relative z-10 flex flex-col md:flex-row items-center gap-6 justify-between">
        <div className="flex-1 text-center md:text-left">
          <div className="inline-flex items-center gap-2 mb-3 px-3 py-1 rounded-full bg-indigo-100 dark:bg-indigo-900/50 text-indigo-700 dark:text-indigo-300 text-xs font-bold uppercase tracking-wider border border-indigo-200 dark:border-indigo-800">
             <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" /></svg>
             Pro Feature
          </div>
          <h2 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
            Advanced AI Analytics
          </h2>
          <p className="text-sm text-gray-600 dark:text-gray-300 mb-4 max-w-xl">
            See exactly which products are driving revenue, automate cross-selling, and get daily AI strategy briefings.
          </p>

          <div className="flex flex-col sm:flex-row gap-4 items-center">
             <Link href="/pricing" className="w-full sm:w-auto px-6 py-2.5 bg-gray-900 dark:bg-white text-white dark:text-gray-900 font-semibold rounded-xl hover:bg-black dark:hover:bg-gray-100 transition-colors shadow-sm text-center">
                Upgrade to Pro ($79/mo)
             </Link>
             <span className="text-gray-400 font-medium text-sm">or</span>

             {!referralLink ? (
                 <button
                   onClick={handleGenerateLink}
                   disabled={generating}
                   className="w-full sm:w-auto px-6 py-2.5 bg-indigo-600 text-white font-semibold rounded-xl hover:bg-indigo-700 transition-colors shadow-sm disabled:opacity-70 flex justify-center items-center gap-2"
                 >
                   {generating ? 'Generating...' : 'Generate Referral Link'}
                 </button>
             ) : (
                 <div className="flex flex-col sm:flex-row gap-2 w-full sm:w-auto">
                    <button
                       onClick={handleCopy}
                       className={`px-4 py-2 text-sm font-bold rounded-lg transition-all flex items-center justify-center gap-2 ${copied ? 'bg-green-100 text-green-700' : 'bg-indigo-50 text-indigo-700 hover:bg-indigo-100 border border-indigo-200'}`}
                    >
                       {copied ? 'Copied Link!' : 'Copy Link'}
                    </button>
                    <button
                      onClick={handleWhatsAppShare}
                      className="px-4 py-2 bg-[#25D366] hover:bg-[#1ebd5a] text-white rounded-lg font-bold text-sm shadow-sm transition-all flex items-center justify-center gap-2"
                    >
                      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                      WhatsApp
                    </button>
                    <button
                      onClick={handleTwitterShare}
                      className="px-4 py-2 bg-black hover:bg-gray-800 text-white rounded-lg font-bold text-sm shadow-sm transition-all flex items-center justify-center gap-2"
                    >
                      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/></svg>
                      Share on X
                    </button>
                 </div>
             )}
             {referralError && <p className="text-sm text-red-600" role="status">{referralError}</p>}
          </div>
          <p className="mt-3 text-xs text-gray-500">Referral rewards are applied only after backend verification.</p>
        </div>
      </div>
    </div>
  );
}
