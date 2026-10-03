"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { useClipboardFeedback } from "@/hooks/useClipboardFeedback";
import { notifyQueueIdentityChange } from "@/lib/sync/queueIdentity";
import { useCloudInvitation } from "../referrals/useCloudInvitation";

type Progress = { phase: "loading" | "ready" | "held"; count: number | null; message: string; receipt?: string };
const targetInvites = 3;
export function UnlockProFeaturesWidget() {
  const [progress, setProgress] = useState<Progress>({ phase: "loading", count: null, message: "Verifying referral progress…" });
  const [retry, setRetry] = useState(0);
  const generation = useRef(0);
  const retire = useCallback(() => {
    generation.current += 1;
    setProgress({ phase: "held", count: null, message: "Your session changed. Reload to verify referral progress." });
  }, []);
  const invitation = useCloudInvitation(retire);
  const owner = invitation.verifiedOwner;
  const userId = owner?.userId; const tenantId = owner?.tenantId; const expiresAt = owner?.expiresAt;
  const confirmedLink = invitation.phase === "created" ? invitation.link : "";
  const clipboard = useClipboardFeedback(invitation.link);

  useEffect(() => {
    if (!userId || !tenantId || !expiresAt) return;
    const current = ++generation.current; const controller = new AbortController();
    const active = () => current === generation.current && !controller.signal.aborted;
    setProgress({ phase: "loading", count: null, message: "Loading recorded invitation progress…" });
    void (async () => {
      try {
        if (expiresAt <= Date.now()) { notifyQueueIdentityChange(); return; }
        const headers = new Headers({ "x-ohc-expected-user": userId, "x-ohc-expected-tenant": tenantId });
        if (headers.get("x-ohc-expected-user") !== userId || headers.get("x-ohc-expected-tenant") !== tenantId) throw new Error("Owner headers cannot be represented");
        const response = await fetch("/api/v1/growth/team-invites/aggregated-metrics", { headers, credentials: "same-origin", cache: "no-store", redirect: "error", signal: controller.signal });
        if (!active()) return;
        if (response.status === 401 || response.status === 403) { notifyQueueIdentityChange(); return; }
        const data = await response.json();
        if (!active()) return;
        if (response.status === 409 && ["session_identity_changed", "queued owner does not match the current session"].includes(data?.error ?? data?.reason)) { notifyQueueIdentityChange(); return; }
        if (expiresAt <= Date.now()) { notifyQueueIdentityChange(); return; }
        if (response.status !== 200 || !data || Array.isArray(data) || data.error != null || (data.success !== undefined && data.success !== true) || !Number.isSafeInteger(data.total_invites) || data.total_invites < 0) throw new Error("Unverified metrics");
        setProgress({ phase: "ready", count: data.total_invites, receipt: confirmedLink, message: "Recorded team invitations. Acceptance and Pro entitlement are not verified by this count." });
      } catch {
        if (active()) setProgress({ phase: "held", count: null, message: "Referral progress is unavailable. Retry to read the current recorded count." });
      }
    })();
    return () => { generation.current += 1; controller.abort(); };
  }, [userId, tenantId, expiresAt, retry, confirmedLink]);
  useEffect(() => {
    if (!owner && invitation.phase !== "verifying") setProgress(previous => previous.phase === "held" ? previous : { phase: "held", count: null, message: "Referral progress is unavailable until your account is verified." });
  }, [owner, invitation.phase]);

  const settled = progress.phase === "ready" && progress.receipt === confirmedLink;
  const count = settled ? progress.count : null;
  const reached = count !== null && count >= targetInvites;
  const busy = progress.phase === "loading" || invitation.phase === "verifying" || (progress.phase === "ready" && !settled);
  const shareUrl = "https://twitter.com/intent/tweet?" + new URLSearchParams({ text: `Join me on OmniSolo OneHumanCorp: ${invitation.link}` });
  return <div data-testid="unlock-pro-features-widget" aria-busy={busy} data-voice-assistant-surface="glass" className="glassmorphism p-6 rounded-2xl mb-6 shadow-sm border border-purple-100 dark:border-purple-900/50 bg-gradient-to-br from-white to-purple-50/50 dark:from-gray-900 dark:to-purple-900/20">
    <div className="flex justify-between items-start mb-4 gap-3">
      <div><h3 className="font-bold text-gray-900 dark:text-white font-outfit text-xl">✨ Unlock Pro Features (Referral Progress)</h3><p className="text-sm text-gray-600 dark:text-gray-300 mt-1">Track recorded invitations. Any associated reward must be verified by the billing service.</p></div>
      {count !== null && <div className="bg-purple-100 dark:bg-purple-900/40 text-purple-700 dark:text-purple-300 text-xs font-bold px-3 py-1 rounded-full">{count} / {targetInvites} Invites</div>}
    </div>
    <p role="status" aria-label="Referral progress status" className="text-sm mb-4">{progress.message}</p>
    {count !== null && <div className="mb-6"><div role="progressbar" aria-label="Recorded invitation progress" aria-valuemin={0} aria-valuemax={targetInvites} aria-valuenow={Math.min(count, targetInvites)} className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-3 overflow-hidden"><div className={reached ? "h-3 bg-emerald-500" : "h-3 bg-indigo-600"} style={{ width: `${Math.min(count / targetInvites * 100, 100)}%` }} /></div></div>}
    {progress.phase === "held" && owner && <button type="button" onClick={() => setRetry(value => value + 1)} className="app-button mb-3">Retry Referral Progress</button>}
    {reached ? <div className="bg-emerald-50 dark:bg-emerald-900/20 border border-emerald-200 rounded-xl p-4 text-center"><h4 className="font-bold">Invite target reached</h4><p className="text-sm">Billing verification is required before any Pro entitlement is applied.</p></div> : <>
      {!invitation.link ? <button type="button" className="app-button" disabled={!settled || invitation.phase !== "ready"} onClick={() => void invitation.create("pending")}>{invitation.phase === "requesting" ? "Creating Invite Link…" : "Create Invite Link"}</button> : settled && <div className="flex flex-wrap gap-3">
        <button type="button" className="app-button" disabled={clipboard.state === "pending"} onClick={() => void clipboard.copy(invitation.link)}>{clipboard.state === "copied" ? "Copied Link!" : clipboard.state === "pending" ? "Copying…" : "Copy Invite Link"}</button>
        <a href={shareUrl} target="_blank" rel="noopener noreferrer" className="app-button">Share on X</a>
      </div>}
      <p role="status" aria-label="Referral invitation status" className="text-sm mt-3">{invitation.message}</p>
      {invitation.link && clipboard.message && <p role={clipboard.state === "error" ? "alert" : "status"}>{clipboard.message}</p>}
    </>}
  </div>;
}
