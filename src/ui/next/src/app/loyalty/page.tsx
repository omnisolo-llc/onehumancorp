"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function LoyaltyPage() {
  const router = useRouter();
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);

  const [pointRatio, setPointRatio] = useState(1);
  const [isSaving, setIsSaving] = useState(false);
  const [isLoaded, setIsLoaded] = useState(false);

  useEffect(() => {
    const fetchSettings = async () => {
        try {
            const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
            const res = await fetch('/api/v1/growth/loyalty/settings', {
              headers: { 'x-tenant-id': tenantId }
            });
            if (res.ok) {
                const data = await res.json();
                setPointRatio(data.point_ratio);
            }
            const proRes = await fetch('/api/v1/billing/entitlements', {
              headers: { 'x-tenant-id': tenantId }
            });
            if (proRes.ok) {
                const proData = await proRes.json();
                setHasPro(proData.tier === 'Pro' || proData.tier === 'Enterprise');
            }
        } catch (e) {
            console.error('Failed to load loyalty settings or entitlements', e);
        } finally {
            setIsLoaded(true);
        }
    };

    fetchSettings();
  }, []);

  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const handleSave = async () => {
    if (!hasPro) {
      setShowSoftPaywall(true);
      return;
    }

    setIsSaving(true);
    try {
        const tenantId = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
        const res = await fetch('/api/v1/growth/loyalty/settings', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'x-tenant-id': tenantId
            },
            body: JSON.stringify({ point_ratio: pointRatio })
        });

        if (res.ok) {
            setToastMessage('Loyalty settings saved! AI agents will now automatically apply these rules.');
            setTimeout(() => setToastMessage(null), 3000);
        } else {
            setToastMessage('Failed to save loyalty settings.');
            setTimeout(() => setToastMessage(null), 3000);
        }
    } catch (e) {
        console.error('Error saving settings', e);
        setToastMessage('Failed to save loyalty settings.');
        setTimeout(() => setToastMessage(null), 3000);
    } finally {
        setIsSaving(false);
    }
  };

  const claimTrialExtension = () => {
    const tenant = typeof localStorage !== 'undefined' ? localStorage.getItem('tenant_id') || 'DEFAULT' : 'DEFAULT';
    window.open(`https://twitter.com/intent/tweet?text=${encodeURIComponent('I just unlocked powerful AI tools for my business on One Human Corp! Start your own business today: ohc://join?ref=' + tenant)}`, '_blank');
    setHasPro(true);
    setShowSoftPaywall(false);
  };

  return (
    <div className="flex flex-col min-h-screen font-inter bg-[#F5F5F7]">
      {toastMessage && (
        <div className="fixed bottom-4 right-4 bg-gray-900 text-white px-6 py-3 rounded-xl shadow-2xl z-50 animate-fade-in-up font-medium text-sm border border-gray-700">
            {toastMessage}
        </div>
      )}

      {/* Header */}
      <header className="px-6 py-4 flex items-center justify-between border-b sticky top-0 z-50 bg-white/65 backdrop-blur-md border-white/40">
         <div className="flex items-center gap-3">
            <h1 className="text-2xl font-bold font-outfit text-[#1D1D1F] tracking-tight">AI Loyalty Engine 💖</h1>
            {!hasPro && <span className="px-2 py-0.5 bg-yellow-100 text-yellow-700 rounded text-xs font-bold uppercase tracking-wider">Pro Feature</span>}
         </div>
         <button
           onClick={() => router.push('/dashboard')}
           className="px-4 py-2 bg-gray-200 rounded-md text-sm font-medium hover:bg-gray-300 transition-colors"
         >
           Back to Dashboard
         </button>
      </header>

      <main className="p-6 md:p-8 flex-1 w-full max-w-4xl mx-auto flex flex-col gap-6">
        <div className="bg-white/65 backdrop-blur-md border border-white/40 shadow-sm rounded-2xl p-6 md:p-8">
            <h2 className="text-xl font-bold font-outfit text-gray-900 mb-2">Automated Loyalty Rules</h2>
            <p className="text-sm text-gray-600 mb-6">Set up your earning rules. Your AI Customer Success Ambassador will automatically track points in the LoyaltyLedger and proactively notify customers when they have rewards to spend.</p>

            <div className="flex flex-col gap-4 max-w-md">
                <div className="flex flex-col gap-2">
                    <label className="text-sm font-semibold text-gray-700">Points Earned per $1 Spent</label>
                    <div className="flex items-center gap-4">
                        <input
                            type="number"
                            min="1"
                            value={pointRatio}
                            onChange={(e) => setPointRatio(Number(e.target.value))}
                            className="p-3 border rounded-xl bg-white w-24 text-center font-bold"
                        />
                        <span className="text-gray-500 font-medium text-sm">Points</span>
                    </div>
                </div>

                <div className="mt-4">
                     <button
                        onClick={handleSave}
                        disabled={isSaving}
                        className={`w-full py-3 bg-gradient-to-r from-purple-600 to-pink-600 text-white rounded-xl text-sm font-bold shadow-md hover:shadow-lg transition-all ${isSaving ? 'opacity-70' : ''}`}
                     >
                        {isSaving ? 'Saving...' : 'Activate AI Loyalty Engine'}
                     </button>
                </div>
            </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="bg-white/65 backdrop-blur-md border border-white/40 shadow-sm rounded-2xl p-6 relative overflow-hidden">
                <div className="absolute top-0 right-0 p-4 opacity-10 text-6xl">🤖</div>
                <h3 className="font-bold font-outfit text-gray-900 mb-2">Zero-Touch Redemption</h3>
                <p className="text-sm text-gray-600">You don't need to do anything. Customers will automatically see their point balance at checkout and can apply discounts with a single tap.</p>
            </div>

            <div className="bg-white/65 backdrop-blur-md border border-white/40 shadow-sm rounded-2xl p-6 relative overflow-hidden">
                <div className="absolute top-0 right-0 p-4 opacity-10 text-6xl">📊</div>
                <h3 className="font-bold font-outfit text-gray-900 mb-2">Event-Sourced Ledger</h3>
                <p className="text-sm text-gray-600">Every point earned or spent is cryptographically recorded in the tenant-isolated LoyaltyLedger, ensuring perfect auditability.</p>
            </div>
        </div>
      </main>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/40 backdrop-blur-sm z-50 flex items-center justify-center p-4">
            <div className="bg-white rounded-3xl p-8 max-w-md w-full shadow-2xl relative overflow-hidden">
                <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-br from-yellow-300 to-orange-400 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2 opacity-30 pointer-events-none"></div>
                <h3 className="text-2xl font-bold font-outfit text-gray-900 mb-2">Pro Feature 🌟</h3>
                <p className="text-gray-600 mb-6 text-sm">
                    The Autonomous Loyalty Engine is a Pro feature. Automatically turn one-time buyers into lifelong fans without lifting a finger.
                </p>

                <div className="flex flex-col gap-3">
                    <button
                        onClick={claimTrialExtension}
                        className="w-full py-3 bg-black text-white rounded-xl font-bold text-sm shadow-md hover:bg-gray-800 transition-all flex items-center justify-center gap-2"
                    >
                        Share on X to unlock for free
                    </button>
                    <button
                        onClick={() => router.push('/upgrade-roi')}
                        className="w-full py-3 bg-gray-100 text-gray-800 rounded-xl font-bold text-sm hover:bg-gray-200 transition-all"
                    >
                        View Upgrade Plans
                    </button>
                    <button
                        onClick={() => setShowSoftPaywall(false)}
                        className="w-full py-2 text-gray-500 text-xs font-medium hover:text-gray-700"
                    >
                        Cancel
                    </button>
                </div>
            </div>
        </div>
      )}

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
