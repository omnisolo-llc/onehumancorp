"use client";

// Pricing Page Implementation
import React from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function PricingPage() {
  const router = useRouter();

  const handleUpgrade = (tier: string) => {
    router.push('/checkout?tier=' + tier);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900">
      <header className="px-4 md:px-6 py-4 flex flex-col md:flex-row items-center justify-between gap-4 sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b border-white/40 shadow-sm">
        <WithTooltip id="pricing-tier-tooltip" defaultText="Select the plan that best fits your business needs.">
          <h1 className="text-2xl font-bold font-outfit text-center md:text-left text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">Pricing Plans</h1>
        </WithTooltip>
        <button onClick={() => router.push('/dashboard')} className="min-w-[44px] min-h-[44px] px-4 py-2 bg-gray-100 rounded-md text-sm font-medium text-gray-800 hover:bg-gray-200 transition-colors flex items-center justify-center">
          Back to Dashboard
        </button>
      </header>

      <main id="pricing-screen" className="p-4 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-8">
        <div className="text-center mb-8">
          <p className="text-lg text-gray-600">Plain-language pricing — no hidden fees. Choose the best plan to grow your small business.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {/* Free Tier */}
          <div className="p-6 md:p-8 flex flex-col justify-between bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-lg rounded-[24px] hover:shadow-xl transition-shadow duration-300">
            <div>
              <h3 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Free</h3>
              <p className="text-xl font-semibold mb-4 text-gray-900">$0 <span className="text-sm font-normal text-gray-500">/ month</span></p>
              <ul className="text-sm text-gray-700 space-y-3 mb-6">
                <li className="flex items-center gap-2"><span>✓</span> 1 Agent Limit</li>
                <li className="flex items-center gap-2"><span>✓</span> 100 AI actions / month</li>
                <li className="flex items-center gap-2"><span>✓</span> 500MB Storage Quota</li>
                <li className="flex items-center gap-2"><span>✓</span> 10 Products Limit</li>
              </ul>
            </div>
            <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors flex items-center justify-center" disabled>
              Current Plan
            </button>
          </div>

          {/* Starter Tier */}
          <div className="p-6 md:p-8 flex flex-col justify-between relative bg-white/80 backdrop-blur-2xl saturate-200 border border-indigo-200 shadow-xl rounded-[24px] hover:shadow-2xl transition-shadow duration-300">
            <div className="absolute top-0 right-0 bg-indigo-600 text-white text-xs font-bold px-3 py-1 rounded-bl-xl rounded-tr-[24px]">Recommended</div>
            <div>
              <h3 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Starter</h3>
              <p className="text-xl font-semibold mb-2 text-gray-900">$29 <span className="text-sm font-normal text-gray-500">/ month</span></p>
              <p className="text-xs text-indigo-600 font-medium mb-4">Suggested for growing stores</p>
              <ul className="text-sm text-gray-700 space-y-3 mb-6">
                <li className="flex items-center gap-2"><span>✓</span> 3 Agents Limit</li>
                <li className="flex items-center gap-2"><span>✓</span> 1,000 AI actions / month</li>
                <li className="flex items-center gap-2"><span>✓</span> 5GB Storage Quota</li>
                <li className="flex items-center gap-2"><span>✓</span> 100 Products Limit</li>
              </ul>
            </div>
            <button onClick={() => handleUpgrade('Starter')} className="w-full min-h-[44px] px-4 py-2 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-colors shadow-sm flex items-center justify-center">
              Upgrade to Starter via Stripe
            </button>
          </div>

          {/* Pro Tier */}
          <div className="p-6 md:p-8 flex flex-col justify-between bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-lg rounded-[24px] hover:shadow-xl transition-shadow duration-300">
            <div>
              <h3 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Pro</h3>
              <p className="text-xl font-semibold mb-4 text-gray-900">$79 <span className="text-sm font-normal text-gray-500">/ month</span></p>
              <ul className="text-sm text-gray-700 space-y-3 mb-6">
                <li className="flex items-center gap-2"><span>✓</span> 10 Agents Limit</li>
                <li className="flex items-center gap-2"><span>✓</span> Unlimited AI actions</li>
                <li className="flex items-center gap-2"><span>✓</span> 50GB Storage Quota</li>
                <li className="flex items-center gap-2"><span>✓</span> Unlimited Products</li>
              </ul>
            </div>
            <button onClick={() => handleUpgrade('Pro')} className="w-full min-h-[44px] px-4 py-2 bg-gray-800 text-white rounded-lg font-medium hover:bg-gray-900 transition-colors shadow-sm flex items-center justify-center">
              Upgrade to Pro via Stripe
            </button>
          </div>

          {/* Business Tier */}
          <div className="p-6 md:p-8 flex flex-col justify-between bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-lg rounded-[24px] hover:shadow-xl transition-shadow duration-300">
            <div>
              <h3 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Business</h3>
              <p className="text-xl font-semibold mb-4 text-gray-900">$299 <span className="text-sm font-normal text-gray-500">/ month</span></p>
              <ul className="text-sm text-gray-700 space-y-3 mb-6">
                <li className="flex items-center gap-2"><span>✓</span> Unlimited Agents</li>
                <li className="flex items-center gap-2"><span>✓</span> Unlimited AI actions</li>
                <li className="flex items-center gap-2"><span>✓</span> 500GB Storage Quota</li>
                <li className="flex items-center gap-2"><span>✓</span> Unlimited Products</li>
              </ul>
            </div>
            <button onClick={() => handleUpgrade('Business')} className="w-full min-h-[44px] px-4 py-2 bg-gray-800 text-white rounded-lg font-medium hover:bg-gray-900 transition-colors shadow-sm flex items-center justify-center">
              Upgrade to Business via Stripe
            </button>
          </div>
        </div>

        <div className="text-center mt-4">
            <p className="text-sm text-gray-600 px-2">100% money back guarantee. Secure SSL payments powered by Stripe.</p>
        </div>

        <div className="mt-8 p-6 md:p-8 bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-lg rounded-[24px] w-full">
            <h2 className="text-xl font-bold font-outfit mb-4">Frequently Asked Questions</h2>
            <div className="mb-4">
                <h3 className="font-semibold">How do I upgrade, downgrade, or cancel?</h3>
                <p className="text-gray-700 text-sm mt-1">Self-serve billing! You can upgrade, downgrade, or cancel anytime straight from the My Plan page.</p>
            </div>
            <div>
                <h3 className="font-semibold">What is the storage limit?</h3>
                <p className="text-gray-700 text-sm mt-1">Storage limits vary by plan, starting at 500MB for Free and up to 500GB for Business.</p>
            </div>
        </div>

        <PoweredByOHC tenantId="ohc" />
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
