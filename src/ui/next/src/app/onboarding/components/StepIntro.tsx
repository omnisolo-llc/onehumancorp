import React from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './IconLabel';

export function StepIntro({ syncStateToBackend }: { syncStateToBackend: (state: any) => void }) {
  const { updateState, error, bio } = useOnboardingStore();

  return (
    <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
      <div className="w-16 h-16 bg-[#eef2ff] dark:bg-[#0066FF]/20 rounded-full flex items-center justify-center mb-6">
        <svg className="w-8 h-8 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
      </div>
      <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center">10-Minute Setup Wizard</h2>
      <p className="text-gray-500 dark:text-[#A1A1A6] text-sm text-center mb-8 leading-relaxed max-w-sm">
        Zero tech skills needed. We do the heavy lifting. Review and add any extra details to help our AI generate the perfect store.
      </p>

      <div className="flex flex-col gap-4 w-full">
        <button
          className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] rounded-[8px]"
          onClick={() => { updateState({ step: 1, chatStep: 1 }); syncStateToBackend({ step: 1, chatStep: 1 }); }}
        >
          Start My Business
        </button>
        <button
          type="button"
          className="w-full bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-semibold hover:border-gray-400 dark:hover:border-gray-500 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
          onClick={() => { updateState({ bio: "" }); updateState({ step: -1 }); syncStateToBackend({ step: -1, bio: "" }); }}
        >
          Instant Build
        </button>
        <button
          type="button"
          className="w-full bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-semibold hover:border-gray-400 dark:hover:border-gray-500 transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
          onClick={() => { updateState({ step: 0 }); syncStateToBackend({ step: 0 }); }}
        >
          Conversational Setup
        </button>
      </div>
    </div>
  );
}
