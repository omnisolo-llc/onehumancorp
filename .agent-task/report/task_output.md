issue_title: "[Architecture] Autonomous Offline-First Agentic Quote-to-Cash Engine"
issue_description: |
  ## Title
  **Autonomous Offline-First Agentic Quote-to-Cash Engine**

  ## Problem Statement
  For field service owners like Carlos (Handyman) or agency principals like Nora, capturing demand on the go and converting it to paid work is highly fragmented. Carlos frequently operates in basements or remote job sites with zero cellular connectivity. If he can't draft a quote, get it approved, and take a deposit immediately, the lead goes cold. Existing solutions either require an active internet connection or force the owner to use multiple disconnected tools (a notebook, an invoicing app, and a payment terminal). They need an invisible, offline-capable assistant that can instantly draft a service quote from a brief voice note, queue it for sending, and collect a deposit via Tap-to-Pay without needing a signal.

  ## Research Report
  Based on our analysis of field service and small business platforms:
  - **Shopify/Wix/Squarespace:** These platforms are optimized for physical or digital product sales, not service-based quoting, dynamic project scoping, or offline field operations.
  - **Square/Stripe Terminal:** Offer robust offline card-present payments, but they are detached from the quoting and customer-approval lifecycle.
  - **ServiceTitan/Jobber:** Extremely powerful for field services but highly complex, expensive, and require significant manual configuration. They fail the "Grandmother Test" for a solo operator like Carlos.
  - **OHC Opportunity:** By combining offline-first CRDT (Conflict-free Replicated Data Type) syncing, Stripe Terminal SDKs, and the OHC "Sales Agent," we can create a zero-touch "Quote-to-Cash" flow. The owner speaks into their phone, the agent drafts the quote locally, the customer taps their card to approve and pay the deposit, and the system reconciles everything seamlessly once connectivity is restored.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Edge Device (Mobile 375px)
          UI[OHC Mobile App] --> VoiceCapture[Voice / Text Intake]
          VoiceCapture --> LocalAgent[Local Edge AI Sales Agent]
          LocalAgent --> DraftQuote[Local Quote Generation]
          DraftQuote --> TapToPay[Stripe Terminal Offline Tap-to-Pay]
          TapToPay --> LocalStore[(Local CRDT Ledger)]
          LocalStore --> OutboxQueue[Outbox Sync Queue]
      end
      OutboxQueue -- Network Reconnection --> BackendAPI[OHC Backend Sync API]
      BackendAPI --> CloudPostgres[(Postgres Cloud Ledger)]
      CloudPostgres --> AgentMesh[KAIROS Agent Orchestration Hub]
      AgentMesh --> FinanceAgent[Finance & Operations Agents]
  ```

  ### UI Wireframes / Screen Flow (375px First)
  1. **Intake Screen:** A single, prominent microphone button ("Tell OHC about the job").
  2. **Quote Draft Card:** A frosted glass translucent card displaying the generated quote (Services, Materials, Labor, Total). Editable via simple text or another voice prompt ("Add $50 for extra copper piping").
  3. **Customer Approval & Deposit:** A clear, full-width "Collect Deposit" button that seamlessly invokes the native Tap-to-Pay interface.
  4. **Offline Indicator:** A subtle, non-intrusive "Saved Offline" pill in the header, assuring the owner that the data is safe and will sync later.

  ### Mobile UX Flow
  The flow must feel entirely native and instantaneous. There should be no blocking loading spinners if the network is unavailable. The local edge AI handles basic quote structuring, and the Tap-to-Pay SDK securely caches the card details. When the device regains connectivity, a background process transparently pushes the queued transactions and quote approvals to the backend.

  ### AI Agent Integration Points
  - **Sales Agent (Edge/Cloud):** Parses the initial unstructured voice/text input to itemize the quote. Learns from past quotes to suggest accurate pricing.
  - **Finance Agent (Cloud):** Upon sync, reconciles the offline deposit, generates the formal tax-compliant invoice, and schedules the final payment reminder.
  - **Operations Agent (Cloud):** Blocks out the required calendar time and updates inventory levels for materials used.

  ### Key Design Decisions
  - **Local CRDT Storage:** Ensures no data loss and handles merge conflicts gracefully when multiple devices are used.
  - **Stripe Terminal SDK Offline Mode:** Essential for capturing payments in signal dead zones.
  - **Voice-First Intake:** Significantly reduces friction for operators wearing gloves or carrying tools.

  ## Implementation Prompt
  Implement the Autonomous Offline-First Agentic Quote-to-Cash Engine.

  **User-Facing Outcome:** The owner can open the app, speak a job description, instantly see a structured quote, and collect a card tap deposit—all while completely disconnected from the internet.

  **Critical User Journey (CUJ):**
  1. App opened in airplane mode.
  2. Owner inputs job details (text/mock voice).
  3. App generates quote locally.
  4. Owner hits "Collect Deposit" (mock offline Tap-to-Pay).
  5. App stores data locally.
  6. Network restored -> App syncs quote and payment to backend.

  **Acceptance Criteria:**
  - Create the backend endpoints to handle outbox syncing of quotes and offline payments.
  - Update the Flutter/Mobile client to support local quote generation and offline payment queuing.
  - Ensure 100% of the core quoting and deposit flow functions without a network connection.
  - Implement automated E2E tests validating the offline creation -> reconnect -> sync lifecycle.
  - Ensure the UI adheres to the premium Translucent Glass design tokens and 375px mobile constraints.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
