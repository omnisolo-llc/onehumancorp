"use client";

import React from 'react';
import { useRouter } from 'next/navigation';

const tiers = [
  {
    name: 'Free',
    price: '$0',
    description: 'Perfect for exploring and starting your journey.',
    features: [
      '100 AI actions / month',
      '500 MB Storage',
      '1 AI Agent',
      '10 Products',
      'Basic Analytics',
    ],
    buttonText: 'Current Plan',
    buttonClass: 'bg-gray-100 text-gray-800 cursor-default',
  },
  {
    name: 'Starter',
    price: '$29',
    unit: '/mo',
    description: 'Grow your business with more power and scale.',
    features: [
      '1,000 AI actions / month',
      '5 GB Storage',
      '3 AI Agents',
      '100 Products',
      'Standard Analytics',
      'Custom Domain Support',
    ],
    buttonText: 'Upgrade to Starter',
    buttonClass: 'bg-indigo-600 text-white hover:bg-indigo-700',
    highlight: true,
  },
  {
    name: 'Pro',
    price: '$79',
    unit: '/mo',
    description: 'For established businesses needing full automation.',
    features: [
      'Unlimited AI actions',
      '50 GB Storage',
      '10 AI Agents',
      'Unlimited Products',
      'Advanced Analytics & ROI',
      'Priority Support',
    ],
    buttonText: 'Upgrade to Pro',
    buttonClass: 'bg-indigo-600 text-white hover:bg-indigo-700',
  },
  {
    name: 'Business',
    price: '$299',
    unit: '/mo',
    description: 'Maximum scale for high-volume enterprises.',
    features: [
      'Unlimited AI actions',
      '500 GB Storage',
      'Unlimited AI Agents',
      'Unlimited Products',
      'White-label options',
      '24/7 Dedicated Concierge',
    ],
    buttonText: 'Upgrade to Business',
    buttonClass: 'bg-indigo-600 text-white hover:bg-indigo-700',
  },
];

export default function PricingPage() {
  const router = useRouter();

  const handleUpgrade = (tierName: string) => {
    // In a real scenario, this would redirect to a Stripe checkout session or a checkout page
    console.log(`Upgrading to ${tierName}`);
    router.push(`/checkout?plan=${tierName.toLowerCase()}`);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Choose Your Plan</h1>
        <button onClick={() => router.back()} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Go Back
        </button>
      </header>

      <main className="p-6 md:p-12 flex-1 max-w-7xl mx-auto w-full">
        <div className="text-center mb-12">
          <h2 className="text-4xl font-bold font-outfit text-gray-900 mb-4">Simple, Transparent Pricing</h2>
          <p className="text-xl text-gray-600">No hidden fees. AI-powered infrastructure at every scale.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {tiers.map((tier) => (
            <div
              key={tier.name}
              className={`flex flex-col p-8 shadow-sm transition-transform hover:scale-105 ${tier.highlight ? 'ring-2 ring-indigo-600' : ''}`}
              style={{
                background: 'rgba(255, 255, 255, 0.65)',
                backdropFilter: 'blur(20px) saturate(200%)',
                border: '1px solid rgba(255, 255, 255, 0.4)',
                borderRadius: '24px',
              }}
            >
              <div className="mb-6">
                <h3 className="text-2xl font-bold font-outfit text-gray-900">{tier.name}</h3>
                <div className="mt-4 flex items-baseline">
                  <span className="text-5xl font-extrabold font-outfit text-gray-900">{tier.price}</span>
                  {tier.unit && <span className="ml-1 text-xl font-medium text-gray-500">{tier.unit}</span>}
                </div>
                <p className="mt-4 text-gray-600 leading-relaxed">{tier.description}</p>
              </div>

              <ul className="flex-1 space-y-4 mb-8">
                {tier.features.map((feature) => (
                  <li key={feature} className="flex items-start gap-3">
                    <svg className="w-5 h-5 text-green-500 flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path>
                    </svg>
                    <span className="text-gray-700 text-sm font-medium">{feature}</span>
                  </li>
                ))}
              </ul>

              <button
                onClick={() => tier.name !== 'Free' && handleUpgrade(tier.name)}
                className={`w-full py-4 rounded-xl font-bold transition-colors ${tier.buttonClass}`}
              >
                {tier.buttonText}
              </button>
            </div>
          ))}
        </div>

        <div className="mt-16 text-center">
            <p className="text-gray-500 text-sm">
                * All plans include our OHC Premium Token library design system and real-time observability dashboards.<br/>
                Payments processed securely via Stripe. Automated WebP compression applied to all images.
            </p>
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
