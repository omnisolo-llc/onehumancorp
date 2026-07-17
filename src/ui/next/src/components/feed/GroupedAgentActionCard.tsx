import React, { useState } from "react";
import { AgentActionCard } from "./AgentActionCard";

export const GroupedAgentActionCard = ({
  groupKey,
  title,
  items,
  queuedActionIds,
  editingId,
  editContent,
  editQuotePrice,
  editQuoteScope,
  setEditingId,
  setEditContent,
  setEditQuotePrice,
  setEditQuoteScope,
  handleDecision,
}: any) => {
  const [isExpanded, setIsExpanded] = useState(false);

  const handleApproveAll = () => {
    items.forEach((item: any) => {
      handleDecision(item.id, true, undefined, item.event_source);
    });
  };

  return (
    <div
      className="glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-5 shadow-sm flex flex-col gap-4 transition-all duration-300 overflow-hidden break-words whitespace-normal"
      data-testid={`grouped-triage-card-${groupKey}`}
    >
      <div className="flex flex-col gap-1">
        <div className="flex items-center justify-between gap-2">
          <div className="flex flex-wrap items-center gap-2">
             <span className="text-xs font-bold uppercase tracking-wider text-[#0066FF] bg-[#0066FF]/10 dark:bg-[#0066FF]/20 px-2 py-1 rounded-[8px]">
               Grouped Actions
             </span>
          </div>
          <span className="text-sm font-semibold font-outfit px-2 py-1 bg-gray-100 dark:bg-gray-800 rounded-full text-gray-700 dark:text-gray-300">
            {items.length} items
          </span>
        </div>
        <h3 className="text-lg font-semibold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] leading-snug mt-1 tracking-wide">
          {items.length} new {title}
        </h3>
        <p className="text-sm text-gray-600 dark:text-gray-400 mt-1 break-words">
          The AI has drafted responses and actions for these related items. Review and approve all with one tap, or expand to view individually.
        </p>
      </div>

      <div className="flex flex-col sm:flex-row gap-3 w-full mt-2">
        <button
          onClick={handleApproveAll}
          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-green-500 text-white font-medium hover:bg-green-600 transition-all duration-200 shadow-md flex items-center justify-center"
          aria-label="Approve All"
          data-testid={`approve-all-${groupKey}`}
        >
          Approve All
        </button>
        <button
          onClick={() => setIsExpanded(!isExpanded)}
          className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
          aria-label={isExpanded ? "Collapse" : "Review Individually"}
          data-testid={`expand-${groupKey}`}
        >
          {isExpanded ? "Collapse" : "Review Individually"}
        </button>
      </div>

      {isExpanded && (
        <div className="flex flex-col gap-4 mt-4 pt-4 border-t border-gray-200 dark:border-gray-700" data-testid={`expanded-items-${groupKey}`}>
          {items.map((approval: any) => (
            <AgentActionCard
              key={approval.id}
              approval={approval}
              queuedActionIds={queuedActionIds}
              editingId={editingId}
              editContent={editContent}
              editQuotePrice={editQuotePrice}
              editQuoteScope={editQuoteScope}
              setEditingId={setEditingId}
              setEditContent={setEditContent}
              setEditQuotePrice={setEditQuotePrice}
              setEditQuoteScope={setEditQuoteScope}
              handleDecision={handleDecision}
            />
          ))}
        </div>
      )}
    </div>
  );
};
