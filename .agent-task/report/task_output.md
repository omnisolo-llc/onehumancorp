outcome: no_work
issue_title: Implement Universal Offline-First Tap-to-Pay POS
issue_description: |
  # Finding: No Work (Blocked / Out of Scope)

  The issue requests building a universal offline-first Tap-to-Pay POS using Flutter for the mobile app (iOS/Android), SQLite for local storage, and a Go backend with a Stripe Terminal integration.

  ## Evaluation against 2026-09-18-usage-audit Contract Constraints

  The current One Human Corp strategy (`RESEARCH.md`, revision 2026-09-18-usage-audit) explicitly states:

  1.  **Codebase/Tech Stack Conflict:** The product uses a Rust backend + Tauri/Next.js desktop shell, not Flutter and Go. "The supported build is Cargo/Rust + Next.js/Node + Tauri, not Bazel. ... Do not dismiss Next paths from a legacy label or create another frontend/rewrite." (from `docs/development/native-build.md`).
  2.  **Explicit Scope Gate:** The `RESEARCH.md` document clearly states: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  3.  **No POS Engine in Active Scope:** The current active business-capability map focuses on digital-service and consulting loops (quotes, digital delivery, invoicing). Although retail POS and physical operations are noted as *future* deferred capabilities (e.g., "Maker, baker, boutique", "Food/preorder operator"), they are currently marked as "After reliable payment and reservation foundations" and are not part of the active development scope without explicit new evidence.
  4.  **No Hardware Mandate:** Native mobile builds and POS terminal functionality are explicitly identified as outside the immediate headless testing loop, with mobile builds relying on `npm run mobile:android/mobile:ios`. A full offline-first Flutter rewrite contradicts the mandate to "Reuse the current Rust backend and trace actual UI packaging".

  ## Superpowers Workflow Provenance
  - Loaded skills:
    - `skills/using-superpowers/SKILL.md`
    - `skills/brainstorming/SKILL.md`
    - `skills/writing-plans/SKILL.md`
  - Upstream commit revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
