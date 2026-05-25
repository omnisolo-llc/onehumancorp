import React from 'react';

export default function Step2({ businessName, setBusinessName, handleNext, setStep }: any) {
  return (
    <div className="flex flex-col flex-1 justify-center animate-fade-in">
      <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">What's the name of your business?</h2>
      <p className="text-gray-500 text-sm mb-6">Don't worry, you can change this later.</p>
      <input
        type="text"
        value={businessName}
        onChange={(e) => setBusinessName(e.target.value)}
        onKeyDown={(e) => { if (e.key === 'Enter') handleNext(); }}
        placeholder="e.g. Maya's Cakes"
        className="w-full p-4 rounded-[12px] border border-white/50 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none transition-all text-lg mb-4 bg-white/40 backdrop-blur-md shadow-sm"
        autoFocus
        enterKeyHint="next"
        autoComplete="off"
        autoCapitalize="words"
      />
      <div className="flex gap-3">
        <button
          onClick={() => setStep(1)}
          className="px-6 py-4 rounded-[12px] font-bold bg-white/50 backdrop-blur-sm text-gray-700 hover:bg-white/70 shadow-sm border border-white/40 transition-all"
        >
          Back
        </button>
        <button
          onClick={handleNext}
          className="flex-1 bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[12px] font-bold shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all"
        >
          Next
        </button>
      </div>
    </div>
  );
}
