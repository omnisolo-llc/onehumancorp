"use client";

import React, { useState } from 'react';
import DepartmentSettingsModal from './DepartmentSettingsModal';

type ApprovalInboxProps = {
  departmentId: string;
  departmentName: string;
  approvals: any[];
  onBack: () => void;
  onApprove: (id: string) => void;
  onReject: (id: string) => void;
};

export default function ApprovalInbox({
  departmentId,
  departmentName,
  approvals,
  onBack,
  onApprove,
  onReject
}: ApprovalInboxProps) {
  const [showSettings, setShowSettings] = useState(false);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] max-w-[375px] min-h-[812px] bg-gray-50 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">

        {/* Header */}
        <div className="pt-12 pb-4 px-4 bg-white border-b border-gray-100 flex items-center justify-between sticky top-0 z-10">
          <div className="flex items-center gap-3">
            <button
              onClick={onBack}
              className="p-2 -ml-2 rounded-full hover:bg-gray-100 transition-colors"
            >
              <svg className="w-5 h-5 text-gray-700" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 19l-7-7 7-7" /></svg>
            </button>
            <h1 className="text-xl font-bold font-outfit text-gray-900">{departmentName}</h1>
          </div>
          <button
            onClick={() => setShowSettings(true)}
            className="p-2 rounded-full hover:bg-gray-100 transition-colors text-gray-600"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" /><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" /></svg>
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto px-4 py-6 bg-gray-50 hide-scrollbar">
          {approvals.length === 0 ? (
            <div className="h-full flex flex-col items-center justify-center opacity-60 mt-20">
               <svg className="w-16 h-16 text-gray-300 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4" /></svg>
               <p className="text-gray-500 font-medium">All caught up!</p>
            </div>
          ) : (
            <div className="space-y-4">
              {approvals.map((req) => (
                <div key={req.id} className="bg-white rounded-2xl shadow-sm border border-gray-100 p-5">
                   <div className="flex items-center gap-2 mb-3">
                     <span className="w-2 h-2 rounded-full bg-blue-500 animate-pulse"></span>
                     <span className="text-xs font-semibold text-blue-600 uppercase tracking-wider">{req.action_risk === 'DraftForReview' || req.action_risk === 'HIGH' ? 'Needs Review' : 'Auto-Execute'}</span>
                   </div>
                   <h3 className="font-medium text-gray-900 leading-snug mb-2">{req.description}</h3>

                   {req.payload && req.payload.draft_reply && (
                     <div className="mt-3 p-3 bg-gray-50 rounded-xl border border-gray-100">
                       <p className="text-sm text-gray-600 italic">"{req.payload.draft_reply}"</p>
                     </div>
                   )}

                   <div className="flex gap-3 mt-5">
                      <button
                        onClick={() => onApprove(req.id)}
                        className="flex-1 bg-gray-900 hover:bg-black text-white text-sm font-medium py-2.5 rounded-xl transition-all"
                      >
                        Approve
                      </button>
                      <button
                        onClick={() => onReject(req.id)}
                        className="flex-1 bg-gray-100 hover:bg-gray-200 text-gray-700 text-sm font-medium py-2.5 rounded-xl transition-all"
                      >
                        Reject
                      </button>
                   </div>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
      {showSettings && (
        <DepartmentSettingsModal
          departmentId={departmentId}
          departmentName={departmentName}
          onClose={() => setShowSettings(false)}
        />
      )}
    </div>
  );
}
