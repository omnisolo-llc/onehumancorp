import React from 'react';

interface Step1Props {
  businessType: string;
  setBusinessType: (v: string) => void;
  handleNext: () => void;
}

export default function Step1({ businessType, setBusinessType, handleNext }: Step1Props) {
  return (
    <div className="flex flex-col flex-1 justify-center animate-fade-in">
      <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">What do you do?</h2>
      <p className="text-gray-500 text-sm mb-6">Tell us what you sell or the services you provide.</p>
      <input
        type="text"
        value={businessType}
        onChange={(e) => setBusinessType(e.target.value)}
        onKeyDown={(e) => { if (e.key === 'Enter') handleNext(); }}
        placeholder="e.g. Sell cakes, plumbing"
        className="w-full p-4 rounded-[12px] border border-white/50 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none transition-all text-lg mb-4 bg-white/40 backdrop-blur-md shadow-sm"
        autoFocus
        enterKeyHint="next"
        autoComplete="off"
        autoCapitalize="sentences"
      />
      <button
        onClick={handleNext}
        className="w-full bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[12px] font-bold shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all"
      >
        Next
      </button>
    </div>
  );
}
