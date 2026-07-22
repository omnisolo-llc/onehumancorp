"use client";

import React, { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { useAgentWebSocket } from "../../hooks/useAgentWebSocket";

import { AmbassadorReplyCard } from "../dashboard/AmbassadorReplyCard";

interface FeedItem {
  id: string;
  tenant_id: string;
  event_source: string;
  context_payload?: any;
  proposed_action?: any;
  lifecycle_state: string;
  created_at: string;
  updated_at: string;
}

export default function FeedPage() {
  const router = useRouter();
  const [items, setItems] = useState<FeedItem[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [processingId, setProcessingId] = useState<string | null>(null);
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editValue, setEditValue] = useState<string>("");

  const fetchFeed = async () => {
    try {
      const res = await fetch("/api/v1/agent-feed");
      if (!res.ok) {
        throw new Error("Failed to fetch feed");
      }
      const data = await res.json();
      // Only show pending items on this feed view
      setItems(
        (data.items || []).filter(
          (i: any) =>
            i.lifecycle_state !== "APPROVED" &&
            i.lifecycle_state !== "DISMISSED",
        ),
      );
    } catch (err: any) {
      setError(err.message);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchFeed();
  }, []);

  const feedWsUrl = (() => {
    if (typeof window === "undefined") return "";
    const protocol = window.location.protocol === "https:" ? "wss:" : "ws:";
    const isLocalhost =
      window.location.hostname === "localhost" ||
      window.location.hostname === "127.0.0.1";
    return isLocalhost
      ? `ws://127.0.0.1:18789/api/v1/feed/ws`
      : `${protocol}//${window.location.host}/api/v1/feed/ws`;
  })();

  useAgentWebSocket({
    url: feedWsUrl,
    onMessage: (item: any) => {
      if (item.error) {
        console.error("Agent feed WS error:", item.error);
        return;
      }

      if (!item?.id) return;

      if (
        String(item.lifecycle_state || "").toUpperCase() === "PENDING_APPROVAL"
      ) {
        setItems((current) => [
          item,
          ...current.filter((existing) => existing.id !== item.id),
        ]);
      } else if (
        String(item.lifecycle_state || "").toUpperCase() === "APPROVED" ||
        String(item.lifecycle_state || "").toUpperCase() === "DISMISSED"
      ) {
        setItems((current) =>
          current.filter((existing) => existing.id !== item.id),
        );
      } else if (
        String(item.status || "").toUpperCase() === "DRAFT" ||
        String(item.status || "").toUpperCase() === "PENDING"
      ) {
        setItems((current) => [
          item,
          ...current.filter((existing) => existing.id !== item.id),
        ]);
      } else if (item.status) {
        setItems((current) =>
          current.filter((existing) => existing.id !== item.id),
        );
      }
    },
  });

  const startEditing = (item: FeedItem) => {
    setEditingId(item.id);
    const isAmbassador =
      item.proposed_action?.feature_type === "ambassador_reply" ||
      item.context_payload?.feature_type === "ambassador_reply";
    const isPromoter =
      item.proposed_action?.feature_type === "social_post_draft" ||
      item.context_payload?.feature_type === "social_post_draft";
    const promoterPayload = isPromoter
      ? item.proposed_action || item.context_payload
      : null;
    const textToEdit = isAmbassador
      ? (item.proposed_action || item.context_payload)?.generated_response ||
        (item.proposed_action || item.context_payload)?.draft_reply
      : item.context_payload?.summary ||
        item.proposed_action?.description ||
        "A new update requires your attention.";
    setEditValue(textToEdit || "");
  };

  const saveEdit = async (id: string) => {
    const item = items.find((i) => i.id === id);
    if (!item) return;

    const isAmbassador =
      item.proposed_action?.feature_type === "ambassador_reply" ||
      item.context_payload?.feature_type === "ambassador_reply";
    const isPromoter =
      item.proposed_action?.feature_type === "social_post_draft" ||
      item.context_payload?.feature_type === "social_post_draft";
    const promoterPayload = isPromoter
      ? item.proposed_action || item.context_payload
      : null;

    const updatedProposed = {
      ...item.proposed_action,
      description: isAmbassador ? item.proposed_action?.description : editValue,
      generated_response: isAmbassador
        ? editValue
        : item.proposed_action?.generated_response,
    };

    const updatedContext = {
      ...item.context_payload,
      summary: isAmbassador ? item.context_payload?.summary : editValue,
      generated_response: isAmbassador
        ? editValue
        : item.context_payload?.generated_response,
    };

    setItems((prev) =>
      prev.map((i) => {
        if (i.id === id) {
          return {
            ...i,
            proposed_action: updatedProposed,
            context_payload: updatedContext,
          };
        }
        return i;
      }),
    );

    await handleAction(id, "PENDING_APPROVAL", updatedProposed, updatedContext);
    setEditingId(null);
  };

  const cancelEdit = () => {
    setEditingId(null);
  };

  const handleAction = async (
    id: string,
    state: string,
    updatedProposed?: any,
    updatedContext?: any,
  ) => {
    const item = items.find((i) => i.id === id);
    if (state === "APPROVED") {
      if (item?.proposed_action?.action_type === "Draft Quote") {
        router.push(`/quotes/${item.proposed_action.quote_id}`);
        return;
      }
      if (item?.proposed_action?.action_type === "Draft Booking") {
        // Optimistic UI or fetch the status change
        // For Draft Booking, it confirms it in the backend and maybe we navigate to booking detail or just resolve here.
        // We'll proceed with normal backend request to approve it so `action_router` handles it.
      }
    }

    try {
      setProcessingId(id);

      const bodyPayload: any = { state };
      const proposed = updatedProposed || item?.proposed_action;
      const context = updatedContext || item?.context_payload;

      if (proposed) bodyPayload.proposed_action = proposed;
      if (context) bodyPayload.context_payload = context;

      const res = await fetch(`/api/v1/agent-feed/${id}/state`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(bodyPayload),
      });
      if (!res.ok) throw new Error("Action failed");

      // Update UI optimistically or refetch
      if (state === "APPROVED" || state === "DISMISSED") {
        setItems((prev) => prev.filter((item) => item.id !== id));
      }
    } catch (err: any) {
      alert(err.message);
    } finally {
      setProcessingId(null);
    }
  };

  return (
    <AppShell
      title="Daily Work"
      subtitle="Your daily priorities, coordinated by your team."
      actions={[{ label: "New Sale", href: "/pos/terminal", primary: true }]}
    >
      <div
        className="w-full max-w-full overflow-hidden px-4 mx-auto space-y-4"
        data-testid="agent-feed"
      >
        {loading && (
          <div className="flex justify-center items-center py-12">
            <p className="text-gray-500 font-medium">Checking your feed...</p>
          </div>
        )}

        {error && (
          <div className="glassmorphism bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 p-4 text-center shadow-sm rounded-[16px]">
            <p className="text-[#FF3B30] dark:text-[#DE1B1B] font-medium mb-2">
              We couldn't load your feed.
            </p>
            <p className="text-sm text-gray-500">{error}</p>
          </div>
        )}

        {!loading && !error && items.length === 0 && (
          <div
            className="glassmorphism bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm flex flex-col items-center justify-center p-12 text-center rounded-[16px]"
            data-testid="agent-feed-empty"
          >
            <div className="w-16 h-16 bg-[#e8f7ef] dark:bg-[rgba(23,166,106,0.2)] rounded-full flex items-center justify-center mb-4">
              <svg
                className="w-8 h-8 text-[#17a66a]"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M5 13l4 4L19 7"
                ></path>
              </svg>
            </div>
            <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-2">
              You're all caught up!
            </h3>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              There are no pending actions for you right now. Your team is
              handling things.
            </p>
          </div>
        )}

        <div className="flex flex-col gap-4">
          {items.map((item) => {
            const isProcessing = processingId === item.id;
            const isAmbassador =
              item.proposed_action?.feature_type === "ambassador_reply" ||
              item.context_payload?.feature_type === "ambassador_reply";
            const isPromoter =
              item.proposed_action?.feature_type === "social_post_draft" ||
              item.context_payload?.feature_type === "social_post_draft";
            const promoterPayload = isPromoter
              ? item.proposed_action || item.context_payload
              : null;
            const ambassadorPayload = isAmbassador
              ? item.proposed_action || item.context_payload
              : null;
            const isDisputeResolution =
              item.proposed_action?.feature_type === "dispute_resolution" ||
              item.context_payload?.feature_type === "dispute_resolution";
            const disputePayload = isDisputeResolution
              ? item.proposed_action || item.context_payload
              : null;
            const isInvoiceDraft =
              item.proposed_action?.feature_type === "invoice_draft" ||
              item.context_payload?.feature_type === "invoice_draft";
            const invoicePayload = isInvoiceDraft
              ? item.proposed_action || item.context_payload
              : null;
            const isInvoiceFollowup =
              item.proposed_action?.feature_type === "invoice_followup" ||
              item.context_payload?.feature_type === "invoice_followup";
            const invoiceFollowupPayload = isInvoiceFollowup
              ? item.proposed_action || item.context_payload
              : null;

            return (
              <div
                key={item.id}
                className={`glassmorphism bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm rounded-[16px] p-5 relative overflow-hidden break-words whitespace-normal transition-all duration-300 ${isProcessing ? "opacity-50 scale-[0.98]" : "animate-fade-in"}`}
                data-testid="agent-feed-card"
              >
                <div className="flex justify-between items-start mb-3">
                  <span
                    className={`text-[11px] font-bold uppercase tracking-wider ${isDisputeResolution || isInvoiceFollowup ? "text-[#FF9500] dark:text-[#FF9F0A]" : "text-[#0066FF] dark:text-[#0071E3]"} flex items-center gap-1.5`}
                  >
                    <span
                      className={`w-2 h-2 rounded-full ${isDisputeResolution || isInvoiceFollowup ? "bg-[#FF9500] dark:bg-[#FF9F0A]" : "bg-[#0066FF] dark:bg-[#0071E3]"} opacity-80`}
                    ></span>
                    {isDisputeResolution
                      ? "DISPUTE RESOLUTION"
                      : isInvoiceFollowup
                        ? "ACTION REQUIRED"
                        : isInvoiceDraft
                          ? "INVOICE DRAFT"
                          : isAmbassador
                            ? "CUSTOMER MESSAGE"
                            : item.proposed_action?.action_type ===
                                "Draft Quote"
                              ? "SMART ESTIMATE"
                              : item.proposed_action?.action_type ===
                                  "Draft Follow-up"
                                ? "DEPOSIT FOLLOW-UP"
                                : item.proposed_action?.action_type ===
                                    "Draft Booking"
                                  ? "NEW BOOKING REQUEST"
                                  : item.event_source.replace(/_/g, " ")}
                  </span>
                  <span className="text-[11px] text-gray-400 font-medium">
                    {new Date(item.created_at).toLocaleTimeString([], {
                      hour: "2-digit",
                      minute: "2-digit",
                    })}
                  </span>
                </div>

                <h3 className="font-bold text-gray-900 dark:text-white text-[15px] mb-2 leading-snug">
                  {isDisputeResolution
                    ? `Dispute from ${disputePayload?.sender_id || "Customer"}`
                    : isInvoiceFollowup
                      ? `Action Required: Overdue Invoice`
                      : isInvoiceDraft
                        ? `Draft Invoice ready for ${invoicePayload?.milestone_name || "Phase 1"}`
                        : isAmbassador
                          ? `New Message from ${ambassadorPayload.sender_id || "Customer"}`
                          : isPromoter
                            ? `New Product: ${promoterPayload?.product_name || "Marketing Draft"}`
                            : item.proposed_action?.action_type ===
                                "Draft Quote"
                              ? `Drafted Estimate for ${item.context_payload?.customer_name || "Customer"}`
                              : item.proposed_action?.action_type ===
                                  "Draft Follow-up"
                                ? `Unpaid Deposit: ${item.context_payload?.customer_name || "Customer"}`
                                : item.proposed_action?.action_type ===
                                    "Draft Booking"
                                  ? `Drafted Booking for ${item.context_payload?.customer_name || "Customer"}`
                                  : item.proposed_action?.title ||
                                    "Review Required"}
                </h3>

                {isAmbassador ? (
                  <AmbassadorReplyCard
                    approval={item}
                    isEditing={editingId === item.id}
                    editContent={editValue}
                    setEditContent={setEditValue}
                    onEdit={() => startEditing(item)}
                    onCancelEdit={() => cancelEdit()}
                    onSaveEdit={() => saveEdit(item.id)}
                    onApprove={() => handleAction(item.id, "APPROVED")}
                    onDismiss={() => handleAction(item.id, "DISMISSED")}
                  />
                ) : editingId === item.id ? (
                  <>
                    <div className="mb-5">
                      {isDisputeResolution ? (
                        <div className="flex flex-col gap-3">
                          <div className="bg-[#FFF5E5] dark:bg-[rgba(255,149,0,0.1)] p-3 rounded-lg border border-[#FFD699] dark:border-[rgba(255,149,0,0.3)]">
                            <p className="text-[13px] text-[#8C5300] dark:text-[#FF9F0A] italic mb-1">
                              "{disputePayload?.original_message}"
                            </p>
                            {disputePayload?.past_orders && (
                              <span className="inline-block text-[10px] font-semibold text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/30 px-2 py-0.5 rounded-full mt-1">
                                {disputePayload?.past_orders}
                              </span>
                            )}
                          </div>
                          <div>
                            <p className="text-[11px] font-bold text-gray-500 uppercase mb-1">
                              Proposed Resolution
                            </p>
                            <p className="text-[13px] text-gray-900 dark:text-white leading-relaxed mb-3">
                              {disputePayload?.generated_response}
                            </p>
                            <div className="bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 shadow-sm rounded-[8px] overflow-hidden">
                              {disputePayload?.refund_amount && (
                                <div className="flex items-center gap-3 p-3 border-b border-gray-100 dark:border-gray-700">
                                  <input
                                    type="checkbox"
                                    defaultChecked
                                    className="w-4 h-4 text-[#FF9500] rounded border-gray-300 focus:ring-[#FF9500]"
                                  />
                                  <span className="text-[13px] text-gray-800 dark:text-gray-200 font-medium">
                                    Issue ${disputePayload?.refund_amount}{" "}
                                    Refund
                                  </span>
                                </div>
                              )}
                              {disputePayload?.operational_action && (
                                <div className="flex items-center gap-3 p-3">
                                  <input
                                    type="checkbox"
                                    defaultChecked
                                    className="w-4 h-4 text-[#FF9500] rounded border-gray-300 focus:ring-[#FF9500]"
                                  />
                                  <span className="text-[13px] text-gray-800 dark:text-gray-200 font-medium">
                                    {disputePayload?.operational_action}
                                  </span>
                                </div>
                              )}
                            </div>
                          </div>
                        </div>
                      ) : isInvoiceDraft ? (
                        <div className="flex flex-col gap-3">
                          <div className="bg-green-50 dark:bg-green-900/20 p-3 rounded-lg border border-green-100 dark:border-green-800/50">
                            <p className="text-[13px] text-green-700 dark:text-green-300 font-medium mb-1">
                              Generated Invoice
                            </p>
                            <p className="text-[11px] text-green-600/70 dark:text-green-400/70">
                              Review the drafted invoice. Select "Approve &
                              Send" to email the client and generate a secure
                              payment link.
                            </p>
                          </div>
                          <div className="space-y-3 mt-2">
                            <div className="p-3 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 rounded-[8px] shadow-sm">
                              <p className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-1 flex items-center gap-1">
                                Project
                              </p>
                              <p className="text-[13px] font-medium text-gray-800 dark:text-gray-200">
                                {invoicePayload?.project_name}
                              </p>
                              <p className="text-[11px] text-gray-500 mt-0.5">
                                {invoicePayload?.milestone_name}
                              </p>
                            </div>
                            <div className="flex justify-between items-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg border border-gray-100 dark:border-gray-700">
                              <span className="text-[12px] font-medium text-gray-600 dark:text-gray-400">
                                Total Amount Due
                              </span>
                              <span className="text-[16px] font-bold text-gray-900 dark:text-white">
                                $
                                {(
                                  (invoicePayload?.amount_cents || 0) / 100
                                ).toFixed(2)}
                              </span>
                            </div>
                          </div>
                        </div>
                      ) : isInvoiceFollowup ? (
                        <div className="flex flex-col gap-3">
                          <div className="bg-amber-50 dark:bg-amber-900/20 p-3 rounded-lg border border-amber-100 dark:border-amber-800/50">
                            <p className="text-[13px] text-amber-700 dark:text-amber-300 font-medium mb-1">
                              Overdue Invoice Detected
                            </p>
                            <p className="text-[11px] text-amber-600/70 dark:text-amber-400/70">
                              The Finance Agent noticed an overdue invoice and
                              has drafted a reminder via{" "}
                              {invoiceFollowupPayload?.suggested_channel ||
                                "email"}
                              .
                            </p>
                          </div>
                          <div className="space-y-3 mt-2">
                            <div className="p-3 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 rounded-[8px] shadow-sm">
                              <p className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-1 flex items-center gap-1">
                                Status
                              </p>
                              <p className="text-[13px] font-medium text-gray-800 dark:text-gray-200 whitespace-pre-wrap">
                                {invoiceFollowupPayload?.original_message}
                              </p>
                            </div>
                            <div className="p-3 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 rounded-[8px] shadow-sm">
                              <p className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-1 flex items-center gap-1">
                                Drafted Reminder
                              </p>
                              <p className="text-[13px] font-medium text-gray-800 dark:text-gray-200 whitespace-pre-wrap">
                                {invoiceFollowupPayload?.generated_response}
                              </p>
                            </div>
                          </div>
                        </div>
                      ) : isPromoter ? (
                        <div className="flex flex-col gap-3">
                          <div className="bg-indigo-50 dark:bg-indigo-900/20 p-3 rounded-lg border border-indigo-100 dark:border-indigo-800/50">
                            <p className="text-[13px] text-indigo-700 dark:text-indigo-300 font-medium mb-1">
                              Generated Marketing Posts
                            </p>
                            <p className="text-[11px] text-indigo-600/70 dark:text-indigo-400/70">
                              Review the captions drafted for your new product.
                              Select "Approve & Schedule" to push these to your
                              linked channels.
                            </p>
                          </div>
                          <div className="space-y-3 mt-2">
                            <div className="p-3 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 rounded-[8px] shadow-sm">
                              <p className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-1 flex items-center gap-1">
                                📱 Instagram
                              </p>
                              <p className="text-[12px] text-gray-800 dark:text-gray-200 whitespace-pre-wrap">
                                {promoterPayload?.instagram}
                              </p>
                            </div>
                            <div className="p-3 bg-white/65 backdrop-blur-[30px] backdrop-saturate-[2.1] border border-white/40 dark:bg-[#16161a]/70 dark:backdrop-blur-[30px] dark:backdrop-saturate-[2.1] dark:border-white/10 rounded-[8px] shadow-sm">
                              <p className="text-[10px] font-bold text-gray-500 uppercase tracking-wider mb-1 flex items-center gap-1">
                                🎵 TikTok
                              </p>
                              <p className="text-[12px] text-gray-800 dark:text-gray-200 whitespace-pre-wrap">
                                {promoterPayload?.tiktok}
                              </p>
                            </div>
                          </div>
                        </div>
                      ) : null}
                    </div>

                    {isPromoter ? (
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <button
                          onClick={() => handleAction(item.id, "APPROVED")}
                          disabled={isProcessing}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                          aria-label="Approve & Schedule"
                          data-testid="feed-approve-btn"
                        >
                          {isProcessing
                            ? "Processing..."
                            : "Approve & Schedule"}
                        </button>
                        <button
                          onClick={() => startEditing(item)}
                          disabled={isProcessing}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Edit Draft"
                          data-testid="feed-edit-btn"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleAction(item.id, "DISMISSED")}
                          disabled={isProcessing}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          aria-label="Dismiss Draft"
                          data-testid="feed-dismiss-btn"
                        >
                          Dismiss
                        </button>
                      </div>
                    ) : (
                      <div className="flex flex-col sm:flex-row gap-3 w-full">
                        <button
                          onClick={() => handleAction(item.id, "APPROVED")}
                          disabled={isProcessing}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 bg-[#0066FF] text-white font-medium hover:bg-[#0052CC] transition-all duration-200 shadow-md flex items-center justify-center"
                          data-testid="feed-approve-btn"
                        >
                          {isProcessing
                            ? "Processing..."
                            : item.proposed_action?.feature_type ===
                                  "invoice_draft" ||
                                item.context_payload?.feature_type ===
                                  "invoice_draft"
                              ? "Approve & Send"
                              : item.proposed_action?.action_type ===
                                  "Draft Quote"
                                ? "Review Estimate"
                                : item.proposed_action?.action_type ===
                                    "Draft Follow-up"
                                  ? "Send Follow-up"
                                  : item.proposed_action?.action_type ===
                                      "Draft Booking"
                                    ? "Approve & Confirm"
                                    : item.proposed_action?.action_type ===
                                        "Reassign Shift"
                                      ? "Approve & Notify"
                                      : "Approve"}
                        </button>
                        <button
                          onClick={() => startEditing(item)}
                          disabled={isProcessing}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          data-testid="feed-edit-btn"
                        >
                          Edit
                        </button>
                        <button
                          onClick={() => handleAction(item.id, "DISMISSED")}
                          disabled={isProcessing}
                          className="flex-1 min-h-[44px] min-w-[44px] px-4 border border-gray-300 dark:border-gray-600 text-[#1D1D1F] dark:text-[#F5F5F7] font-medium hover:bg-gray-100 dark:hover:bg-gray-800 transition-all duration-200 flex items-center justify-center"
                          data-testid="feed-dismiss-btn"
                        >
                          Dismiss
                        </button>
                      </div>
                    )}
                  </>
                ) : null}
              </div>
            );
          })}
        </div>
      </div>
    </AppShell>
  );
}
