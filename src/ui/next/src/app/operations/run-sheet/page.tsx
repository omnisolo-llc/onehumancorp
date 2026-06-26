'use client';

import React, { useState, useEffect } from 'react';
import { AppShell } from '../../components/AppShell';
import { motion, AnimatePresence } from 'framer-motion';

export default function DailyRunSheetPage() {
  const [runSheet, setRunSheet] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [suggestion, setSuggestion] = useState<any>(null);

  useEffect(() => {
    fetchRunSheet();
  }, []);

  const fetchRunSheet = async () => {
    try {
      const res = await fetch('/api/v1/dispatch/run-sheet');
      if (res.ok) {
        const data = await res.json();
        setRunSheet(data);

        // Simulate a new urgent request arriving after 3 seconds
        setTimeout(() => {
            setSuggestion({
                title: "New urgent request: Leak at 123 Main St.",
                message: "AI suggests inserting at 1:00 PM (adds 10 mins travel).",
                appointmentId: "mock-appt-123"
            });
        }, 3000);
      }
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const acceptSuggestion = async () => {
    if (!suggestion) return;
    try {
        const res = await fetch('/api/v1/dispatch/inject-job', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                tenant_id: "storefront",
                date: new Date().toISOString().split('T')[0],
                appointment_id: suggestion.appointmentId
            })
        });
        if (res.ok) {
            setSuggestion(null);
            alert("Suggestion accepted! The AI agent will notify other customers of the shifted schedule.");
            // Re-fetch run sheet
            fetchRunSheet();
        }
    } catch (e) {
        console.error(e);
    }
  };

  return (
    <AppShell title="Daily Run Sheet">
      <div className="p-4 sm:p-6 lg:p-8 max-w-[375px] mx-auto min-h-screen relative font-outfit">
        <header className="mb-6">
          <h1 className="text-2xl font-bold text-[#1D1D1F] dark:text-[#F5F5F7]">Today</h1>
          <p className="text-[#86868B] dark:text-[#A1A1A6] text-sm mt-1 font-inter">Your daily run sheet.</p>
        </header>

        <div className="space-y-4 pb-24">
            {loading ? (
                <div className="text-center text-[#86868B] py-10">Loading route...</div>
            ) : runSheet?.stops?.length > 0 ? (
                runSheet.stops.map((stop: any, index: number) => (
                    <div key={stop.id} className={`glassmorphism rounded-2xl p-4 border shadow-sm flex flex-col gap-2 relative overflow-hidden ${stop.status === 'Completed' ? 'border-white/20 opacity-60' : 'border-blue-200 dark:border-blue-800'}`}>
                        {stop.status !== 'Completed' && (
                            <div className="absolute left-0 top-0 bottom-0 w-1 bg-[#0066FF] rounded-l-2xl"></div>
                        )}
                        <div className="flex justify-between items-center">
                            <span className="text-sm font-bold text-[#1D1D1F] dark:text-white">
                                {stop.estimated_arrival ? new Date(stop.estimated_arrival).toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'}) : 'Time TBD'}
                            </span>
                            <span className="px-2 py-1 bg-gray-100 dark:bg-gray-800 text-xs rounded-full text-gray-600 dark:text-gray-300">
                                {stop.status}
                            </span>
                        </div>
                        <div className="font-semibold text-lg text-[#1D1D1F] dark:text-white">
                            {stop.appointment?.job_name || 'Service Stop'}
                        </div>
                        <div className="text-sm text-[#86868B]">
                            {stop.appointment?.customer_name || 'Customer'} • {stop.appointment?.location_address || 'Address'}
                        </div>
                        {stop.notes && (
                            <div className="mt-2 text-xs text-[#1D1D1F] dark:text-white bg-white/50 dark:bg-black/20 p-2 rounded-lg border border-black/5 dark:border-white/5">
                                {stop.notes}
                            </div>
                        )}
                        {stop.status !== 'Completed' && (
                            <button className="mt-2 w-full py-2 bg-[#0071E3] hover:bg-blue-600 text-white rounded-xl text-sm font-medium transition-colors">
                                Start Job
                            </button>
                        )}
                    </div>
                ))
            ) : (
                <div className="text-center text-[#86868B] py-10 bg-white/10 rounded-2xl border border-white/20">
                    No stops scheduled for today.
                </div>
            )}
        </div>

        <AnimatePresence>
            {suggestion && (
                <motion.div
                    initial={{ y: 100, opacity: 0 }}
                    animate={{ y: 0, opacity: 1 }}
                    exit={{ y: 100, opacity: 0 }}
                    className="fixed bottom-6 left-1/2 -translate-x-1/2 w-[calc(100%-2rem)] max-w-[343px] glassmorphism p-4 rounded-2xl border border-white/30 shadow-2xl z-50 bg-white/80 dark:bg-black/80 backdrop-blur-xl"
                >
                    <div className="flex items-start gap-3">
                        <div className="text-xl mt-1">🚨</div>
                        <div>
                            <h4 className="font-bold text-[#1D1D1F] dark:text-white text-sm">{suggestion.title}</h4>
                            <p className="text-xs text-[#86868B] mt-1 mb-3">{suggestion.message}</p>
                            <div className="flex gap-2">
                                <button
                                    onClick={acceptSuggestion}
                                    className="flex-1 bg-black dark:bg-white text-white dark:text-black py-2 rounded-xl text-xs font-semibold"
                                >
                                    Accept & Notify
                                </button>
                                <button
                                    onClick={() => setSuggestion(null)}
                                    className="flex-1 bg-gray-200 dark:bg-gray-800 text-gray-800 dark:text-gray-200 py-2 rounded-xl text-xs font-medium"
                                >
                                    Dismiss
                                </button>
                            </div>
                        </div>
                    </div>
                </motion.div>
            )}
        </AnimatePresence>
      </div>
    </AppShell>
  );
}
