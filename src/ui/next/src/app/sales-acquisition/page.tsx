"use client";

import React, { useState } from 'react';
import Link from 'next/link';

export default function SalesAcquisitionPage() {
  const [autonomousQuoting, setAutonomousQuoting] = useState(false);
  const [basePrice, setBasePrice] = useState("");
  const [pricingRules, setPricingRules] = useState("");

  const handleSave = () => {
    alert("Saved successfully!");
  };

  return (
    <div className="min-h-screen bg-gray-50 p-8 font-inter">
      <div className="max-w-3xl mx-auto bg-white rounded-xl shadow p-8">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-3xl font-bold font-outfit text-gray-900">Sales & Acquisition</h1>
          <Link href="/dashboard" className="text-blue-600 hover:text-blue-800">Back to Dashboard</Link>
        </div>

        <section className="mb-8 border-b pb-8">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-semibold text-gray-800">Autonomous Quoting</h2>
            <label className="flex items-center cursor-pointer">
              <div className="relative">
                <input
                  type="checkbox"
                  className="sr-only"
                  checked={autonomousQuoting}
                  onChange={(e) => setAutonomousQuoting(e.target.checked)}
                />
                <div className={`block w-14 h-8 rounded-full ${autonomousQuoting ? 'bg-green-500' : 'bg-gray-300'} transition-colors`}></div>
                <div className={`dot absolute left-1 top-1 bg-white w-6 h-6 rounded-full transition-transform ${autonomousQuoting ? 'transform translate-x-6' : ''}`}></div>
              </div>
            </label>
          </div>
          <p className="text-sm text-gray-500 mb-4">
            Allow our AI to automatically respond to customer inquiries with a customized quote and calendar booking link.
          </p>

          {autonomousQuoting && (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">Base Pricing Rules</label>
                <textarea
                  className="w-full border border-gray-300 rounded-md p-2 text-gray-700"
                  rows={4}
                  placeholder="e.g. $50/hr base, plus materials"
                  value={pricingRules}
                  onChange={(e) => setPricingRules(e.target.value)}
                ></textarea>
              </div>
            </div>
          )}
        </section>

        <div className="flex justify-end">
          <button
            onClick={handleSave}
            className="bg-blue-600 text-white px-6 py-2 rounded-md hover:bg-blue-700 transition-colors font-medium"
          >
            Save Settings
          </button>
        </div>
      </div>
    </div>
  );
}
