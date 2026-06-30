"use client";

import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function QRCodeGeneratorPage() {
  const [tenantId, setTenantId] = useState('my-store');
  const [url, setUrl] = useState('');
  const [qrColor, setQrColor] = useState('#111827');
  const [qrSize, setQrSize] = useState(256);

  useEffect(() => {
    const tenant = localStorage.getItem('tenant') || 'my-store';
    setTenantId(tenant);
    setUrl(`https://ohc.app/${tenant}`);
  }, []);

  const qrImageUrl = `https://api.qrserver.com/v1/create-qr-code/?size=${qrSize}x${qrSize}&data=${encodeURIComponent(url)}&color=${qrColor.replace('#', '')}&bgcolor=ffffff`;

  const handleDownload = async () => {
    try {
      const response = await fetch(qrImageUrl);
      const blob = await response.blob();
      const blobUrl = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = blobUrl;
      a.download = `ohc-qr-code-${tenantId}.png`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      window.URL.revokeObjectURL(blobUrl);
    } catch (e) {
      console.error('Failed to download QR code', e);
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      <header className="px-4 md:px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-[30px] saturate-[210%] border-white/40">
        <h1 className="text-xl md:text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">QR Code Generator</h1>
        <Link href="/dashboard" className="px-3 py-1.5 md:px-4 md:py-2 bg-gray-200 rounded-md text-xs md:text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </Link>
      </header>

      <main className="p-4 md:p-8 flex-1 w-full max-w-6xl mx-auto">
        <div className="text-center mb-10 max-w-2xl mx-auto">
          <div className="w-16 h-16 mx-auto bg-gradient-to-br from-indigo-500 to-purple-600 rounded-2xl flex items-center justify-center text-3xl shadow-lg mb-6 text-white">
            📱
          </div>
          <h2 className="text-3xl md:text-4xl font-bold font-outfit text-gray-900 mb-4 tracking-tight">Connect Offline to Online</h2>
          <p className="text-gray-600 text-lg leading-relaxed">
            Generate a custom QR code for your physical store, flyers, or business cards. Scan it to instantly open your OHC storefront.
          </p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 items-start">
          {/* Controls */}
          <div className="app-card bg-white p-6 rounded-2xl shadow-xl w-full border border-gray-100">
            <h3 className="text-xl font-bold font-outfit text-gray-900 mb-6">Customize Your QR Code</h3>

            <div className="space-y-6">
              <div>
                <label htmlFor="qr-url" className="block text-sm font-semibold text-gray-700 mb-2">Target URL</label>
                <input
                  id="qr-url"
                  type="text"
                  value={url}
                  onChange={(e) => setUrl(e.target.value)}
                  className="w-full px-4 py-3 bg-white border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-indigo-500 text-gray-900 font-medium"
                />
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-3">Color</label>
                <div className="flex gap-3">
                  {['#111827', '#4f46e5', '#16a34a', '#dc2626', '#d97706'].map(c => (
                    <button
                      key={c}
                      onClick={() => setQrColor(c)}
                      className={`w-10 h-10 rounded-full border-2 transition-transform ${qrColor === c ? 'scale-110 border-gray-400' : 'border-transparent'}`}
                      style={{ backgroundColor: c }}
                    />
                  ))}
                  <input
                    type="color"
                    value={qrColor}
                    onChange={(e) => setQrColor(e.target.value)}
                    className="w-10 h-10 p-0 border-0 rounded-full overflow-hidden cursor-pointer"
                  />
                </div>
              </div>

              <div>
                <label className="block text-sm font-semibold text-gray-700 mb-2">Size: {qrSize}px</label>
                <input
                  type="range"
                  min="128"
                  max="1024"
                  step="64"
                  value={qrSize}
                  onChange={(e) => setQrSize(parseInt(e.target.value))}
                  className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
                />
              </div>

              <div className="pt-6 border-t border-gray-100">
                <button
                  onClick={handleDownload}
                  className="w-full py-4 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl shadow-md hover:shadow-lg transition-all text-sm flex items-center justify-center gap-2"
                >
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
                  Download PNG
                </button>
              </div>
            </div>
          </div>

          {/* Preview */}
          <div className="app-card bg-white p-6 rounded-2xl shadow-xl w-full flex flex-col justify-center min-h-[400px] relative overflow-hidden group border border-gray-100">
             <div className="absolute top-4 right-4 px-3 py-1 bg-indigo-100 text-indigo-700 text-xs font-bold rounded-full tracking-wide">LIVE PREVIEW</div>

             <div className="text-center flex flex-col items-center">
                 <h3 className="text-sm font-semibold text-gray-500 uppercase tracking-widest mb-6">Scan to test</h3>
                 <div className="p-4 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] shadow-sm border border-white/40-sm border border-gray-100 inline-block mb-6">
                    {/* Fallback svg while loading image from api */}
                    <img src={qrImageUrl} alt="QR Code" width={256} height={256} className="mx-auto" style={{ width: '256px', height: '256px' }} />
                 </div>

                 <div className="flex items-center justify-center gap-2 text-sm font-medium text-gray-500">
                    <span>⚡ Powered by OHC</span>
                 </div>
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
