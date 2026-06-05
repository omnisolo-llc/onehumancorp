"use client";

// Pricing Page Implementation
import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';

export default function PricingPage() {
  const router = useRouter();
  const [monthlyOrders, setMonthlyOrders] = useState<number>(50);
  const [averageOrderValue, setAverageOrderValue] = useState<number>(40);

  // Growth assumptions with Pro Plan (Advanced AI Marketing + SEO + Review Automation)
  const conversionUplift = 0.25; // 25% increase in conversions
  const aovUplift = 0.15; // 15% increase in Average Order Value from AI cross-selling
  const proPlanCost = 79; // $79/mo

  const currentRevenue = monthlyOrders * averageOrderValue;

  const projectedOrders = Math.round(monthlyOrders * (1 + conversionUplift));
  const projectedAOV = averageOrderValue * (1 + aovUplift);
  const projectedRevenue = projectedOrders * projectedAOV;

  const revenueIncrease = projectedRevenue - currentRevenue;

  const handleUpgrade = (tier: string) => {
    router.push('/checkout?tier=' + tier);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-4 md:px-6 py-4 flex flex-col md:flex-row items-center justify-between border-b gap-4" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <WithTooltip id="pricing-tier-tooltip" defaultText="Select the plan that best fits your business needs.">
          <h1 className="text-2xl font-bold font-outfit text-center md:text-left" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Pricing Plans</h1>
        </WithTooltip>
        <button onClick={() => router.push('/dashboard')} className="min-w-[44px] min-h-[44px] px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors flex items-center justify-center">
          Back to Dashboard
        </button>
      </header>

      <main id="pricing-screen" className="p-4 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-8">
        <div className="text-center mb-8">
          <p className="text-lg" style={{ color: '#86868B' }}>Plain-language pricing — no hidden fees. Choose the best plan to grow your small business.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {/* Free Tier */}
          <div className="p-8 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <div>
              <h3 className="text-2xl font-bold font-outfit mb-2" style={{ color: '#1D1D1F' }}>Free</h3>
              <p className="text-xl font-semibold mb-4" style={{ color: '#1D1D1F' }}>$0 <span className="text-sm font-normal text-gray-500">/ month</span></p>
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
          <div className="p-6 md:p-8 shadow-sm flex flex-col justify-between relative" style={{ background: 'rgba(255, 255, 255, 0.85)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(79, 70, 229, 0.5)', borderRadius: '16px', boxShadow: '0 12px 48px rgba(79, 70, 229, 0.15)' }}>
            <div className="absolute top-0 right-0 bg-indigo-600 text-white text-xs font-bold px-3 py-1 rounded-bl-lg rounded-tr-lg">Recommended</div>
            <div>
              <h3 className="text-2xl font-bold font-outfit mb-2" style={{ color: '#1D1D1F' }}>Starter</h3>
              <p className="text-xl font-semibold mb-2" style={{ color: '#1D1D1F' }}>$29 <span className="text-sm font-normal text-gray-500">/ month</span></p>
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
          <div className="p-6 md:p-8 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <div>
              <h3 className="text-2xl font-bold font-outfit mb-2" style={{ color: '#1D1D1F' }}>Pro</h3>
              <p className="text-xl font-semibold mb-4" style={{ color: '#1D1D1F' }}>$79 <span className="text-sm font-normal text-gray-500">/ month</span></p>
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
          <div className="p-6 md:p-8 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <div>
              <h3 className="text-2xl font-bold font-outfit mb-2" style={{ color: '#1D1D1F' }}>Business</h3>
              <p className="text-xl font-semibold mb-4" style={{ color: '#1D1D1F' }}>$299 <span className="text-sm font-normal text-gray-500">/ month</span></p>
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

        {/* ROI Calculator Section */}
        <section className="mt-12 flex flex-col md:flex-row gap-6 md:gap-8 items-start">
          {/* Input Section */}
          <div className="w-full md:w-1/2 p-6 md:p-8 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
            <div className="mb-6">
               <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Calculate Your Pro Plan ROI</h2>
               <p className="text-sm text-gray-600">
                 See how much extra revenue you could generate by unlocking Advanced AI Marketing, Automated Review Campaigns, and Smart Cross-Selling with the Pro Plan.
               </p>
            </div>

            <div className="flex flex-col gap-6">
              <div>
                <label className="flex justify-between text-sm font-semibold text-gray-800 mb-2">
                  <span>Current Monthly Orders</span>
                  <span className="text-indigo-600 bg-indigo-50 px-2 py-0.5 rounded">{monthlyOrders}</span>
                </label>
                <input
                  type="range"
                  min="10"
                  max="500"
                  step="10"
                  value={monthlyOrders}
                  onChange={(e) => setMonthlyOrders(Number(e.target.value))}
                  className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-indigo-600"
                />
              </div>

              <div>
                <label className="flex justify-between text-sm font-semibold text-gray-800 mb-2">
                  <span>Average Order Value ($)</span>
                  <span className="text-indigo-600 bg-indigo-50 px-2 py-0.5 rounded">${averageOrderValue}</span>
                </label>
                <input
                  type="range"
                  min="10"
                  max="200"
                  step="5"
                  value={averageOrderValue}
                  onChange={(e) => setAverageOrderValue(Number(e.target.value))}
                  className="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer accent-indigo-600"
                />
              </div>

              <div className="mt-4 p-4 bg-indigo-50 rounded-xl border border-indigo-100">
                  <h3 className="text-sm font-bold text-indigo-900 mb-2">Pro Plan Growth Levers:</h3>
                  <ul className="text-sm text-indigo-800 space-y-2">
                      <li className="flex items-center gap-2"><span>✨</span> AI-Powered Upsell Recommendations</li>
                      <li className="flex items-center gap-2"><span>📈</span> Automated Review Generation Emails</li>
                      <li className="flex items-center gap-2"><span>🎯</span> Smart Cart Abandonment Recovery</li>
                  </ul>
              </div>
            </div>
          </div>

          {/* Results Section */}
          <div className="w-full md:w-1/2 flex flex-col gap-6">
            <div className="p-6 md:p-8 shadow-xl bg-gradient-to-br from-[#1D1D1F] to-[#2d2d32] rounded-2xl text-white relative overflow-hidden" style={{ borderRadius: '16px' }}>
               <div className="absolute top-0 right-0 w-48 h-48 bg-indigo-500/20 rounded-bl-full blur-2xl pointer-events-none"></div>
               <div className="absolute bottom-0 left-0 w-32 h-32 bg-purple-500/20 rounded-tr-full blur-xl pointer-events-none"></div>

               <h2 className="text-xl font-bold font-outfit mb-6 text-gray-200">Your Projected Impact</h2>

               <div className="grid grid-cols-2 gap-4 mb-8">
                   <div className="p-4 bg-white/10 rounded-xl border border-white/5 backdrop-blur-sm">
                       <p className="text-xs text-gray-400 uppercase tracking-wider font-semibold mb-1">Current Revenue</p>
                       <p className="text-2xl font-bold">${currentRevenue.toLocaleString()}</p>
                   </div>
                   <div className="p-4 bg-white/10 rounded-xl border border-indigo-500/30 backdrop-blur-sm relative">
                       <div className="absolute -top-2 -right-2 bg-indigo-500 text-[10px] font-bold px-2 py-0.5 rounded-full uppercase tracking-wider">With Pro</div>
                       <p className="text-xs text-indigo-200 uppercase tracking-wider font-semibold mb-1">Projected Revenue</p>
                       <p className="text-2xl font-bold text-white">${Math.round(projectedRevenue).toLocaleString()}</p>
                   </div>
               </div>

               <div className="border-t border-white/10 pt-6 mb-8">
                   <p className="text-sm text-gray-400 mb-2">Estimated Monthly Growth</p>
                   <div className="flex items-baseline gap-3">
                       <span className="text-4xl md:text-5xl font-black font-outfit text-transparent bg-clip-text bg-gradient-to-r from-green-400 to-emerald-300">
                           +${Math.round(revenueIncrease).toLocaleString()}
                       </span>
                       <span className="text-green-400 font-semibold text-lg">/ mo</span>
                   </div>
                   <p className="text-sm text-gray-400 mt-2">
                       That's an extra <strong className="text-white">${Math.round(revenueIncrease * 12).toLocaleString()}</strong> a year!
                   </p>
               </div>

               <div className="flex flex-col gap-3">
                  <button
                    onClick={() => handleUpgrade('Pro')}
                    className="w-full py-4 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-xl shadow-[0_0_20px_rgba(79,70,229,0.3)] hover:shadow-[0_0_25px_rgba(79,70,229,0.5)] transition-all hover:-translate-y-0.5 active:translate-y-0 text-lg flex items-center justify-center gap-2"
                  >
                    <span>🚀</span> Upgrade to Pro Now
                  </button>
               </div>
            </div>
          </div>
        </section>

        <div className="mt-8 p-6 md:p-8 shadow-sm w-full" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(20px) saturate(200%)', border: '1px solid rgba(255, 255, 255, 0.2)', borderRadius: '16px', boxShadow: '0 8px 32px rgba(0, 0, 0, 0.05)' }}>
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

      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
