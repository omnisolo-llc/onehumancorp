outcome: blocked
issue_title: Implement Omnichannel Identity Resolution and Unified AI Inbox for AgentFeed
issue_description: |
  Blocked due to lack of evidence and explicit strategy approval.
  The issue requests new channels (Instagram DMs, WhatsApp, SMS, Email) and Identity Resolution.
  Per RESEARCH.md: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  This issue introduces new channels and expanding omnichannel workflows without evidence that it's prioritized for the current bounded pilot.
  Also, existing code already has omnichannel webhook and agent feed cards (e.g. `src/server/api/omnichannel_webhook.rs`, `src/ui/tauri/src/components/AgentFeedCard.tsx`), meaning this functionality partially exists and this issue might be conflicting scope or superseded.
issue_priority: P0
issue_category: backend
issue_type: feature
issue_label: pending
assignees: []
