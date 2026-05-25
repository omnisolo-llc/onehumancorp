"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function CheckoutPage() {
  const router = useRouter();
  const [isPaid, setIsPaid] = useState(false);
  const [isShared, setIsShared] = useState(false);

  const handleShare = (platform: string) => {
    // Simulate opening a share dialog
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
    const text = encodeURIComponent(`I just upgraded my business on OHC! Check them out: https://ohc.app/join?ref=${tenant}`);

    if (platform === 'twitter') {
      window.open(`https://twitter.com/intent/tweet?text=${text}`, '_blank');
    } else if (platform === 'whatsapp') {
      window.open(`https://wa.me/?text=${text}`, '_blank');
    }

    setIsShared(true);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Checkout</h1>
      </header>

      <main id="checkout-screen" className="p-6 md:p-8 flex-1 max-w-lg mx-auto w-full flex flex-col gap-6">
        {!isPaid ? (
          <>
            <p className="text-gray-700">Please enter your payment details below.</p>

            <div className="p-6 shadow-sm flex flex-col gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
              <p className="text-sm text-gray-600">100% money back guarantee. Secure SSL payments.</p>

              <button
                onClick={() => {
                  setIsPaid(true);
                }}
                className="w-full px-4 py-3 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-colors shadow-sm"
              >
                Pay Now
              </button>

              <button
                onClick={() => router.push('/pricing')}
                className="w-full px-4 py-3 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors"
              >
                Cancel
              </button>
            </div>
          </>
        ) : (
          <div className="p-8 shadow-xl flex flex-col items-center text-center gap-6" style={{ background: 'rgba(255, 255, 255, 0.85)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.8)', borderRadius: '24px' }}>
            <div className="w-16 h-16 bg-green-100 text-green-600 rounded-full flex items-center justify-center text-3xl mb-2 shadow-inner">
              🎉
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900">Payment Successful!</h2>

            {!isShared ? (
              <>
                <p className="text-gray-600">
                  Thank you for your purchase. Want <strong>15% off</strong> your next order? Share your milestone with your network!
                </p>
                <div className="flex flex-col gap-3 w-full mt-4">
                  <button
                    onClick={() => handleShare('twitter')}
                    className="w-full flex items-center justify-center gap-2 bg-black text-white py-3 rounded-xl font-semibold shadow-sm hover:bg-gray-800 transition-all"
                  >
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.008 5.94H5.078z"/></svg>
                    Share on X
                  </button>
                  <button
                    onClick={() => handleShare('whatsapp')}
                    className="w-full flex items-center justify-center gap-2 bg-[#25D366] text-white py-3 rounded-xl font-semibold shadow-sm hover:bg-[#20bd5a] transition-all"
                  >
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24"><path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51a12.8 12.8 0 0 0-.57-.01c-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 0 1-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 0 1-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 0 1 2.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0 0 12.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 0 0 5.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 0 0-3.48-8.413Z"/></svg>
                    Share on WhatsApp
                  </button>
                  <button
                    onClick={() => router.push('/dashboard')}
                    className="w-full py-2 mt-2 text-sm font-semibold text-gray-500 hover:text-gray-700 transition-colors"
                  >
                    No thanks, go to dashboard
                  </button>
                </div>
              </>
            ) : (
              <div className="w-full animate-in fade-in slide-in-from-bottom-4 duration-500">
                <p className="text-green-600 font-semibold mb-4">Thanks for sharing! Here is your discount code:</p>
                <div className="bg-gray-100 border-2 border-dashed border-gray-300 rounded-xl p-4 mb-6">
                  <span className="text-3xl font-mono font-bold tracking-widest text-gray-800">VIRAL15</span>
                </div>
                <button
                  onClick={() => router.push('/dashboard')}
                  className="w-full px-4 py-3 bg-indigo-600 text-white rounded-xl font-semibold hover:bg-indigo-700 transition-colors shadow-md"
                >
                  Continue to Dashboard
                </button>
              </div>
            )}
          </div>
        )}
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
