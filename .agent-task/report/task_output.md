issue_title: "[Architecture] Instant Localized Invoicing & Deposits"
issue_description: |
  # [Architecture] Instant Localized Invoicing & Deposits

  ## Problem Statement
  Carlos (Handyman, 42) relies on word of mouth for his business. He currently uses a mix of SMS texts for quotes, cash or personal Venmo for payments, and has no formal way to collect deposits before starting a job. This fragmented approach leads to lost revenue from no-shows, a lack of professional presentation, and significant administrative overhead when trying to track who has paid. When Carlos finishes a job, he needs to instantly generate a professional invoice on his Android phone, accept partial or full payment on the spot (or via a quick link), and automatically track the status in a centralized ledger. The current OneHumanCorp (OHC) architecture lacks a native, seamlessly integrated quoting, instant invoicing, and deposit collection engine.

  ## Research Report
  **Competitive Analysis:**
  - **Square Invoices:** Highly capable, offering estimates that convert to invoices and native deposit tracking. However, it pulls users into the Square ecosystem, potentially fragmenting their workflow if they use OHC for other business aspects.
  - **QuickBooks:** The industry standard for accounting, but often too complex for a solopreneur like Carlos. Its mobile app can be overwhelming and doesn't pass the "grandmother test."
  - **Stripe Invoicing:** Powerful API but lacks a native, zero-setup mobile interface for non-technical users.
  - **Joist:** Popular among contractors for estimates and invoicing, but it is a point solution, not an all-in-one platform.

  **Market Needs:**
  Service-based businesses (like Carlos) or custom-order businesses (like Maya the baker) absolutely require the ability to take a deposit to secure their time or materials. A seamless flow from "Quote/Estimate" -> "Deposit" -> "Final Invoice" -> "Payment" must be integrated into the OHC platform, accessible entirely via mobile, and managed invisibly by AI agents.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> LocalDB[(Local SQLite / CRDT)];
          App --> QuoteEngine[Quote & Invoice Builder];
          LocalDB --> SyncEngine[Offline Sync Engine];
      end

      SyncEngine -- Network Restored --> Gateway[OHC API Gateway];
      Gateway --> MultiTenantDB[(Cloud Postgres - Tenant Isolated)];
      Gateway --> PaymentGateway[Payment Processor API];
      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> FinanceAgent[Finance: Reconcile Ledger, Track Deposits];
          Agents --> OpsAgent[Ops: Update Calendar/Booking Status];
          Agents --> CSAgent[CS: Follow-up on Unpaid Invoices];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard:** Carlos opens the OHC app. He taps a large, glassmorphic "New Job" button.
  2. **Quote Building:** He enters the customer's phone number and quickly lists services (e.g., "Drywall Repair - $200"). A clean, modular card displays the running total.
  3. **Deposit Setting:** He toggles "Require Deposit" and selects "50%".
  4. **Instant Sending:** He taps "Send Quote". The app generates a professional, localized SMS or WhatsApp message with a secure link.
  5. **Status Tracking:** The dashboard updates to show the quote in a "Pending Deposit" state. Once the customer pays the deposit via the link, the status updates to "Ready to Start," and Carlos is notified.
  6. **Final Invoice:** Upon completion, Carlos opens the job card, taps "Convert to Invoice", adjusts any final materials costs, and collects the remaining balance on the spot (potentially using Tap-to-Pay) or sends the final invoice link.

  ### AI Agent Integration Points
  - **Finance Agent:** Automatically tracks the invoice lifecycle. If an invoice is past due, it triggers the CS Agent. Reconciles the initial deposit with the final payment in the global ledger.
  - **CS Agent (Customer Service):** Drafts and sends polite, personalized follow-up texts/emails for unpaid invoices (e.g., "Hi [Name], just checking in on the invoice for the drywall repair...").
  - **Operations Agent:** If the invoice is tied to a calendar booking (e.g., Leo the music tutor), the Ops agent automatically cancels or reschedules the slot if the deposit isn't paid within the required timeframe.

  ### Key Design Decisions
  - **State Machine Integration:** Invoices must be modeled as a strict state machine (Draft -> Quote -> Deposit Paid -> Finalized -> Paid -> Void) within the multi-tenant database to ensure the Finance Agent has a deterministic source of truth.
  - **Offline-First Resilience:** Carlos might be generating a quote in a basement with no cell service. The quote is built and stored locally (CRDT) and automatically dispatched by the sync engine when he steps outside.
  - **No Implementation Jargon:** The user only sees terms like "Quote," "Deposit," and "Invoice." The underlying state transitions, multi-tenant isolation, and SPIFFE/SPIRE secure identity verification are completely hidden.

  ## Implementation Prompt
  Implement the "Instant Localized Invoicing & Deposits" capability.
  - **User-Facing Outcome:** Users can instantly generate a quote, mandate a deposit, and convert it to a final invoice directly from their mobile device. The system automatically tracks payment status and handles follow-ups.
  - **CUJ (Critical User Journey):**
    1. User creates a new quote for a service.
    2. User specifies a required deposit amount/percentage.
    3. User sends the quote via SMS/Email directly from the app.
    4. Customer pays the deposit via the web link.
    5. App notifies the user that the deposit is secured and the job is ready.
    6. User converts the quote to a final invoice and collects the remaining balance.
  - **Acceptance Criteria:**
    - Mobile-first UI strictly adhering to the glassmorphism and modular card design system.
    - Full offline capability for drafting quotes/invoices.
    - AI agents successfully monitor and react to invoice state changes (e.g., sending reminders).
    - Guaranteed multi-tenant data isolation for all financial records.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
