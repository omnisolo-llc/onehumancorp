issue_title: "[Research] OHC Mobile-First Unified Agent Feed for 375px Operations"
issue_description: |
  # Research Report: OHC Mobile-First Unified Agent Feed

  ## Problem Statement
  Legacy commerce platforms (Shopify, Wix) treat mobile apps as supplementary dashboards for viewing stats and fulfilling basic orders, while requiring a desktop browser for complex store building, campaign creation, and deep management. Non-technical owner/operators (e.g., Fatima the food cart owner, Carlos the handyman) operate entirely from their phones. They need a system that translates complex operations into simple, actionable steps on a 375px screen without requiring manual configuration of forms with 20 toggles.

  ## Research Report
  Based on competitive analysis:
  - **Traditional Builders:** Shopify's mobile app is great for stats but poor for store design or complex setup.
  - **Link-in-Bio Tools:** Tools like Linktree or Stan Store succeeded because of extreme mobile simplicity, but they lack the depth needed for a full business operation (inventory, agentic workflows, multi-channel support).
  - **The OHC Differentiator:** The solution is not making complex forms responsive. The solution is **Agent-Driven Approval UI**. Instead of the user navigating a complex dashboard, OHC agents (Operations, Marketing, Advisory) proactively push Action Cards to a Unified Feed. The user simply taps "Approve", "Edit", or "Discard".

  ## Design Doc

  ### Architecture Overview
  - **Frontend:** Flutter/PWA application strictly designed for 375px viewports first.
  - **UI Shell:** The primary landing screen is the "Unified Agent Feed" (not a traditional static dashboard).
  - **Design System:** Employs OHC Premium Tokens (Glassmorphism containers `rgba(255, 255, 255, 0.65)` with `backdrop-filter: blur(30px) saturate(210%)`, rounded corners `16px`, and minimum 44x44px touch targets).

  ### Mobile UX Flow (375px)
  1. **Landing:** Owner opens the app and sees the "Unified Agent Feed".
  2. **Triage:** The feed presents a prioritized list of Action Cards.
     - *Urgent Card (Operations)*: "3 new orders need fulfillment."
     - *Proposal Card (Marketing)*: "Drafted an Instagram post for the new vegan cake."
     - *Insight Card (Advisory)*: "Revenue is up 15% this week. Consider running a weekend promo."
  3. **Action:** Owner taps a primary CTA on a card (e.g., "Approve & Post").
  4. **Execution:** The card shows a micro-loading state, then a success confirmation (e.g., "Post Scheduled").

  ### AI Agent Integration
  - Agents in the backend (using the `minimax.reason()` or similar LLM pipelines) ingest events (e.g., webhook from Stripe, inventory alert, calendar gap) and generate these Action Cards.
  - The feed acts as the presentation layer for the asynchronous AI job queue.

  ## Implementation Prompt
  **Mission:** Build the Mobile-First Unified Agent Feed UI Shell.
  **CUJ:**
  1. The user logs in and lands on the Unified Agent Feed on a 375px screen.
  2. The feed displays at least three distinct types of Action Cards (Operations Task, Marketing Proposal, Business Insight) populated from real backend data (or documented seed data paths).
  3. The user taps the primary action button on a Marketing Proposal card.
  4. The card transitions to a detailed view (e.g., showing the drafted email text) with an "Approve" button.
  5. The user taps "Approve", the UI shows a pending state, and then a success state, representing the agent executing the task.
  **Acceptance Criteria:**
  - Strict adherence to 375px width (no horizontal scrolling).
  - All interactive elements have >44px touch targets.
  - UI uses the specified Glassmorphism premium tokens.
  - E2E Playwright test verifies this exact flow from login to approval on a mobile viewport.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
