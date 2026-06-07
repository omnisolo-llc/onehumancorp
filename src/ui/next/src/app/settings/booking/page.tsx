"use client";

import React, { useState } from "react";
import { AppShell } from "../../components/AppShell";
import { useRouter } from "next/navigation";

export default function BookingSettingsPage() {
  const router = useRouter();
  const [prompt, setPrompt] = useState("");
  const [loading, setLoading] = useState(false);
  const [rules, setRules] = useState<any>(null);

  const handleExtract = async () => {
    if (!prompt.trim()) return;
    setLoading(true);
    try {
      const res = await fetch("/api/v1/booking/rules", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ prompt }),
      });
      const data = await res.json();
      if (data.success) {
        setRules(data.rules);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <AppShell>
      <div className="max-w-4xl mx-auto py-8 px-4 sm:px-6 lg:px-8">
        <div className="flex items-center gap-4 mb-6">
          <button
            onClick={() => router.back()}
            className="p-2 bg-white border border-gray-200 rounded-full text-gray-500 hover:text-gray-700 hover:bg-gray-50"
          >
            &larr;
          </button>
          <h1 className="text-3xl font-bold font-outfit text-gray-900">
            Booking Rules
          </h1>
        </div>

        <div className="glassmorphism p-6 rounded-2xl mb-8">
          <h2 className="text-xl font-semibold mb-4 text-gray-800">
            Current Schedule
          </h2>
          {rules ? (
            <div className="grid grid-cols-7 gap-2 bg-gray-50 p-4 rounded-xl border border-gray-200 text-sm">
              {["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"].map((day) => (
                <div key={day} className="flex flex-col items-center gap-2">
                  <div className="font-bold text-gray-700">{day}</div>
                  {rules.working_days && rules.working_days.includes(day) ? (
                    <div className="bg-green-100 text-green-800 px-2 py-1 rounded text-xs text-center">
                      {rules.start_time} - {rules.end_time}
                    </div>
                  ) : (
                    <div className="bg-gray-200 text-gray-500 px-2 py-1 rounded text-xs text-center">
                      Off
                    </div>
                  )}
                </div>
              ))}
              <div className="col-span-7 mt-4 flex items-center gap-2 text-indigo-700 font-medium">
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                Buffer time between jobs: {rules.buffer_time_minutes} minutes
              </div>
            </div>
          ) : (
            <div className="bg-gray-50 border border-gray-200 rounded-xl p-8 text-center text-gray-500">
              No booking rules configured. Just tell the assistant your schedule below!
            </div>
          )}
        </div>

        <div className="glassmorphism p-6 rounded-2xl flex flex-col gap-4 shadow-lg border border-indigo-100">
          <h2 className="text-lg font-semibold text-indigo-900 flex items-center gap-2">
            <svg
              className="w-5 h-5 text-indigo-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
            </svg>
            Ask the Sales Agent
          </h2>
          <p className="text-sm text-indigo-700/80 mb-2">
            Instead of clicking toggles, just explain your schedule normally. For example: "I work 9-5 Mon-Fri, but I need 30 mins between jobs to drive, and I don't work Thursday mornings."
          </p>
          <div className="relative">
            <textarea
              className="w-full bg-white border border-gray-300 rounded-xl p-4 pr-32 focus:outline-none focus:ring-2 focus:ring-indigo-500 resize-none shadow-sm min-h-[100px]"
              placeholder="Explain your working hours..."
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              disabled={loading}
            />
            <button
              onClick={handleExtract}
              disabled={loading || !prompt.trim()}
              className="absolute bottom-4 right-4 bg-indigo-600 text-white px-4 py-2 rounded-lg font-semibold text-sm hover:bg-indigo-700 disabled:opacity-50 transition-colors shadow-md"
            >
              {loading ? "Thinking..." : "Extract Rules"}
            </button>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
