import React from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './IconLabel';

function generateSubdomain(name: string): string {
  if (!name || name.trim() === '') return 'my-business.ohc.app';
  const cleanName = name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
  return cleanName ? `${cleanName}.ohc.app` : 'my-business.ohc.app';
}

export function StepSuccess() {
  const { businessName, startResult } = useOnboardingStore();

  return (
    <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in w-full">
      <div className="w-20 h-20 bg-[#34C759]/20 rounded-full flex items-center justify-center mb-6">
        <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
        </svg>
      </div>
      <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">You're Live!</h2>
      <p className="text-gray-500 dark:text-[#A1A1A6] text-sm mb-8 px-4">
        {startResult?.message || "Your business has been successfully launched."}
      </p>

      <div className="w-full space-y-3 mt-auto">
        <div className="p-3 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] flex flex-col items-center mb-6">
           <p className="text-xs text-gray-500 dark:text-[#A1A1A6] uppercase font-bold tracking-wider mb-2">Your Shareable Link</p>
           <div className="flex items-center gap-2">
              <span className="text-[#0066FF] font-semibold">{generateSubdomain(businessName)}</span>
           </div>
        </div>

        <a
          href="/assistant"
          className="flex w-full items-center justify-center bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-bold hover:border-gray-400 dark:hover:border-gray-500 active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
        >
          <IconLabel icon="sparkles">Open Assistant</IconLabel>
        </a>
        <a
          href="/builder"
          className="flex w-full items-center justify-center bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] p-4 font-bold active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
        >
          <IconLabel icon="eye">Preview Storefront</IconLabel>
        </a>
      </div>
    </div>
  );
}
