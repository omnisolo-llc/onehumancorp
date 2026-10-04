outcome: blocked
issue_title: "[Architecture] Edge-Cached Dynamic Storefronts & Agentic SEO Pre-rendering"
issue_description: |
  The request asks for the implementation of Edge-Cached Dynamic Storefronts & Agentic SEO Pre-rendering, including models like `StorefrontCacheConfig`, `SEOPreRenderJob`, and a `CacheInvalidationService` with mock CDN API calls.

  This task is blocked as it directly conflicts with the OneHumanCorp operating contract (revision 2026-09-18-usage-audit) and the current scope gate in `RESEARCH.md`. Specifically:

  1.  **No Expansion Without Evidence:** The operating contract states: "New epics need an explicit evidence-backed decision... New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md." `RESEARCH.md` prioritizes "Correctness before expansion" and "Measure cost before setting rates", focusing on the existing business-capability map (e.g., F01-F15 remediation). Implementing a new edge-caching and SEO pre-rendering architecture (which touches on new channels/storefront expansion) lacks the required documented owner demand, willingness-to-pay evidence, and strategy approval in `RESEARCH.md`.
  2.  **No Cosmetic Churn or Feature Generation:** The contract explicitly prohibits "forced feature generation" and states "A proposed change needs a current-code inventory, observed owner need and reuse analysis." The request represents a new feature epic based on unvalidated hypotheses (e.g., "viral spikes" for non-technical SMB owners) rather than observed owner needs documented in the audit.
  3.  **Strictly Bound to Current Scope:** The instruction requires resolving existing defects (F01-F15, OHC-01-12) over speculative infrastructure upgrades. Creating new Postgres queues (`SKIP LOCKED`) and CDN invalidation mocks is outside the authorized bounds of the current pilot.

  Therefore, I am reporting this as a blocked/no-work finding, as attempting implementation would violate the explicit constraints of the project governance.

  Skills provenance:
  - Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Loaded skills: skills/using-superpowers/SKILL.md
