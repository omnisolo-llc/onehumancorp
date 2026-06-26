import React from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './IconLabel';

interface StepReviewProps {
  handleSaveDraft: () => void;
  saveMessage: string;
  syncStateToBackend: (state: any) => void;
  validationErrors: Record<string, string>;
  setValidationErrors: React.Dispatch<React.SetStateAction<Record<string, string>>>;
  validationError: string;
  setValidationError: React.Dispatch<React.SetStateAction<string>>;
}

export function StepReview({
  handleSaveDraft,
  saveMessage,
  syncStateToBackend,
  validationErrors,
  setValidationErrors,
  validationError,
  setValidationError
}: StepReviewProps) {
  const { updateState, businessName, businessType, categories, firstProductName, firstProductPrice } = useOnboardingStore();

  return (
    <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in">
      <button onClick={() => { updateState({ step: 1, chatStep: 4 }); syncStateToBackend({ step: 1, chatStep: 4 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
      </button>
      <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Review Details</h2>
      <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
        <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
          Here's what our AI figured out. Feel free to tweak these.
        </p>
        <button
          onClick={handleSaveDraft}
          className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
        >
          <IconLabel icon="save">Save Draft</IconLabel>
        </button>
      </div>

      {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

      <div className="space-y-4 flex-1 overflow-y-auto pr-2 w-full">
        <div>
          <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Business Name</label>
          <input
            type="text"
            autoFocus
            autoCapitalize="words"
            value={businessName}
            onChange={(e) => {
              updateState({ businessName: e.target.value });
              setValidationErrors(prev => { const { businessName, ...rest } = prev; return rest; });
            }}
            className={`w-full p-3 sm:p-4 border ${validationErrors.businessName ? 'border-[#FF3B30]' : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]'} outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] transition-all duration-[250ms]`}
          />
          {validationErrors.businessName && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.businessName}</p>}
        </div>
        <div>
          <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Business Type</label>
          <input
            type="text"
            autoCapitalize="words"
            value={businessType}
            onChange={(e) => {
              updateState({ businessType: e.target.value });
              setValidationErrors(prev => { const { businessType, ...rest } = prev; return rest; });
            }}
            className={`w-full p-3 sm:p-4 border ${validationErrors.businessType ? 'border-[#FF3B30]' : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]'} outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] transition-all duration-[250ms]`}
          />
          {validationErrors.businessType && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.businessType}</p>}
        </div>
        <div>
          <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Categories (Comma separated)</label>
          <input
            type="text"
            autoCapitalize="words"
            value={categories.join(', ')}
            onChange={(e) => updateState({ categories: e.target.value.split(',').map(c => c.trim()) })}
            className="w-full p-3 sm:p-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] transition-all duration-[250ms]"
          />
        </div>
        <div className="grid grid-cols-2 gap-2">
           <div>
              <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">First Product</label>
              <input
                type="text"
                autoCapitalize="words"
                value={firstProductName}
                onChange={(e) => updateState({ firstProductName: e.target.value })}
                className="w-full p-3 sm:p-4 border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF] outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] transition-all duration-[250ms]"
              />
           </div>
           <div>
              <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-1">Price</label>
              <input
                type="text"
                inputMode="decimal"
                value={firstProductPrice}
                onChange={(e) => {
                   updateState({ firstProductPrice: e.target.value });
                   if (e.target.value.trim().length > 0 && !/^\d+(\.\d{1,2})?$/.test(e.target.value)) {
                      setValidationErrors(prev => ({ ...prev, firstProductPrice: 'Invalid price.' }));
                   } else {
                      setValidationErrors(prev => { const { firstProductPrice, ...rest } = prev; return rest; });
                   }
                }}
                className={`w-full p-3 sm:p-4 border ${validationErrors.firstProductPrice ? 'border-[#FF3B30]' : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]'} outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] transition-all duration-[250ms]`}
              />
              {validationErrors.firstProductPrice && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.firstProductPrice}</p>}
           </div>
        </div>
      </div>

      {validationError && <p className="text-[#FF3B30] text-sm font-semibold mb-2">{validationError}</p>}
      <div className="mt-auto pt-6 w-full">
        <button
          onClick={() => {
            let hasError = false;
            const newErrors: Record<string, string> = { ...validationErrors };
            if (businessName.trim().length < 3) {
              newErrors.businessName = 'Must be at least 3 characters.';
              hasError = true;
            }
            if (businessType.trim().length === 0) {
              newErrors.businessType = 'Business Type is required to configure your agents.';
              hasError = true;
            }
            if (firstProductPrice.trim().length === 0) {
              newErrors.firstProductPrice = 'A price is needed to set up your Stripe catalog.';
              hasError = true;
            }

            if (hasError || Object.keys(newErrors).length > 0) {
              setValidationErrors(newErrors);
              setValidationError('Please fix the errors before continuing.');
              return;
            }

            setValidationError('');
            updateState({ step: 3 }); syncStateToBackend({ step: 3 });
          }}
          className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
        >
          <IconLabel icon="next">Continue</IconLabel>
        </button>
      </div>
    </div>
  );
}
