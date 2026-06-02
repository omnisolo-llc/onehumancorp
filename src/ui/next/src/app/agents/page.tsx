'use client';

import React, { useState } from 'react';

export default function AgentsPage() {
  const [voiceAiEnabled, setVoiceAiEnabled] = useState(false);
  const [voiceAiPhone, setVoiceAiPhone] = useState('');

  return (
    <div className="p-8 max-w-4xl mx-auto space-y-8">
      <h1 className="text-3xl font-semibold mb-6">AI Agents Dashboard</h1>

      <div id="voice-ai-config" className="bg-white rounded-lg shadow-sm p-6 border border-gray-100">
        <h2 className="text-xl font-medium mb-4 flex items-center">
          <span className="mr-2">🎙️</span> Autonomous Voice AI Receptionist
        </h2>

        <p className="text-gray-600 mb-6">
          Enable the Voice AI assistant to automatically answer customer calls, check calendar availability, provide quotes, and take orders while you are busy.
        </p>

        <div className="space-y-4">
          <label className="flex items-center space-x-3">
            <input
              type="checkbox"
              id="enable-voice-ai"
              checked={voiceAiEnabled}
              onChange={(e) => setVoiceAiEnabled(e.target.checked)}
              className="form-checkbox h-5 w-5 text-blue-600 rounded"
            />
            <span className="text-gray-800 font-medium">Activate Voice AI Receptionist</span>
          </label>

          {voiceAiEnabled && (
            <div className="pl-8 pt-4 space-y-4 animate-fade-in border-l-2 border-blue-100">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Connected Phone Number
                </label>
                <input
                  type="tel"
                  id="voice-ai-phone-number"
                  placeholder="+1 (555) 000-0000"
                  value={voiceAiPhone}
                  onChange={(e) => setVoiceAiPhone(e.target.value)}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border"
                />
                <p className="mt-1 text-xs text-gray-500">
                  This number will be routed to your AI assistant. Ensure call forwarding is configured if using your existing business line.
                </p>
              </div>

              <div>
                 <label className="block text-sm font-medium text-gray-700 mb-1">
                  Operating Hours
                </label>
                <select id="voice-ai-operating-hours" className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm p-2 border">
                  <option value="always">24/7 (Always On)</option>
                  <option value="outside_business">Outside Business Hours</option>
                  <option value="custom">Custom Schedule...</option>
                </select>
              </div>

              <div className="pt-4">
                <button
                   id="save-voice-ai-config"
                   className="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md shadow-sm transition-colors"
                   onClick={() => {
                     // In a real implementation this would trigger an API call to save the config
                     alert('Voice AI configuration saved successfully!');
                   }}
                >
                  Save Configuration
                </button>
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Existing agents would be rendered here */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mt-8 opacity-60">
        <div className="bg-gray-50 rounded-lg p-6 border border-gray-200 border-dashed">
            <h3 className="font-medium mb-2 text-gray-700">The Operations Manager</h3>
            <p className="text-sm text-gray-500">Handles day-to-day execution.</p>
        </div>
        <div className="bg-gray-50 rounded-lg p-6 border border-gray-200 border-dashed">
            <h3 className="font-medium mb-2 text-gray-700">The Promoter</h3>
            <p className="text-sm text-gray-500">Marketing & Advertising.</p>
        </div>
      </div>
    </div>
  );
}
