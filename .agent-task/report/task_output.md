issue_title: "[Architecture] Autonomous Unified Work Triage Feed & Notification Fabric"
issue_description: |
  ## 1. Problem Statement
  Owners like Maya (Baker) and Carlos (Handyman) receive scattered signals across Instagram DMs, SMS, booking requests, payment successes, and inventory alerts. Existing tools (Shopify, Wix) silo these into separate pages (Orders, Customers, Analytics). The owner has to constantly poll different sections of the app to figure out "What needs my attention right now?". They need a single, unified "Work Triage" feed that acts like an AI assistant telling them exactly what is urgent and offering a one-tap action to resolve it.

  ## 2. Research Report
  - **Market Context**: Tencent Workbuddy and Feishu excel at unifying communications and tasks into a single feed, but they are built for large enterprises. Shopify Sidekick is conversational but requires the user to initiate the prompt. We need a proactive feed that surfaces work items intelligently.
  - **The OHC Opportunity**: By routing all domain events (messages, bookings, transactions, system alerts) through a central message bus into an AI-prioritized feed, OHC can deliver on the core promise: "Open OHC and immediately know what needs attention today."
  - **Competitor Gaps**:
    - *Shopify*: Fragmented notifications; lacks proactive AI summarization of daily tasks.
    - *Square*: Good point-of-sale alerts, but weak on multi-channel message aggregation (Instagram DMs + SMS + Email).
    - *Feishu/Lark*: Powerful, but UI is too complex and desktop-centric for a 375px mobile owner like Fatima (Food Cart).

  ## 3. Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Instagram/SMS/Email Webhooks] --> B(Ingestion Gateway)
      C[Stripe/Payment Webhooks] --> B
      D[Internal Booking/System Events] --> B
      B --> E{KAIROS Event Bus / Redis}
      E --> F[AI Work Triage Agent]
      F -->|Contextualizes & Prioritizes| G[(PostgreSQL: TriageFeed)]
      G --> H[Flutter Mobile Client - 375px]
      H --> I[One-Tap Action / AI Draft Reply]
  ```

  ### Data Model (PostgreSQL)
  - `TriageItem`: `id`, `tenant_id`, `source_type` (Message, Booking, Alert), `priority_score` (AI-computed), `status` (Pending, Resolved, Ignored), `summary` (AI-generated plain text), `suggested_action` (JSON payload for one-tap execution).

  ### AI Agent Integration Points
  - **Work Triage Agent**: Subscribes to the Event Bus. When a new event arrives (e.g., missed call + voicemail from a number not in CRM), the agent transcribes it, scores its urgency, cross-references existing CRM records, and creates a `TriageItem`.
  - **Customer Assistant**: If the item is a message, it pre-drafts a reply and stores it in the `suggested_action` payload.

  ### Mobile UX Flow (375px)
  1. **Home Command Center**: The first screen the owner sees. No complex charts first—just a unified feed of "Action Required".
  2. **Triage Card**: Each item is an Apple-style Translucent Glass card. It shows a 1-sentence summary ("New lead from Instagram asked about Vegan Cakes").
  3. **One-Tap Resolution**: A prominent button on the card: "Review AI Reply" or "Approve Quote". Pressing it opens a bottom sheet (modal) to execute without leaving the feed.
  4. **Swiping**: Swipe right to resolve/archive, swipe left to snooze.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Unified Work Triage Feed
  **Target Persona**: Maya the Baker
  **Outcome**: Maya opens the app in the morning and sees a single list of 3 urgent items: 2 Instagram DMs with AI-drafted replies ready to send, and 1 unpaid deposit to follow up on.

  **Next Actions**:
  1. Define the PostgreSQL `TriageItem` schema with Row-Level Security (RLS) on `tenant_id`.
  2. Implement the ingestion layer to route multi-channel events into the AI Work Triage Agent for scoring and summarization.
  3. Build the Flutter mobile UI (375px first) featuring Translucent Glass triage cards and swipe-to-resolve gestures.
  4. Integrate the Customer Assistant to populate the `suggested_action` with pre-drafted responses.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
