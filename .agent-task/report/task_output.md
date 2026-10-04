outcome: blocked
issue_title: "Implement Unified Agentic Autonomous Booking & Resource System for Mobile-First Operations"
issue_description: |
  **Task Target:** GitHub Issue #31982: Implement Unified Agentic Autonomous Booking & Resource System for Mobile-First Operations.

  **Trace Analysis & Findings:**
  1. The user request asks to "Implement the OHC Native Agentic Booking System targeting Leo the Music Tutor", focusing on service selection, availability blocking, deposits via Stripe, and integration with the Operations and Sales AI Agents.
  2. A review of `RESEARCH.md` (Revision: 2026-09-18-usage-audit) establishes the "Current scope: read current code, owner stories and provider products, establish existing capabilities and gaps, and evaluate compute/API charging and BYOK. Evidence comes before another concrete product plan."
  3. `RESEARCH.md` explicitly states: "The earlier $99 subscription, $299 setup, 300-step allowance, fixed cohort and margin targets are SUSPENDED HYPOTHESES, including wherever role prompts below repeat them. Do not implement them as billing or segment constraints." It also notes: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  4. The request outlines a broad epic for an "agentic autonomous booking system" for a specific persona (Leo), new database tables (`Service`, `Resource`, `AvailabilityBlock`, `Booking`), and UI/UX flows.
  5. According to `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md`, "New epics need an explicit evidence-backed decision". We must "prove a reusable operating loop" before expanding, and "Expand from evidence: add vertical-specific fulfillment... only when a current issue demonstrates why the reusable loop cannot satisfy the observed need." The capability map in `RESEARCH.md` notes that for booking ("Reserve capacity" / "P0 correctness / P1 completion"), we must first fix existing defects (OHC-05, OHC-01) before launching a completely new system. `docs/research/native_migration_and_remediation.md` confirms these items are still blocked or in progress.
  6. The prompt itself is an older role catalog description ("OmniSolo: an AI team that runs a small business" / "Decision: Start with solo web, design, and marketing service professionals") which `RESEARCH.md` marks as "Superseded strategy draft — retained for traceability... The material below is historical hypothesis/reference, not the current dispatch contract."
  7. Therefore, implementing this new booking system epic is unauthorized without prior evidence and completion of existing remediation gates.

  **Outcome:** The task is blocked. The requested feature represents a new epic that requires explicit evidence-backed authorization and completion of prerequisite remediation work (as mandated by `RESEARCH.md` and the usage audit), which is not present.

  **Superpowers Workflow:**
  Loaded skills:
  - `skills/using-superpowers/SKILL.md`
  Revision: acac59c9db8ec67cb0be1838f85f2ad9d850c8b7
