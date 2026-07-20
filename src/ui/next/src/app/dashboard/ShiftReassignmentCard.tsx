import React from "react";

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

  const proposalText = `Reassign shift to ${newStaffName}?`;

  return (
    <div
      data-testid="shift-reassignment-card"
      className="bg-[rgba(255,255,255,0.75)] dark:bg-[rgba(22,22,26,0.8)] backdrop-blur-[30px] backdrop-saturate-[180%] border border-[rgba(0,0,0,0.08)] dark:border-[rgba(255,255,255,0.08)] p-6 rounded-[20px] shadow-sm flex flex-col gap-5 relative overflow-hidden transition-all duration-300 hover:shadow-md"
    >
      {/* Decorative gradient orb */}
      <div className="absolute top-0 right-0 w-36 h-36 bg-gradient-to-br from-orange-400/20 to-red-400/20 rounded-full blur-3xl -mr-12 -mt-12 pointer-events-none" />

      <div className="flex flex-col gap-1.5 z-10">
        <span className="text-[11px] font-bold font-outfit uppercase tracking-widest text-orange-500 dark:text-orange-400 mb-1">
          Shift Reassignment
        </span>
        <h3 className="text-xl font-bold font-outfit text-gray-900 dark:text-gray-100 leading-tight">
          Action Required: Shift Coverage
        </h3>
        <div className="text-sm text-gray-600 dark:text-gray-300 mt-2 bg-white/60 dark:bg-black/30 backdrop-blur-md p-3.5 rounded-xl border border-gray-200/50 dark:border-gray-700/50">
           <span className="font-semibold block mb-1.5">Context:</span>
           {shiftContext}
        </div>
      </div>

      <div className="z-10 mt-1 bg-gray-50/80 dark:bg-gray-800/80 backdrop-blur-md p-4 rounded-xl border border-gray-200/50 dark:border-gray-700/50">
         <span className="font-semibold text-sm text-gray-900 dark:text-gray-100 block mb-2">AI Proposal:</span>
         <p className="text-[15px] leading-relaxed text-gray-800 dark:text-gray-200">
            {proposalText}
         </p>
      </div>

      <div className="flex gap-3 z-10 mt-1 pt-1">
        <button
          disabled={isQueued}
          onClick={() => handleDecision(approval.id, true, undefined, "shift_reassignment")}
          className="flex-1 bg-green-500 hover:bg-green-600 text-white px-4 py-3.5 rounded-xl font-semibold text-sm transition-all shadow-sm active:scale-[0.98] disabled:opacity-50 min-h-[48px]"
        >
          {isQueued ? "Syncing..." : "Approve & Notify"}
        </button>
        <button
          disabled={isQueued}
          onClick={() => handleDecision(approval.id, false, undefined, "shift_reassignment")}
          className="flex-1 bg-white dark:bg-gray-800 hover:bg-gray-50 dark:hover:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-200 dark:border-gray-700 px-4 py-3.5 rounded-xl font-semibold text-sm transition-all active:scale-[0.98] disabled:opacity-50 min-h-[48px]"
        >
          Find Someone Else
        </button>
      </div>
    </div>
  );
};
