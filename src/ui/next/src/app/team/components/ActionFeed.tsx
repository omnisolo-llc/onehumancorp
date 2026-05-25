"use client";

import React from "react";
import { ApprovalRequest } from "../page";
import ApprovalCard from "./ApprovalCard";

type Props = {
  approvals: ApprovalRequest[];
  onApprove: (id: string) => void;
  onReject: (id: string) => void;
  onEdit: (id: string) => void;
};

export default function ActionFeed({ approvals, onApprove, onReject, onEdit }: Props) {
  return (
    <div className="flex-1 overflow-y-auto px-4 py-6 pb-24 space-y-4 hide-scrollbar">
      {approvals.length === 0 ? (
        <div className="flex flex-col items-center justify-center h-64 text-center px-8">
          <div className="w-16 h-16 bg-green-50 text-green-500 rounded-full flex items-center justify-center mb-4">
            <svg
              className="w-8 h-8"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M5 13l4 4L19 7"
              />
            </svg>
          </div>
          <h3 className="font-outfit font-bold text-gray-900 text-lg mb-2">
            All Caught Up!
          </h3>
          <p className="text-sm text-gray-500">
            There are no pending actions requiring your review.
          </p>
        </div>
      ) : (
        approvals.map((req) => (
          <ApprovalCard
            key={req.id}
            req={req}
            onApprove={onApprove}
            onReject={onReject}
            onEdit={onEdit}
          />
        ))
      )}
    </div>
  );
}
