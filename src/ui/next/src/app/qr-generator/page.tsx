"use client";

import React, { useState, useEffect } from 'react';
import { QRCodeSVG } from 'qrcode.react';

export default function QrGeneratorPage() {
  const [tenant, setTenant] = useState('my-business');
  const [targetUrl, setTargetUrl] = useState('');
  const [ctaText, setCtaText] = useState('Scan to get started');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const storedTenant = localStorage.getItem('tenant') || 'my-business';
      setTenant(storedTenant);
      setTargetUrl(`https://mybusiness.ohc.store/bio/${storedTenant}`);
    }
    document.title = "QR Code Generator | OHC";
  }, []);

  const trackingUrl = `https://ohc.app/api/v1/growth/qr/scan?tenant=${tenant}&target=${encodeURIComponent(targetUrl)}`;

  const handlePrint = () => {
    window.print();
  };

  const handleCopyLink = () => {
    navigator.clipboard.writeText(trackingUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-6 py-4 flex items-center justify-between border-b bg-white/65 backdrop-blur-md sticky top-0 z-50 shadow-sm print:hidden">
         <div className="flex items-center gap-3">
             <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">QR Code Generator 📱</h1>
             <span className="bg-indigo-100 text-indigo-700 text-xs font-bold px-2.5 py-0.5 rounded-full uppercase tracking-wider">Growth Loop</span>
         </div>
      </header>

      <main className="flex-1 flex flex-col md:flex-row p-6 gap-8 max-w-7xl mx-auto w-full print:p-0 print:m-0 print:w-full">
        <div className="w-full md:w-1/3 flex flex-col gap-6 print:hidden">
            <div className="bg-white rounded-[20px] p-6 shadow-sm border border-gray-200">
                <h2 className="text-xl font-bold font-outfit text-gray-900 mb-4">Design Your Display</h2>
                <p className="text-sm text-gray-600 mb-6">Create a printable QR code poster to convert in-store foot traffic into online engagement.</p>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Destination URL</label>
                    <input
                        type="url"
                        value={targetUrl}
                        onChange={(e) => setTargetUrl(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="https://..."
                    />
                    <p className="text-xs text-gray-500 mt-1">Where should customers go when they scan?</p>
                </div>

                <div className="mb-6">
                    <label className="block text-sm font-medium text-gray-700 mb-2">Call to Action (CTA)</label>
                    <input
                        type="text"
                        value={ctaText}
                        onChange={(e) => setCtaText(e.target.value)}
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        placeholder="e.g. Scan to order"
                    />
                </div>

                <div className="mb-6">
                    <label className="flex items-center gap-2 cursor-pointer">
                        <input
                            type="checkbox"
                            checked={removeBranding}
                            onChange={(e) => setRemoveBranding(e.target.checked)}
                            className="w-4 h-4 text-indigo-600 rounded focus:ring-indigo-500"
                        />
                        <span className="text-sm text-gray-700">Remove "Powered by OHC" branding</span>
                    </label>
                    <p className="text-xs text-gray-500 mt-1 ml-6">Requires Pro plan or higher.</p>
                </div>
            </div>

            <div className="bg-white rounded-[20px] p-6 shadow-sm border border-gray-200 flex flex-col gap-4">
               <h3 className="font-semibold text-gray-900">Publish</h3>
               <button
                  onClick={handlePrint}
                  className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-medium rounded-xl transition-colors shadow-sm"
               >
                  Print Display Card
               </button>
               <button
                  onClick={handleCopyLink}
                  className="w-full py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium rounded-xl transition-colors shadow-sm"
               >
                  {copied ? 'Link Copied!' : 'Copy Tracking Link'}
               </button>
            </div>
        </div>

        <section className="w-full md:w-2/3 flex flex-col items-center justify-center print:w-full print:h-screen">
             <div className="w-full max-w-md bg-white rounded-2xl overflow-hidden shadow-2xl border border-gray-200 relative print:shadow-none print:border-none print:max-w-full print:rounded-none">
                 <div className="p-12 flex flex-col items-center justify-center text-center">
                    <h2 className="text-3xl font-extrabold text-gray-900 mb-8 font-outfit" style={{ letterSpacing: '-0.02em' }}>
                      {ctaText || 'Scan Here'}
                    </h2>

                    <div className="bg-white p-4 rounded-2xl shadow-sm border border-gray-100 mb-8 inline-block">
                        <QRCodeSVG
                            value={trackingUrl}
                            size={200}
                            level="H"
                            includeMargin={true}
                        />
                    </div>

                    <p className="text-gray-500 text-sm font-medium">Point your camera at the QR code to open the link.</p>
                 </div>

                 {!removeBranding && (
                    <div className="bg-gray-50 py-4 border-t border-gray-100 text-center">
                        <a href={`/onboarding?ref=${tenant}`} target="_blank" rel="noreferrer" className="text-gray-400 hover:text-gray-600 font-semibold text-sm transition-colors inline-flex items-center gap-1">
                            ⚡ Powered by OHC
                        </a>
                    </div>
                 )}
             </div>
        </section>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        @media print {
            body { background: white; }
            .print\\:hidden { display: none !important; }
            .print\\:p-0 { padding: 0 !important; }
            .print\\:m-0 { margin: 0 !important; }
            .print\\:w-full { width: 100% !important; max-width: 100% !important; }
            .print\\:h-screen { height: 100vh !important; }
            .print\\:shadow-none { box-shadow: none !important; }
            .print\\:border-none { border: none !important; }
            .print\\:rounded-none { border-radius: 0 !important; }
            @page { margin: 0; size: auto; }
        }
      `}} />
    </div>
  );
}