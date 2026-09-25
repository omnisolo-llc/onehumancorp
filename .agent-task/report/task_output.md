issue_title: "Audit of Standing Authority and Cost Control Documentation vs Implementation"
issue_description: >
  # Documentation Audit: Standing Authority and Cost Control

  **Date:** 2026-09-19
  **Auditor:** Principal Technical Writer & Scribe (L7)
  **Superpowers:** loaded `skills/using-superpowers` and `skills/writing-plans`, `skills/executing-plans`, `skills/brainstorming` from revision 5bf4e78011075bcfc0dc295f0724994cd123ee71

  ## Overview
  This report audits the current state of the codebase against the "Standing Authority and Cost Control" documentation requirements outlined in `RESEARCH.md` (Revision: 2026-09-18-usage-audit) and `docs/research/business_capability_and_usage_economics_audit.md`. The goal is to document how an owner accomplishes verified business work with the current product, specifically regarding standing authority, cost, exceptions, and recovery.

  ## 1. Standing Authority
  **Requirement:** Routine work inside standing authority should execute without repeated approval dialogs. Where an action exceeds granted authority... finish all preparatory work and expose that exact dependency. (RESEARCH.md:219)

  **Implementation Status:**
  - **Tooltips:** In-app help tooltips are implemented across the UI (e.g., `src/ui/next/src/components/TooltipRegistry.tsx`). However, these are mostly descriptive UI elements.
  - **Interactive Walkthroughs:** `InteractiveWalkthrough` (`src/ui/next/src/components/Walkthrough.tsx`) and `useWalkthrough` exist to guide users.
  - **Help Chat Widget:** The `omnisolo-floating-help-widget` (`src/ui/next/src/components/help.tsx`) provides an "Ask AI Help" interface.
  - **Authority Enforcement:** The frontend code (`src/ui/next/`) focuses heavily on UI presentation (tooltips, walkthoughs). The backend must enforce standing authority (as noted in RESEARCH.md: "Enforce tenant/client scoping, owner authority, hard budgets, cancellation and revocation in server-side tools"). The audit in `docs/research/business_capability_and_usage_economics_audit.md` (F12) notes that while some specific fixes were made (e.g., "Invoice-context and job-generation stubs no longer invent completed invoices"), unsupported workflows remain unavailable and "These are specific fixes, not certification of all simulation/approval paths."

  **Documentation Gap:** Currently, the in-app help center and UI elements do not clearly explain to the owner *how* standing authority is configured or enforced on the backend. The UI tools exist (Help Center, Tooltips), but the content explaining the authority boundaries is missing.

  ## 2. Cost Control and Tracking
  **Requirement:** Measure OHC-funded inference, active/reserved compute... Keep customer-direct provider bills separate. (RESEARCH.md)

  **Implementation Status:**
  - **Budgeting:** `src/server/pricing/budget.rs` exists, but the audit (Finding C) states it "increments spend first and reports false after exceeding the limit; its test describes a soft limit. This helper alone does not atomically reserve funds before a provider request."
  - **Usage Attribution:** The audit (Finding D) notes that `services/billing/auditor.rs` lacks provider/model, request ID, and payer mode. Some paths (like the proposal LLM adapter) return `Usage::default()`.
  - **Global Totals Issue:** `services/billing/service.rs` uses global cost snapshots while labeling the response with the requested organization (Finding B).

  **Documentation Gap:** The documentation cannot accurately describe robust cost control or hard spend reservations because the underlying implementation does not yet fully support them. The current cost dashboards should not be documented as "invoice-grade meters".

  ## 3. Exceptions and Recovery
  **Requirement:** Recover after app/worker restarts... A provider outage leaves a visible unresolved task with a retry time, not an invented result. (RESEARCH.md:217)

  **Implementation Status:**
  - Finding A in the economics audit identifies an unbounded repeated processing issue per input event in the billing auditor.

  **Documentation Gap:** The documentation needs to reflect that the system currently handles some state persistence but has known issues (like the billing event loop) that could impact recovery and accurate reporting.

  ## Conclusion and Blockers
  The current implementation does not yet fully support the robust standing authority enforcement, hard budget reservations, and invoice-grade usage telemetry required by the product vision.

  **No-Work / Blocked Outcome:**
  We cannot document these features as fully functional for the user until the underlying backend mechanisms (atomic budget reservations, comprehensive usage tracking across all model paths, and strict tenant isolation in billing reports) are implemented and verified. The documentation should reflect the *current* capabilities, which are still under development in these specific areas.

issue_priority: "P1"
issue_category: "Documentation"
issue_type: "Audit"
issue_label: ["documentation", "audit", "blocked"]
assignees: []
