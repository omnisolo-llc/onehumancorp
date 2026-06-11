'use client';

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function SpinToWinGeneratorPage() {
  const router = useRouter();
  const [tenant, setTenant] = useState('my-store');
  const [campaignTitle, setCampaignTitle] = useState('Spin to Win!');
  const [offerText, setOfferText] = useState('Spin the wheel for a chance to win a discount on your next order.');

  const [slices, setSlices] = useState([
    { id: 1, label: '10% Off', value: '10OFF', color: '#F87171' },
    { id: 2, label: 'No Luck', value: 'NONE', color: '#9CA3AF' },
    { id: 3, label: 'Free Shipping', value: 'FREESHIP', color: '#60A5FA' },
    { id: 4, label: '20% Off', value: '20OFF', color: '#34D399' },
    { id: 5, label: 'No Luck', value: 'NONE', color: '#9CA3AF' },
    { id: 6, label: '$5 Off', value: '5OFF', color: '#FBBF24' },
  ]);

  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const savedTenant = localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store';
      setTenant(savedTenant);
    }
  }, []);

  const generatedLink = `${typeof window !== 'undefined' ? window.location.origin : 'https://ohc.app'}/api/v1/growth/spin-to-win/embed?tenant=${tenant}&title=${encodeURIComponent(campaignTitle)}&offer=${encodeURIComponent(offerText)}&slices=${encodeURIComponent(JSON.stringify(slices.map(s => ({label: s.label, value: s.value, color: s.color}))))}`;

  const embedCode = `<iframe src="${generatedLink}" width="100%" height="500" frameborder="0" style="border-radius: 12px; border: 1px solid #eaeaea;"></iframe>\n<div style="text-align:center; font-size:12px; margin-top:8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color:#6b7280;text-decoration:none;">⚡ Powered by OHC</a></div>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleSliceChange = (index: number, field: string, value: string) => {
    const newSlices = [...slices];
    newSlices[index] = { ...newSlices[index], [field]: value };
    setSlices(newSlices);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gray-50">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-xl font-bold font-outfit text-gray-900 tracking-tight">OneHumanCorp</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="flex-1 max-w-4xl mx-auto w-full p-6 py-10">
        <div className="mb-8">
          <div className="inline-flex items-center gap-2 mb-2 px-3 py-1 rounded-full bg-pink-50 text-pink-700 text-sm font-semibold border border-pink-100 shadow-sm">
            <span>🎁 Virality & Acquisition</span>
          </div>
          <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">
            Spin-to-Win Generator
          </h1>
          <p className="text-gray-600">
            Gamify your email capture! Create a spin-to-win wheel that gives customers discounts in exchange for their email.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div className="space-y-6">
            <div className="glassmorphism p-6 rounded-[16px] shadow-sm border border-white/40">
              <h2 className="text-xl font-semibold mb-4 text-gray-900">Campaign Details</h2>
              <div className="space-y-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Campaign Title</label>
                  <input
                    type="text"
                    value={campaignTitle}
                    onChange={(e) => setCampaignTitle(e.target.value)}
                    className="w-full px-4 py-2 rounded-lg border border-gray-200 focus:outline-none focus:ring-2 focus:ring-pink-500"
                  />
                </div>
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">Offer Text</label>
                  <input
                    type="text"
                    value={offerText}
                    onChange={(e) => setOfferText(e.target.value)}
                    className="w-full px-4 py-2 rounded-lg border border-gray-200 focus:outline-none focus:ring-2 focus:ring-pink-500"
                  />
                </div>
              </div>
            </div>

            <div className="glassmorphism p-6 rounded-[16px] shadow-sm border border-white/40">
              <h2 className="text-xl font-semibold mb-4 text-gray-900">Wheel Slices</h2>
              <div className="space-y-4">
                {slices.map((slice, index) => (
                  <div key={slice.id} className="flex items-center gap-2">
                    <div className="w-8 h-8 rounded-full flex items-center justify-center font-bold text-xs text-gray-500 bg-gray-100 shrink-0">
                      {index + 1}
                    </div>
                    <input
                      type="text"
                      value={slice.label}
                      onChange={(e) => handleSliceChange(index, 'label', e.target.value)}
                      placeholder="Label (e.g. 10% Off)"
                      className="flex-1 px-3 py-1.5 text-sm rounded border border-gray-200"
                    />
                    <input
                      type="text"
                      value={slice.value}
                      onChange={(e) => handleSliceChange(index, 'value', e.target.value)}
                      placeholder="Code"
                      className="w-24 px-3 py-1.5 text-sm rounded border border-gray-200"
                    />
                    <input
                      type="color"
                      value={slice.color}
                      onChange={(e) => handleSliceChange(index, 'color', e.target.value)}
                      className="w-8 h-8 rounded border-none p-0 cursor-pointer"
                    />
                  </div>
                ))}
              </div>
            </div>
          </div>

          <div className="space-y-6">
            <div className="glassmorphism p-6 rounded-[16px] shadow-sm border border-white/40 sticky top-24">
              <h2 className="text-xl font-semibold mb-4 text-gray-900">Preview & Embed</h2>

              <div className="mb-6 p-4 bg-white rounded-xl border border-gray-200 text-center relative overflow-hidden">
                <h3 className="font-bold text-xl mb-1">{campaignTitle}</h3>
                <p className="text-sm text-gray-600 mb-6">{offerText}</p>

                <div className="w-48 h-48 mx-auto relative rounded-full border-4 border-gray-800 shadow-xl overflow-hidden"
                     style={{
                       background: `conic-gradient(
                         ${slices[0].color} 0deg 60deg,
                         ${slices[1].color} 60deg 120deg,
                         ${slices[2].color} 120deg 180deg,
                         ${slices[3].color} 180deg 240deg,
                         ${slices[4].color} 240deg 300deg,
                         ${slices[5].color} 300deg 360deg
                       )`
                     }}>
                  <div className="absolute inset-0 flex items-center justify-center">
                    <div className="w-12 h-12 bg-white rounded-full border-2 border-gray-800 shadow-md flex items-center justify-center font-bold text-xs">
                      SPIN
                    </div>
                  </div>
                </div>

                <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-[100px] w-4 h-4 bg-gray-800 rotate-45 transform origin-bottom border-b-0 border-r-0"></div>
              </div>

              <div className="space-y-3">
                <label className="block text-sm font-medium text-gray-700">Embed Code</label>
                <div className="relative">
                  <textarea
                    readOnly
                    value={embedCode}
                    className="w-full h-32 px-4 py-3 rounded-xl border border-gray-200 bg-gray-50 text-xs font-mono focus:outline-none focus:ring-2 focus:ring-pink-500"
                  />
                </div>

                <button
                  onClick={handleCopy}
                  className="w-full py-3 px-4 bg-gray-900 hover:bg-black text-white font-semibold rounded-xl shadow-md transition-all flex items-center justify-center gap-2"
                >
                  {copied ? (
                    <>
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" /></svg>
                      Copied!
                    </>
                  ) : (
                    <>
                      <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
                      Copy HTML
                    </>
                  )}
                </button>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
