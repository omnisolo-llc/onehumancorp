'use client';

import React, { useState, useEffect } from 'react';
import { AppShell } from "@/app/components/AppShell";
const TopNav = ({ title }: { title: string }) => <div className="font-semibold px-4 py-3 border-b border-[#E5E5EA] dark:border-[#38383A]">{title}</div>;



const fetchJson = async (url: string) => { return { tenant: { base_currency: 'USD', enabled_currencies: ['USD', 'EUR'] } }; };
const putJson = async (url: string, data: any) => { return {}; };

export default function GlobalCommerceSettings() {
  const [baseCurrency, setBaseCurrency] = useState('USD');
  const [enabledCurrencies, setEnabledCurrencies] = useState<string[]>(['USD']);
  const [isLoading, setIsLoading] = useState(true);
  const [isSaving, setIsSaving] = useState(false);

  const availableCurrencies = ['USD', 'EUR', 'GBP', 'CAD', 'AUD', 'JPY'];

  useEffect(() => {
    async function loadSettings() {
      try {
        const res = await fetchJson('/api/v1/settings');
        if (res.tenant) {
          setBaseCurrency(res.tenant.base_currency || 'USD');
          setEnabledCurrencies(res.tenant.enabled_currencies || ['USD']);
        }
      } catch (e) {
        console.error('Failed to load settings', e);
      } finally {
        setIsLoading(false);
      }
    }
    loadSettings();
  }, []);

  const handleSave = async () => {
    setIsSaving(true);
    try {
      await putJson('/api/v1/settings', {
        base_currency: baseCurrency,
        enabled_currencies: enabledCurrencies,
      });
      alert('Settings saved successfully');
    } catch (e) {
      console.error('Failed to save settings', e);
      alert('Failed to save settings');
    } finally {
      setIsSaving(false);
    }
  };

  const toggleCurrency = (currency: string) => {
    if (enabledCurrencies.includes(currency)) {
      if (currency === baseCurrency) return; // Cannot disable base currency
      setEnabledCurrencies(enabledCurrencies.filter(c => c !== currency));
    } else {
      setEnabledCurrencies([...enabledCurrencies, currency]);
    }
  };

  if (isLoading) {
    return (
      <AppShell>
        <div className="flex flex-col h-full bg-white dark:bg-[#1C1C1E]">
          <TopNav title="Global Commerce" />
          <div className="flex-1 p-4 flex items-center justify-center">
            <div className="text-sm text-gray-500">Loading settings...</div>
          </div>
        </div>
      </AppShell>
    );
  }

  return (
    <AppShell>
      <div className="flex flex-col h-full bg-[#F2F2F7] dark:bg-black overflow-y-auto">
        <TopNav title="Global Commerce" />

        <div className="px-4 py-6 max-w-lg mx-auto w-full">
          <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl p-4 shadow-sm mb-6 border border-[#E5E5EA] dark:border-[#38383A]">
            <h2 className="text-[17px] font-semibold text-black dark:text-white mb-1">Base Currency</h2>
            <p className="text-[13px] text-[#8E8E93] mb-4">This is the default currency for your store and reporting.</p>

            <select
              value={baseCurrency}
              onChange={(e) => {
                setBaseCurrency(e.target.value);
                if (!enabledCurrencies.includes(e.target.value)) {
                  setEnabledCurrencies([...enabledCurrencies, e.target.value]);
                }
              }}
              className="w-full bg-[#F2F2F7] dark:bg-[#2C2C2E] border border-transparent rounded-xl px-4 py-3 text-[15px] text-black dark:text-white focus:ring-2 focus:ring-blue-500 outline-none"
            >
              {availableCurrencies.map(c => (
                <option key={c} value={c}>{c}</option>
              ))}
            </select>
          </div>

          <div className="bg-white dark:bg-[#1C1C1E] rounded-2xl p-4 shadow-sm mb-6 border border-[#E5E5EA] dark:border-[#38383A]">
            <h2 className="text-[17px] font-semibold text-black dark:text-white mb-1">Enabled Currencies</h2>
            <p className="text-[13px] text-[#8E8E93] mb-4">Customers will see prices and checkout in these currencies automatically based on their location.</p>

            <div className="space-y-3">
              {availableCurrencies.map(currency => (
                <div key={currency} className="flex items-center justify-between">
                  <span className="text-[15px] text-black dark:text-white">{currency}</span>
                  <button
                    onClick={() => toggleCurrency(currency)}
                    disabled={currency === baseCurrency}
                    className={`relative inline-flex h-7 w-12 items-center rounded-full transition-colors duration-200 ease-in-out focus:outline-none ${
                      enabledCurrencies.includes(currency) ? 'bg-[#34C759]' : 'bg-[#E5E5EA] dark:bg-[#38383A]'
                    } ${currency === baseCurrency ? 'opacity-50 cursor-not-allowed' : ''}`}
                  >
                    <span
                      className={`inline-block h-6 w-6 transform rounded-full bg-white shadow transition duration-200 ease-in-out ${
                        enabledCurrencies.includes(currency) ? 'translate-x-5' : 'translate-x-1'
                      }`}
                    />
                  </button>
                </div>
              ))}
            </div>
          </div>

          <button
            onClick={handleSave}
            disabled={isSaving}
            className="w-full bg-[#007AFF] hover:bg-[#0056b3] text-white rounded-xl py-3.5 font-semibold text-[17px] transition-colors disabled:opacity-50"
          >
            {isSaving ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </div>
    </AppShell>
  );
}
