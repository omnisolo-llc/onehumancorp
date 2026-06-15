issue_title: "Unified Agent Feed & Triage Inbox (Mobile-First)"
issue_description: |
  # Unified Agent Feed & Triage Inbox (Mobile-First)

  ## Problem Statement
  Small business owners and operators (like Maya the Baker or Carlos the Handyman) suffer from fragmented workflows. They receive customer DMs on Instagram, booking requests on WhatsApp, inventory alerts via email, and payment failures in Stripe. Traditional e-commerce platforms (like Shopify or Wix) offer complex, desktop-first dashboards that require the user to hunt for these issues. Owners need a single, unified "inbox" that surfaces critical tasks and, more importantly, provides agent-drafted solutions ready for one-tap approval on a mobile device.

  ## Research Report
  ### The Market Gap
  - **Legacy Platforms (Shopify, Wix):** Their mobile apps are "companion" apps. While good for viewing basic stats or fulfilling simple orders, they fail at complex operations. Setting up discounts or responding intelligently to varied customer queries requires returning to the desktop or juggling multiple third-party apps.
  - **Link-in-Bio Tools (Linktree, Stan Store):** These platforms succeed because they are truly mobile-first. However, they lack the operational depth required to run a full business (e.g., inventory management, complex quoting, intelligent customer support).

  ### The OHC Solution: Invisible AI Automation
  The core of the OHC philosophy is that the platform should do the work, and the owner should approve it. The "Unified Agent Feed" is the UI manifestation of this philosophy. It moves the user from a paradigm of "seeking information and performing tasks" to a paradigm of "reviewing proposals and granting approvals."

  ## Design Doc
  ### Architecture
  The Unified Agent Feed consolidates data from multiple sources into a single stream of actionable cards.

  ```mermaid
  graph TD
      subgraph Event Sources
          A[Webhooks: Instagram/WhatsApp]
          B[System: Inventory/Orders]
          C[Scheduled: Weekly Reports]
      end

      subgraph AI Layer
          D[Intent Classification]
          E[RAG Context Retrieval]
          F[Draft Generation]
      end

      subgraph Unified Agent Feed
          G[Actionable Cards]
          H[1-Tap Approval/Dismissal]
      end

      A --> D
      B --> D
      C --> D
      D --> E
      E --> F
      F --> G
      G --> H
  ```

  ### Mobile UX (375px First)
  - **Single Vertical Stream:** A clean, vertical feed replacing traditional dashboards.
  - **Action Cards:** Each card represents a pending decision (e.g., "Draft promo email?", "3 new orders to fulfill", "Approve reply to @customer").
  - **Touch Targets:** All buttons must be at least 44x44px.
  - **1-Tap Actions:** Primary actions ("Approve & Send", "Fulfill Now") must be prominent, with secondary actions ("Edit", "Dismiss") easily accessible.

  ## Implementation Prompt (For Engineering Swarm)
  **Feature Name:** Unified Agent Feed
  **Target Persona:** Maya the Baker (relies on mobile device, overwhelmed by multiple channels).

  **User-Facing Outcome:** A centralized mobile feed where the user reviews and approves AI-generated proposals for customer replies, marketing campaigns, and operational tasks.

  **Critical User Journey (CUJ):**
  1. The user logs into the OHC mobile app (375px view).
  2. The initial screen is the Unified Agent Feed, displaying actionable cards.
  3. A card shows an Instagram DM from a customer asking about vegan cake availability, along with an AI-drafted reply based on current inventory.
  4. The user taps the "Approve & Send" button (≥ 44x44px).
  5. The card is dismissed, and the reply is sent.

  **Acceptance Criteria:**
  - The UI must be strictly designed for a 375px viewport (no horizontal scrolling).
  - Combine the current fragmented data sources (e.g., `agent_feed_items` and `triage_items`) into a single, cohesive API endpoint and UI feed.
  - Ensure all interactive elements have a minimum 44x44px touch target.
  - Apply the OHC Premium Design Tokens (Glassmorphism, specific typography).
  - Include automated Playwright E2E tests verifying the feed layout and approval interactions on a 375px mobile viewport.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
