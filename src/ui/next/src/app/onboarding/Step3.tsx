import React from 'react';

export default function Step3({ businessCategory, setBusinessCategory, handleIntakeSubmit, setStep, isLoading }: any) {
  return (
    <div className="flex flex-col flex-1 justify-center animate-fade-in">
      <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2">What's your niche?</h2>
      <p className="text-gray-500 text-sm mb-6">Products, services, or bookings.</p>
      <input
        type="text"
        value={businessCategory}
        onChange={(e) => setBusinessCategory(e.target.value)}
        onKeyDown={(e) => { if (e.key === 'Enter') handleIntakeSubmit(); }}
        placeholder="e.g. I bake custom wedding cakes"
        className="w-full p-4 rounded-[12px] border border-white/50 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none transition-all text-lg mb-4 bg-white/40 backdrop-blur-md shadow-sm"
        autoFocus
        enterKeyHint="next"
        autoComplete="off"
        autoCapitalize="sentences"
      />
      <div className="flex gap-3">
        <button
          onClick={() => setStep(2)}
          className="px-6 py-4 rounded-[12px] font-bold bg-white/50 backdrop-blur-sm text-gray-700 hover:bg-white/70 shadow-sm border border-white/40 transition-all"
        >
          Back
        </button>
        <button
          onClick={handleIntakeSubmit}
          disabled={isLoading}
          className="flex-1 bg-gradient-to-r from-[#0066FF] to-[#0052cc] text-white p-4 rounded-[12px] font-bold shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all disabled:opacity-70 flex justify-center items-center"
        >
          {isLoading ? (
            <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
          ) : (
            "Generate Draft"
          )}
        </button>
      </div>
    </div>
  );
}
