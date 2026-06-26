import React from 'react';
import { useOnboardingStore } from '../store';

export function StepInstantBuild({
  handleInstantBuild,
  syncStateToBackend
}: {
  handleInstantBuild: () => void;
  syncStateToBackend: (state: any) => void;
}) {
  const { updateState, error, bio, instantImageUrl, isLoading } = useOnboardingStore();

  return (
    <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
      <button onClick={() => { updateState({ step: -2 }); syncStateToBackend({ step: -2 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
      </button>
      <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Tell us about your business</h2>
      <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-8 leading-relaxed max-w-sm">
        Our AI will handle the rest in 30 seconds.
      </p>

      <div className="flex flex-col gap-4 w-full">
        <textarea
          id="instant-bio"
          data-testid="instant-bio"
          className={`bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] w-full p-4 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms] ${error === "Please tell us about your business." || error ? "border-2 border-[#FF3B30]" : "border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]"}`}
          placeholder="e.g. I run a local bakery that sells custom vegan cakes..."
          rows={6}
          style={{ resize: 'none' }}
          value={bio}
          onChange={(e) => {
            updateState({ bio: e.target.value });
            if (error) updateState({ error: '' });
          }}
        />

        <input
          id="instant-image-url"
          data-testid="instant-image-url"
          type="url"
          className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] rounded-[8px] min-h-[44px] w-full p-3 text-[#1D1D1F] dark:text-[#F5F5F7] outline-none transition-all duration-[250ms]"
          placeholder="Image URL (Optional)"
          value={instantImageUrl}
          onChange={(e) => updateState({ instantImageUrl: e.target.value })}
          inputMode="url"
          autoComplete="url"
        />

        <div className="mt-4">
          <button
            onClick={handleInstantBuild}
            disabled={!bio.trim() || isLoading}
            id="generate-storefront-btn"
            className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px] active:scale-[0.98]"
          >
            Next
          </button>
        </div>
      </div>
    </div>
  );
}
