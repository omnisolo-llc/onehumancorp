"use client";

import React, { useState, useEffect } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';

export default function EmbedCheckoutPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const tenant = searchParams.get('tenant') || 'demo-store';
  const product = searchParams.get('product') || 'Premium Cake';
  const price = searchParams.get('price') || '45.00';
  const theme = searchParams.get('theme') || 'light';

  const [checkoutStatus, setCheckoutStatus] = useState("");
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const getThemeStyles = () => {
    if (theme === 'dark') {
      return {
        background: '#1D1D1F',
        color: '#ffffff',
        borderColor: '#333333'
      };
    }
    return {
      background: '#ffffff',
      color: '#1D1D1F',
      borderColor: '#E5E7EB'
    };
  };

  const handleBuyNow = () => {
      setCheckoutStatus('Checkout process initiated.');
      // In a real app, this might post a message to the parent window or redirect to a Stripe checkout session
      setTimeout(() => {
          setCheckoutStatus('');
          // Redirecting to the main checkout route for demo purposes
          router.push(`/checkout?tenant=${tenant}&product=${encodeURIComponent(product)}&price=${encodeURIComponent(price)}`);
      }, 1000);
  };

  if (!mounted) return null;

  return (
    <div className="w-full h-screen overflow-hidden flex flex-col font-inter" style={{ backgroundColor: 'transparent' }}>
      <div className="w-full flex-1 flex flex-col relative overflow-hidden border" style={{ ...getThemeStyles(), borderRadius: '16px' }}>
          <div className="w-full h-40 bg-gradient-to-br from-indigo-500 to-purple-500 rounded-t-[16px] relative flex items-center justify-center">
              <span className="text-5xl text-white">🛍️</span>
          </div>
          <div className="p-5 flex flex-col flex-1">
              <h4 className="font-bold text-lg font-outfit mb-1" style={{ color: theme === 'dark' ? '#fff' : '#111827' }}>{product}</h4>
              <p className="text-2xl font-bold mb-4" style={{ color: theme === 'dark' ? '#d1d5db' : '#4b5563' }}>${price}</p>

              <div className="mt-auto flex flex-col gap-2">
                <button
                    type="button"
                    onClick={handleBuyNow}
                    className="w-full py-3 bg-indigo-600 hover:bg-indigo-700 text-white font-bold rounded-xl text-sm flex items-center justify-center gap-2 transition-colors shadow-md hover:shadow-lg"
                >
                    Buy Now
                </button>
                <button
                    type="button"
                    onClick={() => setCheckoutStatus('Apple Pay initiated')}
                    className="w-full py-2 bg-black hover:bg-gray-800 text-white font-semibold rounded-xl text-sm flex items-center justify-center gap-2 transition-colors"
                >
                    Pay with Apple Pay
                </button>
              </div>
              {checkoutStatus && <p className="mt-2 text-xs font-semibold text-indigo-600 text-center" role="status">{checkoutStatus}</p>}
          </div>
      </div>
      <div className="mt-3 text-center" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
          <a href={`/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}&source=checkout_embed`} target="_blank" rel="noopener noreferrer" style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }} className="hover:text-indigo-600 transition-colors">
              ⚡ Powered by OHC
          </a>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@400;500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
