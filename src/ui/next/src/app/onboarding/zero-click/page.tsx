"use client";

import React, { useState, useEffect, useRef } from 'react';
import { useProPlan } from '../../components/useProPlan';
import { useRouter } from 'next/navigation';
import { PoweredByOmniSolo } from '../../components/PoweredByOmniSolo';
import { readLaunchResult, readPreparation, resultForPreparation, type PreparedResult } from '../contracts';
import { OnboardingChatAgent } from './components/OnboardingChatAgent';

export default function ZeroClickBuilderPage() {
  const router = useRouter();
  const [generatedStore, setGeneratedStore] = useState<PreparedResult | null>(null);
  const { hasPro } = useProPlan();

  const [pending, setPending] = useState(false);
  const [error, setError] = useState('');
  const busy = useRef(false);
  const epoch = useRef(0);
  const unknownLaunch = useRef(false);
  useEffect(() => () => { epoch.current += 1; }, []);
  const handleLaunch = async () => {
    if (!generatedStore || busy.current) return;
    busy.current = true; setPending(true); setError('');
    const version = ++epoch.current;
    try {
      if (unknownLaunch.current) {
        const response = await fetch('/api/v1/onboarding/state');
        if (!response.ok) throw new Error('Could not check setup status. Reload before retrying.');
        const state = await response.json();
        if (version !== epoch.current) return;
        const receipt = readPreparation(state.preparation);
        if (receipt.preparation_id !== generatedStore.preparation_id || receipt.organization_id !== generatedStore.organization_id || receipt.user_id !== generatedStore.user_id) throw new Error('Setup changed. Reload to review it.');
        if (receipt.status === 'launched') { setGeneratedStore(resultForPreparation(receipt)); unknownLaunch.current = false; return; }
        unknownLaunch.current = false;
      }
      const response = await fetch('/api/v1/onboarding/launch', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ preparation_id: generatedStore.preparation_id }) });
      if (!response.ok) throw new Error('Setup completion could not be confirmed');
      const receipt = readLaunchResult(await response.json(), generatedStore.preparation);
      if (version !== epoch.current) return;
      setGeneratedStore(resultForPreparation(receipt)); localStorage.setItem('has_onboarded', 'true');
    } catch (cause) {
      if (version === epoch.current) { unknownLaunch.current = true; setError(cause instanceof Error ? cause.message : 'Setup completion could not be confirmed'); }
    } finally {
      if (version === epoch.current) { busy.current = false; setPending(false); }
    }
  };

  const handleShare = () => {
    const shareText = `I prepared my business workspace using OmniSolo OneHumanCorp! Start your own for free: https://cloud.omnisolo.co/zero-click-builder?ref=new_store \n\n⚡ Powered by OmniSolo`;
    const shareUrl = `https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText)}`;
    window.open(shareUrl, '_blank');
  };

  const handleChatComplete = (data: import("@/lib/builder-types").OnboardingResult) => {
    setGeneratedStore(resultForPreparation(readPreparation(data.preparation)));
  };

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#16161a] flex flex-col items-center py-12 px-4 sm:px-6 lg:px-8 font-outfit selection:bg-indigo-100 selection:text-indigo-900">
      <div className="w-full max-w-2xl">
        <div className="text-center mb-10">
          <div className="inline-flex items-center justify-center p-3 bg-indigo-100 dark:bg-indigo-900/30 rounded-2xl mb-4">
            <span className="text-3xl">✨</span>
          </div>
          <h1 className="text-4xl font-bold text-[#1D1D1F] dark:text-white tracking-tight mb-3">
            Tell us about your business
          </h1>
          <p className="text-lg text-[#424245] dark:text-[#A1A1A6] max-w-xl mx-auto">
            Prepare a business workspace and product catalog, then review before completing setup.
          </p>
        </div>

        {!generatedStore ? (
          <OnboardingChatAgent onComplete={handleChatComplete} />
        ) : (
          <div className="glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] p-8 mb-8 animate-in fade-in slide-in-from-bottom-4 duration-500">
            <div className="text-center mb-8">
              <div className="inline-flex items-center justify-center w-16 h-16 bg-green-100 dark:bg-green-900/30 text-green-600 rounded-full mb-4">
                <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7"></path>
                </svg>
              </div>
              <h2 className="text-3xl font-bold text-[#1D1D1F] dark:text-white mb-2">
                {generatedStore.status === 'launched' ? 'Setup complete' : 'Your workspace is prepared'}
              </h2>
              <p className="text-[#424245] dark:text-[#A1A1A6]">
                Review your storefront and integrations before sharing or accepting orders.
              </p>
            </div>

            <div className="space-y-6">
              <section aria-label="Saved catalog">
                <h3>Saved catalog</h3>
                <ul>{generatedStore.preparation.catalog.map(product => <li key={product.product_id}>{product.name}: {product.price}</li>)}</ul>
                {generatedStore.status === 'prepared' && <a href="/onboarding">Review or edit setup</a>}
              </section>
              <div className="w-full h-[500px] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] overflow-hidden relative bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px]">
                <iframe
                  src={`/builder?tenant=${encodeURIComponent(generatedStore.organization_id)}&preview=true`}
                  className="w-full h-full border-none"
                  title="Storefront Preview"
                />
              </div>

              <div className="flex flex-col gap-4 pt-4">
                {error && <p role="alert">{error}</p>}
                <button
                  disabled={pending}
                  onClick={generatedStore.status === 'launched' ? () => router.push('/dashboard') : handleLaunch}
                  className="w-full flex items-center justify-center gap-2 bg-[#0066FF] hover:bg-[#005bb5] text-white min-h-[44px] px-6 py-3 rounded-[8px] font-bold text-lg transition-all active:scale-[0.98] shadow-sm hover:shadow-md"
                >
                  {pending ? 'Completing setup...' : generatedStore.status === 'launched' ? 'Go to dashboard' : '🚀 Launch My Store'}
                </button>

                <button
                  onClick={handleShare}
                  className="w-full flex items-center justify-center gap-2 bg-[#1DA1F2] hover:bg-[#1a91da] text-white min-h-[44px] px-6 py-3 rounded-[8px] font-bold text-lg transition-all active:scale-[0.98] shadow-sm hover:shadow-md"
                >
                  🐦 Share on X (Twitter)
                </button>
              </div>
            </div>
          </div>
        )}

        <div className="text-center mt-8">
          <p className="text-sm font-semibold text-gray-500 flex items-center justify-center gap-1">
            <span id="dashboard-footer-viral-link">⚡ Powered by OmniSolo</span>
            {!hasPro && (
              <a href="/pricing" className="text-indigo-500 hover:text-indigo-600 hover:underline ml-1">
                (Upgrade to remove)
              </a>
            )}
          </p>
          <div className="flex justify-center mt-2">
            <PoweredByOmniSolo tenantId="omnisolo" />
          </div>
        </div>
      </div>
    </div>
  );
}
