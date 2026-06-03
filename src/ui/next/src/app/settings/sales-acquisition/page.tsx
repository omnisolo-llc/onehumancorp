"use client";
import React, { useState, useEffect } from 'react';
import Link from 'next/link';

export default function SalesAcquisitionSettings() {
  const [autoQuote, setAutoQuote] = useState(false);
  const [basePrice, setBasePrice] = useState('50');
  const [pricingRules, setPricingRules] = useState('Plus materials');
  const [isSaving, setIsSaving] = useState(false);
  const [saveMessage, setSaveMessage] = useState('');

  // Dummy fetch to populate existing config (optional but good practice)
  useEffect(() => {
     // Usually we would fetch the department settings here.
     // For this flow, we start with defaults.
  }, []);

  const handleSave = async () => {
    setIsSaving(true);
    setSaveMessage('');
    try {
      // Use the existing agent settings endpoint for the 'sales' department
      const res = await fetch('/api/agents/settings/sales', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            tone_of_voice: `Professional. Autonomous quoting enabled: ${autoQuote}. Base rate: $${basePrice}. Pricing rules: ${pricingRules}`,
            auto_approve_limits: 100.0 // arbitrary limit
        })
      });
      if (res.ok) {
        setSaveMessage('Settings saved successfully.');
        setTimeout(() => setSaveMessage(''), 3000);
      } else {
        setSaveMessage('Failed to save settings.');
      }
    } catch (e) {
      setSaveMessage('Network error occurred.');
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8 font-inter">
      <div className="max-w-3xl mx-auto mac-glass-container bg-white/70 backdrop-blur-[30px] rounded-[24px] shadow-2xl p-8 border border-white/40">
        <div className="flex justify-between items-center mb-8">
          <div className="flex items-center gap-3">
            <div className="w-12 h-12 bg-blue-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-blue-600">
               💼
            </div>
            <h1 className="text-3xl font-bold font-outfit text-gray-900 tracking-tight">Sales & Acquisition</h1>
          </div>
          <Link href="/settings" className="text-sm font-semibold text-gray-500 hover:text-gray-900 transition-colors bg-white/50 px-4 py-2 rounded-lg border border-gray-200">Back to Settings</Link>
        </div>

        <section className="mb-8">
          <div className="bg-white/50 p-6 rounded-2xl border border-gray-100 shadow-sm">
              <h2 className="text-xl font-bold font-outfit mb-2 text-gray-800 flex items-center gap-2">
                 <span className="text-[#0066FF]">✨</span> Autonomous Quoting
              </h2>
              <p className="text-sm text-gray-600 mb-6 leading-relaxed max-w-lg">
                 Let the <strong className="text-gray-900">Sales Agent</strong> automatically generate professional quotes and calendar booking links when a customer submits an inquiry. The agent will read their request and apply your pricing rules.
              </p>

              <div className="space-y-6">
                <label className="flex items-center gap-3 text-gray-900 font-semibold cursor-pointer group">
                  <div className={`relative w-12 h-6 rounded-full transition-colors duration-300 ${autoQuote ? 'bg-[#34C759]' : 'bg-gray-300'}`}>
                     <input
                       type="checkbox"
                       checked={autoQuote}
                       onChange={(e) => setAutoQuote(e.target.checked)}
                       className="opacity-0 w-0 h-0"
                     />
                     <span className={`absolute top-1 left-1 bg-white w-4 h-4 rounded-full transition-transform duration-300 shadow-sm ${autoQuote ? 'translate-x-6' : 'translate-x-0'}`}></span>
                  </div>
                  Enable Autonomous Quoting
                </label>

                <div className={`transition-all duration-500 overflow-hidden ${autoQuote ? 'max-h-[500px] opacity-100' : 'max-h-0 opacity-0'}`}>
                  <div className="p-5 bg-white/80 border border-blue-100 rounded-xl shadow-inner space-y-5">
                    <div>
                      <label className="block text-xs font-bold text-gray-700 uppercase tracking-wider mb-2">Base Hourly Rate ($)</label>
                      <div className="relative max-w-xs">
                          <span className="absolute left-4 top-3 text-gray-400 font-medium">$</span>
                          <input
                            type="number"
                            value={basePrice}
                            onChange={(e) => setBasePrice(e.target.value)}
                            className="w-full bg-white border border-gray-200 rounded-xl pl-8 pr-4 py-3 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all"
                          />
                      </div>
                    </div>
                    <div>
                      <label className="block text-xs font-bold text-gray-700 uppercase tracking-wider mb-2">Additional Pricing Rules</label>
                      <textarea
                        value={pricingRules}
                        onChange={(e) => setPricingRules(e.target.value)}
                        placeholder="e.g. Plus materials, $20 travel fee..."
                        className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-[#0066FF] transition-all h-24 resize-none"
                      />
                      <p className="text-[11px] text-gray-500 mt-2 font-medium">The Sales Agent will use these rules to generate the final quote.</p>
                    </div>
                  </div>
                </div>
              </div>
          </div>
        </section>

        <div className="flex items-center gap-4 pt-4 border-t border-gray-200/50">
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="bg-[#0066FF] hover:bg-blue-700 text-white font-bold py-3 px-8 rounded-xl shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all disabled:opacity-70 flex items-center justify-center min-w-[140px]"
          >
            {isSaving ? 'Saving...' : 'Save Settings'}
          </button>
          {saveMessage && (
              <span className={`text-sm font-medium ${saveMessage.includes('success') ? 'text-[#34C759]' : 'text-red-500'}`}>
                  {saveMessage}
              </span>
          )}
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .mac-glass-container {
            backdrop-filter: blur(30px) saturate(210%);
        }
      `}} />
    </div>
  );
}
