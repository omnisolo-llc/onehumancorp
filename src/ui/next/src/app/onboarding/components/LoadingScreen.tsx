import React, { useEffect, useState } from 'react';

export function LoadingScreen() {
  const [loadingStatusIndex, setLoadingStatusIndex] = useState(0);

  const loadingStatuses = [
    "Analyzing your business niche...",
    "Drafting your custom product catalog...",
    "Designing a premium storefront...",
    "Onboarding your AI Sales Agent...",
    "Configuring secure payment processing...",
    "Drafting Legal & Privacy policies...",
    "Optimizing SEO for Google search...",
    "Preparing your Founder Dashboard...",
    "Securing your unique web address...",
    "Almost ready! Finalizing AI personality..."
  ];

  useEffect(() => {
    const interval = setInterval(() => {
      setLoadingStatusIndex(prev => (prev + 1) % loadingStatuses.length);
    }, 2500);
    return () => clearInterval(interval);
  }, [loadingStatuses.length]);

  return (
    <div aria-live="polite" className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in bg-white/5 dark:bg-black/5 backdrop-blur-xl rounded-[24px] border border-white/20 p-10 shadow-2xl overflow-hidden">
      <div className="w-28 h-28 relative mb-10">
        <div className="absolute inset-0 border-[6px] border-[#0066FF]/10 rounded-full"></div>
        <div className="absolute inset-0 border-[6px] border-[#0066FF] rounded-full border-t-transparent animate-spin shadow-[0_0_15px_rgba(0,102,255,0.4)]"></div>
        <div className="absolute inset-4 bg-[#0066FF]/5 rounded-full flex items-center justify-center">
           <svg className="w-10 h-10 text-[#0066FF] animate-pulse" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
        </div>
      </div>
      <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-6 tracking-tight">Building Your Future...</h2>
      <div className="space-y-4 w-full max-w-sm">
        <div className="flex items-center gap-3 animate-fade-in bg-white/10 dark:bg-black/20 p-3 rounded-[12px] border border-white/10">
           <div className="w-2 h-2 rounded-full bg-[#0066FF] animate-ping"></div>
           <p className="text-[#1D1D1F] dark:text-[#F5F5F7] text-sm font-bold tracking-tight">{loadingStatuses[loadingStatusIndex]}</p>
        </div>

        <div className="grid grid-cols-2 gap-2 opacity-40">
           <div className="flex items-center gap-2">
              <div className="w-1.5 h-1.5 rounded-full bg-[#34C759]"></div>
              <span className="text-[10px] font-black uppercase tracking-wider text-gray-500">Cloud Ready</span>
           </div>
           <div className="flex items-center gap-2">
              <div className="w-1.5 h-1.5 rounded-full bg-[#34C759]"></div>
              <span className="text-[10px] font-black uppercase tracking-wider text-gray-500">DB Synced</span>
           </div>
        </div>
      </div>
    </div>
  );
}
