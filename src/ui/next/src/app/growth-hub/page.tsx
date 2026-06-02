"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function GrowthHubPage() {
  const router = useRouter();
  const [storeName, setStoreName] = useState('My Business');
  const [tenant, setTenant] = useState('my-store');
  const [copiedLink, setCopiedLink] = useState(false);
  const [qrGenerated, setQrGenerated] = useState(false);

  useEffect(() => {
    if (typeof window !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-store';
      setTenant(storedTenant);
      setStoreName(storedTenant.replace(/-/g, ' ').replace(/\b\w/g, l => l.toUpperCase()));
    }
  }, []);

  const storeUrl = `https://${tenant}.ohc.store`;

  const handleCopyLink = () => {
    navigator.clipboard.writeText(storeUrl);
    setCopiedLink(true);
    setTimeout(() => setCopiedLink(false), 2000);
  };

  const handleGenerateQR = () => {
    setQrGenerated(true);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7] text-gray-900">
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40 shadow-sm">
        <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Growth Hub 🚀</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-4 py-2 bg-gray-900 text-white rounded-xl text-sm font-semibold hover:bg-black transition-colors shadow-sm"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-6 md:p-10 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-10">
        <div className="text-center md:text-left mb-4">
            <h2 className="text-4xl font-extrabold font-outfit tracking-tight mb-3">Customer Acquisition</h2>
            <p className="text-lg text-gray-600 max-w-2xl">Use these tools to drive traffic to your storefront, convert visitors into customers, and grow your business.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
            {/* Share Card Generator */}
            <section className="flex flex-col gap-6 p-8 rounded-3xl border border-white/40 shadow-xl relative overflow-hidden"
                style={{ background: 'rgba(255, 255, 255, 0.7)', backdropFilter: 'blur(20px) saturate(200%)' }}>
                <div className="absolute top-0 right-0 w-64 h-64 bg-indigo-50 rounded-bl-full -z-10 blur-xl"></div>
                <h3 className="text-2xl font-bold font-outfit">Social Share Card</h3>
                <p className="text-sm text-gray-600">Download a beautiful, branded image to post on Instagram, Facebook, or TikTok.</p>

                <div className="w-full aspect-[1.91/1] bg-gradient-to-br from-indigo-900 via-purple-900 to-black rounded-2xl flex flex-col items-center justify-center p-6 text-white shadow-inner relative overflow-hidden">
                    <div className="z-10 flex flex-col items-center text-center">
                        <h4 className="text-3xl md:text-4xl font-bold font-outfit mb-3">{storeName}</h4>
                        <p className="text-sm md:text-base opacity-80 mb-6">Discover our exclusive products today.</p>
                        <div className="bg-white/10 backdrop-blur-md px-4 py-2 rounded-full border border-white/20">
                             <span className="text-xs font-bold tracking-widest uppercase opacity-90">⚡ Powered by OHC</span>
                        </div>
                    </div>
                </div>

                <div className="flex gap-4 mt-auto">
                    <button className="flex-1 bg-gray-900 text-white font-bold py-3 rounded-xl hover:bg-black transition-all text-sm shadow-md">
                        Download Image
                    </button>
                    <button onClick={handleCopyLink} className="flex-1 bg-white border border-gray-200 text-gray-800 font-bold py-3 rounded-xl hover:bg-gray-50 transition-all text-sm shadow-sm">
                        {copiedLink ? 'Link Copied!' : 'Copy Store Link'}
                    </button>
                </div>
            </section>

            {/* QR Code & Physical Marketing */}
            <section className="flex flex-col gap-6 p-8 rounded-3xl border border-white/40 shadow-xl relative overflow-hidden"
                style={{ background: 'rgba(255, 255, 255, 0.7)', backdropFilter: 'blur(20px) saturate(200%)' }}>
                 <div className="absolute top-0 left-0 w-64 h-64 bg-pink-50 rounded-br-full -z-10 blur-xl"></div>
                 <h3 className="text-2xl font-bold font-outfit">Store QR Code</h3>
                 <p className="text-sm text-gray-600">Print this QR code for your physical location, business cards, or packaging.</p>

                 <div className="flex flex-col items-center justify-center py-6">
                    {qrGenerated ? (
                        <div className="w-48 h-48 bg-white p-4 rounded-2xl shadow-sm border border-gray-100 flex items-center justify-center relative">
                            {/* Dummy QR Code UI */}
                            <div className="w-full h-full border-[8px] border-black p-2 flex flex-wrap gap-1">
                                {Array.from({ length: 16 }).map((_, i) => (
                                    <div key={i} className="w-[20%] h-[20%] bg-black"></div>
                                ))}
                            </div>
                            <div className="absolute bg-white p-1 rounded-md">
                                <span className="text-[10px] font-bold">OHC</span>
                            </div>
                        </div>
                    ) : (
                        <div className="w-48 h-48 bg-gray-100 border-2 border-dashed border-gray-300 rounded-2xl flex items-center justify-center">
                            <span className="text-4xl text-gray-400">📱</span>
                        </div>
                    )}
                 </div>

                 <div className="mt-auto">
                     {!qrGenerated ? (
                         <button id="generate-qr-btn" onClick={handleGenerateQR} className="w-full bg-indigo-600 text-white font-bold py-3 rounded-xl hover:bg-indigo-700 transition-all text-sm shadow-md">
                             Generate QR Code
                         </button>
                     ) : (
                         <button className="w-full bg-gray-900 text-white font-bold py-3 rounded-xl hover:bg-black transition-all text-sm shadow-md">
                             Download High-Res PDF
                         </button>
                     )}
                 </div>
            </section>

            {/* Refer a Business Widget */}
            <section className="col-span-1 md:col-span-2 flex flex-col md:flex-row gap-8 p-8 rounded-3xl border border-white/40 shadow-xl items-center relative overflow-hidden"
                style={{ background: 'rgba(255, 255, 255, 0.8)', backdropFilter: 'blur(20px) saturate(200%)' }}>
                 <div className="flex-1 flex flex-col gap-4">
                     <h3 className="text-2xl font-bold font-outfit text-indigo-900">Invite a Fellow Business Owner</h3>
                     <p className="text-sm text-gray-700 leading-relaxed">
                         Know someone who needs an instant online storefront? Send them your invite link.
                         When they launch their store, you both get $50 in OHC premium credits.
                     </p>
                 </div>
                 <div className="w-full md:w-auto flex flex-col gap-3 min-w-[300px]">
                     <div className="bg-gray-100 p-3 rounded-xl border border-gray-200 font-mono text-xs text-gray-600 text-center break-all">
                         ohc://join?ref={tenant}
                     </div>
                     <button className="w-full bg-indigo-100 text-indigo-700 font-bold py-3 rounded-xl hover:bg-indigo-200 transition-all text-sm border border-indigo-200">
                         Copy Invite Link
                     </button>
                 </div>
            </section>
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
