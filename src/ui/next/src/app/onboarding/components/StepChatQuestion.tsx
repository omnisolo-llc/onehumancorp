import React from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './IconLabel';

interface StepChatQuestionProps {
  handleSaveDraft: () => void;
  saveMessage: string;
  syncStateToBackend: (state: any) => void;
  validationError: string;
  setValidationError: React.Dispatch<React.SetStateAction<string>>;
  handleIntake?: () => void;
  isSubmitting?: boolean;
}

export function StepChatQuestion({
  handleSaveDraft,
  saveMessage,
  syncStateToBackend,
  validationError,
  setValidationError,
  handleIntake,
  isSubmitting
}: StepChatQuestionProps) {
  const { updateState, chatStep, businessName, whatYouSell, location, targetAudience, isLoading } = useOnboardingStore();

  if (chatStep === 1) {
    return (
      <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in w-full">
        <button onClick={() => { updateState({ step: -2 }); syncStateToBackend({ step: -2 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
        </button>
        <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center w-full">What's the name of your business?</h2>
        <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
          <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
            Our AI will instantly generate your storefront, products, and back-office agents.
          </p>
          <button
            onClick={handleSaveDraft}
            className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
          >
            <IconLabel icon="save">Save Draft</IconLabel>
          </button>
        </div>

        {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

        <div className="space-y-4 flex-1 w-full">
          <div>
            <input
              type="text"
              autoFocus
              autoCapitalize="words"
              autoComplete="organization"
              value={businessName}
              onChange={(e) => {
                const val = e.target.value;
                updateState({ businessName: val });
                if (val.trim().length < 3) { setValidationError('Business Name must be at least 3 characters.'); }
                else { setValidationError(''); }
              }}
              onKeyDown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault();
                  if (businessName.trim().length < 3) {
                    setValidationError('Business Name must be at least 3 characters.');
                    return;
                  }
                  setValidationError('');
                  updateState({ chatStep: 2 }); syncStateToBackend({ chatStep: 2 });
                }
              }}
              placeholder="e.g. Maya's Custom Cakes"
              className={`w-full p-3 sm:p-4 outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border ${validationError === 'Business Name must be at least 3 characters.' ? 'border-[#FF3B30]' : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]'} rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] min-h-[44px]`}
              inputMode="text"
              enterKeyHint="next"
            />
          </div>
        </div>

        {validationError && <p className="text-[#FF3B30] text-sm font-semibold mb-2">{validationError}</p>}
        <div className="mt-auto pt-6 w-full">
          <button
            onClick={() => {
              if (businessName.trim().length < 3) {
                setValidationError('Business Name must be at least 3 characters.');
                return;
              }
              setValidationError('');
              updateState({ chatStep: 2 }); syncStateToBackend({ chatStep: 2 });
            }}
            disabled={false}
            className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
          >
            <IconLabel icon="next">Next</IconLabel>
          </button>
        </div>
      </div>
    );
  }

  if (chatStep === 2) {
    return (
      <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in w-full">
        <button onClick={() => { updateState({ chatStep: 1 }); syncStateToBackend({ chatStep: 1 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
        </button>
        <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center w-full">What do you sell?</h2>
        <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
          <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
            Tell us a bit about your products or services.
          </p>
          <button
            onClick={handleSaveDraft}
            className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
          >
            <IconLabel icon="save">Save Draft</IconLabel>
          </button>
        </div>

        {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

        <div className="space-y-4 flex-1 w-full">
          <div>
            <textarea
              autoFocus
              autoCapitalize="sentences"
              value={whatYouSell}
              onChange={(e) => {
                const val = e.target.value;
                updateState({ whatYouSell: val });
                if (!val.trim()) { setValidationError('Please tell us what you sell.'); }
                else { setValidationError(''); }
              }}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  if (!whatYouSell.trim()) {
                    setValidationError('Please tell us what you sell.');
                    return;
                  }
                  setValidationError('');
                  updateState({ chatStep: 3 }); syncStateToBackend({ chatStep: 3 });
                }
              }}
              placeholder="e.g. I bake custom vegan cakes"
              className={`w-full p-3 sm:p-4 outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border ${validationError === 'Please tell us what you sell.' ? 'border-[#FF3B30]' : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]'} rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] h-32 resize-none transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]`}
            />
          </div>
        </div>

        {validationError && <p className="text-[#FF3B30] text-sm font-semibold mb-2">{validationError}</p>}
        <div className="mt-auto pt-6 w-full">
          <button
            onClick={() => {
              if (!whatYouSell.trim()) {
                setValidationError('Please tell us what you sell.');
                return;
              }
              setValidationError('');
              updateState({ chatStep: 3 }); syncStateToBackend({ chatStep: 3 });
            }}
            disabled={false}
            className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
          >
            <IconLabel icon="next">Next</IconLabel>
          </button>
        </div>
      </div>
    );
  }

  if (chatStep === 3) {
    return (
      <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in w-full">
        <button onClick={() => { updateState({ chatStep: 2 }); syncStateToBackend({ chatStep: 2 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
        </button>
        <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center w-full">Where are you located?</h2>
        <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
          <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
            This helps us set up your shipping and tax settings.
          </p>
          <button
            onClick={handleSaveDraft}
            className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
          >
            <IconLabel icon="save">Save Draft</IconLabel>
          </button>
        </div>

        {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

        <div className="space-y-4 flex-1 w-full">
          <div>
            <input
              type="text"
              autoFocus
              autoCapitalize="words"
              value={location}
              onChange={(e) => updateState({ location: e.target.value })}
              onKeyDown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault();
                  e.stopPropagation();
                  if (!location.trim()) {
                    setValidationError('Please tell us your location.');
                    return;
                  }
                  setValidationError('');
                  updateState({ chatStep: 4 }); syncStateToBackend({ chatStep: 4 });
                }
              }}
              placeholder="e.g. Portland, OR"
              className={`w-full p-3 sm:p-4 outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border ${validationError === 'Please tell us your location.' ? 'border-[#FF3B30]' : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]'} rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] min-h-[44px]`}
            />
          </div>
        </div>

        {validationError && <p className="text-[#FF3B30] text-sm font-semibold mb-2">{validationError}</p>}
        <div className="mt-auto pt-6 w-full">
          <button
            onClick={() => {
              if (!location.trim()) {
                setValidationError('Please tell us your location.');
                return;
              }
              setValidationError('');
              updateState({ chatStep: 4 }); syncStateToBackend({ chatStep: 4 });
            }}
            disabled={false}
            className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
          >
            <IconLabel icon="next">Next</IconLabel>
          </button>
        </div>
      </div>
    );
  }

  if (chatStep === 4) {
    return (
      <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in w-full">
        <button onClick={() => { updateState({ chatStep: 3 }); syncStateToBackend({ chatStep: 3 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
        </button>
        <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 text-center w-full">Who is your target audience?</h2>
        <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
          <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
            This helps our AI generate the perfect storefront copy and select the best tools for your business.
          </p>
          <button
            onClick={handleSaveDraft}
            className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
          >
            <IconLabel icon="save">Save Draft</IconLabel>
          </button>
        </div>

        {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

        <div className="space-y-4 flex-1 w-full">
          <div>
            <input
              type="text"
              autoFocus
              autoCapitalize="words"
              value={targetAudience}
              onChange={(e) => updateState({ targetAudience: e.target.value })}
              onKeyDown={(e) => {
                if (e.key === 'Enter') {
                  e.preventDefault();
                  e.stopPropagation();
                  if (!targetAudience.trim()) {
                    setValidationError('Please tell us your target audience.');
                    return;
                  }
                  setValidationError('');
                  if (handleIntake) handleIntake();
                }
              }}
              placeholder="e.g. Local families, Tech startups"
              className={`w-full p-3 sm:p-4 outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border ${validationError === 'Please tell us your target audience.' ? 'border-[#FF3B30]' : 'border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]'} rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] text-lg transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] min-h-[44px]`}
            />
          </div>
        </div>

        {validationError && <p className="text-[#FF3B30] text-sm font-semibold mb-2">{validationError}</p>}
        <div className="mt-auto pt-6 w-full">
          <button
            onClick={() => {
              if (!targetAudience.trim()) {
                setValidationError('Please tell us your target audience.');
                return;
              }
              setValidationError('');
              if (handleIntake) handleIntake();
            }}
            disabled={isLoading || isSubmitting}
            className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#005bb5] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
          >
            {isLoading || isSubmitting ? (
              <span className="flex items-center justify-center gap-2">
                <svg className="animate-spin h-5 w-5 text-white rounded-full shadow-[0_0_10px_rgba(255,255,255,0.5)]" fill="none" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Analyzing...
              </span>
            ) : <IconLabel icon="launch">Next</IconLabel>}
          </button>
        </div>
      </div>
    );
  }

  return null;
}
