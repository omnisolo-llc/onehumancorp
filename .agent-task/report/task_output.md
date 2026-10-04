outcome: no_work
issue_title: "Implement AI Unified Inbox & Agent Feed Differentiation"
issue_description: |
  **Issue**: Implement AI Unified Inbox & Agent Feed Differentiation
  **Evidence**: Verified that the AI Unified Inbox and Omnichannel Customer Memory requirements are already fully implemented. `src/server/api/omnichannel_webhook.rs` handles ingesting external messages and resolving customer identity via `resolve_identity`. `src/server/orchestration/departments/customer_success_agent.rs` hosts The Ambassador agent, which intercepts `tenant.omnichannel.message.received` events, queries past orders/inventory, drafts highly personalized replies via an LLM, and triggers an approval request. This draft populates the `ActionRequiredQueue` and is visible via the `AmbassadorReplyCard` in `src/ui/next/src/app/dashboard/UnifiedAgentFeed.tsx` and `src/ui/next/src/app/feed/page.tsx`. Extensive Playwright tests exist (e.g., `src/ui/next/src/e2e/ambassador-reply.mock-contract.ts` and `src/e2e/e2e_ambassador_rag.mock-contract.ts`) confirming the 375px mobile UI flows and backend endpoints perform identically to the stated acceptance criteria. No further work is required.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
