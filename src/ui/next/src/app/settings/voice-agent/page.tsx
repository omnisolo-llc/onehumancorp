'use client';

import React, { useEffect, useState } from 'react';
import Link from 'next/link';
import { useVoiceAgentStore } from './store';

export default function VoiceAgentSettings() {
  const [mounted, setMounted] = useState(false);

  const {
    phone_number,
    is_enabled,
    primary_language,
    custom_instructions,
    allow_orders,
    allow_booking,
    setPhoneNumber,
    setIsEnabled,
    setPrimaryLanguage,
    setCustomInstructions,
    setAllowOrders,
    setAllowBooking
  } = useVoiceAgentStore();

  useEffect(() => {
    setMounted(true);
  }, []);

  // Empty state for call history since we have no real data yet
  const callHistory: any[] = [];

  if (!mounted) {
    return null; // or a loading spinner
  }

  return (
    <div className="min-h-screen bg-gray-50/50 pb-20 font-inter">
      {/* Top Nav */}
      <div className="sticky top-0 z-40 bg-white/80 backdrop-blur-xl border-b border-gray-200">
        <div className="px-4 h-16 flex items-center justify-between max-w-2xl mx-auto">
          <div className="flex items-center gap-3">
            <Link href="/settings" className="p-2 -ml-2 text-gray-400 hover:text-gray-900 transition-colors rounded-full hover:bg-gray-100">
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
            </Link>
            <h1 className="text-xl font-bold font-outfit text-gray-900">Voice Agent</h1>
          </div>
        </div>
      </div>

      <div className="max-w-2xl mx-auto px-4 py-6 space-y-6">

        {/* Phone Number Card */}
        <div className="mac-glass-container p-6 rounded-2xl relative overflow-hidden">
          <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-50 rounded-bl-full -z-10"></div>
          <div className="flex items-start justify-between mb-4">
            <div className="w-12 h-12 bg-indigo-100 rounded-xl flex items-center justify-center text-2xl shadow-inner text-indigo-600">
              📞
            </div>
            {/* Toggle switch */}
            <label className="flex items-center cursor-pointer">
              <div className="relative">
                <input
                  type="checkbox"
                  className="sr-only"
                  checked={is_enabled}
                  onChange={(e) => setIsEnabled(e.target.checked)}
                  data-testid="ai-receptionist-toggle"
                />
                <div className={`block w-14 h-8 rounded-full transition-colors ${is_enabled ? 'bg-green-500' : 'bg-gray-200'}`}></div>
                <div className={`dot absolute left-1 top-1 bg-white w-6 h-6 rounded-full transition-transform ${is_enabled ? 'transform translate-x-6' : ''}`}></div>
              </div>
            </label>
          </div>
          <h2 className="text-lg font-bold font-outfit text-gray-900 mb-1">AI Receptionist</h2>
          <p className="text-sm text-gray-600 mb-4">Let an AI answer your missed calls and take orders 24/7.</p>

          <div className="bg-white/60 p-4 rounded-xl border border-white/20 shadow-sm backdrop-blur-md">
            <label className="block text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1">Your Business Phone Number</label>
            <div className="text-xl font-mono text-gray-900 tracking-tight">
              {phone_number || <span className="text-gray-400 italic text-sm">No phone number assigned</span>}
            </div>
            {!phone_number && (
              <button
                onClick={() => setPhoneNumber('(555) 123-4567')}
                className="mt-2 text-sm text-indigo-600 font-semibold hover:text-indigo-700"
              >
                Claim Number
              </button>
            )}
          </div>
        </div>

        {/* Configuration Card */}
        <div className="mac-glass-container p-6 rounded-2xl">
          <h2 className="text-lg font-bold font-outfit text-gray-900 mb-4">Behavior Configuration</h2>

          <div className="space-y-5">
            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">Primary Language</label>
              <select
                value={primary_language}
                onChange={(e) => setPrimaryLanguage(e.target.value)}
                className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent appearance-none"
                data-testid="primary-language-select"
              >
                <option value="English">English</option>
                <option value="Spanish">Spanish</option>
                <option value="Arabic">Arabic</option>
              </select>
            </div>

            <div>
              <label className="block text-sm font-semibold text-gray-700 mb-2">Custom Instructions</label>
              <textarea
                value={custom_instructions}
                onChange={(e) => setCustomInstructions(e.target.value)}
                placeholder="e.g. Tell callers to park in the back"
                className="w-full bg-white border border-gray-200 rounded-xl px-4 py-3 text-sm text-gray-900 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent min-h-[100px] resize-none"
                data-testid="custom-instructions-textarea"
              />
            </div>

            <div className="space-y-3 pt-2">
              <label className="flex items-center justify-between p-3 bg-white border border-gray-100 rounded-xl shadow-sm">
                <span className="text-sm font-medium text-gray-700">Allow taking orders</span>
                <input
                  type="checkbox"
                  checked={allow_orders}
                  onChange={(e) => setAllowOrders(e.target.checked)}
                  className="w-5 h-5 text-indigo-600 rounded border-gray-300 focus:ring-indigo-500"
                  data-testid="allow-orders-toggle"
                />
              </label>
              <label className="flex items-center justify-between p-3 bg-white border border-gray-100 rounded-xl shadow-sm">
                <span className="text-sm font-medium text-gray-700">Allow booking appointments</span>
                <input
                  type="checkbox"
                  checked={allow_booking}
                  onChange={(e) => setAllowBooking(e.target.checked)}
                  className="w-5 h-5 text-indigo-600 rounded border-gray-300 focus:ring-indigo-500"
                  data-testid="allow-booking-toggle"
                />
              </label>
            </div>
          </div>
        </div>

        {/* Call History Card */}
        <div className="mac-glass-container p-6 rounded-2xl">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-lg font-bold font-outfit text-gray-900">Call History & Transcripts</h2>
            <span className="text-xs bg-gray-100 text-gray-600 px-2 py-1 rounded-full font-medium">Last 30 Days</span>
          </div>

          <div className="space-y-3">
            {callHistory.length === 0 ? (
              <div className="text-center py-8">
                <div className="w-12 h-12 bg-gray-50 rounded-full flex items-center justify-center mx-auto mb-3">
                  <svg className="w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" /></svg>
                </div>
                <p className="text-sm text-gray-500 font-medium">No calls received yet</p>
                <p className="text-xs text-gray-400 mt-1">When your agent answers a call, it will appear here</p>
              </div>
            ) : (
              callHistory.map((call, idx) => (
                <div key={idx} className="bg-white border border-gray-100 rounded-xl p-4 shadow-sm flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 bg-indigo-50 rounded-full flex items-center justify-center text-indigo-600 font-bold">
                      {call.contact.charAt(0)}
                    </div>
                    <div>
                      <p className="text-sm font-bold text-gray-900">{call.contact}</p>
                      <p className="text-xs text-gray-500">{call.summary}</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <p className="text-xs font-semibold text-gray-900">{call.duration}</p>
                    <p className="text-xs text-gray-400">{call.date}</p>
                  </div>
                </div>
              ))
            )}
          </div>
        </div>

      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
      `}} />
    </div>
  );
}
