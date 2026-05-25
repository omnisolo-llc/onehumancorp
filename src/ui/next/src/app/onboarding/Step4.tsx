import React from 'react';

export default function Step4({
  intakeData,
  firstProductName, setFirstProductName,
  firstProductPrice, setFirstProductPrice,
  template, setTemplate,
  domain, setDomain,
  setStep,
  handleStartOnboarding,
  isLoading
}: any) {
  if (!intakeData) return null;
  return (
    <div className="flex flex-col flex-1 justify-start animate-fade-in pb-8">
      <div className="w-16 h-16 bg-[#eef2ff] rounded-full flex items-center justify-center mb-6 mx-auto shrink-0">
        <span className="text-3xl text-[#0066FF]">✨</span>
      </div>
      <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-2 text-center shrink-0">Ready to Launch!</h2>
      <p className="text-gray-500 text-sm mb-6 text-center shrink-0">Review your AI-generated setup and choose your options.</p>

      <div className="space-y-6 flex-1 overflow-visible">
        {/* Product Section */}
        <div className="bg-white/40 backdrop-blur-md p-5 rounded-[16px] border border-white/50 shadow-sm space-y-3">
           <h3 className="font-bold text-gray-900 font-outfit">First Product/Service</h3>
           <div className="flex gap-3">
             <div className="flex-1">
               <label className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1 block">Name</label>
               <input
                 type="text"
                 value={firstProductName || (intakeData.initial_products?.[0]?.name || '')}
                 onChange={(e) => setFirstProductName(e.target.value)}
                 className="w-full p-3 rounded-[10px] border border-white/50 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none bg-white/60 backdrop-blur-sm text-gray-900 shadow-inner transition-all"
                 placeholder="e.g. Custom Cake"
               />
             </div>
             <div className="w-24">
               <label className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-1 block">Price</label>
               <input
                 type="text"
                 inputMode="decimal"
                 pattern="[0-9]*\.?[0-9]*"
                 value={firstProductPrice || (intakeData.initial_products?.[0]?.price || '')}
                 onChange={(e) => setFirstProductPrice(e.target.value)}
                 className="w-full p-3 rounded-[10px] border border-white/50 focus:border-[#0066FF] focus:ring-2 focus:ring-[#0066FF]/30 outline-none bg-white/60 backdrop-blur-sm text-gray-900 shadow-inner transition-all"
                 placeholder="0.00"
               />
             </div>
           </div>
        </div>

        {/* Template Selection */}
        <div className="space-y-3">
           <h3 className="font-bold text-gray-900 font-outfit pl-1">Choose a Template</h3>
           <div className="grid grid-cols-2 gap-3">
             {['Modern', 'Elegant', 'Playful', 'Minimal'].map((t) => (
               <button
                 key={t}
                 onClick={() => setTemplate(t)}
                 className={`p-3 rounded-[12px] border ${template === t ? 'border-[#0066FF] bg-white/70 backdrop-blur-md text-[#0066FF] font-bold shadow-sm' : 'border-white/50 bg-white/40 backdrop-blur-md text-gray-700 hover:border-white/80'} transition-all text-sm`}
               >
                 {t}
               </button>
             ))}
           </div>
        </div>

        {/* Domain Selection */}
        <div className="space-y-3">
           <h3 className="font-bold text-gray-900 font-outfit pl-1">Domain Name</h3>
           <div className="flex flex-col gap-3">
             <button
               onClick={() => setDomain('free')}
               className={`p-4 rounded-[12px] border flex justify-between items-center ${domain === 'free' ? 'border-[#0066FF] bg-white/70 backdrop-blur-md text-[#0066FF] font-bold shadow-sm' : 'border-white/50 bg-white/40 backdrop-blur-md text-gray-700 hover:border-white/80'} transition-all text-sm`}
             >
               <span>Free OHC Domain</span>
               <span className="text-xs opacity-70 font-normal">myshop.ohc.store</span>
             </button>
             <button
               onClick={() => setDomain('custom')}
               className={`p-4 rounded-[12px] border flex justify-between items-center ${domain === 'custom' ? 'border-[#0066FF] bg-white/70 backdrop-blur-md text-[#0066FF] font-bold shadow-sm' : 'border-white/50 bg-white/40 backdrop-blur-md text-gray-700 hover:border-white/80'} transition-all text-sm`}
             >
               <span>Connect Custom Domain</span>
               <span className="text-xs opacity-70 font-normal">www.myshop.com</span>
             </button>
           </div>
        </div>
      </div>

      <div className="flex gap-3 mt-auto">
        <button
          onClick={() => setStep(3)}
          className="px-6 py-4 rounded-[12px] font-bold bg-white/50 backdrop-blur-sm text-gray-700 hover:bg-white/70 shadow-sm border border-white/40 transition-all"
          disabled={isLoading}
        >
          Edit
        </button>
        <button
          onClick={handleStartOnboarding}
          disabled={isLoading}
          className="flex-1 bg-gradient-to-r from-[#34C759] to-[#2eb350] text-white p-4 rounded-[12px] font-bold shadow-md hover:shadow-lg hover:scale-[1.02] active:scale-[0.98] transition-all disabled:opacity-70 flex justify-center items-center"
        >
          {isLoading ? (
            <span className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
          ) : (
            "Publish Now"
          )}
        </button>
      </div>
    </div>
  );
}
