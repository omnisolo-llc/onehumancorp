"use client";

import { useMemo, useState } from "react";
import { AppShell } from "../components/AppShell";
import { TriageFeed } from "../dashboard/TriageFeed";

export default function TriagePage() {
  return (
    <AppShell
      title="Work Triage"
      subtitle="AI-prioritized inbox and action center."
    >
      <TriageFeed />
    </AppShell>
  );
}
