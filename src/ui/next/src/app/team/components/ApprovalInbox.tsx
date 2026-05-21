"use client";

import React from 'react';
import { ApprovalRequest } from '../page';

type Props = {
  departmentId: string;
  departmentName: string;
  approvals: ApprovalRequest[];
  onBack: () => void;
  onApprove: (id: string) => void;
  onReject: (id: string) => void;
};

export default function ApprovalInbox({ departmentName, approvals, onBack, onApprove, onReject }: Props) {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/60 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button
            onClick={onBack}
            className="w-10 h-10 flex items-center justify-center rounded-full bg-white shadow-sm border border-gray-100 text-gray-500 hover:text-gray-900 transition-colors"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" />
            </svg>
          </button>
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">{departmentName}</h1>
            <p className="text-gray-500 text-xs font-medium uppercase tracking-wider mt-1">Approval Inbox</p>
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 space-y-4 hide-scrollbar">
          {approvals.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-64 text-center px-8">
               <div className="w-16 h-16 bg-green-50 text-green-500 rounded-full flex items-center justify-center mb-4">
                 <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                   <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                 </svg>
               </div>
               <h3 className="font-outfit font-bold text-gray-900 text-lg mb-2">All Caught Up!</h3>
               <p className="text-sm text-gray-500">There are no pending actions requiring your review.</p>
            </div>
          ) : (
            approvals.map(req => (
              <div key={req.id} className="bg-white rounded-2xl p-5 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] border border-gray-100">
                <div className="flex items-center gap-2 mb-3">
                  <span className={`px-2 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider ${
                    req.action_risk.toLowerCase() === 'high'
                      ? 'bg-orange-100 text-orange-700'
                      : 'bg-blue-100 text-blue-700'
                  }`}>
                    {req.action_risk} Risk
                  </span>
                  <span className="text-xs text-gray-400 font-medium">{req.status}</span>
                </div>

                <p className="text-gray-800 text-sm leading-relaxed mb-6 font-medium">
                  {req.description}
                </p>

                <div className="flex gap-3">
                  <button
                    onClick={() => onReject(req.id)}
                    className="flex-1 py-3 px-4 rounded-xl font-semibold text-sm bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-[0.98] transition-all"
                  >
                    Reject / Edit
                  </button>
                  <button
                    onClick={() => onApprove(req.id)}
                    className="flex-1 py-3 px-4 rounded-xl font-bold text-sm bg-blue-600 text-white hover:bg-blue-700 shadow-md shadow-blue-500/20 active:scale-[0.98] transition-all"
                  >
                    Approve
                  </button>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
