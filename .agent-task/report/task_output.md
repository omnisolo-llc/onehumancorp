issue_title: "Compute/API Charging and BYOK Usage Economics UX Audit"
issue_description: |
  **Revision: 2026-09-18-usage-audit**

  **Scope:**
  Evaluated existing code for compute and API charging, customer BYOK, and provider-permitted native subscription access.

  **Findings:**
  - Found existing proxy, native usage events, and cost UI.
  - Discovered `docs/research/business_capability_and_usage_economics_audit.md` which highlights meter feedback, tenancy, attribution, and reservation defects blocking confidence.
  - The current onboarding wizard (`src/ui/next/src/app/onboarding/page.tsx` and `src/ui/next/src/app/website-builder/page.tsx`) does not accurately capture nor reflect managed API and customer-funded modes for cost attribution.
  - The OHC architecture requires sustainable resource rates and a credible BYOK offering. Existing Codex adapter and API/OAuth credential types are reuse assets, not a full demonstration of supported native-client subscription modes.
  - No new UX code changes were made for API billing during this task since the requirement is to evaluate the gaps and report back before a new product plan is approved.

  **Provenance:**
  Skills loaded: superpowers/brainstorming (8ca22dba9a94f28898bbce59f2537ff4d87c747d)
issue_priority: "P1"
issue_category: "research"
issue_type: "audit"
issue_label: ""
assignees: []
