"use client";

import React, { useState } from 'react';

export interface AgentFeedItem {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload?: any;
  proposed_action?: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
}

interface AgentActionCardProps {
  item: AgentFeedItem;
  isProcessing: boolean;
  editingId: string | null;
  editValue: string;
  onEditValueChange: (value: string) => void;
  onStartEditing: (item: AgentFeedItem) => void;
  onSaveEdit: (id: string) => void;
  onCancelEdit: () => void;
  onAction: (id: string, action: string) => void;
}

export function AgentActionCard({
  item,
  isProcessing,
  editingId,
  editValue,
  onEditValueChange,
  onStartEditing,
  onSaveEdit,
  onCancelEdit,
  onAction
}: AgentActionCardProps) {
  const isAmbassador = item.proposed_action?.feature_type === 'ambassador_reply' || item.context_payload?.feature_type === 'ambassador_reply';
  const ambassadorPayload = isAmbassador ? (item.proposed_action || item.context_payload) : null;
  const isDisputeResolution = item.proposed_action?.feature_type === 'dispute_resolution' || item.context_payload?.feature_type === 'dispute_resolution';
  const disputePayload = isDisputeResolution ? (item.proposed_action || item.context_payload) : null;
  const isEditing = editingId === item.id;

  return (
    <div
      className="bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] backdrop-saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] p-5 rounded-[16px] shadow-sm flex flex-col gap-3"
      data-testid="agent-feed-card"
    >
      <div className="flex items-center gap-3">
        <div className={`w-10 h-10 rounded-full flex items-center justify-center shrink-0 ${isDisputeResolution ? 'bg-[#FFF5E5] dark:bg-[#FF9500]/20' : isAmbassador ? 'bg-[#E5F0FF] dark:bg-[#0066FF]/20' : 'bg-blue-100 dark:bg-blue-900/40'}`}>
          {isDisputeResolution ? (
            <svg className="w-5 h-5 text-[#FF9500]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" /></svg>
          ) : isAmbassador ? (
            <svg className="w-5 h-5 text-[#0066FF]" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" /></svg>
          ) : (
            <svg className="w-5 h-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" /></svg>
          )}
        </div>
        <div>
          <h3 className="text-[15px] font-semibold text-gray-900 dark:text-white">
            {isDisputeResolution ? 'Urgent: Dispute Raised' : isAmbassador ? 'Message Drafted' : (item.proposed_action?.action_type || 'Action Proposed')}
          </h3>
          <p className="text-[12px] text-gray-500 font-medium">
            {isDisputeResolution ? 'Requires Immediate Review' : 'Agent pending your approval'}
          </p>
        </div>
      </div>

      {isEditing ? (
        <div className="flex flex-col gap-3">
          <textarea
            value={editValue}
            onChange={(e) => onEditValueChange(e.target.value)}
            className="w-full min-h-[100px] p-3 rounded-[8px] border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-800 text-[#1D1D1F] dark:text-[#F5F5F7] text-sm focus:ring-2 focus:ring-[#0066FF] outline-none transition-all resize-none"
            data-testid={isAmbassador ? "edit-ambassador-reply-textarea" : "edit-proposal-textarea"}
          />
          <div className="flex gap-3">
            <button
              onClick={() => onSaveEdit(item.id)}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              data-testid={isAmbassador ? "save-send-ambassador-reply" : "save-proposal"}
            >
              Save & Approve
            </button>
            <button
              onClick={onCancelEdit}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              data-testid="cancel-edit-proposal"
            >
              Cancel
            </button>
          </div>
        </div>
      ) : (
        <div className="flex flex-col gap-3">
          {isDisputeResolution ? (
            <div className="flex flex-col gap-3">
              <div className="bg-[#FFF5E5] dark:bg-[#FF9500]/10 border border-[#FFD699] dark:border-[#FF9500]/30 p-3 rounded-lg">
                <div className="flex justify-between items-start mb-2">
                  <span className="text-[12px] font-bold text-[#E68A00] dark:text-[#FF9500] uppercase tracking-wider">Dispute Details</span>
                  <span className="text-[14px] font-bold text-gray-900 dark:text-white">${disputePayload?.amount}</span>
                </div>
                <p className="text-[13px] text-gray-800 dark:text-gray-200 leading-relaxed mb-3">"{disputePayload?.customer_claim}"</p>
                <div className="bg-white dark:bg-gray-800 rounded-md overflow-hidden border border-[#FFD699] dark:border-[#FF9500]/30">
                  <div className="bg-gray-50 dark:bg-gray-700/50 px-3 py-2 border-b border-gray-100 dark:border-gray-600">
                    <span className="text-[11px] font-bold text-gray-500 uppercase tracking-wider">Suggested Resolution</span>
                  </div>
                  {disputePayload?.refund_amount && (
                    <div className="flex items-center gap-3 p-3 border-b border-gray-100 dark:border-gray-600">
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
                <p className="text-[13px] text-gray-700 dark:text-gray-300 italic mb-1">"{ambassadorPayload.original_message}"</p>
                {ambassadorPayload.past_orders && (
                  <span className="inline-block text-[10px] font-semibold text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/30 px-2 py-0.5 rounded-full mt-1">
                    {ambassadorPayload.past_orders}
                  </span>
                )}
              </div>
              <div>
                <p className="text-[11px] font-bold text-gray-500 uppercase mb-1">Agent Draft</p>
                <p className="text-[13px] text-gray-900 dark:text-white leading-relaxed">
                  {ambassadorPayload.generated_response}
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

      {!isEditing ? (
        isDisputeResolution ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() => onAction(item.id, 'APPROVED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#FF9500] text-white font-medium hover:bg-[#E68A00] transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve & Resolve"
              data-testid="feed-approve-resolve-btn"
            >
              {isProcessing ? 'Processing...' : 'Approve & Resolve'}
            </button>
            <button
              onClick={() => onStartEditing(item)}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Edit Draft"
              data-testid="feed-edit-btn"
            >
              Edit
            </button>
            <button
              onClick={() => onAction(item.id, 'DISMISSED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss Draft"
              data-testid="feed-dismiss-btn"
            >
              Dismiss
            </button>
          </div>
        ) : isAmbassador ? (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() => onAction(item.id, 'APPROVED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              aria-label="Approve & Send Draft"
              data-testid="feed-approve-btn"
            >
              {isProcessing ? 'Processing...' : 'Send Draft'}
            </button>
            <button
              onClick={() => onStartEditing(item)}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Edit Draft"
              data-testid="feed-edit-btn"
            >
              Edit
            </button>
            <button
              onClick={() => onAction(item.id, 'DISMISSED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              aria-label="Dismiss Draft"
              data-testid="feed-dismiss-btn"
            >
              Dismiss
            </button>
          </div>
        ) : (
          <div className="flex flex-col sm:flex-row gap-3 w-full">
            <button
              onClick={() => onAction(item.id, 'APPROVED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
              data-testid="feed-approve-btn"
            >
              {isProcessing ? 'Processing...' : item.proposed_action?.action_type === 'Draft Quote' ? 'Review Estimate' : item.proposed_action?.action_type === 'Draft Follow-up' ? 'Send Follow-up' : item.proposed_action?.action_type === 'Draft Booking' ? 'Approve & Confirm' : 'Approve'}
            </button>
            <button
              onClick={() => onStartEditing(item)}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              data-testid="feed-edit-btn"
            >
              Edit
            </button>
            <button
              onClick={() => onAction(item.id, 'DISMISSED')}
              disabled={isProcessing}
              className="flex-1 min-h-[44px] min-w-[44px] px-4 rounded-[8px] border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
              data-testid="feed-dismiss-btn"
            >
              Dismiss
            </button>
          </div>
        )
      ) : null}
    </div>
  );
}
