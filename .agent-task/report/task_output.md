issue_title: "OHC-03 Intent-to-operating-business onboarding (No-Work / Blocked)"
issue_priority: "P1"
issue_category: "feature"
issue_type: "blocked"
issue_label: "ohc:lane:activation"
assignees: []
issue_description: |
  **Title**: OHC-03 Intent-to-operating-business onboarding (No-Work / Blocked)

  **Problem Statement**:
  The goal of the task is to fix the zero-click frontend paths in the Next app, aligning them with the current backend/Tauri setup flow by migrating the chat and input interfaces into the single conversational `/onboarding` SetupWizard. However, this task is blocked due to missing business evidence and owner outcome tracking.

  **Research Report**:
  After thoroughly investigating the repository:
  1. The `Instant Build` (zero-click) flow is already mostly integrated into the main wizard at `src/ui/next/src/app/onboarding/page.tsx` (using `step === -1` and `step === 0`).
  2. The duplicated legacy flows exist at `src/ui/next/src/app/onboarding/zero-click/page.tsx` and `src/ui/next/src/app/zero-click-builder/page.tsx`.
  3. However, according to the `docs/research/native_migration_and_remediation.md` audit, OHC-14 ("economics/owner outcomes") and OHC-15 ("premature exclusive segment") explicitly state they are "Blocked / No-Work due to missing prerequisites and owner economic/metric data".
  4. The OneHumanCorp operating contract specifically instructs: "The earlier $99 subscription, 300-step allowance, $299 setup, fixed cohort/margin targets and exclusive web/design/marketing segment are suspended hypotheses... New epics need an explicit evidence-backed decision; assigned concrete defect work may continue."

  The core onboarding logic involves provisioning user resources and relies heavily on pricing/economics logic. Without resolving the prerequisites mentioned in the audit report (missing economic/metric data), moving forward with refactoring the zero-click onboarding flow into the main wizard violates the directive to not start new epics that lack verified business outcomes and owner outcome data.

  **Blocked Prerequisites**:
  - Resolution of missing owner economic/metric data (OHC-14).
  - Explicit evidence-backed business decision for the onboarding workflow and the AI store generation pricing.

  **Superpowers Provenance**:
  - Read `skills/using-superpowers/SKILL.md`
  - Utilized `skills/writing-plans` and `skills/executing-plans` for analysis.
