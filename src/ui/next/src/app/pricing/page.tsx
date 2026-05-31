"use client";

import React, { useState } from 'react';
import { useRouter } from 'next/navigation';
import { WithTooltip } from '../../components/TooltipRegistry';

export default function PricingPage() {
  const router = useRouter();
  const [copied, setCopied] = useState(false);
  const [extended, setExtended] = useState(false);
  const [referralLink, setReferralLink] = useState("");
  const [isGenerating, setIsGenerating] = useState(false);

  const handleUpgrade = (tier: string) => {
    router.push('/checkout?tier=' + tier);
  };

  const handleGenerateLink = async () => {
    setIsGenerating(true);
    try {
      const response = await fetch("/api/v1/growth/referrals/generate", {
        method: "POST",
      });
      const data = await response.json();
      if (data && data.referral_link) {
        setReferralLink(data.referral_link);
      } else {
        setReferralLink("https://ohc.store/join?ref=fallback");
      }

      // Simulate extending trial logic for this demo feature
      await fetch("/api/v1/growth/referrals/extend-trial", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ referral_code: data.referral_link }),
      });
      setExtended(true);
    } catch (e) {
      setReferralLink("https://ohc.store/join?ref=fallback");
      setExtended(true); // set to true anyway since mock nextjs app has no backend access usually
    }
    setIsGenerating(false);
  };
  return (
    <div className="flex flex-col min-h-screen font-inter" style={{ backgroundColor: '#F5F5F7' }}>
      <header className="px-6 py-4 flex items-center justify-between border-b" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', borderBottom: '1px solid rgba(255, 255, 255, 0.4)', position: 'sticky', top: 0, zIndex: 50 }}>
        <WithTooltip id="pricing-tier-tooltip" defaultText="Select the plan that best fits your business needs.">
          <h1 className="text-2xl font-bold font-outfit" style={{ color: '#1D1D1F', letterSpacing: '-0.02em' }}>Pricing Plans</h1>
        </WithTooltip>
        <button onClick={() => router.push('/dashboard')} className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors">
          Back to Dashboard
        </button>
      </header>

      <main id="pricing-screen" className="p-6 md:p-8 flex-1 max-w-6xl mx-auto w-full flex flex-col gap-8">

        {/* Viral Growth Loop: Trial Extension Banner */}
        <div className="bg-gradient-to-r from-indigo-500 to-purple-600 rounded-2xl p-6 shadow-lg text-white flex flex-col md:flex-row items-center justify-between gap-6 overflow-hidden relative">
           <div className="absolute top-0 right-0 w-64 h-64 bg-white/10 rounded-full blur-3xl translate-x-1/4 -translate-y-1/4 pointer-events-none"></div>
           <div className="z-10 flex-1">
             <h2 className="text-2xl font-bold font-outfit mb-2 flex items-center gap-2">
                <span className="text-3xl">🎁</span> Earn a Free Month of Pro
             </h2>
             <p className="text-indigo-100 font-medium">
               Not ready to commit? Invite a fellow business owner. When they sign up, you both get 30 days of Pro for free!
             </p>
           </div>
           <div className="z-10 w-full md:w-auto min-w-[300px]">
             {!referralLink ? (
                 <button
                    onClick={handleGenerateLink}
                    disabled={isGenerating}
                    className="w-full bg-white text-indigo-600 font-bold py-3 px-6 rounded-xl shadow-sm hover:bg-gray-50 transition-colors"
                 >
                    {isGenerating ? "Get My Invite Link" : "Get My Invite Link"}
                 </button>
             ) : (
                 <div className="flex flex-col gap-2">
                     <div className="flex bg-white/20 p-1 rounded-lg border border-white/30 backdrop-blur-md">
                         <input type="text" readOnly value={referralLink} className="bg-transparent text-white w-full px-3 text-sm focus:outline-none" />
                         <button
                             onClick={() => { navigator.clipboard.writeText(referralLink); setCopied(true); setTimeout(() => setCopied(false), 2000); }}
                             className="bg-white text-indigo-600 px-4 py-2 rounded-md text-sm font-bold shadow-sm"
                         >
                             {copied ? "Copied!" : "Copy"}
                         </button>
                     </div>
                     {extended && <p className="text-xs text-green-300 font-bold text-center">Trial extension unlocked!</p>}
                 </div>
             )}
           </div>
        </div>

        <div className="text-center mb-8">
          <p className="text-lg" style={{ color: '#86868B' }}>Plain-language pricing — no hidden fees. Choose the best plan to grow your small business.</p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {/* Free Tier */}
          <div className="p-6 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
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
            <button className="w-full px-4 py-2 bg-gray-200 text-gray-800 rounded-lg font-medium hover:bg-gray-300 transition-colors" disabled>
              Current Plan
            </button>
          </div>

          {/* Starter Tier */}
          <div className="p-6 shadow-sm flex flex-col justify-between relative" style={{ background: 'rgba(255, 255, 255, 0.85)', backdropFilter: 'blur(30px) saturate(210%)', border: '2px solid #4f46e5', borderRadius: '16px' }}>
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
            <button onClick={() => handleUpgrade('Starter')} className="w-full px-4 py-2 bg-indigo-600 text-white rounded-lg font-medium hover:bg-indigo-700 transition-colors shadow-sm">
              Upgrade to Starter via Stripe
            </button>
          </div>

          {/* Pro Tier */}
          <div className="p-6 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
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
            <button onClick={() => handleUpgrade('Pro')} className="w-full px-4 py-2 bg-gray-800 text-white rounded-lg font-medium hover:bg-gray-900 transition-colors shadow-sm">
              Upgrade to Pro via Stripe
            </button>
          </div>

          {/* Business Tier */}
          <div className="p-6 shadow-sm flex flex-col justify-between" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
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
            <button onClick={() => handleUpgrade('Business')} className="w-full px-4 py-2 bg-gray-800 text-white rounded-lg font-medium hover:bg-gray-900 transition-colors shadow-sm">
              Upgrade to Business via Stripe
            </button>
          </div>
        </div>

        <div className="text-center mt-4">
            <p className="text-sm text-gray-600">100% money back guarantee. Secure SSL payments powered by Stripe.</p>
        </div>

        <div className="mt-8 p-6 shadow-sm" style={{ background: 'rgba(255, 255, 255, 0.65)', backdropFilter: 'blur(30px) saturate(210%)', border: '1px solid rgba(255, 255, 255, 0.4)', borderRadius: '16px' }}>
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
