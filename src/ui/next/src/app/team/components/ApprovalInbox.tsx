"use client";

import React from "react";
import { ApprovalRequest } from "../page";
import ActionFeed from "./ActionFeed";

type Props = {
  departmentId: string;
  departmentName: string;
  approvals: ApprovalRequest[];
  onBack: () => void;
  onApprove: (id: string) => void;
  onReject: (id: string) => void;
  onEdit: (id: string) => void;
};

export default function ApprovalInbox({
  departmentName,
  approvals,
  onBack,
  onApprove,
  onReject,
  onEdit,
}: Props) {
  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 font-inter py-10">
      <div className="w-[375px] min-h-[812px] bg-gradient-to-br from-gray-50 to-gray-100 shadow-2xl overflow-hidden flex flex-col relative border-x border-gray-200">
        {/* Header */}
        <div className="pt-12 pb-6 px-6 bg-white/60 backdrop-blur-[30px] border-b border-white/40 sticky top-0 z-10 flex items-center gap-4">
          <button
            onClick={onBack}
            className="w-10 h-10 flex items-center justify-center rounded-full bg-white shadow-sm border border-gray-100 text-gray-500 hover:text-gray-900 transition-colors"
          >
            <svg
              className="w-5 h-5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </button>
          <div>
            <h1 className="text-2xl font-bold font-outfit text-gray-900 tracking-tight">
              {departmentName}
            </h1>
            <p className="text-gray-500 text-xs font-medium uppercase tracking-wider mt-1">
              Approval Inbox
            </p>
          </div>
        </div>

        {/* Content */}
        <ActionFeed approvals={approvals} onApprove={onApprove} onReject={onReject} onEdit={onEdit} />
      </div>
    </div>
  );
}
