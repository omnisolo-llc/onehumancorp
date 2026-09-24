"use client";

import React from "react";
import { AppShell } from "../components/AppShell";

export default function MilestonesPage() {

  return (
    <AppShell title="Milestones" subtitle="Celebrate your business growth achievements.">
      <div className="p-6 max-w-4xl mx-auto space-y-6">
        <h1 className="text-3xl font-bold font-outfit text-gray-900 dark:text-white">
          Milestones & Achievements
        </h1>

        <div className="space-y-4">
          <div className="p-6 bg-white dark:bg-gray-800 rounded-2xl shadow-sm border border-gray-100 dark:border-gray-700">
            <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-white mb-2">
              Four-Figure Club
            </h3>
            <p className="text-sm text-gray-600 dark:text-gray-300 mb-4">
              You surpassed $1,000 in gross revenue! Share this achievement to unlock exclusive rewards.
            </p>
            <div className="p-4 bg-indigo-50 dark:bg-indigo-900/30 rounded-xl text-indigo-700 dark:text-indigo-300 text-sm font-medium">
              Join OmniSolo &amp; get 14 days of Pro free
            </div>
          </div>
        </div>
      </div>
    </AppShell>
  );
}
