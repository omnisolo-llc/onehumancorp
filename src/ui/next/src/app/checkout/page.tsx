"use client";

import { useSearchParams } from 'next/navigation';
import { Suspense } from 'react';

function CheckoutContent() {
  const searchParams = useSearchParams();
  const plan = searchParams?.get('plan') || 'pro';

  return (
      <div className="w-full max-w-md bg-white rounded-xl shadow-lg border border-gray-100 overflow-hidden">
        <div className="bg-blue-600 p-6 text-center text-white">
            <h1 className="text-2xl font-bold mb-1">Checkout</h1>
            <p className="text-blue-100 text-sm opacity-90">Upgrading to {plan.charAt(0).toUpperCase() + plan.slice(1)} Plan</p>
        </div>

        <div className="p-8 text-center" id="checkout-screen">
             <div className="mb-6 flex justify-center">
                 <div className="w-16 h-16 bg-blue-50 text-blue-500 rounded-full flex items-center justify-center text-2xl">
                     🔒
                 </div>
             </div>
             <p className="text-gray-600 mb-8 font-medium">100% money back guarantee. Secure SSL payments powered by Stripe.</p>

             <div className="space-y-3">
                 <button
                    onClick={() => {
                        alert('Payment successful!');
                        window.location.href = '/my-plan';
                    }}
                    className="w-full py-3 bg-blue-600 text-white rounded-lg font-bold hover:bg-blue-700 shadow-sm transition-all"
                 >
                    Pay Now
                 </button>
                 <a
                    href="/pricing"
                    className="block w-full py-3 bg-white border border-gray-300 text-gray-700 rounded-lg font-medium hover:bg-gray-50 transition-all"
                 >
                    Cancel
                 </a>
             </div>
        </div>
      </div>
  );
}

export default function Checkout() {
  return (
    <div className="flex flex-col min-h-screen font-inter bg-gray-50 items-center justify-center p-6">
      <Suspense fallback={<div>Loading checkout...</div>}>
        <CheckoutContent />
      </Suspense>
    </div>
  );
}
