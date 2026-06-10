"use client";

import React, { useState } from "react";

export type AgentFeedItem = {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload: any;
  proposed_action: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
};

interface ActionCardProps {
  item: AgentFeedItem;
  onApprove: (id: string) => Promise<void>;
  onEdit: (id: string) => void;
  onDiscard: (id: string) => Promise<void>;
}

export const ActionCard: React.FC<ActionCardProps> = ({
  item,
  onApprove,
  onEdit,
  onDiscard,
}) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isActioning, setIsActioning] = useState(false);

  const getAgentTheme = (source: string) => {
    const s = source.toLowerCase();
    if (s.includes("ops") || s.includes("inventory") || s.includes("stock") || s.includes("supply")) {
      return {
        label: "Operations",
        icon: "📦",
        colorClass: "text-amber-600 dark:text-amber-400 bg-amber-50 dark:bg-amber-900/30",
        btnClass: "bg-amber-500 hover:bg-amber-600",
      };
    }
    if (s.includes("marketing") || s.includes("promoter") || s.includes("social")) {
      return {
        label: "Marketing",
        icon: "🚀",
        colorClass: "text-pink-600 dark:text-pink-400 bg-pink-50 dark:bg-pink-900/30",
        btnClass: "bg-gradient-to-r from-pink-500 to-indigo-500 hover:from-pink-600 hover:to-indigo-600",
      };
    }
    if (s.includes("cs") || s.includes("customer") || s.includes("message") || s.includes("inbox")) {
      return {
        label: "Customer Success",
        icon: "💬",
        colorClass: "text-[#0066FF] dark:text-blue-400 bg-[#0066FF]/10 dark:bg-blue-900/30",
        btnClass: "bg-[#0066FF] hover:bg-[#0052CC]",
      };
    }
    return {
      label: "Assistant",
      icon: "✨",
      colorClass: "text-indigo-600 dark:text-indigo-400 bg-indigo-50 dark:bg-indigo-900/30",
      btnClass: "bg-indigo-600 hover:bg-indigo-700",
    };
  };

  const theme = getAgentTheme(item.event_source);
  const message = item.proposed_action?.message || item.proposed_action?.action_type || item.event_source.replace("_", " ");

  const handleApprove = async () => {
    setIsActioning(true);
    await onApprove(item.id);
    setIsActioning(false);
  };

  const handleDiscard = async () => {
    setIsActioning(true);
    await onDiscard(item.id);
    setIsActioning(false);
  };

  return (
    <div
      className={`glassmorphism p-5 border border-white/40 dark:border-white/10 shadow-sm flex flex-col gap-4 transition-all duration-300 rounded-[16px] ${isActioning ? 'opacity-50 scale-95' : 'opacity-100'}`}
      data-testid={`action-card-${item.id}`}
    >
      <div className="flex flex-col gap-1">
        <div className="flex justify-between items-start gap-2">
          <div className="flex flex-wrap items-center gap-2">
            <span className={`text-[10px] font-bold uppercase tracking-wider px-2 py-1 rounded-md flex items-center gap-1.5 ${theme.colorClass}`}>
              <span>{theme.icon}</span> {theme.label}
            </span>
            {item.lifecycle_state === "PENDING_APPROVAL" && (
              <span className="text-[10px] font-bold uppercase tracking-wider text-red-600 bg-red-50 dark:bg-red-900/30 dark:text-red-400 px-2 py-1 rounded-md">
                Action Needed
              </span>
            )}
          </div>
          <span className="text-[10px] text-gray-500 font-inter shrink-0 mt-1">
            {new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
          </span>
        </div>

        <h3 className="text-[17px] font-semibold text-[#1D1D1F] dark:text-[#F5F5F7] font-outfit mt-2 leading-tight">
          {message}
        </h3>

        {isExpanded && (
          <div className="mt-3 p-3 bg-gray-50/50 dark:bg-black/20 rounded-xl border border-gray-100 dark:border-white/5 animate-fade-in">
             <div className="text-[11px] uppercase font-bold text-gray-400 mb-2">Context & Details</div>
             <div className="text-sm text-gray-700 dark:text-gray-300 space-y-2">
                {item.context_payload && Object.entries(item.context_payload).map(([key, val]) => (
                  typeof val !== 'object' && (
                    <div key={key} className="flex justify-between">
                      <span className="capitalize">{key.replace('_', ' ')}:</span>
                      <span className="font-medium">{String(val)}</span>
                    </div>
                  )
                ))}
                {(!item.context_payload || Object.keys(item.context_payload).length === 0) && (
                  <p className="italic text-xs">No additional context available.</p>
                )}
             </div>
          </div>
        )}
      </div>

      <div className="flex gap-2 w-full mt-2">
        <button
          onClick={handleApprove}
          disabled={isActioning}
          className={`flex-1 min-h-[44px] rounded-xl font-bold text-sm text-white shadow-sm transition-all active:scale-95 flex items-center justify-center ${theme.btnClass}`}
          aria-label="Approve"
        >
          Approve
        </button>
        <button
          onClick={() => {
            setIsExpanded(!isExpanded);
            onEdit(item.id);
          }}
          disabled={isActioning}
          className="flex-1 min-h-[44px] rounded-xl font-bold text-sm bg-white/80 dark:bg-white/10 hover:bg-white dark:hover:bg-white/20 text-[#1D1D1F] dark:text-[#F5F5F7] border border-gray-200 dark:border-white/10 transition-all active:scale-95 flex items-center justify-center"
          aria-label="Edit"
        >
          {isExpanded ? "Close" : "Edit"}
        </button>
        <button
          onClick={handleDiscard}
          disabled={isActioning}
          className="flex-1 min-h-[44px] rounded-xl font-bold text-sm bg-red-100/80 hover:bg-red-100 text-red-600 dark:bg-red-900/30 dark:hover:bg-red-900/50 dark:text-red-400 transition-all active:scale-95 flex items-center justify-center"
          aria-label="Discard"
        >
          Discard
        </button>
      </div>
    </div>
  );
};
