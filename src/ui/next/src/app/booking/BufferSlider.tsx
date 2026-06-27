"use client";

import React from "react";

interface Props {
  value: number;
  onChange: (val: number) => void;
  label: string;
}

export function BufferSlider({ value, onChange, label }: Props) {
  return (
    <div className="p-4 rounded-xl glassmorphism border border-white/20 bg-white/5 flex flex-col gap-2">
      <div className="flex justify-between items-center mb-1">
        <label className="text-xs font-bold uppercase tracking-tight text-gray-500 dark:text-gray-400">
          {label}
        </label>
        <span className="text-sm font-mono font-bold text-indigo-600 dark:text-indigo-400">
          {value} min
        </span>
      </div>
      <input
        type="range"
        min="0"
        max="120"
        step="15"
        value={value}
        onChange={(e) => onChange(parseInt(e.target.value))}
        className="w-full h-2 bg-gray-200 dark:bg-gray-700 rounded-lg appearance-none cursor-pointer accent-[#0066FF]"
      />
      <div className="flex justify-between text-[10px] text-gray-400 font-medium px-1">
        <span>0m</span>
        <span>60m</span>
        <span>120m</span>
      </div>
    </div>
  );
}
