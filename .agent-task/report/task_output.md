issue_title: '🚀 Nova: Growth and Virality Audit - Existing Implementation Review'
issue_description: |
  # Growth and Virality Audit

  ## Active Business-Capability Map Context
  This audit is performed in the context of the Principal Growth Engineer & Nova (L7) role.
  The goal is to evaluate the existing implementation of growth features, particularly looking for a current issue or research uncertainty that can be addressed to improve customer acquisition or retention.

  ### Workflow Context
  - The focus is on finding an evidence-backed bottleneck in the client-to-cash funnel.
  - OHC growth is viewed as a byproduct of user success.

  ## Existing Implementation Inventory

  Based on a codebase scan, several growth-related modules and E2E tests exist, suggesting previous implementations of various growth loops:

  *   `src/e2e/growth-milestones.spec.ts`
  *   `src/e2e/milestone_alerts.spec.ts`
  *   `src/e2e/flash-sale-generator.spec.ts`
  *   `src/e2e/viral_expandable_badge.spec.ts`
  *   `src/e2e/zero_click_builder.spec.ts`
  *   `src/e2e/viral_affiliate_marketing.mock-contract.ts`
  *   `src/e2e/growth_referral_widget.spec.ts`
  *   `src/e2e/win_back_growth_loop.spec.ts`
  *   `src/e2e/share_to_unlock_generator.spec.ts`
  *   `src/e2e/post-purchase-share.spec.ts`
  *   `src/e2e/referral_widget.spec.ts`
  *   `src/e2e/growth-cloud-bridge.spec.ts`
  *   `src/e2e/gift_cards_growth_loop.spec.ts`
  *   `src/e2e/referral_click_tracking.spec.ts`
  *   `src/e2e/lead_magnet.spec.ts`
  *   `src/e2e/viral_milestones.mock-contract.ts`
  *   `src/e2e/tip_jar_growth_loop.mock-contract.ts`
  *   `src/e2e/viral_trial_extension.spec.ts`
  *   `src/e2e/viral_agent_card_loop.spec.ts`
  *   `src/e2e/viral_before_after_slider.mock-contract.ts`
  *   `src/e2e/spin_to_win_generator.spec.ts`
  *   `src/e2e/viral-secret-menu-generator.spec.ts`
  *   `src/e2e/link-in-bio.spec.ts`
  *   `src/e2e/birthday_club.mock-contract.ts`
  *   `src/e2e/viral_countdown_drop.mock-contract.ts`
  *   `src/e2e/viral_nps_feedback.mock-contract.ts`
  *   `src/e2e/viral-waitlist-generator.spec.ts`
  *   `src/e2e/viral_invite_loop.spec.ts`
  *   `src/e2e/viral_ai_savings_widget.spec.ts`
  *   `src/e2e/hybrid_landing_marketing.spec.ts`

  ## Outcome

  **Blocked/No-Work Finding**

  While there is a vast array of mock contracts and E2E specs for viral loops, widgets, and referral mechanisms, the `RESEARCH.md` contract explicitly states:

  > "The earlier $99 subscription, $299 setup, 300-step allowance, fixed cohort and margin targets are SUSPENDED HYPOTHESES... Do not interpret public anecdotes as customer interviews."
  >
  > "Current scope: read current code, owner stories and provider products, establish existing capabilities and gaps, and evaluate compute/API charging and BYOK. Evidence comes before another concrete product plan."
  >
  > "Select one existing issue or research uncertainty; do not reopen repaired findings or treat historical pricing/segment targets as requirements."
  >
  > "For each assigned issue, inspect actual funnel evidence, current implementation and existing issues. Implement one current-stage improvement only when justified. Track denominators, consent, attribution limitations, delivery quality and cost. A no-work finding is valid; do not manufacture a viral feature."

  **Blocked Prerequisites:**

  There is no current "actual funnel evidence" or "assigned issue" with concrete data showing a bottleneck in the client-to-cash funnel that justifies implementing a new growth loop or modifying an existing one at this moment. The strategic directive is to focus on core operational correctness (F01-F15), reliable billing/costing, and a proven end-to-end service loop before expanding "growth" mechanics. Implementing or fixing a viral widget without this baseline evidence violates the core directive to avoid manufacturing a viral feature.

  ### Required Flow Screenshot/Diagram
  ```mermaid
  graph TD
      A[Audit Start] --> B[Review Codebase for Growth Features]
      B --> C[Identify Extensive List of Growth E2E Specs]
      C --> D[Consult RESEARCH.md Contract]
      D --> E{Is there explicit funnel evidence for a bottleneck?}
      E -- No --> F[Block further implementation]
      F --> G[Report No-Work Finding due to missing prerequisites]
  ```

  ### Superpowers Workflow Provenance
  - Loaded Skills: `skills/using-superpowers/SKILL.md`, `skills/brainstorming/SKILL.md`
  - Repository URL: https://github.com/obra/superpowers.git
  - Revision Hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks Performed: `grep` for "growth" in the codebase, review of `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md`.
  - Outcomes: Identified that while many growth-related tests exist, there is no explicit funnel evidence to justify a new feature or modification per the current strategic constraints. A 'no-work' report is being filed.

issue_priority: P2
issue_category: growth
issue_type: research
issue_label: ohc:lane:growth
assignees: []
