
"use client";

import { AppShell } from "../components/AppShell";
import { UnifiedAgentFeed } from "../dashboard/UnifiedAgentFeed";

export default function InboxPage() {
  return (
    <AppShell
      title="Unified Inbox"
      subtitle="Local-first offline unified customer conversations and drafts."
    >
      <div className="w-full max-w-[375px] mx-auto overflow-hidden">
        <UnifiedAgentFeed />
      </div>
    </AppShell>
  );
}
