"use client";

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { AppShell } from '../components/AppShell';

export default function WorkIntakeWidgetPage() {
  const router = useRouter();
  const [tenantId, setTenantId] = useState('demo-tenant');
  const [buttonColor, setButtonColor] = useState('#0066FF');
  const [formTitle, setFormTitle] = useState('Contact Us');
  const [theme, setTheme] = useState('light');
  const [removeBranding, setRemoveBranding] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);

  const handleRemoveBrandingClick = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.checked) {
      setShowSoftPaywall(true);
      setRemoveBranding(false); // They must upgrade to check it
    }
  };

  const embedCode = `<!-- Powered by OHC -->
<div id="ohc-work-intake-widget" style="width: 100%; max-width: 400px; margin: 0 auto; filter: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))">
  <iframe
      src="https://ohc.app/api/v1/growth/work-intake/embed?tenant=${tenantId}&theme=${theme}&title=${encodeURIComponent(formTitle)}&color=${encodeURIComponent(buttonColor)}"
      width="320"
      height="400"
      frameBorder="0"
      scrolling="no"
      style="border: none; border-radius: 16px; background-color: transparent; width: 100%;"
  ></iframe>
  ${!removeBranding ? `<div style="font-family: sans-serif; text-align: center; font-size: 12px; margin-top: 8px;">
      <a href="https://ohc.app/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}" target="_blank" rel="noreferrer" style="color: #6b7280; text-decoration: none; font-weight: 600;">⚡ Powered by OHC</a>
  </div>` : ''}
</div>`;

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <AppShell title="Work Intake Widget" subtitle="Generate an embeddable contact form to capture leads directly into your triage inbox.">
      <div className="max-w-6xl mx-auto space-y-6 lg:space-y-0 lg:flex lg:gap-8">
        {/* Configuration Panel */}
        <div className="flex-1 space-y-6">
          <div className="bg-white/80 dark:bg-black/40 backdrop-blur-md rounded-2xl p-6 border border-gray-200 dark:border-gray-800 shadow-sm">
            <h2 className="text-xl font-semibold mb-4 text-gray-900 dark:text-white">Widget Configuration</h2>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Tenant ID (For routing)
                </label>
                <input
                  type="text"
                  value={tenantId}
                  onChange={(e) => setTenantId(e.target.value)}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-transparent"
                  data-testid="input-tenant-id"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Form Title
                </label>
                <input
                  type="text"
                  value={formTitle}
                  onChange={(e) => setFormTitle(e.target.value)}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-transparent"
                  data-testid="input-form-title"
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Theme
                </label>
                <select
                  value={theme}
                  onChange={(e) => setTheme(e.target.value)}
                  className="w-full p-2 border border-gray-300 dark:border-gray-700 rounded-lg bg-transparent"
                  data-testid="select-theme"
                >
                  <option value="light">Light</option>
                  <option value="dark">Dark</option>
                  <option value="auto">Auto (System)</option>
                </select>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  Button Color
                </label>
                <div className="flex items-center gap-3">
                  <input
                    type="color"
                    value={buttonColor}
                    onChange={(e) => setButtonColor(e.target.value)}
                    className="w-10 h-10 rounded cursor-pointer"
                    data-testid="input-button-color"
                  />
                  <span className="text-sm text-gray-500">{buttonColor}</span>
                </div>
              </div>

              <div className="pt-4 border-t border-gray-200 dark:border-gray-800">
                <label className="flex items-center gap-3 cursor-pointer group">
                  <input
                    type="checkbox"
                    checked={removeBranding}
                    onChange={handleRemoveBrandingClick}
                    className="w-5 h-5 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600 cursor-pointer"
                    data-testid="input-remove-branding"
                  />
                  <span className="text-sm font-medium text-gray-700 dark:text-gray-300 group-hover:text-indigo-600 transition-colors">
                    Remove "Powered by OHC" watermark
                  </span>
                  <span className="ml-auto text-xs font-bold text-indigo-600 bg-indigo-50 px-2 py-1 rounded-full uppercase tracking-wider">
                    Pro
                  </span>
                </label>
              </div>
            </div>
          </div>

          <div className="bg-white/80 dark:bg-black/40 backdrop-blur-md rounded-2xl p-6 border border-gray-200 dark:border-gray-800 shadow-sm">
             <button
               onClick={() => setShowModal(true)}
               className="w-full py-4 min-h-[44px] min-w-[44px] font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600 hover:bg-indigo-700 rounded-lg"
               data-testid="btn-get-code"
             >
               Get Embed Code
             </button>
          </div>
        </div>

        {/* Preview Panel */}
        <div className="flex-1">
          <div className="bg-white/80 dark:bg-black/40 backdrop-blur-md rounded-2xl p-6 border border-gray-200 dark:border-gray-800 shadow-sm h-full min-h-[500px]">
            <h2 className="text-xl font-semibold mb-6 text-gray-900 dark:text-white">Live Preview</h2>

            <div className="flex justify-center w-full" data-testid="widget-preview">
               <div style={{ width: '100%', maxWidth: '400px', margin: '0 auto', filter: 'drop-shadow(0 25px 25px rgb(0 0 0 / 0.15))' }}>
                  {/* Mock iframe representation for preview */}
                  <div style={{ width: '100%', height: '400px', border: '1px solid #e5e7eb', borderRadius: '16px', backgroundColor: theme === 'dark' ? '#1f2937' : '#ffffff', overflow: 'hidden', display: 'flex', flexDirection: 'column' }}>
                      <div style={{ padding: '20px', flex: 1 }}>
                          <h3 style={{ fontSize: '1.25rem', fontWeight: 600, marginBottom: '1rem', color: theme === 'dark' ? '#ffffff' : '#111827' }}>{formTitle}</h3>
                          <div style={{ width: '100%', height: '38px', marginBottom: '1rem', border: '1px solid #d1d5db', borderRadius: '6px' }}></div>
                          <div style={{ width: '100%', height: '38px', marginBottom: '1rem', border: '1px solid #d1d5db', borderRadius: '6px' }}></div>
                          <div style={{ width: '100%', height: '80px', marginBottom: '1rem', border: '1px solid #d1d5db', borderRadius: '6px' }}></div>
                          <button style={{ width: '100%', padding: '0.75rem', backgroundColor: buttonColor, color: 'white', border: 'none', borderRadius: '6px', fontWeight: 500 }}>
                            Submit Request
                          </button>
                      </div>
                  </div>
                  {!removeBranding && (
                      <div style={{ fontFamily: 'sans-serif', textAlign: 'center', fontSize: '12px', marginTop: '8px' }}>
                          <span style={{ color: '#6b7280', textDecoration: 'none', fontWeight: 600 }}>⚡ Powered by OHC</span>
                      </div>
                  )}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[9999] flex items-center justify-center p-4" data-testid="paywall-modal">
          <div className="bg-white w-full max-w-md p-8 shadow-2xl relative overflow-hidden font-inter border border-indigo-100 text-center rounded-2xl">
            <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
                data-testid="btn-close-paywall"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-indigo-100 flex items-center justify-center text-3xl rounded-full text-indigo-600 mx-auto mb-6">
              ✨
            </div>
            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Make the Work Intake Widget 100% yours. Upgrade to Pro to remove the "Powered by OHC" watermark.
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 min-h-[44px] min-w-[44px] font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90 bg-indigo-600 hover:bg-indigo-700 rounded-lg"
              data-testid="btn-upgrade-pro"
            >
              Upgrade to Pro
            </button>
          </div>
        </div>
      )}

      {/* Embed Code Modal */}
      {showModal && (
        <div className="fixed inset-0 z-[100] flex items-center justify-center p-4">
            <div className="absolute inset-0 bg-black/40 backdrop-blur-[30px] saturate-[210%]" onClick={() => setShowModal(false)}></div>
            <div className="bg-white dark:bg-gray-900 rounded-2xl shadow-2xl p-8 max-w-xl w-full relative z-10 animate-fade-in-up border border-gray-200 dark:border-gray-800">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900 dark:text-white">Embed Work-Intake Widget</h2>
                <p className="text-gray-600 dark:text-gray-400 mb-6">Copy and paste this HTML snippet into your website to capture leads instantly.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-48 p-4 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 min-h-[44px] min-w-[44px] font-mono text-sm text-gray-800 dark:text-gray-300 resize-none rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF]/20 focus:border-[#0066FF] transition-all"
                        data-testid="embed-code-block"
                    />
                    <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                         <button
                            onClick={handleCopy}
                            className="p-2 bg-white dark:bg-gray-700 rounded-lg border dark:border-gray-600 shadow-sm text-gray-600 dark:text-gray-300 hover:text-[#0071E3] transition-colors"
                            title="Copy to clipboard"
                            data-testid="btn-copy-code-icon"
                        >
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2 2h-8a2 2 0 00-2 2v8a2 2 0 002 2z"></path></svg>
                        </button>
                    </div>
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 rounded-lg bg-[#0071E3] hover:bg-blue-700 text-white font-medium min-h-[44px] min-w-[44px] transition-colors shadow-sm flex items-center justify-center gap-2"
                        data-testid="btn-copy-code"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 rounded-lg bg-gray-100 dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700 text-gray-800 dark:text-white font-medium min-h-[44px] min-w-[44px] transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}
    </AppShell>
  );
}
