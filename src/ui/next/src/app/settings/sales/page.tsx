"use client";
import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function SalesSettingsPage() {
  const router = useRouter();

  const [isAutonomousQuotingEnabled, setIsAutonomousQuotingEnabled] = useState(false);
  const [basePricingRules, setBasePricingRules] = useState('');
  const [saved, setSaved] = useState(false);

  useEffect(() => {
    async function fetchSettings() {
      try {
        const tenantId = typeof window !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
        const res = await fetch(`/api/settings/sales-proxy`, {
          headers: { 'x-tenant-id': tenantId }
        });
        if (res.ok) {
          const data = await res.json();
          setIsAutonomousQuotingEnabled(data.autonomous_quoting_enabled || false);
          setBasePricingRules(data.base_pricing_rules || '');
        }
      } catch (e) {
        console.error("Failed to load settings:", e);
      }
    }
    fetchSettings();
  }, []);

  const handleSave = async () => {
    try {
      const tenantId = typeof window !== 'undefined' ? localStorage.getItem('tenant') || 'my-store' : 'my-store';
      await fetch('/api/settings/sales-proxy', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'x-tenant-id': tenantId
        },
        body: JSON.stringify({
          autonomous_quoting_enabled: isAutonomousQuotingEnabled,
          base_pricing_rules: basePricingRules
        })
      });
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (e) {
      console.error("Failed to save settings:", e);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8 font-inter">
      <div className="max-w-3xl mx-auto bg-white rounded-xl shadow p-8">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-3xl font-bold font-outfit text-gray-900">Sales & Acquisition</h1>
          <Link href="/settings" className="text-blue-600 hover:text-blue-800">Back to Settings</Link>
        </div>

        {/* Autonomous Quoting */}
        <section className="mb-8 border-b pb-8">
          <h2 className="text-xl font-semibold mb-2 text-gray-800">Autonomous Quoting</h2>
          <p className="text-sm text-gray-500 mb-4">
            Automatically generate quotes and send booking links to customers based on your pricing rules.
          </p>

          <div className="space-y-6">
            <label className="flex items-center gap-3 cursor-pointer">
              <div className="relative">
                <input
                  type="checkbox"
                  className="sr-only"
                  checked={isAutonomousQuotingEnabled}
                  onChange={(e) => setIsAutonomousQuotingEnabled(e.target.checked)}
                />
                <div className={`block w-14 h-8 rounded-full transition-colors ${isAutonomousQuotingEnabled ? 'bg-blue-600' : 'bg-gray-300'}`}></div>
                <div className={`dot absolute left-1 top-1 bg-white w-6 h-6 rounded-full transition-transform ${isAutonomousQuotingEnabled ? 'transform translate-x-6' : ''}`}></div>
              </div>
              <span className="text-gray-700 font-medium text-sm">
                Enable Autonomous Quoting
              </span>
            </label>

            {isAutonomousQuotingEnabled && (
              <div className="bg-blue-50 border border-blue-100 p-4 rounded-xl">
                <label className="block text-sm font-semibold text-gray-900 mb-2">Base Pricing Rules</label>
                <p className="text-xs text-gray-500 mb-3">
                  Describe how you price your services. Our AI will use this to generate accurate quotes for customer requests.
                </p>
                <textarea
                  value={basePricingRules}
                  onChange={(e) => setBasePricingRules(e.target.value)}
                  placeholder="e.g., $50/hr base rate, plus materials. Minimum 2 hours for any job."
                  className="w-full min-h-[100px] border border-gray-200 rounded-lg p-3 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
                />
              </div>
            )}
          </div>
        </section>

        <div className="flex items-center gap-4 pt-4">
          <button
            onClick={handleSave}
            className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-8 rounded-lg shadow-sm transition-all"
          >
            {saved ? "Saved!" : "Save"}
          </button>
        </div>
      </div>
    </div>
  );
}
