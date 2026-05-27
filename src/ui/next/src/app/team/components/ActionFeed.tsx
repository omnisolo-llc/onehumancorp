"use client";

import React from 'react';
import ApprovalCard from './ApprovalCard';

type FeedItem = {
  type: 'completed';
  id: string;
  department: string;
  description: string;
  timestamp: string;
};

type ApprovalItem = {
  type: 'approval';
  id: string;
  department: string;
  description: string;
  title: string;
  body: string;
};

export type ActionItem = FeedItem | ApprovalItem;

type Props = {
  items: ActionItem[];
  onApprove: (id: string, newBody?: string) => void;
  onReject: (id: string) => void;
};

export default function ActionFeed({ items, onApprove, onReject }: Props) {
  return (
    <div className="flex flex-col gap-4">
      {items.map((item) => {
        if (item.type === 'completed') {
          return (
            <div key={item.id} className="bg-white/40 backdrop-blur-[20px] saturate-[210%] border border-white/40 rounded-2xl p-4 shadow-sm flex flex-col">
              <div className="flex items-center justify-between mb-2">
                <div className="flex items-center gap-2">
                  <span className="px-2 py-1 rounded-md text-[10px] font-bold uppercase tracking-wider bg-green-100 text-green-700">
                    Completed
                  </span>
                  <span className="text-xs text-gray-500 font-medium">{item.department}</span>
                </div>
                <span className="text-xs text-gray-400">{item.timestamp}</span>
              </div>
              <p className="text-sm text-gray-800 font-medium">{item.description}</p>
            </div>
          );
        } else {
          return (
            <ApprovalCard
              key={item.id}
              id={item.id}
              department={item.department}
              title={item.title}
              body={item.body}
              onApprove={onApprove}
              onReject={onReject}
            />
          );
        }
      })}
      {items.length === 0 && (
        <div className="text-center text-sm text-gray-500 py-4">No recent actions.</div>
      )}
    </div>
  );
}
