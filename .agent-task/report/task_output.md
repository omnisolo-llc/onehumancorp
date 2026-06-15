issue_title: "[Research] AI Automated Task Recovery and Customer Nurturing Workflows"
issue_description: |
  # Research Report: AI Automated Task Recovery and Customer Nurturing Workflows

  ## 1. Problem Statement
  Small business owners and independent professionals (like Carlos the handyman or Leo the tutor) often struggle with following up on cold leads, recovering abandoned requests or handling check-ins after a service. The traditional approach requires them to constantly monitor their CRM and manual follow up, which is time-consuming and prone to human error. There's a clear opportunity for OHC to automate this workflow, converting lost potential into revenue while saving the owner’s time.

  ## 2. Market Mapping & Competitor Discovery

  ### Competitor Analysis
  - **HubSpot Breeze / ActiveCampaign:** Both have powerful automation workflows for recovering abandoned deals. However, they rely on complex condition builders (if/then logic trees) that overwhelm non-technical users.
  - **Shopify Flow / Klaviyo:** Excellent for e-commerce abandoned cart recovery, but very limited for service-based businesses or complex, multi-touch personalized follow-ups.
  - **Lindy.ai / 11x.ai:** Provide strong AI executive assistants that handle follow-ups, but they often lack deep integration into the native booking and inventory systems.

  ### The OHC Gap
  Existing tools either require complex visual builders or focus purely on simple e-commerce carts. OHC needs a zero-configuration, agent-driven recovery system. The owner shouldn't build an "abandoned lead" workflow; the AI should proactively identify aging leads and propose a personalized follow-up message based on the customer's context.

  ## 3. Design Doc: The "Nurture Agent" Workflow

  ### Architecture & AI Agent Integration
  - **Agent Role:** The Nurture Agent (part of the Customer & Relationship Assistant) runs daily background jobs via the PostgreSQL `SKIP LOCKED` job queue.
  - **Trigger:** The agent scans for specific invariants: e.g., "Leads without interaction for 48 hours," "Appointments completed 7 days ago without a review," or "Draft quotes not approved after 3 days."
  - **Action:** Instead of auto-sending (which feels risky to owners), the agent drafts a personalized, context-aware message and surfaces it to the Unified Agent Feed as a pending action.

  ### Mobile UX Flow (375px First)
  1.  **Feed Context:** The owner opens the app. At the top of their Unified Feed is a card from the Nurture Agent.
  2.  **The Card:** "You have 3 unapproved quotes from last week. Should I follow up?"
  3.  **Review Step:** Tapping the card reveals a swipeable list. Each item shows the customer name, the quote details, and a pre-drafted message (e.g., "Hi [Name], just checking if you had any questions about the repair estimate?").
  4.  **One-Tap Action:** The owner taps "Send All" or can individually tap to edit or discard.

  ## 4. Implementation Prompt

  **Objective:** Implement the "Nurture Agent" background scanner and integrate its outputs into the Unified Agent Feed for mobile-first approval.

  **User-Facing Outcome:** The owner (e.g., Carlos) receives proactive cards in their feed suggesting personalized follow-ups for stale leads or unapproved quotes, requiring only a single tap to execute.

  **Critical User Journey (CUJ):**
  1. System generates a mock scenario: A quote for "Roof Repair" was sent to "John Doe" 4 days ago and remains in `draft/sent` status without approval.
  2. The background job runs and detects the stale quote.
  3. The Nurture Agent drafts a follow-up SMS/Email.
  4. Owner logs into the OHC mobile UI (375px viewport).
  5. Owner sees a new "Nurture Action" card in their feed.
  6. Owner taps the card, reviews the drafted message for John Doe, and taps "Approve & Send".
  7. System marks the task as complete and simulates sending the message.

  **Acceptance Criteria:**
  - Background worker reliably identifies stale entities (quotes/leads) based on timestamps.
  - AI prompt successfully generates context-aware follow-up messages.
  - UI seamlessly displays the proposed action in a mobile-first card layout (touch targets > 44px).
  - Playwright E2E test verifies the entire flow from detection to owner approval.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
