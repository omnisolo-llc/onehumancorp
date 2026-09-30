issue_title: "Growth investigation finding: deferring viral loops until core retention is proven"
issue_description: |
  **Research Findings**
  Based on the active business-capability map and scope priorities in RESEARCH.md (Revision: 2026-09-18-usage-audit), referral widgets, share cards, paywalls, and viral loops are explicitly deferred until retained paid value is proven.

  The codebase inspection reveals existing referral and viral loop elements in src/server/api/growth.rs such as handle_one_tap_referral_embed, handle_review_reward_submit, and handle_secret_menu_embed, which provide referral links and share-to-unlock functionalities. There is also OnboardingFunnel tracking in src/proto/hub.proto and src/server/services/growth/service.rs. However, according to the current strict guidance, we must focus on resolving evidence-backed bottlenecks in the current client-to-cash funnel rather than manufacturing new viral features.

  Given no specific evidence-backed retention gap or bottleneck in the current funnel was provided to resolve, and the strict instruction to not manufacture viral features or treat historical pricing targets as requirements, the proper outcome here is a "no-work finding". We will defer implementing any new growth loops until there is measured owner acquisition or retention data that proves retained paid value.

  **Provenance**
  - Superpowers workflow loaded: using-superpowers (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - Brainstorming skill loaded: brainstorming (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
issue_priority: "P2"
issue_category: "Growth"
issue_type: "Research"
issue_label: "ohc:lane:growth"
assignees: []
