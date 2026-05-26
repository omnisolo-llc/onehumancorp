'use client';

import React, { useState } from 'react';
export default function ReferralsPage() {
  const [copied, setCopied] = useState(false);
  const [copiedMessage, setCopiedMessage] = useState(false);
  const referralLink = "ohc://join?ref=DEFAULT";
  const inviteMessage = `Launch your business online instantly with OHC! Use my invite link: ${referralLink}`;

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col font-inter">
      <title>Referral Dashboard</title>

      <main className="flex-1 max-w-4xl w-full mx-auto p-4 md:p-8 mt-16 md:mt-0">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-8">Referral Dashboard</h1>

        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8 mb-8 relative overflow-hidden">
          <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-50 rounded-bl-full -z-10"></div>

          <div className="max-w-2xl">
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-4">Grow Together & Earn Rewards</h2>
            <p className="text-gray-600 mb-8">
              When your friends launch their storefront on OHC, they get priority AI setup, and you earn <strong className="text-gray-900">$50 credit</strong> toward your premium tools.
            </p>

            <div className="mb-8">
              <label className="block text-sm font-semibold text-gray-700 uppercase tracking-wide mb-3">Your Referral Link</label>
              <div className="flex flex-col sm:flex-row gap-3">
                <div className="flex-1 bg-gray-50 border border-gray-200 rounded-xl px-4 py-3 flex items-center">
                  <span id="referral-link" className="text-gray-800 font-mono text-sm break-all">{referralLink}</span>
                </div>
                <button
                  onClick={() => {
                    navigator.clipboard.writeText(referralLink);
                    setCopied(true);
                    setTimeout(() => setCopied(false), 2000);
                  }}
                  className={`px-6 py-3 rounded-xl text-sm font-bold transition-all sm:w-auto w-full ${copied ? 'bg-green-100 text-green-700' : 'bg-gray-900 text-white hover:bg-black'}`}
                >
                  {copied ? 'Copied!' : 'Copy'}
                </button>
              </div>
            </div>

            <div className="border-t border-gray-100 pt-8">
              <h3 className="text-lg font-bold font-outfit text-gray-900 mb-4">Share Tools</h3>
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <button
                  onClick={() => {
                    navigator.clipboard.writeText(inviteMessage);
                    setCopiedMessage(true);
                    setTimeout(() => setCopiedMessage(false), 2000);
                  }}
                  className="flex items-center justify-center gap-2 bg-indigo-50 text-indigo-700 p-4 rounded-xl font-semibold text-sm hover:bg-indigo-100 transition-colors"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
                  {copiedMessage ? 'Invite message copied!' : 'Copy Invite Message'}
                </button>

                                <a
                  href={`https://wa.me/?text=${encodeURIComponent(inviteMessage)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-[#25D366] text-white p-4 rounded-xl font-semibold text-sm hover:bg-[#20bd5a] transition-colors"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                  WhatsApp
                </a>
                <button
                  onClick={() => window.open(`https://instagram.com/`, '_blank')}
                  className="flex items-center justify-center gap-2 bg-gradient-to-r from-purple-500 to-pink-500 text-white p-4 rounded-xl font-semibold text-sm hover:opacity-90 transition-opacity"
                >
                  Share to Instagram
                </button>
                <a
                  href={`https://twitter.com/intent/tweet?text=${encodeURIComponent(inviteMessage)}`}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center justify-center gap-2 bg-black text-white p-4 rounded-xl font-semibold text-sm shadow-sm hover:bg-gray-800 transition-all col-span-1 sm:col-span-2"
                >
                  <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                  X (Twitter)
                </a>
              </div>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mb-8">
            <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8">
                <h3 className="text-xl font-bold font-outfit text-gray-900 mb-6">Embed on Your Website</h3>
                <p className="text-sm text-gray-600 mb-4">Add a beautiful, high-converting OHC storefront widget directly to your existing website.</p>
                <div className="bg-gray-900 text-gray-300 p-4 rounded-xl font-mono text-xs overflow-x-auto mb-4">
                    <pre id="embed-code">
{`<iframe src="https://mybusiness.ohc.store/api/v1/growth/storefront/embed"
  width="100%"
  height="600"
  frameborder="0"
  style="border-radius: 12px; border: 1px solid #eaeaea;">
</iframe>`}
                    </pre>
                </div>
                <button
                  className="w-full bg-gray-100 text-gray-800 font-bold py-3 rounded-xl text-sm hover:bg-gray-200 transition-colors"
                  onClick={() => {
                      navigator.clipboard.writeText(`<iframe src="https://mybusiness.ohc.store/api/v1/growth/storefront/embed" width="100%" height="600" frameborder="0" style="border-radius: 12px; border: 1px solid #eaeaea;"></iframe>`);
                      alert("Embed code copied!");
                  }}
                >
                    Copy Embed Code
                </button>
            </div>

            <div className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 md:p-8">
               <h3 className="text-xl font-bold font-outfit text-gray-900 mb-6">Manage Data</h3>
               <p className="text-sm text-gray-600 mb-6">Track your referral performance, view recent invites, or export your growth data.</p>

               <div className="space-y-3">
                   <button className="w-full bg-white border border-gray-200 text-gray-800 font-bold py-3 rounded-xl text-sm hover:bg-gray-50 transition-colors flex items-center justify-center gap-2">
                       <svg className="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                       View Referral Logs
                   </button>
                   <button className="w-full bg-white border border-gray-200 text-gray-800 font-bold py-3 rounded-xl text-sm hover:bg-gray-50 transition-colors flex items-center justify-center gap-2">
                       <svg className="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
                       Export Data
                   </button>
               </div>
            </div>
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
