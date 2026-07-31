"use client";

import { useState } from "react";
import { AppShell } from "../components/AppShell";

export default function GrowthPage() {
  const [copied, setCopied] = useState(false);
  const referralLink = "https://ohc.com/join?ref=YOUR_BUSINESS";

  const handleCopy = () => {
    navigator.clipboard.writeText(referralLink);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <AppShell title="Growth & Referrals" subtitle="Grow your business with OHC">
      <div className="flex flex-col gap-6 p-4 md:p-8 max-w-4xl mx-auto w-full">
        <section className="bg-white dark:bg-[#1C1C1E] p-6 rounded-2xl shadow-sm border border-black/5 dark:border-white/5">
          <h2 className="text-xl font-semibold mb-2">One-Tap Referral</h2>
          <p className="text-black/60 dark:text-white/60 mb-4">
            Share your unique link and earn credit when other businesses join OHC.
          </p>
          <div className="flex flex-col sm:flex-row gap-3">
            <input
              type="text"
              readOnly
              value={referralLink}
              className="flex-1 px-4 py-3 bg-black/5 dark:bg-white/5 rounded-xl text-black dark:text-white outline-none focus:ring-2 focus:ring-blue-500/50"
            />
            <button
              onClick={handleCopy}
              className="px-6 py-3 bg-blue-500 text-white font-medium rounded-xl hover:bg-blue-600 transition-colors active:scale-[0.98]"
            >
              {copied ? "Copied!" : "Copy Link"}
            </button>
          </div>
        </section>

        <section className="bg-white dark:bg-[#1C1C1E] p-6 rounded-2xl shadow-sm border border-black/5 dark:border-white/5">
          <h2 className="text-xl font-semibold mb-2">Milestones</h2>
          <p className="text-black/60 dark:text-white/60 mb-6">
            Track your progress and celebrate successes.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div className="p-4 bg-black/5 dark:bg-white/5 rounded-xl flex flex-col items-center justify-center text-center">
              <span className="text-3xl font-bold mb-1">10</span>
              <span className="text-sm text-black/60 dark:text-white/60">Orders Completed</span>
              <div className="w-full bg-black/10 dark:bg-white/10 h-1.5 rounded-full mt-3 overflow-hidden">
                 <div className="bg-green-500 w-full h-full"></div>
              </div>
            </div>
             <div className="p-4 bg-black/5 dark:bg-white/5 rounded-xl flex flex-col items-center justify-center text-center opacity-70">
              <span className="text-3xl font-bold mb-1">50</span>
              <span className="text-sm text-black/60 dark:text-white/60">Orders Completed</span>
              <div className="w-full bg-black/10 dark:bg-white/10 h-1.5 rounded-full mt-3 overflow-hidden">
                 <div className="bg-green-500 w-[20%] h-full"></div>
              </div>
            </div>
            <div className="p-4 bg-black/5 dark:bg-white/5 rounded-xl flex flex-col items-center justify-center text-center opacity-70">
              <span className="text-3xl font-bold mb-1">100</span>
              <span className="text-sm text-black/60 dark:text-white/60">Orders Completed</span>
               <div className="w-full bg-black/10 dark:bg-white/10 h-1.5 rounded-full mt-3 overflow-hidden">
                 <div className="bg-green-500 w-[10%] h-full"></div>
              </div>
            </div>
          </div>
        </section>
      </div>
    </AppShell>
  );
}
