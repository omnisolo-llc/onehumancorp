"use client";

import React, { useState } from 'react';
import { ApprovalRequest } from '../page';

type Props = {
  request: ApprovalRequest;
  onApprove: (id: string, newDescription?: string) => void;
  onReject: (id: string) => void;
};

export default function ApprovalCard({ request, onApprove, onReject }: Props) {
  const [isEditing, setIsEditing] = useState(false);
  const [editedDescription, setEditedDescription] = useState(request.description);

  const handleApprove = () => {
    onApprove(request.id, editedDescription !== request.description ? editedDescription : undefined);
    setIsEditing(false);
  };

  return (
    <div className="bg-white/65 backdrop-blur-[20px] rounded-2xl p-5 shadow-[0_4px_20px_-4px_rgba(0,0,0,0.05)] border border-white/40 mb-4">
      <div className="flex items-center gap-2 mb-3">
        <span className={`px-2 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider ${
          request.action_risk.toLowerCase() === 'high'
            ? 'bg-orange-100 text-orange-700'
            : 'bg-blue-100 text-blue-700'
        }`}>
          {request.action_risk} Risk
        </span>
        <span className="text-xs text-gray-500 font-medium">{request.department}</span>
      </div>

      <h4 className="font-outfit font-semibold text-gray-900 text-sm mb-2">
        {request.department} drafted a response
      </h4>

      {isEditing ? (
        <textarea
          className="w-full text-gray-800 text-sm leading-relaxed mb-4 font-medium bg-white/50 border border-gray-200 rounded-xl p-3 focus:outline-none focus:ring-2 focus:ring-blue-500"
          value={editedDescription}
          onChange={(e) => setEditedDescription(e.target.value)}
          rows={4}
        />
      ) : (
        <p className="text-gray-800 text-sm leading-relaxed mb-6 font-medium bg-white/30 p-3 rounded-xl border border-white/20">
          {editedDescription}
        </p>
      )}

      <div className="flex gap-2">
        {isEditing ? (
          <>
            <button
              onClick={() => setIsEditing(false)}
              className="flex-1 py-2 px-3 rounded-xl font-semibold text-xs bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-[0.98] transition-all"
            >
              Cancel
            </button>
            <button
              onClick={handleApprove}
              className="flex-1 py-2 px-3 rounded-xl font-bold text-xs bg-blue-600 text-white hover:bg-blue-700 shadow-sm active:scale-[0.98] transition-all"
            >
              Save & Approve
            </button>
          </>
        ) : (
          <>
            <button
              onClick={() => onReject(request.id)}
              className="flex-1 py-2 px-3 rounded-xl font-semibold text-xs bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-[0.98] transition-all"
            >
              Reject
            </button>
            <button
              onClick={() => setIsEditing(true)}
              className="flex-1 py-2 px-3 rounded-xl font-semibold text-xs bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-[0.98] transition-all"
            >
              Edit
            </button>
            <button
              onClick={handleApprove}
              className="flex-[2] py-2 px-3 rounded-xl font-bold text-xs bg-blue-600 text-white hover:bg-blue-700 shadow-sm shadow-blue-500/20 active:scale-[0.98] transition-all"
            >
              Approve & Send
            </button>
          </>
        )}
      </div>
    </div>
  );
}
