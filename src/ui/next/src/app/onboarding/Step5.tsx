import React from 'react';

interface StartResult {
  message?: string;
}

interface Step5Props {
  startResult: StartResult;
}

export default function Step5({ startResult }: Step5Props) {
  if (!startResult) return null;
  return (
    <div className="flex flex-col flex-1 justify-center items-center text-center animate-fade-in">
      <div className="w-20 h-20 bg-green-50 rounded-full flex items-center justify-center mb-6">
        <svg className="w-10 h-10 text-[#34C759]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
        </svg>
      </div>
      <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">You're Live!</h2>
      <p className="text-gray-500 text-sm mb-8 px-4">
        {startResult.message || "Your business has been successfully launched."}
      </p>

      <div className="w-full space-y-3 mt-auto">
        <a
          href="/dashboard"
          className="block w-full bg-[#1D1D1F] text-white p-4 rounded-[8px] font-bold shadow-md hover:bg-black active:scale-[0.98] transition-all"
        >
          Go to Dashboard
        </a>
        <a
          href="/builder"
          className="block w-full bg-white/70 backdrop-blur-md text-[#1D1D1F] border border-white/50 p-4 rounded-[8px] font-bold shadow-sm hover:bg-white/90 active:scale-[0.98] transition-all"
        >
          Preview Storefront
        </a>
      </div>
    </div>
  );
}
