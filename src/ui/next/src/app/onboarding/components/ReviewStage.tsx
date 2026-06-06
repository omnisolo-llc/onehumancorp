import React, { useState } from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './Icons';

export function ReviewStage({ onSaveDraft }: { onSaveDraft: () => Promise<void> }) {
  const {
    setStep,
    businessName, setBusinessName,
    businessType, setBusinessType,
    categories, setCategories,
    firstProductName, setFirstProductName,
    firstProductPrice, setFirstProductPrice,
    saveMessage
  } = useOnboardingStore();

  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});
  const [localError, setLocalError] = useState('');

  return (
    <div className="flex flex-col flex-1 overflow-hidden animate-fade-in">
      <div className="flex items-center justify-between mb-6 shrink-0">
        <button onClick={() => setStep(1)} className="text-[#0066FF] text-sm font-bold flex items-center gap-1 hover:underline transition-all">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back to Chat
        </button>
        <button
          onClick={onSaveDraft}
          className="text-xs font-bold text-[#0066FF] bg-[#0066FF]/10 px-3 py-1.5 rounded-full hover:bg-[#0066FF]/20 transition-all"
        >
          <IconLabel icon="save">Save Draft</IconLabel>
        </button>
      </div>

      <div className="mb-6 shrink-0">
        <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 tracking-tight">Review Your Business</h2>
        <p className="text-gray-500 dark:text-[#A1A1A6] text-sm font-medium">
          Our AI has drafted your store profile. Fine-tune any details before we launch.
        </p>
      </div>

      {saveMessage && <p className="text-[#34C759] text-xs font-bold mb-4 animate-fade-in">{saveMessage}</p>}
      {localError && <p className="text-red-500 text-xs font-bold mb-4 animate-shake">{localError}</p>}

      <div className="space-y-5 flex-1 overflow-y-auto custom-scrollbar pr-2 pb-4">
        <div className="space-y-4">
          <div className="mac-glass-container p-4 rounded-[16px] border border-white/20">
            <label className="block text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-[0.1em] mb-2">Business Identity</label>
            <div className="space-y-4">
              <div>
                <span className="block text-xs font-bold text-gray-500 dark:text-gray-400 mb-1.5">Business Name</span>
                <input
                  type="text"
                  autoFocus
                  value={businessName}
                  onChange={(e) => {
                    setBusinessName(e.target.value);
                    if (e.target.value.trim().length < 3) {
                      setValidationErrors(prev => ({ ...prev, businessName: 'Must be at least 3 characters.' }));
                    } else {
                      setValidationErrors(prev => { const { businessName, ...rest } = prev; return rest; });
                    }
                  }}
                  className={`w-full p-3 rounded-[12px] bg-white/50 dark:bg-black/20 border ${validationErrors.businessName ? 'border-red-500' : 'border-black/5 dark:border-white/5 focus:border-[#0066FF]'} outline-none text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all shadow-sm`}
                />
                {validationErrors.businessName && <p className="text-red-500 text-[10px] font-bold mt-1.5">{validationErrors.businessName}</p>}
              </div>
              <div>
                <span className="block text-xs font-bold text-gray-500 dark:text-gray-400 mb-1.5">Business Type</span>
                <input
                  type="text"
                  value={businessType}
                  onChange={(e) => {
                    setBusinessType(e.target.value);
                    if (e.target.value.trim().length === 0) {
                      setValidationErrors(prev => ({ ...prev, businessType: 'Required to configure your agents.' }));
                    } else {
                      setValidationErrors(prev => { const { businessType, ...rest } = prev; return rest; });
                    }
                  }}
                  className={`w-full p-3 rounded-[12px] bg-white/50 dark:bg-black/20 border ${validationErrors.businessType ? 'border-red-500' : 'border-black/5 dark:border-white/5 focus:border-[#0066FF]'} outline-none text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all shadow-sm`}
                />
                {validationErrors.businessType && <p className="text-red-500 text-[10px] font-bold mt-1.5">{validationErrors.businessType}</p>}
              </div>
            </div>
          </div>

          <div className="mac-glass-container p-4 rounded-[16px] border border-white/20">
            <label className="block text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-[0.1em] mb-2">Offerings & Catalog</label>
            <div className="space-y-4">
              <div>
                <span className="block text-xs font-bold text-gray-500 dark:text-gray-400 mb-1.5">Focus Areas</span>
                <input
                  type="text"
                  value={categories.join(', ')}
                  onChange={(e) => setCategories(e.target.value.split(',').map(c => c.trim()))}
                  className="w-full p-3 rounded-[12px] bg-white/50 dark:bg-black/20 border border-black/5 dark:border-white/5 focus:border-[#0066FF] outline-none text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all shadow-sm"
                />
              </div>
              <div className="grid grid-cols-2 gap-3">
                 <div>
                    <span className="block text-xs font-bold text-gray-500 dark:text-gray-400 mb-1.5">Signature Item</span>
                    <input
                      type="text"
                      value={firstProductName}
                      onChange={(e) => setFirstProductName(e.target.value)}
                      className="w-full p-3 rounded-[12px] bg-white/50 dark:bg-black/20 border border-black/5 dark:border-white/5 focus:border-[#0066FF] outline-none text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all shadow-sm"
                    />
                 </div>
                 <div>
                    <span className="block text-xs font-bold text-gray-500 dark:text-gray-400 mb-1.5">Price ($)</span>
                    <input
                      type="text"
                      inputMode="decimal"
                      value={firstProductPrice}
                      onChange={(e) => {
                         setFirstProductPrice(e.target.value);
                         if (e.target.value.trim().length === 0) {
                            setValidationErrors(prev => ({ ...prev, firstProductPrice: 'A price is needed.' }));
                         } else if (!/^\d+(\.\d{1,2})?$/.test(e.target.value)) {
                            setValidationErrors(prev => ({ ...prev, firstProductPrice: 'Invalid price.' }));
                         } else {
                            setValidationErrors(prev => { const { firstProductPrice, ...rest } = prev; return rest; });
                         }
                      }}
                      className={`w-full p-3 rounded-[12px] bg-white/50 dark:bg-black/20 border ${validationErrors.firstProductPrice ? 'border-red-500' : 'border-black/5 dark:border-white/5 focus:border-[#0066FF]'} outline-none text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all shadow-sm`}
                    />
                    {validationErrors.firstProductPrice && <p className="text-red-500 text-[10px] font-bold mt-1.5">{validationErrors.firstProductPrice}</p>}
                 </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="mt-auto pt-6 shrink-0">
        <button
          onClick={() => {
            if (businessName.trim().length < 3) {
              setLocalError('Business Name must be at least 3 characters.');
              return;
            }
            if (Object.keys(validationErrors).length > 0) {
              setLocalError('Please fix the errors before continuing.');
              return;
            }
            setLocalError('');
            setStep(3);
          }}
          disabled={!businessName.trim() || !businessType.trim() || categories.length === 0 || !firstProductName.trim() || !firstProductPrice.trim()}
          className="w-full bg-[#0066FF] text-white h-[58px] rounded-[16px] font-bold text-lg shadow-[0_4px_14px_0_rgba(0,102,255,0.4)] hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
        >
          <IconLabel icon="next">Everything Looks Good</IconLabel>
        </button>
      </div>
    </div>
  );
}
