"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function Referrals() {
  const router = useRouter();
  const [copied, setCopied] = useState<boolean>(false);
  const [copiedMessage, setCopiedMessage] = useState<boolean>(false);
  const [referralLink, setReferralLink] = useState<string>("ohc://join?ref=DEFAULT");

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Referral Dashboard</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
             <div className="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center text-sm font-bold text-gray-600">
                 AC
             </div>
         </div>
      </header>
      <main className="p-6 md:p-8 flex-1 max-w-4xl mx-auto w-full flex flex-col gap-8">
        <section className="mb-6 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Your Referral Link</h2>
          <div className="flex gap-2">
            <div id="referral-link" className="flex-1 bg-gray-50 border border-gray-200 rounded-lg px-3 py-2 text-sm text-gray-600">{referralLink}</div>
            <button
              onClick={() => {
                window.alert('Copied');
                setCopied(true);
                setTimeout(() => setCopied(false), 2000);
              }}
              className={`px-4 py-2 rounded-lg text-sm font-semibold transition-all ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
            >
              Copy
            </button>
          </div>
        </section>

        <section className="mb-6 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Share Tools</h2>
          <div className="flex flex-col gap-4">
            <button className="w-full flex items-center justify-center gap-2 bg-[#E1306C] text-white py-3 rounded-xl font-semibold text-sm shadow-sm hover:bg-[#C13584] transition-all">
                Share to Instagram
            </button>
            <button
              onClick={() => {
                setCopiedMessage(true);
                setTimeout(() => setCopiedMessage(false), 2000);
              }}
              className={`w-full max-w-full overflow-hidden py-3 rounded-xl text-sm font-semibold transition-all ${copiedMessage ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
            >
              Copy Invite Message
            </button>
            {copiedMessage && <div className="text-center text-sm text-green-600 mt-2">Invite message copied!</div>}
          </div>
        </section>

        <section className="mb-6 p-6 shadow-md" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
          <div className="flex gap-4">
              <button className="flex-1 py-3 rounded-xl text-sm font-semibold transition-all bg-gray-200 text-gray-800 hover:bg-gray-300">
                View Referral Logs
              </button>
              <button className="flex-1 py-3 rounded-xl text-sm font-semibold transition-all bg-gray-200 text-gray-800 hover:bg-gray-300">
                Export Data
              </button>
          </div>
        </section>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
