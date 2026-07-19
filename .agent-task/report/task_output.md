issue_title: "Implement 'Agent Feed' with Native Mobile-First Action Cards"
issue_description: |
  ## Mission Queue Protocol Brief

  **Problem Statement:**
  Traditional platforms like Shopify or Wix rely heavily on dashboards that require users to proactively seek out information or initiate actions from a desktop. For non-technical owners running businesses on their mobile devices (e.g., Maya the baker or Carlos the handyman), dashboards are confusing and time-consuming. These users suffer from "Setup Paralysis" and "Fragmented Operations" across various tools. They don't want to use reactive software tools; they want the platform to proactively perform the work (via AI Agents) and simply present them with a unified stream of actionable approvals. We currently lack a true "zero-to-one" autonomous operations feed.

  **Research Report:**
  Based on competitive analysis (Track 1 & Track 2), legacy systems (Shopify, Squarespace) and modern AI tools (HubSpot Breeze) require desktop interfaces or reactive chatbots. While Link-in-Bio tools (Linktree) are successfully mobile-first, they lack complex business capabilities. A critical unresolved pain point in OHC (Track 3) is allowing users to manage complex business tasks on a small screen without clutter. The OHC differentiator is adopting an **"Approval" Interface Paradigm**. Rather than having the user fill out forms, AI Agents draft responses, emails, and actions. The user receives these as "Action Cards" in a unified "Agent Feed" designed entirely for a 375px mobile viewport, enabling 1-tap "Approve & Send" or "Edit" workflows.

  **Design Doc & Architecture:**
  The new Agent Feed is the central nervous system for business owners, replacing traditional dashboards.

  *Architecture Diagram:*
  ```mermaid
  sequenceDiagram
      actor Customer
      participant External API (Webhook)
      participant OHC Backend (Event Bus)
      participant AI Intent Classifier
      participant RAG & Database
      participant AI Agent (e.g. Promoter/Operations)
      participant OHC Mobile App (Agent Feed)
      actor Owner

      Customer->>External API (Webhook): e.g., Sends DM or Places Order
      External API (Webhook)->>OHC Backend (Event Bus): Ingest Event
      OHC Backend (Event Bus)->>AI Intent Classifier: Classify Intent
      AI Intent Classifier->>RAG & Database: Fetch Context (Policies, Inventory)
      RAG & Database-->>AI Agent (e.g. Promoter/Operations): Context & State
      AI Agent (e.g. Promoter/Operations)->>OHC Backend (Event Bus): Generate Drafted Action/Reply
      OHC Backend (Event Bus)-->>OHC Mobile App (Agent Feed): Push Action Card
      Owner->>OHC Mobile App (Agent Feed): Reviews Action Card on 375px Screen
      Owner->>OHC Mobile App (Agent Feed): Taps "Approve"
      OHC Mobile App (Agent Feed)->>OHC Backend (Event Bus): Execute Action
      OHC Backend (Event Bus)->>External API (Webhook): Send Reply / Complete Task
  ```

  *Mobile UX Flow:*
  The feed is a vertical list of cards on a 375px width screen.
  1. The user opens the app and sees the Agent Feed.
  2. Action Cards are displayed based on priority and lifecycle state (e.g., "Draft Reply", "Social Post Draft", "Review Draft Quote", "Weekly Payout Summary").
  3. Each Action Card has a consistent structure: Agent Badge, Priority, Context description, Content (the drafted message or summary), and Action Buttons (min 44x44px touch targets).
  4. Buttons should typically be: a primary "Approve & Send" (or "Take Over") and a secondary "Edit" (or "Dismiss").
  5. Tapping "Edit" morphs the card state to reveal a text input element (optimized for mobile keyboards) where the user can refine the AI's draft, along with "Save & Approve" and "Cancel" buttons.

  *AI Agent Integration Points:*
  - Integration with the backend for pushing state updates.
  - Intent classification output creates specialized payloads like `SocialPostDraft` or `WeeklyPayoutSummary` which the frontend renders natively as distinct card variations.

  *Key Design Decisions:*
  - **Mobile Input Optimization:** To ensure the smoothest mobile experience on virtual keyboards and scrolling on a 375px screen, we must ensure text inputs are mobile-friendly and don't cause screen jumping.
  - **Premium Tokens:** Use the established Glassmorphism (translucent materials) and typography (OHC Premium Tokens) for a clean Apple/Ubiquiti-style hierarchy.

  **Implementation Prompt:**
  As the implementer agent, you will develop the unified Agent Feed for the mobile web (and Tauri) frontend.
  - **Objective:** Create a seamless 375px-optimized feed component that pulls from the backend and displays action requirement objects. Implement variations like `ReviewDraftQuoteCard` and `PayoutSummaryCard`.
  - **CUJ:** Maya opens the OHC app and sees a pending Draft Reply from the Customer Success Agent in the Agent Feed. She taps "Edit", modifies the text using her mobile keyboard smoothly, taps "Save", and the card updates and dismisses smoothly.
  - **Acceptance Criteria:**
    - The feed has 0 horizontal scrolling on a 375px viewport.
    - Touch targets are >= 44x44px.
    - Provide E2E Playwright tests simulating the approval and editing of a card.

  **Priority:** P0
  **Estimated Scope:** Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
