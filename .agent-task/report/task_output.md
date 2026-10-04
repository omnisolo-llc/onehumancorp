---
outcome: no_work
issue_title: "Integrate Stripe Terminal / Tap to Pay for In-Person Payments"
issue_description: |
  The requested Stripe Terminal integration is already fully implemented.
  Evidence:
  - `src/server/integrations/stripe/terminal.rs` contains the exact backend ConnectionToken and PaymentIntent logic requested.
  - `src/ui/next/src/app/pos/terminal/StripeTerminalClient.tsx` provides the Tap to Pay and Send link UI.
  - `src/server/api/billing_webhook.rs` handles the in-person transaction webhooks (`terminal.reader.action.succeeded`).
  - `src/server/api/terminal_api.rs` correctly deducts inventory via `InventoryService` (`catalog.rs` manages the `inventory_count` field).
  - The E2E tests (`src/e2e/e2e_pos_flow.mock-contract.ts`) explicitly cover the offline queueing and simulated offline reader logic.

  Workflow provenance:
  - Superpowers revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Loaded skills: `skills/using-superpowers/SKILL.md`, `skills/brainstorming/SKILL.md`
---
