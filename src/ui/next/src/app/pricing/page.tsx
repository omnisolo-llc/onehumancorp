"use client";

// Pricing Page Implementation
import React, { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { WithTooltip } from '../../components/TooltipRegistry';
import { PoweredByOHC } from '../components/PoweredByOHC';
import { ViralTrialExtensionWidget } from '../components/ViralTrialExtensionWidget';

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
        <div className="mb-8 p-6 app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-indigo-200/50 shadow-xl rounded-2xl w-full">
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
                <div className="p-4 bg-white rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">AI Actions Used</p>
                    <p className="text-xl font-bold text-gray-900">
                        {planDetails?.ai_actions_used || 0}
                        <span className="text-sm font-normal text-gray-500 ml-1">/ {planDetails?.ai_actions_limit || '∞'}</span>
                    </p>
                </div>
                <div className="p-4 bg-white rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Storage Used</p>
                    <p className="text-xl font-bold text-gray-900">
                        {planDetails?.storage_used_bytes ? (planDetails.storage_used_bytes / (1024 * 1024)).toFixed(1) : 0} MB
                        <span className="text-sm font-normal text-gray-500 ml-1">
                            / {planDetails?.storage_limit_bytes ? (planDetails.storage_limit_bytes / (1024 * 1024)).toFixed(0) + ' MB' : '∞'}
                        </span>
                    </p>
                </div>
                <div className="p-4 bg-white rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Estimated Next Bill</p>
                    <p className="text-xl font-bold text-gray-900">
                        ${((planDetails?.next_bill_estimated || 0) / 100).toFixed(2)}
                    </p>
                </div>
            </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 md:gap-6 w-full">
          {/* Free Tier */}
          <div className="p-6 flex flex-col justify-between app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg rounded-2xl hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 w-full">
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
            {loading ? (
              <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-500 rounded-xl font-medium flex items-center justify-center cursor-not-allowed" disabled>
                Loading...
              </button>
            ) : currentPlan === 'Free' || !currentPlan ? (
              <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 rounded-xl font-medium flex items-center justify-center cursor-not-allowed" disabled>
                Current Plan
              </button>
            ) : (
              <button onClick={handleManageBilling} className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 rounded-xl font-medium flex items-center justify-center hover:bg-gray-300 transition-colors">
                Downgrade to Free
              </button>
            )}
            {(!loading && (currentPlan === 'Free' || !currentPlan)) && (
              <ViralTrialExtensionWidget />
            )}
          </div>

          {/* Starter Tier */}
          <div className="p-6 flex flex-col justify-between relative app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-xl rounded-2xl hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 w-full">
            <div className="absolute top-0 right-0 bg-indigo-600 text-white text-xs font-bold px-3 py-1 rounded-bl-xl rounded-tr-2xl">Recommended</div>
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
            {loading ? (
              <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-500 rounded-xl font-medium flex items-center justify-center cursor-not-allowed" disabled>
                Loading...
              </button>
            ) : currentPlan === 'Starter' ? (
              <button onClick={handleManageBilling} className="w-full min-h-[44px] px-4 py-2 bg-indigo-100 text-indigo-700 hover:bg-indigo-200 rounded-xl font-medium flex items-center justify-center transition-colors">
                Manage Plan
              </button>
            ) : (
              <button onClick={() => handleUpgrade('Starter')} className="w-full min-h-[44px] px-4 py-2 bg-indigo-600 text-white hover:bg-indigo-700 rounded-xl font-medium transition-colors shadow-sm flex items-center justify-center">
                Upgrade to Starter via Stripe
              </button>
            )}
          </div>

          {/* Pro Tier */}
          <div className="p-6 flex flex-col justify-between app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg rounded-2xl hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 w-full">
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
            {loading ? (
              <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-500 rounded-xl font-medium flex items-center justify-center cursor-not-allowed" disabled>
                Loading...
              </button>
            ) : currentPlan === 'Pro' ? (
              <button onClick={handleManageBilling} className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 hover:bg-gray-300 rounded-xl font-medium flex items-center justify-center transition-colors">
                Manage Plan
              </button>
            ) : (
              <button onClick={() => handleUpgrade('Pro')} className="w-full min-h-[44px] px-4 py-2 bg-gray-900 text-white hover:bg-black rounded-xl font-medium transition-colors shadow-sm flex items-center justify-center">
                Upgrade to Pro via Stripe
              </button>
            )}
          </div>

          {/* Business Tier */}
          <div className="p-6 flex flex-col justify-between app-card ohc-growth-card glass-card backdrop-blur-xl bg-white/40 border border-white/20 shadow-lg rounded-2xl hover:-translate-y-1 hover:shadow-2xl transition-all duration-300 w-full">
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
            {loading ? (
              <button className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-500 rounded-xl font-medium flex items-center justify-center cursor-not-allowed" disabled>
                Loading...
              </button>
            ) : currentPlan === 'Business' ? (
              <button onClick={handleManageBilling} className="w-full min-h-[44px] px-4 py-2 bg-gray-200 text-gray-800 hover:bg-gray-300 rounded-xl font-medium flex items-center justify-center transition-colors">
                Manage Plan
              </button>
            ) : (
              <button onClick={() => handleUpgrade('Business')} className="w-full min-h-[44px] px-4 py-2 bg-gray-900 text-white hover:bg-black rounded-xl font-medium transition-colors shadow-sm flex items-center justify-center">
                Upgrade to Business via Stripe
              </button>
            )}
          </div>
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
