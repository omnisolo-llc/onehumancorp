issue_title: "Implement the Unified Agent Feed (Mobile MVP)"
issue_description: |
  # Research Report: Unified Agent Feed (Mobile MVP)

  ## Problem Statement
  Legacy e-commerce platforms (like Shopify and Wix) rely on complex, desktop-first admin dashboards. When small business owners (like Maya the baker or Carlos the handyman) attempt to run their business from their phones, they are forced into clunky, cramped "companion apps" that fail to facilitate complex operations (e.g., setting up discounts, managing subscriptions, drafting emails). They need a mobile-first operations paradigm that shifts from "complex forms" to "agent-driven approval."

  ## Research Report
  - **Market Context:** Mobile-first creators thrive on Link-in-Bio tools (Linktree, Stan Store) because of their extreme simplicity, but these tools lack deep business features (inventory, POS, CRM).
  - **The Mobile Management Gap:** Shopify's mobile app is great for viewing stats but terrible for editing. OHC must enable 100% of operations on a 375px screen.
  - **The Solution:** The "Approval" Interface Paradigm. Instead of navigating menus to configure a discount, the user gives a natural language command ("Run a 20% off sale..."). The relevant Agent (Marketing) drafts the logic and presents a simple "Card" in a vertical feed. The user just taps "Approve".

  ## Design Doc
  - **Architecture:** A unified feed service that aggregates actionable items from various AI Agents (Operations, Marketing, Advisory).
  - **Mobile UX Flow (375px first):**
    - The main dashboard is a vertical scrollable feed of "Agent Proposals" and "Urgent Tasks".
    - Each item is a card. Cards use OHC Premium Tokens (macOS Translucent Glass materials, Ubiquiti UniFi modular layouts).
    - Touch targets are strictly >= 44x44px. No horizontal scrolling.
  - **AI Agent Integration:**
    - Operations Agent creates cards for unfulfilled orders or low stock.
    - Marketing Agent creates cards proposing new campaigns or social posts.
    - Advisory Agent creates cards highlighting anomalies or suggesting business optimizations.

  ## Implementation Prompt
  **Objective:** Build a mobile-first (375px) "Unified Agent Feed" that replaces the traditional complex admin dashboard. This feed presents actionable cards from all OHC Agents.

  **User-Facing Outcome:** When the user opens the OHC app, they see a vertical feed of "Agent Proposals" and "Urgent Items" instead of a static graph and a complex hamburger menu.

  **Critical User Journey (CUJ):**
  1. User logs in and lands on the home dashboard (simulated 375px screen).
  2. The feed displays actionable cards from different agents:
     - *Card 1 (Operations)*: "3 new orders to fulfill. [Fulfill Now]"
     - *Card 2 (Advisory)*: "It's been 30 days since your last promo. Should I draft an email? [Yes, draft it]"
  3. User taps "Yes, draft it" on Card 2.
  4. The card expands or transitions to show the AI-drafted content, with an "Approve & Send" button at the bottom.

  **Acceptance Criteria:**
  - Fully responsive, mobile-first design targeting 375px viewports.
  - Integration with the backend Agent service to fetch real actionable items (no mock data in production).
  - Proper styling using OHC design tokens.
  - At least 5 Playwright E2E tests covering this CUJ.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
