"use client";

import React from 'react';
import { WithTooltip } from '../../../components/TooltipRegistry';

type Props = {
  name: string;
  pendingCount: number;
  onClick: () => void;
};

export default function DepartmentCard({ name, pendingCount, onClick }: Props) {
  const disabled = pendingCount === 0;

  return (
    <WithTooltip id="department-card-tooltip" defaultText={disabled ? "No pending approvals for this department." : "Click to view and manage pending approvals for this department."}>
    <button
      onClick={disabled ? undefined : onClick}
      disabled={disabled}
      aria-disabled={disabled}
      className={`w-full text-left bg-white backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-2xl p-5 flex items-center justify-between group mb-4 ${
        disabled
          ? 'opacity-60 cursor-not-allowed shadow-none'
          : 'shadow-sm hover:shadow-md transition-all active:scale-[0.98]'
      }`}
    >
      <div className="flex items-center gap-4">
        <div className={`w-12 h-12 rounded-full flex items-center justify-center shadow-inner flex-shrink-0 ${
          disabled ? 'bg-gray-100 border border-gray-200 text-gray-400' : 'bg-gradient-to-tr from-blue-100 to-blue-50 border border-blue-200/50'
        }`}>
           <span className={`text-xl font-bold font-outfit ${disabled ? 'text-gray-400' : 'text-[#0071E3]'}`}>{name.replace(/^The\s+/i, '').charAt(0).toUpperCase()}</span>
        </div>

        <div>
          <h3 className={`font-outfit font-semibold text-lg ${disabled ? 'text-gray-500' : 'text-gray-900'}`}>{name}</h3>
          {pendingCount > 0 ? (
            <p className="text-sm font-medium text-orange-600 mt-0.5">
              {pendingCount} item{pendingCount > 1 ? 's' : ''} awaiting approval
            </p>
          ) : (
            <p className="text-sm text-gray-500 mt-0.5">Active and running</p>
          )}
        </div>
      </div>

      <div className={`transition-colors ${disabled ? 'text-transparent' : 'text-gray-300 group-hover:text-[#0066FF]'}`}>
        {!disabled && (
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
        )}
      </div>
    </button>
    </WithTooltip>
  );
}
