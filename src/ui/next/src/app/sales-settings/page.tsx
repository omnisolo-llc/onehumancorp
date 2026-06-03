"use client";
import React, { useState, useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';

export default function SalesSettingsPage() {
  const router = useRouter();
  const [autonomousQuoting, setAutonomousQuoting] = useState(false);
  const [basePricingRules, setBasePricingRules] = useState('');
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    fetch('/api/v1/sales-settings')
      .then(res => res.json())
      .then(data => {
        if (data.settings) {
          setAutonomousQuoting(data.settings.autonomousQuoting);
          setBasePricingRules(data.settings.basePricingRules);
        }
      });
  }, []);

  const handleSave = async () => {
    setIsSaving(true);
    try {
      await fetch('/api/v1/sales-settings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ autonomousQuoting, basePricingRules })
      });
      router.push('/dashboard');
    } catch (e) {
      console.error(e);
      alert('Failed to save settings');
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex justify-center font-inter">
      <div className="w-full max-w-[375px] bg-white min-h-screen shadow-xl relative overflow-x-hidden flex flex-col p-6">
        <div className="flex justify-between items-center mb-6 mt-8">
          <h1 className="text-2xl font-bold font-outfit text-gray-900">Sales & Acquisition</h1>
          <Link href="/dashboard" className="text-sm text-blue-600 hover:text-blue-800">Back</Link>
        </div>

        <section className="mb-8">
          <h2 className="text-lg font-semibold mb-2 text-gray-800">Auto-Quote & Book</h2>
          <p className="text-sm text-gray-500 mb-4">Let AI automatically generate quotes based on predefined rules, check availability, and send booking links to customers without manual intervention.</p>

          <div className="flex items-center gap-3 mb-6">
            <button
              onClick={() => setAutonomousQuoting(!autonomousQuoting)}
              className={`w-12 h-6 rounded-full p-1 transition-colors ${autonomousQuoting ? 'bg-blue-600' : 'bg-gray-300'}`}
            >
              <div className={`bg-white w-4 h-4 rounded-full shadow-md transform transition-transform ${autonomousQuoting ? 'translate-x-6' : ''}`}></div>
            </button>
            <span className="text-sm font-medium text-gray-700">Autonomous Quoting</span>
          </div>

          {autonomousQuoting && (
            <div className="space-y-4 animate-in fade-in slide-in-from-top-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Base Pricing Rules</label>
                <textarea
                  value={basePricingRules}
                  onChange={(e) => setBasePricingRules(e.target.value)}
                  placeholder="e.g., $50/hr base, plus materials"
                  className="w-full h-32 p-3 border rounded-lg text-gray-700 text-sm focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none resize-none"
                />
              </div>
            </div>
          )}
        </section>

        <div className="mt-auto pt-4 pb-8">
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-4 rounded-xl shadow-sm transition-colors"
          >
            {isSaving ? 'Saving...' : 'Save Settings'}
          </button>
        </div>
      </div>
    </div>
  );
}
