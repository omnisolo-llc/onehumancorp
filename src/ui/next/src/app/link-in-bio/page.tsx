'use client';

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function LinkInBioPage() {
  const [tenant, setTenant] = useState<string>('my-store');
  const [links, setLinks] = useState([
    { title: 'Shop My Storefront', url: 'https://ohc.store/my-store' },
    { title: 'Book a Consultation', url: 'https://ohc.store/my-store/book' },
    { title: 'Follow on Instagram', url: 'https://instagram.com/my-store' }
  ]);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant');
      if (storedTenant) {
        setTenant(storedTenant);
        setLinks([
          { title: 'Shop My Storefront', url: `https://ohc.store/${storedTenant}` },
          { title: 'Book a Consultation', url: `https://ohc.store/${storedTenant}/book` },
          { title: 'Follow on Instagram', url: `https://instagram.com/${storedTenant}` }
        ]);
      }
    }
  }, []);

  const bioUrl = `https://ohc.bio/${tenant}`;

  return (
    <div className="min-h-screen bg-gray-50 font-inter">
      <header className="bg-white border-b border-gray-200 px-6 py-4 flex items-center justify-between">
        <div className="flex items-center gap-4">
          <Link href="/dashboard" className="text-gray-500 hover:text-gray-900 transition-colors">
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 19l-7-7m0 0l7-7m-7 7h18" /></svg>
          </Link>
          <h1 className="text-xl font-bold font-outfit text-gray-900">Viral Link-in-Bio</h1>
        </div>
      </header>

      <main className="max-w-6xl mx-auto px-6 py-8 flex flex-col md:flex-row gap-8">
        <div className="flex-1">
          <div className="bg-white rounded-[16px] shadow-sm border border-gray-200 p-6 mb-6">
            <h2 className="text-lg font-semibold mb-2">Share Your Business Everywhere</h2>
            <p className="text-sm text-gray-600 mb-6">One link to route your Instagram and TikTok followers to your storefront, booking page, and social channels. The more you share, the more customers you get!</p>

            <div className="bg-blue-50 border border-blue-100 rounded-lg p-4 flex items-center justify-between mb-8">
              <div>
                <p className="text-xs font-semibold text-blue-800 uppercase mb-1">Your Bio Link</p>
                <p className="text-sm text-blue-900 font-mono">{bioUrl}</p>
              </div>
              <button
                onClick={() => {
                  navigator.clipboard.writeText(bioUrl);
                  setCopied(true);
                  setTimeout(() => setCopied(false), 2000);
                }}
                className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-md text-sm font-semibold transition-colors flex items-center gap-2"
              >
                {copied ? 'Copied!' : 'Copy Link'}
              </button>
            </div>

            <h3 className="font-semibold mb-4 text-gray-900">Manage Your Links</h3>
            <div className="space-y-4">
              {links.map((link, idx) => (
                <div key={idx} className="border border-gray-200 rounded-lg p-4 bg-gray-50 flex items-center justify-between">
                  <div>
                    <p className="font-medium text-sm text-gray-900">{link.title}</p>
                    <p className="text-xs text-gray-500 mt-1">{link.url}</p>
                  </div>
                  <button className="text-gray-400 hover:text-gray-600">
                    <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" /></svg>
                  </button>
                </div>
              ))}
              <button className="w-full py-3 border-2 border-dashed border-gray-300 rounded-lg text-gray-600 font-medium text-sm hover:border-gray-400 hover:text-gray-800 transition-colors">
                + Add New Link
              </button>
            </div>
          </div>
        </div>

        <div className="w-full md:w-[375px] shrink-0">
          <div className="sticky top-8">
            <h3 className="text-sm font-semibold text-gray-500 mb-4 text-center uppercase tracking-wider">Live Preview</h3>

            {/* Phone Frame */}
            <div className="border-[14px] border-gray-900 rounded-[2.5rem] bg-gray-900 relative shadow-2xl mx-auto w-[320px] h-[650px] overflow-hidden">
              {/* Notch */}
              <div className="absolute top-0 inset-x-0 h-6 bg-gray-900 rounded-b-3xl w-40 mx-auto z-20"></div>

              {/* Screen */}
              <div className="bg-gradient-to-br from-indigo-50 to-pink-50 w-full h-full relative overflow-y-auto">
                <div className="p-6 flex flex-col min-h-full items-center text-center pt-16">

                  {/* Profile */}
                  <div className="w-20 h-20 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full mb-4 shadow-sm flex items-center justify-center text-white text-2xl font-bold font-outfit">
                    {tenant.charAt(0).toUpperCase()}
                  </div>
                  <h2 className="text-lg font-bold text-gray-900 font-outfit mb-1">@{tenant}</h2>
                  <p className="text-sm text-gray-600 mb-8 max-w-[200px]">The best products, services, and content curated just for you.</p>

                  {/* Links */}
                  <div className="w-full space-y-3 mb-8">
                    {links.map((link, idx) => (
                      <a key={idx} href="#" className="block w-full bg-white/60 backdrop-blur-md border border-white/40 shadow-sm rounded-xl py-3 px-4 text-sm font-semibold text-gray-800 hover:bg-white transition-all hover:scale-[1.02]">
                        {link.title}
                      </a>
                    ))}
                  </div>

                  {/* Powered By OHC Loop */}
                  <div className="mt-auto pb-4">
                    <a href={`https://ohc.store/join?ref=${tenant}`} target="_blank" rel="noopener noreferrer" className="inline-flex items-center gap-1.5 text-xs text-gray-500 hover:text-gray-800 transition-colors group">
                      <span>⚡ Powered by</span>
                      <span className="font-bold font-outfit">OHC</span>
                      <svg className="w-3.5 h-3.5 opacity-0 -ml-2 group-hover:opacity-100 group-hover:ml-0 transition-all text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14 5l7 7m0 0l-7 7m7-7H3" /></svg>
                    </a>
                  </div>

                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
