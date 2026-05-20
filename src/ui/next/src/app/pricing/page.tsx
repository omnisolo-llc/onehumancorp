"use client";

import { useState } from "react";

export default function Pricing() {
  const [loading, setLoading] = useState(false);

  const handleCheckout = (plan: string) => {
    setLoading(true);
    // Simulating checkout redirect
    setTimeout(() => {
        window.location.href = `/checkout?plan=${plan}`;
    }, 500);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
         <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Pricing Plans</h1>
      </header>

      <main className="p-6 md:p-8 flex-1 max-w-5xl mx-auto w-full flex flex-col gap-8 text-center">
        <p className="text-gray-600 text-lg mb-4">Plain-language pricing — no hidden fees. Choose the best plan to grow your small business.</p>
        <p className="text-sm font-semibold text-green-600 mb-8">Secure SSL payments powered by Stripe.</p>

        <div className="grid grid-cols-1 md:grid-cols-4 gap-6">
          {/* Free Tier */}
          <div className="p-6 shadow-sm flex flex-col bg-white border border-gray-200 rounded-xl">
            <h2 className="text-xl font-bold mb-2">Free</h2>
            <p className="text-3xl font-bold mb-4">$0 <span className="text-sm font-normal text-gray-500">/ mo</span></p>
            <ul className="text-left text-sm text-gray-600 mb-6 flex-1 space-y-2">
              <li>100 AI actions / month</li>
              <li>500MB Storage</li>
              <li>Basic support</li>
            </ul>
            <button className="w-full py-2 bg-gray-100 text-gray-800 rounded font-medium hover:bg-gray-200" disabled>Current Plan</button>
          </div>

          {/* Starter Tier */}
          <div className="p-6 shadow-sm flex flex-col bg-white border border-gray-200 rounded-xl relative">
            <h2 className="text-xl font-bold mb-2">Starter</h2>
            <p className="text-3xl font-bold mb-4">$9 <span className="text-sm font-normal text-gray-500">/ mo</span></p>
            <ul className="text-left text-sm text-gray-600 mb-6 flex-1 space-y-2">
              <li>1000 AI actions / month</li>
              <li>5GB Storage</li>
              <li>Email support</li>
            </ul>
            <button onClick={() => handleCheckout('starter')} className="w-full py-2 bg-blue-600 text-white rounded font-medium hover:bg-blue-700">Upgrade to Starter via Stripe</button>
          </div>

          {/* Pro Tier */}
          <div className="p-6 shadow-sm flex flex-col bg-blue-50 border-2 border-blue-500 rounded-xl relative">
            <div className="absolute top-0 right-0 bg-blue-500 text-white text-xs font-bold px-2 py-1 rounded-bl-lg rounded-tr-lg">POPULAR</div>
            <h2 className="text-xl font-bold mb-2 text-blue-900">Pro</h2>
            <p className="text-3xl font-bold mb-4 text-blue-900">$29 <span className="text-sm font-normal text-blue-700">/ mo</span></p>
            <ul className="text-left text-sm text-blue-800 mb-6 flex-1 space-y-2">
              <li>Unlimited AI actions</li>
              <li>50GB Storage</li>
              <li>Priority support</li>
            </ul>
            <button onClick={() => handleCheckout('pro')} className="w-full py-2 bg-blue-600 text-white rounded font-medium hover:bg-blue-700">Upgrade to Pro via Stripe</button>
          </div>

          {/* Business Tier */}
          <div className="p-6 shadow-sm flex flex-col bg-white border border-gray-200 rounded-xl">
            <h2 className="text-xl font-bold mb-2">Business</h2>
            <p className="text-3xl font-bold mb-4">$79 <span className="text-sm font-normal text-gray-500">/ mo</span></p>
            <ul className="text-left text-sm text-gray-600 mb-6 flex-1 space-y-2">
              <li>Unlimited AI actions</li>
              <li>500GB Storage</li>
              <li>24/7 Phone support</li>
              <li>Custom SLAs</li>
            </ul>
            <button onClick={() => handleCheckout('business')} className="w-full py-2 bg-blue-600 text-white rounded font-medium hover:bg-blue-700">Upgrade to Business via Stripe</button>
          </div>
        </div>
      </main>
    </div>
  );
}
