issue_title: "Implement Autonomous Omni-Channel Intake & Quote Generation Engine"
issue_description: |
  ## Problem Statement
  Service professionals like Carlos (Handyman) and Nora (Agency Principal) manage a highly fragmented intake process. Potential clients reach out via SMS, Instagram DMs, website forms, or email. The manual process of gathering requirements, checking calendar availability, drafting an initial estimate, and sending a deposit link creates immense friction and leads to dropped leads. Existing platforms (Shopify, Wix) treat services as static products, while booking tools (Calendly) don't handle dynamic quoting or cross-channel communication. OHC needs an engine that unifies intake across all channels and uses AI to autonomously draft quotes and deposit requests.

  ## Research Report
  ### Competitive Analysis
  - **Shopify/Wix**: Built for physical or standard digital products. Service quoting requires messy workarounds or expensive third-party apps, lacking omni-channel integration.
  - **HoneyBook/Dubsado**: Strong CRM for creatives, but requires the user to manually build templates, review every inquiry, and trigger workflows. Not a "zero setup" or fully autonomous solution.
  - **Calendly**: Great for scheduling, but has no quoting capabilities and operates outside the merchant's main business OS.

  ### Market Needs
  Non-technical operators need an assistant that acts like a receptionist and an estimator. When a message like "I need my sink fixed this week" comes in via WhatsApp, the platform should proactively parse the intent, ask follow-up questions if necessary, draft a quote based on predefined service rates, and present it to the owner for a one-tap approval.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Intake Channels
          SMS[Twilio / SMS] --> Gateway[Omni-Channel Gateway];
          IG[Instagram DMs] --> Gateway;
          Web[Web Intake Form] --> Gateway;
      end

      Gateway --> Triage[Work Triage Queue];

      subgraph AI Departments
          Triage --> SalesAgent[Sales & Revenue Agent];
          Triage --> OpsAgent[Operations Agent];
          SalesAgent -- Context --> Knowledge[Knowledge & Documents Agent];
      end

      SalesAgent --> DraftQuote[Quote Generation Engine];
      OpsAgent --> CheckCal[Availability Check];

      DraftQuote --> OHCApp[OHC Mobile App - 375px];
      CheckCal --> OHCApp;

      OHCApp -- Approval --> Stripe[Stripe Payment Link / Deposit];
      Stripe --> Customer[Send to Customer via Source Channel];
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification**: Carlos receives a push notification: "New quote drafted for Sink Repair (Via SMS)".
  2. **Work Triage Feed**: Tapping the notification opens the OHC app to the unified Work Feed. A Translucent Glass card shows the parsed customer request, the drafted quote ($150 + $50 parts), and suggested times based on his calendar.
  3. **One-Tap Action**: The card presents an "Approve & Send Quote" primary button, alongside an "Edit" button.
  4. **Customer Experience**: Once approved, the customer receives a unified web link (sent back via their original channel) containing the quote details and a Stripe checkout for a 50% deposit.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Unifies messages and routes inquiries to the correct department.
  - **Sales & Revenue Agent**: Drafts the quote by comparing the customer's text against the owner's service catalog and past pricing memory.
  - **Operations Agent**: Cross-references the drafted quote with current calendar availability to suggest valid booking slots.

  ### Key Design Decisions
  - **Unified Intent parsing**: The system must treat all incoming text (SMS, IG, Web) identically, extracting `CustomerIntent` before invoking business logic.
  - **Owner in the Loop**: AI drafts quotes but does *not* send binding estimates without explicit owner approval.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Goal:** Build the Omni-Channel Intake & Quote Generation Engine.

  **User-Facing Outcome:** The owner can connect SMS or IG, receive an unstructured service request, and open the OHC app to find a fully drafted, actionable quote with a deposit link ready for one-tap approval.

  **Critical User Journey (CUJ):**
  1. A prospective client sends an SMS: "Can you fix a leaky pipe on Tuesday?"
  2. The backend webhook receives the message and triggers the AI intent classification.
  3. The Sales Agent drafts a quote for "Basic Plumbing Repair" and the Ops agent confirms Tuesday availability.
  4. The owner opens the OHC mobile app (375px), sees the drafted quote in their feed, and taps "Approve & Send".
  5. The customer receives an SMS back with a secure OHC quote & deposit payment link.

  **Acceptance Criteria:**
  - Create the `Inquiry` and `QuoteDraft` data models with strict PostgreSQL multi-tenant RLS.
  - Implement the AI triage pipeline to parse raw text into structured service requests.
  - Build the mobile-first approval card UI matching the premium OHC design system.
  - Write E2E Playwright tests covering the full flow from incoming webhook simulation to owner UI approval.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []