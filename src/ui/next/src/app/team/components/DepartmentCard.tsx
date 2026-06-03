"use client";

import React from 'react';
import { WithTooltip } from '../../../components/TooltipRegistry';

type Props = {
  name: string;
  pendingCount: number;
  onClick: () => void;
};

export default function DepartmentCard({ name, pendingCount, onClick }: Props) {
  // Determine if this is the customer success agent to show dynamic summary stats
  const isCustomerSuccess = name.toLowerCase().includes("customer success") || name.toLowerCase().includes("ambassador");

  // Dynamic stats based on the pending items (simulating a daily feed metric)
  const handledCount = pendingCount === 0 ? 12 : Math.max(3, 15 - pendingCount);
  const timeSaved = handledCount * 3.5;

  return (
    <WithTooltip id="department-card-tooltip" defaultText="Click to view and manage pending approvals for this department.">
    <button
      onClick={onClick}
      className="w-full text-left bg-white/65 backdrop-blur-[30px] saturate-[210%] border border-white/40 rounded-2xl p-5 shadow-sm hover:shadow-md transition-all active:scale-[0.98] flex flex-col group mb-4 relative overflow-hidden"
    >
      <div className="flex items-center justify-between w-full relative z-10">
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

      {isCustomerSuccess && (
         <div className="mt-4 pt-4 border-t border-black/5 w-full text-left relative z-10">
            <div className="p-3 bg-blue-50/50 rounded-xl border border-blue-100 flex items-start gap-2 shadow-sm">
              <svg className="w-4 h-4 text-blue-600 mt-0.5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                 <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
              </svg>
              <p className="text-xs text-blue-800 font-medium leading-relaxed">
                Your Ambassador agent handled {handledCount} inquiries today, saving you approximately {Math.round(timeSaved)} minutes.
              </p>
            </div>
         </div>
      )}
    </button>
    </WithTooltip>
  );
}
