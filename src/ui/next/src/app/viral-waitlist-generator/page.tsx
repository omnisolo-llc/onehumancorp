"use client";

import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import { PoweredByOHC } from '../components/PoweredByOHC';

export default function ViralWaitlistGeneratorPage() {
  const router = useRouter();
  const [productName, setProductName] = useState('My Awesome New Service');
  const [description, setDescription] = useState('Join the waitlist to get early access and special perks!');
  const [showModal, setShowModal] = useState(false);
  const [copied, setCopied] = useState(false);
  const [tenant, setTenant] = useState('DEFAULT');
  const [hasPro, setHasPro] = useState(false);
  const [showSoftPaywall, setShowSoftPaywall] = useState(false);
  const [embedCode, setEmbedCode] = useState('');

  useEffect(() => {
    if (typeof window !== 'undefined') {
      setTenant(localStorage.getItem('tenant_id') || 'DEFAULT');
      setHasPro(localStorage.getItem('has_pro') === 'true');
    }
  }, []);

  const handleGenerate = () => {
    let code = `<!-- OHC Waitlist Widget -->
<div id="ohc-waitlist-widget"></div>
<script>
  (function() {
    const container = document.getElementById('ohc-waitlist-widget');
    container.innerHTML = \`
      <div style="font-family: sans-serif; max-width: 400px; margin: 0 auto; text-align: center; padding: 24px; border: 1px solid #e5e7eb; border-radius: 12px; background: #fff; box-shadow: 0 4px 6px -1px rgba(0,0,0,0.1);">
        <h3 style="margin-top: 0; color: #111827; font-size: 20px; font-weight: bold;">${productName.replace(/"/g, '&quot;')}</h3>
        <p style="color: #6b7280; font-size: 14px; margin-bottom: 20px;">${description.replace(/"/g, '&quot;')}</p>
        <form action="https://ohc.app/api/v1/growth/waitlist" method="POST" style="display: flex; gap: 8px;">
          <input type="hidden" name="tenant" value="${tenant}" />
          <input type="email" name="email" required placeholder="Enter your email" style="flex: 1; padding: 10px; border: 1px solid #d1d5db; border-radius: 6px; box-sizing: border-box;" />
          <button type="submit" style="background: #0066ff; color: white; border: none; padding: 10px 16px; border-radius: 6px; font-weight: bold; cursor: pointer;">Join</button>
        </form>
        <p style="color: #9ca3af; font-size: 12px; margin-top: 12px; margin-bottom: 0;">Join 1,234 others on the waitlist</p>\`;`;

    if (!hasPro) {
      code += `
    container.innerHTML += \`
        <div style="margin-top: 16px; font-size: 12px;">
          <a href="https://ohc.app/onboarding?ref=${tenant}&source=waitlist_widget" target="_blank" style="color: #9ca3af; text-decoration: none;">⚡ Powered by OHC</a>
        </div>\`;`;
    }

    code += `
      container.innerHTML += \`
      </div>
    \`;

    // Add submit handler to use fetch instead of full page redirect if possible
    const form = container.querySelector('form');
    if (form) {
      form.addEventListener('submit', function(e) {
        e.preventDefault();
        const btn = form.querySelector('button');
        const originalText = btn.innerText;
        btn.innerText = 'Joining...';
        btn.disabled = true;

        fetch(form.action, {
          method: 'POST',
          body: new FormData(form)
        }).then(() => {
          btn.innerText = 'Joined!';
          btn.style.background = '#10b981';
          form.querySelector('input[type="email"]').value = '';
        }).catch(() => {
          btn.innerText = 'Error';
          btn.style.background = '#ef4444';
        }).finally(() => {
          setTimeout(() => {
            btn.innerText = originalText;
            btn.disabled = false;
            btn.style.background = '#0066ff';
          }, 3000);
        });
      });
    }
  })();
</script>`;

    setEmbedCode(code);
    setShowModal(true);
    setCopied(false);
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(embedCode).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  };

  const handleToggleBranding = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!hasPro) {
      e.preventDefault();
      setShowSoftPaywall(true);
      return;
    }
  };

  return (
    <div className="min-h-screen flex flex-col font-inter bg-gradient-to-br from-indigo-50 via-purple-50 to-pink-50 items-center justify-center py-10 px-4">
      <div className="w-full max-w-5xl bg-white/80 backdrop-blur-xl rounded-[24px] shadow-sm border border-gray-100 flex flex-col lg:flex-row gap-8 relative z-10">
        <div className="flex-1 p-8 space-y-6">
            <div>
                <button onClick={() => router.push('/dashboard')} className="mb-4 px-4 py-2 bg-white rounded-lg text-sm font-medium hover:bg-gray-50 border border-gray-200 transition-colors shadow-sm text-gray-700">
                  &larr; Back to Dashboard
                </button>
                <h1 className="text-3xl font-bold font-outfit text-gray-900 mb-2">Viral Waitlist Generator 🚀</h1>
                <p className="text-gray-600 mb-8 text-sm">Create a viral waitlist widget for your new service or product.</p>
            </div>

            <div className="space-y-4">
                <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Product/Service Name</label>
                    <input
                        type="text"
                        value={productName}
                        onChange={(e) => setProductName(e.target.value)}
                        className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                        placeholder="My Awesome New Service"
                    />
                </div>
                <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1">Description</label>
                    <textarea
                        value={description}
                        onChange={(e) => setDescription(e.target.value)}
                        rows={3}
                        className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                        placeholder="Join the waitlist to get early access..."
                    />
                </div>

                <div className="flex items-center justify-between p-4 bg-gray-50 rounded-xl border border-gray-200 mt-4">
                     <div>
                        <p className="font-medium text-gray-900 text-sm">Remove OHC Branding</p>
                        <p className="text-xs text-gray-500">Requires Pro plan</p>
                     </div>
                     <div className="relative inline-flex items-center">
                        <input
                            type="checkbox"
                            checked={hasPro}
                            onChange={handleToggleBranding}
                            className="w-10 h-5 bg-gray-300 rounded-full appearance-none checked:bg-[#0066FF] transition-colors cursor-pointer"
                            aria-label="Toggle Branding"
                        />
                        <div className={`absolute w-4 h-4 bg-white rounded-full shadow top-0.5 left-0.5 transition-transform ${hasPro ? 'translate-x-5' : ''} pointer-events-none`}></div>
                     </div>
                </div>
            </div>

            <div className="mt-8">
                <button
                    onClick={handleGenerate}
                    className="w-full py-3 bg-[#0071E3] hover:bg-blue-700 text-white font-medium rounded-xl transition-colors shadow-sm"
                >
                    Generate Widget
                </button>
            </div>
        </div>

        <div className="flex-1 flex flex-col p-8 bg-gray-50/50 rounded-r-[24px]">
           <h2 className="text-xl font-semibold font-outfit text-gray-900 mb-4">Preview</h2>
           <div className="flex-1 rounded-2xl shadow-inner border-2 border-dashed border-gray-300 relative overflow-hidden flex items-center justify-center p-6 min-h-[400px] flex-col">
                <div className="w-full max-w-sm p-6 rounded-xl text-center bg-white shadow-sm border border-gray-200 relative z-10">
                    <h3 className="text-xl font-bold text-gray-900 mb-2">{productName}</h3>
                    <p className="text-sm text-gray-600 mb-4">{description}</p>
                    <div className="flex gap-2">
                        <input type="email" placeholder="Enter your email" className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-indigo-500 text-black" disabled />
                        <button className="px-4 py-2 bg-indigo-600 text-white font-bold rounded-lg shadow-sm whitespace-nowrap" disabled>Join</button>
                    </div>
                    <p className="text-xs text-gray-400 mt-3">Join 1,234 others on the waitlist</p>
                </div>

                {!hasPro && (
                    <div className="mt-6 text-center relative z-10" style={{ fontFamily: 'sans-serif', fontSize: '12px' }}>
                        <PoweredByOHC tenantId={tenant} />
                    </div>
                )}
           </div>
        </div>
      </div>

      {/* Embed Modal */}
      {showModal && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-[30px] saturate-[210%]">
            <div className="app-card p-8 max-w-xl w-full shadow-2xl relative animate-in fade-in bg-white rounded-2xl">
                <button
                    aria-label="Close embed modal"
                    onClick={() => setShowModal(false)}
                    className="absolute top-6 right-6 text-gray-400 hover:text-gray-600 transition-colors"
                >
                    <svg className="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>

                <h2 className="text-2xl font-bold font-outfit mb-2 text-gray-900">Embed Waitlist</h2>
                <p className="text-gray-600 mb-6">Copy and paste this HTML snippet into your website.</p>

                <div className="relative group">
                    <textarea
                        readOnly
                        value={embedCode}
                        className="w-full h-40 p-4 bg-gray-50 border border-gray-200 font-mono text-sm text-gray-800 resize-none focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all rounded-xl"
                    />
                </div>

                <div className="mt-6 flex flex-col sm:flex-row gap-3">
                    <button
                        onClick={handleCopy}
                        className="flex-1 py-3 bg-[#0071E3] hover:bg-blue-700 text-white font-medium rounded-xl transition-colors shadow-sm flex items-center justify-center gap-2"
                    >
                        {copied ? 'Copied!' : 'Copy Code'}
                    </button>
                    <button
                        onClick={() => setShowModal(false)}
                        className="flex-1 py-3 bg-gray-100 hover:bg-gray-200 text-gray-800 font-medium rounded-xl transition-colors"
                    >
                        Close
                    </button>
                </div>
            </div>
        </div>
      )}

      {/* Soft Paywall Modal */}
      {showSoftPaywall && (
        <div className="fixed inset-0 bg-black/60 z-[60] flex items-center justify-center p-4">
          <div className="app-card w-full max-w-md rounded-2xl p-8 shadow-2xl relative overflow-hidden font-inter border border-blue-100 text-center bg-white">
            <div className="absolute top-0 right-0 w-32 h-32 bg-blue-50 rounded-bl-full -z-10"></div>

            <div className="flex justify-end mb-2">
              <button
                aria-label="Close paywall"
                onClick={() => setShowSoftPaywall(false)}
                className="text-gray-400 hover:text-gray-600 rounded-full hover:bg-gray-100 transition-colors w-8 h-8 flex items-center justify-center"
              >
                <span className="text-xl leading-none">&times;</span>
              </button>
            </div>

            <div className="w-16 h-16 bg-gradient-to-br from-[#0066FF] to-indigo-600 rounded-2xl flex items-center justify-center text-3xl shadow-lg mx-auto mb-6 text-white font-bold">
              PRO
            </div>

            <h2 className="text-2xl font-bold font-outfit text-gray-900 mb-3">Upgrade to Pro</h2>
            <p className="text-gray-600 mb-6 text-sm leading-relaxed">
              Remove OHC branding and make the waitlist widget fully yours. Upgrade to Pro today!
            </p>

            <button
              onClick={() => { setShowSoftPaywall(false); router.push('/pricing'); }}
              className="w-full py-4 rounded-xl font-bold text-white mb-4 transition-all shadow-md hover:shadow-lg hover:opacity-90"
              style={{ background: 'linear-gradient(135deg, #0066ff 0%, #3b82f6 100%)' }}
            >
              Upgrade to Pro
            </button>

            <button
              onClick={() => setShowSoftPaywall(false)}
              className="mt-2 text-gray-500 hover:text-gray-700 font-medium text-sm w-full"
            >
              Maybe Later
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
