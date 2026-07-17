issue_title: "Implement the Mobile-First Unified Agent Feed (Assistant-First Shell)"
issue_description: |
  ## Title: The OHC Mobile-First Unified Agent Feed (Assistant-First Shell)

  ## Problem Statement
  Current e-commerce and small business platforms (Shopify, Wix, Squarespace) trap owners in complex, multi-tab admin dashboards. For personas like Maya (Home Baker) and Carlos (Field Service Owner) who operate entirely from a 375px mobile device, traditional dashboards are overwhelming and fail to answer the most critical question: "What do I need to do right now?"
  Owners don't need a list of tools; they need a trusted assistant that tells them what requires their attention, why it matters, and provides a one-tap action to resolve it (e.g., "Approve drafted reply", "Accept deposit", "Restock inventory").

  ## Research Report
  Our competitive analysis indicates a massive gap in the SMB market:
  - **Legacy Giants (Shopify, Wix, Squarespace):** Rely heavily on "Companion Apps" which are good for checking stats but terrible for complex operational tasks. Actioning tasks requires deep navigation into settings.
  - **AI-Native Rivals (Durable, Lindy):** While Durable excels at fast setup, it lacks deep operational automation. Lindy is great for personal executive tasks but not tailored for robust multi-channel commerce and POS integration.
  - **The OHC Opportunity:** By replacing the traditional "Dashboard" with a "Unified Agent Feed," OHC can become the first truly assistant-led operational OS for SMBs. This directly aligns with the OHC Promise: "Ask one assistant; it coordinates messages, customers, tasks, calendar, documents, payments, analytics, and agent work behind the scenes."

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Events[Webhook & State Events] --> EventBus[Event Bus / Message Queue]
      EventBus --> TriageAgent[Work Triage Agent]

      TriageAgent --> Context[Tenant Context & RAG Memory]
      Context --> DraftGen[LLM Draft & Action Generation]

      DraftGen --> NotificationService[Notification & Feed Service]
      NotificationService --> MobileApp[OHC Mobile Shell 375px]

      MobileApp --> Action[Owner Approves/Edits Card]
      Action --> ExecutionRouter[Execution Router]
      ExecutionRouter --> Systems[Stripe, SendGrid, IG Graph API]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  The core paradigm is the "Approval Interface".
  1. **The Shell:** Upon opening the OHC app, the user lands on the "Today" tab. No charts, no complex menus. Just a vertical feed of Action Cards.
  2. **Action Cards:** Each card represents a pending decision drafted by an AI Agent (e.g., Marketing Agent, Operations Agent, Customer Success Agent).
      - **Card Anatomy:**
          - **Header:** Agent Identity (e.g., "Operations Assistant") & Urgency Token (e.g., 🔴 Urgent).
          - **Context:** Plain text description (e.g., "Maya, 3 cake inquiries came in overnight via Instagram.")
          - **Drafted Action:** The proposed response or operational change.
          - **Actions:** Large (>= 44x44px touch targets) buttons: [Approve & Send], [Edit], [Dismiss].
  3. **Visual Excellence:** The UI must employ the OHC Premium Token library: macOS-style Translucent Glass materials, clean Ubiquiti UniFi modular dashboard card layouts, restrained typography, and a strict 375px width boundary without horizontal scrolling.

  ### AI Agent Integration Points
  - **Work Triage:** Groups disparate events (IG DMs, new Stripe payments, low inventory) into a unified priority queue.
  - **Specialized Agents:** (Sales, Finance, Customer Success) subscribe to the Triage queue, generate domain-specific drafts using tenant context (Redis memory), and push Action Cards to the user's feed.

  ### Key Design Decisions
  - **Zero Dashboard:** We are explicitly removing the static "stats dashboard" from the home screen to enforce the "Assistant-First" paradigm.
  - **One-Tap Execution:** All complex logic (e.g., updating DB state, making API calls to Stripe) is abstracted behind the single "Approve" button on the Action Card.
  - **Tenant Isolation:** All feed data must be strictly scoped by `tenant_id` at the database level (PostgreSQL RLS) and cache level (Redis).

  ## Implementation Prompt
  **Objective:** Build the core Mobile-First (375px) Unified Agent Feed UI and the backend notification pipeline that delivers Action Cards to it.

  **Critical User Journey (CUJ):**
  1. Maya opens the OHC mobile app on a simulated iPhone (375px).
  2. The home screen displays the Unified Agent Feed with two cards:
      - *Card 1 (Operations):* "Inventory Low: You only have 1 Red Velvet Cake left. Restock?" -> [Approve Restock]
      - *Card 2 (Customer Success):* "Drafted reply to Carlos on IG regarding vegan options." -> [Approve & Send]
  3. Maya taps [Approve Restock] on Card 1.
  4. The card visually transitions to a "Success" state, and the backend inventory ledger is updated via the Operations Agent.

  **Acceptance Criteria:**
  - Layout must be strictly constrained to 375px (no horizontal scrolling).
  - All interactive elements must have a minimum 44x44px touch target.
  - Apply Translucent Glass styling and UniFi card layouts to the Action Cards.
  - Implement a full-loop E2E Playwright test covering the user approving an Action Card from the feed.
  - Zero mock data in the UI; use real backend state or documented seed scripts for verification.
  - Ensure 100% unit test coverage for new backend feed logic.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
