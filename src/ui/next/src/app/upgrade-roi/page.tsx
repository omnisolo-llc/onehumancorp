"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';

export default function UpgradeROIPage() {
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
  const netProfitIncrease = revenueIncrease - proPlanCost;

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      {/* Header */}
      <header className="px-4 md:px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
        <h1 className="text-xl md:text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">Pro Plan ROI Calculator 📈</h1>
        <button
          onClick={() => router.push('/dashboard')}
          className="px-3 py-1.5 md:px-4 md:py-2 bg-gray-200 rounded-md text-xs md:text-sm font-medium hover:bg-gray-300 transition-colors"
        >
          Back to Dashboard
        </button>
      </header>

      <main className="p-4 md:p-8 flex-1 w-full max-w-5xl mx-auto flex flex-col md:flex-row gap-6 md:gap-8 items-start">
        {/* Input Section */}
        <section className="w-full md:w-1/2 p-6 md:p-8 shadow-md bg-white/65 backdrop-blur-md border border-white/40 rounded-2xl">
          <div className="mb-6">
             <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Calculate Your Growth</h2>
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
                <h3 className="text-sm font-bold text-indigo-900 mb-2">Pro Plan Features Include:</h3>
                <ul className="text-sm text-indigo-800 space-y-2">
                    <li className="flex items-center gap-2"><span>✨</span> AI-Powered Upsell Recommendations</li>
                    <li className="flex items-center gap-2"><span>📈</span> Automated Review Generation Emails</li>
                    <li className="flex items-center gap-2"><span>🎯</span> Smart Cart Abandonment Recovery</li>
                </ul>
            </div>
          </div>
        </section>

        {/* Results Section */}
        <section className="w-full md:w-1/2 flex flex-col gap-6">
          <div className="p-6 md:p-8 shadow-xl bg-gradient-to-br from-[#1D1D1F] to-[#2d2d32] rounded-2xl text-white relative overflow-hidden">
             {/* Decorative Background Elements */}
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
                  onClick={() => router.push('/checkout?tier=Pro')}
                  className="w-full py-4 bg-indigo-600 hover:bg-indigo-500 text-white font-bold rounded-xl shadow-[0_0_20px_rgba(79,70,229,0.3)] hover:shadow-[0_0_25px_rgba(79,70,229,0.5)] transition-all hover:-translate-y-0.5 active:translate-y-0 text-lg flex items-center justify-center gap-2"
                >
                  <span>🚀</span> Upgrade to Pro ($79/mo)
                </button>
                <p className="text-center text-xs text-gray-400">
                  Cancel anytime. 100% money-back guarantee.
                </p>
             </div>
          </div>
        </section>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
