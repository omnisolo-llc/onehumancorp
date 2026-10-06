"use client";
import { parseManualInboxReceipt, manualInboxReceiptStatus, type ManualInboxReceipt } from "@/lib/inboxManualReceipt";
import { messageDeliveryStatus } from "@/lib/messageDeliveryStatus";


import { errorMessage } from '@/lib/errors';
import { Fragment, Suspense, useEffect, useMemo, useState, useRef, type ReactNode } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import { AppShell } from "../components/AppShell";
import { useQuery } from "@powersync/react";
import { PowerSyncProvider } from "../../lib/powersync/PowerSyncProvider";
import { QUEUE_IDENTITY_EPOCH_KEY } from '@/lib/sync/queueIdentity';

type Message = {
  id: string;
  source?: string;
  content?: string;
  original_content?: string;
  translated_from_language?: string;
  draft_reply?: string;
  status?: string;
  sender_id?: string;
  customer_id?: string;
  checkout_link?: string | null;
  proposed_product_id?: string | null;
  created_at?: string;
};

function badgeTone(status?: string) {
  const delivery = messageDeliveryStatus(status);
  if (delivery) return delivery.tone;
  const normalized = (status || "").toLowerCase();
  if (["closed", "sent", "resolved", "auto_replied"].includes(normalized)) return "good";
  if (["open", "pending", "pending_approval", ""].includes(normalized)) return "warn";
  if (["failed", "blocked"].includes(normalized)) return "bad";
  return "";
}


function normalizeExternalHttpUrl(value: string) {
  try {
    const url = new URL(value);
    return url.protocol === "http:" || url.protocol === "https:" ? url.toString() : null;
  } catch {
    return null;
  }
}

function textWithLineBreaks(value: string, keyPrefix: string): ReactNode[] {
  return value.split("\n").flatMap((line, index) => [
    index > 0 ? <br key={`${keyPrefix}-break-${index}`} /> : null,
    <Fragment key={`${keyPrefix}-line-${index}`}>{line}</Fragment>,
  ]);
}

function renderMessageContent(content: string): ReactNode {
  if (!content) return "Empty message";

  const tokenPattern = /\[Media:\s*(.+?)\s+-\s+(https?:\/\/[^\]\s]+)\]|!\[([^\]]*)\]\((https?:\/\/[^)\s]+)\)/g;
  const nodes: ReactNode[] = [];
  let cursor = 0;
  let tokenIndex = 0;

  for (const match of content.matchAll(tokenPattern)) {
    const offset = match.index ?? cursor;
    nodes.push(...textWithLineBreaks(content.slice(cursor, offset), `text-${tokenIndex}`));

    const mediaType = match[1]?.trim();
    const rawUrl = match[2] ?? match[4];
    const url = rawUrl ? normalizeExternalHttpUrl(rawUrl) : null;
    const alt = mediaType ?? match[3] ?? "Attached image";

    if (!url) {
      nodes.push(...textWithLineBreaks(match[0], `invalid-${tokenIndex}`));
    } else if (!mediaType || mediaType.startsWith("image/")) {
      nodes.push(
        <span className="my-2 block" key={`image-${tokenIndex}`}>
          {/* Customer media uses an external runtime URL rather than a build-time image asset. */}
          <img
            src={url}
            alt={alt}
            className="h-auto max-h-[300px] max-w-full rounded-md shadow-sm"
          />
        </span>,
      );
    } else {
      nodes.push(
        <span className="my-2 block" key={`attachment-${tokenIndex}`}>
          <a
            href={url}
            target="_blank"
            rel="noopener noreferrer"
            className="text-blue-600 underline"
          >
            Attached Media ({mediaType})
          </a>
        </span>,
      );
    }

    cursor = offset + match[0].length;
    tokenIndex += 1;
  }

  nodes.push(...textWithLineBreaks(content.slice(cursor), `text-${tokenIndex}`));
  return nodes;
}

function formatStatus(status?: string) {
  const normalized = (status || "").toLowerCase();
  const delivery = messageDeliveryStatus(normalized);
  if (delivery) return delivery.label;
  return status || "Open";
}

function CustomerContextCard({ customerId }: { customerId: string }) {
  const [summary, setSummary] = useState<{ total_interactions: number; segments: string[]; preferences: string[]; summary: string } | null>(null);

  useEffect(() => {
    async function fetchSummary() {
      try {
        const res = await fetch(`/api/v1/memory/summary/${customerId}`);
        if (res.ok) {
          const data = await res.json();
          setSummary(data);
        }
      } catch {
        // Silently handled on load failure during rapid navigation
      }
    }
    fetchSummary();
  }, [customerId]);

  if (!summary) return null;
  if (summary.total_interactions === 0 && summary.segments.length === 0) return null;

  return (
    <div className="mt-4 rounded-lg border border-gray-100 bg-blue-50/50 p-4 dark:border-white/10 dark:bg-blue-900/10">
      <div className="mb-2 flex items-center justify-between">
        <h3 className="text-sm font-semibold text-blue-900 dark:text-blue-100">Unified Customer Memory</h3>
        <span className="app-badge good">{summary.total_interactions} interactions</span>
      </div>
      {summary.segments.length > 0 && (
        <div className="mb-2 text-xs text-gray-700 dark:text-gray-300">
          <span className="font-semibold text-gray-900 dark:text-white">Segments: </span>
          {summary.segments.join(", ")}
        </div>
      )}
      {summary.preferences.length > 0 && (
        <div className="mb-2 text-xs text-gray-700 dark:text-gray-300">
          <span className="font-semibold text-gray-900 dark:text-white">Preferences: </span>
          {summary.preferences.join(", ")}
        </div>
      )}
      <div className="text-xs text-gray-600 dark:text-gray-400">
        {summary.summary}
      </div>
    </div>
  );
}

type ManualReplyBody = { message_id: string; approved: boolean; edited_reply: string; request_id: string };
type ManualReplyRequest = { body: ManualReplyBody; receipt?: ManualInboxReceipt; needsReadback: boolean; mayClearDraft: boolean };
type ManualOperation = { messageId: string; kind: 'prepare' | 'send' | 'read' | 'dismiss' };

function InboxWorkspace({
  messages,
  sourceLabel,
}: {
  messages: Message[];
  sourceLabel: string;
}) {
  const router = useRouter();
  const searchParams = useSearchParams();
  const requestedId = searchParams.get('messageId') || null;
  const [selection, setSelection] = useState<{ request: string | null; id: string | null }>({ request: requestedId, id: requestedId });
  const selectedId = selection.request === requestedId ? selection.id : requestedId;
  const [showOriginal, setShowOriginal] = useState(false);
  const [actionStatus, setViewActionStatus] = useState("");
  const [replyDrafts, setReplyDrafts] = useState(() => new Map<string, string>());
  const activeMessage = useRef<string | null>(null);
  const sessionEpoch = useRef(0);
  const manualRequests = useRef(new Map<string, ManualReplyRequest>());
  const unreadReceipts = useRef(new Set<string>());
  const manualOperation = useRef<ManualOperation | null>(null);
  const draftVersions = useRef(new Map<string, number>());
  const [busyOperation, setBusyOperation] = useState<ManualOperation | null>(null);
  const selectionOwner = useRef({ messageId: null as string | null, requestedId });

  useEffect(() => {
    setSelection({ request: requestedId, id: requestedId });
  }, [requestedId]);

  const selected = useMemo(() => {
    if (messages.length === 0) return null;
    return selectedId === null ? messages[0] : messages.find((m) => m.id === selectedId) || null;
  }, [messages, selectedId]);
  useEffect(() => {
    setShowOriginal(false); setViewActionStatus('');
  }, [selected?.id, requestedId]);
  // Retire operation ownership during render, even for a fast away/back navigation.
  // An old preparation may finish, but cannot continue into a provider send.
  if (selectionOwner.current.messageId !== (selected?.id ?? null) || selectionOwner.current.requestedId !== requestedId
    || (['resolved', 'dismissed'].includes(selected?.status ?? '') && ['prepare', 'send'].includes(manualOperation.current?.kind ?? ''))) {
    manualOperation.current = null;
    selectionOwner.current = { messageId: selected?.id ?? null, requestedId };
  }
  activeMessage.current = selected?.id ?? null;
  const renderedEpoch = sessionEpoch.current;
  const draftId = selected?.id ?? null;
  const manualReply = draftId ? replyDrafts.get(draftId) ?? '' : '';
  const setManualReply = (value: string | ((previous: string) => string)) => {
    if (!draftId || renderedEpoch !== sessionEpoch.current) return;
    draftVersions.current.set(draftId, (draftVersions.current.get(draftId) ?? 0) + 1);
    if (manualOperation.current?.kind === 'prepare' && manualOperation.current.messageId === draftId) {
      manualOperation.current = null;
      setBusyOperation(null);
    }
    setReplyDrafts(previous => new Map(previous).set(draftId, typeof value === 'function' ? value(previous.get(draftId) ?? '') : value));
  };
  const setActionStatus = (value: string) => {
    if (renderedEpoch === sessionEpoch.current && activeMessage.current === draftId) setViewActionStatus(value);
  };
  useEffect(() => {
    const clear = () => {
      sessionEpoch.current += 1; manualOperation.current = null; manualRequests.current.clear(); unreadReceipts.current.clear(); draftVersions.current.clear();
      setReplyDrafts(new Map()); setViewActionStatus(''); setBusyOperation(null);
    };
    const retire = () => { manualOperation.current = null; setBusyOperation(null); };
    const storage = (event: StorageEvent) => { if (event.key === null || event.key === QUEUE_IDENTITY_EPOCH_KEY) clear(); };
    window.addEventListener('omnisolo_auth_changed', clear); window.addEventListener('storage', storage); window.addEventListener('pagehide', retire);
    return () => { sessionEpoch.current += 1; activeMessage.current = null; manualOperation.current = null; window.removeEventListener('omnisolo_auth_changed', clear); window.removeEventListener('storage', storage); window.removeEventListener('pagehide', retire); };
  }, []);

  const [pendingApprovals, setPendingApprovals] = useState<{ id: string; payload?: { inbox_message_id?: string; drafted_response?: string; draft_reply?: string } | string }[]>([]);

  useEffect(() => {
    async function fetchApprovals() {
      try {
        const res = await fetch(`/api/v1/agents/approvals?limit=50`);
        if (res.ok) {
          const data = await res.json();
          setPendingApprovals(data.pending_approvals || []);
        }
      } catch {
        // Silently handled on load failure during rapid navigation
      }
    }
    fetchApprovals();
  }, []);

  const activeApproval = useMemo(() => {
    if (!selected) return null;
    return pendingApprovals.find((a) => {
      try {
        const payload = typeof a.payload === 'string' ? JSON.parse(a.payload) : a.payload;
        return payload && payload.inbox_message_id === selected.id;
      } catch  {
        return false;
      }
    });
  }, [selected, pendingApprovals]);


  const openCount = messages.filter((message) => !["closed", "resolved"].includes((message.status || "").toLowerCase())).length;
  const unreadLeadsCount = messages.filter((message) => (message.status || "").toLowerCase() === "unread").length;

  async function handleDraftQuoteWithAI(message: Message) {
    try {
      setActionStatus("Drafting quote with AI...");
      const res = await fetch("/api/v1/quotes/draft_agent", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          inquiry: message.content || "",
          customer_id: message.customer_id || message.sender_id || "unknown",
        }),
      });
      if (!res.ok) throw new Error("Failed to draft quote");
      const data = await res.json();
      if (data.id) {
        setActionStatus("Quote drafted successfully!");
        router.push(`/quotes/${data.id}`);
      }
    } catch (err) {
      setActionStatus(`Error drafting quote: ${errorMessage(err, '')}`);
    } finally {
      setTimeout(() => setActionStatus(""), 3000);
    }
  }


  const receiptBlocked = (request?: ManualReplyRequest) => !!request && (request.needsReadback
    || ['unknown', 'accepted', 'dismissed', 'resolved'].includes(request.receipt?.state ?? ''));
  const isManualBusy = !!busyOperation && busyOperation === manualOperation.current;
  const currentManualRequest = draftId ? manualRequests.current.get(draftId) : undefined;
  const previousSend = currentManualRequest?.receipt?.prior_send;

  function ownsOperation(operation: ManualOperation) {
    return manualOperation.current === operation && renderedEpoch === sessionEpoch.current
      && activeMessage.current === operation.messageId;
  }

  function applyManualReceipt(receipt: ManualInboxReceipt, request: ManualReplyRequest, restoreDraft = false, version?: number) {
    request.receipt = receipt; request.needsReadback = false;
    manualRequests.current.set(receipt.message_id, request);
    setActionStatus(manualInboxReceiptStatus(receipt));
    setReplyDrafts(previous => {
      const draft = previous.get(receipt.message_id);
      if (request.mayClearDraft && receipt.state === 'accepted' && draft === request.body.edited_reply) {
        return new Map(previous).set(receipt.message_id, '');
      }
      if (restoreDraft && !['accepted', 'dismissed', 'resolved'].includes(receipt.state)
        && !draft && version === (draftVersions.current.get(receipt.message_id) ?? 0)) {
        return new Map(previous).set(receipt.message_id, receipt.draft_reply);
      }
      return previous;
    });
  }

  async function handleSendManualReply(inboxMessageId: string) {
    if (!manualReply.trim() || manualReply.length > 16_000 || manualOperation.current || renderedEpoch !== sessionEpoch.current) return;
    const existing = manualRequests.current.get(inboxMessageId);
    if (receiptBlocked(existing) || unreadReceipts.current.has(inboxMessageId)) {
      setActionStatus('Check saved reply status before attempting another send. Your draft is preserved.');
      return;
    }
    const operation: ManualOperation = { messageId: inboxMessageId, kind: 'prepare' };
    manualOperation.current = operation; setBusyOperation(operation);
    try {
      const request: ManualReplyRequest = existing?.receipt?.state === 'pending' && existing.body.edited_reply === manualReply
        ? existing : { body: { message_id: inboxMessageId, approved: true, edited_reply: manualReply, request_id: crypto.randomUUID() }, needsReadback: true, mayClearDraft: true };
      // Keep request identity and draft in memory. Reload recovery reads actor-scoped server state.
      manualRequests.current.set(inboxMessageId, request);
      request.needsReadback = true; request.mayClearDraft = true;
      setActionStatus('Preparing reply...');
      const preparedResponse = await fetch('/api/v1/ui/omni_inbox/action', {
        method: 'POST', headers: { 'Content-Type': 'application/json' }, cache: 'no-store',
        body: JSON.stringify({ ...request.body, prepare_only: true }),
      });
      if (!ownsOperation(operation)) return;
      if (!preparedResponse.ok) throw new Error('Preparation unconfirmed');
      const prepared = parseManualInboxReceipt(await preparedResponse.json(), inboxMessageId, request.body);
      if (!ownsOperation(operation)) return;
      if (prepared.state !== 'pending') { applyManualReceipt(prepared, request); return; }
      operation.kind = 'send';
      setActionStatus('Requesting provider acceptance...');
      const response = await fetch('/api/v1/ui/omni_inbox/action', {
        method: 'POST', headers: { 'Content-Type': 'application/json' }, cache: 'no-store', body: JSON.stringify(request.body),
      });
      if (!ownsOperation(operation)) return;
      if (!response.ok) throw new Error('Provider acknowledgement unconfirmed');
      const receipt = parseManualInboxReceipt(await response.json(), inboxMessageId, request.body);
      if (ownsOperation(operation)) applyManualReceipt(receipt, request);
    } catch {
      if (ownsOperation(operation)) setActionStatus('Check saved reply status before attempting another send. Your draft is preserved.');
    } finally {
      if (ownsOperation(operation)) { manualOperation.current = null; setBusyOperation(null); }
    }
  }

  async function handleReadManualReceipt(inboxMessageId: string) {
    if (manualOperation.current || renderedEpoch !== sessionEpoch.current) return;
    const operation: ManualOperation = { messageId: inboxMessageId, kind: 'read' };
    unreadReceipts.current.add(inboxMessageId);
    manualOperation.current = operation; setBusyOperation(operation);
    let known = manualRequests.current.get(inboxMessageId);
    const version = draftVersions.current.get(inboxMessageId) ?? 0;
    try {
      setActionStatus('Checking saved reply status...');
      const query = new URLSearchParams({ message_id: inboxMessageId, ...(known ? { request_id: known.body.request_id } : {}) });
      let response = await fetch(`/api/v1/ui/omni_inbox/action?${query}`, { cache: 'no-store' });
      if (!ownsOperation(operation)) return;
      if (response.status === 404 && known) {
        // A conflicting preparation need not have been recorded. Recover the
        // actor's existing request instead of treating an unused UUID as no send.
        query.delete('request_id');
        known = undefined;
        response = await fetch(`/api/v1/ui/omni_inbox/action?${query}`, { cache: 'no-store' });
        if (!ownsOperation(operation)) return;
      }
      if (response.status === 404) {
        unreadReceipts.current.delete(inboxMessageId);
        manualRequests.current.delete(inboxMessageId);
        setActionStatus('No saved reply request was found for this message and account. Your draft is preserved.');
        return;
      }
      if (!response.ok) throw new Error('Receipt read unavailable');
      const receipt = parseManualInboxReceipt(await response.json(), inboxMessageId, known?.body);
      if (!ownsOperation(operation)) return;
      unreadReceipts.current.delete(inboxMessageId);
      const request = known ?? { body: { message_id: inboxMessageId, approved: true, edited_reply: receipt.draft_reply, request_id: receipt.request_id }, needsReadback: false, mayClearDraft: false };
      applyManualReceipt(receipt, request, true, version);
    } catch {
      if (ownsOperation(operation)) setActionStatus('Saved reply status is unavailable. Your draft is preserved; check again before sending.');
    } finally {
      if (ownsOperation(operation)) { manualOperation.current = null; setBusyOperation(null); }
    }
  }

  async function handleDismissMessage(inboxMessageId: string) {
    if (renderedEpoch !== sessionEpoch.current || manualOperation.current?.kind === 'send') return;
    // Dismissal supersedes preparation immediately, before its network response can arrive.
    const operation: ManualOperation = { messageId: inboxMessageId, kind: 'dismiss' };
    manualOperation.current = operation; setBusyOperation(operation);
    try {
      const request: ManualReplyRequest = { body: { message_id: inboxMessageId, approved: false, edited_reply: manualReply, request_id: crypto.randomUUID() }, needsReadback: true, mayClearDraft: false };
      manualRequests.current.set(inboxMessageId, request);
      setActionStatus('Dismissing message...');
      const response = await fetch('/api/v1/ui/omni_inbox/action', {
        method: 'POST', headers: { 'Content-Type': 'application/json' }, cache: 'no-store', body: JSON.stringify(request.body),
      });
      if (!ownsOperation(operation)) return;
      if (!response.ok) throw new Error('Dismissal unconfirmed');
      const receipt = parseManualInboxReceipt(await response.json(), inboxMessageId, request.body);
      if (receipt.state !== 'dismissed') throw new Error('Dismissal unconfirmed');
      if (ownsOperation(operation)) applyManualReceipt(receipt, request);
    } catch {
      if (ownsOperation(operation)) setActionStatus('Dismissal is unconfirmed. Check saved reply status before taking another action. Your draft is preserved.');
    } finally {
      if (ownsOperation(operation)) { manualOperation.current = null; setBusyOperation(null); }
    }
  }

  const fileInputRef = useRef<HTMLInputElement>(null);

  function handleAttachPhoto() {
    fileInputRef.current?.click();
  }

  async function handleFileSelected(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async (event) => {
      const base64String = event.target?.result as string;
      setManualReply(prev => prev + (prev.endsWith(" ") || prev === "" ? "" : " ") + `![Image](${base64String})`);
    };
    reader.readAsDataURL(file);

    // reset input
    if (fileInputRef.current) {
        fileInputRef.current.value = "";
    }
  }

  async function handleApproveAndSend(inboxMessageId: string) {
    try {
      const approval = pendingApprovals.find((a) => {
        try {
          const payload = typeof a.payload === 'string' ? JSON.parse(a.payload) : a.payload;
          return payload && payload.inbox_message_id === inboxMessageId;
        } catch  {
          return false;
        }
      });

      if (!approval) {
        setActionStatus("Could not find a pending approval for this message.");
        return;
      }

      const approveRes = await fetch(`/api/v1/agents/approvals/${approval.id}`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ approved: true })
      });

      if (approveRes.ok) {
        const receipt: unknown = await approveRes.json();
        if (!receipt || typeof receipt !== "object" || !("success" in receipt) || receipt.success !== true) {
          throw new Error("Approval acknowledgement is unconfirmed");
        }
        setActionStatus("Approval recorded. Check the message status for provider acceptance; delivery is unconfirmed.");
      } else {
        setActionStatus("Approval or provider acceptance is unconfirmed. Check the saved message status before retrying.");
      }
    } catch (e) {
      console.error(e);
      setActionStatus("Approval outcome is unknown. Check the saved status before retrying.");
    }
  }

  return (
    <AppShell
      title="Unified Inbox"
      subtitle="Local-first offline unified customer conversations and drafts."
      statusItems={[
        { label: "Messages", value: String(messages.length), tone: messages.length > 0 ? "good" : "neutral" },
        { label: "Open", value: String(openCount), tone: openCount > 0 ? "warn" : "good" },
      ]}
      actions={[{ label: "Audit", href: "/agent-audit-dashboard.html" }]}
    >
      <div className="mb-2 text-xs text-gray-500">
        Conversations for the current workspace.
      </div>
      {actionStatus && <div className="mb-4 app-badge" role="status">{actionStatus}</div>}
      <div className="w-full max-w-[375px] mx-auto md:max-w-none" data-testid="inbox-settled">
        <div className="app-grid two gap-4">
          <section className="app-panel glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] overflow-hidden">
            <div className="app-panel-header border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] p-4">
              <div>
                <div className="app-panel-title font-bold text-gray-900 dark:text-white">Message Queue</div>
                <div className="app-list-subtitle text-xs text-gray-500">{sourceLabel}</div>
              </div>
            </div>
            <div id="messages-list" className="app-list p-2">
              {unreadLeadsCount > 0 && (
                <div className="app-card daily-summary mb-4 bg-gradient-to-r from-blue-500/10 to-purple-500/10 border border-blue-500/20 p-4 rounded-xl">
                  <div className="text-sm font-semibold text-blue-900 dark:text-blue-100">
                    ✨ You have {unreadLeadsCount} unread {unreadLeadsCount === 1 ? 'lead' : 'leads'}.
                  </div>
                </div>
              )}
              {messages.length === 0 ? (
                <div className="app-empty">No inbox messages found for this tenant.</div>
              ) : messages.map((message) => (
                <button
                  key={message.id}
                  type="button"
                  onClick={() => {
                    setSelection({ request: requestedId, id: message.id });
                    setShowOriginal(false);
                  }}
                  className={`app-list-item min-h-[44px] min-w-[44px] w-full text-left p-3 mb-2 rounded-[8px] transition-all backdrop-filter ${selected?.id === message.id ? "bg-white/60 dark:bg-black/20 shadow-sm" : "hover:bg-black/5 dark:hover:bg-white/5 bg-white/10"}`}
                >
                  <div className="min-w-0">
                    <div className="app-list-title">{message.source || "Unknown source"}</div>
                    <div className="app-list-subtitle truncate">{message.content || message.original_content || "Empty message"}</div>
                  </div>
                  <span className={`app-badge ${badgeTone(message.status)}`}>{formatStatus(message.status)}</span>
                </button>
              ))}
            </div>

                {/* Manual Reply Box */}
                {selected && selected.status !== "resolved" && selected.status !== "dismissed" && (
                  <div className="mt-6 pt-4 border-t border-gray-200 dark:border-gray-700">
                    <div className="app-metric-label mb-2">Manual Reply</div>
                    <textarea
                      className="w-full min-h-[100px] p-3 rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-zinc-800 text-gray-900 dark:text-gray-100 mb-3"
                      placeholder="Type your reply here..."
                      value={manualReply}
                      onChange={(e) => setManualReply(e.target.value)}
                    />
                    <div className="flex gap-3 mt-4 flex-wrap">
                      <button
                        onClick={() => handleSendManualReply(selected.id)}
                        className="app-btn-primary min-h-[44px] min-w-[44px] rounded-[8px]"
                        disabled={!manualReply.trim() || manualReply.length > 16_000 || isManualBusy || receiptBlocked(currentManualRequest) || unreadReceipts.current.has(selected.id)}
                      >
                        Send Reply
                      </button>
                      <button
                        onClick={() => handleDismissMessage(selected.id)}
                        disabled={manualOperation.current?.kind === 'send' || manualOperation.current?.kind === 'dismiss' || ['dismissed', 'resolved'].includes(currentManualRequest?.receipt?.state ?? '')}
                        className="app-btn-secondary min-h-[44px] min-w-[44px] rounded-[8px]"
                      >Dismiss message</button>
                      <button
                        onClick={handleAttachPhoto}
                        className="app-btn-secondary flex items-center gap-2 min-h-[44px] min-w-[44px] rounded-[8px]"
                      >
                        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13" /></svg>
                        Attach Photo
                      </button>
                      <input type="file" accept="image/*" className="hidden" ref={fileInputRef} onChange={handleFileSelected} />
                    </div>
                  </div>
                )}
          </section>

          <section className="app-panel glassmorphism bg-[rgba(255,255,255,0.65)] dark:bg-[rgba(22,22,26,0.7)] backdrop-blur-[30px] saturate-[210%] border border-[rgba(255,255,255,0.4)] dark:border-[rgba(255,255,255,0.1)] rounded-[16px] overflow-hidden">
            <div className="app-panel-header border-b border-[rgba(255,255,255,0.2)] dark:border-[rgba(255,255,255,0.1)] bg-[rgba(255,255,255,0.4)] dark:bg-[rgba(22,22,26,0.5)] p-4">
              <div className="app-panel-title font-bold text-gray-900 dark:text-white">Conversation Detail</div>
            </div>
            {!selected ? (
              <div className="app-empty p-8 text-center text-gray-500">{selectedId ? 'The requested message is unavailable in this workspace.' : 'Select a database-backed message to inspect it.'}</div>
            ) : (
              <div className="app-panel-body p-5">
                <div className="mb-4 flex items-center justify-between">
                  <div>
                    <div className="app-metric-label">Source</div>
                    <div className="mt-1 text-sm font-semibold text-gray-900">{selected.source || "Unknown source"}</div>
                  </div>
                  {selected.sender_id && (
                    <div className="text-right">
                      <div className="app-metric-label">Sender</div>
                      <div className="mt-1 flex items-center gap-2">
                        <span className="text-sm font-semibold text-gray-900">{selected.sender_id}</span>
                        {selected.customer_id && (
                          <span className="app-badge good">Known Customer</span>
                        )}
                      </div>
                    </div>
                  )}
                </div>
                {selected.customer_id && (
                  <CustomerContextCard customerId={selected.customer_id} />
                )}
                <div className="mb-4">
                  <div className="flex items-center justify-between gap-3">
                    <div className="app-metric-label">Customer Message</div>
                    {selected.original_content && selected.original_content !== selected.content && (
                      <button
                        type="button"
                        className="app-badge"
                        onClick={() => setShowOriginal((value) => !value)}
                      >
                        {showOriginal ? "Translated" : `Original ${selected.translated_from_language || ""}`.trim()}
                      </button>
                    )}
                  </div>
                  <div className="mt-2 rounded-md border border-gray-200 bg-gray-50 p-3 text-sm leading-6 text-gray-800">
                    <div>{renderMessageContent((showOriginal ? selected.original_content : selected.content) || "Empty message")}</div>
                  </div>
                </div>

                <div className="mb-4">
                  <div className="app-metric-label">Draft Reply</div>
                  <div className="mt-2 rounded-md border border-gray-200 bg-white p-3 text-sm leading-6 text-gray-800">
                    <div>{renderMessageContent(selected.draft_reply || "No draft reply stored for this message.")}</div>
                  </div>
                  {selected.checkout_link && (
                    <div className="mt-3 flex items-center bg-white/60 dark:bg-black/20 border border-black/5 dark:border-white/5 rounded-lg p-3 backdrop-filter backdrop-blur-md">
                      <div className="bg-blue-600 text-white rounded w-8 h-8 flex items-center justify-center font-bold mr-3">🛍️</div>
                      <div className="flex-1 min-w-0">
                        <p className="text-sm font-semibold text-gray-900 dark:text-white m-0">Product: {selected.proposed_product_id || "Checkout Link"}</p>
                        <a href={selected.checkout_link} target="_blank" rel="noreferrer" className="text-xs text-blue-600 dark:text-blue-400 truncate block">{selected.checkout_link}</a>
                      </div>
                    </div>
                  )}
                </div>

                <div className="grid grid-cols-2 gap-3">
                  <div className="app-card">
                    <div className="app-metric-label">Status</div>
                    <div className="mt-2"><span className={`app-badge ${badgeTone(selected.status)}`}>{formatStatus(selected.status)}</span></div>
                  </div>
                  <div className="app-card">
                    <div className="app-metric-label">Created</div>
                    <div className="mt-2 text-sm font-semibold text-gray-900">{selected.created_at || "Unknown"}</div>
                  </div>
                </div>
                <button
                  onClick={() => handleReadManualReceipt(selected.id)}
                  disabled={isManualBusy}
                  className="app-btn-secondary mt-4 min-h-[44px] min-w-[44px] rounded-[8px]"
                >Check saved reply status</button>
                {previousSend && (
                  <section aria-label="Previous reply attempt" className="mt-4 rounded-md border p-3">
                    <div className="app-metric-label">Previous reply attempt</div>
                    <p>{manualInboxReceiptStatus({ ...previousSend, message_id: selected.id })}</p>
                    <div className="whitespace-pre-wrap">{previousSend.draft_reply}</div>
                  </section>
                )}
                {badgeTone(selected.status) === "warn" && (
                  <div className="mt-6">
                    {(() => {
                      let buttonText = "✨ Approve & Send Draft";
                      let parsedPayload = null;
                      if (activeApproval && activeApproval.payload) {
                        try {
                          parsedPayload = typeof activeApproval.payload === 'string' ? JSON.parse(activeApproval.payload) : activeApproval.payload;
                        } catch { /* Optional local state or response decoding failed; retain the existing fallback. */ }
                        if (parsedPayload && parsedPayload.action_type === "Draft Quote") {
                           let amount = 0;
                           if (parsedPayload.total_amount_cents !== undefined && parsedPayload.total_amount_cents !== null) {
                              amount = parsedPayload.total_amount_cents / 100;
                           } else if (parsedPayload.total_amount !== undefined && parsedPayload.total_amount !== null) {
                              amount = parsedPayload.total_amount;
                           }
                           buttonText = `✨ Send quote for $${amount.toFixed(2)}`;
                        } else if (parsedPayload && parsedPayload.action_type === "Draft Booking") {
                           buttonText = "✨ Approve booking";
                        } else if (parsedPayload && parsedPayload.action_type === "Draft Reply") {
                           buttonText = "✨ Approve & Send Draft";
                        } else if (parsedPayload && parsedPayload.feature_type === "ambassador_reply") {
                           buttonText = "✨ Approve & Send Draft";
                        }
                      }

                      const isInventoryDeduction = selected.draft_reply?.includes("[Send & Deduct Inventory]");
                      if (isInventoryDeduction) {
                        return (
                          <button
                            className="app-button primary w-full min-h-[44px] min-w-[44px] backdrop-filter bg-white/10 glassmorphism shadow-lg bg-gradient-to-r from-green-500/80 to-emerald-600/80 text-white font-bold border border-white/20"
                            onClick={() => handleApproveAndSend(selected.id)}
                          >
                            ✨ Approve & Send (Deduct Inventory)
                          </button>
                        );
                      }

                      return (
                        <button
                          className="app-button primary w-full min-h-[44px] min-w-[44px] rounded-[8px] backdrop-filter bg-white/10"
                          onClick={() => handleApproveAndSend(selected.id)}
                        >
                          {buttonText}
                        </button>
                      );
                    })()}
                  </div>
                )}
                {!activeApproval && (
                  <div className="mt-4 flex flex-col gap-4">
                    <button
                      onClick={() => handleDraftQuoteWithAI(selected)}
                      className="app-button w-full min-h-[44px] min-w-[44px] rounded-[8px] bg-gradient-to-r from-purple-500 to-indigo-600 text-white font-bold shadow-lg hover:from-purple-600 hover:to-indigo-700 transition-all flex items-center justify-center gap-2"
                    >✨ Draft Quote with AI</button>
                  </div>
                )}
              </div>
            )}
          </section>
        </div>
      </div>
    </AppShell>
  );
}

function PowerSyncInboxContent() {
  const { data } = useQuery<Message>("SELECT * FROM omni_inbox_messages ORDER BY created_at DESC");
  const [apiMessages, setApiMessages] = useState<Message[]>([]);

  useEffect(() => {
    fetch('/api/v1/ui/omni_inbox')
      .then((res) => (res.ok ? res.json() : []))
      .then((json) => {
        if (Array.isArray(json)) setApiMessages(json);
      })
      .catch(() => {});
  }, []);

  const messages = data && data.length > 0 ? data : apiMessages;
  return <InboxWorkspace messages={messages} sourceLabel="Loaded securely via PowerSync local embedded DB." />;
}

function InboxLoadingState() {
  return (
    <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
      <div className="mb-2 text-xs text-gray-500">
        Loading conversations for the current workspace.
      </div>
      <div className="app-panel" aria-busy="true">
        <div className="app-empty">Loading inbox messages...</div>
      </div>
    </AppShell>
  );
}

function ApiInboxFallback() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState("");

  useEffect(() => {
    async function loadMessages() {
      setLoading(true);
      setError("");
      try {
        const res = await fetch(`/api/v1/ui/omni_inbox`);
        if (!res.ok) throw new Error("Failed to load inbox messages");
        const data = await res.json();
        setMessages(Array.isArray(data) ? data : []);
      } catch (err) {
        setError(errorMessage(err, "Failed to load inbox messages"));
      } finally {
        setLoading(false);
      }
    }
    loadMessages();
  }, []);

  if (error) {
    return (
      <AppShell title="Unified Inbox" subtitle="Local-first offline unified customer conversations and drafts.">
        <div className="mb-2 text-xs text-gray-500">
          Current workspace conversations could not be loaded.
        </div>
        <div className="app-panel" data-testid="inbox-settled">
          <div className="app-empty">{error}</div>
        </div>
      </AppShell>
    );
  }

  if (loading) {
    return <InboxLoadingState />;
  }

  return <InboxWorkspace messages={messages} sourceLabel="Live inbox messages for the current tenant." />;
}

export default function InboxPage() {
  return (
    <Suspense fallback={<InboxLoadingState />}>
    <PowerSyncProvider
      fallback={<InboxLoadingState />}
      unsupportedFallback={<ApiInboxFallback />}
    >
      <PowerSyncInboxContent />
    </PowerSyncProvider>
    </Suspense>
  );
}
