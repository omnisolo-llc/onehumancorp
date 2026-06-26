import React from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './IconLabel';

interface StepStyleTeamProps {
  handleStartOnboarding: () => void;
  handleSaveDraft: () => void;
  saveMessage: string;
  syncStateToBackend: (state: any) => void;
  validationErrors: Record<string, string>;
  setValidationErrors: React.Dispatch<React.SetStateAction<Record<string, string>>>;
  isSubmitting: boolean;
}

export function StepStyleTeam({
  handleStartOnboarding,
  handleSaveDraft,
  saveMessage,
  syncStateToBackend,
  validationErrors,
  setValidationErrors,
  isSubmitting
}: StepStyleTeamProps) {
  const {
    updateState, websiteTemplate, domainChoice,
    adminName, adminEmail, adminPassword,
    aiAgents, aiAutoRespond, isLoading
  } = useOnboardingStore();

  return (
    <div className="flex flex-col justify-center items-center gap-4 flex-1 animate-fade-in w-full">
      <button onClick={() => { updateState({ step: 2 }); syncStateToBackend({ step: 2 }); }} className="self-start text-[#0066FF] text-sm font-semibold mb-4 flex items-center gap-1 min-h-[44px] min-w-[44px] p-2">
        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back
      </button>
      <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2">Style & Team</h2>
      <div className="flex items-start sm:items-center justify-between mb-6 w-full gap-2">
        <p className="text-gray-500 dark:text-[#A1A1A6] text-sm pr-4">
          Pick your storefront vibe. We'll automatically assign the best AI agents to manage it.
        </p>
        <button
          onClick={handleSaveDraft}
          className="text-sm font-semibold text-[#0066FF] hover:underline whitespace-nowrap shrink-0 ml-auto flex items-center justify-center min-h-[44px] min-w-[44px] p-2"
        >
          <IconLabel icon="save">Save Draft</IconLabel>
        </button>
      </div>

      {saveMessage && <p className="text-[#34C759] text-sm font-semibold mb-2">{saveMessage}</p>}

      <div className="space-y-4 flex-1 overflow-y-auto pr-2 w-full hide-scrollbar">
        <div>
          <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Website Template</label>
          <div className="grid grid-cols-2 gap-3">
            {['Modern', 'Minimal', 'Bold', 'Classic'].map(template => (
              <div
                key={template}
                onClick={() => updateState({ websiteTemplate: template })}
                className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] ${websiteTemplate === template ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] hover:border-gray-400 dark:hover:border-gray-500 text-[#1D1D1F] dark:text-white'}`}
              >
                <div className="font-semibold text-sm">{template}</div>
              </div>
            ))}
          </div>
        </div>

        <div className="pt-2 border-t border-white/50 dark:border-[rgba(255,255,255,0.1)]">
          <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Web Address</label>
          <div className="grid grid-cols-2 gap-3 mb-2">
            <div
              onClick={() => updateState({ domainChoice: 'subdomain' })}
              className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] flex flex-col items-center justify-center text-center ${domainChoice === 'subdomain' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500'}`}
            >
              <span className="font-semibold text-sm mb-1">Free Subdomain</span>
              <span className="text-[10px] opacity-70">your-name.ohc.app</span>
            </div>
            <div
              onClick={() => updateState({ domainChoice: 'custom' })}
              className={`p-3 border cursor-pointer transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] flex flex-col items-center justify-center text-center ${domainChoice === 'custom' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'border-white/50 dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] text-[#1D1D1F] dark:text-white hover:border-gray-400 dark:hover:border-gray-500'}`}
            >
              <span className="font-semibold text-sm mb-1">Custom Domain</span>
              <span className="text-[10px] opacity-70">your-name.com</span>
            </div>
          </div>
        </div>

        <div className="pt-2 border-t border-white/50 dark:border-[rgba(255,255,255,0.1)]">
          <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Account Setup</label>
          <div className="space-y-3 mb-4">
            <div>
              <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Name</label>
              <input
                type="text"
                autoCapitalize="words"
                autoComplete="name"
                value={adminName}
                onChange={(e) => {
                  const val = e.target.value;
                  updateState({ adminName: val });
                  if (!val.trim()) {
                    setValidationErrors(prev => ({ ...prev, adminName: 'Admin Name is required' }));
                  } else {
                    setValidationErrors(prev => { const { adminName, ...rest } = prev; return rest; });
                  }
                }}
                placeholder="e.g. Maya Smith"
                className={`w-full p-3 sm:p-4 border ${validationErrors.adminName ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]"} outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] transition-all duration-[250ms]`}
                inputMode="text"
                enterKeyHint="next"
              />
              {validationErrors.adminName && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.adminName}</p>}
            </div>
            <div>
              <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Email</label>
              <input
                type="email"
                autoCapitalize="none"
                autoComplete="email"
                inputMode="email"
                enterKeyHint="next"
                value={adminEmail}
                onChange={(e) => {
                  const val = e.target.value;
                  updateState({ adminEmail: val });
                  if (!val.trim()) {
                    setValidationErrors(prev => ({ ...prev, adminEmail: 'Admin Email is required' }));
                  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(val)) {
                    setValidationErrors(prev => ({ ...prev, adminEmail: 'Please enter a valid email address' }));
                  } else {
                    setValidationErrors(prev => { const { adminEmail, ...rest } = prev; return rest; });
                  }
                }}
                placeholder="you@example.com"
                className={`w-full p-3 sm:p-4 border ${validationErrors.adminEmail ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]"} outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] transition-all duration-[250ms]`}
              />
              {validationErrors.adminEmail && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.adminEmail}</p>}
            </div>
            <div>
              <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 mb-1">Admin Password</label>
              <input
                type="password"
                enterKeyHint="done"
                autoComplete="new-password"
                value={adminPassword}
                onChange={(e) => {
                  const val = e.target.value;
                  updateState({ adminPassword: val });
                  if (!val.trim()) {
                    setValidationErrors(prev => ({ ...prev, adminPassword: 'Password is required' }));
                  } else if (val.length < 8 || !/\d/.test(val)) {
                    setValidationErrors(prev => ({ ...prev, adminPassword: 'Password must be at least 8 characters and contain a number' }));
                  } else {
                    setValidationErrors(prev => { const { adminPassword, ...rest } = prev; return rest; });
                  }
                }}
                placeholder="••••••••"
                className={`w-full p-3 sm:p-4 border ${validationErrors.adminPassword ? "border-[#FF3B30]" : "border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] focus:border-[#0066FF]"} outline-none bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] rounded-[8px] text-[#1D1D1F] dark:text-[#F5F5F7] min-h-[44px] transition-all duration-[250ms]`}
              />
              {validationErrors.adminPassword && <p className="text-[#FF3B30] text-xs mt-1">{validationErrors.adminPassword}</p>}
            </div>
          </div>
        </div>

        <div className="pt-2 border-t border-white/50 dark:border-[rgba(255,255,255,0.1)]">
          <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300 uppercase tracking-wide mb-2">Auto-Configured AI Departments</label>
          <p className="text-gray-500 dark:text-[#A1A1A6] text-xs mb-2">
            Here are the AI departments we've configured for you.
          </p>
          <div className="flex flex-col sm:flex-row flex-wrap gap-2 mt-2">
            {aiAgents.map(agent => (
              <div
                key={agent}
                className="px-3 py-1.5 rounded-[8px] border border-[#34C759] bg-[#34C759]/10 text-[#34C759] flex items-center gap-1.5 text-sm font-semibold transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)]"
              >
                <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                {agent}
              </div>
            ))}
          </div>
        </div>

        <div className="pt-2">
          <label className="flex items-center justify-between cursor-pointer p-3 bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[8px] text-[#1D1D1F] dark:text-white transition-all duration-[250ms]">
            <span className="font-semibold text-sm">Allow AI to Auto-Respond</span>
            <input
              type="checkbox"
              className="sr-only"
              checked={aiAutoRespond}
              onChange={(e) => updateState({ aiAutoRespond: e.target.checked })}
            />
            <div className={`w-10 h-6 rounded-full transition-colors ${aiAutoRespond ? 'bg-[#34C759]' : 'bg-gray-300 dark:bg-gray-600'} relative`}>
               <div className={`w-4 h-4 rounded-full bg-white absolute top-1 transition-transform ${aiAutoRespond ? 'translate-x-5' : 'translate-x-1'}`}></div>
            </div>
          </label>
        </div>
      </div>

      <div className="mt-auto pt-6 w-full">
        <button
          onClick={handleStartOnboarding}
          disabled={isLoading || isSubmitting}
          className="w-full bg-[#0066FF] text-white p-4 font-bold shadow-[0_4px_14px_0_rgba(0,102,255,0.39)] hover:bg-[#0052cc] active:scale-[0.98] transition-all duration-[250ms] ease-[cubic-bezier(0.4,0,0.2,1)] disabled:opacity-50 disabled:cursor-not-allowed rounded-[8px]"
        >
          {isLoading || isSubmitting ? (
            <span className="flex items-center justify-center gap-2">
              <svg className="animate-spin h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Launching...
            </span>
          ) : <IconLabel icon="launch">Approve & Publish</IconLabel>}
        </button>
      </div>
    </div>
  );
}
