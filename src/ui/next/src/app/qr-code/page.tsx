"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function QRCodePage() {
  const router = useRouter();
  const [storeName, setStoreName] = useState('My Store');
  const [storeUrl, setStoreUrl] = useState('https://ohc.store/');
  const [qrColor, setQrColor] = useState('#000000');
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [hasPro, setHasPro] = useState(false);

  useEffect(() => {
    if (typeof localStorage !== 'undefined') {
      const tenant = localStorage.getItem('tenant_id') || 'DEFAULT';
      setStoreUrl(`https://ohc.store/${tenant}`);
      setStoreName(localStorage.getItem('business_name') || 'My Store');
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleColorChange = (color: string) => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }
    setQrColor(color);
  };

  const handleAddLogo = () => {
      if (!hasPro) {
          setShowSoftPaywall(true);
          return;
      }
      alert("Logo upload logic would go here!");
  };

  const qrImageUrl = `https://api.qrserver.com/v1/create-qr-code/?size=300x300&data=${encodeURIComponent(storeUrl)}&color=${qrColor.replace('#', '')}&bgcolor=ffffff`;

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Offline Marketing 📱</h1>
         <div className="flex items-center gap-3">
             <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
               Back to Dashboard
             </button>
         </div>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col md:flex-row gap-8 items-start">
        {/* Settings Section */}
        <section className="w-full md:w-1/3 flex flex-col gap-6">
            <div className="p-6 shadow-md rounded-2xl border" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)' }}>
                <h2 className="text-xl font-semibold font-outfit mb-4" style={{ color: '#1D1D1F' }}>Customize QR Flyer</h2>
                <div className="flex flex-col gap-5">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Store Name Display</label>
                        <input
                            type="text"
                            value={storeName}
                            onChange={(e) => setStoreName(e.target.value)}
                            className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500"
                        />
                    </div>
                    <div>
                        <label className="block text-sm font-medium text-gray-700 mb-1">Store URL</label>
                        <input
                            type="text"
                            value={storeUrl}
                            readOnly
                            className="w-full px-4 py-2 bg-gray-50 border border-gray-200 text-gray-500 rounded-lg cursor-not-allowed"
                        />
                    </div>

                    <div className="border-t border-gray-100 pt-4">
                        <label className="flex items-center justify-between text-sm font-medium text-gray-700 mb-2">
                            QR Code Color
                            {!hasPro && <span className="text-xs bg-yellow-100 text-yellow-800 px-2 py-0.5 rounded font-bold">PRO</span>}
                        </label>
                        <div className="flex gap-3">
                            <button onClick={() => setQrColor('#000000')} className={`w-8 h-8 rounded-full border-2 ${qrColor === '#000000' ? 'border-indigo-600' : 'border-gray-200'}`} style={{ backgroundColor: '#000000' }}></button>
                            <button onClick={() => handleColorChange('#4F46E5')} className={`w-8 h-8 rounded-full border-2 ${qrColor === '#4F46E5' ? 'border-indigo-600' : 'border-gray-200'}`} style={{ backgroundColor: '#4F46E5' }}></button>
                            <button onClick={() => handleColorChange('#E11D48')} className={`w-8 h-8 rounded-full border-2 ${qrColor === '#E11D48' ? 'border-indigo-600' : 'border-gray-200'}`} style={{ backgroundColor: '#E11D48' }}></button>
                            <button onClick={() => handleColorChange('#16A34A')} className={`w-8 h-8 rounded-full border-2 ${qrColor === '#16A34A' ? 'border-indigo-600' : 'border-gray-200'}`} style={{ backgroundColor: '#16A34A' }}></button>
                        </div>
                    </div>

                    <div>
                         <label className="flex items-center justify-between text-sm font-medium text-gray-700 mb-2">
                            Add Logo to Center
                            {!hasPro && <span className="text-xs bg-yellow-100 text-yellow-800 px-2 py-0.5 rounded font-bold">PRO</span>}
                        </label>
                        <button
                            onClick={handleAddLogo}
                            className="w-full py-2 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium rounded-lg transition-colors flex items-center justify-center gap-2 border border-gray-200"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" /></svg>
                            Upload Brand Logo
                        </button>
                    </div>
                </div>
            </div>

            <div className="flex flex-col gap-3">
                 <button className="w-full py-3 bg-gray-900 hover:bg-black text-white font-bold rounded-xl shadow-md transition-all flex items-center justify-center gap-2">
                     <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" /></svg>
                     Download Flyer (PDF)
                 </button>
                 <button className="w-full py-3 bg-white border border-gray-200 hover:bg-gray-50 text-gray-800 font-bold rounded-xl shadow-sm transition-all flex items-center justify-center gap-2">
                     Download QR Image (PNG)
                 </button>
            </div>
        </section>

        {/* Live Preview Section */}
        <section className="w-full md:w-2/3 flex flex-col items-center">
             <h2 className="text-xl font-semibold font-outfit mb-4 self-start" style={{ color: '#1D1D1F' }}>Flyer Preview</h2>

             {/* The Printable Flyer Layout */}
             <div className="w-full max-w-[500px] aspect-[1/1.414] bg-white rounded-lg shadow-2xl flex flex-col items-center p-12 text-center relative border border-gray-100">
                 <div className="flex-1 flex flex-col items-center justify-center w-full mt-4">
                     <h1 className="text-4xl sm:text-5xl font-black font-outfit text-gray-900 mb-2 leading-tight tracking-tight uppercase">
                         {storeName}
                     </h1>
                     <p className="text-xl text-gray-600 font-medium mb-10 tracking-widest uppercase">
                         Scan to Order
                     </p>

                     <div className="p-4 bg-white rounded-3xl shadow-lg border-4 border-gray-100 mb-8" style={{ borderColor: qrColor === '#000000' ? '#f3f4f6' : qrColor }}>
                         {/* eslint-disable-next-line @next/next/no-img-element */}
                         <img src={qrImageUrl} alt="QR Code" width={240} height={240} className="rounded-xl object-contain" />
                     </div>

                     <p className="text-lg font-bold text-gray-800 tracking-wide font-mono bg-gray-100 px-4 py-2 rounded-lg">
                         {storeUrl.replace('https://', '')}
                     </p>
                 </div>

                 {/* Viral Growth Loop Footer */}
                 <div className="absolute bottom-6 w-full flex flex-col items-center justify-center text-gray-400 gap-1 opacity-80">
                     <span className="text-xs font-bold uppercase tracking-widest flex items-center gap-1">
                         <span className="w-3 h-3 bg-indigo-600 rounded-sm inline-block"></span>
                         Powered by OHC
                     </span>
                     <span className="text-[10px] font-medium">Create your own free store at ohc.store</span>
                 </div>
             </div>
             <p className="text-sm text-gray-500 mt-6 text-center max-w-md">
                 Print this flyer and place it on your counter, market stall, or include it in your packaging to turn offline interactions into online sales.
             </p>
        </section>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="bg-white w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="text-5xl mb-4">🎨</div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Custom Branding</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Customizing QR code colors and adding your logo is a Pro feature. Upgrade to our Pro plan to make your offline marketing stand out.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); window.location.href = '/pricing'; }}
              className="w-full py-4 rounded-xl font-bold text-white mb-3 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
            >
              Upgrade to Pro
            </button>
            <button
              onClick={() => setShowSoftPaywall(false)}
              className="w-full py-3 rounded-xl font-bold transition-all text-gray-600 hover:bg-gray-100"
            >
              Keep Free Version
            </button>
          </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}