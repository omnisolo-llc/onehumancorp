import React from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './Icons';

export function SuccessStage() {
  const { startResult } = useOnboardingStore();

  if (!startResult) return null;

  return (
    <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in overflow-hidden">
      <div className="w-24 h-24 bg-[#34C759]/15 rounded-full flex items-center justify-center mb-8 shadow-inner border border-[#34C759]/20">
        <svg className="w-12 h-12 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
        </svg>
      </div>
      <h2 className="text-4xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-3 tracking-tight">You're Live!</h2>
      <p className="text-gray-500 dark:text-[#A1A1A6] text-lg mb-10 px-6 font-medium leading-relaxed">
        {startResult.message || "Your business has been successfully launched."}
      </p>

      <div className="w-full space-y-4 mt-auto">
        <div className="p-4 mac-glass-container rounded-[20px] flex flex-col items-center border border-white/20 shadow-sm">
           <p className="text-[10px] text-gray-400 dark:text-gray-500 uppercase font-black tracking-[0.2em] mb-2">Public Storefront</p>
           <div className="flex items-center gap-2">
              <span className="text-[#0066FF] font-black text-lg">my-business.ohc.store</span>
              <svg className="w-4 h-4 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" /></svg>
           </div>
        </div>

        <div className="grid grid-cols-1 gap-3">
          <a
            href="/dashboard"
            className="flex w-full items-center justify-center bg-[#1D1D1F] dark:bg-white text-white dark:text-[#1D1D1F] h-[58px] rounded-[16px] font-bold text-lg shadow-xl hover:scale-[1.02] active:scale-[0.98] transition-all"
          >
            <IconLabel icon="dashboard">Enter Dashboard</IconLabel>
          </a>
          <a
            href="/builder"
            className="flex w-full items-center justify-center mac-glass-container text-[#1D1D1F] dark:text-[#F5F5F7] h-[58px] rounded-[16px] font-bold text-lg border border-white/20 hover:bg-white/40 dark:hover:bg-white/5 active:scale-[0.98] transition-all"
          >
            <IconLabel icon="eye">Preview Storefront</IconLabel>
          </a>
        </div>
      </div>
    </div>
  );
}
