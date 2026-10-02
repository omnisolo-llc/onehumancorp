outcome: no_work
issue_title: "🎨 Canvas: [blocked no-work finding: Native Rust Omnichannel Inbox & AI Routing]"
issue_description: |
  **Verified trace limitations:**
  - The issue requests the removal of Chatwoot and the implementation of a Native Rust Omnichannel Inbox.
  - Review of the codebase reveals that Chatwoot was previously completely removed (documented in `docs/reports/production_agent_optimization_report.md` under `CHAT-00 — Chatwoot removal` on 2026-07-13).
  - The native Rust omnichannel inbox is already implemented and exists in `src/server/integrations/omnichannel` and `src/ui/next/src/app/inbox/page.tsx`, complete with Playwright E2E tests (`src/ui/next/src/e2e/omni_inbox.spec.ts`, `src/ui/next/src/e2e/omni_inbox_triage.spec.ts`).
  - Thus, the requested feature is already implemented, resulting in a no-work finding.
  - `make test` and `make lint` failed to complete due to missing node modules (`next` and `pg`), timeouts, formatting diffs, and missing glib. These do not invalidate the no-work finding itself.

  **Executed test commands:**
  - `make test` (Failed)
  - `cd ../../../ && make test` (Failed)
  - `cd /app && make test` (Failed)
  - `cd ../../ && make test` (Failed)
  - `cd /app && make test` (Timed out)
  - `make test-backend` (Timed out)
  - `cargo check --locked --workspace --exclude app --all-targets` (Passed)
  - `cd src/ui/next && npm run lint:node -- --ignore-pattern ".scratch/**"` (Failed)
  - `npm run lint -- --ignore-pattern ".scratch/**"` (Failed)
  - `cd /app && npm run lint:node -- --ignore-pattern ".scratch/**"` (Failed)
  - `cd /app && make test` (Failed)
  - `cd /app && make lint` (Failed)
  - `cd /app && make lint` (Failed)
  - `make test` (Failed)
  - `make lint` (Failed)

  **Loaded Superpowers revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)**
