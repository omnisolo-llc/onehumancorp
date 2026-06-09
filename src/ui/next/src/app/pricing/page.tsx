"use client";

// Pricing Page Implementation - Refactored for Premium OHC Aesthetic
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
    <div className="flex flex-col min-h-screen font-inter bg-[#f4f6f8] text-[#1D1D1F] selection:bg-indigo-100">
      {/* Background ambient glows */}
      <div className="fixed inset-0 overflow-hidden pointer-events-none z-0">
        <div className="absolute -top-[10%] -left-[10%] w-[40%] h-[40%] rounded-full bg-indigo-200/30 blur-[120px]" />
        <div className="absolute top-[20%] -right-[5%] w-[30%] h-[30%] rounded-full bg-purple-200/20 blur-[100px]" />
        <div className="absolute -bottom-[10%] left-[20%] w-[35%] h-[35%] rounded-full bg-blue-100/30 blur-[110px]" />
      </div>

      <header className="px-4 md:px-8 py-5 flex flex-col md:flex-row items-center justify-between gap-4 sticky top-0 z-50 bg-white/75 backdrop-blur-3xl saturate-[1.8] border-b border-white/20 shadow-[0_2px_20px_-10px_rgba(0,0,0,0.05)]">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 bg-indigo-600 rounded-xl flex items-center justify-center text-white font-bold text-xl shadow-lg shadow-indigo-200/50">O</div>
          <WithTooltip id="pricing-tier-tooltip" defaultText="Select the plan that best fits your business needs.">
            <h1 className="text-2xl font-extrabold font-outfit tracking-tight text-[#1D1D1F]">
              Subscription Plans
            </h1>
          </WithTooltip>
        </div>
        <button
          onClick={() => router.push('/dashboard')}
          className="min-w-[44px] h-11 px-5 bg-white/80 hover:bg-white border border-gray-200/50 rounded-2xl text-sm font-semibold text-gray-800 transition-all active:scale-95 shadow-sm flex items-center justify-center gap-2 group"
        >
          <svg className="w-4 h-4 transition-transform group-hover:-translate-x-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M15 19l-7-7 7-7" />
          </svg>
          Dashboard
        </button>
      </header>

      <main id="pricing-screen" className="relative z-10 p-4 md:p-12 flex-1 max-w-7xl mx-auto w-full flex flex-col gap-10">
        <div className="text-center max-w-2xl mx-auto">
          <h2 className="text-4xl md:text-5xl font-extrabold font-outfit mb-4 tracking-tight text-gray-900 leading-tight">
            Scale your <span className="text-transparent bg-clip-text bg-gradient-to-r from-indigo-600 to-purple-600">momentum</span>
          </h2>
          <p className="text-lg text-gray-600 font-medium leading-relaxed">
            Plain-language pricing with no hidden fees. AI-powered tools designed to help owners focus on what matters.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 items-stretch">
          {/* Free Tier */}
          <div className="p-8 flex flex-col justify-between bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-[0_8px_30px_rgb(0,0,0,0.04)] rounded-[32px] hover:shadow-[0_20px_40px_rgba(0,0,0,0.06)] transition-all duration-500 group">
            <div>
              <div className="mb-6">
                <h3 className="text-xl font-bold font-outfit text-gray-500 uppercase tracking-widest text-xs mb-2">Base</h3>
                <h4 className="text-3xl font-extrabold font-outfit text-gray-900 mb-2">Free</h4>
                <div className="flex items-baseline gap-1">
                  <span className="text-4xl font-extrabold font-outfit text-gray-900">$0</span>
                  <span className="text-sm font-semibold text-gray-500">/ month</span>
                </div>
              </div>
              <div className="h-px w-full bg-gradient-to-r from-transparent via-gray-200 to-transparent mb-8" />
              <ul className="space-y-4 mb-10">
                <PricingFeature text="1 Assistant Agent" />
                <PricingFeature text="100 AI actions / mo" />
                <PricingFeature text="500MB Secure Storage" />
                <PricingFeature text="10 Product Listings" />
              </ul>
            </div>
            <button className="w-full h-[52px] bg-gray-100 text-gray-400 rounded-2xl font-bold text-sm transition-colors flex items-center justify-center cursor-default" disabled>
              Current Active Plan
            </button>
          </div>

          {/* Starter Tier */}
          <div className="p-8 flex flex-col justify-between relative bg-white/90 backdrop-blur-2xl saturate-200 border-2 border-indigo-500/30 shadow-[0_20px_50px_rgba(79,70,229,0.1)] rounded-[32px] hover:shadow-[0_30px_60px_rgba(79,70,229,0.15)] transition-all duration-500 scale-[1.02] z-10 overflow-hidden group">
            <div className="absolute top-0 right-0 bg-indigo-600 text-white text-[10px] font-black px-4 py-1.5 rounded-bl-2xl uppercase tracking-tighter shadow-sm">Popular Choice</div>
            <div>
              <div className="mb-6">
                <h3 className="text-xl font-bold font-outfit text-indigo-600 uppercase tracking-widest text-xs mb-2">Growth</h3>
                <h4 className="text-3xl font-extrabold font-outfit text-gray-900 mb-2">Starter</h4>
                <div className="flex items-baseline gap-1">
                  <span className="text-4xl font-extrabold font-outfit text-gray-900">$29</span>
                  <span className="text-sm font-semibold text-gray-500">/ month</span>
                </div>
                <p className="text-xs text-indigo-600 font-bold mt-2 flex items-center gap-1.5">
                  <span className="w-1.5 h-1.5 rounded-full bg-indigo-600 animate-pulse" />
                  Best for growing operations
                </p>
              </div>
              <div className="h-px w-full bg-gradient-to-r from-transparent via-indigo-100 to-transparent mb-8" />
              <ul className="space-y-4 mb-10">
                <PricingFeature text="3 Specialized Agents" active />
                <PricingFeature text="1,000 AI actions / mo" active />
                <PricingFeature text="5GB High-speed Storage" active />
                <PricingFeature text="100 Product Listings" active />
                <PricingFeature text="Priority Response Times" active />
              </ul>
            </div>
            <button
              onClick={() => handleUpgrade('Starter')}
              className="w-full h-[52px] bg-indigo-600 text-white rounded-2xl font-bold text-sm hover:bg-indigo-700 transition-all active:scale-[0.98] shadow-[0_10px_20px_-5px_rgba(79,70,229,0.4)] flex items-center justify-center group-hover:translate-y-[-2px]"
            >
              Upgrade to Starter
            </button>
          </div>

          {/* Pro Tier */}
          <div className="p-8 flex flex-col justify-between bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-[0_8px_30px_rgb(0,0,0,0.04)] rounded-[32px] hover:shadow-[0_20px_40px_rgba(0,0,0,0.06)] transition-all duration-500 group">
            <div>
              <div className="mb-6">
                <h3 className="text-xl font-bold font-outfit text-gray-500 uppercase tracking-widest text-xs mb-2">Advanced</h3>
                <h4 className="text-3xl font-extrabold font-outfit text-gray-900 mb-2">Pro</h4>
                <div className="flex items-baseline gap-1">
                  <span className="text-4xl font-extrabold font-outfit text-gray-900">$79</span>
                  <span className="text-sm font-semibold text-gray-500">/ month</span>
                </div>
              </div>
              <div className="h-px w-full bg-gradient-to-r from-transparent via-gray-200 to-transparent mb-8" />
              <ul className="space-y-4 mb-10">
                <PricingFeature text="10 Multi-role Agents" />
                <PricingFeature text="Unlimited AI actions" />
                <PricingFeature text="50GB Cloud Storage" />
                <PricingFeature text="Unlimited Products" />
                <PricingFeature text="Advanced Analytics" />
              </ul>
            </div>
            <button
              onClick={() => handleUpgrade('Pro')}
              className="w-full h-[52px] bg-gray-900 text-white rounded-2xl font-bold text-sm hover:bg-black transition-all active:scale-[0.98] shadow-[0_10px_20px_-5px_rgba(0,0,0,0.2)] flex items-center justify-center group-hover:translate-y-[-2px]"
            >
              Upgrade to Pro
            </button>
          </div>

          {/* Business Tier */}
          <div className="p-8 flex flex-col justify-between bg-white/60 backdrop-blur-2xl saturate-200 border border-white/40 shadow-[0_8px_30px_rgb(0,0,0,0.04)] rounded-[32px] hover:shadow-[0_20px_40px_rgba(0,0,0,0.06)] transition-all duration-500 group">
            <div>
              <div className="mb-6">
                <h3 className="text-xl font-bold font-outfit text-gray-500 uppercase tracking-widest text-xs mb-2">Enterprise</h3>
                <h4 className="text-3xl font-extrabold font-outfit text-gray-900 mb-2">Business</h4>
                <div className="flex items-baseline gap-1">
                  <span className="text-4xl font-extrabold font-outfit text-gray-900">$299</span>
                  <span className="text-sm font-semibold text-gray-500">/ month</span>
                </div>
              </div>
              <div className="h-px w-full bg-gradient-to-r from-transparent via-gray-200 to-transparent mb-8" />
              <ul className="space-y-4 mb-10">
                <PricingFeature text="Unlimited Agents" />
                <PricingFeature text="Unlimited AI actions" />
                <PricingFeature text="500GB Vault Storage" />
                <PricingFeature text="Custom Handoff Logic" />
                <PricingFeature text="White-glove Onboarding" />
              </ul>
            </div>
            <button
              onClick={() => handleUpgrade('Business')}
              className="w-full h-[52px] bg-gray-800 text-white rounded-2xl font-bold text-sm hover:bg-gray-900 transition-all active:scale-[0.98] shadow-[0_10px_20px_-5px_rgba(0,0,0,0.15)] flex items-center justify-center group-hover:translate-y-[-2px]"
            >
              Contact Sales
            </button>
          </div>
        </div>

        <div className="flex flex-col md:flex-row items-center justify-center gap-6 mt-4">
            <div className="flex items-center gap-2 px-4 py-2 bg-green-50 rounded-full border border-green-100">
              <svg className="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clipRule="evenodd" />
              </svg>
              <span className="text-sm font-bold text-green-800">100% money back guarantee</span>
            </div>
            <div className="flex items-center gap-3">
              <span className="text-xs font-semibold text-gray-500 uppercase tracking-widest">Payments Secured By</span>
              <div className="px-3 py-1 bg-white rounded-lg border border-gray-100 shadow-sm font-bold text-[#635BFF] text-sm italic">Stripe</div>
            </div>
        </div>

        <section className="mt-12 bg-white/40 backdrop-blur-3xl saturate-[1.5] border border-white/50 shadow-2xl rounded-[40px] overflow-hidden">
            <div className="p-8 md:p-12">
              <h2 className="text-2xl font-extrabold font-outfit mb-8 text-gray-900">Frequently Asked Questions</h2>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-8">
                  <div className="group">
                      <h3 className="font-bold text-gray-900 flex items-center gap-3 transition-colors group-hover:text-indigo-600">
                        <span className="w-6 h-6 rounded-full bg-indigo-50 flex items-center justify-center text-xs text-indigo-600">Q</span>
                        How do I manage my subscription?
                      </h3>
                      <p className="text-gray-600 text-sm mt-3 leading-relaxed pl-9">
                        One Human Corp provides a self-serve billing portal. You can upgrade, downgrade, or cancel your plan at any time directly from your <span className="font-bold underline cursor-pointer" onClick={() => router.push('/plan')}>Account Plan</span> settings.
                      </p>
                  </div>
                  <div className="group">
                      <h3 className="font-bold text-gray-900 flex items-center gap-3 transition-colors group-hover:text-indigo-600">
                        <span className="w-6 h-6 rounded-full bg-indigo-50 flex items-center justify-center text-xs text-indigo-600">Q</span>
                        What counts as an AI action?
                      </h3>
                      <p className="text-gray-600 text-sm mt-3 leading-relaxed pl-9">
                        An action is any primary task performed by an agent, such as drafting a complex reply, generating an invoice, or analyzing a sales trend. Basic navigation and search are always unlimited.
                      </p>
                  </div>
                  <div className="group">
                      <h3 className="font-bold text-gray-900 flex items-center gap-3 transition-colors group-hover:text-indigo-600">
                        <span className="w-6 h-6 rounded-full bg-indigo-50 flex items-center justify-center text-xs text-indigo-600">Q</span>
                        Is there a storage limit for files?
                      </h3>
                      <p className="text-gray-600 text-sm mt-3 leading-relaxed pl-9">
                        Yes, storage varies by tier. We automatically optimize your business photos to WebP format to help you stay within your limits and keep your storefront fast.
                      </p>
                  </div>
                  <div className="group">
                      <h3 className="font-bold text-gray-900 flex items-center gap-3 transition-colors group-hover:text-indigo-600">
                        <span className="w-6 h-6 rounded-full bg-indigo-50 flex items-center justify-center text-xs text-indigo-600">Q</span>
                        Can I change my plan later?
                      </h3>
                      <p className="text-gray-600 text-sm mt-3 leading-relaxed pl-9">
                        Absolutely. When you upgrade, the new features are available immediately. Downgrades take effect at the end of your current billing cycle.
                      </p>
                  </div>
              </div>
            </div>
            <div className="bg-gradient-to-r from-indigo-500 to-purple-600 p-8 md:p-12 text-white flex flex-col md:flex-row items-center justify-between gap-8">
              <div>
                <h3 className="text-2xl font-bold font-outfit mb-2">Still have questions?</h3>
                <p className="text-indigo-100 font-medium">Our support team is here to help you get started.</p>
              </div>
              <button className="h-[52px] px-8 bg-white text-indigo-600 rounded-2xl font-bold text-sm hover:bg-indigo-50 transition-all active:scale-95 shadow-xl">
                Contact Support
              </button>
            </div>
        </section>

        <footer className="mt-12 py-10 border-t border-gray-200 flex flex-col items-center gap-6">
          <PoweredByOHC tenantId="ohc" />
          <p className="text-xs font-semibold text-gray-400 uppercase tracking-widest text-center">
            &copy; {new Date().getFullYear()} One Human Corp. All rights reserved.
          </p>
        </footer>
      </main>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800&family=Outfit:wght@500;600;700;800;900&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}

function PricingFeature({ text, active = false }: { text: string; active?: boolean }) {
  return (
    <li className="flex items-center gap-3 group/feat">
      <div className={`flex-shrink-0 w-6 h-6 rounded-full flex items-center justify-center ${active ? 'bg-indigo-100 text-indigo-600' : 'bg-gray-100 text-gray-400'}`}>
        <svg className="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
        </svg>
      </div>
      <span className={`text-sm font-semibold tracking-tight transition-colors ${active ? 'text-gray-800' : 'text-gray-500'}`}>
        {text}
      </span>
    </li>
  );
}
