issue_title: "Implement Unified Agent Feed (Mobile MVP)"
issue_description: |
  **Problem Statement:**
  Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by traditional complex admin dashboards. They need a mobile-first (375px) "Unified Agent Feed" that presents actionable cards from all OHC Agents (Marketing, Operations, Advisory) directly on their mobile devices, turning "complex software" into an intuitive "approval workflow."

  **Research Report:**
  Our market research (e.g., `docs/business/market_research/ohc_smb_mobile_first_design_research.md` and `docs/business/market_research/agent_feed_deep_dive.md`) indicates that legacy platforms (Shopify, Wix) fail on mobile because they try to cram desktop paradigms into small screens. Link-in-bio tools succeed through simplicity but lack robust business features. OHC's key differentiator is "AI as an invisible employee." The Unified Agent Feed is the central nervous system for this, proactively pushing critical updates, suggested actions, and drafted communications to the user's mobile device for quick review and approval.

  **Design Doc:**
  *Architecture:*
  - A backend event ingestion pipeline (webhook, internal state changes) triggers LLMs to classify intent and draft actions.
  - These drafted actions are persisted as `AgentFeedItem` records associated with the user's tenant.
  - The frontend fetches these items and displays them in a unified feed.

  *Mobile UX Flow (375px first):*
  1. User opens the OHC app and sees the "Unified Agent Feed" section at the top.
  2. The feed displays a vertical list of actionable cards.
  3. Example Cards:
     - "3 new orders to fulfill. [Fulfill Now]"
     - "It's been 30 days since your last promo. Should I draft an email? [Yes, draft it]"
     - "Drafted reply to customer on Instagram. [Approve & Send]"
  4. Interactions: Tapping an action button processes the request (e.g., approving a draft) and optimistically removes/updates the card.
  5. The UI must use OHC Premium Tokens (clean Apple/Ubiquiti-style hierarchy, translucent materials) and guarantee a minimum 44x44px touch target for all interactive elements.

  **Implementation Prompt:**
  Build a mobile-first (375px) "Unified Agent Feed" component for the OHC dashboard.
  - Ensure the feed renders correctly on small screens without horizontal scrolling.
  - Display actionable agent cards (e.g., drafted replies, operations alerts).
  - Implement 1-tap "Approve" or "Action" buttons on these cards (minimum 44x44px touch targets).
  - Use optimistic UI updates when an action is taken.
  - Include automated E2E Playwright tests verifying the agent feed rendering and interaction on a mobile viewport.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
