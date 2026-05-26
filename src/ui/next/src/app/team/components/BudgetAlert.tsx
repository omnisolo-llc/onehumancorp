import React from 'react';

type Props = {
  budget: number;
};

export default function BudgetAlert({ budget }: Props) {
  // Assuming a default maximum of 100 for the AI limit, budget 10 represents 10% left, meaning we've consumed 90%
  const isAlertThreshold = budget <= 10;

  if (!isAlertThreshold) return null;

  return (
    <div className="fixed bottom-6 left-1/2 transform -translate-x-1/2 z-50 animate-in fade-in slide-in-from-bottom-5 duration-300">
      <div className="bg-white/70 backdrop-blur-[30px] saturate-[210%] px-5 py-3.5 rounded-2xl shadow-xl shadow-orange-500/10 border border-orange-100 flex items-center gap-4 text-sm font-outfit max-w-[340px] w-full">
        <div className="flex-shrink-0 w-8 h-8 bg-orange-100 rounded-full flex items-center justify-center text-orange-500">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
        </div>
        <div className="flex-1">
          <div className="text-gray-900 font-semibold text-[13px] leading-tight mb-0.5">Your agents have been busy!</div>
          <div className="text-orange-600 font-medium text-[11px] leading-tight opacity-90">You are at 90% of your AI budget.</div>
        </div>
        <button className="flex-shrink-0 bg-gray-900 text-white hover:bg-gray-800 transition-colors px-3 py-1.5 rounded-lg text-xs font-semibold shadow-sm active:scale-95">
          Upgrade
        </button>
      </div>
    </div>
  );
}
