"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function TestimonialCollectorPage() {
  const router = useRouter();
  const [tenantId, setTenantId] = useState('DEFAULT');
  const [referralLink, setReferralLink] = useState('');
  const [collectionLink, setCollectionLink] = useState('');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const tenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'DEFAULT';
      setTenantId(tenant);

      const host = typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app';
      // The collection link sends the customer to a mock submission route, which inherently features the watermark loop
      setCollectionLink(`${host}/embed/testimonial?tenant=${encodeURIComponent(tenant)}`);
      setReferralLink(`${host}/onboarding?ref=${encodeURIComponent(tenant)}`);
    }
  }, []);

  const handleCopy = () => {
    if (collectionLink) {
      navigator.clipboard.writeText(collectionLink);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleWhatsApp = () => {
    if (collectionLink) {
      const message = `Hi! Thanks for choosing us. We'd love it if you could leave a quick testimonial here: ${collectionLink}`;
      const url = `https://wa.me/?text=${encodeURIComponent(message)}`;
      window.open(url, '_blank');
    }
  };

  const handleTwitter = () => {
    if (collectionLink) {
      const message = `Help us grow! If you enjoyed our service, please leave a quick review: ${collectionLink} \n\n⚡ Powered by OHC`;
      const url = `https://twitter.com/intent/tweet?text=${encodeURIComponent(message)}`;
      window.open(url, '_blank');
    }
  };

  return (
      <div className="w-full max-w-4xl mx-auto space-y-6">
        <div>
          <h1 className="text-2xl font-bold text-gray-900 tracking-tight">Testimonial Collector</h1>
          <p className="text-gray-500 text-sm mt-1">Gather reviews and earn referrals.</p>
        </div>

        <div className="app-card p-6 border border-blue-100 bg-blue-50/30 rounded-2xl shadow-sm">
          <div className="flex items-center gap-2 mb-4">
            <div className="w-10 h-10 rounded-full bg-blue-100 flex items-center justify-center">
              <span className="text-xl">⭐️</span>
            </div>
            <div>
              <h2 className="text-lg font-bold text-gray-900 leading-tight">Your Collection Link</h2>
              <p className="text-sm text-gray-600">Send this to your customers</p>
            </div>
          </div>

          <div className="space-y-3">
             <div className="flex items-center gap-2 bg-white p-2 rounded-xl border border-gray-200">
               <input id="collection-link"
                 type="text"
                 readOnly
                 value={collectionLink}
                 className="bg-transparent border-none outline-none text-sm w-full text-gray-700 px-2"
               />
               <button
                 onClick={handleCopy}
                 className="px-4 py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 text-sm font-semibold rounded-lg transition-colors"
               >
                 {copied ? 'Copied!' : 'Copy'}
               </button>
             </div>
             <button
               onClick={handleWhatsApp}
               className="w-full py-3 px-4 rounded-xl font-bold text-sm bg-[#25D366] text-white hover:bg-[#1ebd5a] shadow-sm active:scale-[0.98] transition-all flex items-center justify-center gap-2"
             >
               Send via WhatsApp
             </button>
             <button
               onClick={handleTwitter}
               className="w-full py-3 px-4 rounded-xl font-bold text-sm bg-black text-white hover:bg-gray-800 shadow-sm active:scale-[0.98] transition-all flex items-center justify-center gap-2"
             >
               Share on X (Twitter)
             </button>
          </div>
        </div>

        <div className="app-card p-6 border border-gray-100 rounded-2xl">
           <h3 className="text-lg font-bold text-gray-900 mb-2">How the Growth Loop Works</h3>
           <p className="text-sm text-gray-600 mb-4">
             Your testimonial collection page includes a discreet <span className="font-semibold text-gray-800">"⚡ Powered by OHC"</span> watermark at the bottom.
           </p>
           <div className="bg-gray-50 rounded-xl p-4 border border-gray-200 space-y-3">
              <div className="flex items-start gap-3">
                <div className="w-6 h-6 shrink-0 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center text-xs font-bold">1</div>
                <p className="text-sm text-gray-700 leading-tight pt-0.5">Customer leaves you a 5-star review.</p>
              </div>
              <div className="flex items-start gap-3">
                <div className="w-6 h-6 shrink-0 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center text-xs font-bold">2</div>
                <p className="text-sm text-gray-700 leading-tight pt-0.5">They notice the fast, elegant OHC experience.</p>
              </div>
              <div className="flex items-start gap-3">
                <div className="w-6 h-6 shrink-0 rounded-full bg-blue-100 text-blue-700 flex items-center justify-center text-xs font-bold">3</div>
                <p className="text-sm text-gray-700 leading-tight pt-0.5">They click the watermark and sign up using your embedded referral tag, earning you a $50 credit!</p>
              </div>
           </div>
        </div>

        <div className="app-card p-6 border border-gray-100 rounded-2xl text-center">
            <h3 className="text-lg font-bold text-gray-900 mb-1">Preview Your Watermark</h3>
            <p className="text-sm text-gray-500 mb-4">This is what your customers will see at the bottom of the form.</p>
            <div className="pt-4 border-t border-gray-100 inline-block w-full">
                <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${encodeURIComponent(tenantId)}`} target="_blank" rel="noopener noreferrer" className="inline-flex items-center gap-1 text-sm font-semibold text-gray-500 hover:text-indigo-600 transition-colors">
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
                    Powered by OHC
                </a>
            </div>
        </div>

      </div>
  );
}
