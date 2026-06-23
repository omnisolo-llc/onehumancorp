import React, { useState } from 'react';

type AgentFeedItem = {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload: any;
  proposed_action: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
};

interface AgentActionCardProps {
  item: AgentFeedItem;
  isProcessing: boolean;
  editingId: string | null;
  editContent: string;
  setEditContent: (val: string) => void;
  startEditing: (item: AgentFeedItem) => void;
  saveEdit: (id: string) => void;
  cancelEdit: () => void;
  handleAction: (id: string, action: string) => void;
}

export type { AgentFeedItem };
export function AgentActionCard({
  item,
  isProcessing,
  editingId,
  editContent,
  setEditContent,
  startEditing,
  saveEdit,
  cancelEdit,
  handleAction
}: AgentActionCardProps) {
  const isDisputeResolution = item.event_source === 'CUSTOMER_DISPUTE';
  const isAmbassador = item.event_source === 'AMBASSADOR_REPLY';

  const disputePayload = isDisputeResolution ? item.proposed_action?.payload || item.context_payload : null;
  const ambassadorPayload = isAmbassador ? item.proposed_action?.payload || item.context_payload : null;

  return (
    <div
      className={`glassmorphism p-5 relative overflow-hidden transition-all duration-300 rounded-[16px] backdrop-blur-[30px] backdrop-saturate-[210%] ${isProcessing ? 'opacity-50 scale-[0.98]' : 'animate-fade-in'}`}
      data-testid="agent-feed-card"
    >
      <div className="flex justify-between items-start mb-3">
        <span className={`text-[11px] font-bold uppercase tracking-wider ${isDisputeResolution ? 'text-[#FF9500] dark:text-[#FF9F0A]' : 'text-[#0066FF] dark:text-[#0071E3]'} flex items-center gap-1.5`}>
          <span className={`w-2 h-2 rounded-full ${isDisputeResolution ? 'bg-[#FF9500] dark:bg-[#FF9F0A]' : 'bg-[#0066FF] dark:bg-[#0071E3]'} opacity-80`}></span>
          {isDisputeResolution ? 'DISPUTE RESOLUTION' : isAmbassador ? 'CUSTOMER MESSAGE' : item.proposed_action?.action_type === 'Draft Quote' ? 'SMART ESTIMATE' : item.proposed_action?.action_type === 'Draft Follow-up' ? 'DEPOSIT FOLLOW-UP' : item.proposed_action?.action_type === 'Draft Booking' ? 'NEW BOOKING REQUEST' : item.event_source.replace(/_/g, ' ')}
        </span>
        <span className="text-[11px] text-gray-400 font-medium">
          {new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
        </span>
      </div>

      <h3 className="font-bold text-gray-900 dark:text-white text-[15px] mb-2 leading-snug">
        {isDisputeResolution
          ? `Dispute from ${disputePayload?.sender_id || 'Customer'}`
          : isAmbassador
          ? `New Message from ${ambassadorPayload?.sender_id || 'Customer'}`
          : item.proposed_action?.action_type === 'Draft Quote'
          ? `Drafted Estimate for ${item.context_payload?.customer_name || 'Customer'}`
          : item.proposed_action?.action_type === 'Draft Follow-up'
          ? `Unpaid Deposit: ${item.context_payload?.customer_name || 'Customer'}`
          : item.proposed_action?.action_type === 'Draft Booking'
          ? `Tentative Booking: ${item.context_payload?.customer_name || 'Customer'}`
          : item.proposed_action?.message || item.proposed_action?.action_type || 'Action Required'}
      </h3>

      {editingId === item.id ? (
        <div className="mb-4">
          <textarea
            className="w-full min-h-[44px] p-3 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none mb-3"
            rows={4}
            value={editContent}
            onChange={(e) => setEditContent(e.target.value)}
            data-testid="feed-edit-textarea"
            autoFocus
          />
          <div className="flex gap-3">
            <button
              onClick={() => saveEdit(item.id)}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              data-testid="feed-save-btn"
            >
              {isProcessing ? 'Saving...' : 'Save & Send'}
            </button>
            <button
              onClick={cancelEdit}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              data-testid="feed-cancel-btn"
            >
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <div className="mb-5">
          {isDisputeResolution ? (
            <div className="flex flex-col gap-3">
              <div className="bg-[#FFF5E5] dark:bg-[rgba(255,149,0,0.1)] p-3 rounded-lg border border-[#FFD699] dark:border-[rgba(255,149,0,0.3)]">
                <p className="text-[13px] text-[#8C5300] dark:text-[#FF9F0A] italic mb-1">"{disputePayload?.original_message}"</p>
                {disputePayload?.past_orders && (
                  <span className="inline-block text-[10px] font-semibold text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/30 px-2 py-0.5 rounded-full mt-1">
                    {disputePayload?.past_orders}
                  </span>
                )}
              </div>
              <div>
                <p className="text-[11px] font-bold text-gray-500 uppercase mb-1">Proposed Resolution</p>
                <p className="text-[13px] text-gray-900 dark:text-white leading-relaxed mb-3">
                  {disputePayload?.generated_response}
                </p>
                <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 overflow-hidden">
                  {disputePayload?.refund_amount && (
                    <div className="flex items-center gap-3 p-3 border-b border-gray-100 dark:border-gray-700">
                      <input type="checkbox" defaultChecked className="w-4 h-4 text-[#FF9500] rounded border-gray-300 focus:ring-[#FF9500]" />
                      <span className="text-[13px] text-gray-800 dark:text-gray-200 font-medium">Issue ${disputePayload?.refund_amount} Refund</span>
                    </div>
                  )}
                  {disputePayload?.operational_action && (
                    <div className="flex items-center gap-3 p-3">
                      <input type="checkbox" defaultChecked className="w-4 h-4 text-[#FF9500] rounded border-gray-300 focus:ring-[#FF9500]" />
                      <span className="text-[13px] text-gray-800 dark:text-gray-200 font-medium">{disputePayload?.operational_action}</span>
                    </div>
                  )}
                </div>
              </div>
            </div>
          ) : isAmbassador ? (
            <div className="flex flex-col gap-3">
              <div className="bg-gray-50 dark:bg-gray-800 p-3 rounded-lg border border-gray-100 dark:border-gray-700">
                <p className="text-[13px] text-gray-700 dark:text-gray-300 italic mb-1">"{ambassadorPayload?.original_message}"</p>
                {ambassadorPayload?.past_orders && (
                  <span className="inline-block text-[10px] font-semibold text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/30 px-2 py-0.5 rounded-full mt-1">
                    {ambassadorPayload?.past_orders}
                  </span>
                )}
              </div>
              <div>
                <p className="text-[11px] font-bold text-gray-500 uppercase mb-1">Agent Draft</p>
                <p className="text-[13px] text-gray-900 dark:text-white leading-relaxed">
                  {ambassadorPayload?.generated_response}
                </p>
              </div>
            </div>
          ) : (
            <p className="text-[13px] text-gray-600 dark:text-gray-300 leading-relaxed mb-2">
              {item.proposed_action?.action_type === 'Draft Quote'
                ? (item.context_payload?.context || 'AI has drafted a new estimate based on recent customer inquiry.')
                : item.proposed_action?.action_type === 'Draft Booking'
                ? (item.context_payload?.context || 'AI has locked in a tentative time slot based on recent customer inquiry.')
                : (item.context_payload?.summary || item.proposed_action?.description || 'A new update requires your attention.')}
            </p>
          )}
        </div>
      )}

      {(!editingId || editingId !== item.id) && (
        isDisputeResolution ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() => handleAction(item.id, 'APPROVED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] bg-[#FF9500] text-white font-medium hover:bg-[#E68A00] transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve & Resolve"
              data-testid="feed-approve-resolve-btn"
            >
              {isProcessing ? 'Processing...' : 'Approve & Resolve'}
            </button>
            <button
              onClick={() => startEditing(item)}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Edit Draft"
              data-testid="feed-edit-btn"
            >
              Edit
            </button>
            <button
              onClick={() => handleAction(item.id, 'DISMISSED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss Draft"
              data-testid="feed-dismiss-btn"
            >
              Dismiss
            </button>
          </div>
        ) : isAmbassador ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() => handleAction(item.id, 'APPROVED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve & Send Draft"
              data-testid="feed-approve-btn"
            >
              {isProcessing ? 'Processing...' : 'Send Draft'}
            </button>
            <button
              onClick={() => startEditing(item)}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Edit Draft"
              data-testid="feed-edit-btn"
            >
              Edit
            </button>
            <button
              onClick={() => handleAction(item.id, 'DISMISSED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss Draft"
              data-testid="feed-dismiss-btn"
            >
              Dismiss
            </button>
          </div>
        ) : (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() => handleAction(item.id, 'APPROVED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              data-testid="feed-approve-btn"
            >
              {isProcessing ? 'Processing...' : item.proposed_action?.action_type === 'Draft Quote' ? 'Review Estimate' : item.proposed_action?.action_type === 'Draft Follow-up' ? 'Send Follow-up' : item.proposed_action?.action_type === 'Draft Booking' ? 'Approve & Confirm' : 'Approve'}
            </button>
            <button
              onClick={() => startEditing(item)}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              data-testid="feed-edit-btn"
            >
              Edit
            </button>
            <button
              onClick={() => handleAction(item.id, 'DISMISSED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[16px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              data-testid="feed-dismiss-btn"
            >
              Dismiss
            </button>
          </div>
        )
      )}
    </div>
  );
}
