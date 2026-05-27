"use client";

import React, { useState } from 'react';

type Props = {
  id: string;
  department: string;
  title: string;
  body: string;
  onApprove: (id: string, newBody?: string) => void;
  onReject: (id: string) => void;
};

export default function ApprovalCard({ id, department, title, body, onApprove, onReject }: Props) {
  const [isEditing, setIsEditing] = useState(false);
  const [editBody, setEditBody] = useState(body);

  return (
    <div className="bg-white/65 backdrop-blur-[20px] saturate-[210%] border border-white/40 rounded-2xl p-5 shadow-sm hover:shadow-md transition-all flex flex-col mb-4">
      <div className="flex items-center gap-2 mb-2">
        <span className="px-2 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider bg-orange-100 text-orange-700">
          Requires Approval
        </span>
        <span className="text-xs text-gray-500 font-medium">{department}</span>
      </div>

      <h4 className="font-outfit font-semibold text-gray-900 text-sm mb-2">{title}</h4>

      {isEditing ? (
        <textarea
          className="w-full text-sm text-gray-700 bg-white/50 border border-gray-200 rounded-lg p-2 mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500"
          rows={3}
          value={editBody}
          onChange={(e) => setEditBody(e.target.value)}
        />
      ) : (
        <p className="text-sm text-gray-700 mb-4 bg-white/40 p-3 rounded-lg italic">"{body}"</p>
      )}

      <div className="flex gap-2 mt-auto">
        {isEditing ? (
          <>
            <button
              onClick={() => setIsEditing(false)}
              className="flex-1 py-2 px-3 rounded-xl font-semibold text-xs bg-gray-100 text-gray-700 hover:bg-gray-200 transition-colors"
            >
              Cancel
            </button>
            <button
              onClick={() => {
                setIsEditing(false);
                onApprove(id, editBody);
              }}
              className="flex-1 py-2 px-3 rounded-xl font-bold text-xs bg-blue-600 text-white hover:bg-blue-700 transition-colors"
            >
              Save & Send
            </button>
          </>
        ) : (
          <>
            <button
              onClick={() => onReject(id)}
              className="flex-1 py-2 px-3 rounded-xl font-semibold text-xs bg-gray-100 text-gray-700 hover:bg-gray-200 transition-colors"
            >
              Reject
            </button>
            <button
              onClick={() => setIsEditing(true)}
              className="flex-1 py-2 px-3 rounded-xl font-semibold text-xs bg-gray-100 text-gray-700 hover:bg-gray-200 transition-colors"
            >
              Edit
            </button>
            <button
              onClick={() => onApprove(id, editBody)}
              className="flex-1 py-2 px-3 rounded-xl font-bold text-xs bg-blue-600 text-white hover:bg-blue-700 transition-colors"
            >
              Approve & Send
            </button>
          </>
        )}
      </div>
    </div>
  );
}
