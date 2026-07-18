"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function InteractivePollGeneratorPage() {
  const router = useRouter();
  const [question, setQuestion] = useState('What flavor should we make next?');
  const [options, setOptions] = useState(['Chocolate', 'Vanilla', 'Strawberry']);
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const [requireEmail, setRequireEmail] = useState(false);
  const [tenant, setTenant] = useState('my-store');
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [removeBranding, setRemoveBranding] = useState(false);
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);

  useEffect(() => {
    const checkState = () => {
      const tid = typeof window !== 'undefined' ? (localStorage.getItem('tenant_id') || localStorage.getItem('tenant') || 'my-store') : 'my-store';
      setTenant(tid);
      if (typeof window !== 'undefined') {
        setHasPro(localStorage.getItem('has_pro') === 'true');
      }
    };

    checkState();

    // Expose for testing
    if (typeof window !== 'undefined') {
      (window as any).__forceCheckProState = checkState;
    }

    window.addEventListener('storage', checkState);
    return () => {
      window.removeEventListener('storage', checkState);
      if (typeof window !== 'undefined') delete (window as any).__forceCheckProState;
    };
  }, []);

  const handleOptionChange = (index: number, value: string) => {
    const newOptions = [...options];
    newOptions[index] = value;
    setOptions(newOptions);
  };

  const handleAddOption = () => {
    if (options.length < 4) {
      setOptions([...options, 'New Option']);
    }
  };

  const handleRemoveOption = (index: number) => {
    if (options.length > 2) {
      const newOptions = options.filter((_, i) => i !== index);
      setOptions(newOptions);
    }
  };

  const embedUrl = `https://ohc.app/api/v1/growth/interactive-poll/embed?tenant=${tenant}&q=${encodeURIComponent(question)}&opts=${encodeURIComponent(options.join(','))}&theme=${theme}&email=${requireEmail}&hideBranding=${removeBranding}`;

  const embedCode = `<iframe src="${embedUrl}" width="100%" height="${requireEmail ? '350' : '280'}" frameborder="0" scrolling="no" style="border:none; overflow:hidden; border-radius:16px;"></iframe>` + (removeBranding ? '' : `
<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;"><a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenant}" target="_blank" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a></div>`);

  const handleGenerate = () => {
    setShowModal(true);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  const handleRemoveBrandingToggle = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro && e.target.checked) {
      setShowSoftPaywall(true);
      return;
    }
    setRemoveBranding(e.target.checked);
  };

  const previewBg = theme === 'dark' ? 'bg-[#1D1D1F]' : 'bg-white';
  const previewText = theme === 'dark' ? 'text-[#F5F5F7]' : 'text-[#1D1D1F]';
  const previewBorder = theme === 'dark' ? 'border-[#424245]' : 'border-[#E5E5EA]';

  return (
    <div className="min-h-screen bg-[#F5F5F7] dark:bg-[#000000] p-6 font-sans selection:bg-blue-500/30">
      <div className="max-w-6xl mx-auto">
        <header className="mb-8 flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">Interactive Poll Generator</h1>
            <p className="text-gray-500 dark:text-gray-400 mt-2">Create engaging polls to capture leads and customer preferences.</p>
          </div>
          <button
            onClick={() => router.push('/dashboard')}
            className="text-sm font-medium text-blue-600 hover:text-blue-700 dark:text-blue-400 dark:hover:text-blue-300 transition-colors"
          >
            &larr; Back to Dashboard
          </button>
        </header>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Configuration Panel */}
          <div className="bg-white/80 dark:bg-[#1C1C1E]/80 backdrop-blur-xl rounded-2xl p-6 shadow-sm border border-white/20 dark:border-white/10 flex flex-col h-full">
            <div className="flex-1 overflow-y-auto pr-2 space-y-6">

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Poll Question</label>
                <input
                  type="text"
                  value={question}
                  onChange={(e) => setQuestion(e.target.value)}
                  className="w-full px-4 py-2.5 rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all outline-none"
                  placeholder="E.g., What should we build next?"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">Poll Options (Min 2, Max 4)</label>
                <div className="space-y-3">
                  {options.map((opt, i) => (
                    <div key={i} className="flex gap-2 items-center">
                      <input
                        type="text"
                        value={opt}
                        onChange={(e) => handleOptionChange(i, e.target.value)}
                        className="flex-1 px-4 py-2 rounded-xl border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 outline-none"
                        placeholder={`Option ${i + 1}`}
                      />
                      {options.length > 2 && (
                        <button
                          onClick={() => handleRemoveOption(i)}
                          className="p-2 text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
                          title="Remove option"
                        >
                          ✕
                        </button>
                      )}
                    </div>
                  ))}
                  {options.length < 4 && (
                    <button
                      onClick={handleAddOption}
                      className="text-sm text-blue-600 dark:text-blue-400 font-medium hover:underline flex items-center gap-1"
                    >
                      + Add Option
                    </button>
                  )}
                </div>
              </div>

              <div className="pt-4 border-t border-gray-100 dark:border-gray-800">
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">Widget Theme</label>
                <div className="flex gap-4">
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      name="theme"
                      value="light"
                      checked={theme === 'light'}
                      onChange={() => setTheme('light')}
                      className="text-blue-600 focus:ring-blue-500 h-4 w-4"
                    />
                    <span className="text-gray-700 dark:text-gray-300 text-sm">Light</span>
                  </label>
                  <label className="flex items-center gap-2 cursor-pointer">
                    <input
                      type="radio"
                      name="theme"
                      value="dark"
                      checked={theme === 'dark'}
                      onChange={() => setTheme('dark')}
                      className="text-blue-600 focus:ring-blue-500 h-4 w-4"
                    />
                    <span className="text-gray-700 dark:text-gray-300 text-sm">Dark</span>
                  </label>
                </div>
              </div>

              <div className="pt-4 border-t border-gray-100 dark:border-gray-800">
                <label className="flex items-center gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={requireEmail}
                    onChange={(e) => setRequireEmail(e.target.checked)}
                    className="rounded border-gray-300 text-blue-600 shadow-sm focus:border-blue-300 focus:ring focus:ring-blue-200 focus:ring-opacity-50 h-5 w-5"
                  />
                  <div>
                    <span className="block text-sm font-medium text-gray-900 dark:text-white">Require Email to Vote</span>
                    <span className="block text-xs text-gray-500 dark:text-gray-400">Capture leads before showing results</span>
                  </div>
                </label>
              </div>

              <div className="pt-4 border-t border-gray-100 dark:border-gray-800">
                <label className="flex items-center gap-3 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={removeBranding}
                    onChange={handleRemoveBrandingToggle}
                    className="rounded border-gray-300 text-blue-600 shadow-sm focus:border-blue-300 focus:ring focus:ring-blue-200 focus:ring-opacity-50 h-5 w-5"
                  />
                  <div>
                    <span className="block text-sm font-medium text-gray-900 dark:text-white flex items-center gap-2">
                      Remove OHC Branding {hasPro ? '' : <span className="bg-gradient-to-r from-amber-200 to-yellow-400 text-yellow-900 text-[10px] font-bold px-1.5 py-0.5 rounded-sm tracking-wider uppercase">Pro</span>}
                    </span>
                    <span className="block text-xs text-gray-500 dark:text-gray-400">Hide the &quot;Powered by OHC&quot; link</span>
                  </div>
                </label>
              </div>
            </div>

            <div className="pt-6 mt-auto">
              <button
                onClick={handleGenerate}
                className="w-full bg-[#0071E3] hover:bg-[#0077ED] text-white font-medium py-3 rounded-xl transition-all shadow-sm hover:shadow-md active:scale-[0.98]"
              >
                Generate Embed Code
              </button>
            </div>
          </div>

          {/* Live Preview Panel */}
          <div className="bg-gray-100 dark:bg-[#111111] rounded-2xl p-8 border border-gray-200 dark:border-gray-800 flex flex-col items-center justify-center relative overflow-hidden min-h-[500px]">
            <div className="absolute top-4 left-4 text-xs font-semibold tracking-wider text-gray-400 uppercase">Live Preview</div>

            <div className={`w-full max-w-sm rounded-2xl shadow-xl border ${previewBg} ${previewText} ${previewBorder} overflow-hidden transition-colors duration-300 p-6`}>
              <h3 className="text-xl font-bold mb-4 text-center">{question || 'Ask a question'}</h3>

              <div className="space-y-3 mb-6">
                {options.map((opt, i) => (
                  <button key={i} className={`w-full text-left px-4 py-3 rounded-xl border ${theme === 'dark' ? 'border-gray-700 hover:bg-gray-800' : 'border-gray-200 hover:bg-gray-50'} transition-colors flex items-center justify-between group`}>
                    <span className="font-medium">{opt || `Option ${i + 1}`}</span>
                    <div className={`w-4 h-4 rounded-full border-2 ${theme === 'dark' ? 'border-gray-600 group-hover:border-blue-400' : 'border-gray-300 group-hover:border-blue-500'} flex items-center justify-center`}></div>
                  </button>
                ))}
              </div>

              {requireEmail && (
                <div className="mb-4">
                  <input
                    type="email"
                    placeholder="Enter your email to vote"
                    className={`w-full px-4 py-2.5 rounded-xl border ${theme === 'dark' ? 'bg-gray-900 border-gray-700 text-white' : 'bg-gray-50 border-gray-200 text-gray-900'} outline-none focus:ring-2 focus:ring-blue-500 text-sm`}
                    readOnly
                  />
                </div>
              )}

              <button className="w-full bg-[#0071E3] hover:bg-[#0077ED] text-white font-medium py-2.5 rounded-xl transition-colors text-sm">
                Vote Now
              </button>
            </div>

            {!removeBranding && (
              <div className="mt-4">
                <PoweredByOHC tenantId={tenant} />
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Embed Code Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
          <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl max-w-2xl w-full p-6 shadow-2xl border border-white/10">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-xl font-bold text-gray-900 dark:text-white">Your Embed Code</h3>
              <button
                onClick={() => setShowModal(false)}
                className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
              >
                ✕
              </button>
            </div>
            <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
              Copy and paste this HTML into your website, blog, or store builder (like Shopify, Wix, or WordPress).
            </p>

            <div className="relative">
              <pre className="bg-gray-50 dark:bg-[#111111] p-4 rounded-xl text-sm font-mono text-gray-800 dark:text-gray-300 overflow-x-auto border border-gray-200 dark:border-gray-800">
                {embedCode}
              </pre>
              <button
                onClick={handleCopy}
                className="absolute top-3 right-3 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 shadow-sm rounded-lg px-3 py-1.5 text-sm font-medium text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
              >
                {copied ? 'Copied!' : 'Copy Code'}
              </button>
            </div>

            <div className="mt-6 flex justify-end">
              <button
                onClick={() => setShowModal(false)}
                className="bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-900 dark:text-white font-medium py-2 px-6 rounded-xl transition-colors"
              >
                Close
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-md">
          <div className="bg-white dark:bg-[#1C1C1E] rounded-3xl max-w-md w-full p-8 shadow-2xl border border-white/10 text-center relative overflow-hidden">
            <div className="absolute top-0 left-0 w-full h-1 bg-gradient-to-r from-blue-500 via-indigo-500 to-purple-500"></div>

            <div className="w-16 h-16 bg-blue-50 dark:bg-blue-900/30 rounded-full flex items-center justify-center mx-auto mb-6">
              <span className="text-3xl">✨</span>
            </div>

            <h3 className="text-2xl font-bold text-gray-900 dark:text-white mb-2 tracking-tight">Upgrade to Pro</h3>
            <p className="text-gray-600 dark:text-gray-400 mb-6">
              Removing the OHC branding from widgets is a Pro feature. Upgrade your workspace to unlock white-labeling and advanced analytics.
            </p>

            <div className="space-y-3">
              <button
                onClick={() => {
                  setShowSoftPaywall(false);
                  router.push('/plan');
                }}
                className="w-full bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-700 hover:to-indigo-700 text-white font-medium py-3 px-6 rounded-xl transition-all shadow-md hover:shadow-lg active:scale-[0.98]"
              >
                View Pro Plans
              </button>
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="w-full bg-transparent hover:bg-gray-50 dark:hover:bg-gray-800/50 text-gray-600 dark:text-gray-400 font-medium py-3 px-6 rounded-xl transition-colors"
              >
                Keep Branding for Now
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
