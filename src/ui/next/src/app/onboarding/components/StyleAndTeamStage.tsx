import React, { useState } from 'react';
import { useOnboardingStore } from '../store';
import { IconLabel } from './Icons';

export function StyleAndTeamStage({ onLaunch, onSaveDraft }: { onLaunch: () => Promise<void>; onSaveDraft: () => Promise<void> }) {
  const {
    setStep,
    websiteTemplate, setWebsiteTemplate,
    domainChoice, setDomainChoice,
    adminName, setAdminName,
    adminEmail, setAdminEmail,
    adminPassword, setAdminPassword,
    aiAgents, setAiAgents,
    aiAutoRespond, setAiAutoRespond,
    isLoading,
    saveMessage
  } = useOnboardingStore();

  const [validationErrors, setValidationErrors] = useState<Record<string, string>>({});

  const handleLaunch = async () => {
    const errors: Record<string, string> = {};
    if (!adminName.trim()) {
      errors.adminName = 'Admin Name is required';
    }
    if (!adminEmail.trim()) {
      errors.adminEmail = 'Admin Email is required';
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(adminEmail)) {
      errors.adminEmail = 'Please enter a valid email address';
    }
    if (!adminPassword.trim()) {
      errors.adminPassword = 'Password is required';
    } else if (adminPassword.length < 8 || !/\d/.test(adminPassword)) {
      errors.adminPassword = 'Password must be at least 8 characters and contain a number';
    }

    if (Object.keys(errors).length > 0) {
      setValidationErrors(errors);
      return;
    }

    setValidationErrors({});
    await onLaunch();
  };

  return (
    <div className="flex flex-col flex-1 overflow-hidden animate-fade-in">
      <div className="flex items-center justify-between mb-6 shrink-0">
        <button onClick={() => setStep(2)} className="text-[#0066FF] text-sm font-bold flex items-center gap-1 hover:underline transition-all">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg> Back to Review
        </button>
        <button
          onClick={onSaveDraft}
          className="text-xs font-bold text-[#0066FF] bg-[#0066FF]/10 px-3 py-1.5 rounded-full hover:bg-[#0066FF]/20 transition-all"
        >
          <IconLabel icon="save">Save Draft</IconLabel>
        </button>
      </div>

      <div className="mb-6 shrink-0">
        <h2 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-2 tracking-tight">Style & Team</h2>
        <p className="text-gray-500 dark:text-[#A1A1A6] text-sm font-medium">
          Pick your storefront vibe. We'll automatically assign the best AI agents to manage it.
        </p>
      </div>

      {saveMessage && <p className="text-[#34C759] text-xs font-bold mb-4 animate-fade-in">{saveMessage}</p>}

      <div className="space-y-6 flex-1 overflow-y-auto custom-scrollbar pr-2 pb-4">
        <div className="space-y-6">
          <div>
            <label className="block text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-[0.1em] mb-3">Storefront Vibe</label>
            <div className="grid grid-cols-2 gap-3">
              {['Modern', 'Minimal', 'Bold', 'Classic'].map(template => (
                <div
                  key={template}
                  onClick={() => setWebsiteTemplate(template)}
                  className={`p-4 rounded-[16px] border cursor-pointer transition-all flex flex-col gap-2 ${websiteTemplate === template ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] shadow-inner' : 'mac-glass-container border-black/5 dark:border-white/5 hover:border-gray-300 dark:hover:border-gray-600 text-[#1D1D1F] dark:text-white'}`}
                >
                  <div className="font-bold text-sm">{template}</div>
                  <div className="h-1 w-8 bg-current opacity-20 rounded-full"></div>
                </div>
              ))}
            </div>
          </div>

          <div>
            <label className="block text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-[0.1em] mb-3">Web Address</label>
            <div className="grid grid-cols-2 gap-3">
              <div
                onClick={() => setDomainChoice('subdomain')}
                className={`p-4 rounded-[16px] border cursor-pointer transition-all flex flex-col gap-1 ${domainChoice === 'subdomain' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] shadow-inner' : 'mac-glass-container border-black/5 dark:border-white/5 text-[#1D1D1F] dark:text-white hover:border-gray-300 dark:hover:border-gray-600'}`}
              >
                <span className="font-bold text-sm">Free Subdomain</span>
                <span className="text-[10px] opacity-60 font-medium">your-name.ohc.store</span>
              </div>
              <div
                onClick={() => setDomainChoice('custom')}
                className={`p-4 rounded-[16px] border cursor-pointer transition-all flex flex-col gap-1 ${domainChoice === 'custom' ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF] shadow-inner' : 'mac-glass-container border-black/5 dark:border-white/5 text-[#1D1D1F] dark:text-white hover:border-gray-300 dark:hover:border-gray-600'}`}
              >
                <span className="font-bold text-sm">Custom Domain</span>
                <span className="text-[10px] opacity-60 font-medium">your-name.com</span>
              </div>
            </div>
          </div>

          <div className="mac-glass-container p-5 rounded-[20px] border border-white/20">
            <label className="block text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-[0.1em] mb-4">Founder Account</label>
            <div className="space-y-4">
              <div>
                <span className="block text-xs font-bold text-gray-500 dark:text-gray-400 mb-1.5">Full Name</span>
                <input
                  type="text"
                  value={adminName}
                  onChange={(e) => setAdminName(e.target.value)}
                  placeholder="Maya Smith"
                  className={`w-full p-3 rounded-[12px] bg-white/50 dark:bg-black/20 border ${validationErrors.adminName ? "border-red-500" : "border-black/5 dark:border-white/5 focus:border-[#0066FF]"} outline-none text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all shadow-sm`}
                />
                {validationErrors.adminName && <p className="text-red-500 text-[10px] font-bold mt-1.5">{validationErrors.adminName}</p>}
              </div>
              <div>
                <span className="block text-xs font-bold text-gray-500 dark:text-gray-400 mb-1.5">Email Address</span>
                <input
                  type="email"
                  value={adminEmail}
                  onChange={(e) => setAdminEmail(e.target.value)}
                  placeholder="you@example.com"
                  className={`w-full p-3 rounded-[12px] bg-white/50 dark:bg-black/20 border ${validationErrors.adminEmail ? "border-red-500" : "border-black/5 dark:border-white/5 focus:border-[#0066FF]"} outline-none text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all shadow-sm`}
                />
                {validationErrors.adminEmail && <p className="text-red-500 text-[10px] font-bold mt-1.5">{validationErrors.adminEmail}</p>}
              </div>
              <div>
                <span className="block text-xs font-bold text-gray-500 dark:text-gray-400 mb-1.5">Secure Password</span>
                <input
                  type="password"
                  value={adminPassword}
                  onChange={(e) => setAdminPassword(e.target.value)}
                  placeholder="••••••••"
                  className={`w-full p-3 rounded-[12px] bg-white/50 dark:bg-black/20 border ${validationErrors.adminPassword ? "border-red-500" : "border-black/5 dark:border-white/5 focus:border-[#0066FF]"} outline-none text-[#1D1D1F] dark:text-[#F5F5F7] font-medium transition-all shadow-sm`}
                />
                {validationErrors.adminPassword && <p className="text-red-500 text-[10px] font-bold mt-1.5">{validationErrors.adminPassword}</p>}
              </div>
            </div>
          </div>

          <div>
            <label className="block text-[10px] font-black text-gray-400 dark:text-gray-500 uppercase tracking-[0.1em] mb-3">Your AI Teammates</label>
            <div className="space-y-2">
              {['Sales Agent', 'Support Agent', 'Marketing Agent'].map(agent => {
                 const isSelected = aiAgents.includes(agent);
                 return (
                   <div
                     key={agent}
                     onClick={() => {
                       if (isSelected) {
                         setAiAgents(aiAgents.filter(a => a !== agent));
                       } else {
                         setAiAgents([...aiAgents, agent]);
                       }
                     }}
                     className={`p-4 rounded-[16px] border cursor-pointer flex items-center justify-between transition-all ${isSelected ? 'border-[#0066FF] bg-[#0066FF]/10 text-[#0066FF]' : 'mac-glass-container border-black/5 dark:border-white/5 text-[#1D1D1F] dark:text-white'}`}
                   >
                     <div className="flex items-center gap-3">
                        <div className={`w-8 h-8 rounded-full flex items-center justify-center ${isSelected ? 'bg-[#0066FF] text-white' : 'bg-gray-100 dark:bg-white/10 text-gray-500'}`}>
                           <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" /></svg>
                        </div>
                        <span className="font-bold text-sm">{agent}</span>
                     </div>
                     <div className={`w-6 h-6 rounded-full border-2 flex items-center justify-center transition-all ${isSelected ? 'border-[#0066FF] bg-[#0066FF]' : 'border-gray-300 dark:border-white/20'}`}>
                        {isSelected && <svg className="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>}
                     </div>
                   </div>
                 );
              })}
            </div>
          </div>

          <div className="pt-2">
            <label className="flex items-center justify-between cursor-pointer p-4 rounded-[16px] mac-glass-container border border-white/20 text-[#1D1D1F] dark:text-white">
              <div className="flex flex-col gap-0.5">
                <span className="font-bold text-sm">Autonomous AI Response</span>
                <span className="text-[10px] opacity-60 font-medium">Allow AI to handle customer messages while you sleep</span>
              </div>
              <input
                type="checkbox"
                className="sr-only"
                checked={aiAutoRespond}
                onChange={(e) => setAiAutoRespond(e.target.checked)}
              />
              <div className={`w-12 h-7 rounded-full transition-colors ${aiAutoRespond ? 'bg-[#34C759]' : 'bg-gray-300 dark:bg-gray-600'} relative shadow-inner`}>
                 <div className={`w-5 h-5 rounded-full bg-white absolute top-1 transition-transform shadow-md ${aiAutoRespond ? 'translate-x-6' : 'translate-x-1'}`}></div>
              </div>
            </label>
          </div>
        </div>
      </div>

      <div className="mt-auto pt-6 shrink-0">
        <button
          onClick={handleLaunch}
          disabled={isLoading}
          className="w-full bg-[#0066FF] text-white h-[58px] rounded-[16px] font-bold text-lg shadow-[0_4px_14px_0_rgba(0,102,255,0.4)] hover:bg-[#0052cc] active:scale-[0.98] transition-all disabled:opacity-50"
        >
          {isLoading ? (
            <span className="flex items-center justify-center gap-2">
              <svg className="animate-spin h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Launching...
            </span>
          ) : <IconLabel icon="launch">Launch My Business</IconLabel>}
        </button>
      </div>
    </div>
  );
}
