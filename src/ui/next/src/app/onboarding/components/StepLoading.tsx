import React from 'react';

export function StepLoading() {
  return (
    <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[16px] p-4 sm:p-8">
      <div className="w-24 h-24 relative mb-8">
        <div className="absolute inset-0 border-4 border-[#0066FF]/20 rounded-full"></div>
        <div className="absolute inset-0 border-4 border-[#0066FF] rounded-full border-t-transparent animate-spin"></div>
      </div>
      <h2 className="text-2xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4">Building Your Business...</h2>
      <div className="space-y-2">
        <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse">Generating your product catalog</p>
        <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '0.5s' }}>Configuring payment settings</p>
        <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1s' }}>Designing your storefront</p>
        <p className="text-gray-500 dark:text-[#A1A1A6] text-sm animate-pulse" style={{ animationDelay: '1.5s' }}>Onboarding your AI agents</p>
      </div>
    </div>
  );
}
