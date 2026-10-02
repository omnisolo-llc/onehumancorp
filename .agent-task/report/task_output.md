outcome: no_work
issue_title: OHC Native Omnichannel Customer Support & Chat Engine
issue_description: |
  The requested omnichannel engine functionality is already fully implemented in the current repository. The existing implementation replaces the deprecated Chatwoot dependency with a native, built-in omnichannel engine, including multi-tenant message ingestion and a "Work Triage" UI with AI-suggested draft replies.

  The existing implementation files are:
  - Backend message ingestion and AI queue routing: `src/server/api/omnichannel_webhook.rs`
  - Frontend Unified Work Triage feed with AI draft review/approval capabilities (375px mobile-first layout): `src/ui/next/src/app/triage/page.tsx`
  - UI Card implementations for the triage dashboard: `src/ui/next/src/app/dashboard/UnifiedAgentFeed.tsx`, `src/ui/next/src/app/dashboard/InstagramDMCard.tsx`
  - AI prompt orchestration for the inbox: `src/server/orchestration/departments/customer_success_agent.rs`
  - Database schemas handling the omnichannel routing and RLS filtering: `src/server/db/migrations/1001_create_omni_inbox_messages_and_quotes_fix.sql`

  Since the feature is already completed, integrated, and verified to exist within the codebase matching the required specifications in issue #36325, this task is marked as a "no-work" finding to prevent unnecessary reimplementation or code duplication.

  Workflow provenance:
  Superpowers repository loaded at revision `8ca22dba9a94f28898bbce59f2537ff4d87c747d`.
