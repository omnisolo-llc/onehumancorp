"use client";

import React from "react";

interface Props {
  percentage: number;
  timeSlot: string;
  onMitigate: () => void;
}

export function OverloadAlert({ percentage, timeSlot, onMitigate }: Props) {
  if (percentage < 100) return null;

  return (
    <div className="p-4 mb-4 rounded-[12px] bg-red-500/10 border border-red-500/50 backdrop-blur-md flex flex-col gap-3">
      <div className="flex items-center gap-2 text-red-600 dark:text-red-400 font-bold">
        <span>🚨</span>
        <span>Capacity Overload: {percentage}%</span>
      </div>
      <p className="text-sm text-red-700 dark:text-red-300">
        Your workload for <strong>{timeSlot}</strong> exceeds available resources. Consider rescheduling or pausing new intake.
      </p>
      <button
        onClick={onMitigate}
        className="min-h-[44px] w-full rounded-[8px] bg-red-600 text-white font-semibold hover:bg-red-700 transition-colors shadow-sm"
      >
        Mitigate Load
      </button>
    </div>
  );
}
