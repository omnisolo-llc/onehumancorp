import React from 'react';
import { AppShell } from '../components/AppShell';

export default function OperationsPage() {
  return (
    <AppShell title="Operations Copilot">
      <div className="p-4 sm:p-6 lg:p-8 max-w-7xl mx-auto space-y-8 animate-in fade-in duration-500">
        <header className="mb-8">
          <h1 className="text-3xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] tracking-tight">Today</h1>
          <p className="text-[#86868B] dark:text-[#A1A1A6] text-lg mt-2 font-inter">Your daily schedule and operations overview.</p>
        </header>

        <section className="glassmorphism p-6 border border-white/40 dark:border-white/10 shadow-sm relative overflow-hidden">
          <div className="absolute inset-0 bg-gradient-to-br from-blue-50/50 to-purple-50/50 dark:from-blue-900/10 dark:to-purple-900/10 -z-10" />
          <h2 className="text-lg font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] mb-4 flex items-center gap-2">
            <span className="text-xl">☀️</span> Morning Briefing
          </h2>
          <div className="text-[#1D1D1F] dark:text-[#F5F5F7] text-sm font-inter">
            <p className="mb-2">You have 4 appointments today. 1 client still needs to pay their deposit.</p>
          </div>
        </section>

        <section className="space-y-4">
            <div className="flex flex-col gap-4">
                {/* Past Appointment */}
                <div className="glassmorphism p-4 border border-white/40 dark:border-white/10 shadow-sm flex flex-col sm:flex-row gap-4 items-start sm:items-center opacity-60">
                    <div className="w-16 flex flex-col items-center justify-center shrink-0">
                        <span className="text-sm font-medium text-[#86868B] dark:text-[#A1A1A6]">9:00 AM</span>
                    </div>
                    <div className="flex-1">
                        <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Piano Lesson</h3>
                        <p className="text-sm text-[#86868B] dark:text-[#A1A1A6]">Alice Smith</p>
                    </div>
                    <div className="flex flex-col sm:flex-row gap-2 shrink-0">
                        <span className="px-3 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 text-xs font-medium rounded-full self-start">Paid</span>
                    </div>
                </div>

                {/* Current Appointment */}
                <div className="glassmorphism p-4 border border-blue-200 dark:border-blue-800 shadow-md flex flex-col sm:flex-row gap-4 items-start sm:items-center relative overflow-hidden">
                    <div className="absolute left-0 top-0 bottom-0 w-1 bg-[#0066FF] rounded-l-[16px]"></div>
                    <div className="w-16 flex flex-col items-center justify-center shrink-0">
                        <span className="text-sm font-bold text-[#0071E3] dark:text-blue-400">11:00 AM</span>
                        <span className="text-xs text-[#0066FF] font-medium">Now</span>
                    </div>
                    <div className="flex-1">
                        <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] text-lg">Guitar Lesson</h3>
                        <p className="text-sm text-[#86868B] dark:text-[#A1A1A6]">Sarah Johnson</p>
                        <div className="mt-2 text-sm text-[#1D1D1F] dark:text-[#F5F5F7] bg-white/50 dark:bg-black/20 p-2 rounded-lg border border-black/5 dark:border-white/5">
                            <span className="font-medium flex items-center gap-1"><span className="text-xs">🤖</span> AI Summary:</span> 3rd lesson. Focus: Jazz scales. She struggled with chords last week.
                        </div>
                    </div>
                    <div className="flex flex-col gap-2 shrink-0 w-full sm:w-auto">
                        <span className="px-3 py-1 bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400 text-xs font-medium rounded-full self-start sm:self-end">Deposit Required</span>
                        <button className="px-4 py-2 bg-[#0071E3] hover:bg-blue-700 text-white text-sm font-medium rounded-lg transition-colors shadow-sm">Message Client</button>
                    </div>
                </div>

                 {/* Future Appointment */}
                 <div className="glassmorphism p-4 border border-white/40 dark:border-white/10 shadow-sm flex flex-col sm:flex-row gap-4 items-start sm:items-center hover:shadow-md transition-shadow">
                    <div className="w-16 flex flex-col items-center justify-center shrink-0">
                        <span className="text-sm font-medium text-[#1D1D1F] dark:text-[#F5F5F7]">2:00 PM</span>
                    </div>
                    <div className="flex-1">
                        <h3 className="font-semibold text-[#1D1D1F] dark:text-[#F5F5F7]">Vocal Coaching</h3>
                        <p className="text-sm text-[#86868B] dark:text-[#A1A1A6]">Mike Brown</p>
                    </div>
                    <div className="flex flex-col sm:flex-row gap-2 shrink-0">
                        <span className="px-3 py-1 bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 text-xs font-medium rounded-full self-start">Paid</span>
                    </div>
                </div>
            </div>
        </section>
      </div>
    </AppShell>
  );
}
