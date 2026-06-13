"use client";

import { AppShell } from "../components/AppShell";
import { TriageFeed } from "../dashboard/TriageFeed";

function tenantId() {
  if (typeof window === "undefined") return "default";
  return localStorage.getItem("tenant_id") || localStorage.getItem("tenant") || "default";
}

export default function TriagePage() {
  return (
    <AppShell
      title="Work Triage"
      subtitle="AI-prioritized inbox and action center."
    >
      <TriageFeed tenantId={tenantId()} />
    </AppShell>
  );
}
