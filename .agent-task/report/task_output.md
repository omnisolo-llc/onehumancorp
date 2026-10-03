outcome: no_work
issue_title: "Implement Intelligent Tap-to-Pay Visibility and Cash/External Logging for Physical Retail (Priya)"
issue_description: |-
  The requested feature for Tap-to-Pay and Physical Retail logging (the "Priya" Gap) is already substantially implemented in the codebase.

  Evidence of existing implementation:
  - `src/ui/next/src/app/pos/page.tsx`: Contains the "POS Terminal" UI with support for adding items to cart, offline detection, local storage queueing (`pos_offline_queue`), and processing payments via Tap-to-Pay endpoints (`/api/v1/terminal/charge`).
  - `src/ui/next/src/app/pos/omnichannel/page.tsx`: Contains the "New In-Store Sale" UI, creating omnichannel carts and integrating `StripeTerminalClient`.
  - Database schema: `src/server/migrations/076_pos_offline_transactions.sql` handles pending offline transactions, and `src/server/db/migrations/162_edge_ledger.sql` contains `edge_ledger_transactions` handling `tenant_id`, `device_signature`, and offline modes.
  - End-to-end tests: `src/e2e/e2e_pos_flow.md` outlines the test suite, while files like `src/e2e/pos_tap_to_pay.mock-contract.ts`, `src/ui/next/src/e2e/pos_terminal.mock-contract.ts` cover offline syncing, Stripe intent mocking, and the core Tap-to-Pay interaction flows.
  - Persona Context: "Priya" is already hardcoded into onboarding flows (`src/server/services/onboarding/personas.rs`), dummy UI tests, and various mock contracts.

  As per the system instructions, when a feature requested in an issue is already complete or requires unavailable authorization/evidence, we must return a no_work or blocked outcome instead of inventing dummy changes or duplicate features.

  Skills checked during analysis:
  - `.agent-scratch/superpowers/skills/using-superpowers/SKILL.md` (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - `.agent-scratch/superpowers/skills/writing-plans/SKILL.md` (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - `.agent-scratch/superpowers/skills/brainstorming/SKILL.md` (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
