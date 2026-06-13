issue_title: "[Research] Autonomous Payment Recovery & Invoice Management Architecture"
issue_description: |
  # Task: Autonomous Payment Recovery & Invoice Management Architecture

  ## Problem Statement
  Small business owners and independent professionals (like Carlos the handyman and Nora the agency principal) often struggle with delayed payments, uncollected deposits, and the manual burden of tracking unpaid invoices. The current OHC system lacks a dedicated, AI-driven mechanism to proactively follow up on pending payments, recover abandoned checkout sessions, or manage the lifecycle of an invoice (from draft to paid to receipt) autonomously. Owners currently have to manually check their bank accounts or Stripe dashboard to see who has paid, and then manually draft emails or messages to remind clients. This breaks the "Assistant-First" promise of OHC, where the system should do the coordinating and tracking.

  ## Research Report
  ### Market Dynamics
  - **The Pain of Unpaid Work**: SMBs experience significant cash flow issues due to late payments. The manual effort required to chase payments strains client relationships and consumes valuable owner time.
  - **Competitor Solutions**:
    - **Stripe & Square**: Offer automated retries for subscriptions and basic payment links, but lack deeply integrated, conversational AI follow-ups that understand the context of the work (e.g., "Hi [Client], just a reminder that the deposit for the plumbing job on Tuesday is due...").
    - **Shopify**: Handles abandoned carts well for e-commerce, but less effective for service-based businesses or complex project invoices.
    - **HoneyBook/Dubsado**: Strong invoice and workflow automation for creatives, but often require heavy initial setup and are not truly "agentic" (they follow rigid rule-based triggers rather than intelligently adapting to the conversation or context).
  - **The OHC Opportunity**: OHC can differentiate by integrating the "Finance & Decision Assistant" with the "Customer & Relationship Assistant". When an invoice goes unpaid, the system doesn't just send a robotic reminder; an AI agent drafts a contextual, polite follow-up message in the owner's preferred channel (email, WhatsApp, SMS), ready for 1-click approval or full autonomous sending based on owner preference.

  ## Design Doc
  ### High-Level Architecture
  The solution introduces an `InvoiceLifecycleAgent` (a specialized background worker) and extends the existing payment/ledger data models to support state transitions and autonomous recovery actions.

  **Architecture Diagram**
  ```mermaid
  erDiagram
      Tenant ||--o{ Invoice : owns
      Customer ||--o{ Invoice : billed_to
      Invoice {
          string id
          string status "draft, open, past_due, paid, uncollectible"
          datetime due_date
          int amount_cents
      }
      Invoice ||--o{ CommunicationEvent : triggers
      CommunicationEvent {
          string id
          string status "drafted, approved, sent"
          string channel "email, sms, whatsapp"
          string drafted_content
      }
      FinanceAgent ||--o{ Invoice : monitors
      FinanceAgent ||--o{ CommunicationEvent : generates
  ```

  **Core Components:**
  1.  **Invoice & Payment Intent Schema Expansion**: Enhance the database schema to track detailed payment states (e.g., `draft`, `sent`, `viewed`, `partially_paid`, `overdue`), due dates, and linked `CommunicationEvent` records.
  2.  **Payment Event Webhook Handler**: A robust webhook receiver (e.g., for Stripe) that strictly handles idempotency and updates the internal ledger and invoice states reliably.
  3.  **The Finance Agent (Background Job)**: A scheduled job (using the PostgreSQL `SKIP LOCKED` pattern) that periodically scans for invoices nearing their due date or currently overdue.
  4.  **Agentic Follow-up Workflow**:
      - When an invoice is flagged, the Finance Agent queries the Customer Context (past interactions, preferred channel).
      - It calls the LLM (Gemini Pro/MiniMax) to draft a contextual reminder message.
      - The draft is placed in the "Work Triage" feed for the owner (e.g., "Nora, Client X's $500 deposit is 2 days late. Should I send this reminder?").
      - Upon owner approval (or if configured for auto-send), the message is dispatched via the relevant channel adapter.

  ### AI Agent Integration Points
  - **Trigger**: Time-based (due date approaching/passed) or Event-based (payment link abandoned after X hours).
  - **Context Gathering**: The agent pulls the specific `Invoice` details, the `Customer` profile, and the `Project/Booking` context.
  - **Action**: Generates a `DraftMessage` entity in the system, linked to the `Invoice`, and surfaces it in the OHC mobile app's main assistant feed.

  ### Mobile UX Flow (375px First)
  1.  **The Feed**: The owner opens the app and sees a high-priority card: "1 Overdue Invoice. Client: ACME Corp. Amount: $1,200."
  2.  **The Detail View**: Tapping the card opens a unified view showing the invoice status, a timeline of past communications, and the AI-drafted reminder message.
  3.  **The Action**: A prominent "Review & Send" button. The owner can edit the text directly in a native text field or tap "Send Now".
  4.  **The Confirmation**: A clear success state ("Reminder Sent to ACME Corp via Email") and the card is dismissed from the urgent feed.

  ## Implementation Prompt
  **User Persona**: Nora (Agency Principal) / Carlos (Handyman).
  **The Goal**: Automatically track pending payments and surface intelligent, contextual reminder drafts to the owner when payments are late.
  **CUJ (Critical User Journey)**:
  1. An invoice in the system becomes "overdue".
  2. The backend Finance Agent detects this and drafts a reminder message using the LLM, incorporating the client's name and the specific service details.
  3. The drafted message appears as an actionable card in the owner's Work Triage feed in the UI.
  4. The owner reviews the message, optionally edits it, and clicks "Approve & Send".
  5. The system records the communication and updates the invoice status to reflect that a reminder was sent.

  **Acceptance Criteria**:
  - Extend the data model to support invoice due dates, specific payment statuses, and linked draft messages.
  - Implement a background worker (or scheduled task) that identifies overdue payments based on the new schema.
  - Integrate with the LLM provider to generate a contextual reminder draft based on the invoice and customer data.
  - Create the API endpoints necessary for the frontend to fetch these pending drafts and submit an approval/send action.
  - Build the mobile-first UI components (a feed card and a detail/edit view) following the Translucent Glass / UniFi design system, ensuring perfect rendering on a 375px screen.
  - E2E Tests must cover the full lifecycle: from an invoice turning overdue, to the draft appearing in the feed, to the owner approving and sending it.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
