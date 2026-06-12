"use client";

// Pricing Page Implementation
import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';
import { PoweredByOHC } from '../components/PoweredByOHC';


interface PricingPlan {
  id: string;
  name: string;
  price_cents: number;
  ai_action_limit: number | null;
  storage_limit_mb: number | null;
  agent_limit: number | null;
  product_limit: number | null;
}

export default function PricingPage() {
  const [plans, setPlans] = useState<PricingPlan[]>([]);
  const [currentPlan, setCurrentPlan] = useState<string>('Free');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchPricingData() {
      try {
        const token = localStorage.getItem('token');
        const headers = { 'Authorization': `Bearer ${token}` };

        const [plansRes, myPlanRes] = await Promise.all([
          fetch('/api/billing/pricing-plans', { headers }),
          fetch('/api/billing/my-plan', { headers })
        ]);

        if (plansRes.ok) {
            const result = await plansRes.json();
            setPlans(result.plans || []);
        }

        if (myPlanRes.ok) {
            const planResult = await myPlanRes.json();
            setCurrentPlan(planResult.current_plan || 'Free');
        }
      } catch (err) {
        console.error("Error fetching pricing data", err);
      } finally {
        setLoading(false);
      }
    }
    fetchPricingData();
  }, []);

  if (loading) {
      return (
          <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900 w-full overflow-x-hidden p-4 md:p-8" data-testid="pricing-loading">
              <div className="max-w-6xl mx-auto w-full flex flex-col gap-6 animate-pulse">
                  <div className="h-10 bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-xl w-1/4"></div>
                  <div className="h-64 bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 rounded-2xl w-full"></div>
              </div>
          </div>
      );
  }

  const formatStorage = (mb: number | null) => {
      if (mb === null) return "Unlimited";
      if (mb >= 1024) return (mb / 1024).toFixed(0) + "GB";
      return mb + "MB";
  };

  const router = useRouter();

  const handleUpgrade = (tier: string) => {
    router.push('/checkout?tier=' + tier);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900 w-full overflow-x-hidden">
      <header className="px-4 py-4 flex items-center justify-between sticky top-0 z-50 bg-white/70 backdrop-blur-xl saturate-200 border-b border-white/40 shadow-sm w-full">
        <WithTooltip id="pricing-tier-tooltip" defaultText="Select the plan that best fits your business needs.">
          <h1 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">Pricing Plans</h1>
        </WithTooltip>
        <button onClick={() => router.push('/dashboard')} className="min-w-[44px] min-h-[44px] px-3 py-2 bg-gray-100 rounded-xl text-sm font-medium text-gray-800 hover:bg-gray-200 transition-colors flex items-center justify-center">
          Back
        </button>
      </header>

      <main id="pricing-screen" className="p-4 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-6">
        <div className="text-center mb-4 md:mb-8 max-w-2xl mx-auto">
          <p className="text-base md:text-lg text-gray-600 leading-relaxed">Plain-language pricing — no hidden fees. Choose the best plan to grow your small business.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 md:gap-6 w-full">
          {plans.map((plan) => {
            const isCurrent = currentPlan.toLowerCase() === plan.name.toLowerCase();
            const isRecommended = plan.name.toLowerCase() === 'starter';

            return (
              <div key={plan.id} className={`p-6 flex flex-col justify-between app-card bg-white/70 backdrop-blur-xl saturate-200 ${isRecommended ? 'border-indigo-200 shadow-xl hover:shadow-2xl' : 'border-white/40 hover:shadow-xl'} transition-shadow duration-300 w-full rounded-2xl relative`}>
                {isRecommended && <div className="absolute top-0 right-0 bg-indigo-600 text-white text-xs font-bold px-3 py-1 rounded-bl-xl rounded-tr-2xl">Recommended</div>}
                <div>
                  <h3 className="text-2xl font-bold font-outfit mb-2 text-gray-900">{plan.name}</h3>
                  <p className="text-xl font-semibold mb-2 text-gray-900">${plan.price_cents / 100} <span className="text-sm font-normal text-gray-500">/ month</span></p>
                  {isRecommended && <p className="text-xs text-indigo-600 font-medium mb-4">Suggested for growing stores</p>}
                  {!isRecommended && <div className="h-8"></div>}
                  <ul className="text-sm text-gray-700 space-y-3 mb-6 mt-2">
                    <li className="flex items-center gap-2"><span>✓</span> {plan.agent_limit === null ? 'Unlimited' : plan.agent_limit} Agent Limit</li>
                    <li className="flex items-center gap-2"><span>✓</span> {plan.ai_action_limit === null ? 'Unlimited' : plan.ai_action_limit.toLocaleString()} AI actions / month</li>
                    <li className="flex items-center gap-2"><span>✓</span> {formatStorage(plan.storage_limit_mb)} Storage Quota</li>
                    <li className="flex items-center gap-2"><span>✓</span> {plan.product_limit === null ? 'Unlimited' : plan.product_limit} Products Limit</li>
                  </ul>
                </div>
                {isCurrent ? (
                  <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 rounded-xl font-medium flex items-center justify-center cursor-not-allowed" disabled>
                    Current Plan
                  </button>
                ) : (
                  <button onClick={() => handleUpgrade(plan.name)} className={`w-full min-h-[44px] px-4 py-2 ${isRecommended ? 'bg-indigo-600 text-white hover:bg-indigo-700' : 'bg-gray-900 text-white hover:bg-black'} rounded-xl font-medium transition-colors shadow-sm flex items-center justify-center`}>
                    Upgrade to {plan.name}
                  </button>
                )}
              </div>
            );
          })}
        </div>

        <div className="text-center mt-4 mb-2">
            <p className="text-xs md:text-sm text-gray-500 px-2">100% money back guarantee. Secure SSL payments powered by Stripe.</p>
        </div>

        <div className="p-6 app-card bg-white/70 backdrop-blur-xl saturate-200 border border-white/40 w-full mt-2 rounded-2xl">
            <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900">Frequently Asked Questions</h2>
            <div className="space-y-4">
              <div>
                  <h3 className="font-semibold text-gray-800">How do I upgrade, downgrade, or cancel?</h3>
                  <p className="text-gray-600 text-sm mt-1 leading-relaxed">Stripe Billing for self-serve plan upgrades, downgrades, and cancellation. You can upgrade, downgrade, or cancel anytime straight from the My Plan page.</p>
              </div>
              <div>
                  <h3 className="font-semibold text-gray-800">What is the storage limit?</h3>
                  <p className="text-gray-600 text-sm mt-1 leading-relaxed">Storage limits vary by plan, starting at 500MB for Free and up to 500GB for Business.</p>
              </div>
            </div>
        </div>

        <div className="flex justify-center mt-4">
          <PoweredByOHC tenantId="ohc" />
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
