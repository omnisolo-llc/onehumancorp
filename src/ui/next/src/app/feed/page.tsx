"use client";

import { AppShell } from "../components/AppShell";
import { UnifiedAgentFeed } from "./UnifiedAgentFeed";

export default function FeedPage() {
  return (
    <AppShell>
      <main className="p-4 md:p-6 w-full max-w-md mx-auto min-h-screen">
        <UnifiedAgentFeed />
      </main>
    </AppShell>
  );
}
