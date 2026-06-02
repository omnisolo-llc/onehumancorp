"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { WithTooltip } from "../../components/TooltipRegistry";

export default function VoiceAgentDashboard() {
  const [isEnabled, setIsEnabled] = useState(false);
  const [primaryLanguage, setPrimaryLanguage] = useState("English");
  const [customInstructions, setCustomInstructions] = useState("");
  const [phoneNumber, setPhoneNumber] = useState("(555) 123-4567");
  const [allowOrders, setAllowOrders] = useState(true);
  const [allowBookings, setAllowBookings] = useState(true);

  useEffect(() => {
    fetch('/api/voice-agent/config')
      .then(res => res.json())
      .then(config => {
        setIsEnabled(config.isEnabled ?? false);
        setPrimaryLanguage(config.primaryLanguage ?? "English");
        setCustomInstructions(config.customInstructions ?? "");
        setAllowOrders(config.allowOrders ?? true);
        setAllowBookings(config.allowBookings ?? true);
      })
      .catch(err => console.error("Failed to load config", err));
  }, []);

  const handleSave = async () => {
    try {
      const response = await fetch('/api/voice-agent/config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          isEnabled,
          primaryLanguage,
          customInstructions,
          allowOrders,
          allowBookings
        })
      });
      if (response.ok) {
        alert("Voice Agent settings saved!");
      }
    } catch (error) {
      console.error("Failed to save config", error);
      alert("Failed to save settings.");
    }
  };

  const mockCalls = [
    { id: 1, contact: "Maya G.", date: "Today, 10:30 AM", duration: "1m 45s", summary: "Asked about vegan cake options. AI directed them to custom order form." },
    { id: 2, contact: "Unknown", date: "Yesterday, 2:15 PM", duration: "3m 20s", summary: "Booked plumbing estimate for Tuesday at 3 PM." },
    { id: 3, contact: "Carlos R.", date: "Mon, 9:00 AM", duration: "45s", summary: "Called to confirm store hours. AI provided correct times." }
  ];

  return (
    <div className="min-h-screen bg-gray-50 pb-20 font-inter">
      <header className="bg-white border-b border-gray-100 p-4 sticky top-0 z-10 glass-header shadow-sm">
        <div className="flex items-center justify-between max-w-4xl mx-auto">
          <Link href="/dashboard" className="text-blue-600 font-medium">← Back</Link>
          <h1 className="text-xl font-bold text-gray-900 font-outfit text-center flex-1">Voice Agent</h1>
          <div className="w-12"></div>
        </div>
      </header>

      <main className="p-4 max-w-4xl mx-auto space-y-6 mt-4">
        {/* Settings Card */}
        <section className="bg-white/80 backdrop-blur-[20px] saturate-[200%] p-6 rounded-2xl shadow-sm border border-white/40 glassmorphism">
          <div className="flex justify-between items-center mb-6">
            <div>
              <h2 className="text-xl font-bold text-gray-900 font-outfit">AI Receptionist</h2>
              <p className="text-sm text-gray-500 mt-1">Your Business Phone Number: <strong className="text-gray-800">{phoneNumber}</strong></p>
            </div>

            <WithTooltip id="voice_toggle">
                <label className="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" className="sr-only peer" checked={isEnabled} onChange={(e) => setIsEnabled(e.target.checked)} />
                <div className="w-14 h-7 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all peer-checked:bg-blue-600"></div>
                </label>
            </WithTooltip>
          </div>

          <div className={`space-y-5 transition-opacity duration-300 ${isEnabled ? 'opacity-100' : 'opacity-50 pointer-events-none'}`}>
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Primary Language</label>
              <select
                className="w-full p-3 border border-gray-200 rounded-xl bg-white/50 focus:ring-2 focus:ring-blue-500 outline-none transition-all"
                value={primaryLanguage}
                onChange={(e) => setPrimaryLanguage(e.target.value)}
              >
                <option value="English">English</option>
                <option value="Spanish">Spanish</option>
                <option value="Arabic">Arabic</option>
                <option value="French">French</option>
              </select>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Allowed Actions</label>
              <div className="space-y-3">
                <label className="flex items-center space-x-3 p-3 border border-gray-100 rounded-xl hover:bg-gray-50 transition-colors">
                  <input type="checkbox" className="w-5 h-5 text-blue-600 rounded border-gray-300 focus:ring-blue-500" checked={allowOrders} onChange={(e) => setAllowOrders(e.target.checked)} />
                  <span className="text-gray-700">Allow taking orders & pre-orders</span>
                </label>
                <label className="flex items-center space-x-3 p-3 border border-gray-100 rounded-xl hover:bg-gray-50 transition-colors">
                  <input type="checkbox" className="w-5 h-5 text-blue-600 rounded border-gray-300 focus:ring-blue-500" checked={allowBookings} onChange={(e) => setAllowBookings(e.target.checked)} />
                  <span className="text-gray-700">Allow booking appointments</span>
                </label>
              </div>
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">Custom Instructions</label>
              <textarea
                className="w-full p-3 border border-gray-200 rounded-xl bg-white/50 focus:ring-2 focus:ring-blue-500 outline-none transition-all resize-none"
                rows={4}
                placeholder="e.g., 'Tell callers to park in the back' or 'Mention our weekend special on cupcakes.'"
                value={customInstructions}
                onChange={(e) => setCustomInstructions(e.target.value)}
              />
            </div>

            <button
              onClick={handleSave}
              className="w-full bg-blue-600 text-white font-medium py-3 rounded-xl hover:bg-blue-700 transition-colors active:scale-[0.98]"
            >
              Save Configuration
            </button>
          </div>
        </section>

        {/* Call History */}
        <section className="bg-white/80 backdrop-blur-[20px] saturate-[200%] p-6 rounded-2xl shadow-sm border border-white/40 glassmorphism mt-6">
          <h2 className="text-xl font-bold text-gray-900 font-outfit mb-4">Call History & Transcripts</h2>
          <div className="space-y-4">
            {mockCalls.map(call => (
              <details key={call.id} className="group border border-gray-100 rounded-xl bg-white/60 overflow-hidden [&_summary::-webkit-details-marker]:hidden">
                <summary className="p-4 cursor-pointer flex justify-between items-center hover:bg-gray-50 transition-colors">
                  <div className="flex flex-col">
                    <span className="font-semibold text-gray-900">{call.contact}</span>
                    <span className="text-xs text-gray-500 mt-1">{call.date} • {call.duration}</span>
                  </div>
                  <div className="text-blue-600 group-open:rotate-180 transition-transform">
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m6 9 6 6 6-6"/></svg>
                  </div>
                </summary>
                <div className="p-4 border-t border-gray-100 bg-gray-50/50">
                  <h4 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">AI Summary</h4>
                  <p className="text-sm text-gray-700 mb-4">{call.summary}</p>
                  <div className="flex space-x-3">
                    <button className="flex-1 bg-white border border-gray-200 text-gray-700 py-2 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors shadow-sm">Read Transcript</button>
                    <button className="flex-1 bg-white border border-gray-200 text-gray-700 py-2 rounded-lg text-sm font-medium hover:bg-gray-50 transition-colors shadow-sm">Listen (Audio)</button>
                  </div>
                </div>
              </details>
            ))}
          </div>
        </section>
      </main>
    </div>
  );
}
