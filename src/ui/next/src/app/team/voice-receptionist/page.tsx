"use client";

import React, { useState } from 'react';
import Link from 'next/link';

export default function VoiceReceptionistPage() {
  const [enabled, setEnabled] = useState(false);
  const [phone, setPhone] = useState("+1 (555) 019-2834");
  const [voice, setVoice] = useState("friendly-female");
  const [language, setLanguage] = useState("english");
  const [instructions, setInstructions] = useState("");
  const [saved, setSaved] = useState(false);

  const handleSave = () => {
    setSaved(true);
    setTimeout(() => setSaved(false), 3000);
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col items-center">
      <div className="w-full max-w-[375px] bg-white min-h-screen shadow-2xl relative flex flex-col font-inter">

        {/* macOS-style Translucent Glass header */}
        <header className="sticky top-0 z-50 bg-white/65 backdrop-blur-[20px] saturate-[200%] border-b border-gray-200 px-5 pt-12 pb-4 flex items-center justify-between">
          <Link href="/team" className="text-indigo-600 hover:text-indigo-700 p-2 -ml-2 rounded-xl transition-colors">
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M15 19l-7-7 7-7" />
            </svg>
          </Link>
          <h1 className="font-outfit font-bold text-gray-900 text-lg absolute left-1/2 -translate-x-1/2">
            Phone Receptionist
          </h1>
          <div className="w-8"></div>
        </header>

        <main className="flex-1 overflow-y-auto p-5 pb-24 space-y-8">

          {/* Main Toggle */}
          <div className="bg-gradient-to-br from-indigo-50 to-blue-50 rounded-2xl p-6 border border-indigo-100 shadow-sm">
            <div className="flex items-center justify-between mb-2">
              <h2 className="font-outfit font-bold text-gray-900 text-lg">Let AI answer my missed calls</h2>
              <button
                onClick={() => setEnabled(!enabled)}
                className={`relative inline-flex h-7 w-12 items-center rounded-full transition-colors ${enabled ? 'bg-indigo-600' : 'bg-gray-300'}`}
              >
                <span className={`inline-block h-5 w-5 transform rounded-full bg-white transition-transform ${enabled ? 'translate-x-6' : 'translate-x-1'}`} />
              </button>
            </div>
            <p className="text-sm text-gray-600 mt-2 leading-relaxed">
              When you're busy, your AI will answer instantly, handle the inquiry, and text the customer a link.
            </p>
          </div>

          <div className={`space-y-8 transition-opacity duration-300 ${enabled ? 'opacity-100' : 'opacity-50 pointer-events-none'}`}>

            {/* Number Selection */}
            <div className="space-y-3">
              <label className="block text-sm font-bold text-gray-900">Your Business Number</label>
              <div className="relative">
                <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg className="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                  </svg>
                </div>
                <select
                  value={phone}
                  onChange={(e) => setPhone(e.target.value)}
                  className="block w-full pl-10 pr-10 py-3 text-base border border-gray-200 focus:outline-none focus:ring-2 focus:ring-indigo-500 rounded-xl bg-white text-gray-900 font-medium appearance-none"
                >
                  <option value="+1 (555) 019-2834">+1 (555) 019-2834 (Main)</option>
                  <option value="Get New Number">Get a new local number...</option>
                </select>
                <div className="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                  <svg className="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fillRule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clipRule="evenodd" />
                  </svg>
                </div>
              </div>
            </div>

            {/* Language Selection */}
            <div className="space-y-3">
              <label className="block text-sm font-bold text-gray-900">What languages do your customers speak?</label>
              <div className="flex flex-wrap gap-2">
                {[
                  { id: 'english', label: 'English' },
                  { id: 'spanish', label: 'Español' },
                  { id: 'arabic', label: 'العربية' },
                  { id: 'french', label: 'Français' },
                ].map((lang) => (
                  <button
                    key={lang.id}
                    onClick={() => setLanguage(lang.id)}
                    className={`px-4 py-2 rounded-full text-sm font-semibold transition-all ${language === lang.id ? 'bg-indigo-600 text-white shadow-md shadow-indigo-200' : 'bg-gray-100 text-gray-600 hover:bg-gray-200'}`}
                  >
                    {lang.label}
                  </button>
                ))}
              </div>
            </div>

            {/* Voice Selection */}
            <div className="space-y-3">
              <label className="block text-sm font-bold text-gray-900">Receptionist Voice</label>
              <div className="grid grid-cols-2 gap-3">
                {[
                  { id: 'friendly-female', label: 'Friendly', icon: '👩🏽' },
                  { id: 'professional-male', label: 'Professional', icon: '👨🏻‍💼' },
                ].map((v) => (
                  <button
                    key={v.id}
                    onClick={() => setVoice(v.id)}
                    className={`flex items-center gap-2 p-3 rounded-xl border-2 transition-all text-left ${voice === v.id ? 'border-indigo-600 bg-indigo-50' : 'border-gray-100 bg-white hover:border-gray-200'}`}
                  >
                    <span className="text-xl">{v.icon}</span>
                    <span className={`text-sm font-semibold ${voice === v.id ? 'text-indigo-900' : 'text-gray-700'}`}>{v.label}</span>
                  </button>
                ))}
              </div>
            </div>

            {/* Knowledge Card */}
            <div className="space-y-3">
              <label className="block text-sm font-bold text-gray-900">What should the receptionist know?</label>
              <textarea
                value={instructions}
                onChange={(e) => setInstructions(e.target.value)}
                placeholder="e.g., I'm Carlos, I charge $50/hr, I don't do electrical work. Always text them the booking link."
                className="w-full h-32 p-4 border border-gray-200 rounded-xl bg-white text-gray-900 text-sm focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-none leading-relaxed"
              />
            </div>

            {/* Advanced Settings Dropdown */}
            <details className="group border border-gray-200 rounded-xl bg-white overflow-hidden">
              <summary className="flex justify-between items-center font-medium cursor-pointer list-none p-4 text-sm text-gray-600 hover:bg-gray-50">
                <span>Advanced Settings</span>
                <span className="transition group-open:rotate-180">
                  <svg fill="none" height="24" shapeRendering="geometricPrecision" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" viewBox="0 0 24 24" width="24"><path d="M6 9l6 6 6-6"></path></svg>
                </span>
              </summary>
              <div className="text-neutral-600 mt-3 group-open:animate-fadeIn p-4 pt-0 border-t border-gray-100">
                <p className="text-xs text-gray-500 mb-2">SIP Trunking & WebRTC Config</p>
                <div className="bg-gray-100 p-3 rounded text-xs font-mono break-all text-gray-600">
                  sip:agent-voice@sip.onehumancorp.internal:5060<br/>
                  wss://voice.onehumancorp.com/stream
                </div>
              </div>
            </details>

          </div>
        </main>

        {/* Sticky Action Bar */}
        <div className="absolute bottom-0 w-full p-5 bg-white border-t border-gray-100 pb-safe">
          <button
            onClick={handleSave}
            className="w-full bg-indigo-600 hover:bg-indigo-700 text-white font-bold py-4 rounded-2xl shadow-lg shadow-indigo-200 transition-all active:scale-[0.98] flex items-center justify-center gap-2 min-h-[56px]"
          >
            {saved ? (
              <>
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" /></svg>
                Saved
              </>
            ) : 'Save Settings'}
          </button>
        </div>
      </div>

      <style dangerouslySetInnerHTML={{__html: `
        @import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Outfit:wght@500;600;700;800&display=swap');
        .font-inter { font-family: 'Inter', sans-serif; }
        .font-outfit { font-family: 'Outfit', sans-serif; }
        .pb-safe { padding-bottom: env(safe-area-inset-bottom, 20px); }
      `}} />
    </div>
  );
}
