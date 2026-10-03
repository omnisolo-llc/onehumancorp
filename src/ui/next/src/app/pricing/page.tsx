"use client";

// Pricing Page Implementation
import React, { useEffect, useRef, useState } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { WithTooltip } from '../../components/TooltipRegistry';
import { PoweredByOmniSolo } from '../components/PoweredByOmniSolo';
import '../components/ViralTrialExtensionWidget';
import { PricingCard } from './PricingCard';
import { fetchForOwnedBusinessAction, fetchForOwnedBusinessRead, onboardingOwner, onboardingSessionEpoch, openOnboardingSession, subscribeOnboardingInvalidation } from '../onboarding/draftSession';
import { hasVerifiedOfflineQueueOwner, QUEUE_IDENTITY_EPOCH_KEY, sameOwner, subscribeQueueIdentityReadiness, type QueueOwner } from '@/lib/sync/queueIdentity';

type BillingScope = { owner: QueueOwner; epoch: number; storageEpoch: string | null };
function billingScopeActive(scope: BillingScope | null, requireFreshIdentity = true): scope is BillingScope {
  const owner = onboardingOwner();
  try {
    return !!scope && !!owner && scope.epoch === onboardingSessionEpoch() && sameOwner(scope.owner, owner)
      && scope.storageEpoch === localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY) && (!requireFreshIdentity || hasVerifiedOfflineQueueOwner(scope.owner));
  } catch { return false; }
}


type PlanSummary = Partial<import('@/lib/business-records').BillingPlan> & { current_plan: string };
const metric = (value: unknown): number | undefined => typeof value === 'number' && Number.isSafeInteger(value) && value >= 0 ? value : undefined;
const limit = (value: unknown): number | null | undefined => value === null ? null : metric(value);
function readPlanSummary(value: unknown): PlanSummary | null {
  if (!value || typeof value !== 'object' || Array.isArray(value)) return null;
  const data = value as Record<string, unknown>;
  if (data.error != null || ('success' in data && data.success !== true) || typeof data.current_plan !== 'string') return null;
  const current_plan = ['Free', 'Starter', 'Pro', 'Business'].find(plan => plan.toLowerCase() === (data.current_plan as string).toLowerCase());
  if (!current_plan) return null;
  return {
    current_plan,
    ai_actions_used: metric(data.ai_actions_used), ai_actions_limit: limit(data.ai_actions_limit),
    storage_used_bytes: metric(data.storage_used_bytes), storage_limit_bytes: limit(data.storage_limit_bytes),
    next_bill_estimated: typeof data.next_bill_estimated === 'number' && Number.isSafeInteger(data.next_bill_estimated) ? data.next_bill_estimated : undefined,
  };
}

export default function PricingPage() {
  useRouter();

  const [currentPlan, setCurrentPlan] = useState<string | null>(null);
  const [planDetails, setPlanDetails] = useState<PlanSummary | null>(null);
  const [loading, setLoading] = useState(true);
  const [isAnnual, setIsAnnual] = useState(false);
  const [billingError, setBillingError] = useState<string | null>(null);
  const checkoutPending = useRef(false);
  const portalPending = useRef(false);
  const scope = useRef<BillingScope | null>(null);

  useEffect(() => {
    let active = true;
    let retired = false;
    const retire = () => {
      retired = true; scope.current = null;
      if (active) {
        setCurrentPlan(null); setPlanDetails(null); setLoading(false);
        setBillingError('Your session changed. Reload pricing to verify billing access.');
      }
    };
    const unsubscribeSession = subscribeOnboardingInvalidation(retire);
    const unsubscribeIdentity = subscribeQueueIdentityReadiness(() => {
      if (scope.current && hasVerifiedOfflineQueueOwner() && !hasVerifiedOfflineQueueOwner(scope.current.owner)) retire();
    });
    void (async () => {
      try {
        const owner = await openOnboardingSession();
        if (!active || retired) return;
        const current: BillingScope = { owner, epoch: onboardingSessionEpoch(), storageEpoch: localStorage.getItem(QUEUE_IDENTITY_EPOCH_KEY) };
        scope.current = current;
        const response = await fetchForOwnedBusinessRead('/api/v1/billing/my-plan', owner);
        const summary = response.status === 200 ? readPlanSummary(await response.json()) : null;
        if (!active || scope.current !== current || !billingScopeActive(current)) return;
        setCurrentPlan(summary?.current_plan ?? null); setPlanDetails(summary);
      } catch {
        if (active && !retired) { setCurrentPlan(null); setPlanDetails(null); }
      } finally {
        if (active && !retired) setLoading(false);
      }
    })();
    return () => { active = false; scope.current = null; unsubscribeSession(); unsubscribeIdentity(); };
  }, []);

  const handleManageBilling = async () => {
    const current = scope.current;
    if (portalPending.current || !billingScopeActive(current, false)) return;
    const active = () => scope.current === current && billingScopeActive(current);
    portalPending.current = true;
    setBillingError(null);
    try {
      const response = await fetchForOwnedBusinessAction('/api/v1/billing/create-billing-portal-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
      }, current.owner, () => { if (!active()) throw new Error('Billing view changed'); });

      if (!response.ok || ![200, 201].includes(response.status)) {
        throw new Error('Failed to create billing portal session');
      }

      const data: unknown = await response.json();
      if (!data || typeof data !== 'object' || Array.isArray(data)
        || ('success' in data && data.success !== true) || ('error' in data && data.error != null)
        || !('url' in data) || typeof data.url !== 'string'
        || data.url.trim() !== data.url || data.url.includes('\\')) {
        throw new Error('Invalid billing portal receipt');
      }
      const portalUrl = new URL(data.url);
      if (portalUrl.protocol !== 'https:' || portalUrl.hostname !== 'billing.stripe.com'
        || portalUrl.port || portalUrl.username || portalUrl.password || portalUrl.pathname === '/') {
        throw new Error('Invalid billing portal destination');
      }
      if (active()) window.location.href = portalUrl.href;
    } catch {
      if (active()) setBillingError('The billing portal is unavailable. Please try again.');
    } finally {
      portalPending.current = false;
    }
  };

  const handleUpgrade = async (tier: string, isAnnualSelected?: boolean) => {
    const current = scope.current;
    if (checkoutPending.current || !billingScopeActive(current, false)) return;
    const active = () => scope.current === current && billingScopeActive(current);
    checkoutPending.current = true;
    setBillingError(null);
    try {
      const response = await fetchForOwnedBusinessAction('/api/v1/billing/create-checkout-session', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ tier, is_subscription: true, subscription_interval: isAnnualSelected ? 'year' : 'month' }),
      }, current.owner, () => { if (!active()) throw new Error('Billing view changed'); });

      if (!response.ok || ![200, 201].includes(response.status)) {
        throw new Error('Failed to create checkout session');
      }

      const data: unknown = await response.json();
      if (!data || typeof data !== 'object' || Array.isArray(data)
        || ('success' in data && data.success !== true) || ('error' in data && data.error != null)
        || !('checkout_url' in data) || typeof data.checkout_url !== 'string'
        || data.checkout_url.trim() !== data.checkout_url || data.checkout_url.includes('\\')) {
        throw new Error('Invalid checkout receipt');
      }
      const checkoutUrl = new URL(data.checkout_url);
      if (checkoutUrl.protocol !== 'https:' || checkoutUrl.hostname !== 'checkout.stripe.com'
        || checkoutUrl.port || checkoutUrl.username || checkoutUrl.password || checkoutUrl.pathname === '/') {
        throw new Error('Invalid checkout destination');
      }
      if (active()) window.location.href = checkoutUrl.href;
    } catch {
      if (active()) setBillingError('Checkout is unavailable. Your plan has not changed. Please try again.');
    } finally {
      checkoutPending.current = false;
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
        {billingError && <p role="alert" className="rounded-xl border border-red-200 bg-red-50 p-4 text-red-800">{billingError}</p>}
        <div className="text-center mb-4 md:mb-8 max-w-2xl mx-auto">
          <p className="text-base md:text-lg text-gray-600 leading-relaxed">Plain-language pricing — no hidden fees. Choose the best plan to grow your small business.</p>
        </div>

        <div className="flex justify-center mb-8">
          <label className="flex items-center cursor-pointer relative">
            <span className={`mr-3 text-sm font-medium ${!isAnnual ? 'text-gray-900' : 'text-gray-500'}`}>Monthly</span>
            <div className="relative">
              <input type="checkbox" id="billing-toggle" className="sr-only" checked={isAnnual} onChange={() => setIsAnnual(!isAnnual)} />
              <div className="block bg-gray-200 w-14 h-8 rounded-full"></div>
              <div className={`dot absolute left-1 top-1 bg-white w-6 h-6 rounded-full transition ${isAnnual ? 'transform translate-x-6 bg-indigo-600' : ''}`}></div>
            </div>
            <span className={`ml-3 text-sm font-medium flex items-center ${isAnnual ? 'text-gray-900' : 'text-gray-500'}`}>
              Annual <span className="ml-2 px-2 py-0.5 rounded-full bg-green-100 text-green-800 text-xs font-bold">Save 20%</span>
            </span>
          </label>
        </div>

        {/* My Plan Section */}
        <div className="mb-8 p-6 app-card omnisolo-growth-card glass-card backdrop-blur-2xl bg-white/40 border border-white/40 shadow-xl rounded-2xl w-full">
            <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 mb-6">
                <div>
                    <h2 className="text-2xl font-bold font-outfit text-gray-900">My Plan: {loading ? 'Verifying…' : currentPlan ?? 'Unavailable'}</h2>
                    <p className="text-sm text-gray-500 mt-1">Cost transparency and usage tracking</p>
                </div>
                <button onClick={handleManageBilling} disabled={loading || currentPlan === null} className="min-h-[44px] px-6 py-2 bg-indigo-600 text-white hover:bg-indigo-700 rounded-xl font-medium transition-colors shadow-sm flex items-center justify-center whitespace-nowrap">
                    Manage Plan & Billing
                </button>
            </div>

            {!loading && currentPlan === null && <p role="status">Current billing data is unavailable. <a href="/pricing">Retry plan lookup</a></p>}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                <div className="p-4 bg-white/60 rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">AI Actions Used</p>
                    <p className="text-xl font-bold text-gray-900">
                        {planDetails?.ai_actions_used ?? 'Unknown'}
                        <span className="text-sm font-normal text-gray-500 ml-1">{' / '}{planDetails?.ai_actions_limit === null ? 'Unlimited' : planDetails?.ai_actions_limit ?? 'Unknown'}</span>
                    </p>
                </div>
                <div className="p-4 bg-white/60 rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Storage Used</p>
                    <p className="text-xl font-bold text-gray-900">
                        {planDetails?.storage_used_bytes === undefined ? 'Unknown' : `${(planDetails.storage_used_bytes / (1024 * 1024)).toFixed(1)} MB`}
                        <span className="text-sm font-normal text-gray-500 ml-1">
                            {' / '}{planDetails?.storage_limit_bytes === null ? 'Unlimited' : planDetails?.storage_limit_bytes === undefined ? 'Unknown' : `${(planDetails.storage_limit_bytes / (1024 * 1024)).toFixed(0)} MB`}
                        </span>
                    </p>
                </div>
                <div className="p-4 bg-white/60 rounded-xl border border-gray-100">
                    <p className="text-xs text-gray-500 font-semibold uppercase tracking-wider mb-1">Estimated Next Bill</p>
                    <p className="text-xl font-bold text-gray-900">
                        {planDetails?.next_bill_estimated === undefined ? 'Unknown' : `$${(planDetails.next_bill_estimated / 100).toFixed(2)}`}
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
            basePrice={29}
            isAnnual={isAnnual}
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
            basePrice={79}
            isAnnual={isAnnual}
            features={["10 Agents Limit", "Unlimited AI actions", "50GB Storage Quota", "Unlimited Products"]}
            currentPlan={currentPlan}
            loading={loading}
            onManageBilling={handleManageBilling}
            onUpgrade={handleUpgrade}
          />
          <PricingCard
            tierName="Business"
            price="$299"
            basePrice={299}
            isAnnual={isAnnual}
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

        <div className="p-6 app-card omnisolo-growth-card glass-panel backdrop-blur-2xl bg-white/40 border border-white/40 w-full mt-2 rounded-2xl">
            <h2 className="text-xl font-bold font-outfit mb-4 text-gray-900">Frequently Asked Questions</h2>
            <div className="space-y-4">
              <div>
                  <h3 className="font-semibold text-gray-800">How do I upgrade, downgrade, or cancel?</h3>
                  <p className="text-gray-600 text-sm mt-1 leading-relaxed">Stripe Billing for self-serve plan upgrades, downgrades, and cancellation. You can upgrade, downgrade, or cancel anytime straight from the My Plan page or by clicking "Manage Plan" above.</p>
                  <button onClick={handleManageBilling} disabled={loading || currentPlan === null} className="mt-2 text-indigo-600 hover:text-indigo-800 text-sm font-medium underline">Manage Billing Portal</button>
              </div>
              <div>
                  <h3 className="font-semibold text-gray-800">What is the storage limit?</h3>
                  <p className="text-gray-600 text-sm mt-1 leading-relaxed">Storage limits vary by plan, starting at 500MB for Free and up to 500GB for Business.</p>
              </div>
            </div>
        </div>

        <div className="flex flex-col sm:flex-row justify-center items-center gap-4 mt-4">
          <a
            href="https://omnisolo.co"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-1.5 px-4 py-2 rounded-full border border-gray-200 bg-white/50 backdrop-blur-[30px] saturate-[210%] hover:bg-white/80 hover:shadow-sm transition-all text-xs font-semibold hover:text-indigo-600 uppercase tracking-widest font-outfit text-gray-600"
          >
            ⚡ OmniSolo
          </a>
          <PoweredByOmniSolo tenantId="omnisolo" />
        </div>
      </main>

      <style dangerouslySetInnerHTML={{__html: `

        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        /* The .omnisolo-growth-card styles are now managed globally in globals.css for design token consistency */
      `}} />
    </div>
  );
}
