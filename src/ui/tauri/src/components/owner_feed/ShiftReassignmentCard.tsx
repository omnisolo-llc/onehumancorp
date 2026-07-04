import React from 'react';

type AgentFeedItem = {
  id: string;
  tenant_id?: string;
  event_source: string;
  context_payload: any;
  proposed_action: any;
  lifecycle_state: string;
  created_at: string;
  updated_at?: string;
};

interface ShiftReassignmentCardProps {
  approval: AgentFeedItem;
  queuedActionIds: Set<string>;
  handleDecision: (
    id: string,
    approved: boolean,
    editContent?: string,
    event_source?: string,
  ) => void;
}

export const ShiftReassignmentCard: React.FC<ShiftReassignmentCardProps> = ({
  approval,
  queuedActionIds,
  handleDecision,
}) => {
  const isQueued = queuedActionIds.has(approval.id);
  const context = typeof approval.context_payload === "string"
      ? JSON.parse(approval.context_payload)
      : approval.context_payload;
  const proposedAction = typeof approval.proposed_action === "string"
      ? JSON.parse(approval.proposed_action)
      : approval.proposed_action;

  let shiftContext = context?.context || "Action Required: Shift Coverage";

  let newStaffName = "a replacement";
  let originalStaffName = "a staff member";

  if (proposedAction && typeof proposedAction === "object") {
     if (proposedAction.new_staff_name) {
       newStaffName = proposedAction.new_staff_name;
     }
     if (proposedAction.original_staff_name) {
       originalStaffName = proposedAction.original_staff_name;
     }
  }

  const proposalText = \`Reassign shift to \${newStaffName}?\`;

  return (
    <div
      data-testid="shift-reassignment-card"
      className="bg-[rgba(255,255,255,0.7)] dark:bg-[rgba(22,22,26,0.75)] backdrop-blur-[40px] backdrop-saturate-[200%] border border-[rgba(0,0,0,0.1)] dark:border-[rgba(255,255,255,0.1)] p-5 rounded-[24px] shadow-[0_8px_30px_rgb(0,0,0,0.04)] dark:shadow-[0_8px_30px_rgb(0,0,0,0.12)] flex flex-col gap-4 relative overflow-hidden transition-all duration-300"
    >
      <div className="absolute top-0 right-0 w-32 h-32 bg-gradient-to-br from-[#FF9500]/20 to-[#FF2D55]/20 rounded-full blur-3xl -mr-10 -mt-10 pointer-events-none" />

      <div className="flex flex-col gap-1 z-10">
        <span className="text-xs font-bold font-outfit uppercase tracking-widest text-[#FF9500] dark:text-[#FFA733] mb-1">
          Shift Reassignment
        </span>
        <h3 className="text-xl font-bold font-outfit text-[#1D1D1F] dark:text-[#F5F5F7] leading-tight">
          Action Required: Shift Coverage
        </h3>
        <div className="text-sm text-[#86868B] dark:text-[#98989D] mt-2 bg-white/50 dark:bg-black/20 p-3 rounded-xl border border-black/5 dark:border-white/5">
           <span className="font-semibold block mb-1">Context:</span>
           {shiftContext}
        </div>
      </div>

      <div className="z-10 mt-2 bg-[#F5F5F7]/80 dark:bg-[#1C1C1E]/80 backdrop-blur-md p-4 rounded-xl border border-[rgba(0,0,0,0.05)] dark:border-[rgba(255,255,255,0.05)]">
         <span className="font-semibold text-sm text-[#1D1D1F] dark:text-[#F5F5F7] block mb-2">AI Proposal:</span>
         <p className="text-base text-[#1D1D1F] dark:text-[#F5F5F7]">
            {proposalText}
         </p>
      </div>

      <div className="flex gap-3 z-10 mt-2 pt-2">
        <button
          disabled={isQueued}
          onClick={() => handleDecision(approval.id, true, undefined, "shift_reassignment")}
          className="flex-1 bg-[#34C759] hover:bg-[#30D158] text-white px-4 py-3 rounded-xl font-semibold font-outfit text-sm transition-all duration-200 shadow-sm active:scale-[0.98] disabled:opacity-50 min-h-[44px]"
        >
          {isQueued ? "Syncing..." : "Approve & Notify"}
        </button>
        <button
          disabled={isQueued}
          onClick={() => handleDecision(approval.id, false, undefined, "shift_reassignment")}
          className="flex-1 bg-white dark:bg-[#2C2C2E] hover:bg-gray-50 dark:hover:bg-[#3A3A3C] text-[#1D1D1F] dark:text-[#F5F5F7] border border-[rgba(0,0,0,0.1)] dark:border-[rgba(255,255,255,0.1)] px-4 py-3 rounded-xl font-semibold font-outfit text-sm transition-all duration-200 active:scale-[0.98] disabled:opacity-50 min-h-[44px]"
        >
          Find Someone Else
        </button>
      </div>
    </div>
  );
};
