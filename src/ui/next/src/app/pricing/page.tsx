"use client";

import { useState } from "react";

export default function Pricing() {
  const [showCheckout, setShowCheckout] = useState(false);

  return (
    <div className="flex flex-col min-h-screen font-inter dark:bg-[#16161A] bg-[#F5F5F7] transition-colors duration-300">
      <header className="px-6 py-4 flex items-center justify-between border-b dark:border-white/10 border-black/10 dark:bg-[#16161A]/70 bg-white/65 backdrop-blur-[30px] saturate-[210%] sticky top-0 z-50">
         <h1 className="text-2xl font-bold font-outfit dark:text-[#F5F5F7] text-[#1D1D1F] tracking-tight">Pricing Plans</h1>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8">
        <p className="text-center dark:text-gray-400 text-gray-500 mb-4">Secure encrypted payments.</p>
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
            <div className="p-6 rounded-[16px] shadow-sm border dark:border-white/5 border-gray-100 dark:bg-[#16161A]/70 bg-white">
                <h2 className="text-xl font-semibold mb-2 font-outfit dark:text-[#F5F5F7] text-[#1D1D1F]">Free</h2>
                <p className="text-sm dark:text-gray-400 text-gray-500 mb-4">100 Smart actions / month</p>
                <button className="w-full py-2 dark:bg-gray-700 bg-gray-100 dark:text-gray-300 text-gray-800 rounded-[8px] font-medium">Current Plan</button>
            </div>

            <div className="p-6 rounded-[16px] shadow-md border dark:border-[#0066FF]/30 border-blue-200 dark:bg-[#16161A]/70 bg-white relative">
                <div className="absolute top-0 right-0 bg-[#0066FF] text-white text-xs font-bold px-2 py-1 rounded-bl-[8px] rounded-tr-[16px]">POPULAR</div>
                <h2 className="text-xl font-semibold mb-2 font-outfit dark:text-[#F5F5F7] text-[#1D1D1F]">Starter</h2>
                <p className="text-sm dark:text-gray-400 text-gray-500 mb-4">500 Smart actions / month</p>
                <button className="w-full py-2 dark:bg-[#0066FF]/20 bg-blue-50 dark:text-blue-300 text-[#0066FF] rounded-[8px] font-medium hover:bg-blue-100 dark:hover:bg-[#0066FF]/30 transition-colors">Upgrade to Starter</button>
            </div>

            <div className="p-6 rounded-[16px] shadow-sm border dark:border-white/5 border-gray-100 dark:bg-[#16161A]/70 bg-white">
                <h2 className="text-xl font-semibold mb-2 font-outfit dark:text-[#F5F5F7] text-[#1D1D1F]">Pro</h2>
                <p className="text-sm dark:text-gray-400 text-gray-500 mb-4">Unlimited Smart actions</p>
                <button
                    onClick={() => setShowCheckout(true)}
                    className="w-full py-2 bg-[#0066FF] text-white rounded-[8px] font-medium transition-colors"
                >
                    Upgrade to Pro
                </button>
            </div>

            <div className="p-6 rounded-[16px] shadow-sm border dark:border-white/5 border-gray-100 dark:bg-[#16161A]/70 bg-white">
                <h2 className="text-xl font-semibold mb-2 font-outfit dark:text-[#F5F5F7] text-[#1D1D1F]">Business</h2>
                <p className="text-sm dark:text-gray-400 text-gray-500 mb-4">Custom assistance & SLA</p>
                <button className="w-full py-2 dark:bg-[#F5F5F7] bg-[#1D1D1F] dark:text-[#1D1D1F] text-white rounded-[8px] font-medium transition-colors">Contact Sales</button>
            </div>
        </div>

        {showCheckout && (
            <section id="checkout-screen" className="mt-8 p-6 rounded-[16px] shadow-sm border dark:border-[#0066FF]/30 border-blue-100 dark:bg-[#0066FF]/10 bg-blue-50">
                <h2 className="text-xl font-semibold mb-4 font-outfit dark:text-[#F5F5F7] text-[#1D1D1F]">Checkout</h2>
                <p className="text-sm dark:text-gray-400 text-gray-600 mb-4">Secure encrypted payments.</p>
                <div className="h-32 dark:bg-[#16161A]/70 bg-white rounded-[8px] flex items-center justify-center border dark:border-gray-700 border-gray-200 dark:text-gray-500 text-gray-400">
                    Payment processing...
                </div>
            </section>
        )}
      </main>
    </div>
  );
}
