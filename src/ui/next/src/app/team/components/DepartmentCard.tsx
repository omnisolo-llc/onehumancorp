"use client";

import React from 'react';

type Props = {
  name: string;
  pendingCount: number;
  autonomyLevel: string;
  onAutonomyChange: (level: string) => void;
  onClick: () => void;
};

export default function DepartmentCard({ name, pendingCount, autonomyLevel, onAutonomyChange, onClick }: Props) {
  return (
    <div className="bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-2xl p-5 shadow-sm hover:shadow-md transition-all mb-4">
      <div
        onClick={onClick}
        className="cursor-pointer w-full text-left active:scale-[0.98] flex items-center justify-between group mb-4"
      >
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-full bg-gradient-to-tr from-blue-100 to-blue-50 flex items-center justify-center border border-blue-200/50 shadow-inner flex-shrink-0">
           <span className="text-xl font-bold text-blue-600 font-outfit">{name.charAt(4)}</span>
        </div>

        <div>
          <h3 className="font-outfit font-semibold text-gray-900 text-lg">{name}</h3>
          {pendingCount > 0 ? (
            <p className="text-sm font-medium text-orange-600 mt-0.5">
              {pendingCount} item{pendingCount > 1 ? 's' : ''} awaiting approval
            </p>
          ) : (
            <p className="text-sm text-gray-500 mt-0.5">Active and running</p>
          )}
        </div>
      </div>

        <div className="text-gray-300 group-hover:text-blue-500 transition-colors">
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
        </div>
      </div>

      <div className="mt-2 pt-3 border-t border-gray-100 flex items-center justify-between">
        <span className="text-xs font-semibold text-gray-500 uppercase tracking-wide">Autonomy</span>
        <select
          value={autonomyLevel}
          onChange={(e) => onAutonomyChange(e.target.value)}
          className="text-xs bg-gray-50 border border-gray-200 text-gray-700 rounded-md px-2 py-1 outline-none focus:ring-1 focus:ring-blue-500"
        >
          <option value="Draft Only">Draft Only</option>
          <option value="Standard">Standard</option>
          <option value="Full Autopilot">Full Autopilot</option>
        </select>
      </div>
    </div>
  );
}
