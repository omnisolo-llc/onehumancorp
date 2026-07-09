"use client";

// Pricing Page Implementation
import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { WithTooltip } from '../../components/TooltipRegistry';
import { PoweredByOHC } from '../components/PoweredByOHC';
import { ViralTrialExtensionWidget } from '../components/ViralTrialExtensionWidget';
import { PricingCard } from './PricingCard';


export default function PricingPage() {
  const router = useRouter();

  const [currentPlan, setCurrentPlan] = useState<string | null>(null);
  const [planDetails, setPlanDetails] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchPlanData = async () => {
      try {
        const token = localStorage.getItem('token');
        const response = await fetch('/api/billing/my-plan', {
          headers: token ? { 'Authorization': `Bearer ${token}` } : {}
        });
        if (response.ok) {
          const json = await response.json();
          setCurrentPlan(json.current_plan);
          setPlanDetails(json);
        }
      } catch (error) {
        console.error('Failed to fetch plan data:', error);
      } finally {
        setLoading(false);
      }
    };

    fetchPlanData();
  }, []);

  const handleManageBilling = async () => {
    try {
      const token = localStorage.getItem('token');
      const response = await fetch('/api/billing/create-billing-portal-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(token ? { 'Authorization': `Bearer ${token}` } : {})
        },
      });

      if (!response.ok) {
        throw new Error('Failed to create billing portal session');
      }

      const data = await response.json();
      if (data.url) {
        window.location.href = data.url;
      }
    } catch (error) {
      console.error('Upgrade error:', error);
      alert('Failed to initiate billing portal. Please try again.');
    }
  };

  const handleUpgrade = async (tier: string) => {
    try {
      const token = localStorage.getItem('token');
      const response = await fetch('/api/billing/create-checkout-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(token ? { 'Authorization': `Bearer ${token}` } : {})
        },
        body: JSON.stringify({ tier }),
      });

      if (!response.ok) {
        throw new Error('Failed to create checkout session');
      }

      const data = await response.json();
      if (data.checkout_url) {
        window.location.href = data.checkout_url;
      }
    } catch (error) {
      console.error('Error upgrading plan:', error);
      alert('Failed to initiate upgrade. Please try again.');
    }
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-gradient-to-br from-indigo-50 via-white to-purple-50 text-gray-900 w-full overflow-x-hidden max-w-[100vw]">
      <header className="px-4 py-4 flex items-center justify-between sticky top-0 z-50 app-panel-header shadow-sm w-full">
        <WithTooltip id="pricing-tier-tooltip" defaultText="Select the plan that best fits your business needs.">
          <h1 className="text-xl md:text-2xl font-bold font-outfit text-gray-900 tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-gray-900 to-gray-600">Pricing Plans</h1>
        </WithTooltip>
        <Link href="/dashboard" className="min-w-[44px] min-h-[44px] px-3 py-2 bg-gray-100 rounded-xl text-sm font-medium text-gray-800 hover:bg-gray-200 transition-colors flex items-center justify-center">Back to Dashboard</Link>
      </header>

      <main id="pricing-screen" className="p-4 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-6">
        <div className="text-center mb-4 md:mb-8 max-w-2xl mx-auto">
          <p className="text-base md:text-lg text-gray-600 leading-relaxed">Plain-language pricing — no hidden fees. Choose the best plan to grow your small business.</p>
        </div>

        {/* My Plan Section */}
        <div className="mb-8 p-6 app-card ohc-growth-card glass-card shadow-xl rounded-2xl w-full">
            <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-6">
                <div>
                    <h2 className="text-2xl font-bold font-outfit text-gray-900">My Plan: {currentPlan || 'Free'}</h2>
                    <p className="text-sm text-gray-500 mt-1">Cost transparency and usage tracking</p>
                </div>
                <button onClick={handleManageBilling} className="min-h-[44px] px-6 py-2 bg-indigo-600 text-white hover:bg-indigo-700 rounded-xl font-medium transition-colors shadow-sm flex items-center justify-center whitespace-nowrap">
                    Manage Plan & Billing
                </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="p-4 bg-white/60 rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">AI Actions Used</p>
                    <p className="text-xl font-bold text-gray-900">
                        {planDetails?.ai_actions_used || 0}
                        <span className="text-sm font-normal text-gray-500 ml-1">/ {planDetails?.ai_actions_limit || '∞'}</span>
                    </p>
                </div>
                <div className="p-4 bg-white/60 rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Storage Used</p>
                    <p className="text-xl font-bold text-gray-900">
                        {planDetails?.storage_used_bytes ? (planDetails.storage_used_bytes / (1024 * 1024)).toFixed(1) : 0} MB
                        <span className="text-sm font-normal text-gray-500 ml-1">
                            / {planDetails?.storage_limit_bytes ? (planDetails.storage_limit_bytes / (1024 * 1024)).toFixed(0) + ' MB' : '∞'}
                        </span>
                    </p>
                </div>
                <div className="p-4 bg-white/60 rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Estimated Next Bill</p>
                    <p className="text-xl font-bold text-gray-900">
                        ${((planDetails?.next_bill_estimated || 0) / 100).toFixed(2)}
                    </p>
                </div>
            </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 md:gap-6 w-full">
          <PricingCard
            tierName="Free"
            price="$0"
            features={["1 Agent Limit", "100 AI actions / month", "500MB Storage Quota", "10 Products Limit"]}
            currentPlan={currentPlan}
            loading={loading}
            onManageBilling={handleManageBilling}
            onUpgrade={handleUpgrade}
          />
          <PricingCard
            tierName="Starter"
            price="$29"
            isRecommended={true}
            recommendationText="Suggested for growing stores"
            features={["3 Agents Limit", "1,000 AI actions / month", "5GB Storage Quota", "100 Products Limit"]}
            currentPlan={currentPlan}
            loading={loading}
            onManageBilling={handleManageBilling}
            onUpgrade={handleUpgrade}
          />
          <PricingCard
            tierName="Pro"
            price="$79"
            features={["10 Agents Limit", "Unlimited AI actions", "50GB Storage Quota", "Unlimited Products"]}
            currentPlan={currentPlan}
            loading={loading}
            onManageBilling={handleManageBilling}
            onUpgrade={handleUpgrade}
          />
          <PricingCard
            tierName="Business"
            price="$299"
            features={["Unlimited Agents", "Unlimited AI actions", "500GB Storage Quota", "Unlimited Products"]}
            currentPlan={currentPlan}
            loading={loading}
            onManageBilling={handleManageBilling}
            onUpgrade={handleUpgrade}
          />
        </div>

        <div className="text-center mt-4 mb-2">
            <p className="text-xs md:text-sm text-gray-500 px-2">100% money back guarantee. Secure SSL payments powered by Stripe.</p>
        </div>

        <div className="p-6 app-card ohc-growth-card glass-panel w-full mt-2">
            <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900">Frequently Asked Questions</h2>
            <div className="space-y-4">
              <div>
                  <h3 className="font-semibold text-gray-800">How do I upgrade, downgrade, or cancel?</h3>
                  <p className="text-gray-600 text-sm mt-1 leading-relaxed">Stripe Billing for self-serve plan upgrades, downgrades, and cancellation. You can upgrade, downgrade, or cancel anytime straight from the My Plan page or by clicking "Manage Plan" above.</p>
                  <button onClick={handleManageBilling} className="mt-2 text-indigo-600 hover:text-indigo-800 text-sm font-medium underline">Manage Billing Portal</button>
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
        /* The .ohc-growth-card styles are now managed globally in globals.css for design token consistency */
      `}} />
    </div>
  );
}
