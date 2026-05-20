"use client";

import { useState } from "react";
import Link from "next/link";

export default function PricingPage() {
  const [checkoutPlan, setCheckoutPlan] = useState<string | null>(null);

  if (checkoutPlan) {
    return (
      <div className="flex flex-col min-h-screen bg-gray-50 font-inter p-8">
        <h1 className="text-3xl font-bold mb-4">Checkout</h1>
        <div id="checkout-screen" className="bg-white p-6 rounded-lg shadow-md max-w-md">
          <p className="mb-4">You are upgrading to the {checkoutPlan} plan.</p>
          <p className="text-sm text-gray-600">Secure SSL payments.</p>
          <div className="mt-6 flex gap-4">
            <button className="bg-blue-600 text-white px-4 py-2 rounded-lg font-medium">Confirm Payment</button>
            <button onClick={() => setCheckoutPlan(null)} className="bg-gray-200 text-gray-800 px-4 py-2 rounded-lg font-medium">Cancel</button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col min-h-screen bg-gray-50 font-inter p-8">
      <div className="max-w-6xl mx-auto w-full">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-8 text-center">Pricing Plans</h1>

        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          {/* Free Tier */}
          <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-200 flex flex-col">
            <h2 className="text-xl font-bold text-gray-800 mb-2">Free</h2>
            <p className="text-3xl font-extrabold mb-4">$0<span className="text-lg text-gray-500 font-normal">/mo</span></p>
            <ul className="mb-6 space-y-2 flex-1 text-sm text-gray-600">
              <li>100 AI actions / month</li>
              <li>500MB Storage</li>
              <li>1 Agent Limit</li>
              <li>10 Products Limit</li>
            </ul>
            <button className="w-full bg-gray-100 text-gray-800 py-2 rounded-lg font-medium" disabled>Current Plan</button>
          </div>

          {/* Starter Tier */}
          <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-200 flex flex-col relative">
            <h2 className="text-xl font-bold text-gray-800 mb-2">Starter</h2>
            <p className="text-3xl font-extrabold mb-4">$29<span className="text-lg text-gray-500 font-normal">/mo</span></p>
            <ul className="mb-6 space-y-2 flex-1 text-sm text-gray-600">
              <li>1000 AI actions / month</li>
              <li>5000MB Storage</li>
              <li>3 Agents Limit</li>
              <li>100 Products Limit</li>
            </ul>
            <button onClick={() => setCheckoutPlan('Starter')} className="w-full bg-blue-600 text-white py-2 rounded-lg font-medium hover:bg-blue-700 transition">Upgrade to Starter via Stripe</button>
          </div>

          {/* Pro Tier */}
          <div className="bg-blue-50 p-6 rounded-xl shadow-md border border-blue-200 flex flex-col relative">
            <div className="absolute top-0 right-0 bg-blue-600 text-white text-xs font-bold px-3 py-1 rounded-bl-lg rounded-tr-lg">POPULAR</div>
            <h2 className="text-xl font-bold text-blue-900 mb-2">Pro</h2>
            <p className="text-3xl font-extrabold mb-4 text-blue-900">$99<span className="text-lg text-blue-600 font-normal">/mo</span></p>
            <ul className="mb-6 space-y-2 flex-1 text-sm text-blue-800">
              <li>Unlimited AI actions</li>
              <li>50000MB Storage</li>
              <li>10 Agents Limit</li>
              <li>Unlimited Products</li>
            </ul>
            <button onClick={() => setCheckoutPlan('Pro')} className="w-full bg-blue-600 text-white py-2 rounded-lg font-medium hover:bg-blue-700 transition">Upgrade to Pro via Stripe</button>
          </div>

          {/* Business Tier */}
          <div className="bg-white p-6 rounded-xl shadow-sm border border-gray-200 flex flex-col">
            <h2 className="text-xl font-bold text-gray-800 mb-2">Business</h2>
            <p className="text-3xl font-extrabold mb-4">$299<span className="text-lg text-gray-500 font-normal">/mo</span></p>
            <ul className="mb-6 space-y-2 flex-1 text-sm text-gray-600">
              <li>Unlimited AI actions</li>
              <li>512000MB Storage</li>
              <li>Unlimited Agents</li>
              <li>Unlimited Products</li>
            </ul>
            <button onClick={() => setCheckoutPlan('Business')} className="w-full bg-gray-800 text-white py-2 rounded-lg font-medium hover:bg-gray-900 transition">Upgrade to Business via Stripe</button>
          </div>
        </div>

        <p className="text-center text-sm text-gray-500 mt-8">Secure SSL payments powered by Stripe.</p>
      </div>
    </div>
  );
}
