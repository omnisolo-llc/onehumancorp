outcome: no_work
issue_title: Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)
issue_description: |
  # Research Report

  ## Objective
  Implement a Native Rust Omnichannel Chat System as a replacement for Chatwoot, providing functional parity with Chatwoot's core models (Inbox, Conversation, Message, Contact) built on OHC's stack (Rust, PostgreSQL with RLS).

  ## Findings
  An exhaustive audit of the OneHumanCorp source code reveals that the requested features have already been fully implemented.

  1. **SQL Schemas and RLS Policies:**
     The file `src/server/migrations/233_chat_omnichannel.sql` establishes the exact requested schemas (or their functional equivalents under the `chat_` prefix rather than `omni_`, e.g., `chat_inboxes`, `chat_conversations`, `chat_messages`, `chat_contacts`) and enforces deep multi-tenant boundaries using PostgreSQL Row Level Security (RLS) with `tenant_id = current_setting('app.current_tenant_id', true)::uuid`. Additionally, tables such as `omni_inbox_messages` have RLS implemented across multiple migration files (e.g., `146_omni_inbox_rls.sql`, `1001_create_omni_inbox_messages_and_quotes_fix.sql`).

  2. **Core Service Logic and Messaging Capabilities:**
     The core omnichannel logic is found in files like `src/server/domain/inbox.rs` and `src/server/lib.rs`. `inbox.rs` handles the execution of automated/AI-drafted replies (retrieved via `generated_response` or `draft_reply`) and routes outbound messages through integrations like Twilio (WhatsApp/SMS) and Meta (Instagram/Facebook).

  3. **AI Integration Points:**
     The orchestrator (`src/server/orchestration/departments/orchestrator.rs`) and the repository patterns (`src/server/domain/repository/agent_feed_repo.rs`) contain the necessary logic to push interactions into the AI job queue and pull AI-generated draft replies for owner approval (the triage feed). The E2E tests inside `src/e2e/omni_inbox_triage.mock-contract.ts` and `src/e2e/tests/omni_inbox_differentiation.mock-contract.ts` comprehensively verify the triage flow, 1-tap approvals, edits, and dismissals of AI drafts from omnichannel webhooks.

  ## Conclusion
  The "Native Rust Omnichannel Chat System" is already natively integrated, fully conforming to OHC's stack requirements (Rust, PostgreSQL with RLS). Reimplementing it would duplicate existing, battle-tested code and violate the core directive to not invent dummy changes or rewrite functional systems without evidence of a gap.

  Since the functionality requested in Issue #36311 is already fully implemented, this task results in a `no_work` outcome.
